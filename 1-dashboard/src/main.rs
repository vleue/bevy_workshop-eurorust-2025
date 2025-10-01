use bevy::prelude::*;

#[cfg(feature = "battery")]
mod battery;
#[cfg(feature = "radio")]
mod music;
#[cfg(feature = "odometer")]
mod odometer;
#[cfg(feature = "speedometer")]
mod speed;
mod turn;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            turn::turn_plugin,
            #[cfg(feature = "speedometer")]
            speed::speed_plugin,
            #[cfg(feature = "battery")]
            battery::battery_plugin,
            #[cfg(feature = "odometer")]
            odometer::odometer_plugin,
            #[cfg(feature = "radio")]
            music::music_plugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
