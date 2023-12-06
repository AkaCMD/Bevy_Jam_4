use bevy::audio::PlaybackMode;

use super::*;

pub struct Plugin;

#[derive(Resource)]
struct Sfx(Handle<AudioSource>);

#[derive(Event, Default)]
pub struct PlaySFX;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_bgm, load_sfx))
            .add_event::<PlaySFX>()
            .add_systems(Update, play_sfx);
    }
}

fn load_bgm(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/bgm.wav"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: bevy::audio::Volume::new_absolute(0.5),
            ..default()
        },
    });
}

fn load_sfx(mut commands: Commands, asset_server: Res<AssetServer>) {
    let quark_sound = asset_server.load("audio/quark.wav");

    // Insert the Audio resource with the loaded sound effect
    commands.insert_resource(Sfx(quark_sound));
}

fn play_sfx(sfx_assets: Res<Sfx>, mut events: EventReader<PlaySFX>, mut commands: Commands) {
    for _ in events.read() {
        let quark_sound = sfx_assets.0.clone();
        commands.spawn(AudioBundle {
            source: quark_sound,
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
