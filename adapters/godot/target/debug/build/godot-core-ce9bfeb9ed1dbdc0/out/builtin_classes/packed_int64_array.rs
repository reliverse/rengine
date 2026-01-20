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
pub struct InnerPackedInt64Array < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPackedInt64Array < 'a > {
    pub fn from_outer(outer: &PackedInt64Array) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn get(&self, index: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(814usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "get", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, index: i64, value: i64,) {
        type CallRet = ();
        type CallParams = (i64, i64,);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(815usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "set", self.sys_ptr, args)
        }
    }
    pub fn size(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(816usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(817usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(818usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "push_back", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(819usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: &PackedInt64Array,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, PackedInt64Array >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(820usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "append_array", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, index: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(821usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "remove_at", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, at_index: i64, value: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, i64,);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(822usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "insert", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(823usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "fill", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(824usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "resize", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(825usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "clear", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(826usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "has", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(827usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "reverse", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64,) -> PackedInt64Array {
        type CallRet = PackedInt64Array;
        type CallParams = (i64, i64,);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(828usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "slice", self.sys_ptr, args)
        }
    }
    pub fn to_byte_array(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(829usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "to_byte_array", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(830usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "sort", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&mut self, value: i64, before: bool,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, bool,);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(831usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "bsearch", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&mut self,) -> PackedInt64Array {
        type CallRet = PackedInt64Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(832usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "duplicate", self.sys_ptr, args)
        }
    }
    pub fn find(&self, value: i64, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(833usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, value: i64, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(834usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(835usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "count", self.sys_ptr, args)
        }
    }
    pub fn erase(&mut self, value: i64,) -> bool {
        type CallRet = bool;
        type CallParams = (i64,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(836usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedInt64Array", "erase", self.sys_ptr, args)
        }
    }
}
impl PackedInt64Array {
    
}