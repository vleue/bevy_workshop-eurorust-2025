use std::time::Duration;

use bevy::{
    asset::LoadState, camera::visibility::RenderLayers, prelude::*, scene::SceneInstanceReady,
    time::common_conditions::on_timer,
};

use bevy_pointcloud::{
    PointCloudPlugin,
    loader::las::LasLoaderPlugin,
    point_cloud::PointCloud3d,
    point_cloud_material::{PointCloudMaterial, PointCloudMaterial3d},
};

use crate::{Play, PointCloudDataset};

const DATASET: &str = "kitti-2011_09_26_drive_0005_sync";
// const DATASET: &str = "kitti-2011_09_26_drive_0051_sync";
// const DATASET: &str = "kitti-2011_09_26_drive_0091_sync";
// const DATASET: &str = "kitti-2011_09_26_drive_0093_sync";
// const DATASET: &str = "kitti-2011_09_26_drive_0096_sync";
// const DATASET: &str = "kitti-2011_09_29_drive_0071_sync";

pub fn dataset_plugin(app: &mut App) {
    app.add_plugins((PointCloudPlugin, LasLoaderPlugin))
        .add_systems(Startup, load_meshes)
        .add_systems(
            Update,
            load_pointcloud.run_if(on_timer(Duration::from_secs_f32(0.05))),
        );
}

fn load_meshes(
    mut commands: Commands,
    mut point_cloud_materials: ResMut<Assets<PointCloudMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(PointCloudDataset {
        material_fpv: point_cloud_materials.add(PointCloudMaterial { point_size: 50.0 }),
        material_tp: point_cloud_materials.add(PointCloudMaterial { point_size: 200.0 }),
        point_clouds: vec![],
        loaded: false,
    });

    commands
        .spawn((
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("car.glb"))),
            Transform::from_xyz(0.0, -2.0, 0.0).with_scale(Vec3::splat(1.25)),
            RenderLayers::layer(2),
        ))
        .observe(
            |ready: On<SceneInstanceReady>, mut commands: Commands, children: Query<&Children>| {
                for child in children.iter_descendants(ready.entity) {
                    commands.entity(child).insert(RenderLayers::layer(2));
                }
            },
        );

    commands.spawn((PointLight::default(), RenderLayers::layer(2)));
}

#[allow(clippy::too_many_arguments)]
fn load_pointcloud(
    mut commands: Commands,
    existing: Query<Entity, With<PointCloud3d>>,
    existing_sprite: Query<Entity, With<Sprite>>,
    asset_server: Res<AssetServer>,
    mut play: ResMut<Play>,
    mut dataset: ResMut<PointCloudDataset>,
) {
    if !play.playing && !play.is_changed() {
        return;
    }

    if dataset.point_clouds.is_empty() {
        dataset.point_clouds.push(asset_server.load(format!(
            "{}/velodyne/{:0>10}.laz",
            DATASET, play.current_frame
        )));
    }

    let point_cloud = dataset
        .point_clouds
        .get(play.current_frame)
        .unwrap()
        .clone();

    match asset_server.get_load_state(&point_cloud) {
        // Error means we have reach the end of the frames
        Some(LoadState::Failed(_)) => {
            play.current_frame = 0;
            dataset.point_clouds.pop();
            dataset.loaded = true;
        }
        // Loaded, ready to display
        Some(LoadState::Loaded) => {}
        // Still loading
        _ => return,
    }

    for entity in &existing {
        commands.entity(entity).despawn();
    }
    if let Ok(entity) = existing_sprite.single() {
        commands.entity(entity).despawn();
    }

    commands.spawn((
        PointCloud3d(point_cloud.clone()),
        PointCloudMaterial3d(dataset.material_fpv.clone()),
        RenderLayers::layer(1),
    ));
    commands.spawn((
        PointCloud3d(point_cloud.clone()),
        PointCloudMaterial3d(dataset.material_tp.clone()),
        RenderLayers::layer(2),
    ));
    commands.spawn(Sprite::from_image(asset_server.load(format!(
        "{}/image_03/{:0>10}.png",
        DATASET, play.current_frame
    ))));

    if play.playing {
        if dataset.loaded {
            play.current_frame = (play.current_frame + 1) % dataset.point_clouds.len();
        } else {
            play.current_frame += 1;
            dataset.point_clouds.push(asset_server.load(format!(
                "{}/velodyne/{:0>10}.laz",
                DATASET, play.current_frame
            )));
        }
    }
}
