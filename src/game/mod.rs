use bevy::prelude::*;

mod audio;
mod cursor;
mod level;
mod player;
mod ui;
mod utils;

use utils::{Direction, *};

pub const RESIZE: f32 = 0.1;
pub const SPRITE_SIZE: f32 = 640.0 * RESIZE;

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
