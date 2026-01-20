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
pub struct InnerRid < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerRid < 'a > {
    pub fn from_outer(outer: &Rid) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn is_valid(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(604usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rid", "is_valid", self.sys_ptr, args)
        }
    }
    pub fn get_id(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(605usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Rid", "get_id", self.sys_ptr, args)
        }
    }
}
impl Rid {
    
}