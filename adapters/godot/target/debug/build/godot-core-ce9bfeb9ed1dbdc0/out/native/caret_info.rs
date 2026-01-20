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
#[doc = "[`ToGodot`] and [`FromGodot`] are implemented for `*mut CaretInfo` and `*const CaretInfo`."]
#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct CaretInfo {
    pub leading_caret: Rect2, pub trailing_caret: Rect2, pub leading_direction: crate::classes::text_server::Direction, pub trailing_direction: crate::classes::text_server::Direction,
}
impl CaretInfo {
    
}
impl GodotConvert for * mut CaretInfo {
    type Via = i64;
    
}
impl ToGodot for * mut CaretInfo {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * mut CaretInfo {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}
impl GodotConvert for * const CaretInfo {
    type Via = i64;
    
}
impl ToGodot for * const CaretInfo {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * const CaretInfo {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}