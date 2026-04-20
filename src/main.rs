mod app;
mod error;
mod input;
mod model;
mod storage;
mod ui;

use anyhow::Result;

fn main() -> Result<()> {
    let app = app::App::new();

    // Iteration 1: core architecture and domain models are wired in.
    println!(
        "Kanban Lite initialized with {} sample cards.",
        app.board.cards.len()
    );

    Ok(())
}
