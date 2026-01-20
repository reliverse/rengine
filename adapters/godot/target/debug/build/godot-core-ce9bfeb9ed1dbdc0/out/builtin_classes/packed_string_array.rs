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
pub struct InnerPackedStringArray < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPackedStringArray < 'a > {
    pub fn from_outer(outer: &PackedStringArray) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn get(&self, index: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(883usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "get", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, index: i64, value: impl AsArg < GString >,) {
        type CallRet = ();
        type CallParams < 'a0, > = (i64, CowArg < 'a0, GString >,);
        let args = (index, value.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(884usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "set", self.sys_ptr, args)
        }
    }
    pub fn size(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(885usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(886usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (value.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(887usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "push_back", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (value.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(888usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: &PackedStringArray,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(889usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "append_array", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, index: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(890usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "remove_at", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, at_index: i64, value: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (i64, CowArg < 'a0, GString >,);
        let args = (at_index, value.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(891usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "insert", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: impl AsArg < GString >,) {
        type CallRet = ();
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (value.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(892usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "fill", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(893usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "resize", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(894usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "clear", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (value.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(895usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "has", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(896usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "reverse", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64,) -> PackedStringArray {
        type CallRet = PackedStringArray;
        type CallParams = (i64, i64,);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(897usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "slice", self.sys_ptr, args)
        }
    }
    pub fn to_byte_array(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(898usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "to_byte_array", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(899usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "sort", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&mut self, value: impl AsArg < GString >, before: bool,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
        let args = (value.into_arg(), before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(900usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "bsearch", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&mut self,) -> PackedStringArray {
        type CallRet = PackedStringArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(901usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "duplicate", self.sys_ptr, args)
        }
    }
    pub fn find(&self, value: impl AsArg < GString >, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (value.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(902usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, value: impl AsArg < GString >, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (value.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(903usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (value.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(904usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "count", self.sys_ptr, args)
        }
    }
    pub fn erase(&mut self, value: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (value.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(905usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedStringArray", "erase", self.sys_ptr, args)
        }
    }
}
impl PackedStringArray {
    
}