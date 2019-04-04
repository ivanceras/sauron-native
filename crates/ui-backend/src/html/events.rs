
use virtual_dom::builder::Attribute;
use virtual_dom::builder::on;
use virtual_dom::{Callback,Value};


macro_rules! builder_events {
    ( $(
         $(#[$attr:meta])*
         $name:ident => $event:ident;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            pub fn $name<'a, F>(f: F) -> Attribute<'a>
                where F: Into<Callback<Value>>
                {
                    on(stringify!($event), f)
                }
         )*
    }
}

// Mouse events
builder_events!{
 onauxclick => auxclick;
 onclick  => click;
 oncontextmenu =>contextmenu;
 ondblclick  => dblclick;
 onmousedown => mousedown;
 onmouseenter => mouseenter;
 onmouseleave => mouseleave;
 onmousemove => mousemove;
 onmouseover => mouseover;
 onmouseout => mouseout;
 onmouseup => mouseup;
 onpointerlockchange => pointerlockchange;
 onpointerlockerror => pointerlockerror;
 onselect => select;
 onwheel => wheel;
}
