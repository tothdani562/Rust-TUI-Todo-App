use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};

use crate::app::{AddCardStep, App, AppMode};
use crate::model::{Board, Column, Priority};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    frame.render_widget(Clear, frame.area());

    let root = bounded_center_rect(frame.area(), 140, 32);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(4)])
        .split(root);

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

    let footer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(2)])
        .split(chunks[1]);

    let help = Paragraph::new(help_text(app))
        .block(Block::default().borders(Borders::TOP).title("Help"));
    frame.render_widget(help, footer_chunks[0]);

    let status = Paragraph::new(app.status_message.as_str())
        .style(Style::default().fg(Color::LightYellow))
        .block(Block::default().borders(Borders::TOP).title("Status"));
    frame.render_widget(status, footer_chunks[1]);

    if let AppMode::AddCard(draft) = &app.mode {
        render_add_card_modal(frame, draft);
    }
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

fn help_text(app: &App) -> Line<'static> {
    if app.is_creating_card() {
        return Line::from(vec![
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": next/confirm  "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": cancel  "),
            Span::styled("Backspace", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": delete char  "),
            Span::styled("P/Tab", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": cycle priority"),
        ]);
    }

    Line::from(vec![
        Span::styled("Arrows", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": move  "),
        Span::styled("A", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": add  "),
        Span::styled("M", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": move card  "),
        Span::styled("D", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": delete  "),
        Span::styled("Q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": quit"),
    ])
}

fn render_add_card_modal(frame: &mut Frame<'_>, draft: &crate::app::AddCardDraft) {
    let area = centered_rect(70, 45, frame.area());

    frame.render_widget(Clear, area);

    let title_style = if draft.step == AddCardStep::Title {
        Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let description_style = if draft.step == AddCardStep::Description {
        Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let priority_style = if draft.step == AddCardStep::Priority {
        Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let text = Text::from(vec![
        Line::from(Span::styled("Create New Card", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![
            Span::styled("Title: ", title_style),
            Span::raw(draft.title.as_str()),
        ]),
        Line::from(vec![
            Span::styled("Description: ", description_style),
            Span::raw(draft.description.as_str()),
        ]),
        Line::from(vec![
            Span::styled("Priority: ", priority_style),
            Span::raw(match draft.priority {
                Priority::Low => "Low",
                Priority::Medium => "Medium",
                Priority::High => "High",
            }),
        ]),
        Line::from(""),
        Line::from("Enter: next/confirm | Esc: cancel | P/Tab: cycle priority"),
    ]);

    let popup = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Add Card")
            .border_style(Style::default().fg(Color::LightBlue)),
    );

    frame.render_widget(popup, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}

fn bounded_center_rect(area: Rect, max_width: u16, max_height: u16) -> Rect {
    let width = area.width.min(max_width).max(1);
    let height = area.height.min(max_height).max(1);
    let x = area.x + (area.width.saturating_sub(width) / 2);
    let y = area.y + (area.height.saturating_sub(height) / 2);

    Rect {
        x,
        y,
        width,
        height,
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
