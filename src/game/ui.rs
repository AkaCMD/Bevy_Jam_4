use bevy::window::PrimaryWindow;

use super::{
    level::{BreadCount, CurrentLevelIndex, TotalBreadCount},
    *,
};
pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                show_title_and_name,
                show_level_title,
                show_hints,
                show_stuffed_ducks_count,
            ),
        )
        .add_event::<Won>()
        .add_systems(
            Update,
            (
                won,
                update_level_title,
                next_level_button,
                update_stuffed_ducks_count,
            ),
        );
    }
}

fn show_title_and_name(mut commands: Commands, asset_server: Res<AssetServer>) {
    // game title
    commands.spawn(
        TextBundle::from_section(
            "QUARK!!! on ICE",
            TextStyle {
                font: asset_server.load("fonts/NotJamChunky8.ttf"),
                font_size: 30.0,
                color: Color::ORANGE,
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

    // author name
    commands.spawn(
        TextBundle::from_section(
            "a game by Minda Chen",
            TextStyle {
                font: asset_server.load("fonts/NotJamChunky8.ttf"),
                font_size: 20.0,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Right)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            right: Val::Px(10.0),
            ..default()
        }),
    );
}

#[derive(Component)]
struct StuffedDucksCount;

fn show_stuffed_ducks_count(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    bread_count: Res<BreadCount>,
    total_bread_count: Res<TotalBreadCount>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        TextBundle::from_section(
            format!(
                "{}/{}",
                total_bread_count.0 - bread_count.0,
                total_bread_count.0
            ),
            TextStyle {
                font: asset_server.load("fonts/NotJamChunky8.ttf"),
                font_size: 30.0,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(window.width() / 2.0 - 45.0),
            ..default()
        }),
        StuffedDucksCount,
    ));
}

fn update_stuffed_ducks_count(
    bread_count: Res<BreadCount>,
    total_bread_count: Res<TotalBreadCount>,
    mut stuffed_ducks_count: Query<&mut Text, With<StuffedDucksCount>>,
) {
    if bread_count.is_changed() || total_bread_count.is_changed() {
        for mut text in stuffed_ducks_count.iter_mut() {
            text.sections[0].value = format!(
                "{}/{}",
                total_bread_count.0 - bread_count.0,
                total_bread_count.0
            );
        }
    }
}

#[derive(Component)]
struct LevelTitle;

fn show_level_title(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_index: Res<CurrentLevelIndex>,
) {
    commands.spawn((
        TextBundle::from_section(
            format!("Level{}", level_index.0),
            TextStyle {
                font: asset_server.load("fonts/NotJamChunky8.ttf"),
                font_size: 30.0,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        LevelTitle,
    ));
}

// HINTS:
// Click to choose the duck
// WASD to move
// R to reset
// One duck, one bread
fn show_hints(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style_important = TextStyle {
        font: asset_server.load("fonts/NotJamChunky8.ttf"),
        font_size: 20.0,
        color: Color::ORANGE,
    };
    let text_style_normal = TextStyle {
        font: asset_server.load("fonts/NotJamChunky8.ttf"),
        font_size: 20.0,
        ..default()
    };
    commands.spawn((TextBundle::from_sections([
        TextSection::new("Click ", text_style_important.clone()),
        TextSection::new("to choose the duck\n", text_style_normal.clone()),
        TextSection::new("WASD ", text_style_important.clone()),
        TextSection::new("to move\n", text_style_normal.clone()),
        TextSection::new("R ", text_style_important.clone()),
        TextSection::new("to reset\n", text_style_normal.clone()),
        TextSection::new("[ ] ", text_style_important.clone()),
        TextSection::new("to skip levels\n\n", text_style_normal.clone()),
        TextSection::new("One duck, one bread\n", text_style_important.clone()),
    ])
    .with_text_alignment(TextAlignment::Right)
    .with_style(Style {
        position_type: PositionType::Absolute,
        top: Val::Px(300.0),
        right: Val::Px(10.0),
        ..default()
    }),));
}

fn update_level_title(
    mut commands: Commands,
    level_index: Res<CurrentLevelIndex>,
    mut level_title: Query<&mut Text, With<LevelTitle>>,
    ui_query: Query<Entity, With<MutUI>>,
) {
    if level_index.is_changed() {
        for mut text in level_title.iter_mut() {
            text.sections[0].value = format!("Level{}", level_index.0);
        }

        // Despawn ui elements
        for entity in ui_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.75, 0.75, 0.75);

#[derive(Event, Default)]
pub struct Won;

#[derive(Component)]
pub struct MutUI;

fn won(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<Won>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for _ in events.read() {
        let window = window_query.get_single().unwrap();
        commands.spawn((
            TextBundle::from_section(
                "Win!",
                TextStyle {
                    font: asset_server.load("fonts/NotJamChunky8.ttf"),
                    font_size: 40.0,
                    color: Color::ORANGE,
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(40.0),
                right: Val::Px(window.width() / 2.0 - 80.0),
                ..default()
            }),
            MutUI,
        ));

        // Show next level button
        commands
            .spawn((NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },))
            .insert(MutUI)
            .with_children(|parent| {
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .insert(MutUI)
                    .with_children(|parent| {
                        parent
                            .spawn(TextBundle::from_section(
                                "Next Level",
                                TextStyle {
                                    font: asset_server.load("fonts/NotJamChunky8.ttf"),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ))
                            .insert(MutUI);
                    });
            });
    }
}

fn next_level_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut level_index: ResMut<CurrentLevelIndex>,
    levels: Res<level::Levels>,
) {
    // Handle invalid level index
    if level::load_level(level_index.0, levels).is_err() {
        info!("Invalid level index");
        level_index.0 -= 1;
        return;
    }
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::WHITE;
                level_index.0 += 1;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
