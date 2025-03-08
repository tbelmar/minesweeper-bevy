use bevy::input::{common_conditions::*, InputPlugin};
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::ops::Add;
use KeyCode::{KeyA, KeyD, KeyE, KeyQ, KeyS, KeyW};

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
    let settings = GameSettings {
        dimensions: Dimensions(10, 10),
        bomb_count: 15,
    };

    let center: Coordinates = (settings.dimensions.0 / 2, settings.dimensions.1 / 2).into();

    App::new()
        .add_plugins(InputPlugin)
        .insert_resource(Board::default())
        .insert_resource(settings)
        .insert_resource(PlayerPosition(center))
        .add_systems(Startup, initialize_board)
        .add_systems(Update, handle_wasd_input)
        .add_systems(
            Update,
            (
                open_tile.run_if(input_just_pressed(KeyQ)),
                flag_tile.run_if(input_just_pressed(KeyE)),
            ),
        )
        .run();
}

#[derive(Resource, Copy, Clone)]
struct GameSettings {
    dimensions: Dimensions,
    bomb_count: i32,
}

#[derive(Resource)]
struct PlayerPosition(Coordinates);

#[derive(Copy, Clone)]
struct Dimensions(i32, i32);

#[derive(Event)]
struct OpenTileEvent(Coordinates);

fn initialize_board(settings: Res<GameSettings>, mut board: ResMut<Board>) {
    let GameSettings {
        dimensions: Dimensions(width, height),
        bomb_count,
    } = *settings.into_inner();

    let mut tiles = HashMap::new();

    let mut bombs = 0;

    // Insert bombs
    while bombs < bomb_count {
        let mut coordinates = Coordinates { x: 0, y: 0 };

        loop {
            coordinates.x = rand::random_range(0..width);
            coordinates.y = rand::random_range(0..height);

            if tiles.get(&coordinates).is_none() {
                break;
            }
        }

        tiles.insert(
            coordinates,
            Tile {
                content: TileType::Bomb,
                open: false,
                flagged: false,
            },
        );

        bombs += 1;
    }

    // Populate tiles
    for x in 0..width {
        for y in 0..height {
            let coordinates = Coordinates { x, y };

            let mut neighbor_count = 0;

            if tiles.get(&coordinates).is_none() {
                for neighbor in NEIGHBORS {
                    if tiles.get(&(coordinates + neighbor.into())).is_some() {
                        neighbor_count += 1;
                    }
                }
            }

            tiles.insert(
                coordinates,
                Tile {
                    content: TileType::Number(neighbor_count),
                    open: false,
                    flagged: false,
                },
            );
        }
    }

    board.width = width;
    board.height = height;
    board.tiles = tiles;
}

fn handle_wasd_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_pos: ResMut<PlayerPosition>,
) {
    let Coordinates { mut x, mut y } = player_pos.0;

    if keyboard_input.just_pressed(KeyW) {
        y -= 1;
    }
    if keyboard_input.just_pressed(KeyA) {
        x -= 1;
    }
    if keyboard_input.just_pressed(KeyS) {
        y += 1;
    }
    if keyboard_input.just_pressed(KeyD) {
        x += 1;
    }

    player_pos.0 = Coordinates { x, y }
}

fn open_tile(mut board: ResMut<Board>, player_pos: Res<PlayerPosition>) {
    if let Some(tile) = board.tiles.get_mut(&player_pos.0) {
        if !tile.flagged {
            tile.open = true;
        }
    }
}

fn flag_tile(mut board: ResMut<Board>, player_pos: Res<PlayerPosition>) {
    if let Some(tile) = board.tiles.get_mut(&player_pos.0) {
        if !tile.open {
            tile.flagged = !tile.flagged;
        }
    }
}

#[derive(Resource, Default)]
struct Board {
    width: i32,
    height: i32,
    tiles: HashMap<Coordinates, Tile>,
}

#[derive(Component)]
struct Tile {
    content: TileType,
    open: bool,
    flagged: bool,
}

enum TileType {
    Bomb,
    Number(i32),
}

#[derive(Component, PartialEq, Eq, Hash, Copy, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
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
