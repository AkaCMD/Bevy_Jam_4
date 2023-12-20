use bevy::{prelude::*, render::camera::ScalingMode, window::PrimaryWindow};
use bevy_tweening::{lens::*, *};

mod audio;
mod cursor;
mod level;
mod player;
mod ui;
mod utils;

use utils::{Direction, *};

pub const RESIZE: f32 = 0.1;
pub const SPRITE_SIZE: f32 = 640.0 * RESIZE;

pub const MY_ORANGE: Color = Color::rgb(222.0 / 255.0, 112.0 / 255.0, 40.0 / 255.0);
pub const MY_BROWN: Color = Color::rgb(91.0 / 255.0, 75.0 / 255.0, 73.0 / 255.0);
pub const DARK_MODE_BG_COLOR: Color = Color::rgb(45.0 / 255.0, 47.0 / 255.0, 47.0 / 255.0);
pub const LIGHT_MODE_BG_COLOR: Color = Color::rgb(200.0 / 255.0, 200.0 / 255.0, 205.0 / 255.0);

#[derive(Resource)]
pub struct ResizeFactor(f32);
impl Default for ResizeFactor {
    fn default() -> Self {
        ResizeFactor(0.1)
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            player::Plugin,
            audio::Plugin,
            level::Plugin,
            ui::Plugin,
            cursor::Plugin,
        ))
        .init_resource::<ResizeFactor>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, on_resize_window);
    }
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let mut my_2d_camera_bundle = Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    };
    my_2d_camera_bundle.projection.scaling_mode = ScalingMode::FixedHorizontal(1280.0);
    commands.spawn(my_2d_camera_bundle);
}

// TODO: How to scale all the ui elements?
fn on_resize_window(
    mut resize_reader: EventReader<bevy::window::WindowResized>,
    mut resize_factor: ResMut<ResizeFactor>,
) {
    for e in resize_reader.read() {
        resize_factor.0 = 0.1 * e.height / 720.0;
        info!("{}", resize_factor.0);

        // Resize all the ui
    }
}
