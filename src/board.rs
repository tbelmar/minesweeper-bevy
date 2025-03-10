use crate::{
    types::{Coordinates, FlagTileEvent, GameSettings, GameState, OpenTileEvent},
    NEIGHBORS,
};
use bevy::{prelude::*, utils::HashMap};
use tile::{Tile, TileType};

pub mod tile;

#[derive(Resource, Default)]
pub struct Board {
    pub width: i32,
    pub height: i32,
    pub tiles: HashMap<Coordinates, Tile>,
    pub tiles_left: i32,
}

pub fn initialize_board(settings: Res<GameSettings>, mut board: ResMut<Board>) {
    let GameSettings {
        dimensions: (width, height),
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
                kind: TileType::Bomb,
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

            let mut neighbor_bomb_count = 0;

            if tiles.get(&coordinates).is_none() {
                for neighbor in NEIGHBORS {
                    if let Some(Tile {
                        kind: TileType::Bomb,
                        ..
                    }) = tiles.get(&(coordinates + neighbor.into()))
                    {
                        neighbor_bomb_count += 1;
                    }
                }

                tiles.insert(
                    coordinates,
                    Tile {
                        kind: TileType::Number(neighbor_bomb_count),
                        open: false,
                        flagged: false,
                    },
                );
            }
        }
    }

    board.width = width;
    board.height = height;
    board.tiles = tiles;
    board.tiles_left = width * height - bomb_count;
}

pub fn open_tile(
    mut board: ResMut<Board>,
    mut evs_open_tile: ParamSet<(EventReader<OpenTileEvent>, EventWriter<OpenTileEvent>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut neighbors_to_open = Vec::<Coordinates>::new();

    for OpenTileEvent(pos, original) in evs_open_tile.p0().read() {
        let mut flag_count = 0;

        if let Some(tile) = board.tiles.get_mut(pos) {
            if !tile.flagged && !tile.open {
                tile.open = true;
                match tile.kind {
                    TileType::Bomb => {
                        next_state.set(GameState::Lost);
                    }
                    TileType::Number(_) => {
                        board.tiles_left -= 1;
                        if board.tiles_left == 0 {
                            next_state.set(GameState::Won);
                        }
                    }
                }
            } else if tile.open {
                for neighbor in NEIGHBORS {
                    if let Some(Tile {
                        flagged: true,
                        open: false,
                        ..
                    }) = board.tiles.get(&(*pos + neighbor.into()))
                    {
                        flag_count += 1;
                    }
                }
            }
        }

        if let Some(Tile {
            kind: TileType::Number(n),
            ..
        }) = board.tiles.get(pos)
        {
            if *original && *n == flag_count {
                for neighbor in NEIGHBORS {
                    neighbors_to_open.push(*pos + neighbor.into());
                }
            }
        }
    }

    for pos in neighbors_to_open.iter() {
        if let Some(Tile { open: false, .. }) = board.tiles.get(pos) {
            evs_open_tile.p1().send(OpenTileEvent(*pos, false));
        }
    }
}

pub fn open_adjacent_tiles(
    board: ResMut<Board>,
    mut evs_open_tile: ParamSet<(EventReader<OpenTileEvent>, EventWriter<OpenTileEvent>)>,
) {
    let mut neighbors = Vec::<Coordinates>::new();

    for OpenTileEvent(pos, _) in evs_open_tile.p0().read() {
        if let Some(Tile {
            kind: TileType::Number(0),
            ..
        }) = board.tiles.get(pos)
        {
            for neighbor in NEIGHBORS {
                let neighbor_pos = *pos + neighbor.into();
                if let Some(Tile { open: false, .. }) = board.tiles.get(&neighbor_pos) {
                    neighbors.push(neighbor_pos);
                }
            }
        }
    }

    for neighbor in neighbors.iter() {
        evs_open_tile.p1().send(OpenTileEvent(*neighbor, false));
    }
}

pub fn flag_tile(mut board: ResMut<Board>, mut ev_flag_tile: EventReader<FlagTileEvent>) {
    for FlagTileEvent(pos) in ev_flag_tile.read() {
        if let Some(tile) = board.tiles.get_mut(pos) {
            if !tile.open {
                tile.flagged = !tile.flagged;
            }
        }
    }
}
