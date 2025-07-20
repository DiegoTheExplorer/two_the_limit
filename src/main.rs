mod game_state;

use crate::game_state::tfe_tile::TileValue;
use bevy::input::keyboard::Key;
use bevy::{color::palettes::css::*, prelude::*};
use game_state::grid::GameState;
use game_state::tfe_tile::Position;
use rand::Rng;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: [800., 600.].into(),
            title: "Bevy CSS Grid Layout Example".to_string(),
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, (spawn_layout, init_game).chain())
    .add_systems(Update, (user_input_handler, shift_tiles))
    .add_event::<ShiftTilesTrigger>()
    .run();
}

fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/TerminessNerdFont-Regular.ttf");
    commands.spawn(Camera2d);
    commands.insert_resource(GameState {
        ..Default::default()
    });
    commands.init_resource::<GameState>();

    commands
        .spawn((
            Node {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
                grid_template_rows: vec![
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::px(20.),
                ],
                ..default()
            },
            BackgroundColor(Color::WHITE),
        ))
        .with_children(|builder| {
            // Header
            builder
                .spawn(Node {
                    display: Display::Grid,
                    grid_column: GridPlacement::span(2),
                    padding: UiRect::all(Val::Px(6.0)),
                    ..default()
                })
                .with_children(|builder| {
                    spawn_nested_text_bundle(builder, font.clone(), "Bevy CSS Grid Layout Example");
                });

            builder
                .spawn((
                    Node {
                        height: Val::Percent(100.0),
                        aspect_ratio: Some(1.0),
                        display: Display::Grid,
                        padding: UiRect::all(Val::Px(24.0)),
                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                        row_gap: Val::Px(12.0),
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                ))
                .with_children(|builder| {
                    for x in 1..5 {
                        for y in 1..5 {
                            spawn_tile(builder, LIGHT_GRAY, Position { x, y });
                        }
                    }
                });

            // Right side bar (auto placed in row 2, column 2)
            builder
                .spawn((
                    Node {
                        display: Display::Grid,
                        align_items: AlignItems::Start,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(10.)),
                        grid_template_rows: vec![
                            GridTrack::auto(),
                            GridTrack::auto(),
                            GridTrack::fr(1.0),
                        ],
                        row_gap: Val::Px(10.),
                        ..default()
                    },
                    BackgroundColor(BLACK.into()),
                ))
                .with_children(|builder| {
                    builder.spawn((
                        Text::new("Sidebar"),
                        TextFont {
                            font: font.clone(),
                            ..default()
                        },
                    ));
                    builder.spawn((
                        Text::new("Sample text that will auto-wrap"),
                        TextFont {
                            font: font.clone(),
                            font_size: 13.0,
                            ..default()
                        },
                    ));
                    builder.spawn(Node::default());
                });
        });
}

fn spawn_tile(builder: &mut ChildSpawnerCommands, color: Srgba, pos: Position) {
    builder.spawn((
        Node {
            display: Display::Grid,
            padding: UiRect::all(Val::Px(3.0)),
            grid_column: GridPlacement::start(pos.x),
            grid_row: GridPlacement::start(pos.y),
            align_content: AlignContent::Center,
            ..default()
        },
        BackgroundColor(color.into()),
        Position { x: pos.x, y: pos.y },
        TileValue { val: 0 },
        Text::new(""),
        TextLayout::new_with_justify(JustifyText::Center),
        TextFont {
            font_size: 200.0,
            ..default()
        },
    ));
}

fn spawn_nested_text_bundle(builder: &mut ChildSpawnerCommands, font: Handle<Font>, text: &str) {
    builder.spawn((
        Text::new(text),
        TextFont { font, ..default() },
        TextColor::BLACK,
    ));
}

fn init_game(
    mut tiles: Query<(
        &mut BackgroundColor,
        &mut Position,
        &mut TileValue,
        &mut Text,
    )>,
    mut game_state: ResMut<GameState>,
) {
    let mut rng = rand::rng();
    let init_tile_1 = Position {
        x: rng.random_range(1..5),
        y: rng.random_range(1..5),
    };
    let init_tile_2 = Position {
        x: rng.random_range(1..5),
        y: rng.random_range(1..5),
    };

    game_state.grid[init_tile_1.y as usize][init_tile_1.x as usize] = 2;
    game_state.grid[init_tile_2.y as usize][init_tile_2.x as usize] = 2;
}

#[derive(Event)]
struct ShiftTilesTrigger(KeyCode);

fn user_input_handler(
    mut ev_shift_tiles: EventWriter<ShiftTilesTrigger>,
    key_press: Res<ButtonInput<KeyCode>>,
) {
    if key_press.just_pressed(KeyCode::ArrowLeft) {
        ev_shift_tiles.write(ShiftTilesTrigger(KeyCode::ArrowLeft));
    }
    if key_press.just_pressed(KeyCode::ArrowRight) {
        ev_shift_tiles.write(ShiftTilesTrigger(KeyCode::ArrowRight));
    }
    if key_press.just_pressed(KeyCode::ArrowUp) {
        ev_shift_tiles.write(ShiftTilesTrigger(KeyCode::ArrowUp));
    }
    if key_press.just_pressed(KeyCode::ArrowDown) {
        ev_shift_tiles.write(ShiftTilesTrigger(KeyCode::ArrowDown));
    }
}

fn shift_tiles(
    mut events: EventReader<ShiftTilesTrigger>,
    mut tiles: Query<(
        &mut Node,
        &mut BackgroundColor,
        &mut Position,
        &mut TileValue,
        &mut Text,
    )>,
) {
    events.read().for_each(|key_press| {});
}
