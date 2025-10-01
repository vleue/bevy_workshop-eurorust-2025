# States

Bevy provides an abstraction and helpers to control systems that execute based on the application's state, aptly named "states."

```rust,no_run
# extern crate bevy;
# extern crate bevy_ecs;
# extern crate bevy_state;
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
        .init_state::<ApplicationState>()
        .add_systems(OnEnter(ApplicationState::Splash), display_title)
        .add_systems(Update, switch_to_menu.run_if(in_state(ApplicationState::Splash)))
        .run();
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
enum ApplicationState {
    #[default]
    Splash,
    Menu,
}

#[derive(Resource)]
struct SplashScreenTimer(Timer);

fn display_title(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Text::new("Bevy Workshop"),
                TextFont {
                    font_size: 130.0,
                    ..default()
                },
            ),
            (
                Text::new("EuroRust 2025"),
                TextFont {
                    font_size: 100.0,
                    ..default()
                },
            )
        ],
        DespawnOnExit(ApplicationState::Splash),
    ));

    commands.insert_resource(SplashScreenTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn switch_to_menu(
    mut next: ResMut<NextState<ApplicationState>>,
    mut timer: ResMut<SplashScreenTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        next.set(ApplicationState::Menu);
    }
}
```

## State-Based Schedules

When using states, additional schedules are available: `OnEnter`, `OnExit`, and `OnTransition`.

## Changing States

States can be changed using the `NextState` resource.

## State-Scoped Entities

By adding the `DespawnOnExit` component, all entities and their hierarchy marked with this component will be despawned when exiting the state.
