use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
};

use crate::speed::Speed;

pub fn battery_plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<BatteryMaterial>::default())
        .init_state::<BatteryStatus>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_battery.run_if(in_state(BatteryStatus::Unplugged)),
                charging_battery.run_if(in_state(BatteryStatus::Charging)),
                display_battery,
            ),
        )
        .insert_resource(BatteryLevel(100.0));
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
pub struct BatteryLevel(f32);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum BatteryStatus {
    Charging,
    #[default]
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
                Sprite::from_image(asset_server.load("signals/battery_low.png")),
                Visibility::Hidden
            ),
            (
                Sprite::from_image(asset_server.load("signals/battery_ok.png")),
                Visibility::Hidden
            ),
            (
                Mesh2d(meshes.add(Rectangle::new(300.0, 50.0))),
                MeshMaterial2d(materials.add(BatteryMaterial { level: 1.0 })),
                Transform::from_xyz(250.0, 0.0, 0.0),
            )
        ],
    ));
}

#[derive(Component)]
struct BatteryIndicator;

fn update_battery(
    mut battery: ResMut<BatteryLevel>,
    speed: Res<Speed>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<BatteryStatus>>,
) {
    battery.0 = (battery.0 - (time.delta_secs() * (speed.0.powf(2.0)) / 1500.0)).max(0.0);
    if battery.0 <= 0.0 {
        next_state.set(BatteryStatus::Charging);
    }
}

fn charging_battery(
    mut battery: ResMut<BatteryLevel>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<BatteryStatus>>,
) {
    battery.0 = (battery.0 + time.delta_secs() * 10.0).min(100.0);
    if battery.0 >= 100.0 {
        next_state.set(BatteryStatus::Unplugged);
    }
}

fn display_battery(
    battery: Res<BatteryLevel>,
    indicator: Single<&Children, With<BatteryIndicator>>,
    mut visibility: Query<&mut Visibility>,
    material: Single<&MeshMaterial2d<BatteryMaterial>>,
    mut progress_materials: ResMut<Assets<BatteryMaterial>>,
    battery_status: Res<State<BatteryStatus>>,
) {
    if battery.is_changed() {
        progress_materials.get_mut(material.id()).unwrap().level = battery.0 / 100.0;

        match battery_status.get() {
            BatteryStatus::Charging => {
                *visibility.get_mut(indicator[0]).unwrap() = Visibility::Visible;
                *visibility.get_mut(indicator[1]).unwrap() = Visibility::Hidden;
                *visibility.get_mut(indicator[2]).unwrap() = Visibility::Hidden;
            }
            BatteryStatus::Unplugged if battery.0 < 20.0 => {
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
