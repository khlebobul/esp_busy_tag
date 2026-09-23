use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use serde::Deserialize;
use serde_json::{json, Value};
use std::io::{Read, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

#[derive(Clone, Deserialize)]
#[allow(dead_code)]
struct Info {
    device: String,
    version: String,
    board: String,
}

#[derive(Clone, Debug)]
struct Device {
    port: String,
}

#[derive(Clone, Copy, PartialEq)]
enum Status {
    Free,
    Busy,
    Meeting,
}

impl Status {
    fn value(self) -> &'static str {
        match self {
            Self::Free => "free",
            Self::Busy => "busy",
            Self::Meeting => "meeting",
        }
    }
    fn label(self) -> &'static str {
        match self {
            Self::Free => "Free",
            Self::Busy => "Busy",
            Self::Meeting => "Meeting",
        }
    }
    fn key(self) -> &'static str {
        match self {
            Self::Free => "f",
            Self::Busy => "b",
            Self::Meeting => "m",
        }
    }
    fn color(self) -> Color {
        match self {
            Self::Free => Color::Rgb(48, 209, 88),
            Self::Busy => Color::Rgb(255, 69, 58),
            Self::Meeting => Color::Rgb(10, 132, 255),
        }
    }
}

fn open(port: &str) -> Result<Box<dyn serialport::SerialPort>, String> {
    serialport::new(port, 115_200)
        .timeout(Duration::from_millis(800))
        .open()
        .map_err(|e| e.to_string())
}

fn request_info(port_name: &str) -> Option<Device> {
    let mut port = open(port_name).ok()?;
    port.clear(serialport::ClearBuffer::Input).ok()?;
    port.write_all(b"{\"cmd\":\"info\"}\n").ok()?;
    let mut response = Vec::new();
    loop {
        let mut byte = [0];
        if port.read(&mut byte).is_err() {
            break;
        }
        if byte[0] == b'\n' {
            break;
        }
        if response.len() == 511 {
            return None;
        }
        response.push(byte[0]);
    }
    let info: Info = serde_json::from_slice(&response).ok()?;
    (info.device == "desk-display").then_some(Device {
        port: port_name.into(),
    })
}

fn find_device() -> Option<Device> {
    serialport::available_ports()
        .ok()?
        .into_iter()
        .find_map(|port| request_info(&port.port_name))
}

fn send_command(port: &str, command: &Value) -> Result<(), String> {
    let mut serial = open(port)?;
    let line = serde_json::to_vec(command).map_err(|e| e.to_string())?;
    serial
        .write_all(&line)
        .and_then(|_| serial.write_all(b"\n"))
        .map_err(|e| e.to_string())
}

enum Command_ {
    Find,
    SetStatus(Status),
}

enum UiEvent {
    Found(Option<Device>),
    StatusSent(Result<(), String>),
}

fn worker(rx: Receiver<Command_>, out: Sender<UiEvent>) {
    let mut device: Option<Device> = None;
    while let Ok(cmd) = rx.recv() {
        let event = match cmd {
            Command_::Find => {
                let found = find_device();
                device = found.clone();
                UiEvent::Found(found)
            }
            Command_::SetStatus(status) => {
                let res = match &device {
                    Some(d) => send_command(
                        &d.port,
                        &json!({ "cmd": "status", "value": status.value() }),
                    ),
                    None => Err("Device not connected".into()),
                };
                UiEvent::StatusSent(res)
            }
        };
        if out.send(event).is_err() {
            break;
        }
    }
}

struct App {
    device: Option<Device>,
    active: Status,
    pending: Option<Status>,
    notice: String,
    quit: bool,
}

impl App {
    fn send(&self, tx: &Sender<Command_>, command: Command_) {
        let _ = tx.send(command);
    }

    fn set_status(&mut self, tx: &Sender<Command_>, status: Status) {
        if self.device.is_none() {
            self.notice = "Device not connected".into();
            return;
        }
        self.pending = Some(status);
        self.send(tx, Command_::SetStatus(status));
    }
}

fn draw(f: &mut Frame, app: &App) {
    let area = f.area();
    let inner = Rect {
        x: area.x + 2,
        y: area.y,
        width: area.width.saturating_sub(4),
        height: area.height,
    };
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(5),
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(inner);

    // Header
    let connected = app.device.is_some();
    let dot = if connected { "●" } else { "○" };
    let header = Line::from(vec![
        Span::styled("ESP32 Busy Tag", Style::default().bold()),
        Span::raw("   "),
        Span::styled(
            dot,
            Style::default().fg(if connected {
                Color::Rgb(48, 209, 88)
            } else {
                Color::DarkGray
            }),
        ),
        Span::styled(
            if connected {
                format!(" {}", app.device.as_ref().unwrap().port)
            } else {
                " offline".to_string()
            },
            Style::default().fg(Color::DarkGray),
        ),
    ]);
    f.render_widget(Paragraph::new(header), chunks[0]);

    // Big status
    let status = Paragraph::new(Line::from(Span::styled(
        app.active.label().to_uppercase(),
        Style::default().fg(app.active.color()).bold(),
    )))
    .alignment(Alignment::Center);
    f.render_widget(status, chunks[1]);

    // Status options
    let spans: Vec<Span> = {
        let mut spans: Vec<Span> = Vec::new();
        for (i, status) in [Status::Free, Status::Busy, Status::Meeting]
            .into_iter()
            .enumerate()
        {
            if i > 0 {
                spans.push(Span::raw("    "));
            }
            let is_active = status == app.active;
            let style = if is_active {
                Style::default()
                    .fg(status.color())
                    .bold()
                    .add_modifier(Modifier::REVERSED)
            } else {
                Style::default().fg(status.color())
            };
            spans.push(Span::styled(
                format!("[{}] {}", status.key(), status.label()),
                style,
            ));
        }
        spans
    };
    f.render_widget(
        Paragraph::new(Line::from(spans)).alignment(Alignment::Center),
        chunks[2],
    );

    // Notice
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            app.notice.as_str(),
            Style::default().fg(Color::DarkGray),
        )))
        .alignment(Alignment::Center),
        chunks[3],
    );

    // Footer
    let footer = Line::from(vec![
        key_hint("f", "Free"),
        Span::raw("  "),
        key_hint("b", "Busy"),
        Span::raw("  "),
        key_hint("m", "Meeting"),
        Span::raw(if app.device.is_some() { "   ·   " } else { "" }),
        key_hint("r", "Reconnect"),
        Span::raw("   ·   "),
        key_hint("q", "Quit"),
    ]);
    f.render_widget(
        Paragraph::new(footer).alignment(Alignment::Center),
        chunks[4],
    );
}

fn key_hint(key: &str, label: &str) -> Span<'static> {
    Span::styled(
        format!("[{key}] {label}"),
        Style::default().fg(Color::DarkGray),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let (cmd_tx, cmd_rx) = mpsc::channel::<Command_>();
    let (evt_tx, evt_rx) = mpsc::channel::<UiEvent>();

    let worker_handle = std::thread::spawn(move || worker(cmd_rx, evt_tx));

    let mut app = App {
        device: None,
        active: Status::Free,
        pending: None,
        notice: "".into(),
        quit: false,
    };

    app.send(&cmd_tx, Command_::Find);

    while !app.quit {
        terminal.draw(|f| draw(f, &app))?;

        while let Ok(event) = evt_rx.try_recv() {
            match event {
                UiEvent::Found(device) => {
                    app.device = device;
                    app.notice = if app.device.is_some() {
                        "".into()
                    } else {
                        "not found".into()
                    };
                }
                UiEvent::StatusSent(res) => match res {
                    Ok(()) => {
                        if let Some(status) = app.pending.take() {
                            app.active = status;
                        }
                    }
                    Err(e) => {
                        app.pending = None;
                        app.notice = e;
                    }
                },
            }
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_key(&mut app, &cmd_tx, key.code);
                }
            }
        }
    }

    ratatui::restore();
    drop(worker_handle);
    Ok(())
}

fn handle_key(app: &mut App, tx: &Sender<Command_>, code: KeyCode) {
    match code {
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => app.quit = true,
        KeyCode::Char('f') | KeyCode::Char('1') => app.set_status(tx, Status::Free),
        KeyCode::Char('b') | KeyCode::Char('2') => app.set_status(tx, Status::Busy),
        KeyCode::Char('m') | KeyCode::Char('3') => app.set_status(tx, Status::Meeting),
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.send(tx, Command_::Find);
            app.notice = "searching…".into();
        }
        _ => {}
    }
}
