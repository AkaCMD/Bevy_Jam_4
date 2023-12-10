use super::{cursor::ArrowHint, player::Duck, ui::Won, *};
use bevy::utils::thiserror;
use thiserror::Error;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_level)
            .init_resource::<Level>()
            .init_resource::<Levels>()
            .init_resource::<CurrentLevelIndex>()
            .init_resource::<BreadCount>()
            .init_resource::<TotalBreadCount>()
            //.add_event::<PrintLevel>()
            .add_event::<UpdateLevel>()
            .add_systems(
                Update,
                (
                    //print_level,
                    update_level,
                    level_restart,
                    load_other_level,
                    change_level_cheats,
                ),
            );
    }
}

#[derive(Resource)]
pub struct Levels {
    pub level1: &'static str,
    pub level2: &'static str,
    pub level3: &'static str,
    pub level4: &'static str,
    pub level5: &'static str,
    pub level6: &'static str,
    pub level7: &'static str,
    pub level8: &'static str,
}
// wasm version can't use std library
impl Default for Levels {
    fn default() -> Self {
        #[cfg(target_os = "windows")]
        let (level1, level2, level3, level4, level5, level6, level7, level8) = (
            include_str!("..\\..\\assets\\levels\\level1.txt"),
            include_str!("..\\..\\assets\\levels\\level2.txt"),
            include_str!("..\\..\\assets\\levels\\level3.txt"),
            include_str!("..\\..\\assets\\levels\\level4.txt"),
            include_str!("..\\..\\assets\\levels\\level5.txt"),
            include_str!("..\\..\\assets\\levels\\level6.txt"),
            include_str!("..\\..\\assets\\levels\\level7.txt"),
            include_str!("..\\..\\assets\\levels\\level8.txt"),
        );

        #[cfg(target_os = "linux")]
        let (level1, level2, level3, level4, level5, level6, level7, level8) = (
            include_str!("../../assets/levels/level1.txt"),
            include_str!("../../assets/levels/level2.txt"),
            include_str!("../../assets/levels/level3.txt"),
            include_str!("../../assets/levels/level4.txt"),
            include_str!("../../assets/levels/level5.txt"),
            include_str!("../../assets/levels/level6.txt"),
            include_str!("../../assets/levels/level7.txt"),
            include_str!("../../assets/levels/level8.txt"),
        );

        #[cfg(target_os = "macos")]
        let (level1, level2, level3, level4, level5, level6, level7, level8) = (
            include_str!("../../assets/levels/level1.txt"),
            include_str!("../../assets/levels/level2.txt"),
            include_str!("../../assets/levels/level3.txt"),
            include_str!("../../assets/levels/level4.txt"),
            include_str!("../../assets/levels/level5.txt"),
            include_str!("../../assets/levels/level6.txt"),
            include_str!("../../assets/levels/level7.txt"),
            include_str!("../../assets/levels/level8.txt"),
        );

        #[cfg(target_arch = "wasm32")]
        let (level1, level2, level3, level4, level5, level6, level7, level8) = (
            include_str!("../../assets/levels/level1.txt"),
            include_str!("../../assets/levels/level2.txt"),
            include_str!("../../assets/levels/level3.txt"),
            include_str!("../../assets/levels/level4.txt"),
            include_str!("../../assets/levels/level5.txt"),
            include_str!("../../assets/levels/level6.txt"),
            include_str!("../../assets/levels/level7.txt"),
            include_str!("../../assets/levels/level8.txt"),
        );

        Levels {
            level1,
            level2,
            level3,
            level4,
            level5,
            level6,
            level7,
            level8,
        }
    }
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Fail to load level!")]
    FailToLoadLevels,
}

pub fn load_level(level_index: i32, levels: Res<Levels>) -> anyhow::Result<Level> {
    let level_content = match level_index {
        1 => levels.level1,
        2 => levels.level2,
        3 => levels.level3,
        4 => levels.level4,
        5 => levels.level5,
        6 => levels.level6,
        7 => levels.level7,
        8 => levels.level8,
        _ => return Err(GameError::FailToLoadLevels.into()),
    };

    let level_data: Vec<Vec<char>> = level_content
        .to_string()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    Ok(Level(level_data))
}

#[derive(Resource, Default)]
pub struct Level(pub Vec<Vec<char>>);

#[derive(Resource)]
pub struct CurrentLevelIndex(pub i32);

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
enum ObjectType {
    Wall,
    Ice,
    DuckOnIce,
    BreadOnIce,
    BreakingIce,
    DuckOnWater,
    DuckOnBreakingIce,
}

#[derive(Component)]
pub struct Object;

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
    level_index: ResMut<CurrentLevelIndex>,
    mut bread_count: ResMut<BreadCount>,
    mut total_bread_count: ResMut<TotalBreadCount>,
    levels: Res<Levels>,
    // event
    mut events: EventWriter<Won>,
) {
    // Load the level from a .txt file
    if let Ok(level) = load_level(level_index.0, levels) {
        spawn_sprites(
            &mut commands,
            &level.0,
            &asset_server,
            &mut bread_count,
            &mut events,
            false,
        );
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
    object_query: Query<Entity, (With<Object>, Without<Duck>, Without<ArrowHint>)>,
    // resource
    asset_server: Res<AssetServer>,
    level: Res<Level>,
    mut bread_count: ResMut<BreadCount>,
) {
    for _ in events_update.read() {
        // Do not despawn ducks, update the translations of ducks
        // Do not despawn the arrow hint
        for object in &object_query {
            commands.entity(object).despawn();
        }

        spawn_sprites(
            &mut commands,
            &level.0,
            &asset_server,
            &mut bread_count,
            &mut events,
            true,
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
                translation: Vec3::new(position.x, position.y, 1.0),
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
        Duck {
            logic_position,
            is_stuffed: false,
            can_move: true,
        },
        Object,
    ));
}

fn spawn_sprites(
    commands: &mut Commands,
    level: &[Vec<char>],
    asset_server: &Res<AssetServer>,
    bread_count: &mut ResMut<BreadCount>,
    // event
    events: &mut EventWriter<Won>,
    // when updates, do not respawn ducks
    is_update: bool,
) {
    bread_count.0 = 0;
    // spawn the sprites
    for (row_index, row) in level.iter().enumerate() {
        for (col_index, &ch) in row.iter().enumerate() {
            let position = logic_position_to_translation((col_index, row_index));
            let object_type = match ch {
                '@' => ObjectType::Wall,
                '#' => ObjectType::Ice,
                'D' => ObjectType::DuckOnIce,
                'B' => ObjectType::BreadOnIce,
                '*' => ObjectType::BreakingIce,
                'P' => ObjectType::DuckOnWater,
                'O' => ObjectType::DuckOnBreakingIce,
                _ => continue,
            };

            match object_type {
                ObjectType::Wall => {
                    spawn_object(commands, position, asset_server.load("sprites/wall.png"));
                }
                ObjectType::Ice => {
                    spawn_object(commands, position, asset_server.load("sprites/ice.png"));
                }
                ObjectType::DuckOnIce => {
                    spawn_object(commands, position, asset_server.load("sprites/ice.png"));
                    if !is_update {
                        spawn_duck(
                            commands,
                            position,
                            asset_server.load("sprites/duck.png"),
                            (col_index, row_index),
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
                    if !is_update {
                        spawn_duck(
                            commands,
                            position,
                            asset_server.load("sprites/duck.png"),
                            (col_index, row_index),
                        );
                    }
                }
                ObjectType::DuckOnBreakingIce => {
                    spawn_object(
                        commands,
                        position,
                        asset_server.load("sprites/breaking_ice.png"),
                    );
                    if !is_update {
                        spawn_duck(
                            commands,
                            position,
                            asset_server.load("sprites/duck.png"),
                            (col_index, row_index),
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
    level_index: ResMut<CurrentLevelIndex>,
    levels: Res<Levels>,
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
            events,
        );
    }
}

fn load_other_level(
    mut commands: Commands,
    // query
    object_query: Query<Entity, With<Object>>,
    // resource
    level_index: ResMut<CurrentLevelIndex>,
    asset_server: Res<AssetServer>,
    bread_count: ResMut<BreadCount>,
    total_bread_count: ResMut<TotalBreadCount>,
    levels: Res<Levels>,
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
    if input.just_pressed(KeyCode::BracketLeft) {
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
