use virtual_dom::diff;
use virtual_dom::{Callback, Element, Node, Text, Value};

use std::any::Any;
use std::collections::BTreeMap;

fn main() {
    let txt = Node::Text(Text {
        text: "World!".into(),
    });
    let v1 = "somev1";
    let v2 = 11;

    let x = 3u8;
    let s = 1;

    let div = Node::Element(Element {
        tag: "div".into(),
        attrs: {
            let mut hm: BTreeMap<String, Value> = BTreeMap::new();
            hm.insert("v1".into(), "somev1".into());
            hm.insert("v1".into(), v1.into());
            hm.insert("v2".into(), v2.into());
            hm.insert("id".into(), 0.into());
            hm
        },
        events: {
            let mut hm: BTreeMap<String, Callback<Value>> = BTreeMap::new();
            let hello = |x: Value| {
                print!("hello  {}", x);
            };
            let hi = |s: Value| {
                println!("hi: {:#?} ", s);
            };
            let hic: Callback<Value> = hi.into();
            hic.emit(s);
            hic.emit(s);
            hic.emit(x);
            let helloc: Callback<Value> = hello.into();
            helloc.emit(x);
            hm.insert("click".into(), hello.into());
            hm.insert("mousedown".into(), hi.into());
            hm
        },
        children: vec![Node::Text(Text {
            text: "Hello".into(),
        })],
        namespace: None,
    });

    println!("{}", div);
    let diff = diff::diff(&txt, &div);
    println!("diff: {:#?}", diff);
}
