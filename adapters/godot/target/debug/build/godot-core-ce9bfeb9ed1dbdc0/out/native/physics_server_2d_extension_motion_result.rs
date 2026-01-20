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
#[doc = "[`ToGodot`] and [`FromGodot`] are implemented for `*mut PhysicsServer2DExtensionMotionResult` and `*const PhysicsServer2DExtensionMotionResult`."]
#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct PhysicsServer2DExtensionMotionResult {
    pub travel: Vector2, pub remainder: Vector2, pub collision_point: Vector2, pub collision_normal: Vector2, pub collider_velocity: Vector2, pub collision_depth: real, pub collision_safe_fraction: real, pub collision_unsafe_fraction: real, pub collision_local_shape: i32, pub collider_id: ObjectId, pub collider: Rid, pub collider_shape: i32,
}
impl PhysicsServer2DExtensionMotionResult {
    
}
impl GodotConvert for * mut PhysicsServer2DExtensionMotionResult {
    type Via = i64;
    
}
impl ToGodot for * mut PhysicsServer2DExtensionMotionResult {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * mut PhysicsServer2DExtensionMotionResult {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}
impl GodotConvert for * const PhysicsServer2DExtensionMotionResult {
    type Via = i64;
    
}
impl ToGodot for * const PhysicsServer2DExtensionMotionResult {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * const PhysicsServer2DExtensionMotionResult {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}