use browser::html::attributes::*;
use browser::html::events::*;
use browser::html::*;
use browser::DomUpdater;
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