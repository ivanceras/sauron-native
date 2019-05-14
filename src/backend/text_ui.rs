use crate::{widget::Widget, Backend, Component};
use std::{
    cell::RefCell,
    fmt::Debug,
    io::{self, Stdout},
    rc::Rc,
};
use termion::{
    input::MouseTerminal,
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Text, Widget as TermWidget},
    Frame, Terminal,
};

use sauron_vdom::{Event};
use sauron_vdom::event::KeyEvent;
use std::{marker::PhantomData, sync::mpsc, thread, time::Duration};
use termion::{
    event::{
        Event as TermEvent, Key as TermKey, MouseButton as TermMouseButton,
        MouseEvent as TermMouseEvent,
    },
    input::TermRead,
};

use crate::{Attribute, Node};
use nodes::TuiWidget;
use sauron_vdom::builder::element;
use tui::style::{Color, Modifier, Style};

mod nodes;

pub struct TuiBackend<APP, MSG> {
    terminal:
        Rc<RefCell<Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>>>,
    app: APP,
    _phantom_msg: PhantomData<MSG>,
}

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

impl<APP, MSG> TuiBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Clone + Debug + 'static,
{
    fn start_draw_loop(&self) {
        let events = Events::new();
        loop {
            self.terminal.borrow_mut().draw(|mut frame| {
                self.draw_ui(frame);
            });

            match events.next().expect("couldn't read events") {
                Event::KeyEvent(key) => match key.key.as_ref() {
                    "q" => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    fn draw_ui<B>(&self, mut frame: tui::Frame<B>)
    where
        B: tui::backend::Backend,
    {
        let view = self.app.view();
        let frame_size = frame.size();
        let tui_view = nodes::convert_widget_node_tree_to_tui_widget(view);
        self.draw_widget_node_tree(tui_view, &mut frame, frame_size);
    }
    fn draw_widget_node_tree<B>(&self, tui_widget: TuiWidget, frame: &mut Frame<B>, area: Rect)
    where
        MSG: Clone + Debug + 'static,
        B: tui::backend::Backend,
    {
        match tui_widget {
            TuiWidget::Layout(layout) => {
                let chunks = Layout::default()
                    .direction(layout.direction)
                    .constraints(layout.constraints)
                    .split(area);
                for (i, child) in layout.children.into_iter().enumerate() {
                    self.draw_widget_node_tree(child, frame, chunks[i]);
                }
            }
            TuiWidget::Paragraph(paragraph) => {
                let text: Vec<Text> = paragraph.text.iter().map(|txt| Text::raw(txt)).collect();
                let mut actual_paragraph = Paragraph::new(text.iter());
                if let Some(block) = &paragraph.block {
                    let mut tui_block = tui::widgets::Block::default()
                        .title_style(block.title_style)
                        .borders(block.borders)
                        .border_style(block.border_style)
                        .style(block.style);
                    if let Some(title) = &block.title {
                        tui_block = tui_block.title(&title);
                    }

                    actual_paragraph = actual_paragraph.block(tui_block);
                }
                actual_paragraph.render(frame, area);
            }
            TuiWidget::Block(block) => {
                let mut actual_block = tui::widgets::Block::default()
                    .title_style(block.title_style)
                    .borders(block.borders)
                    .border_style(block.border_style)
                    .style(block.style);
                if let Some(title) = &block.title {
                    actual_block = actual_block.title(&title)
                }
                actual_block.render(frame, area);
            }
            _ => {}
        }
    }
}

impl<APP, MSG> Backend<APP, MSG> for TuiBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: Clone + Debug + 'static,
{
    fn init(app: APP) -> Rc<Self> {
        println!("Initializing terminal backend");
        let terminal = setup_terminal().expect("unable to setup terminal");
        let tui_backend = TuiBackend {
            terminal: Rc::new(RefCell::new(terminal)),
            app,
            _phantom_msg: PhantomData,
        };

        let backend = Rc::new(tui_backend);
        backend
    }

    fn start_render(self: &Rc<Self>) {
        self.start_draw_loop();
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
                                tx.send(Event::KeyEvent(KeyEvent::new(ch.to_string())));
                            }
                        }
                        TermEvent::Mouse(me) => {}
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
                    //tx.send(Event::Tick).unwrap();
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

fn setup_terminal(
) -> Result<Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>, io::Error>
{
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    Terminal::new(backend)
}
