use bevy::{color::palettes::tailwind, prelude::*};

use crate::ApplicationState;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(ApplicationState::Menu), display_title)
        .add_systems(Update, (button_system, change_me));
}

fn display_title(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                children![
                    (
                        Text::new("Bevy Workshop"),
                        TextFont {
                            font_size: 100.0,
                            ..default()
                        },
                        ChangeMe
                    ),
                    (
                        Text::new("EuroRust 2025"),
                        TextFont {
                            font_size: 70.0,
                            ..default()
                        },
                    ),
                ],
            ),
            (
                Node {
                    align_items: AlignItems::Center,
                    width: Val::Percent(60.0),
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                children![
                    (
                        Button,
                        Node {
                            border: UiRect::all(Val::Px(10.0)),
                            padding: UiRect::all(Val::Px(10.0)),
                            width: Val::Px(200.0),
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        BackgroundColor(tailwind::GREEN_600.into()),
                        BorderColor::all(tailwind::GREEN_800),
                        BorderRadius::all(Val::Px(10.0)),
                        children![(
                            Text::new("New"),
                            TextFont {
                                font_size: 50.0,
                                ..default()
                            },
                        )],
                        MenuButton::New,
                    ),
                    (
                        Button,
                        Node {
                            border: UiRect::all(Val::Px(10.0)),
                            padding: UiRect::all(Val::Px(10.0)),
                            width: Val::Px(200.0),
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        BackgroundColor(tailwind::BLUE_600.into()),
                        BorderColor::all(tailwind::BLUE_800),
                        BorderRadius::all(Val::Px(10.0)),
                        children![(
                            Text::new("Open"),
                            TextFont {
                                font_size: 50.0,
                                ..default()
                            },
                        )],
                        MenuButton::Open,
                    ),
                    (
                        Button,
                        Node {
                            border: UiRect::all(Val::Px(10.0)),
                            padding: UiRect::all(Val::Px(10.0)),
                            width: Val::Px(200.0),
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        BackgroundColor(tailwind::RED_600.into()),
                        BorderColor::all(tailwind::RED_800),
                        BorderRadius::all(Val::Px(10.0)),
                        children![(
                            Text::new("Exit"),
                            TextFont {
                                font_size: 50.0,
                                ..default()
                            },
                        )],
                        MenuButton::Exit,
                    ),
                ]
            )
        ],
        DespawnOnExit(ApplicationState::Menu),
    ));
}

#[derive(Component)]
enum MenuButton {
    New,
    Open,
    Exit,
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &MenuButton,
        ),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut color, mut border_color, menu_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match menu_button {
                MenuButton::New => {
                    *color = tailwind::GREEN_200.into();
                    *border_color = BorderColor::all(tailwind::GREEN_400);
                }
                MenuButton::Open => {
                    *color = tailwind::BLUE_200.into();
                    *border_color = BorderColor::all(tailwind::BLUE_400);
                }
                MenuButton::Exit => {
                    *color = tailwind::RED_200.into();
                    *border_color = BorderColor::all(tailwind::RED_400);
                }
            },
            Interaction::Hovered => match menu_button {
                MenuButton::New => {
                    *color = tailwind::GREEN_400.into();
                    *border_color = BorderColor::all(tailwind::GREEN_600);
                }
                MenuButton::Open => {
                    *color = tailwind::BLUE_400.into();
                    *border_color = BorderColor::all(tailwind::BLUE_600);
                }
                MenuButton::Exit => {
                    *color = tailwind::RED_400.into();
                    *border_color = BorderColor::all(tailwind::RED_600);
                }
            },
            Interaction::None => match menu_button {
                MenuButton::New => {
                    *color = tailwind::GREEN_600.into();
                    *border_color = BorderColor::all(tailwind::GREEN_800);
                }
                MenuButton::Open => {
                    *color = tailwind::BLUE_600.into();
                    *border_color = BorderColor::all(tailwind::BLUE_800);
                }
                MenuButton::Exit => {
                    *color = tailwind::RED_600.into();
                    *border_color = BorderColor::all(tailwind::RED_800);
                }
            },
        }
    }
}

#[derive(Component)]
struct ChangeMe;

fn change_me(mut query: Query<&mut Text, With<ChangeMe>>) {
    for mut text in &mut query {
        **text = "Bevy Workshop".to_string();
    }
}
