use bevy::{
    color::palettes::tailwind,
    feathers::{
        controls::toggle_switch,
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    light::NotShadowCaster,
    prelude::*,
    ui::Checked,
    ui_widgets::{Checkbox, ValueChange, observe},
};
use chrono::Timelike;

use crate::{
    natural_time::Date,
    remote_server::{LightHistory, LightHistoryRequest, LightStateChange, ManualLightStateChange},
};

pub fn lights_plugin(app: &mut App) {
    app.add_plugins(MeshPickingPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, display_history)
        .add_observer(on_light_state_changed)
        .add_observer(on_light_history);
}

#[derive(Component)]
pub struct Light {
    max_intensity: f32,
    name: Lights,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Lights {
    Bedroom1,
    Bedroom2,
    Bathroom1,
    Bathroom2,
    Toilets,
    LivingRoom1,
    LivingRoom2,
    Kitchen,
    Hall,
    Hallway,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let light_mesh = meshes.add(Sphere::new(0.15));
    let light_material = materials.add(StandardMaterial {
        base_color: tailwind::YELLOW_400.into(),
        unlit: true,
        ..default()
    });

    spawn_lights(
        &mut commands,
        vec2(4.5, 2.0),
        light_mesh.clone(),
        light_material.clone(),
        Light {
            max_intensity: light_consts::lux::HALLWAY,
            name: Lights::Bedroom1,
        },
    );

    spawn_lights(
        &mut commands,
        vec2(-3.5, 2.0),
        light_mesh.clone(),
        light_material.clone(),
        Light {
            max_intensity: light_consts::lux::OFFICE,
            name: Lights::LivingRoom1,
        },
    );
    spawn_lights(
        &mut commands,
        vec2(-1.0, 2.0),
        light_mesh.clone(),
        light_material.clone(),
        Light {
            max_intensity: light_consts::lux::OFFICE,
            name: Lights::LivingRoom2,
        },
    );

    spawn_lights(
        &mut commands,
        vec2(-4.0, -3.0),
        light_mesh.clone(),
        light_material.clone(),
        Light {
            max_intensity: light_consts::lux::HALLWAY,
            name: Lights::Bedroom2,
        },
    );

    spawn_lights(
        &mut commands,
        vec2(0.0, -3.0),
        light_mesh.clone(),
        light_material.clone(),
        Light {
            max_intensity: light_consts::lux::HALLWAY,
            name: Lights::Kitchen,
        },
    );

    spawn_lights(
        &mut commands,
        vec2(-2.0, -4.0),
        light_mesh.clone(),
        light_material.clone(),
        Light {
            max_intensity: light_consts::lux::HALLWAY,
            name: Lights::Toilets,
        },
    );

    spawn_lights(
        &mut commands,
        vec2(-2.0, -2.0),
        light_mesh.clone(),
        light_material.clone(),
        Light {
            max_intensity: light_consts::lux::HALLWAY,
            name: Lights::Bathroom2,
        },
    );

    spawn_lights(
        &mut commands,
        vec2(4.0, -3.0),
        light_mesh.clone(),
        light_material.clone(),
        Light {
            max_intensity: light_consts::lux::HALLWAY,
            name: Lights::Hall,
        },
    );

    spawn_lights(
        &mut commands,
        vec2(1.25, 0.75),
        light_mesh.clone(),
        light_material.clone(),
        Light {
            max_intensity: light_consts::lux::OFFICE,
            name: Lights::Hallway,
        },
    );

    spawn_lights(
        &mut commands,
        vec2(1.5, 3.5),
        light_mesh.clone(),
        light_material.clone(),
        Light {
            max_intensity: light_consts::lux::OFFICE,
            name: Lights::Bathroom1,
        },
    );
}

fn spawn_lights(
    commands: &mut Commands,
    position: Vec2,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    light: Light,
) {
    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(position.extend(2.5).xzy()),
            PointLight {
                shadows_enabled: true,
                intensity: 0.0,
                ..default()
            },
            NotShadowCaster,
            light,
        ))
        .observe(toggle_light_panel);
}

#[derive(Component)]
struct LightPanel(Lights);

fn toggle_light_panel(
    event: On<Pointer<Press>>,
    mut commands: Commands,
    light: Query<(&Light, &PointLight)>,
    previous_panel: Query<Entity, With<LightPanel>>,
) {
    if let Ok(entity) = previous_panel.single() {
        commands.entity(entity).despawn();
    }

    let (light, pointlight) = light.get(event.entity).unwrap();

    let target_light = light.name.clone();
    commands.spawn((
        LightPanel(target_light),
        Node {
            width: px(300),
            height: px(100),
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::Start,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            right: px(0),
            top: px(200),
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
                    (Text::new("Light: "), ThemedText),
                    (Text::new(format!("{:?}", light.name)), ThemedText),
                ]
            ),
            (
                toggle_switch(()),
                observe(
                    move |value_change: On<ValueChange<bool>>, mut commands: Commands| {
                        if value_change.value {
                            commands.entity(value_change.source).insert(Checked);
                        } else {
                            commands.entity(value_change.source).remove::<Checked>();
                        }
                        commands.trigger(ManualLightStateChange {
                            light: target_light,
                            on: value_change.value,
                        });
                    }
                ),
            ),
        ],
    ));

    if pointlight.intensity != 0.0 {
        commands.queue(|world: &mut World| {
            let entity = world
                .query_filtered::<Entity, With<Checkbox>>()
                .single(world)
                .unwrap();
            world.entity_mut(entity).insert(Checked);
        });
    }

    commands.trigger(LightHistoryRequest {
        light: target_light,
    });
}

fn on_light_state_changed(
    message: On<LightStateChange>,
    mut light: Query<(&mut PointLight, &Light)>,
    panel_open: Query<&LightPanel>,
    toggle: Query<Entity, With<Checkbox>>,
    mut commands: Commands,
) {
    for (mut light, light_data) in light.iter_mut() {
        if light_data.name == message.light {
            if message.on {
                light.intensity = light_data.max_intensity
                    * light_consts::lumens::VERY_LARGE_CINEMA_LIGHT
                    / light_consts::lux::OVERCAST_DAY;
            } else {
                light.intensity = 0.0;
            }
        }
    }
    if let Ok(panel) = panel_open.single() {
        if panel.0 == message.light {
            if message.on {
                commands.entity(toggle.single().unwrap()).insert(Checked);
            } else {
                commands
                    .entity(toggle.single().unwrap())
                    .remove::<Checked>();
            }
        }
    }
}

fn on_light_history(
    message: On<LightHistory>,
    panel_open: Query<(Entity, &LightPanel)>,
    mut commands: Commands,
) {
    if let Ok((entity, panel)) = panel_open.single() {
        if panel.0 == message.light {
            commands
                .entity(entity)
                .insert(History(message.history.clone()));
        }
    }
}

#[derive(Component)]
struct History(Vec<(u32, u32)>);

fn display_history(history: Single<&History>, mut gizmos: Gizmos, date: Res<Date>) {
    let graph_size = 5.0;
    let graph_position = Vec2::new(8.0, 5.0);

    gizmos
        .arrow(
            Vec3::new(graph_position.x, 0.0, graph_position.y),
            Vec3::new(graph_position.x + graph_size, 0.0, graph_position.y),
            tailwind::GRAY_300,
        )
        .with_tip_length(0.2);
    gizmos.line(
        Vec3::new(graph_position.x, 0.0, graph_position.y),
        Vec3::new(graph_position.x, 0.0, graph_position.y - graph_size),
        tailwind::GRAY_300,
    );

    let current_time = date.current_date.hour() * 60 + date.current_date.minute();
    let progress = current_time as f32 / (24.0 * 60.0) * graph_size;
    gizmos.line(
        Vec3::new(graph_position.x + progress, 0.0, graph_position.y),
        Vec3::new(
            graph_position.x + progress,
            0.0,
            graph_position.y - graph_size,
        ),
        tailwind::GRAY_500,
    );

    let mut current_state = 0;
    let mut current_position = 0.0;
    for (time, state) in &history.0 {
        let position = *time as f32 / (24.0 * 60.0) * graph_size;
        let height = graph_size / 3.0 * (current_state + 1) as f32;
        let new_height = graph_size / 3.0 * (state + 1) as f32;
        gizmos.line(
            Vec3::new(
                graph_position.x + current_position,
                0.0,
                graph_position.y - height,
            ),
            Vec3::new(graph_position.x + position, 0.0, graph_position.y - height),
            tailwind::GREEN_500,
        );
        gizmos.line(
            Vec3::new(graph_position.x + position, 0.0, graph_position.y - height),
            Vec3::new(
                graph_position.x + position,
                0.0,
                graph_position.y - new_height,
            ),
            tailwind::GREEN_500,
        );
        current_state = *state;
        current_position = position;
    }
    let height = graph_size / 3.0 * (current_state + 1) as f32;
    gizmos.line(
        Vec3::new(
            graph_position.x + current_position,
            0.0,
            graph_position.y - height,
        ),
        Vec3::new(
            graph_position.x + graph_size,
            0.0,
            graph_position.y - height,
        ),
        tailwind::GREEN_500,
    );
}
