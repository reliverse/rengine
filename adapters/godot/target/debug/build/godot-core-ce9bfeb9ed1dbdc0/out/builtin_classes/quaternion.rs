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
pub struct InnerQuaternion < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerQuaternion < 'a > {
    pub fn from_outer(outer: &Quaternion) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn length(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(354usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "length", self.sys_ptr, args)
        }
    }
    pub fn length_squared(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(355usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "length_squared", self.sys_ptr, args)
        }
    }
    pub fn normalized(&self,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(356usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "normalized", self.sys_ptr, args)
        }
    }
    pub fn is_normalized(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(357usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "is_normalized", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, to: Quaternion,) -> bool {
        type CallRet = bool;
        type CallParams = (Quaternion,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(358usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(359usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "is_finite", self.sys_ptr, args)
        }
    }
    pub fn inverse(&self,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(360usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "inverse", self.sys_ptr, args)
        }
    }
    pub fn log(&self,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(361usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "log", self.sys_ptr, args)
        }
    }
    pub fn exp(&self,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(362usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "exp", self.sys_ptr, args)
        }
    }
    pub fn angle_to(&self, to: Quaternion,) -> f64 {
        type CallRet = f64;
        type CallParams = (Quaternion,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(363usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "angle_to", self.sys_ptr, args)
        }
    }
    pub fn dot(&self, with: Quaternion,) -> f64 {
        type CallRet = f64;
        type CallParams = (Quaternion,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(364usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "dot", self.sys_ptr, args)
        }
    }
    pub fn slerp(&self, to: Quaternion, weight: f64,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = (Quaternion, f64,);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(365usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "slerp", self.sys_ptr, args)
        }
    }
    pub fn slerpni(&self, to: Quaternion, weight: f64,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = (Quaternion, f64,);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(366usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "slerpni", self.sys_ptr, args)
        }
    }
    pub fn spherical_cubic_interpolate(&self, b: Quaternion, pre_a: Quaternion, post_b: Quaternion, weight: f64,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = (Quaternion, Quaternion, Quaternion, f64,);
        let args = (b, pre_a, post_b, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(367usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "spherical_cubic_interpolate", self.sys_ptr, args)
        }
    }
    pub fn spherical_cubic_interpolate_in_time(&self, b: Quaternion, pre_a: Quaternion, post_b: Quaternion, weight: f64, b_t: f64, pre_a_t: f64, post_b_t: f64,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = (Quaternion, Quaternion, Quaternion, f64, f64, f64, f64,);
        let args = (b, pre_a, post_b, weight, b_t, pre_a_t, post_b_t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(368usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "spherical_cubic_interpolate_in_time", self.sys_ptr, args)
        }
    }
    pub fn get_euler(&self, order: i64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (i64,);
        let args = (order,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(369usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "get_euler", self.sys_ptr, args)
        }
    }
    pub fn from_euler(euler: Vector3,) -> Quaternion {
        type CallRet = Quaternion;
        type CallParams = (Vector3,);
        let args = (euler,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(370usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "from_euler", std::ptr::null_mut(), args)
        }
    }
    pub fn get_axis(&self,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(371usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "get_axis", self.sys_ptr, args)
        }
    }
    pub fn get_angle(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(372usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Quaternion", "get_angle", self.sys_ptr, args)
        }
    }
}
impl Quaternion {
    
}