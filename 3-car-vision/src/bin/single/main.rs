use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, render::view::NoIndirectDrawing};

use bevy_pointcloud::{
    PointCloudPlugin,
    loader::las::LasLoaderPlugin,
    point_cloud::PointCloud3d,
    point_cloud_material::{PointCloudMaterial, PointCloudMaterial3d},
    render::PointCloudRenderMode,
};
// use car_vision::camera_controller;

mod camera_controller;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PointCloudPlugin,
            LasLoaderPlugin,
            camera_controller::CameraControllerPlugin,
        ))
        .add_systems(Startup, (setup, load_pointcloud))
        .add_systems(PreUpdate, update_material_on_keypress)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        camera_controller::CameraController::default(),
        Camera3d::default(),
        Transform::default(),
        Camera::default(),
        NoIndirectDrawing,
        Msaa::Off,
        PointCloudRenderMode::default(),
    ));
}

fn load_pointcloud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut point_cloud_materials: ResMut<Assets<PointCloudMaterial>>,
) {
    commands.spawn((
        PointCloud3d(asset_server.load("Untitled_Scan_20_56_03.las")),
        PointCloudMaterial3d(point_cloud_materials.add(PointCloudMaterial { point_size: 50.0 })),
        Transform::from_rotation(Quat::from_rotation_x(FRAC_PI_2)),
    ));
}

fn update_material_on_keypress(
    key_input: Res<ButtonInput<KeyCode>>,
    my_material: Query<&PointCloudMaterial3d>,
    mut point_cloud_materials: ResMut<Assets<PointCloudMaterial>>,
    mut point_cloud_render_mode: Query<&mut PointCloudRenderMode>,
) {
    let Ok(my_material) = my_material.single() else {
        return;
    };
    let point_cloud_material = point_cloud_materials.get_mut(&my_material.0).unwrap();

    if key_input.pressed(KeyCode::ArrowLeft) {
        point_cloud_material.point_size += 1.0;
    }
    if key_input.pressed(KeyCode::ArrowRight) {
        point_cloud_material.point_size -= 1.0;
    }
    if key_input.just_pressed(KeyCode::KeyP) {
        for mut pcrm in &mut point_cloud_render_mode {
            pcrm.use_edl = !pcrm.use_edl;
        }
    }
}
