use crate::process::{access_proc, get_all_process_info};
use std::{thread, time::Duration};
use termion::{input::TermRead, raw::IntoRawMode, screen::AlternateScreen};
use termion::input::MouseTerminal;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Row, Table},
    Terminal,
};
use tui::style::{Color, Style};
use std::io;
use tui::layout::Alignment;
use tui::widgets::Paragraph;

mod process;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let update_interval = Duration::from_millis(500);
    let stdin = termion::async_stdin();
    let mut keys = stdin.keys();

    loop {
        // Fetch updated process information
        access_proc();
        let processes = get_all_process_info();

        // Construct rows with the updated process data
        let rows: Vec<Row> = processes
            .iter()
            .map(|process| {
                Row::new(vec![
                    process.pid().to_string(),
                    process.user().to_string(),
                    process.nice_value().to_string(),
                    process.vm().to_string(),
                    process.res().to_string(),
                    process.shr().to_string(),
                    process.cpu_usage().to_string(),
                    process.mem_usage().to_string(),
                    process.io_usage().clone(),
                    process.time().to_string(),
                    process.command().to_string(),
                ])
            })
            .collect();

        // Draw the table with the updated rows
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            let table = Table::new(rows)
                .block(Block::default().title("Inspector").borders(Borders::ALL))
                .header(Row::new(vec![
                    "PID", "User", "NI", "Virt", "RES", "SHR", "CPU Usage", "Mem Usage", "IO", "Time", "Command",
                ]).style(Style::default().fg(Color::Yellow))) // Optional header style
                .widths(&[
                    Constraint::Percentage(5), // PID
                    Constraint::Percentage(8), // User
                    Constraint::Percentage(5), // NI
                    Constraint::Percentage(8), // Virt
                    Constraint::Percentage(8), // RES
                    Constraint::Percentage(8), // SHR
                    Constraint::Percentage(8), // CPU Usage
                    Constraint::Percentage(8), // Mem Usage
                    Constraint::Percentage(10), // IO - increased space
                    Constraint::Percentage(8),  // Time
                    Constraint::Percentage(14), // Command - reduced space
                ]);


            // Define a paragraph for the footnote
            let footnote = Paragraph::new("Press 'q' to exit")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center);

            // Modify your layout to include space for the footnote
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3), // Adjust the length to fit your header if needed
                    Constraint::Min(0),    // Table will use the rest of the space
                    Constraint::Length(1), // Space for the footnote
                ])
                .split(f.size());

            // Render
            f.render_widget(table, chunks[1]);
            f.render_widget(footnote, chunks[2]);
        })?;

        // Handle input for quitting
        if let Some(Ok(key)) = keys.next() {
            if let termion::event::Key::Char('q') = key {
                break Ok(());
            }
        }

        // Pause the loop for the specified update interval
        thread::sleep(update_interval);
    }

}
