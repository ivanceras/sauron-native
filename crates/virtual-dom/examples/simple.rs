use virtual_dom::{Callback, VElement, VNode, VText, Value};

use std::any::Any;
use std::collections::HashMap;

fn main() {
    let txt = VNode::Text(VText {
        text: "World!".into(),
    });
    let v1 = "somev1";
    let v2 = 11;

    let x = 3u8;
    let s = 1;

    let div = VNode::Element(VElement {
        tag: "div".into(),
        attrs: {
            let mut hm: HashMap<String, Value> = HashMap::new();
            hm.insert("v1".into(), "somev1".into());
            hm.insert("v1".into(), v1.into());
            hm.insert("v2".into(), v2.into());
            hm.insert("id".into(), 0.into());
            hm
        },
        events: {
            let mut hm:HashMap<String, Callback<Value>> = HashMap::new();
            let hello = |x: Value| {
                print!("hello  {}",x);
            };
            let hi = |s: Value| {
                println!("hi: {:#?} ", s);
            };
            let hic: Callback<Value> = hi.into();
            hic.emit(s);
            hic.emit(s);
            hic.emit(x);
            let helloc:Callback<Value> = hello.into();
            helloc.emit(x);
            hm.insert("click".into(), hello.into());
            hm.insert("mousedown".into(), hi.into());
            hm
        },
        children: vec![
            VNode::Text(VText{text:"Hello".into()}),
            txt
        ],
    });

    println!("{}", div);
}
