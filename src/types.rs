use std::ops::Add;

use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Coordinates {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Resource, Copy, Clone)]
pub struct GameSettings {
    pub dimensions: (i32, i32),
    pub bomb_count: i32,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Won,
    Lost,
    #[default]
    Playing,
}

#[derive(Resource)]
pub struct PlayerPosition(pub Coordinates);

#[derive(Event)]
pub struct OpenTileEvent(pub Coordinates, pub bool);

#[derive(Event)]
pub struct FlagTileEvent(pub Coordinates);
