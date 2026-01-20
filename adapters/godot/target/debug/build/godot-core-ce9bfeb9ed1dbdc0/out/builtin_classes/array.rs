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
pub struct InnerArray < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerArray < 'a > {
    pub fn from_outer(outer: &VarArray) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn from_outer_typed < T > (outer: &Array < T >) -> Self where T: crate::meta::ArrayElement {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn size(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(671usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "size", self.sys_ptr, args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(672usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "is_empty", self.sys_ptr, args)
        }
    }
    pub fn clear(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(673usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "clear", self.sys_ptr, args)
        }
    }
    pub fn hash(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(674usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "hash", self.sys_ptr, args)
        }
    }
    pub fn assign(&mut self, array: &VarArray,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(675usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "assign", self.sys_ptr, args)
        }
    }
    pub fn get(&self, index: i64,) -> Variant {
        type CallRet = Variant;
        type CallParams = (i64,);
        let args = (index,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(676usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "get", self.sys_ptr, args)
        }
    }
    pub fn set(&mut self, index: i64, value: &Variant,) {
        type CallRet = ();
        type CallParams < 'a0, > = (i64, RefArg < 'a0, Variant >,);
        let args = (index, RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(677usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "set", self.sys_ptr, args)
        }
    }
    pub fn push_back(&mut self, value: &Variant,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(678usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "push_back", self.sys_ptr, args)
        }
    }
    pub fn push_front(&mut self, value: &Variant,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(679usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "push_front", self.sys_ptr, args)
        }
    }
    pub fn append(&mut self, value: &Variant,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(680usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "append", self.sys_ptr, args)
        }
    }
    pub fn append_array(&mut self, array: &VarArray,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(681usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "append_array", self.sys_ptr, args)
        }
    }
    pub fn resize(&mut self, size: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(682usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "resize", self.sys_ptr, args)
        }
    }
    pub fn insert(&mut self, position: i64, value: &Variant,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (i64, RefArg < 'a0, Variant >,);
        let args = (position, RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(683usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "insert", self.sys_ptr, args)
        }
    }
    pub fn remove_at(&mut self, position: i64,) {
        type CallRet = ();
        type CallParams = (i64,);
        let args = (position,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(684usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "remove_at", self.sys_ptr, args)
        }
    }
    pub fn fill(&mut self, value: &Variant,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(685usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "fill", self.sys_ptr, args)
        }
    }
    pub fn erase(&mut self, value: &Variant,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(686usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "erase", self.sys_ptr, args)
        }
    }
    pub fn front(&self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(687usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "front", self.sys_ptr, args)
        }
    }
    pub fn back(&self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(688usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "back", self.sys_ptr, args)
        }
    }
    pub fn pick_random(&self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(689usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "pick_random", self.sys_ptr, args)
        }
    }
    pub fn find(&self, what: &Variant, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >, i64,);
        let args = (RefArg::new(what), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(690usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "find", self.sys_ptr, args)
        }
    }
    pub fn find_custom(&self, method: &Callable, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >, i64,);
        let args = (RefArg::new(method), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(691usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "find_custom", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, what: &Variant, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >, i64,);
        let args = (RefArg::new(what), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(692usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "rfind", self.sys_ptr, args)
        }
    }
    pub fn rfind_custom(&self, method: &Callable, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >, i64,);
        let args = (RefArg::new(method), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(693usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "rfind_custom", self.sys_ptr, args)
        }
    }
    pub fn count(&self, value: &Variant,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(694usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "count", self.sys_ptr, args)
        }
    }
    pub fn has(&self, value: &Variant,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
        let args = (RefArg::new(value),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(695usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "has", self.sys_ptr, args)
        }
    }
    pub fn pop_back(&mut self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(696usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "pop_back", self.sys_ptr, args)
        }
    }
    pub fn pop_front(&mut self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(697usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "pop_front", self.sys_ptr, args)
        }
    }
    pub fn pop_at(&mut self, position: i64,) -> Variant {
        type CallRet = Variant;
        type CallParams = (i64,);
        let args = (position,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(698usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "pop_at", self.sys_ptr, args)
        }
    }
    pub fn sort(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(699usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "sort", self.sys_ptr, args)
        }
    }
    pub fn sort_custom(&mut self, func: &Callable,) {
        type CallRet = ();
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
        let args = (RefArg::new(func),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(700usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "sort_custom", self.sys_ptr, args)
        }
    }
    pub fn shuffle(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(701usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "shuffle", self.sys_ptr, args)
        }
    }
    pub fn bsearch(&self, value: &Variant, before: bool,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (RefArg < 'a0, Variant >, bool,);
        let args = (RefArg::new(value), before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(702usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "bsearch", self.sys_ptr, args)
        }
    }
    pub fn bsearch_custom(&self, value: &Variant, func: &Callable, before: bool,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Callable >, bool,);
        let args = (RefArg::new(value), RefArg::new(func), before,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(703usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "bsearch_custom", self.sys_ptr, args)
        }
    }
    pub fn reverse(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(704usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "reverse", self.sys_ptr, args)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" You must ensure that the returned array fulfils the safety invariants of [`Array`](crate::builtin::Array), this being:"]
    #[doc = r" - Any values written to the array must match the runtime type of the array."]
    #[doc = r" - Any values read from the array must be convertible to the type `T`."]
    #[doc = r""]
    #[doc = r" If the safety invariant of `Array` is intact, which it must be for any publicly accessible arrays, then `T` must match"]
    #[doc = r" the runtime type of the array. This then implies that both of the conditions above hold. This means that you only need"]
    #[doc = r" to keep the above conditions in mind if you are intentionally violating the safety invariant of `Array`."]
    #[doc = r""]
    #[doc = r" In the current implementation, both cases will produce a panic rather than undefined behavior, but this should not be relied upon."]
    pub unsafe fn duplicate < Ret > (&self, deep: bool,) -> Array < Ret > where Ret: crate::meta::ArrayElement, {
        type CallRet < Ret > = Array < Ret >;
        type CallParams = (bool,);
        let args = (deep,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(705usize);
            Signature::< CallParams, CallRet < Ret > > ::out_builtin_ptrcall(method_bind, "Array", "duplicate", self.sys_ptr, args)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" You must ensure that the returned array fulfils the safety invariants of [`Array`](crate::builtin::Array)."]
    pub unsafe fn duplicate_deep(&self, deep_subresources_mode: i64,) -> VarArray {
        type CallRet = VarArray;
        type CallParams = (i64,);
        let args = (deep_subresources_mode,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(706usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "duplicate_deep", self.sys_ptr, args)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" You must ensure that the returned array fulfils the safety invariants of [`Array`](crate::builtin::Array), this being:"]
    #[doc = r" - Any values written to the array must match the runtime type of the array."]
    #[doc = r" - Any values read from the array must be convertible to the type `T`."]
    #[doc = r""]
    #[doc = r" If the safety invariant of `Array` is intact, which it must be for any publicly accessible arrays, then `T` must match"]
    #[doc = r" the runtime type of the array. This then implies that both of the conditions above hold. This means that you only need"]
    #[doc = r" to keep the above conditions in mind if you are intentionally violating the safety invariant of `Array`."]
    #[doc = r""]
    #[doc = r" In the current implementation, both cases will produce a panic rather than undefined behavior, but this should not be relied upon."]
    pub unsafe fn slice < Ret > (&self, begin: i64, end: i64, step: i64, deep: bool,) -> Array < Ret > where Ret: crate::meta::ArrayElement, {
        type CallRet < Ret > = Array < Ret >;
        type CallParams = (i64, i64, i64, bool,);
        let args = (begin, end, step, deep,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(707usize);
            Signature::< CallParams, CallRet < Ret > > ::out_builtin_ptrcall(method_bind, "Array", "slice", self.sys_ptr, args)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r" You must ensure that the returned array fulfils the safety invariants of [`Array`](crate::builtin::Array), this being:"]
    #[doc = r" - Any values written to the array must match the runtime type of the array."]
    #[doc = r" - Any values read from the array must be convertible to the type `T`."]
    #[doc = r""]
    #[doc = r" If the safety invariant of `Array` is intact, which it must be for any publicly accessible arrays, then `T` must match"]
    #[doc = r" the runtime type of the array. This then implies that both of the conditions above hold. This means that you only need"]
    #[doc = r" to keep the above conditions in mind if you are intentionally violating the safety invariant of `Array`."]
    #[doc = r""]
    #[doc = r" In the current implementation, both cases will produce a panic rather than undefined behavior, but this should not be relied upon."]
    pub unsafe fn filter < Ret > (&self, method: &Callable,) -> Array < Ret > where Ret: crate::meta::ArrayElement, {
        type CallRet < Ret > = Array < Ret >;
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
        let args = (RefArg::new(method),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(708usize);
            Signature::< CallParams, CallRet < Ret > > ::out_builtin_ptrcall(method_bind, "Array", "filter", self.sys_ptr, args)
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" You must ensure that the returned array fulfils the safety invariants of [`Array`](crate::builtin::Array)."]
    pub unsafe fn map(&self, method: &Callable,) -> VarArray {
        type CallRet = VarArray;
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
        let args = (RefArg::new(method),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(709usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "map", self.sys_ptr, args)
        }
    }
    pub fn reduce(&self, method: &Callable, accum: &Variant,) -> Variant {
        type CallRet = Variant;
        type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Callable >, RefArg < 'a1, Variant >,);
        let args = (RefArg::new(method), RefArg::new(accum),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(710usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "reduce", self.sys_ptr, args)
        }
    }
    pub fn any(&self, method: &Callable,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
        let args = (RefArg::new(method),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(711usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "any", self.sys_ptr, args)
        }
    }
    pub fn all(&self, method: &Callable,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
        let args = (RefArg::new(method),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(712usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "all", self.sys_ptr, args)
        }
    }
    pub fn max(&self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(713usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "max", self.sys_ptr, args)
        }
    }
    pub fn min(&self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(714usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "min", self.sys_ptr, args)
        }
    }
    pub fn is_typed(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(715usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "is_typed", self.sys_ptr, args)
        }
    }
    pub fn is_same_typed(&self, array: &VarArray,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
        let args = (RefArg::new(array),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(716usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "is_same_typed", self.sys_ptr, args)
        }
    }
    pub fn get_typed_builtin(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(717usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "get_typed_builtin", self.sys_ptr, args)
        }
    }
    pub fn get_typed_class_name(&self,) -> StringName {
        type CallRet = StringName;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(718usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "get_typed_class_name", self.sys_ptr, args)
        }
    }
    pub fn get_typed_script(&self,) -> Variant {
        type CallRet = Variant;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(719usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "get_typed_script", self.sys_ptr, args)
        }
    }
    pub fn make_read_only(&mut self,) {
        type CallRet = ();
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(720usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "make_read_only", self.sys_ptr, args)
        }
    }
    pub fn is_read_only(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(721usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Array", "is_read_only", self.sys_ptr, args)
        }
    }
}
impl VarArray {
    
}