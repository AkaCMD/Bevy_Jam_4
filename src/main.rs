// This attr removes the console on release builds on Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{asset::AssetMetaCheck, prelude::*};
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;

mod game;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(game::LIGHT_MODE_BG_COLOR))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Jam 4 🦀".into(),
                        mode: if cfg!(debug_assertions) {
                            bevy::window::WindowMode::Windowed
                        } else {
                            bevy::window::WindowMode::BorderlessFullscreen
                        },
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), // All textures are pixelated
        )
        .add_plugins(game::Plugin)
        .add_plugins(TweeningPlugin)
        //.add_plugins(WorldInspectorPlugin::new()) // Only add this in debug version
        .run();
}
