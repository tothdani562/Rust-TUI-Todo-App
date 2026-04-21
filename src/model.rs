use serde::{Deserialize, Serialize};

// A kártyák prioritásának három szintje.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

// A tábla három oszlopa.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Column {
    Todo,
    Doing,
    Done,
}

// Egyetlen feladatkártya teljes adata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Card {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub column: Column,
}

// A teljes táblaállapot, benne a kártyákkal és a kijelöléssel.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Board {
    pub cards: Vec<Card>,
    pub selected_column: Column,
    pub selected_index: usize,
}

impl Board {
    // Visszaadja a három oszlop sorrendjét.
    pub fn columns() -> [Column; 3] {
        [Column::Todo, Column::Doing, Column::Done]
    }

    // Egy oszlop helyét adja meg a fix sorrendben.
    pub fn column_index(column: Column) -> usize {
        match column {
            Column::Todo => 0,
            Column::Doing => 1,
            Column::Done => 2,
        }
    }

    // Egy indexből oszlopot választ, a túl nagy értékeket a végére húzva.
    pub fn column_at_index(index: usize) -> Column {
        Self::columns()[index.min(Self::columns().len() - 1)]
    }

    // A következő oszlopot adja vissza körkörös sorrendben.
    pub fn next_column(column: Column) -> Column {
        match column {
            Column::Todo => Column::Doing,
            Column::Doing => Column::Done,
            Column::Done => Column::Todo,
        }
    }

    // A következő prioritási szintet adja vissza körkörös sorrendben.
    pub fn next_priority(priority: Priority) -> Priority {
        match priority {
            Priority::Low => Priority::Medium,
            Priority::Medium => Priority::High,
            Priority::High => Priority::Low,
        }
    }

    // Kiválogatja a megadott oszlop kártyáit.
    pub fn cards_in_column(&self, column: Column) -> Vec<&Card> {
        self.cards
            .iter()
            .filter(|card| card.column == column)
            .collect()
    }

    // A kijelölt oszlop kártyáit adja vissza.
    pub fn selected_column_cards(&self) -> Vec<&Card> {
        self.cards_in_column(self.selected_column)
    }

    // A kijelölt kártyát adja vissza, ha az index még érvényes.
    pub fn selected_card(&self) -> Option<&Card> {
        self.selected_column_cards()
            .get(self.selected_index)
            .copied()
    }

    // A kijelölt kártya azonosítóját adja vissza.
    pub fn selected_card_id(&self) -> Option<u64> {
        self.selected_card().map(|card| card.id)
    }

    // Új kártyát ad a táblához, és visszaadja az új azonosítót.
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

    // A kijelölt kártyát a következő oszlopba mozgatja.
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

    // Törli a kijelölt kártyát a táblából.
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

    // A kijelölt kártya prioritását lépteti tovább.
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

    // Frissíti egy kártya tartalmát az azonosító alapján.
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

    // A következő szabad kártyaazonosítót számolja ki.
    fn next_card_id(&self) -> u64 {
        self.cards.iter().map(|card| card.id).max().unwrap_or(0) + 1
    }

    // A kijelölést az aktuális oszlop kártyaszámához igazítja.
    pub fn clamp_selection(&mut self) {
        let cards_in_column = self.selected_column_cards().len();

        if cards_in_column == 0 {
            self.selected_index = 0;
            return;
        }

        self.selected_index = self.selected_index.min(cards_in_column - 1);
    }

    // Visszaad egy mintatáblát, ha nincs betöltött adat.
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

// Az alapértelmezett tábla a mintaadatokat használja.
impl Default for Board {
    fn default() -> Self {
        Self::with_sample_cards()
    }
}
