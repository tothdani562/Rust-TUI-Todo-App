use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};

use crate::app::{AddCardStep, App, AppMode, EditCardStep};
use crate::model::{Board, Column, Priority};

// Egy értékhez rövid feliratszöveget párosít a listás és címke-alapú megjelenítéshez.
macro_rules! enum_label {
    ($value:expr, { $( $pattern:pat => $label:expr ),+ $(,)? }) => {{
        match $value {
            $( $pattern => $label, )+
        }
    }};
}

// Kiemeli az éppen aktív űrlapmezőt.
macro_rules! active_field_style {
    ($current_step:expr, $active_step:pat) => {{
        if matches!($current_step, $active_step) {
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        }
    }};
}

// Összeállítja és kirajzolja a teljes TUI képernyőt.
pub fn render(frame: &mut Frame<'_>, app: &App) {
    frame.render_widget(Clear, frame.area());

    let root = bounded_center_rect(frame.area(), 140, 32);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(6),
            Constraint::Min(0),
            Constraint::Length(4),
        ])
        .split(root);

    let title = Paragraph::new(Span::styled(
        "TODO",
        Style::default()
            .fg(Color::LightCyan)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);
    frame.render_widget(title, chunks[0]);

    let header = Paragraph::new(stats_header_text(&app.board, chunks[1].width))
        .block(Block::default().borders(Borders::ALL).title("Board Stats"));
    frame.render_widget(header, chunks[1]);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(chunks[2]);

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
        .split(chunks[3]);

    let help =
        Paragraph::new(help_text(app)).block(Block::default().borders(Borders::TOP).title("Help"));
    frame.render_widget(help, footer_chunks[0]);

    let status_line = format!(
        "Mode: {} | Last action: {}",
        app.current_mode_label(),
        app.status_message
    );
    let status = Paragraph::new(status_line)
        .style(Style::default().fg(Color::LightYellow))
        .block(Block::default().borders(Borders::TOP).title("Status"));
    frame.render_widget(status, footer_chunks[1]);

    if let AppMode::AddCard(draft) = &app.mode {
        render_add_card_modal(frame, draft);
    }

    if let AppMode::EditCard(draft) = &app.mode {
        render_edit_card_modal(frame, draft);
    }

    if let AppMode::ViewCard(card) = &app.mode {
        render_view_card_modal(frame, card);
    }

    if app.show_help {
        render_help_panel(frame, app);
    }
}

// A tábla tetején látható statisztikai összegzést készíti el.
fn stats_header_text(board: &Board, width: u16) -> Text<'static> {
    let todo_count = board.cards_in_column(Column::Todo).len();
    let doing_count = board.cards_in_column(Column::Doing).len();
    let done_count = board.cards_in_column(Column::Done).len();
    let total = board.cards.len();

    let counts_line = Line::from(vec![
        Span::styled(
            format!("Todo: {}", todo_count),
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("   "),
        Span::styled(
            format!("Doing: {}", doing_count),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("   "),
        Span::styled(
            format!("Done: {}", done_count),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("   "),
        Span::styled(
            format!("Total: {}", total),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let bar_width = width.saturating_sub(20).max(10) as usize;

    let todo_line = progress_row("Todo  ", todo_count, total, bar_width, Color::LightBlue);

    let doing_line = progress_row("Doing ", doing_count, total, bar_width, Color::Yellow);

    let done_line = progress_row("Done  ", done_count, total, bar_width, Color::Green);

    Text::from(vec![counts_line, todo_line, doing_line, done_line])
}

// Egyetlen oszlop kártyalistáját és keretét rajzolja ki.
fn render_column(
    frame: &mut Frame<'_>,
    area: Rect,
    column: Column,
    board: &Board,
    selected_column: bool,
) {
    let cards = board.cards_in_column(column);
    let title = format!(
        "{} ({})",
        enum_label!(column, {
            Column::Todo => "Todo",
            Column::Doing => "Doing",
            Column::Done => "Done"
        }),
        cards.len()
    );
    let border_style = if selected_column {
        Style::default().fg(selected_column_color(column))
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

// A footerhez rövid billentyűs segédszöveget ad vissza az aktuális mód alapján.
fn help_text(app: &App) -> Line<'static> {
    if matches!(app.mode, AppMode::ViewCard(_)) {
        return Line::from(vec![
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": close details  "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": close details  "),
            Span::styled("H", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": help"),
        ]);
    }

    if app.is_input_mode() {
        return Line::from(vec![
            Span::styled("H", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": help  "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": next/confirm  "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": cancel  "),
            Span::styled("Backspace", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": delete char  "),
            Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(": cycle priority"),
        ]);
    }

    Line::from(vec![
        Span::styled("Arrows", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": move  "),
        Span::styled("A", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": add  "),
        Span::styled("E", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": edit  "),
        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": details  "),
        Span::styled("P", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": priority  "),
        Span::styled("M", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": move card  "),
        Span::styled("D", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": delete  "),
        Span::styled("H", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": help  "),
        Span::styled("Q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": quit"),
    ])
}

// Középre nyíló súgóablakot rajzol, amely a teljes parancslistát mutatja.
fn render_help_panel(frame: &mut Frame<'_>, app: &App) {
    let area = centered_rect(72, 62, frame.area());
    frame.render_widget(Clear, area);

    let mut lines = vec![
        Line::from(Span::styled(
            "Keyboard Shortcuts",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    if matches!(app.mode, AppMode::ViewCard(_)) {
        lines.extend_from_slice(&[
            Line::from("Enter: close card details"),
            Line::from("Esc: close card details"),
        ]);
    } else if app.is_input_mode() {
        lines.extend_from_slice(&[
            Line::from("Enter: next step / confirm"),
            Line::from("Esc: cancel input mode"),
            Line::from("Backspace: delete character"),
            Line::from("Tab: cycle priority"),
            Line::from("Typing: write field text"),
        ]);
    } else {
        lines.extend_from_slice(&[
            Line::from("Arrow keys: move selection"),
            Line::from("A: add card"),
            Line::from("E: edit selected card"),
            Line::from("Enter: view selected card details"),
            Line::from("M: move selected card forward"),
            Line::from("D: delete selected card"),
            Line::from("P: cycle selected card priority"),
            Line::from("Q: quit application"),
        ]);
    }

    lines.extend_from_slice(&[Line::from(""), Line::from("H: close this help panel")]);

    let popup = Paragraph::new(Text::from(lines)).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Help")
            .border_style(Style::default().fg(Color::LightBlue)),
    );

    frame.render_widget(popup, area);
}

// Az új kártya létrehozásának modális űrlapját rajzolja ki.
fn render_add_card_modal(frame: &mut Frame<'_>, draft: &crate::app::AddCardDraft) {
    let area = centered_rect(70, 45, frame.area());

    frame.render_widget(Clear, area);

    let title_style = active_field_style!(draft.step, AddCardStep::Title);
    let description_style = active_field_style!(draft.step, AddCardStep::Description);
    let priority_style = active_field_style!(draft.step, AddCardStep::Priority);

    let text = Text::from(vec![
        Line::from(Span::styled(
            "Create New Card",
            Style::default().add_modifier(Modifier::BOLD),
        )),
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
            Span::raw(enum_label!(draft.priority, {
                Priority::Low => "Low",
                Priority::Medium => "Medium",
                Priority::High => "High"
            })),
        ]),
        Line::from(""),
        Line::from("Enter: next/confirm | Esc: cancel | Tab: cycle priority"),
    ]);

    let popup = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Add Card")
            .border_style(Style::default().fg(Color::LightBlue)),
    );

    frame.render_widget(popup, area);
}

// A meglévő kártya szerkesztésének modális űrlapját rajzolja ki.
fn render_edit_card_modal(frame: &mut Frame<'_>, draft: &crate::app::EditCardDraft) {
    let area = centered_rect(70, 45, frame.area());

    frame.render_widget(Clear, area);

    let title_style = active_field_style!(draft.step, EditCardStep::Title);
    let description_style = active_field_style!(draft.step, EditCardStep::Description);
    let priority_style = active_field_style!(draft.step, EditCardStep::Priority);

    let text = Text::from(vec![
        Line::from(Span::styled(
            "Edit Card",
            Style::default().add_modifier(Modifier::BOLD),
        )),
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
            Span::raw(enum_label!(draft.priority, {
                Priority::Low => "Low",
                Priority::Medium => "Medium",
                Priority::High => "High"
            })),
        ]),
        Line::from(""),
        Line::from("Enter: next/confirm | Esc: cancel | Tab: cycle priority"),
    ]);

    let popup = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Edit Card")
            .border_style(Style::default().fg(Color::LightBlue)),
    );

    frame.render_widget(popup, area);
}

// A kijelölt kártya részleteit megjelenítő modális ablakot rajzolja ki.
fn render_view_card_modal(frame: &mut Frame<'_>, card: &crate::app::CardPreview) {
    let area = centered_rect(72, 68, frame.area());
    frame.render_widget(Clear, area);

    let text = Text::from(vec![
        Line::from(Span::styled(
            "Card Details",
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("ID: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(card.id.to_string()),
            Span::raw("    "),
            Span::styled("Column: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                enum_label!(card.column, {
                    Column::Todo => "Todo",
                    Column::Doing => "Doing",
                    Column::Done => "Done"
                }),
                Style::default().fg(card_column_color(card.column)),
            ),
            Span::raw("    "),
            Span::styled("Priority: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(
                enum_label!(card.priority, {
                    Priority::Low => "Low",
                    Priority::Medium => "Medium",
                    Priority::High => "High"
                }),
                item_style(card.priority).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Title",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(card.title.clone()),
        Line::from(""),
        Line::from(Span::styled(
            "Description",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(card.description.clone()),
        Line::from(""),
        Line::from(Span::styled(
            "Enter or Esc: close",
            Style::default().fg(Color::DarkGray),
        )),
    ]);

    let popup = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Selected Card")
                .border_style(Style::default().fg(Color::LightBlue)),
        )
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(popup, area);
}

// A képernyő közepére helyezett téglalapot számítja ki százalékos méret alapján.
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

// A képernyő közepére helyezett, de maximális mérethez kötött téglalapot számít.
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

// Egy kártya rövid, több soros listanézetét állítja elő.
fn card_lines(card: &crate::model::Card, content_width: usize) -> Text<'static> {
    let mut lines = Vec::new();
    let title_preview = truncate_with_ellipsis(&card.title, 100);
    let description_preview = truncate_with_ellipsis(&card.description, 100);

    let tag = enum_label!(card.priority, {
        Priority::Low => "[Low]",
        Priority::Medium => "[Med]",
        Priority::High => "[High]"
    });
    let tag_style = priority_tag_style(card.priority);
    let first_line_max_title = content_width
        .saturating_sub(tag.len())
        .saturating_sub(1)
        .max(1);
    let title_lines = wrap_text(&title_preview, first_line_max_title);

    if let Some(first_line) = title_lines.first() {
        lines.push(Line::from(vec![
            Span::styled(
                first_line.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(tag.to_string(), tag_style),
        ]));
    }

    for line in title_lines.iter().skip(1) {
        lines.push(Line::from(Span::styled(
            format!("  {}", line),
            Style::default().add_modifier(Modifier::BOLD),
        )));
    }

    for line in wrap_text(&description_preview, content_width.max(1)) {
        lines.push(Line::from(Span::styled(
            format!("  {}", line),
            Style::default().fg(Color::DarkGray),
        )));
    }

    Text::from(lines)
}

// Egyszerű szóhatár alapú sortörést végez adott maximális szélesség mellett.
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

// A kártya prioritásához színes szövegstílust ad.
fn item_style(priority: Priority) -> Style {
    match priority {
        Priority::Low => Style::default().fg(Color::Green),
        Priority::Medium => Style::default().fg(Color::Yellow),
        Priority::High => Style::default().fg(Color::Red),
    }
}

// A kiválasztott oszlop keretének színét adja vissza.
fn selected_column_color(column: Column) -> Color {
    match column {
        Column::Todo => Color::LightBlue,
        Column::Doing => Color::Yellow,
        Column::Done => Color::Green,
    }
}

// A kártya részletező nézetében használt oszlop-színt adja vissza.
fn card_column_color(column: Column) -> Color {
    match column {
        Column::Todo => Color::LightBlue,
        Column::Doing => Color::Yellow,
        Column::Done => Color::Green,
    }
}

// Levágja a szöveget, ha túl hosszú, és három ponttal jelzi a rövidítést.
fn truncate_with_ellipsis(text: &str, max_chars: usize) -> String {
    let char_count = text.chars().count();
    if char_count <= max_chars {
        return text.to_string();
    }

    let mut result = text.chars().take(max_chars).collect::<String>();
    result.push_str("...");
    result
}

// Egyetlen progress sorból álló statisztikai sávot épít fel.
fn progress_row(
    label: &'static str,
    value: usize,
    total: usize,
    bar_width: usize,
    color: Color,
) -> Line<'static> {
    let mut spans = vec![
        Span::styled(label, Style::default().fg(color)),
        Span::raw(" "),
    ];
    spans.extend(single_progress_bar_spans(value, total, bar_width, color));
    spans.push(Span::raw(" "));
    spans.push(Span::styled(
        format!("{}%", percent(value, total)),
        Style::default().fg(color),
    ));

    Line::from(spans)
}

// Kiszámolja a százalékos arányt nullával védve.
fn percent(value: usize, total: usize) -> usize {
    if total == 0 {
        return 0;
    }

    value.saturating_mul(100) / total
}

// ASCII jellegű töltött és üres blokkokból progress bar-t hoz létre.
fn single_progress_bar_spans(
    value: usize,
    total: usize,
    width: usize,
    fill_color: Color,
) -> Vec<Span<'static>> {
    if width == 0 {
        return vec![Span::raw(String::new())];
    }

    if total == 0 || value == 0 {
        return vec![
            Span::raw("["),
            Span::styled("-".repeat(width), Style::default().fg(Color::DarkGray)),
            Span::raw("]"),
        ];
    }

    let mut fill_len = value.saturating_mul(width) / total;
    if fill_len == 0 {
        fill_len = 1;
    }
    let empty_len = width.saturating_sub(fill_len);

    vec![
        Span::raw("["),
        Span::styled("#".repeat(fill_len), Style::default().fg(fill_color)),
        Span::styled("-".repeat(empty_len), Style::default().fg(Color::DarkGray)),
        Span::raw("]"),
    ]
}

// A prioritáshoz tartozó címkeszínt adja vissza.
fn priority_tag_style(priority: Priority) -> Style {
    match priority {
        Priority::Low => Style::default()
            .fg(Color::Black)
            .bg(Color::Green)
            .add_modifier(Modifier::BOLD),
        Priority::Medium => Style::default()
            .fg(Color::Black)
            .bg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
        Priority::High => Style::default()
            .fg(Color::White)
            .bg(Color::Red)
            .add_modifier(Modifier::BOLD),
    }
}
