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
pub struct InnerCallable < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerCallable < 'a > {
    pub fn from_outer(outer: &Callable) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn create(variant: &Variant, method: impl AsArg < StringName >,) -> Callable {
        type CallRet = Callable;
        type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, CowArg < 'a1, StringName >,);
        let args = (RefArg::new(variant), method.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(606usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "create", std::ptr::null_mut(), args)
        }
    }
    pub fn callv(&self, arguments: &VarArray,) -> Variant {
        type CallRet = Variant;
        type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
        let args = (RefArg::new(arguments),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(607usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "callv", self.sys_ptr, args)
        }
    }
    pub fn is_null(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(608usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "is_null", self.sys_ptr, args)
        }
    }
    pub fn is_custom(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(609usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "is_custom", self.sys_ptr, args)
        }
    }
    pub fn is_standard(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(610usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "is_standard", self.sys_ptr, args)
        }
    }
    pub fn is_valid(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(611usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "is_valid", self.sys_ptr, args)
        }
    }
    pub fn get_object(&self,) -> Option < Gd < crate::classes::Object > > {
        type CallRet = Option < Gd < crate::classes::Object > >;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(612usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "get_object", self.sys_ptr, args)
        }
    }
    pub fn get_object_id(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(613usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "get_object_id", self.sys_ptr, args)
        }
    }
    pub fn get_method(&self,) -> StringName {
        type CallRet = StringName;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(614usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "get_method", self.sys_ptr, args)
        }
    }
    pub fn get_argument_count(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(615usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "get_argument_count", self.sys_ptr, args)
        }
    }
    pub fn get_bound_arguments_count(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(616usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "get_bound_arguments_count", self.sys_ptr, args)
        }
    }
    pub fn get_unbound_arguments_count(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(618usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "get_unbound_arguments_count", self.sys_ptr, args)
        }
    }
    pub fn hash(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(619usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "hash", self.sys_ptr, args)
        }
    }
    pub fn bindv(&mut self, arguments: &VarArray,) -> Callable {
        type CallRet = Callable;
        type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
        let args = (RefArg::new(arguments),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(620usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "bindv", self.sys_ptr, args)
        }
    }
    pub fn unbind(&self, argcount: i64,) -> Callable {
        type CallRet = Callable;
        type CallParams = (i64,);
        let args = (argcount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(621usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "unbind", self.sys_ptr, args)
        }
    }
}
impl Callable {
    pub fn get_bound_arguments(&self,) -> VarArray {
        type CallRet = VarArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(617usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Callable", "get_bound_arguments", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn call(&self, varargs: &[Variant]) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(622usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall_varargs(method_bind, "Callable", "call", sys::SysPtr::force_mut(self.sys()), args, varargs)
        }
    }
    pub fn call_deferred(&self, varargs: &[Variant]) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(623usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall_varargs(method_bind, "Callable", "call_deferred", sys::SysPtr::force_mut(self.sys()), args, varargs)
        }
    }
    pub fn rpc(&self, varargs: &[Variant]) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(624usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall_varargs(method_bind, "Callable", "rpc", sys::SysPtr::force_mut(self.sys()), args, varargs)
        }
    }
    pub fn rpc_id(&self, peer_id: i64, varargs: &[Variant]) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (peer_id,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(625usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall_varargs(method_bind, "Callable", "rpc_id", sys::SysPtr::force_mut(self.sys()), args, varargs)
        }
    }
    pub fn bind(&self, varargs: &[Variant]) -> Callable {
        type CallRet = Callable;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(626usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall_varargs(method_bind, "Callable", "bind", sys::SysPtr::force_mut(self.sys()), args, varargs)
        }
    }
}