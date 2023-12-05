use bevy::prelude::*;

mod audio;
mod level;
mod player;
mod ui;

pub const RESIZE: f32 = 0.1;
pub const SPRITE_SIZE: f32 = 640.0 * RESIZE;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((player::Plugin, audio::Plugin, level::Plugin, ui::Plugin));
    }
}
