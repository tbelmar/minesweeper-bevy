use bevy::input::InputPlugin;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crossterm::event::{
    Event as CrosstermEvent, KeyCode as CrosstermKeyCode, KeyEvent as CrosstermKeyEvent,
};
use crossterm::style::Print;
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, event, execute, queue};
use std::fmt::Display;
use std::io::{stdout, Write};
use std::ops::Add;

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
        bomb_count: 10,
    };

    let center: Coordinates = (settings.dimensions.0 / 2, settings.dimensions.1 / 2).into();

    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(InputPlugin)
        .add_event::<OpenTileEvent>()
        .add_event::<FlagTileEvent>()
        .insert_resource(Board::default())
        .insert_resource(settings)
        .insert_resource(PlayerPosition(center))
        .insert_resource(GameState::Playing)
        .add_systems(Startup, setup_crossterm)
        .add_systems(Startup, initialize_board)
        .add_systems(Update, render_board)
        .add_systems(Update, handle_input)
        .add_systems(
            Update,
            ((open_tile, open_adjacent_tiles).chain(), flag_tile),
        )
        .run();
}

#[derive(Resource)]
enum GameState {
    Won,
    Lost,
    Playing,
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
struct OpenTileEvent(Coordinates, bool);

#[derive(Event)]
struct FlagTileEvent(Coordinates);

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

            let mut neighbor_bomb_count = 0;

            if tiles.get(&coordinates).is_none() {
                for neighbor in NEIGHBORS {
                    if let Some(Tile {
                        content: TileType::Bomb,
                        ..
                    }) = tiles.get(&(coordinates + neighbor.into()))
                    {
                        neighbor_bomb_count += 1;
                    }
                }

                tiles.insert(
                    coordinates,
                    Tile {
                        content: TileType::Number(neighbor_bomb_count),
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

fn handle_input(
    mut player_pos: ResMut<PlayerPosition>,
    board: ResMut<Board>,
    mut ev_open_tile: EventWriter<OpenTileEvent>,
    mut ev_flag_tile: EventWriter<FlagTileEvent>,
    mut ev_exit: ResMut<Events<AppExit>>,
) {
    let Coordinates { mut x, mut y } = player_pos.0;

    if event::poll(std::time::Duration::from_millis(100)).unwrap() {
        if let CrosstermEvent::Key(CrosstermKeyEvent { code, .. }) = event::read().unwrap() {
            match code {
                CrosstermKeyCode::Esc => {
                    let mut stdout = stdout();
                    terminal::disable_raw_mode().unwrap();
                    execute!(stdout, Clear(ClearType::All), LeaveAlternateScreen).unwrap();
                    ev_exit.send(AppExit::Success);
                }
                CrosstermKeyCode::Char('w') => y -= 1,
                CrosstermKeyCode::Char('a') => x -= 1,
                CrosstermKeyCode::Char('s') => y += 1,
                CrosstermKeyCode::Char('d') => x += 1,
                CrosstermKeyCode::Char('q') => {
                    ev_open_tile.send(OpenTileEvent(player_pos.0, true));
                }
                CrosstermKeyCode::Char('e') => {
                    ev_flag_tile.send(FlagTileEvent(player_pos.0));
                }
                _ => {}
            }
        }
    }

    if x < board.width && x >= 0 && y < board.height && y >= 0 {
        player_pos.0 = Coordinates { x, y };
    }
}

fn open_tile(
    mut board: ResMut<Board>,
    mut evs_open_tile: ParamSet<(EventReader<OpenTileEvent>, EventWriter<OpenTileEvent>)>,
    mut game_state: ResMut<GameState>,
) {
    let mut neighbors_to_open = Vec::<Coordinates>::new();

    for OpenTileEvent(pos, original) in evs_open_tile.p0().read() {
        let mut flag_count = 0;

        if let Some(tile) = board.tiles.get_mut(pos) {
            if !tile.flagged && !tile.open {
                tile.open = true;
                match tile.content {
                    TileType::Bomb => {
                        *game_state = GameState::Lost;
                    }
                    TileType::Number(_) => {
                        board.tiles_left -= 1;
                        if board.tiles_left == 0 {
                            *game_state = GameState::Won
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
            content: TileType::Number(n),
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
        evs_open_tile.p1().send(OpenTileEvent(*pos, false));
    }
}

fn open_adjacent_tiles(
    board: ResMut<Board>,
    mut evs_open_tile: ParamSet<(EventReader<OpenTileEvent>, EventWriter<OpenTileEvent>)>,
) {
    let mut neighbors = Vec::<Coordinates>::new();

    for OpenTileEvent(pos, _) in evs_open_tile.p0().read() {
        if let Some(Tile {
            content: TileType::Number(0),
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

fn flag_tile(mut board: ResMut<Board>, mut ev_flag_tile: EventReader<FlagTileEvent>) {
    for FlagTileEvent(pos) in ev_flag_tile.read() {
        if let Some(tile) = board.tiles.get_mut(pos) {
            if !tile.open {
                tile.flagged = !tile.flagged;
            }
        }
    }
}

#[derive(Resource, Default)]
struct Board {
    width: i32,
    height: i32,
    tiles: HashMap<Coordinates, Tile>,
    tiles_left: i32,
}

#[derive(Component)]
struct Tile {
    content: TileType,
    open: bool,
    flagged: bool,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.open {
                match self.content {
                    TileType::Bomb => "* ".to_string(),
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

fn setup_crossterm() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, EnterAlternateScreen, Clear(ClearType::All)).unwrap();
}

fn render_board(board: Res<Board>, player_pos: Res<PlayerPosition>, game_state: Res<GameState>) {
    let mut stdout = stdout();

    queue!(stdout, Clear(ClearType::All)).unwrap();

    for (&Coordinates { x, y }, tile) in board.tiles.iter() {
        queue!(
            stdout,
            cursor::MoveTo((x * 2) as u16, y as u16),
            Print(tile),
            cursor::MoveTo((player_pos.0.x * 2) as u16, player_pos.0.y as u16)
        )
        .unwrap();
    }

    match game_state.into_inner() {
        GameState::Lost => {
            queue!(
                stdout,
                cursor::SavePosition,
                cursor::MoveTo(board.width as u16 * 2, 0),
                Print("[YOU LOST]"),
                cursor::RestorePosition,
            )
            .unwrap();
        }
        GameState::Won => {
            queue!(
                stdout,
                cursor::SavePosition,
                cursor::MoveTo(board.width as u16 * 2, 0),
                Print("[YOU WON]"),
                cursor::RestorePosition,
            )
            .unwrap();
        }
        _ => {
            queue!(
                stdout,
                cursor::SavePosition,
                cursor::MoveTo(board.width as u16 * 2, 0),
                Print(board.tiles_left),
                cursor::RestorePosition
            )
            .unwrap();
        }
    }

    stdout.flush().unwrap();
}
