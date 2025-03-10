use bevy::input::InputPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use board::BoardPlugin;
use input::BoardInputPlugin;
use rendering::BoardRenderPlugin;
use types::{Coordinates, GameSettings, GameState, PlayerPosition};

mod board;
mod input;
mod rendering;
mod types;

const GAME_SETTINGS: GameSettings = GameSettings {
    dimensions: (15, 20),
    bomb_count: 50,
};

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, InputPlugin, StatesPlugin, MinesweeperPlugin))
        .run();
}

pub struct MinesweeperPlugin;

impl Plugin for MinesweeperPlugin {
    fn build(&self, app: &mut App) {
        let (w, h) = GAME_SETTINGS.dimensions;

        let center: Coordinates = (w / 2, h / 2).into();

        app.init_state::<GameState>()
            .insert_resource(GAME_SETTINGS)
            .insert_resource(PlayerPosition(center))
            .add_plugins((BoardPlugin, BoardRenderPlugin, BoardInputPlugin));
    }
}
