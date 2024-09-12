use super::{
    audio::PlaySFX,
    level::{get_entity_on_logic_position, SymbolType::*, UpdateLevel},
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
    fn get_bread_sum(&self) -> u32;
    fn is_stuffed(&self) -> bool;
    fn can_move(&self) -> bool;
    fn set_logic_position(&mut self, position: (usize, usize));
    fn set_can_move(&mut self, can_move: bool);
    fn eat_bread(&mut self);
}

#[derive(Component)]
pub struct CommonDuck {
    pub logic_position: (usize, usize),
    pub can_move: bool, // stuffed_duck on breaking_ice => can't move
    pub bread_sum: u32,
    pub belly_capacity: u32,
}

impl Duck for CommonDuck {
    fn get_logic_position(&self) -> (usize, usize) {
        self.logic_position
    }

    fn is_stuffed(&self) -> bool {
        self.bread_sum == self.belly_capacity
    }

    fn can_move(&self) -> bool {
        self.can_move
    }

    fn set_logic_position(&mut self, position: (usize, usize)) {
        self.logic_position = position;
    }

    fn set_can_move(&mut self, can_move: bool) {
        self.can_move = can_move;
    }

    fn eat_bread(&mut self) {
        self.bread_sum += 1;
    }

    fn get_bread_sum(&self) -> u32 {
        self.bread_sum
    }
}

// the chosen duck
#[derive(Component)]
pub struct Player;

fn player_movement(
    mut commands: Commands,
    // query
    mut player_query: Query<
        (
            &mut Transform,
            &mut Sprite,
            &mut Handle<Image>,
            Option<&mut CommonDuck>,
            Entity,
        ),
        With<Player>,
    >,
    // event
    mut events_sfx: EventWriter<PlaySFX>,
    mut events_update: EventWriter<UpdateLevel>,
    mut event_shake: EventWriter<ShakeOtherDucksInDir>,
    mut events_print: EventWriter<level::PrintLevel>,
    // resource
    key_board_input: Res<ButtonInput<KeyCode>>,
    level: ResMut<level::Level>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((transform, mut sprite, mut image, c_duck, entity)) = player_query.get_single_mut() {
        let duck: &mut dyn Duck = c_duck.unwrap().into_inner();

        if !duck.can_move() {
            return;
        }
        let mut direction = utils::Direction::None;

        if key_board_input.just_pressed(KeyCode::ArrowLeft)
            || key_board_input.just_pressed(KeyCode::KeyA)
        {
            direction = utils::Direction::Left;
            sprite.flip_x = false;
        }
        if key_board_input.just_pressed(KeyCode::ArrowRight)
            || key_board_input.just_pressed(KeyCode::KeyD)
        {
            direction = utils::Direction::Right;
            sprite.flip_x = true;
        }
        if key_board_input.just_pressed(KeyCode::ArrowUp)
            || key_board_input.just_pressed(KeyCode::KeyW)
        {
            direction = utils::Direction::Up;
        }
        if key_board_input.just_pressed(KeyCode::ArrowDown)
            || key_board_input.just_pressed(KeyCode::KeyS)
        {
            direction = utils::Direction::Down;
        }
        if direction != utils::Direction::None {
            let duck_bread_sum_before = duck.get_bread_sum();
            let duck_can_move_before = duck.can_move();
            let duck_is_stuffed_before = duck.is_stuffed();
            let end_position = slip(duck, direction, level);
            let duck_bread_sum_after = duck.get_bread_sum();
            let duck_can_move_after = duck.can_move();
            let duck_is_stuffed_after = duck.is_stuffed();

            // TODO: delay it
            if duck_bread_sum_after > duck_bread_sum_before {
                // play eat sound
                events_sfx.send(PlaySFX {
                    path: "audio/eat.ogg".to_string(),
                    volume: bevy::audio::Volume::new(0.05),
                });
            }

            if !duck_is_stuffed_before && duck_is_stuffed_after {
                *image = asset_server.load("sprites/stuffed_duck.png");
            }

            if duck_can_move_before && !duck_can_move_after {
                // play ice breaking sound
                events_sfx.send(PlaySFX {
                    path: "audio/ice_breaking.ogg".to_string(),
                    volume: bevy::audio::Volume::new(0.4),
                });
            }

            // Update object positions
            duck.set_logic_position(end_position);
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
                volume: bevy::audio::Volume::new(0.4),
            });
            events_print.send(level::PrintLevel);
            events_update.send(UpdateLevel);
        }
    }
}

// Slip until hitting the wall or bread
// common duck
fn slip(
    duck: &mut dyn Duck,
    direction: utils::Direction,
    // resource
    mut level: ResMut<level::Level>,
) -> (usize, usize) {
    // Up: row--, Down: row++, Left: col--, Right: col++
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

fn is_valid_move(symbol: char, duck: &dyn Duck) -> bool {
    symbol != Wall.get_symbol()
        && symbol != DuckOnIce.get_symbol()
        && symbol != DuckOnWater.get_symbol()
        && symbol != DuckOnBreakingIce.get_symbol()
        && symbol != StuffedDuckOnIce.get_symbol()
        && (!duck.is_stuffed() || symbol != BreadOnIce.get_symbol())
}

// TODO: replace it with eat_bread_or_break_ice
fn collide_with_object(symbol: char, duck: &mut dyn Duck) -> bool {
    if symbol == BreadOnIce.get_symbol() {
        duck.eat_bread();
        return true;
    }
    if symbol == BreakingIce.get_symbol() && duck.is_stuffed() {
        duck.set_can_move(false);
        return true;
    }
    false
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
