// This attr removes the console on release builds on Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_tweening::TweeningPlugin;

mod game;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never) // Fix the wasm loading issue
        .insert_resource(ClearColor(Color::rgb(
            200.0 / 255.0,
            200.0 / 255.0,
            205.0 / 255.0,
        )))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Jam 4 🦀".into(),
                        mode: bevy::window::WindowMode::Windowed,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        resizable: false, // TODO: fix the resize issue
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), // All textures are pixelated
        )
        .add_plugins(game::Plugin)
        .add_plugins(TweeningPlugin)
        .run();
}