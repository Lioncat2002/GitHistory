use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{collections::HashMap, io, process::Command, thread, time::Duration};
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
    data: Vec<(String, u64)>,
}

impl GitGraph {
    fn new(logs: String) -> Self {
        GitGraph { logs, data: vec![] }
    }

    fn graph_builder(){
        
    }

    fn data_builder(&mut self) {
        //let re=Regex::new();
        let log_data: Vec<&str> = self.logs.trim().split("\n").collect();
        let mut hashmap_data: HashMap<&str, i32> = HashMap::new();
        for log in log_data {
            *hashmap_data.entry(log).or_insert(0) += 1;
        }

        for (key, val) in hashmap_data {
            self.data.push((key.to_string(), val as u64))
        }
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
        .args([
            "log",
            "--date=short",
            "--pretty=format:%ad",
            "--since=30.days",
        ])
        .output()
        .unwrap();

    let contents = String::from_utf8_lossy(&cmd.stdout).to_string();
    let mut git_graph = GitGraph::new(contents);
    git_graph.data_builder();

    let mut data: [(&str, u64); 15] = [("", 0); 15];
    let mut count = 0;
    for d in &git_graph.data {
        let key = &*d.0;
        data[count] = (key, d.1);
        count += 1;
    }

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Git History").borders(Borders::ALL);
        f.render_widget(block, size);

        let bar_chart = BarChart::default()
            .block(Block::default().title("BarChart").borders(Borders::ALL))
            .bar_width(10)
            .bar_gap(1)
            .bar_style(Style::default().fg(Color::Green))
            .value_style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .label_style(Style::default().fg(Color::White))
            .data(&data)
            .max(15);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(4)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());

        /* 
            let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);
*/
        f.render_widget(bar_chart.clone(), chunks[0]);
        
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
