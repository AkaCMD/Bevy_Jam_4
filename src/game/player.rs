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
    mut player_query: Query<(&mut Transform, &mut Sprite, &mut Duck, Entity), With<Player>>,
    // event
    mut events_sfx: EventWriter<PlaySFX>,
    mut events_update: EventWriter<UpdateLevel>,
    mut events_print: EventWriter<PrintLevel>,
    // resource
    key_board_input: Res<Input<KeyCode>>,
    level: ResMut<level::Level>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((mut transform, mut sprite, mut duck, entity)) = player_query.get_single_mut() {
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
            let end_position = slip(duck.logic_position, direction, level);

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
            events_sfx.send(PlaySFX);
            events_print.send(PrintLevel);
            events_update.send(UpdateLevel);
        }
    }
}

// Slip until hit the wall or bread
// Wall: @
// Bread: B
fn slip(
    logic_position: (usize, usize), // (col, row)
    direction: utils::Direction,
    // resource
    mut level: ResMut<level::Level>,
) -> (usize, usize) {
    // Up: row--
    // Down: row++
    // Left: col--
    // Right: col++
    let rows = level.0.len();
    let cols = level.0[logic_position.1].len();
    let mut position = logic_position;
    match direction {
        utils::Direction::Up => {
            while position.1 > 0
                && level.0[position.1 - 1][position.0] != '@'
                && level.0[position.1 - 1][position.0] != 'D'
            {
                position.1 -= 1;
                if level.0[position.1][position.0] == 'B' {
                    break;
                }
            }
        }
        utils::Direction::Down => {
            while position.1 < rows - 1
                && level.0[position.1 + 1][position.0] != '@'
                && level.0[position.1 + 1][position.0] != 'D'
            {
                position.1 += 1;
                if level.0[position.1][position.0] == 'B' {
                    break;
                }
            }
        }
        utils::Direction::Left => {
            while position.0 > 0
                && level.0[position.1][position.0 - 1] != '@'
                && level.0[position.1][position.0 - 1] != 'D'
            {
                position.0 -= 1;
                if level.0[position.1][position.0] == 'B' {
                    break;
                }
            }
        }
        utils::Direction::Right => {
            while position.0 < cols - 1
                && level.0[position.1][position.0 + 1] != '@'
                && level.0[position.1][position.0 + 1] != 'D'
            {
                position.0 += 1;
                if level.0[position.1][position.0] == 'B' {
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
