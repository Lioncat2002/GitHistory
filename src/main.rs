use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use regex::Regex;
use std::{fs, io, process::Command, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{BarChart, Block, Borders},
    Terminal,
};

#[derive(Debug, Clone)]
struct GitGraph {
    logs: String,
    data:Vec<(String,i32)>
}

impl GitGraph {
    fn new(logs: String) -> Self {
        GitGraph { logs,data:vec![] }
    }

    fn graph_builder(self){
        //let re=Regex::new();
        let log_data:Vec<&str>=self.logs.split("").collect();
        println!("{:?}",log_data);
    }
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let cmd = Command::new("git")
        .args(["log","--date=short","--pretty=format:%ad","--since=15.days"])
        .output()
        .unwrap();

    let contents = String::from_utf8_lossy(&cmd.stdout).to_string();
    println!("{:?}", contents.split(" ").collect::<Vec<&str>>());
    let git_graph = GitGraph::new(contents);
    println!("{:?}", git_graph);
    /*
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Git History").borders(Borders::ALL);
            f.render_widget(block, size);


            let bar_chart = BarChart::default()
                .block(Block::default().title("BarChart").borders(Borders::ALL))
                .bar_width(3)
                .bar_gap(1)
                .bar_style(Style::default().fg(Color::Green))
                .value_style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .label_style(Style::default().fg(Color::White))
                .data(&[("B0", 0), ("B1", 2), ("B2", 2), ("B3", 3)])
                .max(10);


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
    */
    Ok(())
}
