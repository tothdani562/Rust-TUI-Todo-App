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

fn main() -> Result<()> {
    run_app()
}

fn run_app() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = app::App::new();

    let run_result = event_loop(&mut terminal, &mut app);
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
) -> Result<()> {
    draw_frame(terminal, app)?;

    while let Ok(event) = event::read() {
        if let Event::Key(key_event) = event {
            let command = input::map_key_to_command(key_event, app.is_creating_card());
            app.apply_command(command);

            if app.should_quit {
                break;
            }

            draw_frame(terminal, app)?;
        }
    }

    Ok(())
}

fn draw_frame(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &app::App,
) -> Result<()> {
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
