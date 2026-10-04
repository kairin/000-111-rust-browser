use std::io::{self, BufRead};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

enum UserAction {
    Navigate(String),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoopBuilder::<()>::new().build();
    let window = WindowBuilder::new()
        .with_title("Rust Desktop Browser")
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 800.0))
        .build(&event_loop)?;

    let (tx, rx): (Sender<UserAction>, Receiver<UserAction>) = channel();

    // Spawn an isolated background thread to accept URL strings from standard input
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        println!("Browser initialized. Enter target URL and press RETURN:");
        loop {
            let mut buffer = String::new();
            if reader.read_line(&mut buffer).is_ok() {
                let trimmed = buffer.trim();
                if !trimmed.is_empty() {
                    let formatted_target = if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
                        format!("https://{}", trimmed)
                    } else {
                        trimmed.to_string()
                    };
                    let _ = tx.send(UserAction::Navigate(formatted_target));
                }
            }
        }
    });

    let default_url = "https://news.ycombinator.com";

    // Initialize the platform-specific WebView abstraction
    #[cfg(not(target_os = "linux"))]
    let webview = WebViewBuilder::new(&window)
        .with_url(default_url)
        .build()?;

    #[cfg(target_os = "linux")]
    let webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let container = window.default_vbox().expect("GTK vbox container unavailable");
        WebViewBuilder::new_gtk(container)
            .with_url(default_url)
            .build()?
    };

    let event_loop_proxy = event_loop.create_proxy();

    // Coordinate navigation commands with the primary event loop
    thread::spawn(move || {
        while let Ok(action) = rx.recv() {
            match action {
                UserAction::Navigate(url) => {
                    // Evaluate navigation script directly within the WebView instance
                    let script = format!("window.location.href = '{}';", url);
                    let _ = webview.evaluate_script(&script);
                    let _ = event_loop_proxy.send_event(());
                }
            }
        }
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
