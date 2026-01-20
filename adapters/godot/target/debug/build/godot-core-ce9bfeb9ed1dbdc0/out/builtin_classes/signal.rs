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
pub struct InnerSignal < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerSignal < 'a > {
    pub fn from_outer(outer: &Signal) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn is_null(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(627usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "is_null", self.sys_ptr, args)
        }
    }
    pub fn get_object(&self,) -> Option < Gd < crate::classes::Object > > {
        type CallRet = Option < Gd < crate::classes::Object > >;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(628usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "get_object", self.sys_ptr, args)
        }
    }
    pub fn get_object_id(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(629usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "get_object_id", self.sys_ptr, args)
        }
    }
    pub fn get_name(&self,) -> StringName {
        type CallRet = StringName;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(630usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "get_name", self.sys_ptr, args)
        }
    }
    pub fn connect(&mut self, callable: &Callable, flags: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >, i64,);
        let args = (RefArg::new(callable), flags,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(631usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "connect", self.sys_ptr, args)
        }
    }
    pub fn disconnect(&mut self, callable: &Callable,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
        let args = (RefArg::new(callable),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(632usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "disconnect", self.sys_ptr, args)
        }
    }
    pub fn is_connected(&self, callable: &Callable,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
        let args = (RefArg::new(callable),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(633usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "is_connected", self.sys_ptr, args)
        }
    }
    pub fn get_connections(&self,) -> VarArray {
        type CallRet = VarArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(634usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "get_connections", self.sys_ptr, args)
        }
    }
    pub fn has_connections(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(635usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Signal", "has_connections", self.sys_ptr, args)
        }
    }
    pub fn emit(&self, varargs: &[Variant]) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(636usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall_varargs(method_bind, "Signal", "emit", self.sys_ptr, args, varargs)
        }
    }
}
impl Signal {
    
}