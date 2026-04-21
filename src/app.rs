use crate::model::Board;

macro_rules! set_status {
    ($app:ident, $message:expr) => {{
        $app.status_message = $message.to_string();
    }};
}

macro_rules! close_modal {
    ($app:ident, $message:expr) => {{
        $app.mode = AppMode::Normal;
        set_status!($app, $message);
        false
    }};
}

macro_rules! ignore_command {
    () => {
        false
    };
}

macro_rules! mode_label {
    ($mode:expr) => {{
        match $mode {
            AppMode::Normal => "Normal",
            AppMode::AddCard(_) => "Add Card",
            AppMode::EditCard(_) => "Edit Card",
            AppMode::ViewCard(_) => "Card Details",
        }
    }};
}

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
    ViewSelectedCard,
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
    ViewCard(CardPreview),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardPreview {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub priority: crate::model::Priority,
    pub column: crate::model::Column,
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
            Command::ViewSelectedCard => {
                self.open_selected_card_preview();
                false
            }
            Command::NoOp => false,
            Command::InputChar(_)
            | Command::BackspaceInput
            | Command::ConfirmInput
            | Command::CancelInput
            | Command::CyclePriority => ignore_command!(),
        }
    }

    pub fn current_mode_label(&self) -> &'static str {
        mode_label!(self.mode)
    }

    fn apply_modal_command(&mut self, command: Command) -> bool {
        match &self.mode {
            AppMode::AddCard(_) => self.apply_add_card_command(command),
            AppMode::EditCard(_) => self.apply_edit_card_command(command),
            AppMode::ViewCard(_) => self.apply_view_card_command(command),
            AppMode::Normal => false,
        }
    }

    fn apply_view_card_command(&mut self, command: Command) -> bool {
        match command {
            Command::CancelInput | Command::ConfirmInput | Command::Quit => {
                close_modal!(self, "Card details closed")
            }
            Command::ToggleHelp | Command::NoOp => false,
            Command::MoveLeft
            | Command::MoveRight
            | Command::MoveUp
            | Command::MoveDown
            | Command::AddCard
            | Command::StartEditCard
            | Command::MoveCardForward
            | Command::DeleteCard
            | Command::CycleSelectedPriority
            | Command::ViewSelectedCard
            | Command::InputChar(_)
            | Command::BackspaceInput
            | Command::CyclePriority => ignore_command!(),
        }
    }

    fn apply_add_card_command(&mut self, command: Command) -> bool {
        match command {
            Command::CancelInput | Command::Quit => close_modal!(self, "Card creation cancelled"),
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
                                set_status!(self, "Title is required");
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
                            set_status!(self, "Card created");
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
            | Command::ViewSelectedCard
            | Command::NoOp => ignore_command!(),
        }
    }

    fn apply_edit_card_command(&mut self, command: Command) -> bool {
        match command {
            Command::CancelInput | Command::Quit => close_modal!(self, "Edit cancelled"),
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
                                set_status!(self, "Title is required");
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
                        set_status!(self, "Card updated");
                        return true;
                    } else {
                        self.mode = AppMode::Normal;
                        set_status!(self, "Card no longer exists");
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
            | Command::ViewSelectedCard
            | Command::NoOp => ignore_command!(),
        }
    }

    fn start_add_card(&mut self) {
        self.mode = AppMode::AddCard(AddCardDraft::default());
        set_status!(self, "Add card mode");
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
            set_status!(self, "Edit card mode");
        } else {
            set_status!(self, "No card selected");
        }
    }

    fn move_selected_card_forward(&mut self) -> bool {
        if self.board.move_selected_card_forward() {
            set_status!(self, "Card moved");
            true
        } else {
            set_status!(self, "No card selected");
            false
        }
    }

    fn delete_selected_card(&mut self) -> bool {
        if self.board.delete_selected_card() {
            set_status!(self, "Card deleted");
            true
        } else {
            set_status!(self, "No card selected");
            false
        }
    }

    fn cycle_selected_priority(&mut self) -> bool {
        if self.board.cycle_selected_card_priority() {
            set_status!(self, "Priority changed");
            true
        } else {
            set_status!(self, "No card selected");
            false
        }
    }

    fn open_selected_card_preview(&mut self) {
        if let Some(card) = self.board.selected_card() {
            self.mode = AppMode::ViewCard(CardPreview {
                id: card.id,
                title: card.title.clone(),
                description: card.description.clone(),
                priority: card.priority,
                column: card.column,
            });
            set_status!(self, "Card details opened");
        } else {
            set_status!(self, "No card selected");
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
