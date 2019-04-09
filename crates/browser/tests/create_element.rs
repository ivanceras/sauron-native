use browser::dom::CreatedNode;
use browser::html::attributes::*;
use browser::html::events::*;
use browser::html::*;
use browser::*;

use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

use web_sys::{Element, Event, EventTarget, MouseEvent};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn nested_divs() {
    let vdiv = div([], [div([], [div([], [])])]); // <div> <div> <div></div> </div> </div>
    let div: Element = CreatedNode::<Element>::create_dom_node(&vdiv)
        .node
        .unchecked_into();

    assert_eq!(&div.inner_html(), "<div><div></div></div>");
}

#[wasm_bindgen_test]
fn svg_element() {
    let vdiv = div(
        [],
        [svg(
            [xmlns("http://www.w3.org/2000/svg")],
            [circle([cx("50"), cy("50"), r("50")], [])],
        )],
    );
    let div: Element = CreatedNode::<Element>::create_dom_node(&vdiv)
        .node
        .unchecked_into();

    assert_eq!(
        &div.inner_html(),
        r#"<svg xmlns="http://www.w3.org/2000/svg"><circle cx="50" cy="50" r="50"></circle></svg>"#
    );
}

#[wasm_bindgen_test]
fn div_with_attributes() {
    let vdiv = div([id("id-here"), class("two classes")], []);
    let div: Element = CreatedNode::<Element>::create_dom_node(&vdiv)
        .node
        .unchecked_into();

    assert_eq!(&div.id(), "id-here");

    assert!(div.class_list().contains("two"));;
    assert!(div.class_list().contains("classes"));;

    assert_eq!(div.class_list().length(), 2);
}
