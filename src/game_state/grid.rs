use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameState {
    pub grid: [[i32; 4]; 4],
}

impl GameState {
    pub fn new(grid: [[i32; 4]; 4]) -> Self {
        Self { grid: grid.clone() }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState { grid: [[0; 4]; 4] }
    }
}
