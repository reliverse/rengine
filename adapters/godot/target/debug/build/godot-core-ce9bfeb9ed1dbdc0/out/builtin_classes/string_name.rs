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
pub struct InnerStringName < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerStringName < 'a > {
    pub fn from_outer(outer: &StringName) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn casecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(483usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "casecmp_to", self.sys_ptr, args)
        }
    }
    pub fn nocasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(484usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "nocasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn naturalcasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(485usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "naturalcasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn naturalnocasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(486usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "naturalnocasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn filecasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(487usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "filecasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn filenocasecmp_to(&self, to: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (to.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(488usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "filenocasecmp_to", self.sys_ptr, args)
        }
    }
    pub fn length(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(489usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "length", self.sys_ptr, args)
        }
    }
    pub fn substr(&self, from: i64, len: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64, i64,);
        let args = (from, len,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(490usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "substr", self.sys_ptr, args)
        }
    }
    pub fn get_slice(&self, delimiter: impl AsArg < GString >, slice: i64,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (delimiter.into_arg(), slice,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(491usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "get_slice", self.sys_ptr, args)
        }
    }
    pub fn get_slicec(&self, delimiter: i64, slice: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64, i64,);
        let args = (delimiter, slice,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(492usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "get_slicec", self.sys_ptr, args)
        }
    }
    pub fn get_slice_count(&self, delimiter: impl AsArg < GString >,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (delimiter.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(493usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "get_slice_count", self.sys_ptr, args)
        }
    }
    pub fn find(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(494usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "find", self.sys_ptr, args)
        }
    }
    pub fn findn(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(495usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "findn", self.sys_ptr, args)
        }
    }
    pub fn count(&self, what: impl AsArg < GString >, from: i64, to: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64, i64,);
        let args = (what.into_arg(), from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(496usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "count", self.sys_ptr, args)
        }
    }
    pub fn countn(&self, what: impl AsArg < GString >, from: i64, to: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64, i64,);
        let args = (what.into_arg(), from, to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(497usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "countn", self.sys_ptr, args)
        }
    }
    pub fn rfind(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(498usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "rfind", self.sys_ptr, args)
        }
    }
    pub fn rfindn(&self, what: impl AsArg < GString >, from: i64,) -> i64 {
        type CallRet = i64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (what.into_arg(), from,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(499usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "rfindn", self.sys_ptr, args)
        }
    }
    pub fn match_(&self, expr: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (expr.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(500usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "match", self.sys_ptr, args)
        }
    }
    pub fn matchn(&self, expr: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (expr.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(501usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "matchn", self.sys_ptr, args)
        }
    }
    pub fn format(&self, values: &Variant, placeholder: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, CowArg < 'a1, GString >,);
        let args = (RefArg::new(values), placeholder.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(508usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "format", self.sys_ptr, args)
        }
    }
    pub fn replace_char(&self, key: i64, with: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64, i64,);
        let args = (key, with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(511usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "replace_char", self.sys_ptr, args)
        }
    }
    pub fn replace_chars(&self, keys: impl AsArg < GString >, with: i64,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
        let args = (keys.into_arg(), with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(512usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "replace_chars", self.sys_ptr, args)
        }
    }
    pub fn remove_char(&self, what: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (what,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(513usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "remove_char", self.sys_ptr, args)
        }
    }
    pub fn remove_chars(&self, chars: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (chars.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(514usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "remove_chars", self.sys_ptr, args)
        }
    }
    pub fn insert(&self, position: i64, what: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (i64, CowArg < 'a0, GString >,);
        let args = (position, what.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(517usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "insert", self.sys_ptr, args)
        }
    }
    pub fn erase(&self, position: i64, chars: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64, i64,);
        let args = (position, chars,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(518usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "erase", self.sys_ptr, args)
        }
    }
    pub fn to_kebab_case(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(523usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_kebab_case", self.sys_ptr, args)
        }
    }
    pub fn split(&self, delimiter: impl AsArg < GString >, allow_empty: bool, maxsplit: i64,) -> PackedStringArray {
        type CallRet = PackedStringArray;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool, i64,);
        let args = (delimiter.into_arg(), allow_empty, maxsplit,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(524usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "split", self.sys_ptr, args)
        }
    }
    pub fn rsplit(&self, delimiter: impl AsArg < GString >, allow_empty: bool, maxsplit: i64,) -> PackedStringArray {
        type CallRet = PackedStringArray;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool, i64,);
        let args = (delimiter.into_arg(), allow_empty, maxsplit,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(525usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "rsplit", self.sys_ptr, args)
        }
    }
    pub fn unicode_at(&self, at: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (at,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(539usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "unicode_at", self.sys_ptr, args)
        }
    }
    pub fn uri_file_decode(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(560usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "uri_file_decode", self.sys_ptr, args)
        }
    }
    pub fn is_valid_ascii_identifier(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(566usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_ascii_identifier", self.sys_ptr, args)
        }
    }
    pub fn is_valid_unicode_identifier(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(567usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_unicode_identifier", self.sys_ptr, args)
        }
    }
    pub fn lpad(&self, min_length: i64, character: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (i64, CowArg < 'a0, GString >,);
        let args = (min_length, character.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(579usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "lpad", self.sys_ptr, args)
        }
    }
    pub fn rpad(&self, min_length: i64, character: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (i64, CowArg < 'a0, GString >,);
        let args = (min_length, character.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(580usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "rpad", self.sys_ptr, args)
        }
    }
    pub fn pad_decimals(&self, digits: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (digits,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(581usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "pad_decimals", self.sys_ptr, args)
        }
    }
    pub fn pad_zeros(&self, digits: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (digits,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(582usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "pad_zeros", self.sys_ptr, args)
        }
    }
    pub fn to_multibyte_char_buffer(&self, encoding: impl AsArg < GString >,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (encoding.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(590usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_multibyte_char_buffer", self.sys_ptr, args)
        }
    }
    pub fn hash(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(592usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "hash", self.sys_ptr, args)
        }
    }
}
impl StringName {
    pub fn begins_with(&self, text: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(502usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "begins_with", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn ends_with(&self, text: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(503usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "ends_with", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_subsequence_of(&self, text: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(504usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_subsequence_of", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_subsequence_ofn(&self, text: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(505usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_subsequence_ofn", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn bigrams(&self,) -> PackedStringArray {
        type CallRet = PackedStringArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(506usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "bigrams", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn similarity(&self, text: impl AsArg < GString >,) -> f64 {
        type CallRet = f64;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (text.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(507usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "similarity", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn replace(&self, what: impl AsArg < GString >, forwhat: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
        let args = (what.into_arg(), forwhat.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(509usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "replace", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn replacen(&self, what: impl AsArg < GString >, forwhat: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
        let args = (what.into_arg(), forwhat.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(510usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "replacen", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn repeat(&self, count: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (count,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(515usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "repeat", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn reverse(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(516usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "reverse", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn capitalize(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(519usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "capitalize", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_camel_case(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(520usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_camel_case", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_pascal_case(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(521usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_pascal_case", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_snake_case(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(522usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_snake_case", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn split_floats(&self, delimiter: impl AsArg < GString >, allow_empty: bool,) -> PackedFloat64Array {
        type CallRet = PackedFloat64Array;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
        let args = (delimiter.into_arg(), allow_empty,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(526usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "split_floats", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn join(&self, parts: &PackedStringArray,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
        let args = (RefArg::new(parts),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(527usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "join", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_upper(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(528usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_upper", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_lower(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(529usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_lower", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn left(&self, length: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(530usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "left", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn right(&self, length: i64,) -> GString {
        type CallRet = GString;
        type CallParams = (i64,);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(531usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "right", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn strip_edges(&self, left: bool, right: bool,) -> GString {
        type CallRet = GString;
        type CallParams = (bool, bool,);
        let args = (left, right,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(532usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "strip_edges", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn strip_escapes(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(533usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "strip_escapes", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn lstrip(&self, chars: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (chars.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(534usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "lstrip", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn rstrip(&self, chars: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (chars.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(535usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "rstrip", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_extension(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(536usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "get_extension", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_basename(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(537usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "get_basename", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn path_join(&self, path: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (path.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(538usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "path_join", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn indent(&self, prefix: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (prefix.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(540usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "indent", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn dedent(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(541usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "dedent", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn md5_text(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(542usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "md5_text", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn sha1_text(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(543usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "sha1_text", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn sha256_text(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(544usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "sha256_text", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn md5_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(545usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "md5_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn sha1_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(546usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "sha1_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn sha256_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(547usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "sha256_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_empty(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(548usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_empty", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn contains(&self, what: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (what.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(549usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "contains", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn containsn(&self, what: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (what.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(550usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "containsn", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_absolute_path(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(551usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_absolute_path", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_relative_path(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(552usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_relative_path", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn simplify_path(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(553usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "simplify_path", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_base_dir(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(554usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "get_base_dir", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn get_file(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(555usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "get_file", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn xml_escape(&self, escape_quotes: bool,) -> GString {
        type CallRet = GString;
        type CallParams = (bool,);
        let args = (escape_quotes,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(556usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "xml_escape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn xml_unescape(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(557usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "xml_unescape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn uri_encode(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(558usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "uri_encode", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn uri_decode(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(559usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "uri_decode", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn c_escape(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(561usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "c_escape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn c_unescape(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(562usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "c_unescape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn json_escape(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(563usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "json_escape", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn validate_node_name(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(564usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "validate_node_name", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn validate_filename(&self,) -> GString {
        type CallRet = GString;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(565usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "validate_filename", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_identifier(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(568usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_identifier", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_int(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(569usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_int", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_float(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(570usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_float", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_hex_number(&self, with_prefix: bool,) -> bool {
        type CallRet = bool;
        type CallParams = (bool,);
        let args = (with_prefix,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(571usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_hex_number", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_html_color(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(572usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_html_color", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_ip_address(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(573usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_ip_address", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn is_valid_filename(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(574usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "is_valid_filename", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_int(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(575usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_int", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_float(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(576usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_float", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn hex_to_int(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(577usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "hex_to_int", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn bin_to_int(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(578usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "bin_to_int", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn trim_prefix(&self, prefix: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (prefix.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(583usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "trim_prefix", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn trim_suffix(&self, suffix: impl AsArg < GString >,) -> GString {
        type CallRet = GString;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (suffix.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(584usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "trim_suffix", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_ascii_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(585usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_ascii_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_utf8_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(586usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_utf8_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_utf16_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(587usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_utf16_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_utf32_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(588usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_utf32_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn to_wchar_buffer(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(589usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "to_wchar_buffer", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
    pub fn hex_decode(&self,) -> PackedByteArray {
        type CallRet = PackedByteArray;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(591usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "StringName", "hex_decode", sys::SysPtr::force_mut(self.sys()), args)
        }
    }
}