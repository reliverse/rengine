use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, ClassId, CowArg, InParamTuple, OutParamTuple, ParamTuple, RefArg, Signature
};
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
#[repr(transparent)]
pub struct InnerPlane < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPlane < 'a > {
    pub fn from_outer(outer: &Plane) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn normalized(&self,) -> Plane {
        type CallRet = Plane;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(343usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Plane", "normalized", self.sys_ptr, args)
        }
    }
    pub fn get_center(&self,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(344usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Plane", "get_center", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, to_plane: Plane,) -> bool {
        type CallRet = bool;
        type CallParams = (Plane,);
        let args = (to_plane,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(345usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Plane", "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(346usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Plane", "is_finite", self.sys_ptr, args)
        }
    }
    pub fn is_point_over(&self, point: Vector3,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector3,);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(347usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Plane", "is_point_over", self.sys_ptr, args)
        }
    }
    pub fn distance_to(&self, point: Vector3,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector3,);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(348usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Plane", "distance_to", self.sys_ptr, args)
        }
    }
    pub fn has_point(&self, point: Vector3, tolerance: f64,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector3, f64,);
        let args = (point, tolerance,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(349usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Plane", "has_point", self.sys_ptr, args)
        }
    }
    pub fn project(&self, point: Vector3,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (Vector3,);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(350usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Plane", "project", self.sys_ptr, args)
        }
    }
    pub fn intersect_3(&self, b: Plane, c: Plane,) -> Variant {
        type CallRet = Variant;
        type CallParams = (Plane, Plane,);
        let args = (b, c,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(351usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Plane", "intersect_3", self.sys_ptr, args)
        }
    }
    pub fn intersects_ray(&self, from: Vector3, dir: Vector3,) -> Variant {
        type CallRet = Variant;
        type CallParams = (Vector3, Vector3,);
        let args = (from, dir,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(352usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Plane", "intersects_ray", self.sys_ptr, args)
        }
    }
    pub fn intersects_segment(&self, from: Vector3, to: Vector3,) -> Variant {
        type CallRet = Variant;
        type CallParams = (Vector3, Vector3,);
        let args = (from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(353usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Plane", "intersects_segment", self.sys_ptr, args)
        }
    }
}
impl Plane {
    
}