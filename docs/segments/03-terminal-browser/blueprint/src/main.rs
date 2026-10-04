use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;
use std::time::Duration;
use tokio::sync::mpsc;

enum NetworkMessage {
    PageLoaded(String),
    LoadFailed(String),
}

struct ApplicationState {
    url_input: String,
    status_line: String,
    content_lines: Vec<String>,
    vertical_scroll: usize,
    viewport_width: usize,
    is_loading: bool,
}

impl ApplicationState {
    fn new() -> Self {
        Self {
            url_input: String::from("https://news.ycombinator.com"),
            status_line: String::from("Press ENTER to navigate, UP/DOWN to scroll, ESC to quit"),
            content_lines: Vec::new(),
            vertical_scroll: 0,
            viewport_width: 80,
            is_loading: false,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Configure panic hook to ensure the terminal state is restored on abnormal termination
    let original_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_panic_hook(panic_info);
    }));

    let (net_tx, mut net_rx) = mpsc::channel::<NetworkMessage>(10);
    let mut app = ApplicationState::new();

    // Trigger initial page load
    let initial_tx = net_tx.clone();
    let initial_url = app.url_input.clone();
    app.is_loading = true;
    tokio::spawn(async move {
        fetch_and_parse(&initial_url, app.viewport_width, initial_tx).await;
    });

    loop {
        // Drain incoming network messages
        while let Ok(msg) = net_rx.try_recv() {
            app.is_loading = false;
            match msg {
                NetworkMessage::PageLoaded(body) => {
                    app.content_lines = body.lines().map(|s| s.to_string()).collect();
                    app.vertical_scroll = 0;
                    app.status_line = format!("Loaded {} lines successfully.", app.content_lines.len());
                }
                NetworkMessage::LoadFailed(err) => {
                    app.status_line = format!("Network failure: {}", err);
                }
            }
        }

        terminal.draw(|frame| {
            let area = frame.area();
            app.viewport_width = area.width.saturating_sub(2) as usize;

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(5),
                    Constraint::Length(1),
                ])
                .split(area);

            // Render navigation address bar
            let address_block = Block::default().borders(Borders::ALL).title("URL Location");
            let address_paragraph = Paragraph::new(app.url_input.as_str()).block(address_block);
            frame.render_widget(address_paragraph, chunks[0]);

            // Render document content pane
            let content_block = Block::default().borders(Borders::ALL).title("Rendered Document");
            let visible_slice = app
                .content_lines
                .iter()
                .skip(app.vertical_scroll)
                .take(chunks[1].height.saturating_sub(2) as usize)
                .cloned()
                .collect::<Vec<String>>()
                .join("\n");

            let content_display = if app.is_loading {
                "Request in progress...".to_string()
            } else {
                visible_slice
            };

            let content_paragraph = Paragraph::new(content_display).block(content_block);
            frame.render_widget(content_paragraph, chunks[1]);

            // Render status bar
            let status_style = Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD);
            let status_widget = Paragraph::new(app.status_line.as_str()).style(status_style);
            frame.render_widget(status_widget, chunks[2]);
        })?;

        // Process terminal keyboard events
        if event::poll(Duration::from_millis(50))? {
            if let CEvent::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Down => {
                            if app.vertical_scroll < app.content_lines.len().saturating_sub(1) {
                                app.vertical_scroll += 1;
                            }
                        }
                        KeyCode::Up => {
                            app.vertical_scroll = app.vertical_scroll.saturating_sub(1);
                        }
                        KeyCode::Char(c) => {
                            app.url_input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.url_input.pop();
                        }
                        KeyCode::Enter => {
                            if !app.is_loading {
                                app.is_loading = true;
                                app.status_line = format!("Connecting to {}...", app.url_input);
                                let tx_clone = net_tx.clone();
                                let target_url = app.url_input.clone();
                                let target_width = app.viewport_width;
                                tokio::spawn(async move {
                                    fetch_and_parse(&target_url, target_width, tx_clone).await;
                                });
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

async fn fetch_and_parse(target: &str, width: usize, sender: mpsc::Sender<NetworkMessage>) {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build();

    let client = match client {
        Ok(c) => c,
        Err(e) => {
            let _ = sender.send(NetworkMessage::LoadFailed(e.to_string())).await;
            return;
        }
    };

    match client.get(target).send().await {
        Ok(response) => match response.text().await {
            Ok(html_text) => {
                let parsed_text = html2text::from_read(html_text.as_bytes(), width.max(20));
                let _ = sender.send(NetworkMessage::PageLoaded(parsed_text)).await;
            }
            Err(e) => {
                let _ = sender.send(NetworkMessage::LoadFailed(e.to_string())).await;
            }
        },
        Err(e) => {
            let _ = sender.send(NetworkMessage::LoadFailed(e.to_string())).await;
        }
    }
}
