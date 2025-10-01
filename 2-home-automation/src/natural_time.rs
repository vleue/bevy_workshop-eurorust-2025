use bevy::{
    feathers::{
        controls::{ButtonProps, SliderProps, button, slider},
        rounded_corners::RoundedCorners,
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    prelude::*,
    ui_widgets::*,
};
use chrono::{NaiveDateTime, TimeDelta, Timelike};
use std::f32::consts::*;

pub fn natural_time_plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, (animate_sun_direction, set_date, display_date))
        .init_resource::<Date>()
        .insert_resource(Speed(5000.0));
}

fn setup(mut commands: Commands) {
    commands.spawn((DirectionalLight {
        shadows_enabled: true,
        ..default()
    },));

    commands.spawn((
        Node {
            width: px(300),
            height: px(100),
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::Start,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            ..default()
        },
        ThemeBackgroundColor(tokens::WINDOW_BG),
        children![
            (
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    column_gap: px(8),
                    ..default()
                },
                children![
                    (Text::new("Current Time: "), ThemedText),
                    (Text::new(""), ThemedText, Clock),
                ]
            ),
            (
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    column_gap: px(1),
                    ..default()
                },
                children![
                    (
                        button(
                            ButtonProps {
                                corners: RoundedCorners::Left,
                                ..default()
                            },
                            (),
                            Spawn((Text::new("Slower"), ThemedText))
                        ),
                        observe(|_activate: On<Activate>, mut speed: ResMut<Speed>| {
                            speed.0 /= 1.2;
                        })
                    ),
                    (
                        button(
                            ButtonProps {
                                corners: RoundedCorners::None,
                                ..default()
                            },
                            (),
                            Spawn((Text::new("Pause"), ThemedText))
                        ),
                        observe(|_activate: On<Activate>, mut speed: ResMut<Speed>| {
                            speed.0 = 0.0;
                        })
                    ),
                    (
                        button(
                            ButtonProps {
                                corners: RoundedCorners::Right,
                                ..default()
                            },
                            (),
                            Spawn((Text::new("Faster"), ThemedText))
                        ),
                        observe(|_activate: On<Activate>, mut speed: ResMut<Speed>| {
                            speed.0 = (speed.0 * 1.2).max(1000.0);
                        })
                    ),
                ]
            ),
            (
                slider(
                    SliderProps {
                        max: 100.0,
                        value: 0.0,
                        ..default()
                    },
                    (SliderStep(10.), SliderPrecision(2)),
                ),
                Clock,
                observe(
                    |value_change: On<ValueChange<f32>>,
                     mut commands: Commands,
                     mut date: ResMut<Date>| {
                        commands
                            .entity(value_change.source)
                            .insert(SliderValue(value_change.value));
                        date.current_date = date
                            .current_date
                            .with_hour((value_change.value * 24.0 / 100.0).min(23.0) as u32)
                            .unwrap()
                            .with_minute((value_change.value * 24.0 * 60.0 / 100.0) as u32 % 60)
                            .unwrap();
                    }
                )
            ),
        ],
    ));
}

#[derive(Component)]
struct Clock;

#[derive(Resource, Default)]
pub struct Date {
    pub current_date: NaiveDateTime,
}

#[derive(Resource)]
struct Speed(f32);

#[allow(deprecated)]
fn set_date(time: Res<Time>, speed: Res<Speed>, mut date: ResMut<Date>) {
    date.current_date = date
        .current_date
        .checked_add_signed(TimeDelta::new((time.delta_secs() * speed.0) as i64, 0).unwrap())
        .unwrap();
}

fn display_date(
    mut commands: Commands,
    date: Res<Date>,
    mut text: Single<&mut Text, With<Clock>>,
    progress: Single<Entity, (With<Slider>, With<Clock>)>,
) {
    ***text = format!(
        "{:0>2}:{:0>2}",
        date.current_date.hour(),
        date.current_date.minute()
    );
    commands.entity(*progress).insert(SliderValue(
        (date.current_date.hour() * 60 + date.current_date.minute()) as f32 / (24.0 * 60.0) * 100.0,
    ));
}

#[allow(deprecated)]
fn animate_sun_direction(
    date: Res<Date>,
    mut directional_light: Single<&mut Transform, With<DirectionalLight>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    let current_time = date.current_date.hour() as f32
        + (date.current_date.minute() as f32 / 60.0)
        + (date.current_date.second() as f32 / 3600.0);
    directional_light.rotation = Quat::from_rotation_x((current_time - 6.0) / 12.0 * PI + PI);
    ambient_light.brightness = (((current_time - 12.0).abs() - 6.0).min(0.0).abs() + 1.0) * 50.0;
}
