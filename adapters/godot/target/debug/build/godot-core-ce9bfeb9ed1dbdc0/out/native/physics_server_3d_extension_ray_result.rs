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
#[doc = "[`ToGodot`] and [`FromGodot`] are implemented for `*mut PhysicsServer3DExtensionRayResult` and `*const PhysicsServer3DExtensionRayResult`."]
#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct PhysicsServer3DExtensionRayResult {
    pub position: Vector3, pub normal: Vector3, pub rid: Rid, pub collider_id: ObjectId, pub raw_collider_ptr: * mut c_void, pub shape: i32, pub face_index: i32,
}
impl PhysicsServer3DExtensionRayResult {
    #[doc = r" Returns the object as a `Gd<Node>`, or `None` if it no longer exists."]
    pub fn get_collider(&self) -> Option < Gd < Object >> {
        crate::obj::InstanceId::try_from_u64(self.collider_id.id) . and_then(| id | Gd::try_from_instance_id(id) . ok())
    }
    #[doc = r" Sets the object from a `Gd` pointer holding `Node` or a derived class."]
    pub fn set_collider < T > (&mut self, collider: Gd < T >) where T: crate::obj::Inherits < Object > {
        use crate::meta::GodotType as _;
        let obj = collider.upcast();
        #[cfg(safeguards_balanced)]
        #[cfg_attr(published_docs, doc(cfg(safeguards_balanced)))]
        assert !(obj.is_instance_valid(), "provided node is dead");
        let id = obj.instance_id() . to_u64();
        self.collider_id = ObjectId {
            id
        };
        self.raw_collider_ptr = obj.obj_sys() as * mut std::ffi::c_void;
        
    }
}
impl GodotConvert for * mut PhysicsServer3DExtensionRayResult {
    type Via = i64;
    
}
impl ToGodot for * mut PhysicsServer3DExtensionRayResult {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * mut PhysicsServer3DExtensionRayResult {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}
impl GodotConvert for * const PhysicsServer3DExtensionRayResult {
    type Via = i64;
    
}
impl ToGodot for * const PhysicsServer3DExtensionRayResult {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        * self as i64
    }
}
impl FromGodot for * const PhysicsServer3DExtensionRayResult {
    fn try_from_godot(via: Self::Via) -> Result < Self, crate::meta::error::ConvertError > {
        Ok(via as Self)
    }
}