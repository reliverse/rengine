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
pub struct InnerTransform2D < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerTransform2D < 'a > {
    pub fn from_outer(outer: &Transform2D) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn inverse(&self,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(274usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "inverse", self.sys_ptr, args)
        }
    }
    pub fn affine_inverse(&self,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(275usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "affine_inverse", self.sys_ptr, args)
        }
    }
    pub fn get_rotation(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(276usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "get_rotation", self.sys_ptr, args)
        }
    }
    pub fn get_origin(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(277usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "get_origin", self.sys_ptr, args)
        }
    }
    pub fn get_scale(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(278usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "get_scale", self.sys_ptr, args)
        }
    }
    pub fn get_skew(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(279usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "get_skew", self.sys_ptr, args)
        }
    }
    pub fn orthonormalized(&self,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(280usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "orthonormalized", self.sys_ptr, args)
        }
    }
    pub fn rotated(&self, angle: f64,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = (f64,);
        let args = (angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(281usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "rotated", self.sys_ptr, args)
        }
    }
    pub fn rotated_local(&self, angle: f64,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = (f64,);
        let args = (angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(282usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "rotated_local", self.sys_ptr, args)
        }
    }
    pub fn scaled(&self, scale: Vector2,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = (Vector2,);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(283usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "scaled", self.sys_ptr, args)
        }
    }
    pub fn scaled_local(&self, scale: Vector2,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = (Vector2,);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(284usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "scaled_local", self.sys_ptr, args)
        }
    }
    pub fn translated(&self, offset: Vector2,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = (Vector2,);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(285usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "translated", self.sys_ptr, args)
        }
    }
    pub fn translated_local(&self, offset: Vector2,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = (Vector2,);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(286usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "translated_local", self.sys_ptr, args)
        }
    }
    pub fn determinant(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(287usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "determinant", self.sys_ptr, args)
        }
    }
    pub fn basis_xform(&self, v: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (v,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(288usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "basis_xform", self.sys_ptr, args)
        }
    }
    pub fn basis_xform_inv(&self, v: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (v,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(289usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "basis_xform_inv", self.sys_ptr, args)
        }
    }
    pub fn interpolate_with(&self, xform: Transform2D, weight: f64,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = (Transform2D, f64,);
        let args = (xform, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(290usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "interpolate_with", self.sys_ptr, args)
        }
    }
    pub fn is_conformal(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(291usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "is_conformal", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, xform: Transform2D,) -> bool {
        type CallRet = bool;
        type CallParams = (Transform2D,);
        let args = (xform,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(292usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(293usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "is_finite", self.sys_ptr, args)
        }
    }
    pub fn looking_at(&self, target: Vector2,) -> Transform2D {
        type CallRet = Transform2D;
        type CallParams = (Vector2,);
        let args = (target,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(294usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Transform2D", "looking_at", self.sys_ptr, args)
        }
    }
}
impl Transform2D {
    
}