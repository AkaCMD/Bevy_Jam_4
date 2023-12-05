use std::fs;

use super::{player::SpawnDuck, *};
use bevy::window::PrimaryWindow;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_level);
    }
}

struct Level(Vec<Vec<char>>);

impl Default for Level {
    fn default() -> Self {
        Level(Vec::new())
    }
}

enum ObjectType {
    Wall,
    Ice,
    DuckOnIce,
}

#[derive(Component)]
struct Object {}

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
    mut events: EventWriter<SpawnDuck>,
) {
    let window = window_query.get_single().unwrap();
    // Load the level from a .txt file
    if let Ok(level) = load_level_from_file("assets/levels/level1.txt") {
        for row in level.0.iter() {
            for ch in row {
                print!("{}", ch);
            }
            println!();
        }

        // spawn the sprites
        for (row_index, row) in level.0.iter().enumerate() {
            for (col_index, &ch) in row.iter().enumerate() {
                let position = Vec3::new(
                    col_index as f32 * SPRITE_SIZE + window.height() / 4.0,
                    row_index as f32 * (-SPRITE_SIZE) + window.height() * (3.0 / 4.0),
                    0.0,
                );
                let obeject_type = match ch {
                    '@' => ObjectType::Wall,
                    '#' => ObjectType::Ice,
                    'D' => ObjectType::DuckOnIce,
                    _ => continue,
                };

                let sprite_handle: Handle<Image> = match obeject_type {
                    ObjectType::Wall => asset_server.load("sprites/wall.png"),
                    ObjectType::Ice => asset_server.load("sprites/ice.png"),
                    ObjectType::DuckOnIce => {
                        events.send(SpawnDuck(position));
                        asset_server.load("sprites/ice.png")
                    }
                };

                commands.spawn((
                    SpriteBundle {
                        texture: sprite_handle.into(),
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
        }
    }
}
