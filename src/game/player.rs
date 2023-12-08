use std::time::Duration;

use super::{
    audio::PlaySFX,
    level::{PrintLevel, UpdateLevel},
    *,
};
use bevy::window::PrimaryWindow;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, player_movement)
            .add_systems(Update, component_animator_system::<Transform>);
    }
}

#[derive(Component)]
pub struct Duck {
    pub logic_position: (usize, usize),
    pub is_stuffed: bool, // one duck, one bread
}

// the chosen duck
#[derive(Component)]
pub struct Player;

#[derive(Event, Default)]
pub struct SpawnDuck(pub (usize, usize));

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn player_movement(
    mut commands: Commands,
    // query
    mut player_query: Query<
        (
            &mut Transform,
            &mut Sprite,
            &mut Handle<Image>,
            &mut Duck,
            Entity,
        ),
        With<Player>,
    >,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // event
    mut events_sfx: EventWriter<PlaySFX>,
    mut events_update: EventWriter<UpdateLevel>,
    //mut events_print: EventWriter<PrintLevel>,
    // resource
    key_board_input: Res<Input<KeyCode>>,
    level: ResMut<level::Level>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((transform, mut sprite, mut image, mut duck, entity)) = player_query.get_single_mut()
    {
        let mut direction = utils::Direction::None;

        if key_board_input.just_pressed(KeyCode::Left) || key_board_input.just_pressed(KeyCode::A) {
            direction = Direction::Left;
            sprite.flip_x = false;
        }
        if key_board_input.just_pressed(KeyCode::Right) || key_board_input.just_pressed(KeyCode::D)
        {
            direction = Direction::Right;
            sprite.flip_x = true;
        }
        if key_board_input.just_pressed(KeyCode::Up) || key_board_input.just_pressed(KeyCode::W) {
            direction = Direction::Up;
        }
        if key_board_input.just_pressed(KeyCode::Down) || key_board_input.just_pressed(KeyCode::S) {
            direction = Direction::Down;
        }
        if direction != utils::Direction::None {
            let duck_is_stuffed_before = duck.is_stuffed;
            let end_position = slip(&mut duck, direction, level);
            let duck_is_stuffed_after = duck.is_stuffed;

            // TODO: delay it
            if !duck_is_stuffed_before && duck_is_stuffed_after {
                // play eat sound
                events_sfx.send(PlaySFX {
                    path: "audio/eat.ogg".to_string(),
                    volume: bevy::audio::Volume::new_absolute(0.2),
                });
                *image = asset_server.load("sprites/stuffed_duck.png");
            }

            // Update object positions
            duck.logic_position = end_position;
            // Update the translation of ducks
            let v3 =
                logic_position_to_translation(end_position, window_query.get_single().unwrap());
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(300),
                TransformPositionLens {
                    start: transform.translation,
                    end: Vec3::new(v3.x, v3.y, 1.0),
                },
            )
            .with_repeat_count(1);

            commands.entity(entity).insert(Animator::new(tween));
            //let v3 = logic_position_to_translation(end_position, window_query.get_single().unwrap());
            //transform.translation = Vec3::new(v3.x, v3.y, 1.0);

            // play quark sound
            events_sfx.send(PlaySFX {
                path: "audio/quark.wav".to_string(),
                volume: bevy::audio::Volume::new_absolute(0.5),
            });
            //events_print.send(PrintLevel);
            events_update.send(UpdateLevel);
        }
    }
}

// Slip until hitting the wall or bread
// Wall: @
// Bread: B
fn slip(
    duck: &mut Duck, // logic position: (col, row)
    direction: utils::Direction,
    // resource
    mut level: ResMut<level::Level>,
) -> (usize, usize) {
    // Up: row--
    // Down: row++
    // Left: col--
    // Right: col++
    let rows = level.0.len();
    let logic_position = duck.logic_position;
    let cols = level.0[logic_position.1].len();
    let mut position = logic_position;
    match direction {
        utils::Direction::Up => {
            while position.1 > 0
                && level.0[position.1 - 1][position.0] != '@'
                && level.0[position.1 - 1][position.0] != 'D'
                && (!duck.is_stuffed || level.0[position.1 - 1][position.0] != 'B')
            {
                position.1 -= 1;
                if level.0[position.1][position.0] == 'B' {
                    duck.is_stuffed = true;
                    break;
                }
            }
        }
        utils::Direction::Down => {
            while position.1 < rows - 1
                && level.0[position.1 + 1][position.0] != '@'
                && level.0[position.1 + 1][position.0] != 'D'
                && (!duck.is_stuffed || level.0[position.1 + 1][position.0] != 'B')
            {
                position.1 += 1;
                if level.0[position.1][position.0] == 'B' {
                    duck.is_stuffed = true;
                    break;
                }
            }
        }
        utils::Direction::Left => {
            while position.0 > 0
                && level.0[position.1][position.0 - 1] != '@'
                && level.0[position.1][position.0 - 1] != 'D'
                && (!duck.is_stuffed || level.0[position.1][position.0 - 1] != 'B')
            {
                position.0 -= 1;
                if level.0[position.1][position.0] == 'B' {
                    duck.is_stuffed = true;
                    break;
                }
            }
        }
        utils::Direction::Right => {
            while position.0 < cols - 1
                && level.0[position.1][position.0 + 1] != '@'
                && level.0[position.1][position.0 + 1] != 'D'
                && (!duck.is_stuffed || level.0[position.1][position.0 + 1] != 'B')
            {
                position.0 += 1;
                if level.0[position.1][position.0] == 'B' {
                    duck.is_stuffed = true;
                    break;
                }
            }
        }
        _ => (),
    }
    level.0[logic_position.1][logic_position.0] = '#';
    level.0[position.1][position.0] = 'D';
    position
}
