use bevy::{prelude::*, render::camera::ScalingMode, window::PrimaryWindow};
use bevy_asset_loader::prelude::*;
use bevy_tweening::{lens::*, *};

mod audio;
mod cursor;
mod level;
mod player;
mod ui;
mod utils;

use utils::*;

pub const RESIZE: f32 = 0.1;
pub const SPRITE_SIZE: f32 = 640.0 * RESIZE;

pub const MY_ORANGE: Color = Color::srgb(222.0 / 255.0, 112.0 / 255.0, 40.0 / 255.0);
pub const MY_BROWN: Color = Color::srgb(91.0 / 255.0, 75.0 / 255.0, 73.0 / 255.0);
pub const DARK_MODE_BG_COLOR: Color = Color::srgb(45.0 / 255.0, 47.0 / 255.0, 47.0 / 255.0);

pub const DUCK_MOVE_MILI_SECS: u64 = 300;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameStates>()
            .add_loading_state(
                LoadingState::new(GameStates::Loading)
                    .continue_to_state(GameStates::Next)
                    .load_collection::<AudioAssets>()
                    .load_collection::<ImageAssets>(),
            )
            .add_plugins((
                player::Plugin,
                audio::Plugin,
                level::Plugin,
                ui::Plugin,
                cursor::Plugin,
            ))
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let mut my_2d_camera_bundle = Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    };
    my_2d_camera_bundle.projection.scaling_mode = ScalingMode::FixedHorizontal(1280.0);
    commands.spawn(my_2d_camera_bundle);
}

// TODO: How to scale all the ui elements?

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/bgm.ogg")]
    bgm: Handle<AudioSource>,
    #[asset(path = "audio/eat.ogg")]
    eat: Handle<AudioSource>,
    #[asset(path = "audio/ice_breaking.ogg")]
    ice_breaking: Handle<AudioSource>,
    #[asset(path = "audio/quark.ogg")]
    quark: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "sprites/arrow.png")]
    arrow: Handle<Image>,
    #[asset(path = "sprites/bread.png")]
    bread: Handle<Image>,
    #[asset(path = "sprites/breaking_ice.png")]
    breaking_ice: Handle<Image>,
    #[asset(path = "sprites/click_hint.png")]
    click_hint: Handle<Image>,
    #[asset(path = "sprites/duck.png")]
    duck: Handle<Image>,
    #[asset(path = "sprites/ice.png")]
    ice: Handle<Image>,
    #[asset(path = "sprites/stuffed_duck.png")]
    stuffed_duck: Handle<Image>,
    #[asset(path = "sprites/wall.png")]
    wall: Handle<Image>,
    #[asset(path = "sprites/water.png")]
    water: Handle<Image>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameStates {
    #[default]
    Loading,
    Next,
}
