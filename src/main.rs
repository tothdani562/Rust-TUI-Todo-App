mod app;
mod error;
mod input;
mod model;
mod storage;
mod ui;

use std::io;

use anyhow::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::path::Path;

const BOARD_PATH: &str = "data/board.json";

fn main() -> Result<()> {
    run_app()
}

fn run_app() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = initialize_app(Path::new(BOARD_PATH));

    let run_result = event_loop(&mut terminal, &mut app, Path::new(BOARD_PATH));
    let cleanup_result = restore_terminal(&mut terminal);

    match (run_result, cleanup_result) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(error), Ok(())) => Err(error),
        (Ok(()), Err(error)) => Err(error),
        (Err(error), Err(_cleanup_error)) => Err(error),
    }
}

fn event_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut app::App,
    board_path: &Path,
) -> Result<()> {
    draw_frame(terminal, app)?;

    while let Ok(event) = event::read() {
        if let Event::Key(key_event) = event {
            let command = input::map_key_to_command(key_event, app.is_input_mode());
            let board_changed = app.apply_command(command);

            if board_changed && let Err(error) = storage::save_board(board_path, &app.board) {
                app.status_message = format!("Save failed: {}", error.user_message());
            }

            if app.should_quit {
                break;
            }

            draw_frame(terminal, app)?;
        }
    }

    Ok(())
}

fn initialize_app(board_path: &Path) -> app::App {
    match storage::load_board(board_path) {
        Ok(board) => app::App::from_board_with_status(board, "Board loaded from disk"),
        Err(error) => {
            let status_message = error.user_message();
            app::App::from_board_with_status(model::Board::default(), status_message)
        }
    }
}

fn draw_frame(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &app::App) -> Result<()> {
    terminal.draw(|frame| ui::render(frame, app))?;
    Ok(())
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
