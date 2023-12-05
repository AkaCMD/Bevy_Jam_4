use super::*;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, show_title);
    }
}

fn show_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        TextBundle::from_section(
            "QUARK ON ICE!",
            TextStyle {
                font: asset_server.load("fonts/FROGBLOCK-V2.1-by-Polyducks.ttf"),
                font_size: 30.0,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Right)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        }),
    );
}
