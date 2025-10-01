use bevy::prelude::*;

use crate::ApplicationState;

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(ApplicationState::Splash), display_title)
        .add_systems(
            Update,
            switch_to_menu.run_if(in_state(ApplicationState::Splash)),
        );
}

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

#[derive(Resource)]
struct SplashScreenTimer(Timer);

fn switch_to_menu(
    mut next: ResMut<NextState<ApplicationState>>,
    mut timer: ResMut<SplashScreenTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        next.set(ApplicationState::Menu);
    }
}
