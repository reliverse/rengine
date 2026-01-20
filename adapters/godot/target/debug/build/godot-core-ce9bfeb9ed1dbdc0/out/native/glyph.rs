use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, ClassId, CowArg, InParamTuple, OutParamTuple, ParamTuple, RefArg, Signature
};
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
use std::ffi::c_void;
use crate::meta::{
    GodotConvert, FromGodot, ToGodot
};
#[doc = r" Native structure; can be passed via pointer in APIs that are not exposed to GDScript."]
#[doc = r""]
#[doc = "[`ToGodot`] and [`FromGodot`] are implemented for `*mut Glyph` and `*const Glyph`."]
#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Glyph {
    pub start: i32, pub end: i32, pub count: u8, pub repeat: u8, pub flags: u16, pub x_off: f32, pub y_off: f32, pub advance: f32, pub font_rid: Rid, pub font_size: i32, pub index: i32, pub span_index: i32,
}
impl Glyph {
    
}
impl GodotConvert for * mut Glyph {
    type Via = i64;
    
}
impl ToGodot for * mut Glyph {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * mut Glyph {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}
impl GodotConvert for * const Glyph {
    type Via = i64;
    
}
impl ToGodot for * const Glyph {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * const Glyph {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}