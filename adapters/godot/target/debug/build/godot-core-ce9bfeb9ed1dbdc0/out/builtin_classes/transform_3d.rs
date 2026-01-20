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
pub struct InnerTransform3D < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerTransform3D < 'a > {
    pub fn from_outer(outer: &Transform3D) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn inverse(&self,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(418usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "inverse", self.sys_ptr, args)
        }
    }
    pub fn affine_inverse(&self,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(419usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "affine_inverse", self.sys_ptr, args)
        }
    }
    pub fn orthonormalized(&self,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(420usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "orthonormalized", self.sys_ptr, args)
        }
    }
    pub fn rotated(&self, axis: Vector3, angle: f64,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = (Vector3, f64,);
        let args = (axis, angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(421usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "rotated", self.sys_ptr, args)
        }
    }
    pub fn rotated_local(&self, axis: Vector3, angle: f64,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = (Vector3, f64,);
        let args = (axis, angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(422usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "rotated_local", self.sys_ptr, args)
        }
    }
    pub fn scaled(&self, scale: Vector3,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = (Vector3,);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(423usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "scaled", self.sys_ptr, args)
        }
    }
    pub fn scaled_local(&self, scale: Vector3,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = (Vector3,);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(424usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "scaled_local", self.sys_ptr, args)
        }
    }
    pub fn translated(&self, offset: Vector3,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = (Vector3,);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(425usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "translated", self.sys_ptr, args)
        }
    }
    pub fn translated_local(&self, offset: Vector3,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = (Vector3,);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(426usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "translated_local", self.sys_ptr, args)
        }
    }
    pub fn looking_at(&self, target: Vector3, up: Vector3, use_model_front: bool,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = (Vector3, Vector3, bool,);
        let args = (target, up, use_model_front,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(427usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "looking_at", self.sys_ptr, args)
        }
    }
    pub fn interpolate_with(&self, xform: Transform3D, weight: f64,) -> Transform3D {
        type CallRet = Transform3D;
        type CallParams = (Transform3D, f64,);
        let args = (xform, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(428usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "interpolate_with", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, xform: Transform3D,) -> bool {
        type CallRet = bool;
        type CallParams = (Transform3D,);
        let args = (xform,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(429usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(430usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform3D", "is_finite", self.sys_ptr, args)
        }
    }
}
impl Transform3D {
    
}