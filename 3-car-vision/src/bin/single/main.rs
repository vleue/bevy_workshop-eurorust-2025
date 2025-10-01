use bevy::{camera::primitives::Aabb, prelude::*, render::view::NoIndirectDrawing};

use bevy_pointcloud::{
    PointCloudPlugin,
    loader::las::LasLoaderPlugin,
    point_cloud::PointCloud3d,
    point_cloud_material::{PointCloudMaterial, PointCloudMaterial3d},
    render::PointCloudRenderMode,
};

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
        .add_systems(PreUpdate, (center_point_cloud, update_material_on_keypress))
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
        PointCloud3d(asset_server.load("sample-pointclouds/lion_takanawa.copc.laz")),
        // PointCloud3d(asset_server.load("sample-pointclouds/Palac_Moszna.laz")),
        // PointCloud3d(asset_server.load("sample-pointclouds/G_Sw_Anny.laz")),
        PointCloudMaterial3d(point_cloud_materials.add(PointCloudMaterial { point_size: 50.0 })),
        // Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
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

#[allow(clippy::type_complexity)]
fn center_point_cloud(
    mut query: Single<(&Aabb, &mut Transform), (With<PointCloud3d>, Changed<Aabb>)>,
) {
    let aabb = query.0;
    let transform = &mut query.1;
    // Center point cloud
    **transform = Transform::from_translation(
        ((-aabb.center) + Vec3A::new(0.0, aabb.half_extents.y, 0.0)).into(),
    );
}
