//#![deny(warnings)]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "with-tui")] {
        use sauron_ui::backend::text_ui::widget_node_tree_to_tui_widget;
        use std::io;
        use std::io::Stdout;
        use termion::event::Event as TermEvent;
        use termion::event::Key as TermKey;
        use termion::event::MouseButton as TermMouseButton;
        use termion::event::MouseEvent as TermMouseEvent;
        use termion::input::MouseTerminal;
        use termion::raw::IntoRawMode;
        use termion::raw::RawTerminal;
        use termion::screen::AlternateScreen;
        use tui::backend::TermionBackend;
        use tui::layout::{Constraint, Direction, Layout};
        use tui::style::{Color, Style};
        use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
        use tui::Terminal;

        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;
        use std::fmt::Debug;

        use sauron_ui::backend::text_ui::TuiWidget;
        use sauron_ui::event::*;
        use sauron_ui::widget::*;
        use sauron_ui::Node;
        use termion::input::TermRead;
        use sauron_ui::Component;

        /// A small event handler that wrap termion input and tick events. Each event
        /// type is handled in its own thread and returned to a common `Receiver`
        pub struct Events {
            rx: mpsc::Receiver<Event>,
            input_handle: thread::JoinHandle<()>,
            tick_handle: thread::JoinHandle<()>,
        }

        #[derive(Debug, Clone, Copy)]
        pub struct Config {
            pub tick_rate: Duration,
        }

        impl Default for Config {
            fn default() -> Config {
                Config {
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
                                TermEvent::Key(k) => {
                                    if let TermKey::Char(ch) = k {
                                        tx.send(Event::KeyEvent(KeyEvent::new(ch)));
                                    }
                                }
                                TermEvent::Mouse(me) => match me {
                                    TermMouseEvent::Press(btn, x, y) => match btn {
                                        TermMouseButton::Left => {
                                            let event = Event::MouseEvent(MouseEvent::Press(
                                                MouseButton::Left,
                                                x,
                                                y,
                                            ));
                                            tx.send(event);
                                        }
                                        _ => {}
                                    },
                                    _ => {}
                                },
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
                            tx.send(Event::Tick).unwrap();
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

            pub fn next(&self) -> Result<Event, mpsc::RecvError> {
                self.rx.recv()
            }
        }

        #[derive(Clone,Debug)]
        enum Msg{
            Click,
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

        impl Component<Msg> for App {
            fn update(&mut self, msg: Msg) {
            }
            fn view(&self) -> Node<Msg>
            {
                let vdom = row([], [column([], [])]);
                vdom
            }
        }

        fn draw_ui<B>(mut f: tui::Frame<B>, app: &mut App)
        where
            B: tui::backend::Backend,
        {
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

            let widget:Node<Msg> = text("HI --> ");
            let tuiw: TuiWidget<sauron_ui::Widget> = widget_node_tree_to_tui_widget(widget);

            Paragraph::new([Text::raw(&app.input)].iter())
                .style(Style::default().fg(Color::Yellow))
                .render(&mut f, chunks[0]);

            Paragraph::new([Text::raw(&app.formatted)].iter())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL))
                .render(&mut f, chunks[1]);

            let mut messages: Vec<Text> = app
                .messages
                .iter()
                .enumerate()
                .map(|(i, m)| Text::raw(format!("{}: {}", i, m)))
                .collect();

            match tuiw {
                TuiWidget::Text(txt) => {
                    messages.push(txt);
                }
                _ => {}
            }

            List::new(messages.into_iter())
                .block(Block::default().borders(Borders::ALL).title("Data"))
                .render(&mut f, chunks[2]);
        }

        fn setup_terminal(
        ) -> Result<Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>, io::Error>
        {
            let stdout = io::stdout().into_raw_mode()?;
            let stdout = MouseTerminal::from(stdout);
            let stdout = AlternateScreen::from(stdout);
            let backend = TermionBackend::new(stdout);
            Terminal::new(backend)
        }

        fn main() -> Result<(), failure::Error> {
            // Terminal initialization
            let mut terminal = setup_terminal()?;
            let events = Events::new();
            let mut app = App::default();

            loop {
                // Draw UI
                terminal.draw(|mut f| {
                    draw_ui(f, &mut app);
                })?;

                // Handle input
                match events.next()? {
                    Event::KeyEvent(key) => match key.key.as_ref() {
                        "q" => {
                            break;
                        }
                        "\n" => {
                            app.formatted = app.input.to_uppercase();
                            app.messages.push(app.input.drain(..).collect());
                        }
                        key => app.input.push_str(key),
                    },
                    Event::MouseEvent(me) => {
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
    }
}

#[cfg(not(feature = "with-tui"))]
fn main() {}
