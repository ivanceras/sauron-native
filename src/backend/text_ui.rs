use crate::{widget::Widget, AttribKey, Attribute, Backend, Component, Node};
use events::Events;
use itui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Button, Paragraph, Text, Widget as TermWidget},
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
    MSG: 'static,
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
        MSG: 'static,
        B: itui::backend::Backend,
    {
        match tui_widget {
            TuiWidget::Layout(layout) => {
                let chunks = Layout::default()
                    .direction(layout.direction)
                    .constraints(layout.constraints)
                    .split(area);
                for (i, child) in layout.children.into_iter().enumerate() {
                    //TODO: calculate the chunks area for each
                    // of the children, taking into account the preference/properties
                    // set in each of the children
                    self.draw_widget_node_tree(child, frame, chunks[i], event);
                }
            }
            TuiWidget::Paragraph(paragraph) => {
                let text: Vec<Text> = paragraph.text.iter().map(|txt| Text::raw(txt)).collect();
                let mut actual_paragraph: Paragraph<_, MSG> =
                    Paragraph::new(text.iter()).area(area);
                actual_paragraph.events = convert_events(paragraph.events);

                //TODO: the area of widgets should be handled when converting from abstract Widget
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
            TuiWidget::Button(mut button) => {
                button = button.area(area);

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

fn convert_events<MSG>(
    events: Vec<Attribute<MSG>>,
) -> Vec<sauron_vdom::Attribute<&'static str, Event, MSG>>
where
    MSG: 'static,
{
    events
        .into_iter()
        .filter_map(|att| {
            let att_name = att.name.to_static_str();
            if let Some(cb) = att.take_callback() {
                Some(sauron_vdom::Attribute::from_callback(att_name, cb))
            } else {
                None
            }
        })
        .collect()
}

impl AttribKey {
    fn to_static_str(&self) -> &'static str {
        match self {
            AttribKey::ClickEvent => "click",
            AttribKey::InputEvent => "input",
            AttribKey::Value => "value",
            AttribKey::Label => "label",
            _ => panic!("not yet implemented for {}", self),
        }
    }
}

impl<APP, MSG> Backend<APP, MSG> for TuiBackend<APP, MSG>
where
    APP: Component<MSG> + 'static,
    MSG: 'static,
{
    fn init(app: APP) -> Rc<Self> {
        let terminal = setup_terminal().expect("unable to setup terminal");
        let tui_backend = TuiBackend {
            terminal: Rc::new(RefCell::new(terminal)),
            app: Rc::new(RefCell::new(app)),
            _phantom_msg: PhantomData,
        };

        let backend = Rc::new(tui_backend);
        backend.start_draw_loop();
        backend
    }
}

fn setup_terminal() -> Result<TermionTerminal, io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    Terminal::new(backend)
}
