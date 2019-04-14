use std::io;
use termion::event::Event;
use termion::event::Key;
use termion::event::MouseEvent;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::Terminal;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use termion::input::TermRead;

pub enum MyEvent {
    Input(Key),
    MouseEvent(MouseEvent),
    Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<MyEvent>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: Key::Char('q'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.events() {
                    let evt = evt.unwrap();
                    match evt {
                        Event::Key(k) => {
                            tx.send(MyEvent::Input(k));
                        }
                        Event::Mouse(me) => {
                            tx.send(MyEvent::MouseEvent(me));
                        }
                        _ => {}
                    }
                }
            })
        };
        let tick_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let tx = tx.clone();
                loop {
                    tx.send(MyEvent::Tick).unwrap();
                    thread::sleep(config.tick_rate);
                }
            })
        };
        Events {
            rx,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<MyEvent, mpsc::RecvError> {
        self.rx.recv()
    }
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// History of recorded messages
    messages: Vec<String>,
    formatted: String,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            formatted: String::new(),
            messages: Vec::new(),
        }
    }
}

fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();
    let mut app = App::default();

    loop {
        // Draw UI
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(4),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            Paragraph::new([Text::raw(&app.input)].iter())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("SQL"))
                .render(&mut f, chunks[0]);
            Paragraph::new([Text::raw(&app.formatted)].iter())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL))
                .render(&mut f, chunks[1]);
            let messages = app
                .messages
                .iter()
                .enumerate()
                .map(|(i, m)| Text::raw(format!("{}: {}", i, m)));
            List::new(messages)
                .block(Block::default().borders(Borders::ALL).title("Data"))
                .render(&mut f, chunks[2]);
        })?;

        // Put the cursor back inside the input box
        /*
        write!(
            terminal.backend_mut(),
            "{}",
            Goto(4 + app.input.width() as u16, 4)
        )?;
        */

        // Handle input
        match events.next()? {
            MyEvent::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Char('\n') => {
                    app.formatted = app.input.to_uppercase();
                    app.messages.push(app.input.drain(..).collect());
                }
                Key::Char(c) => {
                    app.input.push(c);
                }
                Key::Backspace => {
                    app.input.pop();
                }
                input => {
                    app.messages.push(format!("event: {:?}", input));
                }
            },
            MyEvent::MouseEvent(me) => {
                app.messages.push(format!("Event: {:#?}", me));
                if app.messages.len() > 20 {
                    app.messages = vec![];
                }
            }
            _ => {}
        }
    }
    Ok(())
}
