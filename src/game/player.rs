use super::{
    audio::PlaySFX,
    level::PrintLevel,
    *,
};
use bevy::window::PrimaryWindow;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, player_movement)
            .add_event::<SpawnDuck>()
            .add_systems(Update, spawn_duck);
    }
}

#[derive(Component)]
pub struct Duck {
    logic_position: (usize, usize),
}

#[derive(Event, Default)]
pub struct SpawnDuck(pub (usize, usize));

fn spawn_duck(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<SpawnDuck>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for event in events.read() {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: logic_position_to_translation(
                        event.0,
                        window_query.get_single().unwrap(),
                    ),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::new(1.0 * RESIZE, 1.0 * RESIZE, 1.0),
                },
                texture: asset_server.load("sprites/duck.png"),
                ..default()
            },
            Duck {
                logic_position: event.0,
            },
        ));
    }
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

// TODO: select the duck as player
// TODO: implement the game logic
fn player_movement(
    key_board_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Sprite, &mut Duck), With<Duck>>,
    mut events: EventWriter<PlaySFX>,
    mut level: ResMut<level::Level>,
    mut events_1: EventWriter<PrintLevel>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((mut transform, mut sprite, mut duck)) = player_query.get_single_mut() {
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

        let end_position = slip(level, duck.logic_position, direction);

        // Update object positions
        duck.logic_position = end_position;
        transform.translation =
            logic_position_to_translation(end_position, window_query.get_single().unwrap());

        if direction != utils::Direction::None {
            // play quark sound
            events.send(PlaySFX);        
            events_1.send(PrintLevel);
        }
    }
}

// Slip until hit the wall or bread
// Wall: @
fn slip(
    mut level: ResMut<level::Level>,
    logic_position: (usize, usize), // (col, row)
    direction: utils::Direction,
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
            }
        }
        utils::Direction::Down => {
            while position.1 < rows - 1 && level.0[position.1 + 1][position.0] != '@' {
                position.1 += 1;
            }
        }
        utils::Direction::Left => {
            while position.0 > 0 && level.0[position.1][position.0 - 1] != '@' {
                position.0 -= 1;
            }
        }
        utils::Direction::Right => {
            while position.0 < cols - 1 && level.0[position.1][position.0 + 1] != '@' {
                position.0 += 1;
            }
        }
        _ => (),
    }
    level.0[logic_position.1][logic_position.0] = '#';
    level.0[position.1][position.0] = 'D';
    position
}
