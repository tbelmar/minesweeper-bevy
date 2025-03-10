use bevy::input::InputPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use board::{flag_tile, initialize_board, open_adjacent_tiles, open_tile, Board};
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::{event, execute};
use rendering::{render_board, setup_crossterm};
use std::io::stdout;
use types::{Coordinates, FlagTileEvent, GameSettings, GameState, OpenTileEvent, PlayerPosition};

mod board;
mod rendering;
mod types;

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
        dimensions: (15, 20),
        bomb_count: 50,
    };

    let (w, h) = settings.dimensions;

    let center: Coordinates = (w / 2, h / 2).into();

    App::new()
        .add_plugins((MinimalPlugins, InputPlugin, StatesPlugin))
        .init_state::<GameState>()
        .add_event::<OpenTileEvent>()
        .add_event::<FlagTileEvent>()
        .insert_resource(Board::default())
        .insert_resource(settings)
        .insert_resource(PlayerPosition(center))
        .add_systems(Startup, (setup_crossterm, initialize_board))
        .add_systems(
            Update,
            (
                render_board,
                handle_input,
                ((open_tile, open_adjacent_tiles).chain(), flag_tile)
                    .run_if(in_state(GameState::Playing)),
            ),
        )
        .run();
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
        if let CrosstermEvent::Key(KeyEvent { code, .. }) = event::read().unwrap() {
            match code {
                KeyCode::Esc => {
                    let mut stdout = stdout();
                    terminal::disable_raw_mode().unwrap();
                    execute!(stdout, Clear(ClearType::All)).unwrap();
                    ev_exit.send(AppExit::Success);
                }
                KeyCode::Char('w') | KeyCode::Up => y -= 1,
                KeyCode::Char('a') | KeyCode::Left => x -= 1,
                KeyCode::Char('s') | KeyCode::Down => y += 1,
                KeyCode::Char('d') | KeyCode::Right => x += 1,
                KeyCode::Char('q') => {
                    ev_open_tile.send(OpenTileEvent(player_pos.0, true));
                }
                KeyCode::Char('e') => {
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
