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
pub struct InnerDictionary < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerDictionary < 'a > {
    pub fn from_outer(outer: &VarDictionary) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn size(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(637usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(638usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(639usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "clear", self.sys_ptr, args)
        }
    }
    pub fn assign(&mut self, dictionary: &VarDictionary,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >,);
        let args = (RefArg::new(dictionary),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(640usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "assign", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(641usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "sort", self.sys_ptr, args)
        }
    }
    pub fn merge(&mut self, dictionary: &VarDictionary, overwrite: bool,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >, bool,);
        let args = (RefArg::new(dictionary), overwrite,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(642usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "merge", self.sys_ptr, args)
        }
    }
    pub fn merged(&self, dictionary: &VarDictionary, overwrite: bool,) -> VarDictionary {
        type CallRet = VarDictionary;
        type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >, bool,);
        let args = (RefArg::new(dictionary), overwrite,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(643usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "merged", self.sys_ptr, args)
        }
    }
    pub fn has(&self, key: &Variant,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(key),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(644usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "has", self.sys_ptr, args)
        }
    }
    pub fn has_all(&self, keys: &VarArray,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
        let args = (RefArg::new(keys),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(645usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "has_all", self.sys_ptr, args)
        }
    }
    pub fn find_key(&self, value: &Variant,) -> Variant {
        type CallRet = Variant;
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(646usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "find_key", self.sys_ptr, args)
        }
    }
    pub fn erase(&mut self, key: &Variant,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(key),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(647usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "erase", self.sys_ptr, args)
        }
    }
    pub fn hash(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(648usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "hash", self.sys_ptr, args)
        }
    }
    pub fn keys(&self,) -> VarArray {
        type CallRet = VarArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(649usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "keys", self.sys_ptr, args)
        }
    }
    pub fn values(&self,) -> VarArray {
        type CallRet = VarArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(650usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "values", self.sys_ptr, args)
        }
    }
    pub fn duplicate(&self, deep: bool,) -> VarDictionary {
        type CallRet = VarDictionary;
        type CallParams = (bool,);
        let args = (deep,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(651usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "duplicate", self.sys_ptr, args)
        }
    }
    pub fn duplicate_deep(&self, deep_subresources_mode: i64,) -> VarDictionary {
        type CallRet = VarDictionary;
        type CallParams = (i64,);
        let args = (deep_subresources_mode,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(652usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "duplicate_deep", self.sys_ptr, args)
        }
    }
    pub fn get(&self, key: &Variant, default: &Variant,) -> Variant {
        type CallRet = Variant;
        type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >,);
        let args = (RefArg::new(key), RefArg::new(default),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(653usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "get", self.sys_ptr, args)
        }
    }
    pub fn get_or_add(&mut self, key: &Variant, default: &Variant,) -> Variant {
        type CallRet = Variant;
        type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >,);
        let args = (RefArg::new(key), RefArg::new(default),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(654usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "get_or_add", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, key: &Variant, value: &Variant,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >,);
        let args = (RefArg::new(key), RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(655usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "set", self.sys_ptr, args)
        }
    }
    pub fn is_typed(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(656usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_typed", self.sys_ptr, args)
        }
    }
    pub fn is_typed_key(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(657usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_typed_key", self.sys_ptr, args)
        }
    }
    pub fn is_typed_value(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(658usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_typed_value", self.sys_ptr, args)
        }
    }
    pub fn is_same_typed(&self, dictionary: &VarDictionary,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >,);
        let args = (RefArg::new(dictionary),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(659usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_same_typed", self.sys_ptr, args)
        }
    }
    pub fn is_same_typed_key(&self, dictionary: &VarDictionary,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >,);
        let args = (RefArg::new(dictionary),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(660usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_same_typed_key", self.sys_ptr, args)
        }
    }
    pub fn is_same_typed_value(&self, dictionary: &VarDictionary,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >,);
        let args = (RefArg::new(dictionary),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(661usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_same_typed_value", self.sys_ptr, args)
        }
    }
    pub fn get_typed_key_builtin(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(662usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "get_typed_key_builtin", self.sys_ptr, args)
        }
    }
    pub fn get_typed_value_builtin(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(663usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "get_typed_value_builtin", self.sys_ptr, args)
        }
    }
    pub fn get_typed_key_class_name(&self,) -> StringName {
        type CallRet = StringName;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(664usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "get_typed_key_class_name", self.sys_ptr, args)
        }
    }
    pub fn get_typed_value_class_name(&self,) -> StringName {
        type CallRet = StringName;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(665usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "get_typed_value_class_name", self.sys_ptr, args)
        }
    }
    pub fn get_typed_key_script(&self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(666usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "get_typed_key_script", self.sys_ptr, args)
        }
    }
    pub fn get_typed_value_script(&self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(667usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "get_typed_value_script", self.sys_ptr, args)
        }
    }
    pub fn make_read_only(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(668usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "make_read_only", self.sys_ptr, args)
        }
    }
    pub fn is_read_only(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(669usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "is_read_only", self.sys_ptr, args)
        }
    }
    pub fn recursive_equal(&self, dictionary: &VarDictionary, recursion_count: i64,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >, i64,);
        let args = (RefArg::new(dictionary), recursion_count,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(670usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Dictionary", "recursive_equal", self.sys_ptr, args)
        }
    }
}
impl VarDictionary {
    
}