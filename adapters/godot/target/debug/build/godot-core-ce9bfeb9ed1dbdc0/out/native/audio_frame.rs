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
#[doc = "[`ToGodot`] and [`FromGodot`] are implemented for `*mut AudioFrame` and `*const AudioFrame`."]
#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct AudioFrame {
    pub left: f32, pub right: f32,
}
impl AudioFrame {
    
}
impl GodotConvert for * mut AudioFrame {
    type Via = i64;
    
}
impl ToGodot for * mut AudioFrame {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * mut AudioFrame {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}
impl GodotConvert for * const AudioFrame {
    type Via = i64;
    
}
impl ToGodot for * const AudioFrame {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * const AudioFrame {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}