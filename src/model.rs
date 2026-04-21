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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
