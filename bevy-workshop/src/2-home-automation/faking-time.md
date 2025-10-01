# Faking Time

This example will use more fake data to simulate actual time passing, and a remote server to fetch real-time data.

Let's start with the time simulation.

## Faking the Current Time

The current time is stored in a resource. Another resource controls the speed at which time passes.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::prelude::*;
#[derive(Resource, Default)]
pub struct Date {
    pub current_time: u32,
}

#[derive(Resource)]
struct Speed(f32);

fn set_date(time: Res<Time>, speed: Res<Speed>, mut date: ResMut<Date>) {
    date.current_time = (date.current_time
        + ((time.delta_secs() * speed.0 / 60.0) as u32).max(if speed.0 == 0.0 { 0 } else { 1 }))
        % (24 * 60);
}
```

## Faking the Sun

We don't want to see the sun sphere in the atmosphere, but we want to simulate the light it provides.

Bevy now supports global illumination through its experimental Solari crate, but it's hardware dependent to be able to use raytracing. You can read more on how it works in the [0.17 release notes](https://bevy.org/news/bevy-0-17/#how-it-works).

For a general solution with lower quality, we'll use the [`DirectionalLight` component](https://docs.rs/bevy/0.17.2/bevy/light/struct.DirectionalLight.html) with shadows enabled, and the [`AmbientLight` resource](https://docs.rs/bevy/0.17.2/bevy/light/struct.AmbientLight.html). A system will change the direction of the light, and the brightness of the ambient light based on the current time.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::prelude::*;
# #[derive(Resource, Default)]
# pub struct Date {
#     pub current_time: u32,
# }
# use std::f32::consts::*;
fn animate_sun_direction(
    date: Res<Date>,
    mut directional_light: Single<&mut Transform, With<DirectionalLight>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    let current_time = (date.current_time as f32) / 60.0;
    directional_light.rotation = Quat::from_rotation_x((current_time - 6.0) / 12.0 * PI + PI);
    ambient_light.brightness = (((current_time - 12.0).abs() - 6.0).min(0.0).abs() + 1.0) * 50.0;
}
```

## Taking Control of Time

For this project UI, we'll use the experimental Bevy Feathers widgets. For that we'll need to add the [`FeathersPlugins` plugin group](https://docs.rs/bevy/0.17.2/bevy/feathers/struct.FeathersPlugins.html), and the [`UITheme` resource](https://docs.rs/bevy/0.17.2/bevy/feathers/theme/struct.UiTheme.html).

```rust,no_run
# extern crate bevy;
use bevy::{
    feathers::{FeathersPlugins, dark_theme::create_dark_theme, theme::UiTheme},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FeathersPlugins))
        .insert_resource(UiTheme(create_dark_theme()))
        .add_systems(Startup, setup)
        .run();
}
# fn setup() {}
```

The UI is added in <https://github.com/vleue/bevy_workshop-eurorust-2025/blob/main/2-home-automation/src/natural_time.rs#L26>.
