use std::any::Any;
use std::collections::HashMap;


pub(crate) fn is_equal<'a>(a: &'a Any, b: &'a Any) -> bool {
    if a.is::<String>(){
        a.downcast_ref::<String>() == b.downcast_ref::<String>()
    }
    else if a.is::<u8>(){
        a.downcast_ref::<u8>() == b.downcast_ref::<u8>()
    }
    else{
        false
    }
}

pub(crate) fn any_to_string(a: &Any)->String{
    if a.is::<String>(){
        a.downcast_ref::<String>().unwrap().to_string()
    }
    else if a.is::<&str>(){
        a.downcast_ref::<&str>().unwrap().to_string()
    }
    else if a.is::<u8>(){
        a.downcast_ref::<u8>().unwrap().to_string()
    }
    else if a.is::<i8>(){
        a.downcast_ref::<i8>().unwrap().to_string()
    }
    else if a.is::<u16>(){
        a.downcast_ref::<u16>().unwrap().to_string()
    }
    else if a.is::<i16>(){
        a.downcast_ref::<i16>().unwrap().to_string()
    }
    else if a.is::<u32>(){
        a.downcast_ref::<u32>().unwrap().to_string()
    }
    else if a.is::<i32>(){
        a.downcast_ref::<i32>().unwrap().to_string()
    }
    else if a.is::<u64>(){
        a.downcast_ref::<u64>().unwrap().to_string()
    }
    else if a.is::<i64>(){
        a.downcast_ref::<i64>().unwrap().to_string()
    }
    else if a.is::<u128>(){
        a.downcast_ref::<u128>().unwrap().to_string()
    }
    else if a.is::<i128>(){
        a.downcast_ref::<i128>().unwrap().to_string()
    }
    else{
        "unknown type".to_string()
    }
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn is_equal_strings(){
        let a = "Hello".to_string();
        let b = "Hello".to_string();
        assert!(is_equal(&a, &b));
    }

    #[test]
    fn is_equal_u8(){
        let a = 0u8;
        let b = 0u8;
        assert!(is_equal(&a, &b));
    }

    #[test]
    fn is_equal_maps(){
        let mut m1:HashMap<String, &Any> = HashMap::new();
        m1.insert("a".to_string(), &"Hello");
        m1.insert("b".to_string(), &11);

        let mut m2:HashMap<String, &Any> = HashMap::new();
        m2.insert("a".to_string(), &"Hello");
        m2.insert("b".to_string(), &11);

    }
    
}
