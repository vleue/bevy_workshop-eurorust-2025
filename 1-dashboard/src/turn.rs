use bevy::{color::palettes, prelude::*};

pub fn turn_plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, (react_to_input, blink))
        .add_observer(update_turn_signal);
}

#[derive(Event)]
enum TurnSignal {
    Left,
    Right,
    Stop,
}

#[derive(Component)]
struct TurnSignalIndicator;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Transform::from_xyz(0.0, 300.0, 0.0),
        Visibility::Visible,
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

fn react_to_input(keyboard: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        commands.trigger(TurnSignal::Left);
    } else if keyboard.just_pressed(KeyCode::ArrowRight) {
        commands.trigger(TurnSignal::Right);
    } else if keyboard.just_pressed(KeyCode::Enter) {
        commands.trigger(TurnSignal::Stop);
    };
}

fn update_turn_signal(
    signal: On<TurnSignal>,
    indicator: Single<(Entity, &Children), With<TurnSignalIndicator>>,
    mut commands: Commands,
    mut sprites: Query<&mut Sprite>,
) {
    sprites.get_mut(indicator.1[0]).unwrap().color = palettes::tailwind::GRAY_800.into();
    sprites.get_mut(indicator.1[1]).unwrap().color = palettes::tailwind::GRAY_800.into();

    match signal.event() {
        TurnSignal::Left => {
            commands
                .entity(indicator.0)
                .insert(Blink::on_entity(indicator.1[0]));
        }
        TurnSignal::Right => {
            commands
                .entity(indicator.0)
                .insert(Blink::on_entity(indicator.1[1]));
        }
        TurnSignal::Stop => {
            commands.entity(indicator.0).remove::<Blink>();
        }
    }
}

#[derive(Component)]
struct Blink {
    target: Entity,
    timer: Timer,
}

impl Blink {
    fn on_entity(entity: Entity) -> Self {
        Self {
            target: entity,
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

fn blink(mut blink: Single<&mut Blink>, mut sprites: Query<&mut Sprite>, time: Res<Time>) {
    if blink.timer.tick(time.delta()).just_finished() {
        let mut sprite = sprites.get_mut(blink.target).unwrap();
        sprite.color = if sprite.color == Color::WHITE {
            palettes::tailwind::GRAY_800.into()
        } else {
            Color::WHITE
        };
    }
}
