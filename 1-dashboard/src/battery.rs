use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
};

use crate::speed::Speed;

pub fn battery_plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<BatteryMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (update_battery, display_battery))
        .insert_resource(Battery {
            level: 100.0,
            status: BatteryStatus::Unplugged,
        });
}

#[derive(Asset, TypePath, AsBindGroup, ShaderType, Clone)]
#[uniform(0, BatteryMaterial)]
struct BatteryMaterial {
    level: f32,
}

impl<'a> From<&'a BatteryMaterial> for BatteryMaterial {
    fn from(material: &'a BatteryMaterial) -> Self {
        material.clone()
    }
}

impl Material2d for BatteryMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/battery_bar.wgsl".into()
    }
}

#[derive(Resource)]
pub struct Battery {
    level: f32,
    pub status: BatteryStatus,
}

pub enum BatteryStatus {
    Charging,
    Unplugged,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BatteryMaterial>>,
) {
    commands.spawn((
        Transform::from_xyz(-500.0, -300.0, 0.0).with_scale(Vec3::splat(0.75)),
        Visibility::Visible,
        BatteryIndicator,
        children![
            (
                Sprite::from_image(asset_server.load("signals/battery_charging.png")),
                Visibility::Hidden
            ),
            (
                Sprite::from_image(asset_server.load("signals/battery_empty.png")),
                Visibility::Hidden
            ),
            (
                Sprite::from_image(asset_server.load("signals/battery_ok.png")),
                Visibility::Hidden
            ),
            (
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(BatteryMaterial { level: 1.0 })),
                Transform::from_xyz(250.0, 0.0, 0.0).with_scale(Vec3::new(300.0, 50.0, 0.0)),
            )
        ],
    ));
}

#[derive(Component)]
struct BatteryIndicator;

fn update_battery(mut battery: ResMut<Battery>, speed: Res<Speed>, time: Res<Time>) {
    match battery.status {
        BatteryStatus::Charging => {
            battery.level = (battery.level + time.delta_secs() * 10.0).min(100.0);
            if battery.level >= 100.0 {
                battery.status = BatteryStatus::Unplugged;
            }
        }
        BatteryStatus::Unplugged => {
            battery.level =
                (battery.level - (time.delta_secs() * (speed.0.powf(2.0)) / 1500.0)).max(0.0);
            if battery.level <= 0.0 {
                battery.status = BatteryStatus::Charging;
            }
        }
    }
}

fn display_battery(
    battery: Res<Battery>,
    indicator: Single<&Children, With<BatteryIndicator>>,
    mut visibility: Query<&mut Visibility>,
    material: Query<&MeshMaterial2d<BatteryMaterial>>,
    mut progress_materials: ResMut<Assets<BatteryMaterial>>,
) {
    if battery.is_changed() {
        progress_materials
            .get_mut(material.get(indicator[3]).unwrap().id())
            .unwrap()
            .level = battery.level / 100.0;

        match battery.status {
            BatteryStatus::Charging => {
                *visibility.get_mut(indicator[0]).unwrap() = Visibility::Visible;
                *visibility.get_mut(indicator[1]).unwrap() = Visibility::Hidden;
                *visibility.get_mut(indicator[2]).unwrap() = Visibility::Hidden;
            }
            BatteryStatus::Unplugged if battery.level < 20.0 => {
                *visibility.get_mut(indicator[0]).unwrap() = Visibility::Hidden;
                *visibility.get_mut(indicator[1]).unwrap() = Visibility::Visible;
                *visibility.get_mut(indicator[2]).unwrap() = Visibility::Hidden;
            }
            BatteryStatus::Unplugged => {
                *visibility.get_mut(indicator[0]).unwrap() = Visibility::Hidden;
                *visibility.get_mut(indicator[1]).unwrap() = Visibility::Hidden;
                *visibility.get_mut(indicator[2]).unwrap() = Visibility::Visible;
            }
        }
    }
}
