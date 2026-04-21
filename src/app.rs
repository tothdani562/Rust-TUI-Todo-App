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
        match command {
            Command::Quit => self.should_quit = true,
            Command::MoveLeft => self.move_selection_left(),
            Command::MoveRight => self.move_selection_right(),
            Command::MoveUp => self.move_selection_up(),
            Command::MoveDown => self.move_selection_down(),
            Command::NoOp => {}
        }
    }

    fn move_selection_left(&mut self) {
        let current_index = Board::column_index(self.board.selected_column);

        if current_index > 0 {
            self.board.selected_column = Board::column_at_index(current_index - 1);
        }

        self.board.clamp_selection();
    }

    fn move_selection_right(&mut self) {
        let current_index = Board::column_index(self.board.selected_column);
        let last_index = Board::columns().len() - 1;

        if current_index < last_index {
            self.board.selected_column = Board::column_at_index(current_index + 1);
        }

        self.board.clamp_selection();
    }

    fn move_selection_up(&mut self) {
        if self.board.selected_index > 0 {
            self.board.selected_index -= 1;
        }

        self.board.clamp_selection();
    }

    fn move_selection_down(&mut self) {
        let visible_cards = self.board.selected_column_cards().len();

        if self.board.selected_index + 1 < visible_cards {
            self.board.selected_index += 1;
        }

        self.board.clamp_selection();
    }
}
