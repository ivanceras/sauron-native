use super::images;
use super::Dispatch;
use super::GtkWidget;
use crate::widget::attribute::event::{InputEvent, MouseEvent};
use crate::{
    util,
    widget::attribute::{find_callback, find_value, util::is_scrollable},
    AttribKey, Attribute, Backend, Component, Node, Widget,
};
use gdk_pixbuf::{PixbufLoader, PixbufLoaderExt};
use gio::{prelude::*, ApplicationFlags};
use gtk::{
    prelude::*, Adjustment, Application, ApplicationWindow, Button,
    CheckButton, Container, Entry, EntryBuffer, EventBox, Frame, Image, Label,
    Orientation, Overlay, Paned, RadioButton, ScrolledWindow, TextBuffer,
    TextBufferExt, TextTagTable, TextView, TextViewExt, WidgetExt,
};
use log::*;
use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};

pub(crate) fn from_node_tree<DSP, MSG>(
    program: &DSP,
    widget_node: crate::Node<MSG>,
) -> GtkWidget
where
    MSG: Debug + 'static,
    DSP: Clone + Dispatch<MSG> + 'static,
{
    match widget_node {
        crate::Node::Element(element) => {
            let gtk_widget = from_node(program, &element.tag, &element.attrs);
            let mut children = vec![];
            for child in element.children {
                let gtk_child = from_node_tree(program, child);
                children.push(gtk_child);
            }
            gtk_widget.add_children(children);
            gtk_widget
        }
        crate::Node::Text(_) => unreachable!(),
    }
}

pub(crate) fn from_node<MSG, DSP>(
    program: &DSP,
    widget: &Widget,
    attrs: &[Attribute<MSG>],
) -> GtkWidget
where
    MSG: Debug + 'static,
    DSP: Clone + Dispatch<MSG> + 'static,
{
    match widget {
        // vbox can have many children
        Widget::Vbox => {
            /*
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);
            */

            let vbox = gtk::Box::new(Orientation::Vertical, 0);
            //vbox.set_size_request(width as i32, height as i32);
            GtkWidget::GBox(vbox)
        }
        // hbox can have many children
        Widget::Hbox => {
            /*
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);
            */
            let hbox = gtk::Box::new(Orientation::Horizontal, 0);
            //hbox.set_size_request(width as i32, height as i32);
            GtkWidget::GBox(hbox)
        }
        Widget::GroupBox => {
            let label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.as_str())
                .flatten();
            let frame = Frame::new(label);
            let vbox = gtk::Box::new(Orientation::Vertical, 0);
            frame.add(&vbox);
            GtkWidget::GroupBox(frame)
        }
        // paned has only 2 children
        Widget::Hpane => {
            /*
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            let position = find_value(AttribKey::Position, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(width / 2.0);

            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);
            */

            let hpane = Paned::new(Orientation::Horizontal);
            //hpane.set_size_request(width as i32, height as i32);
            //hpane.set_position(position as i32);
            GtkWidget::Paned(hpane)
        }
        Widget::Vpane => {
            /*
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);
            */
            let vpane = Paned::new(Orientation::Vertical);
            //vpane.set_size_request(width as i32, height as i32);
            GtkWidget::Paned(vpane)
        }
        Widget::Button => {
            println!("it's a button");
            let label =
                find_value(AttribKey::Label, &attrs).map(|v| v.to_string());

            let svg_image_data = find_value(AttribKey::SvgImage, &attrs)
                .map(|v| v.as_bytes())
                .flatten();
            let btn = Button::new();
            if let Some(label) = label {
                btn.set_label(&label);
            }
            if let Some(callbacks) = find_callback(AttribKey::ClickEvent, attrs)
            {
                for cb in callbacks {
                    let cb_clone = cb.clone();
                    let program_clone = program.clone();
                    btn.connect_clicked(move |_| {
                        println!("btn is clicked..");
                        let mouse_event = MouseEvent::default();
                        let msg = cb_clone.emit(mouse_event);
                        program_clone.dispatch(msg);
                    });
                }
            }

            if let Some(svg_image_data) = svg_image_data {
                println!("got an svg image here..");
                let svg_image: Image = images::svg_image(&svg_image_data);
                btn.set_image(Some(&svg_image));
            }
            GtkWidget::Button(btn)
        }
        Widget::Paragraph => {
            let buffer = TextBuffer::new(None::<&TextTagTable>);
            let text_view = TextView::new_with_buffer(&buffer);

            let txt = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            buffer.set_text(&txt);

            GtkWidget::Paragraph(text_view)
        }
        Widget::TextInput => {
            let value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            let buffer = EntryBuffer::new(Some(&*value));
            let entry = Entry::new_with_buffer(&buffer);

            if let Some(callbacks) =
                find_callback(AttribKey::InputEvent, &attrs)
            {
                for cb in callbacks {
                    let cb_clone = cb.clone();
                    let program_clone = program.clone();
                    entry.connect_property_text_notify(move |entry| {
                        let input_event =
                            InputEvent::new(entry.get_buffer().get_text());
                        let msg = cb_clone.emit(input_event);
                        program_clone.dispatch(msg);
                    });
                }
            }
            GtkWidget::TextInput(entry)
        }
        Widget::Label => {
            let value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            let label = Label::new(Some(&*value));

            let event_box = EventBox::new();
            if let Some(callbacks) = find_callback(AttribKey::MouseDown, &attrs)
            {
                for cb in callbacks {
                    println!("label has some mouse down");
                    let cb_clone = cb.clone();
                    let program_clone = program.clone();
                    event_box.connect_button_press_event(
                        move |_view, event| {
                            println!("label is button pressed");
                            let (x, y) = event.get_position();
                            let mouse_event =
                                MouseEvent::pressed(x as i32, y as i32);
                            let msg = cb_clone.emit(mouse_event);
                            program_clone.dispatch(msg);
                            Inhibit(false)
                        },
                    );
                }
            }
            if let Some(callbacks) = find_callback(AttribKey::MouseUp, &attrs) {
                for cb in callbacks {
                    println!("label has some mouse up");
                    let cb_clone = cb.clone();
                    let program_clone = program.clone();
                    event_box.connect_button_release_event(
                        move |_view, event| {
                            println!("label is button released");
                            let (x, y) = event.get_position();
                            let mouse_event =
                                MouseEvent::release(x as i32, y as i32);
                            let msg = cb_clone.emit(mouse_event);
                            program_clone.dispatch(msg);
                            Inhibit(false)
                        },
                    );
                }
            }

            if let Some(callbacks) = find_callback(AttribKey::MouseMove, &attrs)
            {
                for cb in callbacks {
                    println!("label has some mouse up");
                    let cb_clone = cb.clone();
                    let program_clone = program.clone();
                    event_box.connect_motion_notify_event(
                        move |_view, event| {
                            println!("label is button released");
                            let (x, y) = event.get_position();
                            let mouse_event =
                                MouseEvent::mousemove(x as i32, y as i32);
                            let msg = cb_clone.emit(mouse_event);
                            program_clone.dispatch(msg);
                            Inhibit(false)
                        },
                    );
                }
            }
            event_box.add(&label);
            label.show();
            event_box.show();
            GtkWidget::Label(event_box)
        }
        Widget::Checkbox => {
            let label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            let value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.as_bool())
                .unwrap_or(false);

            let cb = CheckButton::new_with_label(&label);
            cb.set_property("active", &value)
                .expect("must be able to set property");
            GtkWidget::Checkbox(cb)
        }
        Widget::Radio => {
            let label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            let value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.as_bool())
                .unwrap_or(false);
            let rb = RadioButton::new_with_label(&label);
            rb.set_property("active", &value)
                .expect("must be able to set property");
            GtkWidget::Radio(rb)
        }
        Widget::Image => {
            let bytes: &[u8] = find_value(AttribKey::Data, &attrs)
                .map(|v| v.as_bytes())
                .flatten()
                .unwrap_or(&[]);
            let image = Image::new();
            let mime = util::image_mime_type(&bytes)
                .expect("unsupported have mime type");
            let pixbuf_loader =
                PixbufLoader::new_with_mime_type(mime).expect("error loader");
            pixbuf_loader
                .write(&bytes)
                .expect("Unable to write svg data into pixbuf_loader");

            pixbuf_loader.close().expect("error creating pixbuf");

            let pixbuf = pixbuf_loader.get_pixbuf();

            image.set_from_pixbuf(Some(
                &pixbuf.expect("error in pixbuf_loader"),
            ));

            /*
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);
            image.set_size_request(width as i32, height as i32);
            */
            GtkWidget::Image(image)
        }
        Widget::Svg => {
            let empty = vec![];
            let bytes = find_value(AttribKey::Data, &attrs)
                .map(|v| v.as_bytes())
                .flatten()
                .unwrap_or(&empty);
            let image = Image::new();
            let pixbuf_loader =
                PixbufLoader::new_with_mime_type("image/svg+xml")
                    .expect("error loader");
            pixbuf_loader
                .write(bytes)
                .expect("Unable to write svg data into pixbuf_loader");

            pixbuf_loader.close().expect("error creating pixbuf");

            let pixbuf = pixbuf_loader.get_pixbuf();

            image.set_from_pixbuf(Some(
                &pixbuf.expect("error in pixbuf_loader"),
            ));
            /*
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);
            */

            if let Some(callbacks) = find_callback(AttribKey::MouseDown, &attrs)
            {
                for cb in callbacks {
                    println!("textview has some mouse down");
                    let cb_clone = cb.clone();
                    let program_clone = program.clone();
                    image.connect_button_press_event(move |_view, event| {
                        println!("textview is button pressed");
                        let (x, y) = event.get_position();
                        let mouse_event =
                            MouseEvent::pressed(x as i32, y as i32);
                        let msg = cb_clone.emit(mouse_event);
                        program_clone.dispatch(msg);
                        Inhibit(false)
                    });
                }
            }

            /*
            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);
            image.set_size_request(width as i32, height as i32);
            */
            if is_scrollable(&attrs) {
                let scroll = ScrolledWindow::new(
                    None::<&Adjustment>,
                    None::<&Adjustment>,
                );
                scroll.add(&image);
                GtkWidget::ImageScrollable(scroll)
            } else {
                GtkWidget::Image(image)
            }
        }
        Widget::TextArea => {
            let value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or(String::new());

            let editable = find_value(AttribKey::Editable, &attrs)
                .map(|v| v.as_bool())
                .unwrap_or(true);

            let buffer = TextBuffer::new(None::<&TextTagTable>);
            buffer.set_text(&value);

            if let Some(callbacks) =
                find_callback(AttribKey::InputEvent, &attrs)
            {
                for cb in callbacks {
                    let cb_clone = cb.clone();
                    let program_clone = program.clone();
                    buffer.connect_end_user_action(move |buffer| {
                        let buffer_text = buffer.get_text(
                            &buffer.get_start_iter(),
                            &buffer.get_end_iter(),
                            true,
                        );
                        if let Some(buffer_text) = buffer_text {
                            let input_event =
                                InputEvent::new(buffer_text.to_string());
                            let msg = cb_clone.emit(input_event);
                            program_clone.dispatch(msg);
                        }
                    });
                }
            }

            let text_view = TextView::new_with_buffer(&buffer);
            text_view.set_monospace(true);
            text_view.set_editable(editable);

            /*
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);
            */

            if let Some(callbacks) = find_callback(AttribKey::MouseDown, &attrs)
            {
                for cb in callbacks {
                    println!("textview has some mouse down");
                    let cb_clone = cb.clone();
                    let program_clone = program.clone();
                    text_view.connect_button_press_event(
                        move |_view, event| {
                            println!("textview is button pressed");
                            let (x, y) = event.get_position();
                            let mouse_event =
                                MouseEvent::pressed(x as i32, y as i32);
                            let msg = cb_clone.emit(mouse_event);
                            program_clone.dispatch(msg);
                            Inhibit(false)
                        },
                    );
                }
            }

            //text_view.set_size_request(width as i32, height as i32);

            if is_scrollable(&attrs) {
                let scroll = ScrolledWindow::new(
                    None::<&Adjustment>,
                    None::<&Adjustment>,
                );
                //scroll.set_size_request(width as i32, height as i32);
                scroll.add(&text_view);
                GtkWidget::TextViewScrollable(scroll)
            } else {
                GtkWidget::TextView(text_view)
            }
        }
        Widget::Overlay => {
            let overlay = Overlay::new();

            /*
            let width = find_value(AttribKey::Width, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);

            let height = find_value(AttribKey::Height, &attrs)
                .map(|v| v.as_f64())
                .flatten()
                .unwrap_or(20.0);
            */
            //overlay.set_size_request(width as i32, height as i32);
            overlay.show_all();
            GtkWidget::Overlay(overlay)
        }
    }
}
