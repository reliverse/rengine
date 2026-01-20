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
#[doc = "[`ToGodot`] and [`FromGodot`] are implemented for `*mut PhysicsServer3DExtensionMotionResult` and `*const PhysicsServer3DExtensionMotionResult`."]
#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct PhysicsServer3DExtensionMotionResult {
    pub travel: Vector3, pub remainder: Vector3, pub collision_depth: real, pub collision_safe_fraction: real, pub collision_unsafe_fraction: real, pub collisions: [PhysicsServer3DExtensionMotionCollision;
    32usize], pub collision_count: i32,
}
impl PhysicsServer3DExtensionMotionResult {
    
}
impl GodotConvert for * mut PhysicsServer3DExtensionMotionResult {
    type Via = i64;
    
}
impl ToGodot for * mut PhysicsServer3DExtensionMotionResult {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * mut PhysicsServer3DExtensionMotionResult {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}
impl GodotConvert for * const PhysicsServer3DExtensionMotionResult {
    type Via = i64;
    
}
impl ToGodot for * const PhysicsServer3DExtensionMotionResult {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * const PhysicsServer3DExtensionMotionResult {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}