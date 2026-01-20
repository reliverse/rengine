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
pub struct InnerRect2 < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerRect2 < 'a > {
    pub fn from_outer(outer: &Rect2) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn get_center(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(181usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "get_center", self.sys_ptr, args)
        }
    }
    pub fn get_area(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(182usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "get_area", self.sys_ptr, args)
        }
    }
    pub fn has_area(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(183usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "has_area", self.sys_ptr, args)
        }
    }
    pub fn has_point(&self, point: Vector2,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector2,);
        let args = (point,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(184usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "has_point", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, rect: Rect2,) -> bool {
        type CallRet = bool;
        type CallParams = (Rect2,);
        let args = (rect,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(185usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(186usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "is_finite", self.sys_ptr, args)
        }
    }
    pub fn intersects(&self, b: Rect2, include_borders: bool,) -> bool {
        type CallRet = bool;
        type CallParams = (Rect2, bool,);
        let args = (b, include_borders,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(187usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "intersects", self.sys_ptr, args)
        }
    }
    pub fn encloses(&self, b: Rect2,) -> bool {
        type CallRet = bool;
        type CallParams = (Rect2,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(188usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "encloses", self.sys_ptr, args)
        }
    }
    pub fn intersection(&self, b: Rect2,) -> Rect2 {
        type CallRet = Rect2;
        type CallParams = (Rect2,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(189usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "intersection", self.sys_ptr, args)
        }
    }
    pub fn merge(&self, b: Rect2,) -> Rect2 {
        type CallRet = Rect2;
        type CallParams = (Rect2,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(190usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "merge", self.sys_ptr, args)
        }
    }
    pub fn expand(&self, to: Vector2,) -> Rect2 {
        type CallRet = Rect2;
        type CallParams = (Vector2,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(191usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "expand", self.sys_ptr, args)
        }
    }
    pub fn get_support(&self, direction: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (direction,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(192usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "get_support", self.sys_ptr, args)
        }
    }
    pub fn grow(&self, amount: f64,) -> Rect2 {
        type CallRet = Rect2;
        type CallParams = (f64,);
        let args = (amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(193usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "grow", self.sys_ptr, args)
        }
    }
    pub fn grow_side(&self, side: i64, amount: f64,) -> Rect2 {
        type CallRet = Rect2;
        type CallParams = (i64, f64,);
        let args = (side, amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(194usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "grow_side", self.sys_ptr, args)
        }
    }
    pub fn grow_individual(&self, left: f64, top: f64, right: f64, bottom: f64,) -> Rect2 {
        type CallRet = Rect2;
        type CallParams = (f64, f64, f64, f64,);
        let args = (left, top, right, bottom,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(195usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "grow_individual", self.sys_ptr, args)
        }
    }
    pub fn abs(&self,) -> Rect2 {
        type CallRet = Rect2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(196usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rect2", "abs", self.sys_ptr, args)
        }
    }
}
impl Rect2 {
    
}