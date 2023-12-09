use super::{player::Duck, *};
use crate::game::player::Player;
use bevy::{input::mouse::MouseButtonInput, window::PrimaryWindow};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
            .add_systems(Update, (get_cursor_position, click_detection));
    }
}

#[derive(Component)]
pub struct ArrowHint;

#[derive(Resource, Default)]
pub struct CursorPosition(pub Vec2);

fn get_cursor_position(
    mut cursor_position: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.get_single().unwrap();
    if let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        cursor_position.0 = cursor_pos;

        // TODO: hover cursor on the duck, show arrow hint
    }
}

fn click_detection(
    mut commands: Commands,
    // event
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    // query
    duck_query: Query<(&Duck, Entity), (With<Duck>, Without<Player>)>,
    player_query: Query<Entity, (With<Duck>, With<Player>)>,
    arrow_hint_query: Query<Entity, With<ArrowHint>>,
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
                if (cursor_position.0 - duck_position).length() < 33.0 {
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
                    }
                    // Clear the previous arrow hint
                    for entity in arrow_hint_query.iter() {
                        commands.entity(entity).despawn();
                    }
                }
                // spawn_upper_object(
                //     &mut commands,
                //     Vec3 {
                //         x: cursor_position.0.x,
                //         y: cursor_position.0.y,
                //         z: 0.0,
                //     },
                //     asset_server.load("sprites/debug.png"),
                // );
                //info!("Duck pos: {:?}", duck_position);
                //info!("Cursor pos: {:?}", cursor_position.0);
                //info!("Distance: {}", cursor_position.0.distance(duck_position));
            }
        }
    }
}
