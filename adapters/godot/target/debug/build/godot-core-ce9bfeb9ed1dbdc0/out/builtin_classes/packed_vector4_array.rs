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
pub struct InnerPackedVector4Array < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPackedVector4Array < 'a > {
    pub fn from_outer(outer: &PackedVector4Array) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn get(&self, index: i64,) -> Vector4 {
        type CallRet = Vector4;
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(975usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "get", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, index: i64, value: Vector4,) {
        type CallRet = ();
        type CallParams = (i64, Vector4,);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(976usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "set", self.sys_ptr, args)
        }
    }
    pub fn size(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(977usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(978usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: Vector4,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector4,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(979usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "push_back", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: Vector4,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector4,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(980usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: &PackedVector4Array,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, PackedVector4Array >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(981usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "append_array", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, index: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(982usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "remove_at", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, at_index: i64, value: Vector4,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, Vector4,);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(983usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "insert", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: Vector4,) {
        type CallRet = ();
        type CallParams = (Vector4,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(984usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "fill", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(985usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "resize", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(986usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "clear", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: Vector4,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector4,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(987usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "has", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(988usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "reverse", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64,) -> PackedVector4Array {
        type CallRet = PackedVector4Array;
        type CallParams = (i64, i64,);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(989usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "slice", self.sys_ptr, args)
        }
    }
    pub fn to_byte_array(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(990usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "to_byte_array", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(991usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "sort", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&mut self, value: Vector4, before: bool,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector4, bool,);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(992usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "bsearch", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&mut self,) -> PackedVector4Array {
        type CallRet = PackedVector4Array;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(993usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "duplicate", self.sys_ptr, args)
        }
    }
    pub fn find(&self, value: Vector4, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector4, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(994usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, value: Vector4, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector4, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(995usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: Vector4,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector4,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(996usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "count", self.sys_ptr, args)
        }
    }
    pub fn erase(&mut self, value: Vector4,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector4,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(997usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedVector4Array", "erase", self.sys_ptr, args)
        }
    }
}
impl PackedVector4Array {
    
}