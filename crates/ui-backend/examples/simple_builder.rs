use ui_backend::html::*;
use ui_backend::html::attributes::*;
use virtual_dom::builder::*;


fn main() {
    let html = div(
        [
            class("some-class"),
            id("some-id"),
            on_click(|_| {
                println!("clicked");
            }),
            attribute("data-id", 1223442),
            on_event("mouseover", |_| {
                println!("i've been clicked");
            }),
        ],
        [input(
             [class("client"),
             r#type("checkbox"),
             ], [])
        ],
    );
    println!("{:#?}", html);
}
