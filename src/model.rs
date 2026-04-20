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
