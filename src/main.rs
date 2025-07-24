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
    .add_systems(Update, (user_input_handler, shift_tiles).chain())
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
                    for y in 1..5 {
                        for x in 1..5 {
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

fn init_game(mut game_state: ResMut<GameState>) {
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
    info!("INITIAL GAME STATE");
    debug_game_state(&game_state);
}

fn spawn_tile(game_state: &mut ResMut<GameState>) {
    let mut rng = rand::rng();
    let mut x = rng.random_range(0..4);
    let mut y = rng.random_range(0..4);
    if game_state.grid[y][x] != 0 {
        while game_state.grid[y][x] != 0 {
            x = rng.random_range(0..4);
            y = rng.random_range(0..4);
        }
    }

    game_state.grid[y][x] = 2;
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
    let mut temp = [0; 4];
    temp.clone_from_slice(arr);
    for i in 0..arr.len() {
        arr[i] = 0;
    }
    for (i, val) in temp.iter().filter(|val| **val != 0).enumerate() {
        arr[i] = *val;
    }
}

fn shift_zeroes_left(arr: &mut [i32; 4]) {
    let mut temp = [0; 4];
    temp.clone_from_slice(arr);
    for i in 0..arr.len() {
        arr[i] = 0;
    }
    let zero_count = temp.iter().filter(|val| **val == 0).count();
    for (i, val) in temp.iter().filter(|val| **val != 0).enumerate() {
        arr[zero_count + i] = *val;
    }
}

fn combine_dupes(arr: &mut [i32; 4]) {
    for i in (0..3).rev() {
        if arr[i] == 0 {
            continue;
        }
        if arr[i] == arr[i + 1] {
            arr[i] = 0;
            arr[i + 1] *= 2;
        }
    }
}

fn shift_tiles(
    mut tiles: Query<(
        &mut Node,
        &mut BackgroundColor,
        &mut Position,
        &mut TileValue,
        &mut Text,
        &mut TextFont,
    )>,
    mut events: EventReader<ShiftTilesTrigger>,
    mut game_state: ResMut<GameState>,
) {
    events.read().for_each(|event| {
        let mut tiles_shifted = false;

        match event.0 {
            KeyCode::ArrowLeft => {
                for y in 0..4 {
                    if game_state.grid[y].iter().all(|val| *val == 0) {
                        continue;
                    }

                    let mut temp_arr = [0; 4];
                    let mut before_shift_arr = [0; 4];
                    temp_arr.clone_from_slice(&game_state.grid[y]);
                    before_shift_arr.clone_from_slice(&game_state.grid[y]);

                    while has_adjacent_dupes(&temp_arr) {
                        shift_zeroes_right(&mut temp_arr);
                        temp_arr.reverse();
                        combine_dupes(&mut temp_arr);
                        temp_arr.reverse();
                        shift_zeroes_right(&mut temp_arr);
                        if !tiles_shifted {
                            tiles_shifted = true;
                        }
                    }

                    shift_zeroes_right(&mut temp_arr);
                    if !before_shift_arr.eq(&temp_arr) {
                        tiles_shifted = true;
                    }
                    for i in 0..4 {
                        game_state.grid[y][i] = temp_arr[i];
                    }
                }
            }
            KeyCode::ArrowRight => {
                for y in 0..4 {
                    if game_state.grid[y].iter().all(|val| *val == 0) {
                        continue;
                    }

                    let mut temp_arr = [0; 4];
                    let mut before_shift_arr = [0; 4];
                    temp_arr.clone_from_slice(&game_state.grid[y]);
                    before_shift_arr.clone_from_slice(&game_state.grid[y]);

                    while has_adjacent_dupes(&temp_arr) {
                        shift_zeroes_left(&mut temp_arr);
                        combine_dupes(&mut temp_arr);
                        shift_zeroes_left(&mut temp_arr);
                        if !tiles_shifted {
                            tiles_shifted = true;
                        }
                    }

                    shift_zeroes_left(&mut temp_arr);
                    if !before_shift_arr.eq(&temp_arr) {
                        tiles_shifted = true;
                    }
                    for i in 0..4 {
                        game_state.grid[y][i] = temp_arr[i];
                    }
                }
            }
            KeyCode::ArrowUp => {
                for x in 0..4 {
                    let mut temp_arr = [0; 4];
                    let mut before_shift_arr = [0; 4];
                    for y in 0..4 {
                        temp_arr[y] = game_state.grid[y][x];
                    }
                    if temp_arr.iter().all(|val| *val == 0) {
                        continue;
                    }
                    before_shift_arr.clone_from_slice(&temp_arr);

                    while has_adjacent_dupes(&temp_arr) {
                        shift_zeroes_right(&mut temp_arr);
                        temp_arr.reverse();
                        combine_dupes(&mut temp_arr);
                        temp_arr.reverse();
                        shift_zeroes_right(&mut temp_arr);
                        if !tiles_shifted {
                            tiles_shifted = true;
                        }
                    }

                    shift_zeroes_right(&mut temp_arr);
                    if !before_shift_arr.eq(&temp_arr) {
                        tiles_shifted = true;
                    }
                    for i in 0..4 {
                        game_state.grid[i][x] = temp_arr[i];
                    }
                }
            }
            KeyCode::ArrowDown => {
                for x in 0..4 {
                    let mut temp_arr = [0; 4];
                    let mut before_shift_arr = [0; 4];
                    for y in 0..4 {
                        temp_arr[y] = game_state.grid[y][x];
                    }
                    if temp_arr.iter().all(|val| *val == 0) {
                        continue;
                    }
                    before_shift_arr.clone_from_slice(&temp_arr);

                    while has_adjacent_dupes(&temp_arr) {
                        shift_zeroes_left(&mut temp_arr);
                        combine_dupes(&mut temp_arr);
                        shift_zeroes_left(&mut temp_arr);
                        if !tiles_shifted {
                            tiles_shifted = true;
                        }
                    }

                    shift_zeroes_left(&mut temp_arr);
                    if !before_shift_arr.eq(&temp_arr) {
                        tiles_shifted = true;
                    }
                    for i in 0..4 {
                        game_state.grid[i][x] = temp_arr[i];
                    }
                }
            }
            _ => {}
        }
        if tiles_shifted {
            spawn_tile(&mut game_state);
        }
        update_ui(&mut tiles, &mut game_state);
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
    tiles: &mut Query<(
        &mut Node,
        &mut BackgroundColor,
        &mut Position,
        &mut TileValue,
        &mut Text,
        &mut TextFont,
    )>,
    game_state: &ResMut<GameState>,
) {
    tiles
        .iter_mut()
        .for_each(|(_node, mut bgc, pos, mut tv, mut txt, mut font)| {
            let (x, y) = (pos.x as usize, pos.y as usize);
            let game_state_val = game_state.grid[y - 1][x - 1];
            tv.val = game_state_val as u32;
            txt.0 = game_state_val.to_string();
            match game_state_val {
                0 => bgc.0 = BLACK.into(),
                2 => bgc.0 = DARK_GRAY.into(),
                4 => bgc.0 = BLUE.into(),
                8 => bgc.0 = PURPLE.into(),
                16 => {
                    bgc.0 = RED.into();
                    *font = TextFont {
                        font_size: 175.0,
                        ..default()
                    };
                }
                32 => {
                    bgc.0 = ORANGE.into();
                    *font = TextFont {
                        font_size: 175.0,
                        ..default()
                    };
                }
                64 => {
                    bgc.0 = YELLOW_GREEN.into();
                    *font = TextFont {
                        font_size: 175.0,
                        ..default()
                    };
                }
                128 => {
                    bgc.0 = GREEN.into();
                    *font = TextFont {
                        font_size: 150.0,
                        ..default()
                    };
                }
                256 => {
                    bgc.0 = PINK.into();
                    *font = TextFont {
                        font_size: 150.0,
                        ..default()
                    };
                }
                512 => {
                    bgc.0 = BROWN.into();
                    *font = TextFont {
                        font_size: 150.0,
                        ..default()
                    };
                }
                1028 => {
                    bgc.0 = SILVER.into();
                    *font = TextFont {
                        font_size: 125.0,
                        ..default()
                    };
                }
                2048 => {
                    bgc.0 = GOLD.into();
                    *font = TextFont {
                        font_size: 125.0,
                        ..default()
                    };
                }
                _ => {
                    bgc.0 = TURQUOISE.into();
                    *font = TextFont {
                        font_size: 100.0,
                        ..default()
                    };
                }
            }
        });
}
