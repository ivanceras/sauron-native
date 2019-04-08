use html_backend::html::attributes::*;
use html_backend::html::events::*;
use html_backend::html::*;
use html_backend::DomUpdater;
use vdom::builder::*;

fn main() {
    let html = div(
        [
            class("some-class"),
            id("some-id"),
            onclick(|_| {
                println!("clicked");
            }),
            attr("data-id", 1),
            on("mouseover", |_| {
                println!("i've been clicked");
            }),
        ],
        [input([class("client"), r#type("checkbox")], [])],
    );

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let root_node = document.body().unwrap().into();
    let dom_updated = DomUpdater::new_replace_mount(html, root_node);
}
