use crate::model::Board;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Quit,
    ToggleHelp,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    AddCard,
    StartEditCard,
    MoveCardForward,
    DeleteCard,
    CycleSelectedPriority,
    InputChar(char),
    BackspaceInput,
    ConfirmInput,
    CancelInput,
    CyclePriority,
    NoOp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppMode {
    Normal,
    AddCard(AddCardDraft),
    EditCard(EditCardDraft),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddCardStep {
    Title,
    Description,
    Priority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditCardStep {
    Title,
    Description,
    Priority,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddCardDraft {
    pub title: String,
    pub description: String,
    pub priority: crate::model::Priority,
    pub step: AddCardStep,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditCardDraft {
    pub card_id: u64,
    pub title: String,
    pub description: String,
    pub priority: crate::model::Priority,
    pub step: EditCardStep,
}

impl Default for AddCardDraft {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            priority: crate::model::Priority::Medium,
            step: AddCardStep::Title,
        }
    }
}

#[derive(Debug, Clone)]
pub struct App {
    pub board: Board,
    pub should_quit: bool,
    pub mode: AppMode,
    pub show_help: bool,
    pub status_message: String,
}

impl App {
    #[cfg(test)]
    pub fn new() -> Self {
        Self::from_board(Board::default())
    }

    #[cfg(test)]
    pub fn from_board(board: Board) -> Self {
        Self::from_board_with_status(board, "Ready")
    }

    pub fn from_board_with_status(board: Board, status_message: impl Into<String>) -> Self {
        Self {
            board,
            should_quit: false,
            mode: AppMode::Normal,
            show_help: false,
            status_message: status_message.into(),
        }
    }

    pub fn is_input_mode(&self) -> bool {
        !matches!(self.mode, AppMode::Normal)
    }

    pub fn apply_command(&mut self, command: Command) -> bool {
        if self.is_input_mode() {
            return self.apply_modal_command(command);
        }

        match command {
            Command::Quit => {
                self.should_quit = true;
                false
            }
            Command::ToggleHelp => {
                self.show_help = !self.show_help;
                false
            }
            Command::MoveLeft => {
                self.move_selection_left();
                false
            }
            Command::MoveRight => {
                self.move_selection_right();
                false
            }
            Command::MoveUp => {
                self.move_selection_up();
                false
            }
            Command::MoveDown => {
                self.move_selection_down();
                false
            }
            Command::AddCard => {
                self.start_add_card();
                false
            }
            Command::StartEditCard => {
                self.start_edit_card();
                false
            }
            Command::MoveCardForward => self.move_selected_card_forward(),
            Command::DeleteCard => self.delete_selected_card(),
            Command::CycleSelectedPriority => self.cycle_selected_priority(),
            Command::NoOp => false,
            Command::InputChar(_)
            | Command::BackspaceInput
            | Command::ConfirmInput
            | Command::CancelInput
            | Command::CyclePriority => false,
        }
    }

    pub fn current_mode_label(&self) -> &'static str {
        match self.mode {
            AppMode::Normal => "Normal",
            AppMode::AddCard(_) => "Add Card",
            AppMode::EditCard(_) => "Edit Card",
        }
    }

    fn apply_modal_command(&mut self, command: Command) -> bool {
        match &self.mode {
            AppMode::AddCard(_) => self.apply_add_card_command(command),
            AppMode::EditCard(_) => self.apply_edit_card_command(command),
            AppMode::Normal => false,
        }
    }

    fn apply_add_card_command(&mut self, command: Command) -> bool {
        match command {
            Command::CancelInput | Command::Quit => {
                self.mode = AppMode::Normal;
                self.status_message = "Card creation cancelled".to_string();
                false
            }
            Command::InputChar(c) => {
                if let AppMode::AddCard(draft) = &mut self.mode {
                    match draft.step {
                        AddCardStep::Title => draft.title.push(c),
                        AddCardStep::Description => draft.description.push(c),
                        AddCardStep::Priority => {}
                    }
                }
                false
            }
            Command::BackspaceInput => {
                if let AppMode::AddCard(draft) = &mut self.mode {
                    match draft.step {
                        AddCardStep::Title => {
                            draft.title.pop();
                        }
                        AddCardStep::Description => {
                            draft.description.pop();
                        }
                        AddCardStep::Priority => {}
                    }
                }
                false
            }
            Command::CyclePriority | Command::MoveLeft | Command::MoveRight => {
                if let AppMode::AddCard(draft) = &mut self.mode
                    && draft.step == AddCardStep::Priority
                {
                    draft.priority = Board::next_priority(draft.priority);
                }
                false
            }
            Command::ConfirmInput => {
                if let AppMode::AddCard(draft) = &mut self.mode {
                    match draft.step {
                        AddCardStep::Title => {
                            if draft.title.trim().is_empty() {
                                self.status_message = "Title is required".to_string();
                            } else {
                                draft.step = AddCardStep::Description;
                            }
                        }
                        AddCardStep::Description => {
                            draft.step = AddCardStep::Priority;
                        }
                        AddCardStep::Priority => {
                            let title = draft.title.trim().to_string();
                            let description = draft.description.trim().to_string();
                            let priority = draft.priority;
                            let target_column = self.board.selected_column;

                            self.board
                                .add_card(title, description, priority, target_column);
                            self.mode = AppMode::Normal;
                            self.status_message = "Card created".to_string();
                            self.board.clamp_selection();
                            return true;
                        }
                    }
                }

                false
            }
            Command::ToggleHelp => false,
            Command::MoveUp
            | Command::MoveDown
            | Command::AddCard
            | Command::StartEditCard
            | Command::MoveCardForward
            | Command::DeleteCard
            | Command::CycleSelectedPriority
            | Command::NoOp => false,
        }
    }

    fn apply_edit_card_command(&mut self, command: Command) -> bool {
        match command {
            Command::CancelInput | Command::Quit => {
                self.mode = AppMode::Normal;
                self.status_message = "Edit cancelled".to_string();
                false
            }
            Command::InputChar(c) => {
                if let AppMode::EditCard(draft) = &mut self.mode {
                    match draft.step {
                        EditCardStep::Title => draft.title.push(c),
                        EditCardStep::Description => draft.description.push(c),
                        EditCardStep::Priority => {}
                    }
                }
                false
            }
            Command::BackspaceInput => {
                if let AppMode::EditCard(draft) = &mut self.mode {
                    match draft.step {
                        EditCardStep::Title => {
                            draft.title.pop();
                        }
                        EditCardStep::Description => {
                            draft.description.pop();
                        }
                        EditCardStep::Priority => {}
                    }
                }
                false
            }
            Command::CyclePriority | Command::MoveLeft | Command::MoveRight => {
                if let AppMode::EditCard(draft) = &mut self.mode
                    && draft.step == EditCardStep::Priority
                {
                    draft.priority = Board::next_priority(draft.priority);
                }
                false
            }
            Command::ConfirmInput => {
                let mut updated_card = None;

                if let AppMode::EditCard(draft) = &mut self.mode {
                    match draft.step {
                        EditCardStep::Title => {
                            if draft.title.trim().is_empty() {
                                self.status_message = "Title is required".to_string();
                            } else {
                                draft.step = EditCardStep::Description;
                            }
                        }
                        EditCardStep::Description => {
                            draft.step = EditCardStep::Priority;
                        }
                        EditCardStep::Priority => {
                            updated_card = Some((
                                draft.card_id,
                                draft.title.trim().to_string(),
                                draft.description.trim().to_string(),
                                draft.priority,
                            ));
                        }
                    }
                }

                if let Some((card_id, title, description, priority)) = updated_card {
                    if self
                        .board
                        .update_card(card_id, title, description, priority)
                    {
                        self.mode = AppMode::Normal;
                        self.status_message = "Card updated".to_string();
                        return true;
                    } else {
                        self.mode = AppMode::Normal;
                        self.status_message = "Card no longer exists".to_string();
                    }
                }

                false
            }
            Command::ToggleHelp => false,
            Command::MoveUp
            | Command::MoveDown
            | Command::AddCard
            | Command::StartEditCard
            | Command::MoveCardForward
            | Command::DeleteCard
            | Command::CycleSelectedPriority
            | Command::NoOp => false,
        }
    }

    fn start_add_card(&mut self) {
        self.mode = AppMode::AddCard(AddCardDraft::default());
        self.status_message = "Add card mode".to_string();
    }

    fn start_edit_card(&mut self) {
        if let Some(card) = self.board.selected_card() {
            self.mode = AppMode::EditCard(EditCardDraft {
                card_id: card.id,
                title: card.title.clone(),
                description: card.description.clone(),
                priority: card.priority,
                step: EditCardStep::Title,
            });
            self.status_message = "Edit card mode".to_string();
        } else {
            self.status_message = "No card selected".to_string();
        }
    }

    fn move_selected_card_forward(&mut self) -> bool {
        if self.board.move_selected_card_forward() {
            self.status_message = "Card moved".to_string();
            true
        } else {
            self.status_message = "No card selected".to_string();
            false
        }
    }

    fn delete_selected_card(&mut self) -> bool {
        if self.board.delete_selected_card() {
            self.status_message = "Card deleted".to_string();
            true
        } else {
            self.status_message = "No card selected".to_string();
            false
        }
    }

    fn cycle_selected_priority(&mut self) -> bool {
        if self.board.cycle_selected_card_priority() {
            self.status_message = "Priority changed".to_string();
            true
        } else {
            self.status_message = "No card selected".to_string();
            false
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_card_from_add_mode() {
        let mut app = App::new();
        let initial_len = app.board.cards.len();

        app.apply_command(Command::AddCard);
        app.apply_command(Command::InputChar('T'));
        app.apply_command(Command::InputChar('1'));
        app.apply_command(Command::ConfirmInput);
        app.apply_command(Command::InputChar('D'));
        app.apply_command(Command::ConfirmInput);
        app.apply_command(Command::ConfirmInput);

        assert_eq!(app.board.cards.len(), initial_len + 1);
        assert!(matches!(app.mode, AppMode::Normal));
    }

    #[test]
    fn cycles_priority_in_normal_mode() {
        let mut app = App::new();
        app.board.selected_column = crate::model::Column::Todo;
        app.board.selected_index = 0;

        app.apply_command(Command::CycleSelectedPriority);

        let card = app
            .board
            .selected_card()
            .expect("selected card should exist");
        assert_eq!(card.priority, crate::model::Priority::Low);
    }

    #[test]
    fn edits_card_title_and_description() {
        let mut app = App::new();
        app.board.selected_column = crate::model::Column::Todo;
        app.board.selected_index = 0;

        app.apply_command(Command::StartEditCard);
        for _ in 0.."Project skeleton".chars().count() {
            app.apply_command(Command::BackspaceInput);
        }
        app.apply_command(Command::InputChar('N'));
        app.apply_command(Command::InputChar('e'));
        app.apply_command(Command::InputChar('w'));
        app.apply_command(Command::ConfirmInput);

        for _ in 0.."Create modules and baseline architecture".chars().count() {
            app.apply_command(Command::BackspaceInput);
        }
        app.apply_command(Command::InputChar('X'));
        app.apply_command(Command::ConfirmInput);
        app.apply_command(Command::ConfirmInput);

        let card = app
            .board
            .selected_card()
            .expect("selected card should exist");
        assert_eq!(card.title, "New");
        assert_eq!(card.description, "X");
    }
}
