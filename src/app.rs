use crate::model::Board;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Quit,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    NoOp,
}

#[derive(Debug, Clone)]
pub struct App {
    pub board: Board,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            should_quit: false,
        }
    }

    pub fn apply_command(&mut self, command: Command) {
        if let Command::Quit = command {
            self.should_quit = true;
        }
    }
}
