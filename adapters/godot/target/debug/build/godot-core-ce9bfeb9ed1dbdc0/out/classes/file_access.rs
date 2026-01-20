#![doc = "Sidecar module for class [`FileAccess`][crate::classes::FileAccess].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FileAccess` enums](https://docs.godotengine.org/en/stable/classes/class_fileaccess.html#enumerations).\n\n"]
use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, ClassId, CowArg, InParamTuple, OutParamTuple, ParamTuple, RefArg, Signature
};
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
use crate::classes::notify::*;
use std::ffi::c_void;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `FileAccess.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`file_access`][crate::classes::file_access]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `FileAccess`](https://docs.godotengine.org/en/stable/classes/class_fileaccess.html).\n\n# Specific notes for this class\n\nThe gdext library provides a higher-level abstraction, which should be preferred: [`GFile`][crate::tools::GFile]."]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<FileAccess>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FileAccess {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl FileAccess {
        pub fn open(path: impl AsArg < GString >, flags: crate::classes::file_access::ModeFlags,) -> Option < Gd < crate::classes::FileAccess > > {
            type CallRet = Option < Gd < crate::classes::FileAccess > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::file_access::ModeFlags,);
            let args = (path.into_arg(), flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "open", None, args,)
            }
        }
        pub(crate) fn open_encrypted_full(path: CowArg < GString >, mode_flags: crate::classes::file_access::ModeFlags, key: RefArg < PackedByteArray >, iv: RefArg < PackedByteArray >,) -> Option < Gd < crate::classes::FileAccess > > {
            type CallRet = Option < Gd < crate::classes::FileAccess > >;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, crate::classes::file_access::ModeFlags, RefArg < 'a1, PackedByteArray >, RefArg < 'a2, PackedByteArray >,);
            let args = (path, mode_flags, key, iv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "open_encrypted", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::open_encrypted_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn open_encrypted(path: impl AsArg < GString >, mode_flags: crate::classes::file_access::ModeFlags, key: &PackedByteArray,) -> Option < Gd < crate::classes::FileAccess > > {
            Self::open_encrypted_ex(path, mode_flags, key,) . done()
        }
        #[inline]
        pub fn open_encrypted_ex < 'a > (path: impl AsArg < GString > + 'a, mode_flags: crate::classes::file_access::ModeFlags, key: &'a PackedByteArray,) -> ExOpenEncrypted < 'a > {
            ExOpenEncrypted::new(path, mode_flags, key,)
        }
        pub fn open_encrypted_with_pass(path: impl AsArg < GString >, mode_flags: crate::classes::file_access::ModeFlags, pass: impl AsArg < GString >,) -> Option < Gd < crate::classes::FileAccess > > {
            type CallRet = Option < Gd < crate::classes::FileAccess > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, crate::classes::file_access::ModeFlags, CowArg < 'a1, GString >,);
            let args = (path.into_arg(), mode_flags, pass.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "open_encrypted_with_pass", None, args,)
            }
        }
        pub(crate) fn open_compressed_full(path: CowArg < GString >, mode_flags: crate::classes::file_access::ModeFlags, compression_mode: crate::classes::file_access::CompressionMode,) -> Option < Gd < crate::classes::FileAccess > > {
            type CallRet = Option < Gd < crate::classes::FileAccess > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::file_access::ModeFlags, crate::classes::file_access::CompressionMode,);
            let args = (path, mode_flags, compression_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "open_compressed", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::open_compressed_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn open_compressed(path: impl AsArg < GString >, mode_flags: crate::classes::file_access::ModeFlags,) -> Option < Gd < crate::classes::FileAccess > > {
            Self::open_compressed_ex(path, mode_flags,) . done()
        }
        #[inline]
        pub fn open_compressed_ex < 'a > (path: impl AsArg < GString > + 'a, mode_flags: crate::classes::file_access::ModeFlags,) -> ExOpenCompressed < 'a > {
            ExOpenCompressed::new(path, mode_flags,)
        }
        pub fn get_open_error() -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_open_error", None, args,)
            }
        }
        pub(crate) fn create_temp_full(mode_flags: crate::classes::file_access::ModeFlags, prefix: CowArg < GString >, extension: CowArg < GString >, keep: bool,) -> Option < Gd < crate::classes::FileAccess > > {
            type CallRet = Option < Gd < crate::classes::FileAccess > >;
            type CallParams < 'a0, 'a1, > = (crate::classes::file_access::ModeFlags, CowArg < 'a0, GString >, CowArg < 'a1, GString >, bool,);
            let args = (mode_flags, prefix, extension, keep,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "create_temp", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_temp_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_temp(mode_flags: crate::classes::file_access::ModeFlags,) -> Option < Gd < crate::classes::FileAccess > > {
            Self::create_temp_ex(mode_flags,) . done()
        }
        #[inline]
        pub fn create_temp_ex < 'a > (mode_flags: crate::classes::file_access::ModeFlags,) -> ExCreateTemp < 'a > {
            ExCreateTemp::new(mode_flags,)
        }
        pub fn get_file_as_bytes(path: impl AsArg < GString >,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_file_as_bytes", None, args,)
            }
        }
        pub fn get_file_as_string(path: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_file_as_string", None, args,)
            }
        }
        pub fn resize(&mut self, length: i64,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (i64,);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "resize", Some(self.__validated_obj()), args,)
            }
        }
        pub fn flush(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "flush", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_path_absolute(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_path_absolute", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_open(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "is_open", Some(self.__validated_obj()), args,)
            }
        }
        pub fn seek(&mut self, position: u64,) {
            type CallRet = ();
            type CallParams = (u64,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "seek", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn seek_end_full(&mut self, position: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "seek_end", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::seek_end_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn seek_end(&mut self,) {
            self.seek_end_ex() . done()
        }
        #[inline]
        pub fn seek_end_ex < 'a > (&'a mut self,) -> ExSeekEnd < 'a > {
            ExSeekEnd::new(self,)
        }
        pub fn get_position(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_length(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn eof_reached(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "eof_reached", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_8(&self,) -> u8 {
            type CallRet = u8;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_8", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_16(&self,) -> u16 {
            type CallRet = u16;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_16", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_32(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_32", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_64(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_64", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_half(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_half", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_float(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_float", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_double(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_double", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_real(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_real", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_buffer(&self, length: i64,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (i64,);
            let args = (length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_line", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_csv_line_full(&self, delim: CowArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (delim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_csv_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_csv_line_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_csv_line(&self,) -> PackedStringArray {
            self.get_csv_line_ex() . done()
        }
        #[inline]
        pub fn get_csv_line_ex < 'a > (&'a self,) -> ExGetCsvLine < 'a > {
            ExGetCsvLine::new(self,)
        }
        pub(crate) fn get_as_text_full(&self, skip_cr: bool,) -> GString {
            type CallRet = GString;
            type CallParams = (bool,);
            let args = (skip_cr,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_as_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_as_text_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_as_text(&self,) -> GString {
            self.get_as_text_ex() . done()
        }
        #[inline]
        pub fn get_as_text_ex < 'a > (&'a self,) -> ExGetAsText < 'a > {
            ExGetAsText::new(self,)
        }
        pub fn get_md5(path: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_md5", None, args,)
            }
        }
        pub fn get_sha256(path: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_sha256", None, args,)
            }
        }
        pub fn is_big_endian(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "is_big_endian", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_big_endian(&mut self, big_endian: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (big_endian,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "set_big_endian", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_error(&self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_error", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_var_full(&self, allow_objects: bool,) -> Variant {
            type CallRet = Variant;
            type CallParams = (bool,);
            let args = (allow_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_var", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_var_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_var(&self,) -> Variant {
            self.get_var_ex() . done()
        }
        #[inline]
        pub fn get_var_ex < 'a > (&'a self,) -> ExGetVar < 'a > {
            ExGetVar::new(self,)
        }
        pub fn store_8(&mut self, value: u8,) -> bool {
            type CallRet = bool;
            type CallParams = (u8,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_8", Some(self.__validated_obj()), args,)
            }
        }
        pub fn store_16(&mut self, value: u16,) -> bool {
            type CallRet = bool;
            type CallParams = (u16,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_16", Some(self.__validated_obj()), args,)
            }
        }
        pub fn store_32(&mut self, value: u32,) -> bool {
            type CallRet = bool;
            type CallParams = (u32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_32", Some(self.__validated_obj()), args,)
            }
        }
        pub fn store_64(&mut self, value: u64,) -> bool {
            type CallRet = bool;
            type CallParams = (u64,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_64", Some(self.__validated_obj()), args,)
            }
        }
        pub fn store_half(&mut self, value: f32,) -> bool {
            type CallRet = bool;
            type CallParams = (f32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_half", Some(self.__validated_obj()), args,)
            }
        }
        pub fn store_float(&mut self, value: f32,) -> bool {
            type CallRet = bool;
            type CallParams = (f32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_float", Some(self.__validated_obj()), args,)
            }
        }
        pub fn store_double(&mut self, value: f64,) -> bool {
            type CallRet = bool;
            type CallParams = (f64,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_double", Some(self.__validated_obj()), args,)
            }
        }
        pub fn store_real(&mut self, value: f32,) -> bool {
            type CallRet = bool;
            type CallParams = (f32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_real", Some(self.__validated_obj()), args,)
            }
        }
        pub fn store_buffer(&mut self, buffer: &PackedByteArray,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn store_line(&mut self, line: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (line.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_line", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn store_csv_line_full(&mut self, values: RefArg < PackedStringArray >, delim: CowArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, PackedStringArray >, CowArg < 'a1, GString >,);
            let args = (values, delim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_csv_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::store_csv_line_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn store_csv_line(&mut self, values: &PackedStringArray,) -> bool {
            self.store_csv_line_ex(values,) . done()
        }
        #[inline]
        pub fn store_csv_line_ex < 'a > (&'a mut self, values: &'a PackedStringArray,) -> ExStoreCsvLine < 'a > {
            ExStoreCsvLine::new(self, values,)
        }
        pub fn store_string(&mut self, string: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_string", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn store_var_full(&mut self, value: RefArg < Variant >, full_objects: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >, bool,);
            let args = (value, full_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_var", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::store_var_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn store_var(&mut self, value: &Variant,) -> bool {
            self.store_var_ex(value,) . done()
        }
        #[inline]
        pub fn store_var_ex < 'a > (&'a mut self, value: &'a Variant,) -> ExStoreVar < 'a > {
            ExStoreVar::new(self, value,)
        }
        pub fn store_pascal_string(&mut self, string: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "store_pascal_string", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pascal_string(&mut self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_pascal_string", Some(self.__validated_obj()), args,)
            }
        }
        pub fn close(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "close", Some(self.__validated_obj()), args,)
            }
        }
        pub fn file_exists(path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "file_exists", None, args,)
            }
        }
        pub fn get_modified_time(file: impl AsArg < GString >,) -> u64 {
            type CallRet = u64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_modified_time", None, args,)
            }
        }
        pub fn get_access_time(file: impl AsArg < GString >,) -> u64 {
            type CallRet = u64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_access_time", None, args,)
            }
        }
        pub fn get_size(file: impl AsArg < GString >,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_size", None, args,)
            }
        }
        pub fn get_unix_permissions(file: impl AsArg < GString >,) -> crate::classes::file_access::UnixPermissionFlags {
            type CallRet = crate::classes::file_access::UnixPermissionFlags;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_unix_permissions", None, args,)
            }
        }
        pub fn set_unix_permissions(file: impl AsArg < GString >, permissions: crate::classes::file_access::UnixPermissionFlags,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::file_access::UnixPermissionFlags,);
            let args = (file.into_arg(), permissions,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "set_unix_permissions", None, args,)
            }
        }
        pub fn get_hidden_attribute(file: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_hidden_attribute", None, args,)
            }
        }
        pub fn set_hidden_attribute(file: impl AsArg < GString >, hidden: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (file.into_arg(), hidden,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "set_hidden_attribute", None, args,)
            }
        }
        pub fn set_read_only_attribute(file: impl AsArg < GString >, ro: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (file.into_arg(), ro,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "set_read_only_attribute", None, args,)
            }
        }
        pub fn get_read_only_attribute(file: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileAccess", "get_read_only_attribute", None, args,)
            }
        }
        #[doc = r" Creates a validated object for FFI boundary crossing."]
        #[doc = r""]
        #[doc = r" Low-level internal method. Validation (liveness/type checks) depend on safeguard level."]
        fn __validated_obj(&self) -> crate::obj::ValidatedObject {
            let raw_gd = unsafe {
                std::mem::transmute::< &Self, &crate::obj::RawGd < Self >> (self)
            };
            raw_gd.validated_object()
        }
        #[doc(hidden)]
        pub fn __object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
    }
    impl crate::obj::GodotClass for FileAccess {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("FileAccess"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for FileAccess {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for FileAccess {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for FileAccess {
        
    }
    impl std::ops::Deref for FileAccess {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FileAccess {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_FileAccess__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `FileAccess` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`FileAccess::open_encrypted_ex`][super::FileAccess::open_encrypted_ex]."]
#[must_use]
pub struct ExOpenEncrypted < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, path: CowArg < 'a, GString >, mode_flags: crate::classes::file_access::ModeFlags, key: CowArg < 'a, PackedByteArray >, iv: CowArg < 'a, PackedByteArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExOpenEncrypted < 'a > {
    fn new(path: impl AsArg < GString > + 'a, mode_flags: crate::classes::file_access::ModeFlags, key: &'a PackedByteArray,) -> Self {
        let iv = PackedByteArray::new();
        Self {
            _phantom: std::marker::PhantomData, path: path.into_arg(), mode_flags: mode_flags, key: CowArg::Borrowed(key), iv: CowArg::Owned(iv),
        }
    }
    #[inline]
    pub fn iv(self, iv: &'a PackedByteArray) -> Self {
        Self {
            iv: CowArg::Borrowed(iv), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::FileAccess > > {
        let Self {
            _phantom, path, mode_flags, key, iv,
        }
        = self;
        re_export::FileAccess::open_encrypted_full(path, mode_flags, key.cow_as_arg(), iv.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`FileAccess::open_compressed_ex`][super::FileAccess::open_compressed_ex]."]
#[must_use]
pub struct ExOpenCompressed < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, path: CowArg < 'a, GString >, mode_flags: crate::classes::file_access::ModeFlags, compression_mode: crate::classes::file_access::CompressionMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExOpenCompressed < 'a > {
    fn new(path: impl AsArg < GString > + 'a, mode_flags: crate::classes::file_access::ModeFlags,) -> Self {
        let compression_mode = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, path: path.into_arg(), mode_flags: mode_flags, compression_mode: compression_mode,
        }
    }
    #[inline]
    pub fn compression_mode(self, compression_mode: crate::classes::file_access::CompressionMode) -> Self {
        Self {
            compression_mode: compression_mode, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::FileAccess > > {
        let Self {
            _phantom, path, mode_flags, compression_mode,
        }
        = self;
        re_export::FileAccess::open_compressed_full(path, mode_flags, compression_mode,)
    }
}
#[doc = "Default-param extender for [`FileAccess::create_temp_ex`][super::FileAccess::create_temp_ex]."]
#[must_use]
pub struct ExCreateTemp < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, mode_flags: crate::classes::file_access::ModeFlags, prefix: CowArg < 'a, GString >, extension: CowArg < 'a, GString >, keep: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateTemp < 'a > {
    fn new(mode_flags: crate::classes::file_access::ModeFlags,) -> Self {
        let prefix = GString::from("");
        let extension = GString::from("");
        let keep = false;
        Self {
            _phantom: std::marker::PhantomData, mode_flags: mode_flags, prefix: CowArg::Owned(prefix), extension: CowArg::Owned(extension), keep: keep,
        }
    }
    #[inline]
    pub fn prefix(self, prefix: impl AsArg < GString > + 'a) -> Self {
        Self {
            prefix: prefix.into_arg(), .. self
        }
    }
    #[inline]
    pub fn extension(self, extension: impl AsArg < GString > + 'a) -> Self {
        Self {
            extension: extension.into_arg(), .. self
        }
    }
    #[inline]
    pub fn keep(self, keep: bool) -> Self {
        Self {
            keep: keep, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::FileAccess > > {
        let Self {
            _phantom, mode_flags, prefix, extension, keep,
        }
        = self;
        re_export::FileAccess::create_temp_full(mode_flags, prefix, extension, keep,)
    }
}
#[doc = "Default-param extender for [`FileAccess::seek_end_ex`][super::FileAccess::seek_end_ex]."]
#[must_use]
pub struct ExSeekEnd < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::FileAccess, position: i64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSeekEnd < 'a > {
    fn new(surround_object: &'a mut re_export::FileAccess,) -> Self {
        let position = 0i64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: i64) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position,
        }
        = self;
        re_export::FileAccess::seek_end_full(surround_object, position,)
    }
}
#[doc = "Default-param extender for [`FileAccess::get_csv_line_ex`][super::FileAccess::get_csv_line_ex]."]
#[must_use]
pub struct ExGetCsvLine < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::FileAccess, delim: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCsvLine < 'a > {
    fn new(surround_object: &'a re_export::FileAccess,) -> Self {
        let delim = GString::from(",");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, delim: CowArg::Owned(delim),
        }
    }
    #[inline]
    pub fn delim(self, delim: impl AsArg < GString > + 'a) -> Self {
        Self {
            delim: delim.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, surround_object, delim,
        }
        = self;
        re_export::FileAccess::get_csv_line_full(surround_object, delim,)
    }
}
#[doc = "Default-param extender for [`FileAccess::get_as_text_ex`][super::FileAccess::get_as_text_ex]."]
#[must_use]
pub struct ExGetAsText < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::FileAccess, skip_cr: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetAsText < 'a > {
    fn new(surround_object: &'a re_export::FileAccess,) -> Self {
        let skip_cr = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, skip_cr: skip_cr,
        }
    }
    #[inline]
    pub fn skip_cr(self, skip_cr: bool) -> Self {
        Self {
            skip_cr: skip_cr, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, skip_cr,
        }
        = self;
        re_export::FileAccess::get_as_text_full(surround_object, skip_cr,)
    }
}
#[doc = "Default-param extender for [`FileAccess::get_var_ex`][super::FileAccess::get_var_ex]."]
#[must_use]
pub struct ExGetVar < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::FileAccess, allow_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetVar < 'a > {
    fn new(surround_object: &'a re_export::FileAccess,) -> Self {
        let allow_objects = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, allow_objects: allow_objects,
        }
    }
    #[inline]
    pub fn allow_objects(self, allow_objects: bool) -> Self {
        Self {
            allow_objects: allow_objects, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, allow_objects,
        }
        = self;
        re_export::FileAccess::get_var_full(surround_object, allow_objects,)
    }
}
#[doc = "Default-param extender for [`FileAccess::store_csv_line_ex`][super::FileAccess::store_csv_line_ex]."]
#[must_use]
pub struct ExStoreCsvLine < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::FileAccess, values: CowArg < 'a, PackedStringArray >, delim: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStoreCsvLine < 'a > {
    fn new(surround_object: &'a mut re_export::FileAccess, values: &'a PackedStringArray,) -> Self {
        let delim = GString::from(",");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, values: CowArg::Borrowed(values), delim: CowArg::Owned(delim),
        }
    }
    #[inline]
    pub fn delim(self, delim: impl AsArg < GString > + 'a) -> Self {
        Self {
            delim: delim.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, values, delim,
        }
        = self;
        re_export::FileAccess::store_csv_line_full(surround_object, values.cow_as_arg(), delim,)
    }
}
#[doc = "Default-param extender for [`FileAccess::store_var_ex`][super::FileAccess::store_var_ex]."]
#[must_use]
pub struct ExStoreVar < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::FileAccess, value: CowArg < 'a, Variant >, full_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStoreVar < 'a > {
    fn new(surround_object: &'a mut re_export::FileAccess, value: &'a Variant,) -> Self {
        let full_objects = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, value: CowArg::Borrowed(value), full_objects: full_objects,
        }
    }
    #[inline]
    pub fn full_objects(self, full_objects: bool) -> Self {
        Self {
            full_objects: full_objects, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, value, full_objects,
        }
        = self;
        re_export::FileAccess::store_var_full(surround_object, value.cow_as_arg(), full_objects,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ModeFlags {
    ord: u64
}
impl ModeFlags {
    pub const READ: ModeFlags = ModeFlags {
        ord: 1u64
    };
    pub const WRITE: ModeFlags = ModeFlags {
        ord: 2u64
    };
    pub const READ_WRITE: ModeFlags = ModeFlags {
        ord: 3u64
    };
    pub const WRITE_READ: ModeFlags = ModeFlags {
        ord: 7u64
    };
    
}
impl std::fmt::Debug for ModeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::READ => "READ", Self::WRITE => "WRITE", Self::READ_WRITE => "READ_WRITE", Self::WRITE_READ => "WRITE_READ", _ => {
                f.debug_struct("ModeFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for ModeFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ModeFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("READ", "READ", ModeFlags::READ), crate::meta::inspect::EnumConstant::new("WRITE", "WRITE", ModeFlags::WRITE), crate::meta::inspect::EnumConstant::new("READ_WRITE", "READ_WRITE", ModeFlags::READ_WRITE), crate::meta::inspect::EnumConstant::new("WRITE_READ", "WRITE_READ", ModeFlags::WRITE_READ)]
        }
    }
}
impl std::ops::BitOr for ModeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for ModeFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for ModeFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for ModeFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ModeFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompressionMode {
    ord: i32
}
impl CompressionMode {
    #[doc(alias = "COMPRESSION_FASTLZ")]
    #[doc = "Godot enumerator name: `COMPRESSION_FASTLZ`"]
    pub const FASTLZ: CompressionMode = CompressionMode {
        ord: 0i32
    };
    #[doc(alias = "COMPRESSION_DEFLATE")]
    #[doc = "Godot enumerator name: `COMPRESSION_DEFLATE`"]
    pub const DEFLATE: CompressionMode = CompressionMode {
        ord: 1i32
    };
    #[doc(alias = "COMPRESSION_ZSTD")]
    #[doc = "Godot enumerator name: `COMPRESSION_ZSTD`"]
    pub const ZSTD: CompressionMode = CompressionMode {
        ord: 2i32
    };
    #[doc(alias = "COMPRESSION_GZIP")]
    #[doc = "Godot enumerator name: `COMPRESSION_GZIP`"]
    pub const GZIP: CompressionMode = CompressionMode {
        ord: 3i32
    };
    #[doc(alias = "COMPRESSION_BROTLI")]
    #[doc = "Godot enumerator name: `COMPRESSION_BROTLI`"]
    pub const BROTLI: CompressionMode = CompressionMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for CompressionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CompressionMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CompressionMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::FASTLZ => "FASTLZ", Self::DEFLATE => "DEFLATE", Self::ZSTD => "ZSTD", Self::GZIP => "GZIP", Self::BROTLI => "BROTLI", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CompressionMode::FASTLZ, CompressionMode::DEFLATE, CompressionMode::ZSTD, CompressionMode::GZIP, CompressionMode::BROTLI]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CompressionMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FASTLZ", "COMPRESSION_FASTLZ", CompressionMode::FASTLZ), crate::meta::inspect::EnumConstant::new("DEFLATE", "COMPRESSION_DEFLATE", CompressionMode::DEFLATE), crate::meta::inspect::EnumConstant::new("ZSTD", "COMPRESSION_ZSTD", CompressionMode::ZSTD), crate::meta::inspect::EnumConstant::new("GZIP", "COMPRESSION_GZIP", CompressionMode::GZIP), crate::meta::inspect::EnumConstant::new("BROTLI", "COMPRESSION_BROTLI", CompressionMode::BROTLI)]
        }
    }
}
impl crate::meta::GodotConvert for CompressionMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CompressionMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CompressionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct UnixPermissionFlags {
    ord: u64
}
impl UnixPermissionFlags {
    #[doc(alias = "UNIX_READ_OWNER")]
    #[doc = "Godot enumerator name: `UNIX_READ_OWNER`"]
    pub const READ_OWNER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 256u64
    };
    #[doc(alias = "UNIX_WRITE_OWNER")]
    #[doc = "Godot enumerator name: `UNIX_WRITE_OWNER`"]
    pub const WRITE_OWNER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 128u64
    };
    #[doc(alias = "UNIX_EXECUTE_OWNER")]
    #[doc = "Godot enumerator name: `UNIX_EXECUTE_OWNER`"]
    pub const EXECUTE_OWNER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 64u64
    };
    #[doc(alias = "UNIX_READ_GROUP")]
    #[doc = "Godot enumerator name: `UNIX_READ_GROUP`"]
    pub const READ_GROUP: UnixPermissionFlags = UnixPermissionFlags {
        ord: 32u64
    };
    #[doc(alias = "UNIX_WRITE_GROUP")]
    #[doc = "Godot enumerator name: `UNIX_WRITE_GROUP`"]
    pub const WRITE_GROUP: UnixPermissionFlags = UnixPermissionFlags {
        ord: 16u64
    };
    #[doc(alias = "UNIX_EXECUTE_GROUP")]
    #[doc = "Godot enumerator name: `UNIX_EXECUTE_GROUP`"]
    pub const EXECUTE_GROUP: UnixPermissionFlags = UnixPermissionFlags {
        ord: 8u64
    };
    #[doc(alias = "UNIX_READ_OTHER")]
    #[doc = "Godot enumerator name: `UNIX_READ_OTHER`"]
    pub const READ_OTHER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 4u64
    };
    #[doc(alias = "UNIX_WRITE_OTHER")]
    #[doc = "Godot enumerator name: `UNIX_WRITE_OTHER`"]
    pub const WRITE_OTHER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 2u64
    };
    #[doc(alias = "UNIX_EXECUTE_OTHER")]
    #[doc = "Godot enumerator name: `UNIX_EXECUTE_OTHER`"]
    pub const EXECUTE_OTHER: UnixPermissionFlags = UnixPermissionFlags {
        ord: 1u64
    };
    #[doc(alias = "UNIX_SET_USER_ID")]
    #[doc = "Godot enumerator name: `UNIX_SET_USER_ID`"]
    pub const SET_USER_ID: UnixPermissionFlags = UnixPermissionFlags {
        ord: 2048u64
    };
    #[doc(alias = "UNIX_SET_GROUP_ID")]
    #[doc = "Godot enumerator name: `UNIX_SET_GROUP_ID`"]
    pub const SET_GROUP_ID: UnixPermissionFlags = UnixPermissionFlags {
        ord: 1024u64
    };
    #[doc(alias = "UNIX_RESTRICTED_DELETE")]
    #[doc = "Godot enumerator name: `UNIX_RESTRICTED_DELETE`"]
    pub const RESTRICTED_DELETE: UnixPermissionFlags = UnixPermissionFlags {
        ord: 512u64
    };
    
}
impl std::fmt::Debug for UnixPermissionFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::READ_OWNER => "READ_OWNER", Self::WRITE_OWNER => "WRITE_OWNER", Self::EXECUTE_OWNER => "EXECUTE_OWNER", Self::READ_GROUP => "READ_GROUP", Self::WRITE_GROUP => "WRITE_GROUP", Self::EXECUTE_GROUP => "EXECUTE_GROUP", Self::READ_OTHER => "READ_OTHER", Self::WRITE_OTHER => "WRITE_OTHER", Self::EXECUTE_OTHER => "EXECUTE_OTHER", Self::SET_USER_ID => "SET_USER_ID", Self::SET_GROUP_ID => "SET_GROUP_ID", Self::RESTRICTED_DELETE => "RESTRICTED_DELETE", _ => {
                f.debug_struct("UnixPermissionFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for UnixPermissionFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < UnixPermissionFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("READ_OWNER", "UNIX_READ_OWNER", UnixPermissionFlags::READ_OWNER), crate::meta::inspect::EnumConstant::new("WRITE_OWNER", "UNIX_WRITE_OWNER", UnixPermissionFlags::WRITE_OWNER), crate::meta::inspect::EnumConstant::new("EXECUTE_OWNER", "UNIX_EXECUTE_OWNER", UnixPermissionFlags::EXECUTE_OWNER), crate::meta::inspect::EnumConstant::new("READ_GROUP", "UNIX_READ_GROUP", UnixPermissionFlags::READ_GROUP), crate::meta::inspect::EnumConstant::new("WRITE_GROUP", "UNIX_WRITE_GROUP", UnixPermissionFlags::WRITE_GROUP), crate::meta::inspect::EnumConstant::new("EXECUTE_GROUP", "UNIX_EXECUTE_GROUP", UnixPermissionFlags::EXECUTE_GROUP), crate::meta::inspect::EnumConstant::new("READ_OTHER", "UNIX_READ_OTHER", UnixPermissionFlags::READ_OTHER), crate::meta::inspect::EnumConstant::new("WRITE_OTHER", "UNIX_WRITE_OTHER", UnixPermissionFlags::WRITE_OTHER), crate::meta::inspect::EnumConstant::new("EXECUTE_OTHER", "UNIX_EXECUTE_OTHER", UnixPermissionFlags::EXECUTE_OTHER), crate::meta::inspect::EnumConstant::new("SET_USER_ID", "UNIX_SET_USER_ID", UnixPermissionFlags::SET_USER_ID), crate::meta::inspect::EnumConstant::new("SET_GROUP_ID", "UNIX_SET_GROUP_ID", UnixPermissionFlags::SET_GROUP_ID), crate::meta::inspect::EnumConstant::new("RESTRICTED_DELETE", "UNIX_RESTRICTED_DELETE", UnixPermissionFlags::RESTRICTED_DELETE)]
        }
    }
}
impl std::ops::BitOr for UnixPermissionFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for UnixPermissionFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for UnixPermissionFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for UnixPermissionFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for UnixPermissionFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::FileAccess;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for FileAccess {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfObject < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}