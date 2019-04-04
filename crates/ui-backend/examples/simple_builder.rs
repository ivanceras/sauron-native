use ui_backend::html::attributes::*;
use ui_backend::html::*;
use virtual_dom::builder::*;
use virtual_dom::diff;

fn main() {
    let old = div(
        [
            class("some-class"),
            id("some-id"),
            on_click(|_| {
                println!("clicked");
            }),
            attribute("data-id", 1),
            on_event("mouseover", |_| {
                println!("i've been clicked");
            }),
        ],
        [input([class("client"), r#type("checkbox")], [])],
    );
    let new = div(
        [
            class("some-class2"),
            id("some-id2"),
            on_click(|_| {
                println!("clicked2");
            }),
            attribute("data-id", 2),
            on_event("mouseover", |_| {
                println!("i've been clicked2");
            }),
        ],
        [input([class("client"), r#type("checkbox")], [])],
    );
    println!("{}", old);
    println!("{}", new);
    let patches = diff(&old, &new);
    println!("patches: {:#?}", patches);
}
