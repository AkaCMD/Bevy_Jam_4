use super::{
    audio::PlaySFX,
    level::{BreadCount, PrintLevel, UpdateLevel},
    ui::Won,
    *,
};
use bevy::window::PrimaryWindow;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
pub struct Duck {
    pub logic_position: (usize, usize),
}

#[derive(Event, Default)]
pub struct SpawnDuck(pub (usize, usize));

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

// TODO: select the duck as player
// TODO: implement the game logic
// TODO: Refactor
fn player_movement(
    key_board_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Sprite, &mut Duck), With<Duck>>,
    mut events: EventWriter<PlaySFX>,
    mut events_update: EventWriter<UpdateLevel>,
    mut events_win: EventWriter<Won>,
    mut level: ResMut<level::Level>,
    mut events_1: EventWriter<PrintLevel>,
    mut bread_count: ResMut<BreadCount>,
) {
    if let Ok((mut transform, mut sprite, mut duck)) = player_query.get_single_mut() {
        let mut direction = utils::Direction::None;

        if key_board_input.just_pressed(KeyCode::Left) || key_board_input.just_pressed(KeyCode::A) {
            direction = Direction::Left;
            sprite.flip_x = false; // TODO: Re-implement it
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

        let end_position = slip(
            level,
            duck.logic_position,
            direction,
            bread_count,
            events_win,
        );

        // Update object positions
        duck.logic_position = end_position;

        if direction != utils::Direction::None {
            // play quark sound
            events.send(PlaySFX);
            events_1.send(PrintLevel);
            events_update.send(UpdateLevel);
        }
    }
}

// Slip until hit the wall or bread
// Wall: @
// Bread: B
fn slip(
    mut level: ResMut<level::Level>,
    logic_position: (usize, usize), // (col, row)
    direction: utils::Direction,
    mut bread_count: ResMut<BreadCount>,
    mut events: EventWriter<Won>,
) -> (usize, usize) {
    // Up: row--
    // Down: row++
    // Left: col--
    // Right: col++
    let rows = level.0.len();
    let mut cols = level.0[logic_position.1].len();
    let mut position = logic_position.clone();
    match direction {
        utils::Direction::Up => {
            while position.1 > 0 && level.0[position.1 - 1][position.0] != '@' {
                position.1 -= 1;
                if level.0[position.1][position.0] == 'B' {
                    bread_count.0 -= 1;
                    // Win condition: no bread left
                    if bread_count.0 == 0 {
                        events.send(Won);
                    }
                    break;
                }
            }
        }
        utils::Direction::Down => {
            while position.1 < rows - 1 && level.0[position.1 + 1][position.0] != '@' {
                position.1 += 1;
                if level.0[position.1][position.0] == 'B' {
                    bread_count.0 -= 1;
                    // Win condition: no bread left
                    if bread_count.0 == 0 {
                        events.send(Won);
                    }
                    break;
                }
            }
        }
        utils::Direction::Left => {
            while position.0 > 0 && level.0[position.1][position.0 - 1] != '@' {
                position.0 -= 1;
                if level.0[position.1][position.0] == 'B' {
                    bread_count.0 -= 1;
                    // Win condition: no bread left
                    if bread_count.0 == 0 {
                        events.send(Won);
                    }
                    break;
                }
            }
        }
        utils::Direction::Right => {
            while position.0 < cols - 1 && level.0[position.1][position.0 + 1] != '@' {
                position.0 += 1;
                if level.0[position.1][position.0] == 'B' {
                    bread_count.0 -= 1;
                    // Win condition: no bread left
                    if bread_count.0 == 0 {
                        events.send(Won);
                    }
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
