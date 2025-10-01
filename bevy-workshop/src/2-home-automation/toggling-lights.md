# Toggling Lights

## Communication with the Remote Server

We will use events to communicate with the remote server. A first event will be triggered when the user requests the light to change state, an observer on that event will call the remote server in an async task. Once done, the async task will trigger a second event to notify the UI that the light state has changed.

```ignore
   user request light change
              |
              v
 ManualLightStateChange event
              |
              v
observer call the remote server --async--> remote server state change
                                                      |
                                                      v
                                          ServerLightStateChange event
                                                      |
                                                      v
                                             observer toggle light
```

What is often done to improve reactivity is to manually send the `ServerLightStateChange` event without waiting for the response form the remote server, as if it already succeeded.

```ignore
   user request light change
              |
              v
 ManualLightStateChange event
              |
              v
observer call the remote server --async--> remote server state change
              |
              v
  ServerLightStateChange event
              |
              v
     observer toggle light
```

## Communication Implementation

We can declare our two events:

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::prelude::*;
# struct Light;
#[derive(Event)]
pub struct ManualLightStateChange {
    pub light: Light,
    pub on: bool,
}

#[derive(Event)]
pub struct ServerLightStateChange {
    pub light: Light,
    pub on: bool,
}
```

And our observer for the `ManualLightStateChange` event:

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::{prelude::*, tasks::IoTaskPool};
# #[derive(Event)]
# pub struct ManualLightStateChange {
#     pub light: Light,
#     pub on: bool,
# }
# #[derive(Event)]
# pub struct ServerLightStateChange {
#     pub light: Light,
#     pub on: bool,
# }
# mod internal {
#     #[derive(Clone, Copy)]
#     pub struct Light;
#     pub enum Device { Light(Light)}
#     pub async fn change_state(device: Device, on: bool) {}
# }
# use internal::*;
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
```

This observer:

- receives the event
- starts an async task to change the state of the light
- triggers a response event

We are using the [`IoTaskPool`](https://docs.rs/bevy/0.17.2/bevy/tasks/struct.IoTaskPool.html) to spawn an async task. If the task is dropped, it could be cancelled, unless it has been detached first.

Bevy exposes three different [`TaskPool`s](https://docs.rs/bevy/0.17.2/bevy/tasks/struct.TaskPool.html):

- [`IoTaskPool`](https://docs.rs/bevy/0.17.2/bevy/tasks/struct.IoTaskPool.html): used by the asset server to load files, recommended for IO-bound tasks that can wait on the OS.
- [`ComputeTaskPool`](https://docs.rs/bevy/0.17.2/bevy/tasks/struct.ComputeTaskPool.html): used for task that must finish before the next frame is rendered. Bevy uses this pool for parallelism inside a system. Tasks in this pool must not block for too long or the application will freeze.
- [`AsyncComputeTaskPool`](https://docs.rs/bevy/0.17.2/bevy/tasks/struct.AsyncComputeTaskPool.html): used for tasks that can take an arbitrary amount of time, such as long-running computations. Not used by Bevy.

## Manually Triggering the Event

The simplest way for the user to trigger the event with our current UI is to have them click on the light they want to toggle. This is easy to enable in Bevy by adding the [`MeshPickingPlugin`](https://docs.rs/bevy/0.17.2/bevy/picking/mesh_picking/struct.MeshPickingPlugin.html), and meshes will now receives [`Pointer` events](https://docs.rs/bevy/0.17.2/bevy/picking/events/struct.Pointer.html) when being interacted with by a mouse pointer.

Other picking backends available are [`UiPickingPlugin`](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.UiPickingPlugin.html)
and [`SpritePickingPlugin`](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.SpritePickingPlugin.html), for UI and 2d sprites.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::prelude::*;
# #[derive(Event)]
# pub struct ManualLightStateChange {
#     pub light: Light,
#     pub on: bool,
# }
# #[derive(Component, Clone, Copy)]
# struct Light;
fn toggle_light(
    event: On<Pointer<Click>>,
    mut commands: Commands,
    light: Query<(&Light, &PointLight)>,
) {
    let (light, point_light) = light.get(event.entity).unwrap();
    commands.trigger(ManualLightStateChange {
        light: *light,
        on: point_light.intensity == 0.0,
    });
}
```

[`Pointer` events](https://docs.rs/bevy/0.17.2/bevy/picking/events/struct.Pointer.html) are [`EntityEvent`](https://docs.rs/bevy/0.17.2/bevy/ecs/event/trait.EntityEvent.html), which means they target a specific entity and must be observed on it.

We must change the `` function to add the observer:

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::{light::NotShadowCaster, prelude::*};
# #[derive(Component)]
# struct Light;
# fn toggle_light(event: On<Pointer<Click>>) {}
fn spawn_lights(
    commands: &mut Commands,
    position: Vec2,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    light: Light,
) {
    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(position.extend(2.5).xzy()),
            PointLight {
                shadows_enabled: true,
                intensity: 0.0,
                ..default()
            },
            NotShadowCaster,
            light,
        ))
        .observe(toggle_light);
}
```

## Reacting to the server response

When receiving the `ServerLightStateChange` event, we should find the matching light thanks to its `Light` component, and change its intensity.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::prelude::*;
# #[derive(Event)]
# pub struct ServerLightStateChange {
#     pub light: Light,
#     pub on: bool,
# }
# #[derive(Component, PartialEq, Eq)]
# struct Light;
fn on_light_state_changed(
    message: On<ServerLightStateChange>,
    mut light: Query<(&mut PointLight, &Light)>,
) {
    for (mut light, light_data) in light.iter_mut() {
        if *light_data == message.light {
            if message.on {
                light.intensity = 300000.0;
            } else {
                light.intensity = 0.0;
            }
        }
    }
}
```
