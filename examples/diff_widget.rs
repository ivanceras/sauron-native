use sauron_native::{widget::*, Node};
use sauron_vdom::diff;
use sauron_native::util::*;

fn main() {
    let old: Node<()> = vbox(
        vec![],
        vec![
            hbox(
                vec![attr("class", "column1")],
                vec![
                    button(vec![value( "column1 element1")]),
                    button(vec![value( "column1 element2")]),
                    button(vec![value( "column1 element3")]),
                    button(vec![value( "column1 element4")]),
                    button(vec![value( "column1 element5")]),
                    button(vec![value( "column1 element6")]),
                ],
            ),
            hbox(
                vec![attr("class", "column2")],
                vec![button(vec![value("column2")]), button(vec![value("c2 element2")])],
            ),
            button(vec![value("Hello")]),
            block("I'm a block kid!"),
            text(
                "Hello, will this be a paragrapah\n
                    The quick brown fox jumps over the lazy\n
                    dog. Lorem ipsun\n
                    The shadows of mordor\n
                     ",
            ),
        ],
    );

    let new: Node<()> = vbox(
        vec![],
        vec![
            hbox(
                vec![attr("class", "column1")],
                vec![
                    button(vec![value( "Changed column1 element1")]),
                    button(vec![value( "column1 element2")]),
                    button(vec![value( "column1 element3")]),
                    button(vec![value( "column1 element4")]),
                    button(vec![value( "Changed column1 element5")]),
                    button(vec![value( "Changed column1 element6")]),
                ],
            ),
            hbox(
                vec![attr("class", "column2-changed")],
                vec![
                    button(vec![value("column2-changed")]),
                    button(vec![value("c2 element2")]),
                ],
            ),
            button(vec![value("Hello")]),
            block("I'm a block kid!"),
            text(
                "Hello, will this be a paragrapah\n
                    The quick brown fox jumps over the lazy\n
                    dog. Lorem ipsun\n
                    The shadows of mordor\n
                     ",
            ),
        ],
    );

    let changed = diff(&old, &new);
    println!("changed: {:#?}", changed);
}
