use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::app::{App, ConnectionStatus};

pub fn render(app: &App, f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Content
            Constraint::Length(3),  // Footer
        ])
        .split(f.area());

    render_header(f, chunks[0], app);
    render_positions(f, chunks[1], app);
    render_footer(f, chunks[2]);
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let connection_status = app.connection_status();
    let (status_icon, status_color) = match connection_status {
        ConnectionStatus::Connected => ("●", Color::Green),
        ConnectionStatus::Disconnected => ("●", Color::Red),
    };

    let status_text = match connection_status {
        ConnectionStatus::Connected => "Connected",
        ConnectionStatus::Disconnected => "Disconnected",
    };

    let last_update = app
        .last_update()
        .map(|u| format!(" ({})", u))
        .unwrap_or_default();

    let title = if let Some(data) = &app.data {
        Line::from(vec![
            Span::styled(status_icon, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled(status_text, Style::default().fg(status_color)),
            Span::styled(last_update, Style::default().fg(Color::DarkGray)),
            Span::raw(" | "),
            Span::styled(
                format!("Balance: ${:.2}", data.account.balance),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(" | "),
            Span::styled(
                format!("Equity: ${:.2}", data.account.equity),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(" | "),
            Span::styled(
                format!("Profit: ${:+.2}", data.account.profit),
                Style::default().fg(if data.account.profit >= 0.0 {
                    Color::Green
                } else {
                    Color::Red
                }),
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled(status_icon, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled(status_text, Style::default().fg(status_color)),
            Span::raw(" | "),
            Span::styled("No Data", Style::default().fg(Color::Yellow)),
        ])
    };

    let header = Paragraph::new(title)
        .block(Block::default().borders(Borders::ALL).title("MT5 Terminal"))
        .style(Style::default());
    f.render_widget(header, area);
}

fn render_positions(f: &mut Frame, area: Rect, app: &App) {
    if let Some(data) = &app.data {
        let header_cells = ["Ticket", "Symbol", "Type", "Volume", "Open", "Current", "Profit"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
        let header = Row::new(header_cells).height(1).bottom_margin(1);

        let rows = data.positions.iter().map(|pos| {
            let profit_color = if pos.is_profitable() {
                Color::Green
            } else {
                Color::Red
            };

            let cells = vec![
                Cell::from(pos.ticket.to_string()),
                Cell::from(pos.symbol.clone()),
                Cell::from(pos.position_type.clone()),
                Cell::from(format!("{:.2}", pos.volume)),
                Cell::from(format!("{:.5}", pos.open_price)),
                Cell::from(format!("{:.5}", pos.current_price)),
                Cell::from(format!("{:.2}", pos.profit)).style(Style::default().fg(profit_color)),
            ];
            Row::new(cells).height(1)
        });

        let widths = [
            Constraint::Length(12),
            Constraint::Length(8),
            Constraint::Length(6),
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Open Positions ({})", data.positions.len())),
            )
            .column_spacing(1);

        f.render_widget(table, area);
    } else {
        let content = Paragraph::new("Loading data from data/positions.json...\n\nIf you see this message:\n1. Ensure MT5 is running\n2. Start the Python service: python python/mt5_service.py\n3. Or create mock data in data/positions.json")
            .block(Block::default().borders(Borders::ALL).title("Positions"))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(content, area);
    }
}

fn render_footer(f: &mut Frame, area: Rect) {
    let help = Line::from(vec![
        Span::styled("q", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("/"),
        Span::styled("ESC", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(": Quit | "),
        Span::styled("Tab", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(": Switch view (coming soon) | "),
        Span::styled("r", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(": Reload data"),
    ]);

    let footer = Paragraph::new(help)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, area);
}
