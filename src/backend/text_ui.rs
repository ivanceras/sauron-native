use crate::{widget::Widget, Attribute, Backend, Component, Node};
use events::Events;
use itui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Text, Widget as TermWidget},
    Frame, Terminal,
};
use nodes::TuiWidget;
use sauron_vdom::{
    builder::element,
    event::{KeyEvent, Modifier},
    Event,
};
use std::{
    cell::RefCell,
    fmt::Debug,
    io::{self, Stdout},
    marker::PhantomData,
    rc::Rc,
    sync::mpsc,
    thread,
    time::Duration,
};
use termion::{
    event::{
        Event as TermEvent, Key as TermKey, MouseButton as TermMouseButton,
        MouseEvent as TermMouseEvent,
    },
    input::{MouseTerminal, TermRead},
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};

mod events;
mod nodes;

type TermionTerminal =
    Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>;

pub struct TuiBackend<APP, MSG> {
    terminal: Rc<RefCell<TermionTerminal>>,
    app: Rc<RefCell<APP>>,
    _phantom_msg: PhantomData<MSG>,
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
                self.draw_ui(frame, &None);
            });
            let event = events.next().ok();
            if let Some(Event::KeyEvent(ref key)) = event {
                // break on CTRL-C
                if key.modifier == Modifier::ctrl() && key.key == "c" {
                    break;
                }
            }
            self.terminal.borrow_mut().draw(|mut frame| {
                self.draw_ui(frame, &event);
            });
            if let Some(event) = event {
                self.app.borrow_mut().on_event(event);
            }
        }
    }

    fn draw_ui<B>(&self, mut frame: itui::Frame<B>, event: &Option<Event>)
    where
        B: itui::backend::Backend,
    {
        let view = self.app.borrow().view();
        let frame_size = frame.size();
        let tui_view = nodes::convert_widget_node_tree_to_tui_widget(view);
        self.draw_widget_node_tree(tui_view, &mut frame, frame_size, event);
    }
    fn draw_widget_node_tree<B>(
        &self,
        tui_widget: TuiWidget<MSG>,
        frame: &mut Frame<B>,
        area: Rect,
        event: &Option<Event>,
    ) where
        MSG: Clone + Debug + 'static,
        B: itui::backend::Backend,
    {
        match tui_widget {
            TuiWidget::Layout(layout) => {
                let chunks = Layout::default()
                    .direction(layout.direction)
                    .constraints(layout.constraints)
                    .split(area);
                for (i, child) in layout.children.into_iter().enumerate() {
                    self.draw_widget_node_tree(child, frame, chunks[i], event);
                }
            }
            TuiWidget::Paragraph(paragraph) => {
                let text: Vec<Text> = paragraph.text.iter().map(|txt| Text::raw(txt)).collect();
                let mut actual_paragraph: Paragraph<_, MSG> =
                    Paragraph::new(text.iter()).area(area);
                actual_paragraph.events = paragraph.events;
                if let Some(block) = &paragraph.block {
                    let mut tui_block = itui::widgets::Block::default()
                        .title_style(block.title_style)
                        .borders(block.borders)
                        .border_style(block.border_style)
                        .area(area)
                        .style(block.style);
                    if let Some(title) = &block.title {
                        tui_block = tui_block.title(&title);
                    }

                    if let Some(event) = event {
                        let cb = tui_block.triggers_event(event);
                        if let Some(cb) = cb {
                            let msg = cb.emit(event.clone());
                            self.app.borrow_mut().update(msg);
                        }
                    }

                    actual_paragraph = actual_paragraph.block(tui_block);
                }
                if let Some(event) = event {
                    let cb = actual_paragraph.triggers_event(event);
                    if let Some(cb) = cb {
                        let msg = cb.emit(event.clone());
                        self.app.borrow_mut().update(msg);
                    }
                }
                actual_paragraph.render(frame);
            }
            TuiWidget::Block(block) => {
                let mut actual_block: Block<MSG> = itui::widgets::Block::default()
                    .title_style(block.title_style)
                    .borders(block.borders)
                    .border_style(block.border_style)
                    .area(area)
                    .style(block.style);
                actual_block.events = block.events;
                if let Some(title) = &block.title {
                    actual_block = actual_block.title(&title)
                }

                if let Some(event) = event {
                    let cb = actual_block.triggers_event(event);
                    if let Some(cb) = cb {
                        let msg = cb.emit(event.clone());
                        self.app.borrow_mut().update(msg);
                    }
                }
                actual_block.render(frame);
            }
            TuiWidget::Button(button) => {
                let mut button = button.clone();
                /*
                let mut actual_block: Block<MSG> = itui::widgets::Block::default()
                    .title_style(block.title_style)
                    .borders(block.borders)
                    .border_style(block.border_style)
                    .area(area)
                    .style(block.style);
                actual_block.events = block.events;
                if let Some(title) = &block.title {
                    actual_block = actual_block.title(&title)
                }
                */
                let mut area1 = area.clone();
                area1.height = 3;
                let label_len = button.text.len();
                area1.width = if label_len > 0 {
                    (label_len + 4) as u16
                } else {
                    10
                };
                button = button.area(area1);

                if let Some(event) = event {
                    let cb = button.triggers_event(event);
                    if let Some(cb) = cb {
                        let msg = cb.emit(event.clone());
                        self.app.borrow_mut().update(msg);
                    }
                }
                button.render(frame);
            }
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
            app: Rc::new(RefCell::new(app)),
            _phantom_msg: PhantomData,
        };

        let backend = Rc::new(tui_backend);
        backend
    }

    fn start_render(self: &Rc<Self>) {
        self.start_draw_loop();
    }
}

fn setup_terminal() -> Result<TermionTerminal, io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    Terminal::new(backend)
}
