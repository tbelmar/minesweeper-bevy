use std::io::stdout;

use crate::{board::Board, PlayerPosition};
use bevy::{input::InputPlugin, prelude::*};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, Clear, ClearType},
};

use super::{Coordinates, FlagTileEvent, OpenTileEvent};

pub struct BoardInputPlugin;

impl Plugin for BoardInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputPlugin)
            .add_systems(Update, handle_input);
    }
}

pub fn handle_input(
    mut player_pos: ResMut<PlayerPosition>,
    board: ResMut<Board>,
    mut ev_open_tile: EventWriter<OpenTileEvent>,
    mut ev_flag_tile: EventWriter<FlagTileEvent>,
    mut ev_exit: ResMut<Events<AppExit>>,
) {
    let Coordinates { mut x, mut y } = player_pos.0;

    if event::poll(std::time::Duration::from_millis(100)).unwrap() {
        if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
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
