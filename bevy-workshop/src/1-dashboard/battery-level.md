# Battery Level

Our car can't run forever, so we need to manage the battery level.

## Manage the Battery

Let's add a resource to keep track of the battery level.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# extern crate bevy_render;
# extern crate bevy_asset;
# extern crate bevy_reflect;
# extern crate encase;
# use bevy::prelude::*;
#[derive(Resource)]
pub struct BatteryLevel(f32);
```

We'll also need a system to update the battery level depending on the car's speed.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# extern crate bevy_render;
# extern crate bevy_asset;
# extern crate bevy_reflect;
# extern crate encase;
# use bevy::prelude::*;
# #[derive(Resource)]
# pub struct BatteryLevel(f32);
# #[derive(Resource)]
# pub struct Speed(f32);
fn update_battery(
    mut battery: ResMut<BatteryLevel>,
    speed: Res<Speed>,
    time: Res<Time>,
) {
    battery.0 = (battery.0 - (time.delta_secs() * (speed.0.powf(2.0)) / 1500.0)).max(0.0);
    if battery.0 <= 0.0 {
        println!("Battery is empty!");
    }
}
```

And to add those to our app, a simple plugin:

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# extern crate bevy_render;
# extern crate bevy_asset;
# extern crate bevy_reflect;
# extern crate encase;
# use bevy::prelude::*;
# #[derive(Resource)]
# pub struct BatteryLevel(f32);
# fn update_battery() {}
pub fn battery_plugin(app: &mut App) {
    app.add_systems(Update, update_battery)
        .insert_resource(BatteryLevel(100.0));
}
```

## Display the Battery Level

The driver should be able to know the battery level. We could display it as text, but let's make a nicer indicator!

We'll write a simple shader. Let's start by defining the material we'll use. In this case we just need to send a `f32` (the battery level) to the shader, it doesn't need anything else as data to be displayed.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# extern crate bevy_render;
# extern crate bevy_asset;
# extern crate bevy_reflect;
# extern crate encase;
# use bevy::{
#     prelude::*,
#     render::render_resource::{AsBindGroup, ShaderType},
#     shader::ShaderRef,
#     sprite_render::{Material2d, Material2dPlugin},
# };
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
```

The first step is to define which data we'll send to the GPU. In our case we just need a `f32`, so it's a simple struct.

Next is to implement the `Material2d` trait for our material. This trait lets us define the shader used to render the material.

The `Material2d` trait needs the `Asset` trait which needs the `TypePath` trait, and those can be directly derived. It also needs the `AsBindGroup` trait which can be derived, but needs some attributes, in this case `#[uniform(0, BatteryMaterial)]`

- `uniform` is the address space: <https://www.w3.org/TR/WGSL/#address-spaces-uniform>
- `0` is the binding index
- `BatteryMaterial` is the Rust struct matching the memory representation on the GPU

Our shader will be written in WGSL.

```wgsl
struct Material {
    level: f32,
}

@group(2) @binding(0)
var<uniform> material: Material;
```

- The struct `Material` must have the same memory representation as the Rust struct `BatteryMaterial`.
- The `var` must have the same address space and binding as the attribute to the `AsBindGroup` derive. The group 2 is the one used by default for user defined data.

```wgsl
#import bevy_sprite::{
    mesh2d_vertex_output::VertexOutput,
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    if abs(in.uv.x - 0.5) < 0.005 {
        // if we are close to the center of the mesh, draw a line in a lighter blue
        return vec4(0.0, 0.7, 0.8, 1.0);
    }

    if in.uv.x < material.level {
        // if we are at a value that's less than the battery level,
        // fill with a color that is between red and blue,
        // closer to red as the level is lower
        return mix(
            vec4(0.9, 0.1, 0.1, 1.0),
            vec4(0.1, 0.4, 0.9, 1.0),
            smoothstep(0.1, 0.75, material.level)
        );
    } else {
        // otherwise just return black
        return vec4(0.0, 0.0, 0.0, 0.0);
    }
}
```

- Import syntax is an extension of WGSL from Bevy. It lets us reuse types and functions between shaders.
- `@fragment` defines the method called by the fragment shader. It takes the output of the vertex shader as input. As we didn't define one in this case, it's the default one defined by Bevy. It's return value is a RGBA color as a `vec4<f32>`.

All that is left is using our new material! For that we'll need to add `Material2dPlugin::<BatteryMaterial>::default()` as a plugin to the application, a mesh using the material and a system to update the material level.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# extern crate bevy_render;
# extern crate bevy_asset;
# extern crate bevy_reflect;
# extern crate encase;
# use bevy::{
#     prelude::*,
#     render::render_resource::{AsBindGroup, ShaderType},
#     shader::ShaderRef,
#     sprite_render::{Material2d, Material2dPlugin},
# };
# #[derive(Asset, TypePath, AsBindGroup, ShaderType, Clone)]
# #[uniform(0, BatteryMaterial)]
# struct BatteryMaterial {
#     level: f32,
# }
# impl<'a> From<&'a BatteryMaterial> for BatteryMaterial {
#     fn from(material: &'a BatteryMaterial) -> Self {
#         material.clone()
#     }
# }
# impl Material2d for BatteryMaterial {
#     fn fragment_shader() -> ShaderRef {
#         "shaders/battery_bar.wgsl".into()
#     }
# }
# #[derive(Resource)]
# struct BatteryLevel(f32);
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BatteryMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(300.0, 50.0))),
        MeshMaterial2d(materials.add(BatteryMaterial { level: 1.0 })),
    ));
}

fn display_battery(
    battery: Res<BatteryLevel>,
    material: Single<&MeshMaterial2d<BatteryMaterial>>,
    mut progress_materials: ResMut<Assets<BatteryMaterial>>,
) {
    if battery.is_changed() {
        progress_materials.get_mut(material.id()).unwrap().level = battery.0 / 100.0;
    }
}
```

## Recharge the Battery

The battery is either unplugged, and the car can move, or charging and the car can't move.

We will handle this with a state, to be able to toggle systems depending on the current state.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# extern crate bevy_render;
# extern crate bevy_asset;
# extern crate bevy_reflect;
# extern crate bevy_state;
# extern crate encase;
# use bevy::prelude::*;
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum BatteryStatus {
    Charging,
    #[default]
    Unplugged,
}
```

The existing `update_battery` system should only run when the battery is unplugged, and we're going to add a new system to charge the battery when it's charging. Both systems should switch to the other state when the battery is either full or empty.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# extern crate bevy_render;
# extern crate bevy_asset;
# extern crate bevy_reflect;
# extern crate bevy_state;
# extern crate encase;
# use bevy::{
#     prelude::*,
#     render::render_resource::{AsBindGroup, ShaderType},
#     shader::ShaderRef,
#     sprite_render::{Material2d, Material2dPlugin},
# };
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# pub enum BatteryStatus {
#     Charging,
#     #[default]
#     Unplugged,
# }
# #[derive(Resource)]
# pub struct BatteryLevel(f32);
# #[derive(Resource)]
# pub struct Speed(f32);
# #[derive(Asset, TypePath, AsBindGroup, ShaderType, Clone)]
# #[uniform(0, BatteryMaterial)]
# struct BatteryMaterial {
#     level: f32,
# }
# impl<'a> From<&'a BatteryMaterial> for BatteryMaterial {
#     fn from(material: &'a BatteryMaterial) -> Self {
#         material.clone()
#     }
# }
# impl Material2d for BatteryMaterial {
#     fn fragment_shader() -> ShaderRef {
#         "shaders/battery_bar.wgsl".into()
#     }
# }
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
```

Another change to do is in the speedometer system that updates the speed:

- it should only run in the `BatteryStatus::Unplugged` state
- a new system should set the speed to `0.0` when the battery is charging

<div class="warning">
    You need to update the battery plugin, adding the state and the new systems with the correct conditions.

The `battery_plugin` should now look like this:

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# extern crate bevy_render;
# extern crate bevy_asset;
# extern crate bevy_reflect;
# extern crate bevy_state;
# extern crate encase;
# use bevy::{
#     prelude::*,
#     render::render_resource::{AsBindGroup, ShaderType},
#     shader::ShaderRef,
#     sprite_render::{Material2d, Material2dPlugin},
# };
# fn setup() {}
# fn update_battery() {}
# fn charging_battery() {}
# fn display_battery() {}
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# pub enum BatteryStatus {
#     Charging,
#     #[default]
#     Unplugged,
# }
# #[derive(Resource)]
# pub struct BatteryLevel(f32);
# #[derive(Resource)]
# pub struct Speed(f32);
# #[derive(Asset, TypePath, AsBindGroup, ShaderType, Clone)]
# #[uniform(0, BatteryMaterial)]
# struct BatteryMaterial {
#     level: f32,
# }
# impl<'a> From<&'a BatteryMaterial> for BatteryMaterial {
#     fn from(material: &'a BatteryMaterial) -> Self {
#         material.clone()
#     }
# }
# impl Material2d for BatteryMaterial {
#     fn fragment_shader() -> ShaderRef {
#         "shaders/battery_bar.wgsl".into()
#     }
# }
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
```

</div>

## Display Battery Indicators

Car dashboard should have more indicators! We'll add one for the battery, with three states:

- Charging
- Ok
- Low

Changing the `setup` system to display those indicators:

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# extern crate bevy_render;
# extern crate bevy_asset;
# extern crate bevy_reflect;
# extern crate bevy_state;
# extern crate encase;
# use bevy::{
#     prelude::*,
#     render::render_resource::{AsBindGroup, ShaderType},
#     shader::ShaderRef,
#     sprite_render::{Material2d, Material2dPlugin},
# };
# fn update_battery() {}
# fn charging_battery() {}
# fn display_battery() {}
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# pub enum BatteryStatus {
#     Charging,
#     #[default]
#     Unplugged,
# }
# #[derive(Resource)]
# pub struct BatteryLevel(f32);
# #[derive(Resource)]
# pub struct Speed(f32);
# #[derive(Asset, TypePath, AsBindGroup, ShaderType, Clone)]
# #[uniform(0, BatteryMaterial)]
# struct BatteryMaterial {
#     level: f32,
# }
# impl<'a> From<&'a BatteryMaterial> for BatteryMaterial {
#     fn from(material: &'a BatteryMaterial) -> Self {
#         material.clone()
#     }
# }
# impl Material2d for BatteryMaterial {
#     fn fragment_shader() -> ShaderRef {
#         "shaders/battery_bar.wgsl".into()
#     }
# }
# #[derive(Component)]
# struct BatteryIndicator;
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
```

And we'll update the `display_battery` system to switch the indicator depending on the battery state and level.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# extern crate bevy_render;
# extern crate bevy_asset;
# extern crate bevy_reflect;
# extern crate bevy_state;
# extern crate encase;
# use bevy::{
#     prelude::*,
#     render::render_resource::{AsBindGroup, ShaderType},
#     shader::ShaderRef,
#     sprite_render::{Material2d, Material2dPlugin},
# };
# fn setup() {}
# fn update_battery() {}
# fn charging_battery() {}
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# pub enum BatteryStatus {
#     Charging,
#     #[default]
#     Unplugged,
# }
# #[derive(Resource)]
# pub struct BatteryLevel(f32);
# #[derive(Resource)]
# pub struct Speed(f32);
# #[derive(Asset, TypePath, AsBindGroup, ShaderType, Clone)]
# #[uniform(0, BatteryMaterial)]
# struct BatteryMaterial {
#     level: f32,
# }
# impl<'a> From<&'a BatteryMaterial> for BatteryMaterial {
#     fn from(material: &'a BatteryMaterial) -> Self {
#         material.clone()
#     }
# }
# impl Material2d for BatteryMaterial {
#     fn fragment_shader() -> ShaderRef {
#         "shaders/battery_bar.wgsl".into()
#     }
# }
# #[derive(Component)]
# struct BatteryIndicator;
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
```
