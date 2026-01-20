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
pub struct InnerPackedColorArray < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerPackedColorArray < 'a > {
    pub fn from_outer(outer: &PackedColorArray) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn get(&self, index: i64,) -> Color {
        type CallRet = Color;
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(952usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "get", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, index: i64, value: Color,) {
        type CallRet = ();
        type CallParams = (i64, Color,);
        let args = (index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(953usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "set", self.sys_ptr, args)
        }
    }
    pub fn size(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(954usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(955usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: Color,) -> bool {
        type CallRet = bool;
        type CallParams = (Color,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(956usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "push_back", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: Color,) -> bool {
        type CallRet = bool;
        type CallParams = (Color,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(957usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: &PackedColorArray,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, PackedColorArray >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(958usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "append_array", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, index: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(959usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "remove_at", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, at_index: i64, value: Color,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64, Color,);
        let args = (at_index, value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(960usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "insert", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: Color,) {
        type CallRet = ();
        type CallParams = (Color,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(961usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "fill", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, new_size: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (new_size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(962usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "resize", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(963usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "clear", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: Color,) -> bool {
        type CallRet = bool;
        type CallParams = (Color,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(964usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "has", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(965usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "reverse", self.sys_ptr, args)
        }
    }
    pub fn slice(&self, begin: i64, end: i64,) -> PackedColorArray {
        type CallRet = PackedColorArray;
        type CallParams = (i64, i64,);
        let args = (begin, end,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(966usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "slice", self.sys_ptr, args)
        }
    }
    pub fn to_byte_array(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(967usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "to_byte_array", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(968usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "sort", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&mut self, value: Color, before: bool,) -> i64 {
        type CallRet = i64;
        type CallParams = (Color, bool,);
        let args = (value, before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(969usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "bsearch", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&mut self,) -> PackedColorArray {
        type CallRet = PackedColorArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(970usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "duplicate", self.sys_ptr, args)
        }
    }
    pub fn find(&self, value: Color, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (Color, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(971usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "find", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, value: Color, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (Color, i64,);
        let args = (value, from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(972usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "rfind", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: Color,) -> i64 {
        type CallRet = i64;
        type CallParams = (Color,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(973usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "count", self.sys_ptr, args)
        }
    }
    pub fn erase(&mut self, value: Color,) -> bool {
        type CallRet = bool;
        type CallParams = (Color,);
        let args = (value,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(974usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "PackedColorArray", "erase", self.sys_ptr, args)
        }
    }
}
impl PackedColorArray {
    
}