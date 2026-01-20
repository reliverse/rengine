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
pub struct InnerRect2i < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerRect2i < 'a > {
    pub fn from_outer(outer: &Rect2i) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn get_center(&self,) -> Vector2i {
        type CallRet = Vector2i;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(197usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "get_center", self.sys_ptr, args)
        }
    }
    pub fn get_area(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(198usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "get_area", self.sys_ptr, args)
        }
    }
    pub fn has_area(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(199usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "has_area", self.sys_ptr, args)
        }
    }
    pub fn has_point(&self, point: Vector2i,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector2i,);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(200usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "has_point", self.sys_ptr, args)
        }
    }
    pub fn intersects(&self, b: Rect2i,) -> bool {
        type CallRet = bool;
        type CallParams = (Rect2i,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(201usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "intersects", self.sys_ptr, args)
        }
    }
    pub fn encloses(&self, b: Rect2i,) -> bool {
        type CallRet = bool;
        type CallParams = (Rect2i,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(202usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "encloses", self.sys_ptr, args)
        }
    }
    pub fn intersection(&self, b: Rect2i,) -> Rect2i {
        type CallRet = Rect2i;
        type CallParams = (Rect2i,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(203usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "intersection", self.sys_ptr, args)
        }
    }
    pub fn merge(&self, b: Rect2i,) -> Rect2i {
        type CallRet = Rect2i;
        type CallParams = (Rect2i,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(204usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "merge", self.sys_ptr, args)
        }
    }
    pub fn expand(&self, to: Vector2i,) -> Rect2i {
        type CallRet = Rect2i;
        type CallParams = (Vector2i,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(205usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "expand", self.sys_ptr, args)
        }
    }
    pub fn grow(&self, amount: i64,) -> Rect2i {
        type CallRet = Rect2i;
        type CallParams = (i64,);
        let args = (amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(206usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "grow", self.sys_ptr, args)
        }
    }
    pub fn grow_side(&self, side: i64, amount: i64,) -> Rect2i {
        type CallRet = Rect2i;
        type CallParams = (i64, i64,);
        let args = (side, amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(207usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "grow_side", self.sys_ptr, args)
        }
    }
    pub fn grow_individual(&self, left: i64, top: i64, right: i64, bottom: i64,) -> Rect2i {
        type CallRet = Rect2i;
        type CallParams = (i64, i64, i64, i64,);
        let args = (left, top, right, bottom,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(208usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "grow_individual", self.sys_ptr, args)
        }
    }
    pub fn abs(&self,) -> Rect2i {
        type CallRet = Rect2i;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(209usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2i", "abs", self.sys_ptr, args)
        }
    }
}
impl Rect2i {
    
}