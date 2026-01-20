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
pub struct InnerString < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerString < 'a > {
    pub fn from_outer(outer: &GString) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn casecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(0usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "casecmp_to", self.sys_ptr, args)
        }
    }
    pub fn nocasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(1usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "nocasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn naturalcasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(2usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "naturalcasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn naturalnocasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(3usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "naturalnocasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn filecasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(4usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "filecasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn filenocasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(5usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "filenocasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn length(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(6usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "length", self.sys_ptr, args)
        }
    }
    pub fn substr(&self, from: i64, len: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64, i64,);
        let args = (from, len,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(7usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "substr", self.sys_ptr, args)
        }
    }
    pub fn get_slice(&self, delimiter: impl AsArg < GString >, slice: i64,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (delimiter.into_arg(), slice,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(8usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "get_slice", self.sys_ptr, args)
        }
    }
    pub fn get_slicec(&self, delimiter: i64, slice: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64, i64,);
        let args = (delimiter, slice,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(9usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "get_slicec", self.sys_ptr, args)
        }
    }
    pub fn get_slice_count(&self, delimiter: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (delimiter.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(10usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "get_slice_count", self.sys_ptr, args)
        }
    }
    pub fn find(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(11usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "find", self.sys_ptr, args)
        }
    }
    pub fn findn(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(12usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "findn", self.sys_ptr, args)
        }
    }
    pub fn count(&self, what: impl AsArg < GString >, from: i64, to: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64, i64,);
        let args = (what.into_arg(), from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(13usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "count", self.sys_ptr, args)
        }
    }
    pub fn countn(&self, what: impl AsArg < GString >, from: i64, to: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64, i64,);
        let args = (what.into_arg(), from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(14usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "countn", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(15usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "rfind", self.sys_ptr, args)
        }
    }
    pub fn rfindn(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(16usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "rfindn", self.sys_ptr, args)
        }
    }
    pub fn match_(&self, expr: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (expr.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(17usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "match", self.sys_ptr, args)
        }
    }
    pub fn matchn(&self, expr: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (expr.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(18usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "matchn", self.sys_ptr, args)
        }
    }
    pub fn format(&self, values: &Variant, placeholder: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, CowArg < 'a1, GString >,);
        let args = (RefArg::new(values), placeholder.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(25usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "format", self.sys_ptr, args)
        }
    }
    pub fn replace_char(&self, key: i64, with: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64, i64,);
        let args = (key, with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(28usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "replace_char", self.sys_ptr, args)
        }
    }
    pub fn replace_chars(&self, keys: impl AsArg < GString >, with: i64,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (keys.into_arg(), with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(29usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "replace_chars", self.sys_ptr, args)
        }
    }
    pub fn remove_char(&self, what: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (what,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(30usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "remove_char", self.sys_ptr, args)
        }
    }
    pub fn remove_chars(&self, chars: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (chars.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(31usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "remove_chars", self.sys_ptr, args)
        }
    }
    pub fn insert(&self, position: i64, what: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (i64, CowArg < 'a0, GString >,);
        let args = (position, what.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(34usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "insert", self.sys_ptr, args)
        }
    }
    pub fn erase(&self, position: i64, chars: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64, i64,);
        let args = (position, chars,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(35usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "erase", self.sys_ptr, args)
        }
    }
    pub fn to_kebab_case(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(40usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "to_kebab_case", self.sys_ptr, args)
        }
    }
    pub fn split(&self, delimiter: impl AsArg < GString >, allow_empty: bool, maxsplit: i64,) -> PackedStringArray {
        type CallRet = PackedStringArray;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool, i64,);
        let args = (delimiter.into_arg(), allow_empty, maxsplit,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(41usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "split", self.sys_ptr, args)
        }
    }
    pub fn rsplit(&self, delimiter: impl AsArg < GString >, allow_empty: bool, maxsplit: i64,) -> PackedStringArray {
        type CallRet = PackedStringArray;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool, i64,);
        let args = (delimiter.into_arg(), allow_empty, maxsplit,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(42usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "rsplit", self.sys_ptr, args)
        }
    }
    pub fn unicode_at(&self, at: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (at,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(56usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "unicode_at", self.sys_ptr, args)
        }
    }
    pub fn hash(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(59usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "hash", self.sys_ptr, args)
        }
    }
    pub fn uri_file_decode(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(78usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "uri_file_decode", self.sys_ptr, args)
        }
    }
    pub fn is_valid_ascii_identifier(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(84usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "is_valid_ascii_identifier", self.sys_ptr, args)
        }
    }
    pub fn is_valid_unicode_identifier(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(85usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "is_valid_unicode_identifier", self.sys_ptr, args)
        }
    }
    pub fn lpad(&self, min_length: i64, character: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (i64, CowArg < 'a0, GString >,);
        let args = (min_length, character.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(97usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "lpad", self.sys_ptr, args)
        }
    }
    pub fn rpad(&self, min_length: i64, character: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (i64, CowArg < 'a0, GString >,);
        let args = (min_length, character.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(98usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "rpad", self.sys_ptr, args)
        }
    }
    pub fn pad_decimals(&self, digits: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (digits,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(99usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "pad_decimals", self.sys_ptr, args)
        }
    }
    pub fn pad_zeros(&self, digits: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (digits,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(100usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "pad_zeros", self.sys_ptr, args)
        }
    }
    pub fn to_multibyte_char_buffer(&self, encoding: impl AsArg < GString >,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (encoding.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(108usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "to_multibyte_char_buffer", self.sys_ptr, args)
        }
    }
}
impl GString {
    pub fn begins_with(&self, text: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(19usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "begins_with", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn ends_with(&self, text: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(20usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "ends_with", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_subsequence_of(&self, text: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(21usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "is_subsequence_of", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_subsequence_ofn(&self, text: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(22usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "is_subsequence_ofn", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn bigrams(&self,) -> PackedStringArray {
        type CallRet = PackedStringArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(23usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "bigrams", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn similarity(&self, text: impl AsArg < GString >,) -> f64 {
        type CallRet = f64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(24usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "similarity", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn replace(&self, what: impl AsArg < GString >, forwhat: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
        let args = (what.into_arg(), forwhat.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(26usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "replace", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn replacen(&self, what: impl AsArg < GString >, forwhat: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
        let args = (what.into_arg(), forwhat.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(27usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "replacen", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn repeat(&self, count: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (count,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(32usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "repeat", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn reverse(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(33usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "reverse", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn capitalize(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(36usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "capitalize", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_camel_case(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(37usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "to_camel_case", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_pascal_case(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(38usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "to_pascal_case", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_snake_case(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(39usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "to_snake_case", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn split_floats(&self, delimiter: impl AsArg < GString >, allow_empty: bool,) -> PackedFloat64Array {
        type CallRet = PackedFloat64Array;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
        let args = (delimiter.into_arg(), allow_empty,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(43usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "split_floats", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn join(&self, parts: &PackedStringArray,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
        let args = (RefArg::new(parts),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(44usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "join", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_upper(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(45usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "to_upper", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_lower(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(46usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "to_lower", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn left(&self, length: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(47usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "left", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn right(&self, length: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(48usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "right", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn strip_edges(&self, left: bool, right: bool,) -> GString {
        type CallRet = GString;
        type CallParams = (bool, bool,);
        let args = (left, right,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(49usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "strip_edges", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn strip_escapes(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(50usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "strip_escapes", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn lstrip(&self, chars: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (chars.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(51usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "lstrip", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn rstrip(&self, chars: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (chars.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(52usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "rstrip", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_extension(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(53usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "get_extension", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_basename(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(54usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "get_basename", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn path_join(&self, path: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (path.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(55usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "path_join", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn indent(&self, prefix: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (prefix.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(57usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "indent", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn dedent(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(58usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "dedent", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn md5_text(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(60usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "md5_text", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn sha1_text(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(61usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "sha1_text", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn sha256_text(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(62usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "sha256_text", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn md5_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(63usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "md5_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn sha1_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(64usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "sha1_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn sha256_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(65usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "sha256_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(66usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "is_empty", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn contains(&self, what: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (what.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(67usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "contains", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn containsn(&self, what: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (what.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(68usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "containsn", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_absolute_path(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(69usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "is_absolute_path", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_relative_path(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(70usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "is_relative_path", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn simplify_path(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(71usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "simplify_path", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_base_dir(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(72usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "get_base_dir", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_file(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(73usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "get_file", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn xml_escape(&self, escape_quotes: bool,) -> GString {
        type CallRet = GString;
        type CallParams = (bool,);
        let args = (escape_quotes,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(74usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "xml_escape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn xml_unescape(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(75usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "xml_unescape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn uri_encode(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(76usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "uri_encode", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn uri_decode(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(77usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "uri_decode", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn c_escape(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(79usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "c_escape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn c_unescape(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(80usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "c_unescape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn json_escape(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(81usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "json_escape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn validate_node_name(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(82usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "validate_node_name", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn validate_filename(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(83usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "validate_filename", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_identifier(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(86usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "is_valid_identifier", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_int(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(87usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "is_valid_int", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_float(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(88usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "is_valid_float", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_hex_number(&self, with_prefix: bool,) -> bool {
        type CallRet = bool;
        type CallParams = (bool,);
        let args = (with_prefix,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(89usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "is_valid_hex_number", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_html_color(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(90usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "is_valid_html_color", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_ip_address(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(91usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "is_valid_ip_address", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_filename(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(92usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "is_valid_filename", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_int(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(93usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "to_int", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_float(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(94usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "to_float", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn hex_to_int(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(95usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "hex_to_int", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn bin_to_int(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(96usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "bin_to_int", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn trim_prefix(&self, prefix: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (prefix.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(101usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "trim_prefix", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn trim_suffix(&self, suffix: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (suffix.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(102usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "trim_suffix", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_ascii_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(103usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "to_ascii_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_utf8_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(104usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "to_utf8_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_utf16_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(105usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "to_utf16_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_utf32_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(106usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "to_utf32_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_wchar_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(107usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "to_wchar_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn hex_decode(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(109usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "hex_decode", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn num_scientific(number: f64,) -> GString {
        type CallRet = GString;
        type CallParams = (f64,);
        let args = (number,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(110usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "num_scientific", std::ptr::null_mut(), args)
        }
    }
    pub fn num(number: f64, decimals: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (f64, i64,);
        let args = (number, decimals,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(111usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "num", std::ptr::null_mut(), args)
        }
    }
    pub fn num_int64(number: i64, base: i64, capitalize_hex: bool,) -> GString {
        type CallRet = GString;
        type CallParams = (i64, i64, bool,);
        let args = (number, base, capitalize_hex,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(112usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "num_int64", std::ptr::null_mut(), args)
        }
    }
    pub fn num_uint64(number: i64, base: i64, capitalize_hex: bool,) -> GString {
        type CallRet = GString;
        type CallParams = (i64, i64, bool,);
        let args = (number, base, capitalize_hex,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(113usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "num_uint64", std::ptr::null_mut(), args)
        }
    }
    pub fn chr(code: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (code,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(114usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "chr", std::ptr::null_mut(), args)
        }
    }
    pub fn humanize_size(size: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (size,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(115usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "String", "humanize_size", std::ptr::null_mut(), args)
        }
    }
}