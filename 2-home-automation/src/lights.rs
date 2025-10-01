use bevy::{color::palettes::tailwind, light::NotShadowCaster, prelude::*};
#[cfg(feature = "light_panel")]
use bevy::{
    feathers::{
        controls::toggle_switch,
        theme::{ThemeBackgroundColor, ThemedText},
        tokens,
    },
    ui::Checked,
    ui_widgets::{Checkbox, ValueChange, observe},
};

#[cfg(feature = "light_history")]
use crate::remote_server::{LightHistory, LightHistoryRequest};
use crate::{
    natural_time::Date,
    remote_server::{ManualLightStateChange, ServerLightStateChange},
};

pub fn lights_plugin(app: &mut App) {
    app.add_plugins(MeshPickingPlugin)
        .add_systems(Startup, setup)
        .add_observer(on_light_state_changed);
    #[cfg(feature = "light_history")]
    app.add_systems(Update, display_history)
        .add_observer(on_light_history);
    #[cfg(feature = "light_panel")]
    app.add_systems(Update, highlight_light);
}

#[derive(Component, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Light {
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

    [
        (vec2(4.5, 2.0), Light::Bedroom1),
        (vec2(-3.5, 2.0), Light::LivingRoom1),
        (vec2(-1.0, 2.0), Light::LivingRoom2),
        (vec2(-4.0, -3.0), Light::Bedroom2),
        (vec2(0.0, -3.0), Light::Kitchen),
        (vec2(-2.0, -4.0), Light::Toilets),
        (vec2(-2.0, -2.0), Light::Bathroom2),
        (vec2(1.5, 3.5), Light::Bathroom1),
        (vec2(1.25, 0.75), Light::Hallway),
        (vec2(4.0, -3.0), Light::Hall),
    ]
    .into_iter()
    .for_each(|(position, name)| {
        spawn_lights(
            &mut commands,
            position,
            light_mesh.clone(),
            light_material.clone(),
            name,
        );
    });
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
        .observe(toggle_light);
}

#[cfg(not(feature = "light_panel"))]
fn toggle_light(
    event: On<Pointer<Click>>,
    mut commands: Commands,
    light: Query<(&Light, &PointLight)>,
) {
    let (light, point_light) = light.get(event.entity).unwrap();
    commands.trigger(ManualLightStateChange {
        light: *light,
        on: point_light.intensity == 0.0,
    });
}

#[cfg(feature = "light_panel")]
#[derive(Component)]
struct LightPanel(Light, Entity);

#[cfg(feature = "light_panel")]
fn toggle_light(
    event: On<Pointer<Click>>,
    mut commands: Commands,
    light: Query<(&Light, &PointLight)>,
    previous_panel: Query<Entity, With<LightPanel>>,
) {
    if let Ok(entity) = previous_panel.single() {
        commands.entity(entity).despawn();
    }

    let (light, pointlight) = light.get(event.entity).unwrap();

    let target_light = *light;
    commands.spawn((
        LightPanel(target_light, event.entity),
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
                    (Text::new(format!("{:?}", light)), ThemedText),
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

    #[cfg(feature = "light_history")]
    commands.trigger(LightHistoryRequest {
        light: target_light,
    });
}

#[cfg(feature = "light_panel")]
fn highlight_light(
    selected_light: Single<&LightPanel>,
    transform: Query<&Transform>,
    mut gizmos: Gizmos,
    time: Res<Time>,
) {
    let transform = transform.get(selected_light.1).unwrap();
    gizmos.sphere(
        transform.to_isometry(),
        0.25 + (time.elapsed_secs() * 10.0).sin() / 25.0,
        tailwind::YELLOW_300,
    );
}

fn on_light_state_changed(
    message: On<ServerLightStateChange>,
    mut light: Query<(&mut PointLight, &Light)>,
    #[cfg(feature = "light_panel")] panel_open: Query<&LightPanel>,
    #[cfg(feature = "light_panel")] toggle: Query<Entity, With<Checkbox>>,
    #[cfg(feature = "light_panel")] mut commands: Commands,
) {
    for (mut light, light_data) in light.iter_mut() {
        if *light_data == message.light {
            if message.on {
                light.intensity = 300000.0;
            } else {
                light.intensity = 0.0;
            }
        }
    }
    #[cfg(feature = "light_panel")]
    if let Ok(panel) = panel_open.single()
        && panel.0 == message.light
    {
        if message.on {
            commands.entity(toggle.single().unwrap()).insert(Checked);
        } else {
            commands
                .entity(toggle.single().unwrap())
                .remove::<Checked>();
        }
    }
}

#[cfg(feature = "light_history")]
fn on_light_history(
    message: On<LightHistory>,
    panel_open: Query<(Entity, &LightPanel)>,
    mut commands: Commands,
) {
    if let Ok((entity, panel)) = panel_open.single()
        && panel.0 == message.light
    {
        commands
            .entity(entity)
            .insert(History(message.history.clone()));
    }
}

#[cfg(feature = "light_history")]
#[derive(Component)]
struct History(Vec<(u32, bool)>);

#[cfg(feature = "light_history")]
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

    let current_time = date.current_time;
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

    let mut current_state = false;
    let mut current_position = 0.0;
    for (time, state) in &history.0 {
        let position = *time as f32 / (24.0 * 60.0) * graph_size;
        let height = graph_size / 3.0 * if current_state { 2.0 } else { 1.0 };
        let new_height = graph_size / 3.0 * if *state { 2.0 } else { 1.0 };
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
    let height = graph_size / 3.0 * if current_state { 2.0 } else { 1.0 };
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
