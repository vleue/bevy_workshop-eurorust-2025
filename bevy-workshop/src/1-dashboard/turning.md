# Turning

## Reacting to User Input

We'll add an event that we can trigger when the users press the turn signal buttons:

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::prelude::*;
#[derive(Event)]
enum TurnSignal {
    Left,
    Right,
    Stop,
}
```

And a system that react on keyboard input to trigger that event:

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::prelude::*;
# #[derive(Event)]
# enum TurnSignal {
#     Left,
#     Right,
#     Stop,
# }
fn react_to_input(keyboard: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        commands.trigger(TurnSignal::Left);
    } else if keyboard.just_pressed(KeyCode::ArrowRight) {
        commands.trigger(TurnSignal::Right);
    } else if keyboard.just_pressed(KeyCode::Enter) {
        commands.trigger(TurnSignal::Stop);
    };
}
```

<div class="warning">

Why trigger an event rather than do the change directly?

This helps with dissociating the input from the actual change, allowing for more flexibility and easier testing.

It also helps if later we need to add other triggers for the same event, for example once our car is self-driving.

</div>

## Making the Lights Blink

We need two more parts:

- a system that will make the image blink over time
- an observer system that will react to the `TurnSignal` event and will mark the entity that needs to blink

We need a `Blink` component with a [`Timer`](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.Timer.html):

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::prelude::*;
#[derive(Component)]
struct Blink {
    target: Entity,
    timer: Timer,
}
```

and an helper to create it:

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::prelude::*;
# #[derive(Component)]
# struct Blink {
#     target: Entity,
#     timer: Timer,
# }
impl Blink {
    fn on_entity(entity: Entity) -> Self {
        Self {
            target: entity,
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}
```

To make the signal blink, we'll change the `color` field of the `Sprite` component, every time the timer is finished:

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::{color::palettes, prelude::*};
# #[derive(Component)]
# struct Blink {
#     target: Entity,
#     timer: Timer,
# }
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
```

## Observing the Event

This system will only run when it received a `TurnSignal` event:

- it will start by turning off all the signals
- then mark the correct one as blinking according to the event

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::{color::palettes, prelude::*};
# #[derive(Component)]
# struct Blink {
#     target: Entity,
#     timer: Timer,
# }
# #[derive(Event)]
# enum TurnSignal {
#     Left,
#     Right,
#     Stop,
# }
# #[derive(Component)]
# struct TurnSignalIndicator;
# impl Blink {
#     fn on_entity(entity: Entity) -> Self {
#         Self {
#             target: entity,
#             timer: Timer::from_seconds(0.5, TimerMode::Repeating),
#         }
#     }
# }
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
```

## Updating the Plugin

We now need to update the plugin to use the new systems and observer:

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# use bevy::{color::palettes, prelude::*};
# #[derive(Event)]
# enum TurnSignal {
#     Left,
#     Right,
#     Stop,
# }
# fn setup() {}
# fn react_to_input() {}
# fn blink() {}
# fn update_turn_signal(signal: On<TurnSignal>) {}
pub fn turn_plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, (react_to_input, blink))
        .add_observer(update_turn_signal);
}
```
