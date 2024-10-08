// This attr removes the console on release builds on Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_wasm_window_resize::WindowResizePlugin;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;

mod game;

fn main() {
    App::new()
        .insert_resource(ClearColor(game::DARK_MODE_BG_COLOR))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Jam 4 🦀".into(),
                        // Bind to canvas included in 'index.html'
                        mode: bevy::window::WindowMode::Windowed,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()) // All textures are pixelated
                .set(AssetPlugin {
                    // Fix web load assets issue
                    meta_check: AssetMetaCheck::Never,
                    ..Default::default()
                }),
        )
        .add_plugins(game::Plugin)
        .add_plugins(TweeningPlugin)
        .add_plugins(WindowResizePlugin)
        //.add_plugins(WorldInspectorPlugin::new()) // Only add this in debug version
        .run();
}
