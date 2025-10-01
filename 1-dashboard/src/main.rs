use bevy::prelude::*;

use crate::{
    battery::battery_plugin, music::music_plugin, odometer::odometer_plugin, speed::speed_plugin,
    turn::turn_plugin,
};

mod battery;
mod music;
mod odometer;
mod speed;
mod turn;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            turn_plugin,
            speed_plugin,
            battery_plugin,
            odometer_plugin,
            music_plugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
