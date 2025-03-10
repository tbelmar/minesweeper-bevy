use bevy::input::InputPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use board::{input::BoardInputPlugin, rendering::BoardRenderPlugin, BoardPlugin, Coordinates};

mod board;

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
