use thiserror::Error;

use std::path::Path;

// Az alkalmazás saját, felhasználóbarát hibatípusai.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("I/O error while accessing {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("Invalid JSON format: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Validation error: {0}")]
    Validation(String),
}

impl AppError {
    // Az operációs rendszer hibaüzenetét az app saját hibájává alakítja.
    pub fn from_io(path: &Path, error: std::io::Error) -> Self {
        let path_text = path.display().to_string();

        match error.kind() {
            std::io::ErrorKind::NotFound => Self::FileNotFound(path_text),
            std::io::ErrorKind::PermissionDenied => Self::PermissionDenied(path_text),
            _ => Self::Io {
                path: path_text,
                source: error,
            },
        }
    }

    // Rövid, felhasználónak szánt hibaüzenetet készít.
    pub fn user_message(&self) -> String {
        match self {
            Self::FileNotFound(path) => {
                format!("No saved board found at {path}. Starting with default board.")
            }
            Self::PermissionDenied(path) => {
                format!("Permission denied for {path}. Check file access rights.")
            }
            Self::Json(_) => {
                "Saved board is corrupted (invalid JSON). Starting with default board.".to_string()
            }
            Self::Validation(message) => format!("Saved data is invalid: {message}"),
            Self::Io { path, .. } => format!("Storage I/O error at {path}"),
        }
    }
}
