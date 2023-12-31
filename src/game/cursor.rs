use super::{
    player::{CommonDuck, GluttonousDuck},
    *,
};
use crate::game::player::Player;
use bevy::{input::mouse::MouseButtonInput, window::PrimaryWindow};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
            .add_systems(Update, get_cursor_position)
            .add_systems(Update, click_detection);
    }
}

const DISTANCE: f32 = (640.0 / 2.0 + 10.0) * RESIZE;

#[derive(Component)]
pub struct ArrowHint;

#[derive(Resource, Default)]
pub struct CursorPosition(pub Vec2);

fn get_cursor_position(
    mut commands: Commands,
    // resource
    asset_server: Res<AssetServer>,
    mut cursor_position: ResMut<CursorPosition>,
    // query
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    duck_query: Query<&CommonDuck, (With<CommonDuck>, Without<Player>)>,
    g_duck_query: Query<&GluttonousDuck, (With<GluttonousDuck>, Without<Player>)>,
    arrow_query: Query<Entity, (With<ArrowHint>, Without<Parent>)>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.get_single().unwrap();
    if let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        cursor_position.0 = cursor_pos;

        for entity in arrow_query.iter() {
            commands.entity(entity).despawn();
        }
        // Hover cursor on the duck, show arrow hint
        for duck in duck_query.iter() {
            let duck_position_v3 = logic_position_to_translation(duck.logic_position);
            let duck_position: Vec2 = Vec2 {
                x: duck_position_v3.x,
                y: duck_position_v3.y,
            };
            if (cursor_position.0 - duck_position).length() < DISTANCE {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(
                                duck_position.x,
                                duck_position.y + SPRITE_SIZE,
                                2.0,
                            ),
                            rotation: Quat::IDENTITY,
                            scale: Vec3::new(1.0 * RESIZE, 1.0 * RESIZE, 1.0),
                        },
                        texture: asset_server.load("sprites/arrow.png"),
                        ..default()
                    },
                    ArrowHint,
                    //level::Object,
                ));
            }
        }

        for g_duck in g_duck_query.iter() {
            let duck_position_v3 = logic_position_to_translation(g_duck.logic_position);
            let duck_position: Vec2 = Vec2 {
                x: duck_position_v3.x,
                y: duck_position_v3.y,
            };
            if (cursor_position.0 - duck_position).length() < DISTANCE * 2. {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(
                                duck_position.x + SPRITE_SIZE / 2.,
                                duck_position.y + SPRITE_SIZE * 1.5,
                                2.0,
                            ),
                            rotation: Quat::IDENTITY,
                            scale: Vec3::new(2.0 * RESIZE, 2.0 * RESIZE, 1.0),
                        },
                        texture: asset_server.load("sprites/arrow.png"),
                        ..default()
                    },
                    ArrowHint,
                    //level::Object,
                ));
            }
        }
    }
}

pub fn click_detection(
    mut commands: Commands,
    // event
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    // query
    duck_query: Query<(&CommonDuck, Entity), (With<CommonDuck>, Without<Player>)>,
    g_duck_query: Query<(&GluttonousDuck, Entity), (With<GluttonousDuck>, Without<Player>)>,
    player_query: Query<Entity, With<Player>>,
    arrow_hint_query: Query<Entity, (With<ArrowHint>, With<Parent>)>,
    // resource
    cursor_position: Res<CursorPosition>,
    asset_server: Res<AssetServer>,
) {
    for event in mouse_button_input_events.read() {
        if event.button == MouseButton::Left {
            for (duck, entity) in duck_query.iter() {
                let duck_position_v3 = logic_position_to_translation(duck.logic_position);
                let duck_position: Vec2 = Vec2 {
                    x: duck_position_v3.x,
                    y: duck_position_v3.y,
                };
                if (cursor_position.0 - duck_position).length() < DISTANCE {
                    //info!("You are the chosen one!");
                    commands
                        .entity(entity)
                        .insert(Player)
                        .with_children(|parent| {
                            parent.spawn((
                                SpriteBundle {
                                    transform: Transform {
                                        translation: Vec3::new(0.0, 500.0, 1.0),
                                        ..default()
                                    },
                                    texture: asset_server.load("sprites/arrow.png"),
                                    ..default()
                                },
                                ArrowHint,
                                level::Object,
                            ));
                        });
                    // Clear the previous player
                    for entity in player_query.iter() {
                        commands.entity(entity).remove::<Player>();
                        commands.entity(entity).clear_children();
                    }
                    // Clear the previous arrow hint
                    for entity in arrow_hint_query.iter() {
                        commands.entity(entity).despawn();
                    }
                }
            }
            for (g_duck, entity) in g_duck_query.iter() {
                let duck_position_v3 = logic_position_to_translation(g_duck.logic_position);
                let duck_position: Vec2 = Vec2 {
                    x: duck_position_v3.x,
                    y: duck_position_v3.y,
                };
                if (cursor_position.0 - duck_position).length() < DISTANCE * 2. {
                    //info!("You are the chosen one!");
                    commands
                        .entity(entity)
                        .insert(Player)
                        .with_children(|parent| {
                            parent.spawn((
                                SpriteBundle {
                                    transform: Transform {
                                        translation: Vec3::new(0.0, 500.0, 1.0),
                                        ..default()
                                    },
                                    texture: asset_server.load("sprites/arrow.png"),
                                    ..default()
                                },
                                ArrowHint,
                                level::Object,
                            ));
                        });
                    // Clear the previous player
                    for entity in player_query.iter() {
                        commands.entity(entity).remove::<Player>();
                        commands.entity(entity).clear_children();
                    }
                    // Clear the previous arrow hint
                    for entity in arrow_hint_query.iter() {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}
