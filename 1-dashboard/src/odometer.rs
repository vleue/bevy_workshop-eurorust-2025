use bevy::prelude::*;

use crate::speed::Speed;

pub fn odometer_plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, (update, display))
        .insert_resource(Distance(0.0));
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(-300.0, -200.0, 0.0),
        Text2d::default(),
        Odometer,
    ));
}

#[derive(Component)]
struct Odometer;

#[derive(Resource)]
struct Distance(f32);

fn display(mut text: Single<&mut Text2d, With<Odometer>>, distance: Res<Distance>) {
    if distance.is_changed() {
        text.0 = format!("odometer: {:>5.1}km", distance.0);
    }
}

fn update(mut distance: ResMut<Distance>, time: Res<Time>, speed: Res<Speed>) {
    distance.0 += speed.0 / 60.0 * time.delta_secs();
}
