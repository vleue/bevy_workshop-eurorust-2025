# The Application

The initial goal is to open a window using Bevy!

## Empty Application

Let's start a new project with Bevy

```sh
cargo new bevy_workshop-eurorust-2025
cd bevy_workshop-eurorust-2025
```

We can add Bevy 0.17 with the default features enabled:

```sh
cargo add bevy@0.17

Updating crates.io index
  Adding bevy v0.17 to dependencies
         Features as of v0.17.1:
         41 activated features
         68 deactivated features
Updating crates.io index
 Locking 468 packages to latest Rust 1.86.0 compatible versions
```

Bevy exposes a lot of features, 133 for the 0.17! [The full list of features is available in the documentation](https://docs.rs/bevy/0.17.2/bevy/#cargo-features). It is important to disable default features and only enable the ones you need. This will improve performance, compilation time and reduce binary size.

This is the most basic Bevy application. It will exit immediately upon running and perform no actions.

```rust
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new().run();
}
```

## Default Bevy Plugins

Default plugins are added to handle windowing, rendering, input, audio, and more. This application opens a window and then does nothing.

```rust,no_run
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
```

Plugins can be configured; in this example, we set a custom title for the window.

```rust,no_run
# extern crate bevy;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Workshop".into(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
```
