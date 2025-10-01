use bevy::prelude::*;

mod menu;
mod splash;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Workshop".into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<ApplicationState>()
        .add_plugins((splash::splash_plugin, menu::menu_plugin))
        .run();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
enum ApplicationState {
    #[default]
    Splash,
    Menu,
}
