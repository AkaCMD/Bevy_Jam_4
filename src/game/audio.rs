use bevy::audio::{PlaybackMode, Volume};

use super::*;

pub struct Plugin;

#[derive(Event, Default)]
pub struct PlaySFX {
    pub source: Handle<AudioSource>,
    pub volume: Volume,
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Next), play_bgm)
            .add_event::<PlaySFX>()
            .add_systems(Update, play_sfx.run_if(in_state(GameStates::Next)));
    }
}

fn play_bgm(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn(AudioBundle {
        source: audio_assets.bgm.clone(),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(0.03),
            ..default()
        },
    });
}

pub fn play_sfx(mut events: EventReader<PlaySFX>, mut commands: Commands) {
    for event in events.read() {
        commands.spawn(AudioBundle {
            source: event.source.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: event.volume,
                ..default()
            },
        });
    }
}
