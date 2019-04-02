use virtual_dom::{Callback, Events, VElement, VNode, VText};

use std::any::Any;
use std::collections::HashMap;

fn main() {
    let txt = VNode::Text(VText {
        text: "Hello".into(),
    });
    let v1 = "somev1";
    let v2 = 11;

    let x = 3u8;
    let s = (1, "hi");

    let div = VNode::Element(VElement {
        tag: "div".into(),
        attrs: {
            let mut hm: HashMap<String, &Any> = HashMap::new();
            hm.insert("v1".into(), &"somev1");
            hm.insert("v1".into(), &v1);
            hm.insert("v2".into(), &v2);
            hm.insert("id".into(), &0);
            hm
        },
        events: Events({
            let mut hm:HashMap<String, Callback<&Any>> = HashMap::new();
            let hello = |x: &Any| {
                print!("hello  ");
                if x.is::<String>() {
                    println!("a string");
                } else if x.is::<u8>() {
                    print!("u8: {}", x.downcast_ref::<u8>().unwrap());
                } else {
                    print!("cant guess");
                }
                println!();
            };
            let hi = |s: &Any| {
                println!("hi: {:#?} ", s);
                if s.is::<(i32, &str)>() {
                    print!("gotcha type");
                    if let Some((v, s)) = s.downcast_ref::<(i32, &str)>() {
                        println!("({},{})", v, s);
                    }
                } else if s.is::<u8>() {
                    print!("u8: {}", s.downcast_ref::<u8>().unwrap());
                } else {
                    print!("dunno type");
                }
                println!();
            };
            let hic: Callback<&Any> = hi.into();
            hic.emit(&s);
            hic.emit(&x);
            let helloc:Callback<&Any> = hello.into();
            helloc.emit(&x);
            hm.insert("click".into(), hello.into());
            hm.insert("mousedown".into(), hi.into());
            hm
        }),
        children: vec![],
    });

    println!("{}", div);
    println!("{}", txt);
}
