use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::app::Command;

pub fn map_key_to_command(key: KeyEvent, is_input_mode: bool) -> Command {
    if key.kind != KeyEventKind::Press {
        return Command::NoOp;
    }

    if is_input_mode {
        return map_input_mode_key(key.code);
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => Command::Quit,
        KeyCode::Left => Command::MoveLeft,
        KeyCode::Right => Command::MoveRight,
        KeyCode::Up => Command::MoveUp,
        KeyCode::Down => Command::MoveDown,
        KeyCode::Char('a') | KeyCode::Char('A') => Command::AddCard,
        KeyCode::Char('m') | KeyCode::Char('M') => Command::MoveCardForward,
        KeyCode::Char('d') | KeyCode::Char('D') => Command::DeleteCard,
        _ => Command::NoOp,
    }
}

fn map_input_mode_key(code: KeyCode) -> Command {
    match code {
        KeyCode::Esc => Command::CancelInput,
        KeyCode::Enter => Command::ConfirmInput,
        KeyCode::Backspace => Command::BackspaceInput,
        KeyCode::Tab => Command::CyclePriority,
        KeyCode::Left => Command::MoveLeft,
        KeyCode::Right => Command::MoveRight,
        KeyCode::Char('p') | KeyCode::Char('P') => Command::CyclePriority,
        KeyCode::Char(c) => Command::InputChar(c),
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

        assert_eq!(map_key_to_command(key, false), Command::NoOp);
    }

    #[test]
    fn maps_add_card_key() {
        let key = KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        };

        assert_eq!(map_key_to_command(key, false), Command::AddCard);
    }

    #[test]
    fn keeps_characters_in_input_mode() {
        let key = KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        };

        assert_eq!(map_key_to_command(key, true), Command::InputChar('q'));
    }
}
