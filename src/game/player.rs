use super::{audio::PlaySFX, *};
use bevy::window::PrimaryWindow;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, player_movement)
            .add_event::<SpawnDuck>()
            .add_systems(Update, spawn_duck);
    }
}

#[derive(Component)]
pub struct Player {}

#[derive(Event, Default)]
pub struct SpawnDuck(pub Vec3);

fn spawn_duck(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut events: EventReader<SpawnDuck>,
) {
    for event in events.read() {
        let window = window_query.get_single().unwrap();
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: event.0,
                    rotation: Quat::IDENTITY,
                    scale: Vec3::new(1.0 * RESIZE, 1.0 * RESIZE, 1.0),
                },
                texture: asset_server.load("sprites/duck.png"),
                ..default()
            },
            Player {},
        ));
    }
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

// TODO: select the duck as player
fn player_movement(
    key_board_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Sprite), With<Player>>,
    mut events: EventWriter<PlaySFX>,
) {
    if let Ok((mut transform, mut sprite)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if key_board_input.just_pressed(KeyCode::Left) || key_board_input.just_pressed(KeyCode::A) {
            direction = Vec3::new(-1.0, 0.0, 0.0);
            sprite.flip_x = false;
        }
        if key_board_input.just_pressed(KeyCode::Right) || key_board_input.just_pressed(KeyCode::D)
        {
            direction = Vec3::new(1.0, 0.0, 0.0);
            sprite.flip_x = true;
        }
        if key_board_input.just_pressed(KeyCode::Up) || key_board_input.just_pressed(KeyCode::W) {
            direction = Vec3::new(0.0, 1.0, 0.0);
        }
        if key_board_input.just_pressed(KeyCode::Down) || key_board_input.just_pressed(KeyCode::S) {
            direction = Vec3::new(0.0, -1.0, 0.0);
        }

        transform.translation += direction * SPRITE_SIZE;

        if direction.length() > 0.0 {
            // play quark sound
            events.send(PlaySFX);
        }
    }
}
