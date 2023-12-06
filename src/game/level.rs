use std::fs;

use super::{player::Duck, *};
use bevy::window::PrimaryWindow;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_level)
            .init_resource::<Level>()
            .init_resource::<CurrentLevelIndex>()
            .init_resource::<BreadCount>()
            .add_event::<PrintLevel>()
            .add_event::<UpdateLevel>()
            .add_systems(Update, (print_level, update_level));
    }
}

#[derive(Resource)]
pub struct Level(pub Vec<Vec<char>>);

impl Default for Level {
    fn default() -> Self {
        Level(Vec::new())
    }
}

#[derive(Resource)]
pub struct CurrentLevelIndex(i32);

impl Default for CurrentLevelIndex {
    fn default() -> Self {
        CurrentLevelIndex(1)
    }
}

#[derive(Resource)]
pub struct BreadCount(pub i32);
impl Default for BreadCount {
    fn default() -> Self {
        BreadCount(1)
    }
}
// TODO: Layers of objects (z axis)
// TODO: Press R to reset
// TODO: Refactor
enum ObjectType {
    Wall,
    Ice,
    DuckOnIce,
    BreadOnIce,
}

#[derive(Component)]
struct Object {}

#[derive(Event, Default)]
pub struct PrintLevel;

#[derive(Event, Default)]
pub struct UpdateLevel;

fn load_level_from_file(file_path: &str) -> Result<Level, std::io::Error> {
    let contents = fs::read_to_string(file_path)?;

    let level_data: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    Ok(Level(level_data))
}

pub fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    level_index: Res<CurrentLevelIndex>,
    mut bread_count: ResMut<BreadCount>,
) {
    bread_count.0 = 0;
    let window = window_query.get_single().unwrap();
    // Load the level from a .txt file
    if let Ok(level) =
        load_level_from_file(format!("assets/levels/level{}.txt", level_index.0).as_str())
    {
        // spawn the sprites
        for (row_index, row) in level.0.iter().enumerate() {
            for (col_index, &ch) in row.iter().enumerate() {
                let position = logic_position_to_translation((col_index, row_index), window);
                let object_type = match ch {
                    '@' => ObjectType::Wall,
                    '#' => ObjectType::Ice,
                    'D' => ObjectType::DuckOnIce,
                    'B' => ObjectType::BreadOnIce,
                    _ => continue,
                };

                match object_type {
                    ObjectType::Wall => {
                        spawn_object(
                            &mut commands,
                            position,
                            asset_server.load("sprites/wall.png"),
                        );
                    }
                    ObjectType::Ice => {
                        spawn_object(
                            &mut commands,
                            position,
                            asset_server.load("sprites/ice.png"),
                        );
                    }
                    ObjectType::DuckOnIce => {
                        spawn_object(
                            &mut commands,
                            position,
                            asset_server.load("sprites/ice.png"),
                        );
                        //events.send(SpawnDuck((col_index, row_index)));
                        spawn_duck(
                            &mut commands,
                            position,
                            asset_server.load("sprites/duck.png"),
                            (col_index, row_index),
                        );
                    }
                    ObjectType::BreadOnIce => {
                        bread_count.0 += 1;
                        spawn_object(
                            &mut commands,
                            position,
                            asset_server.load("sprites/ice.png"),
                        );
                        spawn_upper_object(
                            &mut commands,
                            position,
                            asset_server.load("sprites/bread.png"),
                        );
                    }
                };
            }
        }
        commands.insert_resource(level);
    }
}

pub fn update_level(
    mut commands: Commands,
    level: Res<Level>,
    mut events_update: EventReader<UpdateLevel>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    object_query: Query<Entity, With<Object>>,
    duck_query: Query<Entity, With<Duck>>,
    asset_server: Res<AssetServer>,
) {
    for _ in events_update.read() {
        // TODO: do not despawn ducks, update the translations of ducks
        for object in &object_query {
            commands.entity(object).despawn();
        }
        for duck in &duck_query {
            commands.entity(duck).despawn();
        }
        let window = window_query.get_single().unwrap();
        // spawn the sprites
        for (row_index, row) in level.0.iter().enumerate() {
            for (col_index, &ch) in row.iter().enumerate() {
                let position = logic_position_to_translation((col_index, row_index), window);
                let object_type = match ch {
                    '@' => ObjectType::Wall,
                    '#' => ObjectType::Ice,
                    'D' => ObjectType::DuckOnIce,
                    'B' => ObjectType::BreadOnIce,
                    _ => continue,
                };

                match object_type {
                    ObjectType::Wall => {
                        spawn_object(
                            &mut commands,
                            position,
                            asset_server.load("sprites/wall.png"),
                        );
                    }
                    ObjectType::Ice => {
                        spawn_object(
                            &mut commands,
                            position,
                            asset_server.load("sprites/ice.png"),
                        );
                    }
                    ObjectType::DuckOnIce => {
                        spawn_object(
                            &mut commands,
                            position,
                            asset_server.load("sprites/ice.png"),
                        );
                        //events.send(SpawnDuck((col_index, row_index)));
                        spawn_duck(
                            &mut commands,
                            position,
                            asset_server.load("sprites/duck.png"),
                            (col_index, row_index),
                        );
                    }
                    ObjectType::BreadOnIce => {
                        spawn_object(
                            &mut commands,
                            position,
                            asset_server.load("sprites/ice.png"),
                        );
                        spawn_upper_object(
                            &mut commands,
                            position,
                            asset_server.load("sprites/bread.png"),
                        );
                    }
                };
            }
        }
    }
}

fn spawn_object(commands: &mut Commands, position: Vec3, sprite: Handle<Image>) {
    commands.spawn((
        SpriteBundle {
            texture: sprite.into(),
            transform: Transform {
                translation: position,
                rotation: Quat::IDENTITY,
                scale: Vec3::new(1.0 * RESIZE, 1.0 * RESIZE, 1.0),
            },
            ..default()
        },
        Object {},
    ));
}

fn spawn_upper_object(commands: &mut Commands, position: Vec3, sprite: Handle<Image>) {
    commands.spawn((
        SpriteBundle {
            texture: sprite.into(),
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 1.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::new(1.0 * RESIZE, 1.0 * RESIZE, 1.0),
            },
            ..default()
        },
        Object {},
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
            texture: sprite.into(),
            ..default()
        },
        Duck {
            logic_position: logic_position,
        },
    ));
}

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
