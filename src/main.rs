use bevy::prelude::*;

mod game;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Jam 4 🦀".into(),
                        mode: bevy::window::WindowMode::Windowed,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), // All textures are pixelated
        )
        .add_plugins(game::Plugin)
        .run();
}
