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
pub struct InnerPackedVector2Array < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPackedVector2Array < 'a > {
    pub fn from_outer(outer: &PackedVector2Array) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn get(&self, index: i64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(906usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "get", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, index: i64, value: Vector2,) {
        type CallRet = ();
        type CallParams = (i64, Vector2,);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(907usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "set", self.sys_ptr, args)
        }
    }
    pub fn size(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(908usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(909usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: Vector2,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector2,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(910usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "push_back", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: Vector2,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector2,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(911usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: &PackedVector2Array,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(912usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "append_array", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, index: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(913usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "remove_at", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, at_index: i64, value: Vector2,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, Vector2,);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(914usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "insert", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: Vector2,) {
        type CallRet = ();
        type CallParams = (Vector2,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(915usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "fill", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(916usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "resize", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(917usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "clear", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: Vector2,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector2,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(918usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "has", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(919usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "reverse", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64,) -> PackedVector2Array {
        type CallRet = PackedVector2Array;
        type CallParams = (i64, i64,);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(920usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "slice", self.sys_ptr, args)
        }
    }
    pub fn to_byte_array(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(921usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "to_byte_array", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(922usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "sort", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&mut self, value: Vector2, before: bool,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector2, bool,);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(923usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "bsearch", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&mut self,) -> PackedVector2Array {
        type CallRet = PackedVector2Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(924usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "duplicate", self.sys_ptr, args)
        }
    }
    pub fn find(&self, value: Vector2, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector2, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(925usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, value: Vector2, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector2, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(926usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: Vector2,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector2,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(927usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "count", self.sys_ptr, args)
        }
    }
    pub fn erase(&mut self, value: Vector2,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector2,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(928usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector2Array", "erase", self.sys_ptr, args)
        }
    }
}
impl PackedVector2Array {
    
}