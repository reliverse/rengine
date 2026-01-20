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
#[doc = "[`ToGodot`] and [`FromGodot`] are implemented for `*mut PhysicsServer3DExtensionShapeRestInfo` and `*const PhysicsServer3DExtensionShapeRestInfo`."]
#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct PhysicsServer3DExtensionShapeRestInfo {
    pub point: Vector3, pub normal: Vector3, pub rid: Rid, pub collider_id: ObjectId, pub shape: i32, pub linear_velocity: Vector3,
}
impl PhysicsServer3DExtensionShapeRestInfo {
    
}
impl GodotConvert for * mut PhysicsServer3DExtensionShapeRestInfo {
    type Via = i64;
    
}
impl ToGodot for * mut PhysicsServer3DExtensionShapeRestInfo {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * mut PhysicsServer3DExtensionShapeRestInfo {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}
impl GodotConvert for * const PhysicsServer3DExtensionShapeRestInfo {
    type Via = i64;
    
}
impl ToGodot for * const PhysicsServer3DExtensionShapeRestInfo {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * const PhysicsServer3DExtensionShapeRestInfo {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}