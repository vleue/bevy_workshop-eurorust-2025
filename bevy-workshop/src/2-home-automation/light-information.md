# Light Information

## Light Information Panel

Instead of toggling the light on click, we will now use Bevy Feathers widgets to display a panel with the light name, and a toggle switch to turn it on or off.

You can view the panel declaration there: <https://github.com/vleue/bevy_workshop-eurorust-2025/blob/main/2-home-automation/src/lights.rs#L121>

## Selected Light Gizmo

Gizmos are a great way to draw information directly on screen. By default they are in immediate mode, meaning they must be redrawn every frame. If you have a lot of gizmos or they are mostly static, you can use the retained mode.

We'll draw a simple gizmo around the selected light, and make it dynamic to draw attention to it.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::{color::palettes::tailwind, prelude::*};
# #[derive(Component)]
# struct LightPanel(f32, Entity);
fn highlight_light(
    selected_light: Single<&LightPanel>,
    transform: Query<&Transform>,
    mut gizmos: Gizmos,
    time: Res<Time>,
) {
    let transform = transform.get(selected_light.1).unwrap();
    gizmos.sphere(
        transform.to_isometry(),
        0.25 + (time.elapsed_secs() * 10.0).sin() / 25.0,
        tailwind::YELLOW_300,
    );
}
```
