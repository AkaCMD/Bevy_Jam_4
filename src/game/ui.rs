use super::*;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, show_title)
            .add_event::<Won>()
            .add_systems(Update, won);
    }
}

fn show_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        TextBundle::from_section(
            "Quark On Ice!",
            TextStyle {
                font: asset_server.load("fonts/NotJamChunky8.ttf"),
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

#[derive(Event, Default)]
pub struct Won;

fn won(mut commands: Commands, asset_server: Res<AssetServer>, mut events: EventReader<Won>) {
    for _ in events.read() {
        commands.spawn(
            TextBundle::from_section(
                "Win!",
                TextStyle {
                    font: asset_server.load("fonts/NotJamChunky8.ttf"),
                    font_size: 30.0,
                    ..default()
                },
            )
            .with_text_alignment(TextAlignment::Right)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            }),
        );
    }
}
