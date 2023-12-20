use bevy::prelude::*;
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

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            player::Plugin,
            audio::Plugin,
            level::Plugin,
            ui::Plugin,
            cursor::Plugin,
        ));
    }
}
