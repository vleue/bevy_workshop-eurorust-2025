use bevy::{
    camera::{Viewport, visibility::RenderLayers},
    prelude::*,
    render::view::NoIndirectDrawing,
    window::WindowResized,
};

use bevy_egui::{
    EguiContexts, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext, egui,
};
use bevy_pointcloud::{point_cloud_material::PointCloudMaterial, render::PointCloudRenderMode};

use crate::{Play, PointCloudDataset};

pub fn cameras_plugin(app: &mut App) {
    app.add_plugins(EguiPlugin::default())
        .add_systems(EguiPrimaryContextPass, controls)
        .add_systems(Startup, setup)
        .add_systems(Update, set_camera_viewports);
}

fn controls(
    mut contexts: EguiContexts,
    mut play: ResMut<Play>,
    mut point_cloud_materials: ResMut<Assets<PointCloudMaterial>>,
    dataset: Res<PointCloudDataset>,
) -> Result {
    let mut point_size_fpv = {
        let material = point_cloud_materials.get(&dataset.material_fpv).unwrap();
        material.point_size
    };
    let mut point_size_tp = {
        let material = point_cloud_materials.get(&dataset.material_tp).unwrap();
        material.point_size
    };

    let original_point_size_fpv = point_size_fpv;
    let original_point_size_tp = point_size_tp;
    egui::TopBottomPanel::top("Controls").show(contexts.ctx_mut()?, |ui| {
        ui.horizontal(|ui| {
            ui.add(
                egui::Slider::new(
                    &mut play.current_frame,
                    0..=(dataset.point_clouds.len().max(1) - 1),
                )
                .text("Frame"),
            );
            if ui
                .button(if play.playing { "Pause" } else { "Play" })
                .clicked()
            {
                play.playing = !play.playing;
            }
        });
        ui.horizontal(|ui| {
            ui.add(egui::Slider::new(&mut point_size_fpv, 10.0..=150.0).text("Point Size FPV"));
            ui.add(egui::Slider::new(&mut point_size_tp, 50.0..=500.0).text("Point Size TP"));
        });
    });
    if original_point_size_fpv != point_size_fpv {
        let material = point_cloud_materials
            .get_mut(&dataset.material_fpv)
            .unwrap();
        material.point_size = point_size_fpv;
    }
    if original_point_size_tp != point_size_tp {
        let material = point_cloud_materials.get_mut(&dataset.material_tp).unwrap();
        material.point_size = point_size_tp;
    }
    Ok(())
}

fn setup(mut commands: Commands, mut egui_global_settings: ResMut<EguiGlobalSettings>) {
    egui_global_settings.auto_create_primary_context = false;
    commands.spawn((
        PrimaryEguiContext,
        Camera2d,
        Camera {
            order: 0,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::FixedHorizontal {
                viewport_width: 540.0,
            },
            ..OrthographicProjection::default_2d()
        }),
        CameraPosition {
            cpos: UVec2::new(0, 1),
            ratio: UVec2::new(2, 2),
        },
    ));

    for (index, (layer, camera_pos, ratio)) in [
        (
            RenderLayers::layer(1),
            Vec3::new(-1.0, 0.25, 0.0),
            UVec2::new(2, 2),
        ), // First Person View
        (
            RenderLayers::layer(2),
            Vec3::new(-1.0, 100.0, 0.0),
            UVec2::new(2, 1),
        ), // Top Down View
    ]
    .into_iter()
    .enumerate()
    {
        commands.spawn((
            Camera3d::default(),
            Transform::from_translation(camera_pos).looking_at(Vec3::ZERO, Vec3::Y),
            Camera {
                // Renders cameras with different priorities to prevent ambiguities
                order: index as isize + 1,
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            layer,
            CameraPosition {
                cpos: UVec2::new((index % 2) as u32, (index / 2) as u32),
                ratio,
            },
            NoIndirectDrawing,
            Msaa::Off,
            PointCloudRenderMode {
                use_edl: false,
                ..default()
            },
        ));
    }
}

#[derive(Component)]
struct CameraPosition {
    cpos: UVec2,
    ratio: UVec2,
}

fn set_camera_viewports(
    windows: Query<&Window>,
    mut window_resized_reader: MessageReader<WindowResized>,
    mut cameras: Query<(&CameraPosition, &mut Camera)>,
) {
    for window_resized in window_resized_reader.read() {
        let window = windows.get(window_resized.window).unwrap();

        for (camera_position, mut camera) in &mut cameras {
            let size = window.physical_size() / camera_position.ratio;
            let offset = window.physical_size() / 2;
            camera.viewport = Some(Viewport {
                physical_position: camera_position.cpos * offset,
                physical_size: size,
                ..default()
            });
        }
    }
}
