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
#[doc = "[`ToGodot`] and [`FromGodot`] are implemented for `*mut PhysicsServer3DExtensionMotionCollision` and `*const PhysicsServer3DExtensionMotionCollision`."]
#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct PhysicsServer3DExtensionMotionCollision {
    pub position: Vector3, pub normal: Vector3, pub collider_velocity: Vector3, pub collider_angular_velocity: Vector3, pub depth: real, pub local_shape: i32, pub collider_id: ObjectId, pub collider: Rid, pub collider_shape: i32,
}
impl PhysicsServer3DExtensionMotionCollision {
    
}
impl GodotConvert for * mut PhysicsServer3DExtensionMotionCollision {
    type Via = i64;
    
}
impl ToGodot for * mut PhysicsServer3DExtensionMotionCollision {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * mut PhysicsServer3DExtensionMotionCollision {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}
impl GodotConvert for * const PhysicsServer3DExtensionMotionCollision {
    type Via = i64;
    
}
impl ToGodot for * const PhysicsServer3DExtensionMotionCollision {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * const PhysicsServer3DExtensionMotionCollision {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}