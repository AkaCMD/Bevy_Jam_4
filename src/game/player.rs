use super::{
    audio::PlaySFX,
    level::{get_entity_on_logic_position, ObjectType::*, UpdateLevel},
    *,
};
use bevy::utils::Duration;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_movement,
                component_animator_system::<Transform>,
                shake_other_ducks_in_direction,
            ),
        )
        .add_event::<ShakeOtherDucksInDir>();
    }
}

pub trait Duck {
    fn get_logic_position(&self) -> (usize, usize);
    fn get_occupied_positions(&self) -> Vec<(usize, usize)>;
    fn is_stuffed(&self) -> bool;
    fn can_move(&self) -> bool;
    fn set_logic_position(&mut self, position: (usize, usize));
    fn set_is_stuffed(&mut self, is_stuffed: bool);
    fn set_can_move(&mut self, can_move: bool);
}

#[derive(Component)]
pub struct CommonDuck {
    pub logic_position: (usize, usize),
    pub is_stuffed: bool, // one duck, one bread
    pub can_move: bool,   // stuffed_duck on breaking_ice => can't move
}

impl Duck for CommonDuck {
    fn get_logic_position(&self) -> (usize, usize) {
        self.logic_position
    }

    fn get_occupied_positions(&self) -> Vec<(usize, usize)> {
        vec![self.logic_position]
    }

    fn is_stuffed(&self) -> bool {
        self.is_stuffed
    }

    fn can_move(&self) -> bool {
        self.can_move
    }

    fn set_logic_position(&mut self, position: (usize, usize)) {
        self.logic_position = position;
    }

    fn set_is_stuffed(&mut self, is_stuffed: bool) {
        self.is_stuffed = is_stuffed;
    }

    fn set_can_move(&mut self, can_move: bool) {
        self.can_move = can_move;
    }
}

#[derive(Component)]
pub struct GluttonousDuck {
    pub logic_position: (usize, usize),
    pub has_eaten_bread: u32,
    pub is_stuffed: bool,
    pub can_move: bool,
}

impl Duck for GluttonousDuck {
    fn get_logic_position(&self) -> (usize, usize) {
        self.logic_position
    }

    fn get_occupied_positions(&self) -> Vec<(usize, usize)> {
        vec![
            self.logic_position,
            (self.logic_position.0 + 1, self.logic_position.1),
            (self.logic_position.0, self.logic_position.1 + 1),
            (self.logic_position.0 + 1, self.logic_position.1 + 1),
        ]
    }

    fn is_stuffed(&self) -> bool {
        self.is_stuffed
    }

    fn can_move(&self) -> bool {
        self.can_move
    }

    fn set_logic_position(&mut self, position: (usize, usize)) {
        self.logic_position = position;
    }

    fn set_is_stuffed(&mut self, is_stuffed: bool) {
        self.is_stuffed = is_stuffed;
    }

    fn set_can_move(&mut self, can_move: bool) {
        self.can_move = can_move;
    }
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
            &mut CommonDuck,
            Entity,
        ),
        With<Player>,
    >,
    // event
    mut events_sfx: EventWriter<PlaySFX>,
    mut events_update: EventWriter<UpdateLevel>,
    mut event_shake: EventWriter<ShakeOtherDucksInDir>,
    //mut events_print: EventWriter<PrintLevel>,
    // resource
    key_board_input: Res<Input<KeyCode>>,
    level: ResMut<level::Level>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((transform, mut sprite, mut image, mut duck, entity)) = player_query.get_single_mut()
    {
        if !duck.can_move() {
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
            let duck_is_stuffed_before = duck.is_stuffed();
            let duck_can_move_before = duck.can_move();
            let end_position = slip(&mut duck, direction, level);
            let duck_is_stuffed_after = duck.is_stuffed();
            let duck_can_move_after = duck.can_move();

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
                Duration::from_millis(DUCK_MOVE_MILI_SECS),
                TransformPositionLens {
                    start: transform.translation,
                    end: Vec3::new(v3.x, v3.y, 1.0),
                },
            )
            .with_repeat_count(1);

            // Scale the duck while moving
            let origin_scale = Vec3::new(1.0 * RESIZE, 1.0 * RESIZE, 1.0);
            let new_scale = transform.scale * Vec3::new(1.3, 0.7, 1.);
            let tween_scale = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(DUCK_MOVE_MILI_SECS),
                TransformScaleLens {
                    start: new_scale,
                    end: origin_scale,
                },
            )
            .with_repeat_count(1);

            let track: Tracks<Transform> = Tracks::new(vec![tween_translation, tween_scale]);

            commands.entity(entity).insert(Animator::new(track));
            event_shake.send(ShakeOtherDucksInDir {
                direction,
                player_logic_position: duck.get_logic_position(),
            });
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
    duck: &mut CommonDuck,
    direction: utils::Direction,
    // resource
    mut level: ResMut<level::Level>,
) -> (usize, usize) {
    // Up: row--
    // Down: row++
    // Left: col--
    // Right: col++
    let rows = level.0.len();
    let logic_position = duck.get_logic_position();
    let cols = level.0[logic_position.0].len();
    let mut position = logic_position;
    match direction {
        utils::Direction::Up => {
            while position.0 > 0 && is_valid_move(level.0[position.0 - 1][position.1], duck) {
                position.0 -= 1;
                if collide_with_object(level.0[position.0][position.1], duck) {
                    break;
                }
            }
        }
        utils::Direction::Down => {
            while position.0 < rows - 1 && is_valid_move(level.0[position.0 + 1][position.1], duck)
            {
                position.0 += 1;
                if collide_with_object(level.0[position.0][position.1], duck) {
                    break;
                }
            }
        }
        utils::Direction::Left => {
            while position.1 > 0 && is_valid_move(level.0[position.0][position.1 - 1], duck) {
                position.1 -= 1;
                if collide_with_object(level.0[position.0][position.1], duck) {
                    break;
                }
            }
        }
        utils::Direction::Right => {
            while position.1 < cols - 1 && is_valid_move(level.0[position.0][position.1 + 1], duck)
            {
                position.1 += 1;
                if collide_with_object(level.0[position.0][position.1], duck) {
                    break;
                }
            }
        }
        _ => (),
    }

    // Update symbols on the level
    let mut duck_char: char = DuckOnIce.get_symbol();
    if duck.is_stuffed() {
        duck_char = StuffedDuckOnIce.get_symbol();
    }

    if level.0[logic_position.0][logic_position.1] == DuckOnBreakingIce.get_symbol() {
        level.0[logic_position.0][logic_position.1] = BreakingIce.get_symbol();
    } else {
        level.0[logic_position.0][logic_position.1] = Ice.get_symbol();
    }
    if level.0[position.0][position.1] == BreakingIce.get_symbol() {
        level.0[position.0][position.1] = DuckOnBreakingIce.get_symbol();
    } else {
        level.0[position.0][position.1] = duck_char;
    }
    if !duck.can_move() {
        level.0[position.0][position.1] = DuckOnWater.get_symbol();
    }
    position
}

fn is_valid_move(symbol: char, duck: &impl Duck) -> bool {
    symbol != Wall.get_symbol()
        && symbol != DuckOnIce.get_symbol()
        && symbol != DuckOnWater.get_symbol()
        && symbol != DuckOnBreakingIce.get_symbol()
        && symbol != StuffedDuckOnIce.get_symbol()
        && (!duck.is_stuffed() || symbol != BreadOnIce.get_symbol())
}

fn collide_with_object(symbol: char, duck: &mut impl Duck) -> bool {
    let mut should_stop = false;
    if symbol == BreadOnIce.get_symbol() {
        duck.set_is_stuffed(true);
        should_stop = true;
    }
    if symbol == BreakingIce.get_symbol() && duck.is_stuffed() {
        duck.set_can_move(false);
        should_stop = true;
    }
    should_stop
}

#[derive(Event)]
struct ShakeOtherDucksInDir {
    direction: utils::Direction,
    player_logic_position: (usize, usize),
}

fn shake_other_ducks_in_direction(
    mut commands: Commands,
    level: Res<level::Level>,
    query: Query<(Entity, &Transform), With<level::Object>>,
    mut events: EventReader<ShakeOtherDucksInDir>,
) {
    for e in events.read() {
        let direction = e.direction;
        let mut ducks_to_shake: Vec<Entity> = Vec::new();
        let mut position = e.player_logic_position;
        let rows = level.0.len();
        let cols = level.0[position.0].len();

        let delta: (i32, i32) = match direction {
            utils::Direction::Up => (-1, 0),
            utils::Direction::Down => (1, 0),
            utils::Direction::Left => (0, -1),
            utils::Direction::Right => (0, 1),
            utils::Direction::None => return,
        };

        while position.0 > 0 && position.0 < rows - 1 && position.1 > 0 && position.1 < cols - 1 {
            position.0 = (delta.0 + position.0 as i32) as usize;
            position.1 = (delta.1 + position.1 as i32) as usize;
            let symbol = level.0[position.0][position.1];
            if [
                DuckOnBreakingIce.get_symbol(),
                DuckOnIce.get_symbol(),
                DuckOnWater.get_symbol(),
                StuffedDuckOnIce.get_symbol(),
            ]
            .contains(&symbol)
            {
                if let Some(entity) = get_entity_on_logic_position(position, &query) {
                    ducks_to_shake.push(entity);
                }
            } else {
                break;
            }
        }

        for entity in ducks_to_shake {
            let origin_scale = Vec3::new(1.0 * RESIZE, 1.0 * RESIZE, 1.0);
            let new_scale = origin_scale * Vec3::new(1.3, 0.7, 1.);
            let tween_scale = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(300),
                TransformScaleLens {
                    start: new_scale,
                    end: origin_scale,
                },
            )
            .with_repeat_count(1);
            let delay = Delay::new(Duration::from_millis(DUCK_MOVE_MILI_SECS));
            commands
                .entity(entity)
                .insert(Animator::new(delay.then(tween_scale)));
        }
    }
}

// TODO: finish it
// fn move_together(
//     g_duck: &mut GluttonousDuck,
//     mut level: ResMut<level::Level>,
//     occupied_positions: Vec<(usize, usize)>,
//     direction: utils::Direction,
// ) {
//     let edge_position = find_edge_position(&occupied_positions, direction);
//     match direction {
//         utils::Direction::Up => {
//             for pos in occupied_positions.iter() {
//                 if pos.0 == edge_position {}
//             }
//         }
//         utils::Direction::Down => todo!(),
//         utils::Direction::Left => todo!(),
//         utils::Direction::Right => todo!(),
//         utils::Direction::None => todo!(),
//     }
// }

// fn find_edge_position(
//     occupied_positions: &Vec<(usize, usize)>,
//     direction: utils::Direction,
// ) -> usize {
//     match direction {
//         utils::Direction::Up => occupied_positions.iter().min_by_key(|&(a, _)| a).unwrap().0,
//         utils::Direction::Down => occupied_positions.iter().max_by_key(|&(a, _)| a).unwrap().0,
//         utils::Direction::Left => occupied_positions.iter().min_by_key(|&(_, a)| a).unwrap().1,
//         utils::Direction::Right => occupied_positions.iter().max_by_key(|&(_, a)| a).unwrap().1,
//         utils::Direction::None => 0,
//     }
// }
