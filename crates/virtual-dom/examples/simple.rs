use virtual_dom::builder::*;
use virtual_dom::diff;

fn main() {
    let old = element(
        "div",
        [
            attribute("class", "some-class"),
            attribute("id", "some-id"),
            on_event("click", |_| {
                println!("clicked");
            }),
            attribute("data-id", 1111),
            on_event("mouseover", |_| {
                println!("i've been clicked");
            }),
        ],
        [element("div", [], [text("Hello world!")])],
    );

    let new = element(
        "div",
        [
            attribute("class", "some-class2"),
            attribute("id", "some-id2"),
            on_event("click", |_| {
                println!("clicked2");
            }),
            attribute("data-id", 2222),
            on_event("mouseover", |_| {
                println!("i've been clicked");
            }),
        ],
        [element("div", [], [text("Wazzup!")])],
    );

    println!("old: {}", old);
    println!("new: {}", new);
    let patches = diff(&old, &new);
    println!("patches: {:#?}", patches);
}
