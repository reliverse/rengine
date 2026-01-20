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
#[doc = "[`ToGodot`] and [`FromGodot`] are implemented for `*mut PhysicsServer2DExtensionShapeRestInfo` and `*const PhysicsServer2DExtensionShapeRestInfo`."]
#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct PhysicsServer2DExtensionShapeRestInfo {
    pub point: Vector2, pub normal: Vector2, pub rid: Rid, pub collider_id: ObjectId, pub shape: i32, pub linear_velocity: Vector2,
}
impl PhysicsServer2DExtensionShapeRestInfo {
    
}
impl GodotConvert for * mut PhysicsServer2DExtensionShapeRestInfo {
    type Via = i64;
    
}
impl ToGodot for * mut PhysicsServer2DExtensionShapeRestInfo {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * mut PhysicsServer2DExtensionShapeRestInfo {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}
impl GodotConvert for * const PhysicsServer2DExtensionShapeRestInfo {
    type Via = i64;
    
}
impl ToGodot for * const PhysicsServer2DExtensionShapeRestInfo {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * const PhysicsServer2DExtensionShapeRestInfo {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}