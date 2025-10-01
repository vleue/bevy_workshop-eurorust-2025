# Turn Signals

## Basic Bevy App

We'll start with the basic Bevy app, and a `Camera2d`.

For 2d, it's ofen useful to set the `ImagePlugin` to default to nearest sampling to avoid blurry images.

```rust,no_run
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```

## Displaying an image

Displaying an image is done with the [`Sprite` component](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.Sprite.html), and loading the file is done with the [`AssetServer` resource](https://docs.rs/bevy/0.17.2/bevy/asset/struct.AssetServer.html).

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::prelude::*;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn(
        Sprite::from_image(
            asset_server.load("signals/signal_left.png"),
        )
    );
}
```

Adding this system to a `Startup` schedule will display the image at the center of the screen.

Changing the position of the image can be done by adding a [`Transform` component](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.Transform.html) to the entity.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::prelude::*;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn((
        Sprite::from_image(
            asset_server.load("signals/signal_left.png"),
        ),
        Transform::from_xyz(-50.0, 0.0, 0.0),
    ));
}
```

This will display the image to the left of the center of the screen.

We also want to display the right signal on the right side, and have both images closer to the top of the screen. We could individually set the `Transform` of both sprites, but it's better to use hierarchy for that, set the transform of the group, then have each signal be a child of the group and placed relative to it.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::prelude::*;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn((
        Transform::from_xyz(0.0, 300.0, 0.0),
        children![
            (
                Sprite::from_image(
                    asset_server.load("signals/signal_left.png"),
                ),
                Transform::from_xyz(-50.0, 0.0, 0.0),
            ),
            (
                Sprite::from_image(
                    asset_server.load("signals/signal_right.png"),
                ),
                Transform::from_xyz(50.0, 0.0, 0.0),
            )
        ],
    ));
}
```

The turn signal plugin is adding this `setup` system to the `Startup` schedule of the app:

```rust
# extern crate bevy;
# use bevy::prelude::*;
# fn setup() {}
pub fn turn_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}
```

## Turning off the signals

Right now the signals are brightly colored, but we want them to be dim when they are not active.

Instead of calling the [`Sprite::from_image`](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.Sprite.html#method.from_image) helper function to create a sprite, we can set each field of the struct to the value we need, that gives us more configurability. In this case we can set the [`image` field](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.Sprite.html#structfield.image) to the image, and the [`color` field](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.Sprite.html#structfield.color) to tint the image darker.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::{color::palettes, prelude::*};
# fn setup(asset_server: Res<AssetServer>) {
Sprite {
    image: asset_server.load("signals/signal_left.png"),
    color: palettes::tailwind::GRAY_800.into(),
    ..default()
}
# ;
# }
```

We'll also add a marker component to be able to tag the turn signals group.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::prelude::*;
#[derive(Component)]
struct TurnSignalIndicator;
```

A marker component is a [Zero Sized Type](https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts). This means that it doesn't take up any space in memory, and can be used to tag entities without adding any additional data to them, making it easy to write queries targeting them.

Our `setup` system becomes:

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::{color::palettes, prelude::*};
# #[derive(Component)]
# struct TurnSignalIndicator;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Transform::from_xyz(0.0, 300.0, 0.0),
        Visibility::Visible, // Needed to remove a warning in Bevy
        TurnSignalIndicator,
        children![
            (
                Sprite {
                    image: asset_server.load("signals/signal_left.png"),
                    color: palettes::tailwind::GRAY_800.into(),
                    ..default()
                },
                Transform::from_xyz(-50.0, 0.0, 0.0),
            ),
            (
                Sprite {
                    image: asset_server.load("signals/signal_right.png"),
                    color: palettes::tailwind::GRAY_800.into(),
                    ..default()
                },
                Transform::from_xyz(50.0, 0.0, 0.0),
            )
        ],
    ));
}
```
