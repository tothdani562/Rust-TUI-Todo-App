use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::app::Command;

pub fn map_key_to_command(key: KeyEvent) -> Command {
    if key.kind != KeyEventKind::Press {
        return Command::NoOp;
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => Command::Quit,
        KeyCode::Left => Command::MoveLeft,
        KeyCode::Right => Command::MoveRight,
        KeyCode::Up => Command::MoveUp,
        KeyCode::Down => Command::MoveDown,
        _ => Command::NoOp,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyModifiers, KeyEventKind};

    #[test]
    fn ignores_repeat_events() {
        let key = KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Repeat,
            state: crossterm::event::KeyEventState::NONE,
        };

        assert_eq!(map_key_to_command(key), Command::NoOp);
    }
}
