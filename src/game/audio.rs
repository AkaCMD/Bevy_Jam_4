use bevy::audio::{PlaybackMode, Volume};

use super::*;

pub struct Plugin;

#[derive(Event, Default)]
pub struct PlaySFX {
    pub path: String,
    pub volume: Volume,
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, play_bgm)
            .add_event::<PlaySFX>()
            .add_systems(Update, play_sfx);
    }
}

fn play_bgm(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/bgm.ogg"),
        // source: asset_server.load("audio/Doc.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new_absolute(0.03),
            ..default()
        },
    });
}

pub fn play_sfx(
    mut events: EventReader<PlaySFX>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        commands.spawn(AudioBundle {
            source: asset_server.load(event.path.clone()),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: event.volume,
                ..default()
            },
        });
    }
}
