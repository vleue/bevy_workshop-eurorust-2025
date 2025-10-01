# Multiple Views

## Limiting the Camera to Half the Window

Restricting a camera to part of a window can be done by setting its [viewport](https://docs.rs/bevy/0.17.2/bevy/camera/struct.Camera.html#structfield.viewport). To know the correct size, we will need the [`Window`](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.Window.html).

```rust
# extern crate bevy;
# extern crate bevy_pointcloud;
# use bevy::{camera::Viewport, prelude::*, render::view::NoIndirectDrawing};
# use bevy_pointcloud::{
#     PointCloudPlugin,
#     loader::las::LasLoaderPlugin,
#     point_cloud::PointCloud3d,
#     point_cloud_material::{PointCloudMaterial, PointCloudMaterial3d},
#     render::PointCloudRenderMode,
# };
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut point_cloud_materials: ResMut<Assets<PointCloudMaterial>>,
    window: Single<&Window>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::default(),
        Camera {
            viewport: Some(Viewport {
                physical_position: uvec2(0, 0),
                physical_size: uvec2(window.physical_width() / 2, window.physical_height()),
                ..default()
            }),
            ..default()
        },
        NoIndirectDrawing,
        Msaa::Off,
        PointCloudRenderMode::default(),
        Transform::from_translation(Vec3::new(-1.0, 100.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
```

<div class="warning">

Setting the viewport size and position at Startup won't update it if the window is resized.

For that, you will need a system that reads the [`WindowResized`](https://docs.rs/bevy/0.17.2/bevy/window/struct.WindowResized.html) message, and query the [`Window`](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.Window.html) component to get its physical size, and updating the camera viewport.

</div>

## Adding Another Camera

When spawning multiple cameras, it's important to set different value for their `order` field. Bevy will warn you that rendering can be random if that's not the case.

In our application, each camera will have its own viewport, so there's no ordering ambiguity, but it doesn't hurt to play nice with Bevy.

```rust
# extern crate bevy;
# extern crate bevy_pointcloud;
# use bevy::{camera::Viewport, prelude::*, render::view::NoIndirectDrawing};
# use bevy_pointcloud::{
#     PointCloudPlugin,
#     loader::las::LasLoaderPlugin,
#     point_cloud::PointCloud3d,
#     point_cloud_material::{PointCloudMaterial, PointCloudMaterial3d},
#     render::PointCloudRenderMode,
# };
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut point_cloud_materials: ResMut<Assets<PointCloudMaterial>>,
    window: Single<&Window>,
) {
    // Left side of the window
    commands.spawn((
        Camera3d::default(),
        Transform::default(),
        Camera {
            order: 0,
            viewport: Some(Viewport {
                physical_position: uvec2(0, 0),
                physical_size: uvec2(window.physical_width() / 2, window.physical_height()),
                ..default()
            }),
            ..default()
        },
        NoIndirectDrawing,
        Msaa::Off,
        PointCloudRenderMode::default(),
        Transform::from_xyz(-1.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // right side of the window
    commands.spawn((
        Camera3d::default(),
        Transform::default(),
        Camera {
            order: 1,
            viewport: Some(Viewport {
                physical_position: uvec2(window.physical_width() / 2, 0),
                physical_size: uvec2(window.physical_width() / 2, window.physical_height()),
                ..default()
            }),
            ..default()
        },
        NoIndirectDrawing,
        Msaa::Off,
        PointCloudRenderMode::default(),
        Transform::from_xyz(-1.0, 0.25, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
```

## Displaying Different Things to Each Camera

If you want to display different things to each camera, you can use the [`RenderLayers` component](https://docs.rs/bevy/0.17.2/bevy/camera/visibility/struct.RenderLayers.html). By default, everything is on layer 0. Render layers work as a mask, and a camera will render entities that are on a layer that matches its own.

This can be used to create split screen effects, or in our case to render the point cloud with different point size as the first person view is better with a smaller point size that the top down view.
