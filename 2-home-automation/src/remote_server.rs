use bevy::{platform::collections::HashMap, prelude::*, tasks::IoTaskPool};
use chrono::Timelike;
use crossbeam::channel::{self, Receiver};

use crate::{lights::Lights, natural_time::Date};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Device {
    Light(Lights),
}

mod remote_server_internals {
    use bevy::platform::collections::HashMap;

    use crate::{lights::Lights, remote_server::Device};
    use std::{thread, time::Duration};

    struct Schedule<'a> {
        device: Device,
        changes: &'a [(u32, u32)],
    }

    const TYPICAL_DAY: &[Schedule<'static>] = &[
        Schedule {
            device: Device::Light(Lights::Kitchen),
            changes: &[
                (5 * 60 + 30, 1),
                (7 * 60, 0),
                (17 * 60 + 5, 1),
                (17 * 60 + 35, 0),
                (19 * 60, 1),
                (21 * 60, 0),
            ],
        },
        Schedule {
            device: Device::Light(Lights::Hall),
            changes: &[(17 * 60, 1), (17 * 60 + 30, 0)],
        },
        Schedule {
            device: Device::Light(Lights::Hallway),
            changes: &[
                (5 * 60 + 20, 1),
                (6 * 60 + 5, 0),
                (21 * 60 + 55, 1),
                (22 * 60 + 35, 0),
            ],
        },
        Schedule {
            device: Device::Light(Lights::Bedroom1),
            changes: &[
                (5 * 60 + 17, 1),
                (6 * 60, 0),
                (22 * 60, 1),
                (23 * 60 + 45, 0),
            ],
        },
        Schedule {
            device: Device::Light(Lights::Bathroom1),
            changes: &[
                (5 * 60 + 25, 1),
                (5 * 60 + 28, 0),
                (22 * 60 + 10, 1),
                (22 * 60 + 30, 0),
            ],
        },
        Schedule {
            device: Device::Light(Lights::Bedroom2),
            changes: &[
                (5 * 60 + 15, 1),
                (6 * 60 + 3, 0),
                (22 * 60, 1),
                (23 * 60 + 55, 0),
            ],
        },
        Schedule {
            device: Device::Light(Lights::Bathroom2),
            changes: &[
                (5 * 60 + 15, 1),
                (5 * 60 + 25, 0),
                (22 * 60 + 15, 1),
                (22 * 60 + 30, 0),
            ],
        },
        Schedule {
            device: Device::Light(Lights::LivingRoom1),
            changes: &[
                (6 * 60, 1),
                (6 * 60 + 30, 0),
                (17 * 60 + 30, 1),
                (22 * 60 + 30, 0),
            ],
        },
        Schedule {
            device: Device::Light(Lights::LivingRoom2),
            changes: &[
                (6 * 60, 1),
                (6 * 60 + 30, 0),
                (17 * 60 + 30, 1),
                (22 * 60 + 30, 0),
            ],
        },
    ];

    // Fake async function: simulate an external call to get the state at a given time
    pub async fn get_current_state(current_time: u32) -> HashMap<Device, u32> {
        let mut state = HashMap::new();
        thread::sleep(Duration::from_millis(100));
        for schedule in TYPICAL_DAY.iter() {
            // let mut state = None;
            for (new_time, new_state) in schedule.changes {
                if *new_time < current_time {
                    // state = Some(new_state);
                    state.insert(schedule.device, *new_state);
                } else {
                    break;
                }
            }
            if !state.contains_key(&schedule.device) {
                state.insert(schedule.device, schedule.changes.last().unwrap().1);
            }
        }
        state
    }

    /// Fake async function: external call to the device API
    pub async fn change_state(_device: Device, _new_state: u32) {
        // Nothing to do in our fake remote server
    }

    /// Fake async function: external call to the device API
    pub async fn get_history(device: Device, _date: u32) -> Vec<(u32, u32)> {
        let Some(schedule) = TYPICAL_DAY.iter().find(|d| d.device == device) else {
            return Vec::new();
        };
        schedule.changes.to_vec()
    }
}

pub fn remote_server_plugin(app: &mut App) {
    app.add_systems(Update, (get_latest_state, check_history_requests))
        .add_observer(forward_state_changes)
        .add_observer(forward_history_request);
}

#[derive(Event)]
pub struct LightStateChange {
    pub light: Lights,
    pub on: bool,
}

#[derive(Event)]
pub struct ManualLightStateChange {
    pub light: Lights,
    pub on: bool,
}

#[derive(Event)]
pub struct LightHistoryRequest {
    pub light: Lights,
}

fn get_latest_state(
    mut commands: Commands,
    date: Res<Date>,
    mut previous_states: Local<HashMap<Device, u32>>,
    mut task: Local<Option<Receiver<HashMap<Device, u32>>>>,
) {
    let current_time = date.current_date.hour() * 60 + date.current_date.minute();
    if let Some(running_task) = task.take() {
        if let Ok(done) = running_task.try_recv() {
            for (device, state) in done {
                if previous_states.get(&device) != Some(&state) {
                    previous_states.insert(device, state);
                    match device {
                        Device::Light(light) => commands.trigger(LightStateChange {
                            light,
                            on: state == 1,
                        }),
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
            let state = remote_server_internals::get_current_state(current_time).await;
            let _ = sender.send(state);
        })
        .detach();
}

fn forward_state_changes(event: On<ManualLightStateChange>, mut commands: Commands) {
    let light = event.light;
    let on = event.on;
    IoTaskPool::get()
        .spawn(async move {
            remote_server_internals::change_state(Device::Light(light), if on { 1 } else { 0 })
                .await;
        })
        .detach();
    commands.trigger(LightStateChange {
        light: event.light,
        on: event.on,
    });
}

fn forward_history_request(
    event: On<LightHistoryRequest>,
    date: Res<Date>,
    mut commands: Commands,
) {
    let (sender, receiver) = channel::bounded(1);
    let event_light = event.light;
    let time = date.current_date.hour() * 60 + date.current_date.minute();
    IoTaskPool::get()
        .spawn(async move {
            let _ = sender
                .send(remote_server_internals::get_history(Device::Light(event_light), time).await);
        })
        .detach();
    commands.spawn(DeviceHistoryRequest {
        device: Device::Light(event.light),
        task: receiver,
    });
}

#[derive(Component)]
struct DeviceHistoryRequest {
    device: Device,
    task: Receiver<Vec<(u32, u32)>>,
}

#[derive(Event)]
pub struct LightHistory {
    pub light: Lights,
    pub history: Vec<(u32, u32)>,
}

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
