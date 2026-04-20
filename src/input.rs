use crossterm::event::{KeyCode, KeyEvent};

use crate::app::Command;

pub fn map_key_to_command(key: KeyEvent) -> Command {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => Command::Quit,
        KeyCode::Left => Command::MoveLeft,
        KeyCode::Right => Command::MoveRight,
        KeyCode::Up => Command::MoveUp,
        KeyCode::Down => Command::MoveDown,
        _ => Command::NoOp,
    }
}
