use bevy::{
    color::palettes::{self},
    prelude::*,
};

#[cfg(feature = "battery")]
use crate::battery::BatteryStatus;

pub fn speed_plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                rotate,
                #[cfg(not(feature = "battery"))]
                update_speed,
                #[cfg(feature = "battery")]
                update_speed.run_if(in_state(BatteryStatus::Unplugged)),
                #[cfg(feature = "battery")]
                stop_car.run_if(in_state(BatteryStatus::Charging)),
            ),
        )
        .insert_resource(Speed(0.0));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Transform::from_xyz(-300.0, 0.0, 0.0),
        Visibility::Visible,
        children![
            Sprite {
                image: asset_server.load("speedometer/dial.png"),
                color: palettes::css::GREEN.into(),
                ..default()
            },
            (
                Transform::from_xyz(0.0, -125.0, 0.0).with_scale(Vec3::splat(0.5)),
                Visibility::Visible,
                SpeedometerHand,
                children![(
                    Sprite {
                        image: asset_server.load("speedometer/hand.png"),
                        ..default()
                    },
                    Transform::from_xyz(0.0, 150.0, 0.0),
                ),],
            )
        ],
    ));
}

#[derive(Resource)]
pub struct Speed(pub f32);

#[derive(Component)]
struct SpeedometerHand;

fn rotate(mut transform: Single<&mut Transform, With<SpeedometerHand>>, speed: Res<Speed>) {
    if speed.is_changed() {
        transform.rotation = Quat::from_rotation_z(-speed.0 / 160.0 * 3.0 + 1.5);
    }
}

fn update_speed(mut speed: ResMut<Speed>, input: Res<ButtonInput<KeyCode>>, time: Res<Time>) {
    if input.pressed(KeyCode::Space) {
        speed.0 = (speed.0 + 1.0).min(160.0);
    }
    if speed.0 > 0.0 {
        speed.0.smooth_nudge(&0.0, 0.75, time.delta_secs());
    }
}

#[cfg(feature = "battery")]
fn stop_car(mut speed: ResMut<Speed>, time: Res<Time>) {
    speed.0.smooth_nudge(&0.0, 10.0, time.delta_secs());
}
