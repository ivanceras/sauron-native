use std::any::Any;
use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<String, &Any> = HashMap::new();
    hm.insert("id".into(), &"some-id");
    hm.insert("item".into(), &12);

    println!("playing with any {:#?}", hm);
}
