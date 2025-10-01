#![allow(unused_imports)]

use bevy::{
    feathers::{FeathersPlugins, dark_theme::create_dark_theme, theme::UiTheme},
    prelude::*,
};

use crate::{
    lights::lights_plugin, natural_time::natural_time_plugin, remote_server::remote_server_plugin,
};

mod lights;
mod natural_time;
mod remote_server;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FeathersPlugins))
        .insert_resource(UiTheme(create_dark_theme()))
        .add_plugins((natural_time_plugin, lights_plugin, remote_server_plugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 20., 1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn(SceneRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("1np-simple.glb")),
    ));

    // Fake ceiling to block light
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(12.4, 0.1, 11.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::linear_rgba(0.0, 0.0, 0.0, 0.1),
            alpha_mode: AlphaMode::Blend,
            reflectance: 0.0,
            ..default()
        })),
        Transform::from_xyz(0.0, 2.7, -0.4),
        Pickable::IGNORE,
    ));
}
