use std::fs;
use std::path::Path;

use crate::error::AppError;
use crate::model::Board;

pub fn load_board(path: impl AsRef<Path>) -> Result<Board, AppError> {
    let path = path.as_ref();
    let data = fs::read_to_string(path).map_err(|error| AppError::from_io(path, error))?;
    let board = serde_json::from_str(&data)?;
    validate_board(&board)?;
    Ok(board)
}

pub fn save_board(path: impl AsRef<Path>, board: &Board) -> Result<(), AppError> {
    validate_board(board)?;

    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| AppError::from_io(parent, error))?;
    }

    let json = serde_json::to_string_pretty(board)?;
    fs::write(path, json).map_err(|error| AppError::from_io(path, error))?;
    Ok(())
}

fn validate_board(board: &Board) -> Result<(), AppError> {
    for card in &board.cards {
        if card.title.trim().is_empty() {
            return Err(AppError::Validation(format!(
                "card with id {} has empty title",
                card.id
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Priority;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_test_path(file_name: &str) -> std::path::PathBuf {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_millis();

        std::env::temp_dir().join(format!("app_tui_storage_{millis}_{file_name}"))
    }

    #[test]
    fn saves_and_loads_board() {
        let path = unique_test_path("board.json");
        let board = Board::with_sample_cards();

        save_board(&path, &board).expect("save should succeed");
        let loaded = load_board(&path).expect("load should succeed");

        assert_eq!(loaded.cards.len(), board.cards.len());
        assert_eq!(loaded.selected_column, board.selected_column);
    }

    #[test]
    fn returns_json_error_for_corrupt_file() {
        let path = unique_test_path("corrupt.json");
        fs::write(&path, "{ not json }").expect("test setup should write file");

        let result = load_board(&path);

        assert!(matches!(result, Err(AppError::Json(_))));
    }

    #[test]
    fn rejects_empty_card_title() {
        let path = unique_test_path("invalid.json");
        let mut board = Board::with_sample_cards();
        board.cards[0].title = "   ".to_string();
        board.cards[0].priority = Priority::High;

        let result = save_board(&path, &board);

        assert!(matches!(result, Err(AppError::Validation(_))));
    }
}
