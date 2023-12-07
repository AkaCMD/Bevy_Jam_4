use std::fs;

use super::{player::Duck, ui::Won, *};
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
            .add_systems(
                Update,
                (print_level, update_level, level_restart, load_other_level),
            );
    }
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
}

#[derive(Component)]
struct Object {}

#[derive(Event, Default)]
pub struct PrintLevel;

#[derive(Event, Default)]
pub struct UpdateLevel;

pub fn load_level_from_file(file_path: &str) -> Result<Level, std::io::Error> {
    let contents = fs::read_to_string(file_path)?;

    let level_data: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    Ok(Level(level_data))
}

fn spawn_level(
    mut commands: Commands,
    // query
    window_query: Query<&Window, With<PrimaryWindow>>,
    // resource
    asset_server: Res<AssetServer>,
    level_index: ResMut<CurrentLevelIndex>,
    mut bread_count: ResMut<BreadCount>,
    // event
    mut events: EventWriter<Won>,
) {
    // Load the level from a .txt file
    if let Ok(level) =
        load_level_from_file(format!("assets/levels/level{}.txt", level_index.0).as_str())
    {
        spawn_sprites(
            &mut commands,
            &level.0,
            &window_query,
            &asset_server,
            &mut bread_count,
            &mut events,
            false,
        );
        commands.insert_resource(level);
    }
}

fn update_level(
    mut commands: Commands,
    // event
    mut events_update: EventReader<UpdateLevel>,
    mut events: EventWriter<Won>,
    // query
    window_query: Query<&Window, With<PrimaryWindow>>,
    object_query: Query<Entity, (With<Object>, Without<Duck>)>,
    // resource
    asset_server: Res<AssetServer>,
    level: Res<Level>,
    mut bread_count: ResMut<BreadCount>,
) {
    for _ in events_update.read() {
        // Do not despawn ducks, update the translations of ducks
        for object in &object_query {
            commands.entity(object).despawn();
        }

        spawn_sprites(
            &mut commands,
            &level.0,
            &window_query,
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
        Object {},
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
            texture: sprite,
            ..default()
        },
        Duck {
            logic_position,
            is_full: false,
        },
        Object {},
    ));
}

fn spawn_sprites(
    commands: &mut Commands,
    level: &[Vec<char>],
    window_query: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
    bread_count: &mut ResMut<BreadCount>,
    // event
    events: &mut EventWriter<Won>,
    // when updates, do not respawn ducks
    is_update: bool,
) {
    bread_count.0 = 0;
    let window = window_query.get_single().unwrap();
    // spawn the sprites
    for (row_index, row) in level.iter().enumerate() {
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
                    spawn_object(commands, position, asset_server.load("sprites/wall.png"));
                }
                ObjectType::Ice => {
                    spawn_object(commands, position, asset_server.load("sprites/ice.png"));
                }
                ObjectType::DuckOnIce => {
                    spawn_object(commands, position, asset_server.load("sprites/ice.png"));
                    //events.send(SpawnDuck((col_index, row_index)));
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
            };
        }
    }

    if bread_count.0 == 0 {
        events.send(Won);
    }
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

fn level_restart(
    mut commands: Commands,
    // query
    window_query: Query<&Window, With<PrimaryWindow>>,
    object_query: Query<Entity, With<Object>>,
    ui_query: Query<Entity, With<ui::MutUI>>,
    // resource
    input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    bread_count: ResMut<BreadCount>,
    level_index: ResMut<CurrentLevelIndex>,
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
            window_query,
            asset_server,
            level_index,
            bread_count,
            events,
        );
    }
}

fn load_other_level(
    mut commands: Commands,
    // query
    object_query: Query<Entity, With<Object>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // resource
    level_index: ResMut<CurrentLevelIndex>,
    asset_server: Res<AssetServer>,
    bread_count: ResMut<BreadCount>,
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
            window_query,
            asset_server,
            level_index,
            bread_count,
            events,
        )
    }
}
