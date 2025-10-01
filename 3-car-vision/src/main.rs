use bevy::prelude::*;

use bevy_pointcloud::{point_cloud::PointCloud, point_cloud_material::PointCloudMaterial};

mod cameras;
mod dataset;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((cameras::cameras_plugin, dataset::dataset_plugin))
        .insert_resource(Play {
            current_frame: 0,
            playing: true,
        })
        .run();
}

#[derive(Resource)]
struct Play {
    current_frame: usize,
    playing: bool,
}

#[derive(Resource)]
struct PointCloudDataset {
    material_fpv: Handle<PointCloudMaterial>,
    material_tp: Handle<PointCloudMaterial>,
    point_clouds: Vec<Handle<PointCloud>>,
    loaded: bool,
}
