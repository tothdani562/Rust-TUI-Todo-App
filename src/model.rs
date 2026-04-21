use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Column {
    Todo,
    Doing,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Card {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub column: Column,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Board {
    pub cards: Vec<Card>,
    pub selected_column: Column,
    pub selected_index: usize,
}

impl Board {
    pub fn columns() -> [Column; 3] {
        [Column::Todo, Column::Doing, Column::Done]
    }

    pub fn column_index(column: Column) -> usize {
        match column {
            Column::Todo => 0,
            Column::Doing => 1,
            Column::Done => 2,
        }
    }

    pub fn column_at_index(index: usize) -> Column {
        Self::columns()[index.min(Self::columns().len() - 1)]
    }

    pub fn next_column(column: Column) -> Column {
        match column {
            Column::Todo => Column::Doing,
            Column::Doing => Column::Done,
            Column::Done => Column::Todo,
        }
    }

    pub fn next_priority(priority: Priority) -> Priority {
        match priority {
            Priority::Low => Priority::Medium,
            Priority::Medium => Priority::High,
            Priority::High => Priority::Low,
        }
    }

    pub fn cards_in_column(&self, column: Column) -> Vec<&Card> {
        self.cards
            .iter()
            .filter(|card| card.column == column)
            .collect()
    }

    pub fn selected_column_cards(&self) -> Vec<&Card> {
        self.cards_in_column(self.selected_column)
    }

    pub fn selected_card(&self) -> Option<&Card> {
        self.selected_column_cards()
            .get(self.selected_index)
            .copied()
    }

    pub fn selected_card_id(&self) -> Option<u64> {
        self.selected_card().map(|card| card.id)
    }

    pub fn add_card(
        &mut self,
        title: String,
        description: String,
        priority: Priority,
        column: Column,
    ) -> u64 {
        let id = self.next_card_id();
        self.cards.push(Card {
            id,
            title,
            description,
            priority,
            column,
        });
        id
    }

    pub fn move_selected_card_forward(&mut self) -> bool {
        let selected_id = self.selected_card().map(|card| card.id);

        let Some(card_id) = selected_id else {
            return false;
        };

        if let Some(card) = self.cards.iter_mut().find(|card| card.id == card_id) {
            card.column = Self::next_column(card.column);
            self.clamp_selection();
            return true;
        }

        false
    }

    pub fn delete_selected_card(&mut self) -> bool {
        let selected_id = self.selected_card().map(|card| card.id);

        let Some(card_id) = selected_id else {
            return false;
        };

        let initial_len = self.cards.len();
        self.cards.retain(|card| card.id != card_id);
        let changed = self.cards.len() != initial_len;

        if changed {
            self.clamp_selection();
        }

        changed
    }

    pub fn cycle_selected_card_priority(&mut self) -> bool {
        let Some(card_id) = self.selected_card_id() else {
            return false;
        };

        if let Some(card) = self.cards.iter_mut().find(|card| card.id == card_id) {
            card.priority = Self::next_priority(card.priority);
            return true;
        }

        false
    }

    pub fn update_card(
        &mut self,
        card_id: u64,
        title: String,
        description: String,
        priority: Priority,
    ) -> bool {
        if let Some(card) = self.cards.iter_mut().find(|card| card.id == card_id) {
            card.title = title;
            card.description = description;
            card.priority = priority;
            return true;
        }

        false
    }

    fn next_card_id(&self) -> u64 {
        self.cards.iter().map(|card| card.id).max().unwrap_or(0) + 1
    }

    pub fn clamp_selection(&mut self) {
        let cards_in_column = self.selected_column_cards().len();

        if cards_in_column == 0 {
            self.selected_index = 0;
            return;
        }

        self.selected_index = self.selected_index.min(cards_in_column - 1);
    }

    pub fn with_sample_cards() -> Self {
        Self {
            cards: vec![
                Card {
                    id: 1,
                    title: "Project skeleton".to_string(),
                    description: "Create modules and baseline architecture".to_string(),
                    priority: Priority::High,
                    column: Column::Todo,
                },
                Card {
                    id: 2,
                    title: "Domain model".to_string(),
                    description: "Define Card, Column, Priority, Board".to_string(),
                    priority: Priority::Medium,
                    column: Column::Doing,
                },
                Card {
                    id: 3,
                    title: "Compile check".to_string(),
                    description: "Verify project builds without runtime features".to_string(),
                    priority: Priority::Low,
                    column: Column::Done,
                },
            ],
            selected_column: Column::Todo,
            selected_index: 0,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::with_sample_cards()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn move_card_wraps_after_done() {
        let mut board = Board::with_sample_cards();
        board.selected_column = Column::Done;
        board.selected_index = 0;

        let moved = board.move_selected_card_forward();
        assert!(moved);

        let moved_card = board
            .cards
            .iter()
            .find(|card| card.id == 3)
            .expect("sample card should exist");
        assert_eq!(moved_card.column, Column::Todo);
    }

    #[test]
    fn delete_selected_card_removes_one() {
        let mut board = Board::with_sample_cards();
        board.selected_column = Column::Todo;
        board.selected_index = 0;

        let deleted = board.delete_selected_card();
        assert!(deleted);
        assert_eq!(board.cards.len(), 2);
    }

    #[test]
    fn cycles_selected_priority() {
        let mut board = Board::with_sample_cards();
        board.selected_column = Column::Todo;
        board.selected_index = 0;

        let changed = board.cycle_selected_card_priority();
        assert!(changed);

        let card = board
            .cards
            .iter()
            .find(|card| card.id == 1)
            .expect("sample card should exist");
        assert_eq!(card.priority, Priority::Low);
    }

    #[test]
    fn empty_column_keeps_selection_stable() {
        let mut board = Board {
            cards: vec![Card {
                id: 1,
                title: "Only card".to_string(),
                description: String::new(),
                priority: Priority::Medium,
                column: Column::Todo,
            }],
            selected_column: Column::Doing,
            selected_index: 9,
        };

        board.clamp_selection();

        assert!(board.selected_card().is_none());
        assert_eq!(board.selected_index, 0);
    }

    #[test]
    fn board_json_roundtrip_preserves_state() {
        let board = Board::with_sample_cards();

        let json = serde_json::to_string(&board).expect("board should serialize");
        let restored: Board = serde_json::from_str(&json).expect("board should deserialize");

        assert_eq!(restored, board);
    }
}
