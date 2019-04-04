//! https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
//!
use virtual_dom::builder::attr;
use virtual_dom::builder::Attribute;
use virtual_dom::Value;

macro_rules! builder_attributes {
    ( $(
         $(#[$attr:meta])*
         $name:ident;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            pub fn $name<'a, V>(v: V) -> Attribute<'a>
                where V: Into<Value>
                {
                    attr(stringify!($name), v)
                }
         )*
    };
    ( $(
         $(#[$attr:meta])*
         $name:ident => $attribute:tt;
       )*
     ) => {
        $(
            $(#[$attr])*
            #[inline]
            pub fn $name<'a, V>(v: V) -> Attribute<'a>
                where V: Into<Value>
                {
                    attr($attribute, v)
                }
         )*
    }
}

builder_attributes! {
    accesskey;

    autocapitalize;

    class;

    contextmenu;

    draggable;

    dropzone;

    hidden;

    id;

    inputmode;

    is;

    itemid;

    itemprop;

    itemref;

    itemscope;

    itemtype;

    lang;

    slot;

    spellcheck;

    style;

    tabindex;

    title;

    translate;


}

// special case for type attribute, since type is a rust keyword
builder_attributes!{
    r#type => "type";
}
