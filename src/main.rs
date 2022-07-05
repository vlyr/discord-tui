use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use std::error::Error;
use std::sync::mpsc;
use std::time::Duration;
use tokio_tungstenite::connect_async;

use tui::{
    backend::CrosstermBackend,
    layout::*,
    terminal::Terminal,
    text::Span,
    widgets::{Block, Borders, Paragraph},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (main_tx, main_rx) = mpsc::channel();

    std::thread::spawn(move || {
        event_handler(main_rx).unwrap();
    });

    let mut stdout = std::io::stdout();

    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let size = terminal.size()?;

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(size);

    loop {
        terminal.draw(|f| {
            let paragraph =
                Paragraph::new(Span::raw("Hey")).block(Block::default().borders(Borders::all()));
            f.render_widget(paragraph, chunks[0]);
        })?;

        if poll(Duration::from_millis(500))? {
            if let Event::Key(ev) = read()? {
                match ev.code {
                    KeyCode::Char('q') => break,
                    _ => (),
                }
            }
        }
    }

    disable_raw_mode()?;

    let mut stdout = std::io::stdout();
    stdout.execute(LeaveAlternateScreen)?;

    Ok(())
}

#[tokio::main]
async fn event_handler(receiver: mpsc::Receiver<String>) -> Result<(), Box<dyn Error>> {
    let stream = connect_async("wss://gateway.discord.gg").await?;

    while let Ok(event) = receiver.recv() {}

    Ok(())
}
