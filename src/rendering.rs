use crate::{
    board::Board,
    types::{Coordinates, GameState, PlayerPosition},
};
use bevy::prelude::*;
use crossterm::{
    cursor, execute, queue,
    style::{self, Attribute, Color, Print},
    terminal::{self, Clear, ClearType, EnterAlternateScreen},
};
use std::io::{stdout, Write};

pub fn setup_crossterm() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, EnterAlternateScreen, Clear(ClearType::All)).unwrap();
}

pub fn render_board(
    board: Res<Board>,
    player_pos: Res<PlayerPosition>,
    game_state: Res<State<GameState>>,
) {
    let mut stdout = stdout();

    queue!(stdout, Clear(ClearType::All)).unwrap();

    for (&Coordinates { x, y }, tile) in board.tiles.iter() {
        queue!(
            stdout,
            cursor::MoveTo((x * 2) as u16, y as u16),
            style::SetForegroundColor(Color::DarkGrey),
            Print(tile),
            style::ResetColor,
            cursor::MoveTo((player_pos.0.x * 2) as u16, player_pos.0.y as u16)
        )
        .unwrap();
    }

    queue!(
        stdout,
        cursor::SavePosition,
        cursor::MoveTo(board.width as u16 * 2, 0),
        style::SetAttribute(Attribute::Bold),
        Print("Bevy Minesweeper"),
        cursor::MoveTo(board.width as u16 * 2, 1),
        Print("Author: "),
        style::SetAttribute(Attribute::Reset),
        Print("tbelmar <tomasbelmarcosta@gmail.com>"),
        cursor::RestorePosition
    )
    .unwrap();

    match game_state.get() {
        GameState::Lost => {
            queue!(
                stdout,
                cursor::SavePosition,
                style::SetAttribute(Attribute::Bold),
                style::SetForegroundColor(Color::Red),
                cursor::MoveTo(board.width as u16 * 2, 3),
                Print("YOU LOST "),
                style::SetAttribute(Attribute::Reset),
                style::ResetColor,
                Print(":("),
                cursor::MoveTo(board.width as u16 * 2, 4),
                Print("WOMP\nWOMP"),
                cursor::RestorePosition,
            )
            .unwrap();
        }
        GameState::Won => {
            queue!(
                stdout,
                cursor::SavePosition,
                cursor::MoveTo(board.width as u16 * 2, 3),
                style::SetAttribute(Attribute::Bold),
                style::SetForegroundColor(Color::Green),
                Print("WINNER WINNER"),
                style::SetAttribute(Attribute::Reset),
                style::ResetColor,
                cursor::MoveTo(board.width as u16 * 2, 4),
                Print("CHICKEN DINNER!"),
                cursor::RestorePosition,
            )
            .unwrap();
        }
        _ => {
            queue!(
                stdout,
                cursor::SavePosition,
                cursor::MoveTo(board.width as u16 * 2, 2),
                Print("[WASD] Move"),
                cursor::MoveTo(board.width as u16 * 2, 3),
                Print("[Q] Open"),
                cursor::MoveTo(board.width as u16 * 2, 4),
                Print("[E] Flag"),
                cursor::MoveTo(board.width as u16 * 2, 5),
                Print("[Esc] Quit"),
                cursor::RestorePosition
            )
            .unwrap();
        }
    }

    stdout.flush().unwrap();
}
