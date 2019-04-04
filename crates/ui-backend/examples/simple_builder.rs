use virtual_dom::builder::*;
use ui_backend::html::*;

fn main() {
    let html = div(
        [
            class("some-class"),
            id("some-id"),
            on_click(|_| {
                println!("clicked");
            }),
            attribute("data-id", 1223442),
            on_event("mouseover", |_|{println!("i've been clicked");}),
        ],
        [div([], [])],
    );
    println!("{:#?}", html);
}
