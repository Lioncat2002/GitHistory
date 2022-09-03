use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, Dataset, Axis, Chart, GraphType, BarChart},
    layout::{Layout, Constraint, Direction, Rect},
    Terminal, style::{Style, Color, Modifier}, symbols::{Marker, self}, text::Span
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), io::Error> {
    let datasets = vec![
    Dataset::default()
        .name("data1")
        .marker(symbols::Marker::Dot)
        .graph_type(GraphType::Scatter)
        .style(Style::default().fg(Color::White))
        .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)]),
    Dataset::default()
        .name("data2")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Magenta))
        .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
];
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
            let bar_chart=BarChart::default()
            .block(Block::default().title("BarChart").borders(Borders::ALL))
            .bar_width(3)
            .bar_gap(1)
            .bar_style(Style::default().fg(Color::Green))
            .value_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
            .label_style(Style::default().fg(Color::White))
            .data(&[("B0", 0), ("B1", 2), ("B2", 2), ("B3", 3)])
            .max(10);
        f.render_widget(block, size);

        let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(4)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());
        let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);
        f.render_widget(bar_chart.clone(), top_chunks[0]);
        f.render_widget(bar_chart, top_chunks[1]);
       // f.render_widget(block2, size);
    })?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}