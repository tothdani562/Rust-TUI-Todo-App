use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

use crate::app::App;
use crate::model::{Board, Column, Priority};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(frame.area());

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(chunks[0]);

    for (index, column) in Board::columns().into_iter().enumerate() {
        render_column(
            frame,
            columns[index],
            column,
            &app.board,
            app.board.selected_column == column,
        );
    }

    let help = Paragraph::new(Line::from(vec![
        Span::styled("Arrows", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": move  "),
        Span::styled("Q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": quit"),
    ]))
    .block(Block::default().borders(Borders::TOP));

    frame.render_widget(help, chunks[1]);
}

fn render_column(
    frame: &mut Frame<'_>,
    area: Rect,
    column: Column,
    board: &Board,
    selected_column: bool,
) {
    let cards = board.cards_in_column(column);
    let title = format!("{} ({})", column_label(column), cards.len());
    let border_style = if selected_column {
        Style::default().fg(Color::LightCyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let selected_item_index = if selected_column {
        board
            .selected_card()
            .and_then(|selected_card| cards.iter().position(|card| card.id == selected_card.id))
    } else {
        None
    };

    let content_width = area.width.saturating_sub(4) as usize;

    let items = cards
        .into_iter()
        .map(|card| ListItem::new(card_lines(card, content_width)).style(item_style(card.priority)))
        .collect::<Vec<_>>();

    let mut state = ListState::default();
    if let Some(index) = selected_item_index {
        state.select(Some(index));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style),
        )
        .highlight_style(
            Style::default()
                .bg(Color::LightCyan)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut state);
}

fn column_label(column: Column) -> &'static str {
    match column {
        Column::Todo => "Todo",
        Column::Doing => "Doing",
        Column::Done => "Done",
    }
}

fn priority_label(priority: Priority) -> &'static str {
    match priority {
        Priority::Low => "[Low]",
        Priority::Medium => "[Med]",
        Priority::High => "[High]",
    }
}

fn card_lines(card: &crate::model::Card, content_width: usize) -> Text<'static> {
    let mut lines = Vec::new();

    let title_prefix = format!("{} {}", card.title, priority_label(card.priority));
    let title_lines = wrap_text(&title_prefix, content_width.max(1));

    for (index, line) in title_lines.iter().enumerate() {
        if index == 0 {
            lines.push(Line::from(Span::styled(
                line.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            )));
        } else {
            lines.push(Line::from(Span::styled(
                format!("  {}", line),
                Style::default().add_modifier(Modifier::BOLD),
            )));
        }
    }

    for line in wrap_text(&card.description, content_width.max(1)) {
        lines.push(Line::from(Span::styled(
            format!("  {}", line),
            Style::default().fg(Color::DarkGray),
        )));
    }

    Text::from(lines)
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![String::new()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        let projected_length = if current_line.is_empty() {
            word.len()
        } else {
            current_line.len() + 1 + word.len()
        };

        if projected_length > max_width && !current_line.is_empty() {
            lines.push(current_line);
            current_line = word.to_string();
        } else {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

fn item_style(priority: Priority) -> Style {
    match priority {
        Priority::Low => Style::default().fg(Color::Green),
        Priority::Medium => Style::default().fg(Color::Yellow),
        Priority::High => Style::default().fg(Color::Red),
    }
}
