use std::fs;
use std::path::Path;

use crate::error::AppError;
use crate::model::Board;

// Betölti a táblát JSON fájlból, és ellenőrzi az adatokat.
pub fn load_board(path: impl AsRef<Path>) -> Result<Board, AppError> {
    let path = path.as_ref();
    let data = fs::read_to_string(path).map_err(|error| AppError::from_io(path, error))?;
    let board = serde_json::from_str(&data)?;
    validate_board(&board)?;
    Ok(board)
}

// Elmenti a táblát JSON formátumban, ha az adatok érvényesek.
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

// Alapvető adatellenőrzést végez, mielőtt mentenénk vagy használnánk a táblát.
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
