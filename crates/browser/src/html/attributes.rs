//! https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
//!
use vdom::builder::attr;
use vdom::builder::Attribute;
use vdom::Value;

macro_rules! declare_attributes {
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

declare_attributes! {
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
declare_attributes! {
    r#type => "type";
}

// common attributes
declare_attributes! {
    value;
    key;
    placeholder;
}

// svg attributes
declare_attributes! {
    cx;
    cy;
    r;
    x1;
    y1;
    x2;
    y2;
    xmlns;
    offset;
    stroke;
    fill;
    transform;
    transition;
}

// sizing attribtes
declare_attributes! {
    width;
    height;
}

// attributes that has dash
declare_attributes! {
    #[allow(non_snake_case)]
    strokeWidth => "stroke-width";
    #[allow(non_snake_case)]
    stopColor => "stop-color";
    #[allow(non_snake_case)]
    stopOpacity => "stop-opacity";
    #[allow(non_snake_case)]
    strokeLinecap => "stroke-linecap";
    #[allow(non_snake_case)]
    strokeDasharray => "stroke-dasharray";
    #[allow(non_snake_case)]
    strokeDashoffset => "stroke-dashoffset";
    #[allow(non_snake_case)]
    transformOrigin => "transform-origin";
}
