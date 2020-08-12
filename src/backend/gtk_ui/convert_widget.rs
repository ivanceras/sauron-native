use super::images;
use super::Dispatch;
use super::GtkWidget;
use crate::widget::attribute::util::get_layout;
use crate::widget::event::{InputEvent, MouseEvent};
use crate::{
    widget::attribute::{find_callback, find_value, util::is_scrollable},
    AttribKey, Attribute, Widget,
};
use gdk_pixbuf::{PixbufLoader, PixbufLoaderExt};
use gio::prelude::*;
use gtk::{
    prelude::*, Adjustment, Button, CheckButton, Entry, EntryBuffer, EventBox,
    Frame, HeaderBar, Image, Label, LabelBuilder, Menu, MenuBar, MenuItem,
    Orientation, Overlay, Paned, RadioButton, ScrolledWindow, SearchEntry,
    TextBuffer, TextBufferExt, TextTagTable, TextView, TextViewExt, WidgetExt,
};
use std::fmt::Debug;

pub(crate) fn from_node_tree<DSP, MSG>(
    program: &DSP,
    widget_node: &crate::Node<MSG>,
) -> GtkWidget
where
    MSG: Debug + 'static,
    DSP: Clone + Dispatch<MSG> + 'static,
{
    match widget_node {
        crate::Node::Element(element) => from_node(program, &element),
        crate::Node::Text(_) => unreachable!(),
    }
}

pub(crate) fn from_node<MSG, DSP>(
    program: &DSP,
    element: &crate::Element<MSG>,
) -> GtkWidget
where
    MSG: Debug + 'static,
    DSP: Clone + Dispatch<MSG> + 'static,
{
    let widget: &Widget = element.tag();
    let attrs: &[Attribute<MSG>] = element.get_attributes();
    let children: &[crate::Node<MSG>] = element.get_children();
    let layout = get_layout(&element).expect("must have a layout");
    let width = layout.size.width;
    let height = layout.size.height;

    let mut widget_children: Vec<GtkWidget> = vec![];
    for child in children.iter() {
        let gtk_child = from_node_tree(program, &child);
        widget_children.push(gtk_child);
    }

    match widget {
        // gbox can have many children
        Widget::Vbox | Widget::Hbox => {
            let orientation = match widget {
                Widget::Vbox => Orientation::Vertical,
                Widget::Hbox => Orientation::Horizontal,
                _ => unreachable!(),
            };
            let gbox = gtk::Box::new(orientation, 0);

            for child in widget_children.iter() {
                if let Some(child_widget) = child.as_widget() {
                    //container.pack_start(child_widget, false, false, 0);
                    gbox.add(child_widget);
                } else {
                    println!(
                        "was not able to add child widget: {:?}",
                        child.as_widget()
                    );
                }
            }
            if is_scrollable(&attrs) {
                println!("wrapping the gbox with ScrolledWindow");
                let scroll = ScrolledWindow::new(
                    None::<&Adjustment>,
                    None::<&Adjustment>,
                );
                println!("gbox size: ({},{})", width, height);
                scroll.add(&gbox);
                scroll.set_size_request(width as i32, height as i32);
                scroll.set_propagate_natural_height(true);
                gbox.set_size_request(width as i32, height as i32);
                GtkWidget::GBoxScrollable(scroll)
            } else {
                gbox.set_size_request(width as i32, height as i32);
                GtkWidget::GBox(gbox)
            }
        }
        Widget::GroupBox => {
            let label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.as_str())
                .flatten();
            let frame = Frame::new(label);
            let vbox = gtk::Box::new(Orientation::Vertical, 0);
            vbox.set_size_request(width as i32, height as i32);

            for child in widget_children.iter() {
                if let Some(child_widget) = child.as_widget() {
                    vbox.add(child_widget);
                } else {
                    println!(
                        "was not able to add child widget: {:?}",
                        child.as_widget()
                    );
                }
            }
            frame.add(&vbox);
            GtkWidget::GroupBox(frame)
        }
        // paned has only 2 children
        Widget::Hpane => {
            //TODO: make these infallable and more ergonomic
            let child1_attrs =
                children[0].get_attributes().expect("must have attributes");

            let hpane = Paned::new(Orientation::Horizontal);
            if widget_children.len() != 2 {
                log::warn!("pane should have 2 children");
            }
            if widget_children.len() > 2 {
                log::warn!("pane children excess of 2 is ignored");
            }
            if let Some(child1) =
                widget_children.get(0).map(|c| c.as_widget()).flatten()
            {
                let is_resizable =
                    find_value(AttribKey::Resizable, &child1_attrs)
                        .map(|v| v.as_bool())
                        .unwrap_or(true);
                hpane.pack1(child1, true, true);
                hpane.set_child_resize(child1, is_resizable);
            }
            if let Some(child2) =
                widget_children.get(1).map(|c| c.as_widget()).flatten()
            {
                let child2_attrs =
                    children[1].get_attributes().expect("must have attributes");
                let is_resizable =
                    find_value(AttribKey::Resizable, &child2_attrs)
                        .map(|v| v.as_bool())
                        .unwrap_or(true);
                hpane.pack2(child2, true, true);
                hpane.set_child_resize(child2, is_resizable);
            }
            if let Some(first_child) =
                children.first().map(|c| c.as_element_ref()).flatten()
            {
                let child1_layout =
                    get_layout(first_child).expect("must have a layout");
                hpane.set_position(child1_layout.size.width as i32);
            }

            hpane.set_size_request(width as i32, height as i32);
            GtkWidget::Paned(hpane)
        }
        Widget::Vpane => {
            let vpane = Paned::new(Orientation::Vertical);
            if widget_children.len() != 2 {
                log::warn!("pane should have 2 children");
            }
            if widget_children.len() > 2 {
                log::warn!("pane children excess of 2 is ignored");
            }
            if let Some(child1) =
                widget_children.get(0).map(|c| c.as_widget()).flatten()
            {
                vpane.pack1(child1, true, true);
            }
            if let Some(child2) =
                widget_children.get(1).map(|c| c.as_widget()).flatten()
            {
                vpane.pack2(child2, true, true);
            }
            if let Some(first_child) =
                children.first().map(|c| c.as_element_ref()).flatten()
            {
                let child1_layout =
                    get_layout(first_child).expect("must have a layout");
                vpane.set_position(child1_layout.size.width as i32);
            }
            vpane.set_size_request(width as i32, height as i32);
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

            btn.set_size_request(width as i32, height as i32);
            GtkWidget::Button(btn)
        }
        Widget::Paragraph => {
            let buffer = TextBuffer::new(None::<&TextTagTable>);
            let text_view = TextView::new_with_buffer(&buffer);

            let txt = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or_default();

            buffer.set_text(&txt);

            GtkWidget::Paragraph(text_view)
        }
        Widget::TextInput => {
            let value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or_default();

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
                .unwrap_or_default();

            let event_box = EventBox::new();
            if let Some(callbacks) = find_callback(AttribKey::ClickEvent, attrs)
            {
                for cb in callbacks {
                    let cb_clone = cb.clone();
                    let program_clone = program.clone();
                    // should be on the click event,
                    // TODO: find a way to expresse
                    // click event using button_press and button_release
                    event_box.connect_button_press_event(
                        move |_view, event| {
                            println!("btn is clicked..");
                            let mouse_event = MouseEvent::default();
                            let msg = cb_clone.emit(mouse_event);
                            program_clone.dispatch(msg);
                            Inhibit(false)
                        },
                    );
                }
            }
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

            let label =
                LabelBuilder::new().label(&*value).name("label").build();

            event_box.add(&label);

            label.show();
            event_box.show();
            GtkWidget::Label(event_box)
        }
        Widget::Checkbox => {
            let label = find_value(AttribKey::Label, &attrs)
                .map(|v| v.to_string())
                .unwrap_or_default();

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
                .unwrap_or_default();

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
            let mime = images::image_mime_type(&bytes)
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

            image.set_size_request(width as i32, height as i32);
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

            image.set_size_request(width as i32, height as i32);
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
                .unwrap_or_default();

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

            text_view.set_size_request(width as i32, height as i32);

            if is_scrollable(&attrs) {
                let scroll = ScrolledWindow::new(
                    None::<&Adjustment>,
                    None::<&Adjustment>,
                );
                scroll.set_size_request(width as i32, height as i32);
                scroll.add(&text_view);
                GtkWidget::TextViewScrollable(scroll)
            } else {
                GtkWidget::TextView(text_view)
            }
        }
        Widget::Overlay => {
            let overlay = Overlay::new();

            for (index, child) in widget_children.iter().enumerate() {
                if let Some(child_widget) = child.as_widget() {
                    overlay.add_overlay(child_widget);
                    let c_index = overlay.get_child_index(child_widget);
                    assert_eq!(c_index, index as i32);
                } else {
                    println!(
                        "was not able to add child widget: {:?}",
                        child.as_widget()
                    );
                }
            }
            overlay.set_size_request(width as i32, height as i32);
            overlay.show_all();
            GtkWidget::Overlay(overlay)
        }
        Widget::HeaderBar => {
            let header_bar = HeaderBar::new();

            for child in widget_children.iter() {
                if let Some(child_widget) = child.as_widget() {
                    header_bar.add(child_widget);
                } else {
                    println!(
                        "was not able to add child widget: {:?}",
                        child.as_widget()
                    );
                }
            }
            GtkWidget::HeaderBar(header_bar)
        }
        Widget::MenuBar => {
            let menu_bar = MenuBar::new();

            for child in widget_children.iter() {
                if let Some(child_widget) = child.as_widget() {
                    menu_bar.add(child_widget);
                } else {
                    println!(
                        "was not able to add child widget: {:?}",
                        child.as_widget()
                    );
                }
            }
            GtkWidget::MenuBar(menu_bar)
        }
        Widget::Menu => {
            let menu = Menu::new();

            for child in widget_children.iter() {
                if let Some(child_widget) = child.as_widget() {
                    menu.add(child_widget);
                } else {
                    println!(
                        "was not able to add child widget: {:?}",
                        child.as_widget()
                    );
                }
            }
            GtkWidget::Menu(menu)
        }
        Widget::MenuItem => {
            let menu_item = MenuItem::new();

            if let Some(callbacks) = find_callback(AttribKey::ClickEvent, attrs)
            {
                println!("menu item has click event");
                for cb in callbacks {
                    let cb_clone = cb.clone();
                    let program_clone = program.clone();
                    menu_item.connect_activate(move |_| {
                        println!("menu item is clicked..");
                        let mouse_event = MouseEvent::default();
                        let msg = cb_clone.emit(mouse_event);
                        program_clone.dispatch(msg);
                    });
                }
            } else {
                println!("No click event for menu item");
            }

            for child in widget_children.iter() {
                match child {
                    GtkWidget::Menu(sub_menu) => {
                        menu_item.set_submenu(Some(sub_menu));
                    }
                    _ => {
                        if let Some(child_widget) = child.as_widget() {
                            menu_item.add(child_widget);
                        } else {
                            println!(
                                "was not able to add child widget: {:?}",
                                child.as_widget()
                            );
                        }
                    }
                }
            }
            GtkWidget::MenuItem(menu_item)
        }
        Widget::SearchInput => {
            let value = find_value(AttribKey::Value, &attrs)
                .map(|v| v.to_string())
                .unwrap_or_default();

            let entry = SearchEntry::new();

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
            GtkWidget::SearchInput(entry)
        }
    }
}
