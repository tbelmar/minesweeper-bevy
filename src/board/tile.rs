use bevy::prelude::*;
use std::fmt::Display;

#[derive(Component)]
pub struct Tile {
    pub kind: TileType,
    pub open: bool,
    pub flagged: bool,
}

pub enum TileType {
    Bomb,
    Number(i32),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.open {
                match self.kind {
                    TileType::Bomb => "✹ ".to_string(),
                    TileType::Number(0) => "  ".to_string(),
                    TileType::Number(n) => n.to_string() + " ",
                }
            } else if self.flagged {
                "⚑ ".to_string()
            } else {
                "██".to_string()
            }
        )
    }
}
