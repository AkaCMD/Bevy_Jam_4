use super::{
    cursor::ArrowHint,
    player::{CommonDuck, GluttonousDuck},
    ui::Won,
    *,
};
use bevy::utils::thiserror;
use thiserror::Error;
// TODO: code refactor
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_level)
            .init_resource::<Level>()
            .init_resource::<Levels>()
            .init_resource::<CurrentLevelIndex>()
            .init_resource::<BreadCount>()
            .init_resource::<TotalBreadCount>()
            .init_resource::<LevelStack>()
            .add_event::<PrintLevel>()
            .add_event::<UpdateLevel>()
            .add_systems(
                Update,
                (
                    print_level,
                    update_level,
                    level_restart,
                    load_other_level,
                    change_level_cheats,
                    undo_the_level,
                ),
            );
    }
}

#[derive(Resource)]
pub struct Levels {
    pub levels: Vec<&'static str>,
}

macro_rules! load_levels {
    ($($path:expr),*) => { {
        vec![$(include_str!($path)), *]
     } };
}

// IMPORTANT: Remember to add corresponding level file path
// wasm version can't use std library
// no "," in the last file path
impl Default for Levels {
    fn default() -> Self {
        #[cfg(target_os = "windows")]
        let levels = load_levels!(
            "..\\..\\assets\\levels\\level1.txt",
            "..\\..\\assets\\levels\\level2.txt",
            "..\\..\\assets\\levels\\level3.txt",
            "..\\..\\assets\\levels\\level4.txt",
            "..\\..\\assets\\levels\\level5.txt",
            "..\\..\\assets\\levels\\level6.txt",
            "..\\..\\assets\\levels\\level7.txt",
            "..\\..\\assets\\levels\\level8.txt",
            "..\\..\\assets\\levels\\level9.txt",
            "..\\..\\assets\\levels\\level10.txt",
            "..\\..\\assets\\levels\\level11.txt",
            "..\\..\\assets\\levels\\level12.txt",
            "..\\..\\assets\\levels\\level13.txt",
            "..\\..\\assets\\levels\\level14.txt",
            "..\\..\\assets\\levels\\level15.txt"
        );

        #[cfg(any(target_os = "linux", target_os = "macos", target_arch = "wasm32"))]
        let levels = load_levels!(
            "../../assets/levels/level1.txt",
            "../../assets/levels/level2.txt",
            "../../assets/levels/level3.txt",
            "../../assets/levels/level4.txt",
            "../../assets/levels/level5.txt",
            "../../assets/levels/level6.txt",
            "../../assets/levels/level7.txt",
            "../../assets/levels/level8.txt",
            "../../assets/levels/level9.txt",
            "../../assets/levels/level10.txt",
            "../../assets/levels/level11.txt",
            "../../assets/levels/level12.txt",
            "../../assets/levels/level13.txt",
            "../../assets/levels/level14.txt",
            "../../assets/levels/level15.txt"
        );

        Self { levels }
    }
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Fail to load level!")]
    FailToLoadLevels,
}

pub fn load_level(level_index: usize, levels: Res<Levels>) -> anyhow::Result<Level> {
    levels
        .levels
        .get(level_index - 1)
        .map(|&level_content| {
            let level_data: Vec<Vec<char>> = level_content
                .lines()
                .map(|line| line.chars().collect())
                .collect();
            Level(level_data)
        })
        .ok_or_else(|| GameError::FailToLoadLevels.into())
}

#[derive(Resource)]
pub struct LevelStack(pub Stack<Vec<Vec<char>>>);

impl Default for LevelStack {
    fn default() -> Self {
        LevelStack(Stack::new())
    }
}

#[derive(Resource, Default)]
pub struct Level(pub Vec<Vec<char>>);

#[derive(Resource)]
pub struct CurrentLevelIndex(pub usize);

impl Default for CurrentLevelIndex {
    fn default() -> Self {
        CurrentLevelIndex(1)
    }
}

#[derive(Resource, Default)]
pub struct TotalBreadCount(pub i32);

#[derive(Resource)]
pub struct BreadCount(pub i32);
impl Default for BreadCount {
    fn default() -> Self {
        BreadCount(1)
    }
}
// TODO: Layers of objects (z axis)
pub enum ObjectType {
    Wall,
    Ice,
    BrokenIce,
    DuckOnIce,
    StuffedDuckOnIce,
    BreadOnIce,
    BreakingIce,
    DuckOnWater,
    DuckOnBreakingIce,
    GluttonousDuck,
}

// Symbols
impl ObjectType {
    pub fn get_symbol(self) -> char {
        match self {
            ObjectType::Wall => '@',
            ObjectType::Ice => '#',
            ObjectType::BrokenIce => '^',
            ObjectType::DuckOnIce => 'D',
            ObjectType::StuffedDuckOnIce => 'Q',
            ObjectType::BreadOnIce => 'B',
            ObjectType::BreakingIce => '*',
            ObjectType::DuckOnWater => 'P',
            ObjectType::DuckOnBreakingIce => 'O',
            ObjectType::GluttonousDuck => 'G',
        }
    }
}

#[derive(Component)]
pub struct Object;

// TODO: Multiple grids rigidbody(or object?)
// pub trait Rigidbody {
//     // fn get_occupied_positions(&self) -> Vec<(usize, usize)>;
//     // fn get_force_direction(&self) -> utils::Direction;
//     // fn get_force_source(&self) -> &Entity;

// }

#[derive(Event, Default)]
pub struct PrintLevel;

#[derive(Event, Default)]
pub struct UpdateLevel;

// pub fn load_level_from_file(file_path: &str) -> Result<Level, std::io::Error> {
//     let contents = fs::read_to_string(file_path)?;

//     let level_data: Vec<Vec<char>> = contents
//         .lines()
//         .map(|line| line.chars().collect())
//         .collect();

//     Ok(Level(level_data))
// }

fn spawn_level(
    mut commands: Commands,
    // resource
    asset_server: Res<AssetServer>,
    level_index: Res<CurrentLevelIndex>,
    mut bread_count: ResMut<BreadCount>,
    mut total_bread_count: ResMut<TotalBreadCount>,
    levels: Res<Levels>,
    mut level_stack: ResMut<LevelStack>,
    // event
    mut events: EventWriter<Won>,
) {
    // Load the level from a .txt file
    if let Ok(level) = load_level(level_index.0, levels) {
        // clear the stack
        level_stack.0.clear();

        spawn_sprites(
            &mut commands,
            &level.0,
            &asset_server,
            level_index.0,
            &mut bread_count,
            &mut events,
            true,
        );
        level_stack.0.push(level.0.clone());
        commands.insert_resource(level);
        total_bread_count.0 = bread_count.0;
    }
}

fn update_level(
    mut commands: Commands,
    // event
    mut events_update: EventReader<UpdateLevel>,
    mut events: EventWriter<Won>,
    // add the objects that won't be despawn to the filter
    object_query: Query<
        Entity,
        (
            With<Object>,
            Without<CommonDuck>,
            Without<ArrowHint>,
            Without<GluttonousDuck>,
        ),
    >,
    // resource
    asset_server: Res<AssetServer>,
    level: Res<Level>,
    level_index: Res<CurrentLevelIndex>,
    mut bread_count: ResMut<BreadCount>,
    mut level_stack: ResMut<LevelStack>,
) {
    for _ in events_update.read() {
        // Do not despawn ducks, update the translations of ducks
        // Do not despawn the arrow hint
        for object in &object_query {
            commands.entity(object).despawn();
        }
        level_stack.0.push(level.0.clone());
        spawn_sprites(
            &mut commands,
            &level.0,
            &asset_server,
            level_index.0,
            &mut bread_count,
            &mut events,
            false,
        );
    }
}

fn spawn_object(commands: &mut Commands, position: Vec3, sprite: Handle<Image>) {
    commands.spawn((
        SpriteBundle {
            texture: sprite,
            transform: Transform {
                translation: position,
                rotation: Quat::IDENTITY,
                scale: Vec3::new(1.0 * RESIZE, 1.0 * RESIZE, 1.0),
            },
            ..default()
        },
        Object,
    ));
}

pub fn spawn_upper_object(commands: &mut Commands, position: Vec3, sprite: Handle<Image>) {
    commands.spawn((
        SpriteBundle {
            texture: sprite,
            transform: Transform {
                translation: Vec3::new(position.x, position.y, position.z + 1.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(1.0 * RESIZE, 1.0 * RESIZE, 1.0),
            },
            ..default()
        },
        Object,
    ));
}

fn spawn_duck(
    commands: &mut Commands,
    position: Vec3,
    sprite: Handle<Image>,
    logic_position: (usize, usize),
    level_index: usize,
    asset_server: &Res<AssetServer>,
    is_stuffed: bool,
    can_move: bool,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 1.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(1.0 * RESIZE, 1.0 * RESIZE, 1.0),
            },
            texture: sprite,
            ..default()
        },
        CommonDuck {
            logic_position,
            is_stuffed,
            can_move,
        },
        Object,
    ));
    // Show click hint
    if level_index == 1 {
        spawn_upper_object(
            commands,
            Vec3::new(position.x + 120.0, position.y - 120.0, 1.0),
            asset_server.load("sprites/click_hint.png"),
        );
    }
}

fn spawn_g_duck(
    commands: &mut Commands,
    position: Vec3,
    sprite: Handle<Image>,
    logic_position: (usize, usize),
    can_move: bool,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 1.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(2.0 * RESIZE, 2.0 * RESIZE, 1.0),
            },
            texture: sprite,
            ..default()
        },
        GluttonousDuck {
            logic_position,
            has_eaten_bread: 4,
            is_stuffed: true,
            can_move,
        },
        Object,
    ));
}

fn spawn_sprites(
    commands: &mut Commands,
    level: &[Vec<char>],
    asset_server: &Res<AssetServer>,
    level_index: usize,
    bread_count: &mut ResMut<BreadCount>,
    // event
    events: &mut EventWriter<Won>,
    // when updates, do not respawn ducks
    should_respawn_duck: bool,
) {
    bread_count.0 = 0;
    // spawn the sprites
    for (row_index, row) in level.iter().enumerate() {
        for (col_index, &ch) in row.iter().enumerate() {
            let position = logic_position_to_translation((row_index, col_index));
            let object_type = match ch {
                '@' => ObjectType::Wall,
                '#' => ObjectType::Ice,
                '^' => ObjectType::BrokenIce,
                'D' => ObjectType::DuckOnIce,
                'B' => ObjectType::BreadOnIce,
                '*' => ObjectType::BreakingIce,
                'P' => ObjectType::DuckOnWater,
                'O' => ObjectType::DuckOnBreakingIce,
                'Q' => ObjectType::StuffedDuckOnIce,
                'G' => ObjectType::GluttonousDuck,
                _ => continue,
            };

            match object_type {
                ObjectType::Wall => {
                    spawn_object(commands, position, asset_server.load("sprites/wall.png"));
                }
                ObjectType::Ice => {
                    spawn_object(commands, position, asset_server.load("sprites/ice.png"));
                }
                ObjectType::BrokenIce => {
                    spawn_object(commands, position, asset_server.load("sprites/water.png"));
                }
                ObjectType::DuckOnIce => {
                    spawn_object(commands, position, asset_server.load("sprites/ice.png"));
                    if should_respawn_duck {
                        spawn_duck(
                            commands,
                            position,
                            asset_server.load("sprites/duck.png"),
                            (row_index, col_index),
                            level_index,
                            asset_server,
                            false,
                            true,
                        );
                    }
                }
                ObjectType::StuffedDuckOnIce => {
                    spawn_object(commands, position, asset_server.load("sprites/ice.png"));
                    if should_respawn_duck {
                        spawn_duck(
                            commands,
                            position,
                            asset_server.load("sprites/stuffed_duck.png"),
                            (row_index, col_index),
                            level_index,
                            asset_server,
                            true,
                            true,
                        );
                    }
                }
                ObjectType::BreadOnIce => {
                    bread_count.0 += 1;
                    spawn_object(commands, position, asset_server.load("sprites/ice.png"));
                    spawn_upper_object(commands, position, asset_server.load("sprites/bread.png"));
                }
                ObjectType::BreakingIce => {
                    spawn_object(
                        commands,
                        position,
                        asset_server.load("sprites/breaking_ice.png"),
                    );
                }
                ObjectType::DuckOnWater => {
                    spawn_object(commands, position, asset_server.load("sprites/water.png"));
                    if should_respawn_duck {
                        spawn_duck(
                            commands,
                            position,
                            asset_server.load("sprites/stuffed_duck.png"),
                            (row_index, col_index),
                            level_index,
                            asset_server,
                            true,
                            false,
                        );
                    }
                }
                ObjectType::DuckOnBreakingIce => {
                    spawn_object(
                        commands,
                        position,
                        asset_server.load("sprites/breaking_ice.png"),
                    );
                    if should_respawn_duck {
                        spawn_duck(
                            commands,
                            position,
                            asset_server.load("sprites/duck.png"),
                            (row_index, col_index),
                            level_index,
                            asset_server,
                            false,
                            true,
                        );
                    }
                }
                ObjectType::GluttonousDuck => {
                    spawn_object(commands, position, asset_server.load("sprites/ice.png"));
                    if should_respawn_duck {
                        spawn_g_duck(
                            commands,
                            position
                                + Vec3 {
                                    x: SPRITE_SIZE / 2.,
                                    y: -SPRITE_SIZE / 2.,
                                    z: 0.,
                                },
                            asset_server.load("sprites/g_duck_stuffed.png"),
                            (row_index, col_index),
                            true,
                        );
                    }
                }
            };
        }
    }

    if bread_count.0 == 0 {
        events.send(Won);
    }
}

#[allow(dead_code)]
pub fn print_level(
    level: Res<Level>,
    bread_count: Res<BreadCount>,
    mut events: EventReader<PrintLevel>,
) {
    for _ in events.read() {
        for row in level.0.iter() {
            for ch in row {
                print!("{}", ch);
            }
            println!();
        }
        info!("BreadCount: {}", bread_count.0);
    }
}

fn level_restart(
    mut commands: Commands,
    // query
    object_query: Query<Entity, With<Object>>,
    ui_query: Query<Entity, With<ui::MutUI>>,
    // resource
    input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    bread_count: ResMut<BreadCount>,
    total_bread_count: ResMut<TotalBreadCount>,
    level_index: Res<CurrentLevelIndex>,
    levels: Res<Levels>,
    level_stack: ResMut<LevelStack>,
    // event
    events: EventWriter<Won>,
) {
    if input.just_pressed(KeyCode::R) {
        // Despawn level elements
        for object in &object_query {
            commands.entity(object).despawn();
        }
        // Despawn ui elements
        for entity in ui_query.iter() {
            commands.entity(entity).despawn();
        }
        spawn_level(
            commands,
            asset_server,
            level_index,
            bread_count,
            total_bread_count,
            levels,
            level_stack,
            events,
        );
    }
}

fn load_other_level(
    mut commands: Commands,
    // query
    object_query: Query<Entity, With<Object>>,
    // resource
    level_index: Res<CurrentLevelIndex>,
    asset_server: Res<AssetServer>,
    bread_count: ResMut<BreadCount>,
    total_bread_count: ResMut<TotalBreadCount>,
    levels: Res<Levels>,
    level_stack: ResMut<LevelStack>,
    // event
    events: EventWriter<Won>,
) {
    if level_index.is_changed() {
        // clear the scene
        for entity in object_query.iter() {
            commands.entity(entity).despawn();
        }
        spawn_level(
            commands,
            asset_server,
            level_index,
            bread_count,
            total_bread_count,
            levels,
            level_stack,
            events,
        )
    }
}

// Cheat codes for skipping levels
fn change_level_cheats(
    input: Res<Input<KeyCode>>,
    levels: Res<Levels>,
    mut level_index: ResMut<CurrentLevelIndex>,
) {
    let origin_index = level_index.0;
    if input.just_pressed(KeyCode::BracketLeft) && level_index.0 > 1 {
        level_index.0 -= 1;
    }
    if input.just_pressed(KeyCode::BracketRight) {
        level_index.0 += 1;
    }
    // Handle invalid level index
    if load_level(level_index.0, levels).is_err() {
        //info!("Invalid level index");
        level_index.0 = origin_index;
    }
}

// Undo
fn undo_the_level(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut level_stack: ResMut<LevelStack>,
    asset_server: Res<AssetServer>,
    level_index: Res<CurrentLevelIndex>,
    mut bread_count: ResMut<BreadCount>,
    mut level: ResMut<Level>,
    mut events: EventWriter<Won>,
    object_query: Query<Entity, With<Object>>,
) {
    if input.just_pressed(KeyCode::Z) && level_stack.0.size() >= 2 {
        level_stack.0.pop();
        let origin_level = level_stack.0.peek().unwrap().clone();
        level.0 = origin_level;
        // for row in level.0.iter() {
        //     for ch in row {
        //         print!("{}", ch);
        //     }
        //     println!();
        // }
        for object in &object_query {
            commands.entity(object).despawn();
        }
        spawn_sprites(
            &mut commands,
            &level.0,
            &asset_server,
            level_index.0,
            &mut bread_count,
            &mut events,
            true,
        );
    }
}

// TODO: specify entity type and layer
pub fn get_entity_on_logic_position(
    logic_position: (usize, usize),
    query: &Query<(Entity, &Transform), With<Object>>,
) -> Option<Entity> {
    let entity_translation = logic_position_to_translation(logic_position)
        + Vec3 {
            x: 0.,
            y: 0.,
            z: 1.,
        }; // quick fix for duck, remove or change it later
    for (entity, transform) in query.iter() {
        if transform.translation == entity_translation {
            return Some(entity);
        }
    }
    None
}
