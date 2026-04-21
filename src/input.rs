use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::app::Command;

macro_rules! key_command_map {
    ($value:expr, { $( $pattern:pat => $command:expr ),+ $(,)? ; _ => $fallback:expr }) => {{
        match $value {
            $( $pattern => $command, )+
            _ => $fallback,
        }
    }};
}

pub fn map_key_to_command(key: KeyEvent, is_input_mode: bool) -> Command {
    if key.kind != KeyEventKind::Press {
        return Command::NoOp;
    }

    if is_input_mode {
        return map_input_mode_key(key.code);
    }

    key_command_map!(key.code, {
        KeyCode::Char('q') | KeyCode::Char('Q') => Command::Quit,
        KeyCode::Char('h') | KeyCode::Char('H') => Command::ToggleHelp,
        KeyCode::Left => Command::MoveLeft,
        KeyCode::Right => Command::MoveRight,
        KeyCode::Up => Command::MoveUp,
        KeyCode::Down => Command::MoveDown,
        KeyCode::Char('a') | KeyCode::Char('A') => Command::AddCard,
        KeyCode::Char('e') | KeyCode::Char('E') => Command::StartEditCard,
        KeyCode::Char('m') | KeyCode::Char('M') => Command::MoveCardForward,
        KeyCode::Char('d') | KeyCode::Char('D') => Command::DeleteCard,
        KeyCode::Char('p') | KeyCode::Char('P') => Command::CycleSelectedPriority,
        KeyCode::Enter => Command::ViewSelectedCard;
        _ => Command::NoOp
    })
}

fn map_input_mode_key(code: KeyCode) -> Command {
    key_command_map!(code, {
        KeyCode::Esc => Command::CancelInput,
        KeyCode::Enter => Command::ConfirmInput,
        KeyCode::Backspace => Command::BackspaceInput,
        KeyCode::Tab => Command::CyclePriority,
        KeyCode::Left => Command::MoveLeft,
        KeyCode::Right => Command::MoveRight,
        KeyCode::Char(c) => Command::InputChar(c);
        _ => Command::NoOp
    })
}
