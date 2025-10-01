# Home Setup

## Basic Bevy App

We'll start with the basic Bevy app, with a setup system displaying our 3D scene.

```rust,no_run
# extern crate bevy;
use bevy::{
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
# fn setup() {}
```

## Setting up the 3D Scene

For our home automation visualization, we need a 3D camera looking down at the home model.

```rust
# extern crate bevy;
# use bevy::prelude::*;
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 20., 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
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
```

### Adding a Camera

The camera is positioned at `(0.0, 20.0, 1.0)` - high above the scene - and looks down at the center of the home.

### Loading the Home Model

The home model is loaded from a GLTF file using the [`AssetServer` resource](https://docs.rs/bevy/0.17.2/bevy/asset/struct.AssetServer.html) and the [`SceneRoot` component](https://docs.rs/bevy/0.17.2/bevy/scene/struct.SceneRoot.html).

When loading a scene, like a glTF file, Bevy will automatically load the scene and add it to the hierarchy under the entity with the [`SceneRoot` component](https://docs.rs/bevy/0.17.2/bevy/scene/struct.SceneRoot.html).

### Creating a Fake Ceiling

To make the lighting more realistic, we add a semi-transparent ceiling that blocks light from above. This is done by spawning a cuboid mesh with a dark, semi-transparent material.

The ceiling is a thin cuboid . The material uses `AlphaMode::Blend` to make it semi-transparent, and `Pickable::IGNORE` ensures it won't interfere with mouse interactions in the scene.

As light transmission is more expensive to compute, this is enabled separately from transparency, so our ceiling will be see through but will not let outside light through.

## Spawning the lights

We need to show where the indoor lights are in our house. As it's a simplified model, we'll use the same light everywhere.

To display a basic shape in 3D, we can use the [`Mesh3d` component](https://docs.rs/bevy/0.17.2/bevy/mesh/struct.Mesh3d.html) and the [`MeshMaterial3d` component](https://docs.rs/bevy/0.17.2/bevy/pbr/struct.MeshMaterial3d.html).

To add a light, we can use the [`PointLight` component](https://docs.rs/bevy/0.17.2/bevy/light/struct.PointLight.html). Setting its `intensity` to `0.0` will turn it off.

We can use an helper function to spawn all our lights in the same way:

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::{light::NotShadowCaster, prelude::*};
# #[derive(Component)]
# struct Light;
fn spawn_lights(
    commands: &mut Commands,
    position: Vec2,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    light: Light,
) {
    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(position.extend(2.5).xzy()),
            PointLight {
                shadows_enabled: true,
                intensity: 0.0,
                ..default()
            },
            NotShadowCaster,
            light,
        ));
}
```

We can now use this function to spawn all our lights. We'll also add an enum component to be able to identify each light.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::{color::palettes::tailwind, light::NotShadowCaster, prelude::*};
# fn spawn_lights(
#     commands: &mut Commands,
#     position: Vec2,
#     mesh: Handle<Mesh>,
#     material: Handle<StandardMaterial>,
#     light: Light,
# ) {}
#[derive(Component, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Light {
    Bedroom1,
    Bedroom2,
    Bathroom1,
    Bathroom2,
    Toilets,
    LivingRoom1,
    LivingRoom2,
    Kitchen,
    Hall,
    Hallway,
}

fn light_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let light_mesh = meshes.add(Sphere::new(0.15));
    let light_material = materials.add(StandardMaterial {
        base_color: tailwind::YELLOW_400.into(),
        unlit: true,
        ..default()
    });

    [
        (vec2(4.5, 2.0), Light::Bedroom1),
        (vec2(-3.5, 2.0), Light::LivingRoom1),
        (vec2(-1.0, 2.0), Light::LivingRoom2),
        (vec2(-4.0, -3.0), Light::Bedroom2),
        (vec2(0.0, -3.0), Light::Kitchen),
        (vec2(-2.0, -4.0), Light::Toilets),
        (vec2(-2.0, -2.0), Light::Bathroom2),
        (vec2(1.5, 3.5), Light::Bathroom1),
        (vec2(1.25, 0.75), Light::Hallway),
        (vec2(4.0, -3.0), Light::Hall),
    ]
    .into_iter()
    .for_each(|(position, name)| {
        spawn_lights(
            &mut commands,
            position,
            light_mesh.clone(),
            light_material.clone(),
            name,
        );
    });
}
```
