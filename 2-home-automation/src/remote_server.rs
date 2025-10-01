use bevy::{platform::collections::HashMap, prelude::*, tasks::IoTaskPool};
use crossbeam::channel::{self, Receiver};

use crate::{lights::Light, natural_time::Date};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Device {
    Light(Light),
}

mod internal;

pub fn remote_server_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            #[cfg(feature = "current_state")]
            get_latest_state,
            #[cfg(feature = "light_history")]
            check_history_requests,
            #[cfg(any(not(feature = "light_history"), not(feature = "current_state")))]
            || {},
        ),
    )
    .add_observer(forward_state_changes);
    #[cfg(feature = "light_history")]
    app.add_observer(forward_history_request);
}

#[derive(Event)]
pub struct ServerLightStateChange {
    pub light: Light,
    pub on: bool,
}

#[derive(Event)]
pub struct ManualLightStateChange {
    pub light: Light,
    pub on: bool,
}

#[cfg(feature = "light_history")]
#[derive(Event)]
pub struct LightHistoryRequest {
    pub light: Light,
}

#[cfg(feature = "current_state")]
fn get_latest_state(
    mut commands: Commands,
    date: Res<Date>,
    mut previous_states: Local<HashMap<Device, bool>>,
    mut task: Local<Option<Receiver<HashMap<Device, bool>>>>,
) {
    let current_time = date.current_time;
    if let Some(running_task) = task.take() {
        if let Ok(done) = running_task.try_recv() {
            for (device, state) in done {
                if previous_states.get(&device) != Some(&state) {
                    previous_states.insert(device, state);
                    match device {
                        Device::Light(light) => {
                            commands.trigger(ServerLightStateChange { light, on: state })
                        }
                    };
                }
            }
        } else {
            *task = Some(running_task);
            return;
        }
    }
    let (sender, receiver) = channel::bounded(1);

    *task = Some(receiver);
    IoTaskPool::get()
        .spawn(async move {
            let state = internal::get_current_state(current_time).await;
            let _ = sender.send(state);
        })
        .detach();
}

fn forward_state_changes(event: On<ManualLightStateChange>, mut commands: Commands) {
    let light = event.light;
    let on = event.on;
    IoTaskPool::get()
        .spawn(async move {
            internal::change_state(Device::Light(light), on).await;
        })
        .detach();
    commands.trigger(ServerLightStateChange {
        light: event.light,
        on: event.on,
    });
}

#[cfg(feature = "light_history")]
fn forward_history_request(event: On<LightHistoryRequest>, mut commands: Commands) {
    let (sender, receiver) = channel::bounded(1);
    let event_light = event.light;
    IoTaskPool::get()
        .spawn(async move {
            let _ = sender.send(internal::get_history(Device::Light(event_light)).await);
        })
        .detach();
    commands.spawn(DeviceHistoryRequest {
        device: Device::Light(event.light),
        task: receiver,
    });
}

#[cfg(feature = "light_history")]
#[derive(Component)]
struct DeviceHistoryRequest {
    device: Device,
    task: Receiver<Vec<(u32, bool)>>,
}

#[cfg(feature = "light_history")]
#[derive(Event)]
pub struct LightHistory {
    pub light: Light,
    pub history: Vec<(u32, bool)>,
}

#[cfg(feature = "light_history")]
fn check_history_requests(
    mut commands: Commands,
    requests: Query<(Entity, &DeviceHistoryRequest)>,
) {
    for (entity, running_task) in &requests {
        if let Ok(done) = running_task.task.try_recv() {
            match running_task.device {
                Device::Light(light) => commands.trigger(LightHistory {
                    light,
                    history: done,
                }),
            };
            commands.entity(entity).despawn();
        } else {
            return;
        }
    }
}
