use super::{
    audio::PlaySFX,
    level::{ObjectType::*, UpdateLevel},
    *,
};
use bevy::utils::Duration;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement)
            .add_systems(Update, component_animator_system::<Transform>);
    }
}

#[derive(Component)]
pub struct Duck {
    pub logic_position: (usize, usize),
    pub is_stuffed: bool, // one duck, one bread
    pub can_move: bool,   // stuffed_duck on breaking_ice => can't move
}

// the chosen duck
#[derive(Component)]
pub struct Player;

#[derive(Event, Default)]
pub struct SpawnDuck(pub (usize, usize));

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
        if !duck.can_move {
            return;
        }
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
            let duck_can_move_before = duck.can_move;
            let end_position = slip(&mut duck, direction, level);
            let duck_is_stuffed_after = duck.is_stuffed;
            let duck_can_move_after = duck.can_move;

            // TODO: delay it
            if !duck_is_stuffed_before && duck_is_stuffed_after {
                // play eat sound
                events_sfx.send(PlaySFX {
                    path: "audio/eat.ogg".to_string(),
                    volume: bevy::audio::Volume::new_absolute(0.2),
                });
                *image = asset_server.load("sprites/stuffed_duck.png");
            }

            if duck_can_move_before && !duck_can_move_after {
                // play ice breaking sound
                events_sfx.send(PlaySFX {
                    path: "audio/ice_breaking.ogg".to_string(),
                    volume: bevy::audio::Volume::new_absolute(0.4),
                });
            }

            // Update object positions
            duck.logic_position = end_position;
            // Update the translation of ducks
            let v3 = logic_position_to_translation(end_position);
            let tween_translation = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(300),
                TransformPositionLens {
                    start: transform.translation,
                    end: Vec3::new(v3.x, v3.y, 1.0),
                },
            )
            .with_repeat_count(1);

            // Scale the duck while moving
            let origin_scale = transform.scale;
            let new_scale = transform.scale * Vec3::new(1.3, 0.7, 1.);
            let tween_scale = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(300),
                TransformScaleLens {
                    start: new_scale,
                    end: origin_scale,
                },
            )
            .with_repeat_count(1);

            let track: Tracks<Transform> = Tracks::new(vec![tween_translation, tween_scale]);

            commands.entity(entity).insert(Animator::new(track));
            //let v3 = logic_position_to_translation(end_position, window_query.get_single().unwrap());
            //transform.translation = Vec3::new(v3.x, v3.y, 1.0);

            // play quark sound
            events_sfx.send(PlaySFX {
                path: "audio/quark.ogg".to_string(),
                volume: bevy::audio::Volume::new_absolute(0.5),
            });
            //events_print.send(PrintLevel);
            events_update.send(UpdateLevel);
        }
    }
}

// Slip until hitting the wall or bread
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
            while position.1 > 0 && is_valid_move(level.0[position.1 - 1][position.0], duck) {
                position.1 -= 1;
                if collide_with_object(level.0[position.1][position.0], duck) {
                    break;
                }
            }
        }
        utils::Direction::Down => {
            while position.1 < rows - 1 && is_valid_move(level.0[position.1 + 1][position.0], duck)
            {
                position.1 += 1;
                if collide_with_object(level.0[position.1][position.0], duck) {
                    break;
                }
            }
        }
        utils::Direction::Left => {
            while position.0 > 0 && is_valid_move(level.0[position.1][position.0 - 1], duck) {
                position.0 -= 1;
                if collide_with_object(level.0[position.1][position.0], duck) {
                    break;
                }
            }
        }
        utils::Direction::Right => {
            while position.0 < cols - 1 && is_valid_move(level.0[position.1][position.0 + 1], duck)
            {
                position.0 += 1;
                if collide_with_object(level.0[position.1][position.0], duck) {
                    break;
                }
            }
        }
        _ => (),
    }

    // Update symbols on the level
    let mut duck_char: char = DuckOnIce.get_symbol();
    if duck.is_stuffed {
        duck_char = StuffedDuckOnIce.get_symbol();
    }

    if level.0[logic_position.1][logic_position.0] == DuckOnBreakingIce.get_symbol() {
        level.0[logic_position.1][logic_position.0] = BreakingIce.get_symbol();
    } else {
        level.0[logic_position.1][logic_position.0] = Ice.get_symbol();
    }
    if level.0[position.1][position.0] == BreakingIce.get_symbol() {
        level.0[position.1][position.0] = DuckOnBreakingIce.get_symbol();
    } else {
        level.0[position.1][position.0] = duck_char;
    }
    if !duck.can_move {
        level.0[position.1][position.0] = DuckOnWater.get_symbol();
    }
    position
}

fn is_valid_move(symbol: char, duck: &Duck) -> bool {
    symbol != Wall.get_symbol()
        && symbol != DuckOnIce.get_symbol()
        && symbol != DuckOnWater.get_symbol()
        && symbol != DuckOnBreakingIce.get_symbol()
        && symbol != StuffedDuckOnIce.get_symbol()
        && (!duck.is_stuffed || symbol != BreadOnIce.get_symbol())
}

fn collide_with_object(symbol: char, duck: &mut Duck) -> bool {
    let mut should_stop = false;
    if symbol == BreadOnIce.get_symbol() {
        duck.is_stuffed = true;
        should_stop = true;
    }
    if symbol == BreakingIce.get_symbol() && duck.is_stuffed {
        duck.can_move = false;
        should_stop = true;
    }
    should_stop
}
