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
pub struct InnerBasis < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerBasis < 'a > {
    pub fn from_outer(outer: &Basis) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn inverse(&self,) -> Basis {
        type CallRet = Basis;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(398usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "inverse", self.sys_ptr, args)
        }
    }
    pub fn transposed(&self,) -> Basis {
        type CallRet = Basis;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(399usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "transposed", self.sys_ptr, args)
        }
    }
    pub fn orthonormalized(&self,) -> Basis {
        type CallRet = Basis;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(400usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "orthonormalized", self.sys_ptr, args)
        }
    }
    pub fn determinant(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(401usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "determinant", self.sys_ptr, args)
        }
    }
    pub fn rotated(&self, axis: Vector3, angle: f64,) -> Basis {
        type CallRet = Basis;
        type CallParams = (Vector3, f64,);
        let args = (axis, angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(402usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "rotated", self.sys_ptr, args)
        }
    }
    pub fn scaled(&self, scale: Vector3,) -> Basis {
        type CallRet = Basis;
        type CallParams = (Vector3,);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(403usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "scaled", self.sys_ptr, args)
        }
    }
    pub fn scaled_local(&self, scale: Vector3,) -> Basis {
        type CallRet = Basis;
        type CallParams = (Vector3,);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(404usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "scaled_local", self.sys_ptr, args)
        }
    }
    pub fn get_scale(&self,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(405usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "get_scale", self.sys_ptr, args)
        }
    }
    pub fn get_euler(&self, order: i64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (i64,);
        let args = (order,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(406usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "get_euler", self.sys_ptr, args)
        }
    }
    pub fn tdotx(&self, with: Vector3,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector3,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(407usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "tdotx", self.sys_ptr, args)
        }
    }
    pub fn tdoty(&self, with: Vector3,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector3,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(408usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "tdoty", self.sys_ptr, args)
        }
    }
    pub fn tdotz(&self, with: Vector3,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector3,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(409usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "tdotz", self.sys_ptr, args)
        }
    }
    pub fn slerp(&self, to: Basis, weight: f64,) -> Basis {
        type CallRet = Basis;
        type CallParams = (Basis, f64,);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(410usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "slerp", self.sys_ptr, args)
        }
    }
    pub fn is_conformal(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(411usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "is_conformal", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, b: Basis,) -> bool {
        type CallRet = bool;
        type CallParams = (Basis,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(412usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(413usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "is_finite", self.sys_ptr, args)
        }
    }
    pub fn get_rotation_quaternion(&self,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(414usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "get_rotation_quaternion", self.sys_ptr, args)
        }
    }
    pub fn looking_at(target: Vector3, up: Vector3, use_model_front: bool,) -> Basis {
        type CallRet = Basis;
        type CallParams = (Vector3, Vector3, bool,);
        let args = (target, up, use_model_front,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(415usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "looking_at", std::ptr::null_mut(), args)
        }
    }
    pub fn from_scale(scale: Vector3,) -> Basis {
        type CallRet = Basis;
        type CallParams = (Vector3,);
        let args = (scale,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(416usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "from_scale", std::ptr::null_mut(), args)
        }
    }
    pub fn from_euler(euler: Vector3, order: i64,) -> Basis {
        type CallRet = Basis;
        type CallParams = (Vector3, i64,);
        let args = (euler, order,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(417usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Basis", "from_euler", std::ptr::null_mut(), args)
        }
    }
}
impl Basis {
    
}