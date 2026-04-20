use std::fs;
use std::path::Path;

use crate::error::AppError;
use crate::model::Board;

pub fn load_board(path: impl AsRef<Path>) -> Result<Board, AppError> {
    let data = fs::read_to_string(path)?;
    let board = serde_json::from_str(&data)?;
    Ok(board)
}

pub fn save_board(path: impl AsRef<Path>, board: &Board) -> Result<(), AppError> {
    let json = serde_json::to_string_pretty(board)?;
    fs::write(path, json)?;
    Ok(())
}
