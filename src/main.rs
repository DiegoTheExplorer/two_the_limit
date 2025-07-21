mod game_state;

use crate::game_state::tfe_tile::TileValue;
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
    .add_event::<ShiftTilesTrigger>()
    .add_event::<SpawnTile>()
    .init_resource::<GameState>()
    .add_systems(Startup, (spawn_layout, init_game).chain())
    .add_systems(
        Update,
        (user_input_handler, shift_tiles, spawn_tile, update_ui).chain(),
    )
    .run();
}

fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/TerminessNerdFont-Regular.ttf");
    commands.spawn(Camera2d);

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
                            spawn_ui_tile(builder, LIGHT_GRAY, Position { x, y });
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

fn spawn_ui_tile(builder: &mut ChildSpawnerCommands, color: Srgba, pos: Position) {
    builder.spawn((
        Node {
            display: Display::Grid,
            padding: UiRect::all(Val::Px(3.0)),
            grid_column: GridPlacement::start(pos.y),
            grid_row: GridPlacement::start(pos.x),
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

    game_state.grid[(init_tile_1.y - 1) as usize][(init_tile_1.x - 1) as usize] = 2;
    game_state.grid[(init_tile_2.y - 1) as usize][(init_tile_2.x - 1) as usize] = 2;
}

fn spawn_tile(mut events: EventReader<SpawnTile>, game_state: ResMut<GameState>) {
    events.read().for_each(|_| {
        let mut rng = rand::rng();
        let mut x = rng.random_range(0..4);
        let mut y = rng.random_range(0..4);

        while game_state.grid[x][y] != 0 {
            x = rng.random_range(0..4);
            y = rng.random_range(0..4);
        }

        game_state.grid[y][x];
    });
}

#[derive(Event)]
struct ShiftTilesTrigger(KeyCode);

#[derive(Event)]
struct SpawnTile;

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

fn has_adjacent_dupes(arr: &[i32; 4]) -> bool {
    let mut iter = arr.iter().filter(|val| **val != 0).peekable();
    while let Some(current) = iter.next() {
        if let Some(next) = iter.peek() {
            if *current == **next {
                return true;
            }
        }
    }
    false
}

fn shift_zeroes_right(arr: &mut [i32; 4]) {
    let mut temp;
    for i in 0..3 {
        for j in (i + 1)..4 {
            if arr[i] == 0 {
                temp = arr[j];
                arr[j] = 0;
                arr[i] = temp;
            }
        }
    }
}

fn shift_tiles(
    mut ew: EventWriter<SpawnTile>,
    mut events: EventReader<ShiftTilesTrigger>,
    mut game_state: ResMut<GameState>,
) {
    events.read().for_each(|event| {
        match event.0 {
            KeyCode::ArrowLeft => {
                for y in 0..4 {
                    if game_state.grid[y].iter().all(|val| *val == 0) {
                        continue;
                    }

                    let mut temp_arr = [0; 4];
                    temp_arr.clone_from_slice(&game_state.grid[y]);
                    temp_arr.reverse();

                    // Combination no jutsu
                    while has_adjacent_dupes(&temp_arr) {
                        shift_zeroes_right(&mut temp_arr);
                    }

                    temp_arr.reverse();
                    for i in 0..4 {
                        game_state.grid[y][i] = temp_arr[i];
                    }
                }
                debug_game_state(&game_state);
            }
            KeyCode::ArrowRight => {
                debug_game_state(&game_state);
            }
            KeyCode::ArrowUp => {
                for y in 1..4 {
                    for x in 0..4 {
                        if game_state.grid[y][x] == 0 {
                            continue;
                        }
                        game_state.grid[y - 1][x] = game_state.grid[y][x];
                        game_state.grid[y][x] = 0;
                    }
                }
                debug_game_state(&game_state);
            }
            KeyCode::ArrowDown => {
                for y in (0..3).rev() {
                    for x in (0..4).rev() {
                        if game_state.grid[y][x] == 0 {
                            continue;
                        }
                        game_state.grid[y + 1][x] = game_state.grid[y][x];
                        game_state.grid[y][x] = 0;
                    }
                }
                debug_game_state(&game_state);
            }
            _ => {}
        }
        ew.write(SpawnTile);
    });
}

fn debug_game_state(game_state: &ResMut<GameState>) {
    for y in 0..4 {
        info!(
            "[{}, {}, {}, {}]",
            game_state.grid[y][0],
            game_state.grid[y][1],
            game_state.grid[y][2],
            game_state.grid[y][3]
        );
    }
    info!("_________________________")
}

fn update_ui(
    mut tiles: Query<(
        &mut Node,
        &mut BackgroundColor,
        &mut Position,
        &mut TileValue,
        &mut Text,
    )>,
    game_state: Res<GameState>,
) {
    let gs_iter = game_state.grid.iter().flatten();
    tiles
        .iter_mut()
        .zip(gs_iter)
        .for_each(|((node, bgc, pos, tv, mut txt), g_val)| {
            txt.0 = g_val.to_string();
        });
}
