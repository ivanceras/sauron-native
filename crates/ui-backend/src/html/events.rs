
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
 on_auxclick =>  auxclick;
 on_click   => click;
 on_contextmenu =>  contextmenu;
 on_dblclick   => dblclick;
 on_mousedown =>  mousedown;
 on_mouseenter =>  mouseenter;
 on_mouseleave =>  mouseleave;
 on_mousemove =>  mousemove;
 on_mouseover =>  mouseover;
 on_mouseout =>  mouseout;
 on_mouseup =>  mouseup;
 on_pointerlockchange =>  pointerlockchange;
 on_pointerlockerror =>  pointerlockerror;
 on_select =>  select;
 on_wheel =>  wheel;
}
