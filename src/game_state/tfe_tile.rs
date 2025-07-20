use bevy::prelude::Component;

#[derive(Component)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

#[derive(Component)]
pub struct TileValue {
    pub val: u32,
}
