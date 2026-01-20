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
pub struct InnerPackedVector3Array < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPackedVector3Array < 'a > {
    pub fn from_outer(outer: &PackedVector3Array) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn get(&self, index: i64,) -> Vector3 {
        type CallRet = Vector3;
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(929usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "get", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, index: i64, value: Vector3,) {
        type CallRet = ();
        type CallParams = (i64, Vector3,);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(930usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "set", self.sys_ptr, args)
        }
    }
    pub fn size(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(931usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(932usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: Vector3,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector3,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(933usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "push_back", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: Vector3,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector3,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(934usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: &PackedVector3Array,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, PackedVector3Array >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(935usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "append_array", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, index: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(936usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "remove_at", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, at_index: i64, value: Vector3,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, Vector3,);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(937usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "insert", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: Vector3,) {
        type CallRet = ();
        type CallParams = (Vector3,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(938usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "fill", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(939usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "resize", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(940usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "clear", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: Vector3,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector3,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(941usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "has", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(942usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "reverse", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64,) -> PackedVector3Array {
        type CallRet = PackedVector3Array;
        type CallParams = (i64, i64,);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(943usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "slice", self.sys_ptr, args)
        }
    }
    pub fn to_byte_array(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(944usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "to_byte_array", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(945usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "sort", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&mut self, value: Vector3, before: bool,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector3, bool,);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(946usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "bsearch", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&mut self,) -> PackedVector3Array {
        type CallRet = PackedVector3Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(947usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "duplicate", self.sys_ptr, args)
        }
    }
    pub fn find(&self, value: Vector3, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector3, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(948usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, value: Vector3, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector3, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(949usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: Vector3,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector3,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(950usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "count", self.sys_ptr, args)
        }
    }
    pub fn erase(&mut self, value: Vector3,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector3,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(951usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector3Array", "erase", self.sys_ptr, args)
        }
    }
}
impl PackedVector3Array {
    
}