use std::any::Any;

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
