#![doc = "Sidecar module for class [`DirAccess`][crate::classes::DirAccess].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `DirAccess` enums](https://docs.godotengine.org/en/stable/classes/class_diraccess.html#enumerations).\n\n"]
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
    #[doc = "Godot class `DirAccess.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`dir_access`][crate::classes::dir_access]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `DirAccess`](https://docs.godotengine.org/en/stable/classes/class_diraccess.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<DirAccess>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct DirAccess {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl DirAccess {
        pub fn open(path: impl AsArg < GString >,) -> Option < Gd < crate::classes::DirAccess > > {
            type CallRet = Option < Gd < crate::classes::DirAccess > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2841usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "open", None, args,)
            }
        }
        pub fn get_open_error() -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2842usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_open_error", None, args,)
            }
        }
        pub(crate) fn create_temp_full(prefix: CowArg < GString >, keep: bool,) -> Option < Gd < crate::classes::DirAccess > > {
            type CallRet = Option < Gd < crate::classes::DirAccess > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (prefix, keep,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2843usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "create_temp", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_temp_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_temp() -> Option < Gd < crate::classes::DirAccess > > {
            Self::create_temp_ex() . done()
        }
        #[inline]
        pub fn create_temp_ex < 'a > () -> ExCreateTemp < 'a > {
            ExCreateTemp::new()
        }
        pub fn list_dir_begin(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2844usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "list_dir_begin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_next(&mut self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2845usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_next", Some(self.__validated_obj()), args,)
            }
        }
        pub fn current_is_dir(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2846usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "current_is_dir", Some(self.__validated_obj()), args,)
            }
        }
        pub fn list_dir_end(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2847usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "list_dir_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_files(&mut self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2848usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_files", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_files_at(path: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2849usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_files_at", None, args,)
            }
        }
        pub fn get_directories(&mut self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2850usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_directories", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_directories_at(path: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2851usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_directories_at", None, args,)
            }
        }
        pub fn get_drive_count() -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2852usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_drive_count", None, args,)
            }
        }
        pub fn get_drive_name(idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2853usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_drive_name", None, args,)
            }
        }
        pub fn get_current_drive(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2854usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_current_drive", Some(self.__validated_obj()), args,)
            }
        }
        pub fn change_dir(&mut self, to_dir: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (to_dir.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2855usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "change_dir", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_current_dir_full(&self, include_drive: bool,) -> GString {
            type CallRet = GString;
            type CallParams = (bool,);
            let args = (include_drive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2856usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_current_dir", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_current_dir_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_current_dir(&self,) -> GString {
            self.get_current_dir_ex() . done()
        }
        #[inline]
        pub fn get_current_dir_ex < 'a > (&'a self,) -> ExGetCurrentDir < 'a > {
            ExGetCurrentDir::new(self,)
        }
        pub fn make_dir(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2857usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "make_dir", Some(self.__validated_obj()), args,)
            }
        }
        pub fn make_dir_absolute(path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2858usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "make_dir_absolute", None, args,)
            }
        }
        pub fn make_dir_recursive(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "make_dir_recursive", Some(self.__validated_obj()), args,)
            }
        }
        pub fn make_dir_recursive_absolute(path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "make_dir_recursive_absolute", None, args,)
            }
        }
        pub fn file_exists(&mut self, path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2861usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "file_exists", Some(self.__validated_obj()), args,)
            }
        }
        pub fn dir_exists(&mut self, path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2862usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "dir_exists", Some(self.__validated_obj()), args,)
            }
        }
        pub fn dir_exists_absolute(path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2863usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "dir_exists_absolute", None, args,)
            }
        }
        pub fn get_space_left(&mut self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2864usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_space_left", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn copy_full(&mut self, from: CowArg < GString >, to: CowArg < GString >, chmod_flags: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32,);
            let args = (from, to, chmod_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2865usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "copy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::copy_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn copy(&mut self, from: impl AsArg < GString >, to: impl AsArg < GString >,) -> crate::global::Error {
            self.copy_ex(from, to,) . done()
        }
        #[inline]
        pub fn copy_ex < 'a > (&'a mut self, from: impl AsArg < GString > + 'a, to: impl AsArg < GString > + 'a,) -> ExCopy < 'a > {
            ExCopy::new(self, from, to,)
        }
        pub(crate) fn copy_absolute_full(from: CowArg < GString >, to: CowArg < GString >, chmod_flags: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32,);
            let args = (from, to, chmod_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2866usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "copy_absolute", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::copy_absolute_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn copy_absolute(from: impl AsArg < GString >, to: impl AsArg < GString >,) -> crate::global::Error {
            Self::copy_absolute_ex(from, to,) . done()
        }
        #[inline]
        pub fn copy_absolute_ex < 'a > (from: impl AsArg < GString > + 'a, to: impl AsArg < GString > + 'a,) -> ExCopyAbsolute < 'a > {
            ExCopyAbsolute::new(from, to,)
        }
        pub fn rename(&mut self, from: impl AsArg < GString >, to: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (from.into_arg(), to.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2867usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "rename", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rename_absolute(from: impl AsArg < GString >, to: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (from.into_arg(), to.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2868usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "rename_absolute", None, args,)
            }
        }
        pub fn remove(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2869usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "remove", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_absolute(path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2870usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "remove_absolute", None, args,)
            }
        }
        pub fn is_link(&mut self, path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2871usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "is_link", Some(self.__validated_obj()), args,)
            }
        }
        pub fn read_link(&mut self, path: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2872usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "read_link", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_link(&mut self, source: impl AsArg < GString >, target: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (source.into_arg(), target.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2873usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "create_link", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_bundle(&self, path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2874usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "is_bundle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_include_navigational(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2875usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "set_include_navigational", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_include_navigational(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2876usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_include_navigational", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_include_hidden(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2877usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "set_include_hidden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_include_hidden(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2878usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_include_hidden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_filesystem_type(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2879usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "get_filesystem_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_case_sensitive(&self, path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2880usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "is_case_sensitive", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_equivalent(&self, path_a: impl AsArg < GString >, path_b: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (path_a.into_arg(), path_b.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2881usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DirAccess", "is_equivalent", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for DirAccess {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("DirAccess"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for DirAccess {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for DirAccess {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for DirAccess {
        
    }
    impl std::ops::Deref for DirAccess {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for DirAccess {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_DirAccess__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `DirAccess` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`DirAccess::create_temp_ex`][super::DirAccess::create_temp_ex]."]
#[must_use]
pub struct ExCreateTemp < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, prefix: CowArg < 'a, GString >, keep: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateTemp < 'a > {
    fn new() -> Self {
        let prefix = GString::from("");
        let keep = false;
        Self {
            _phantom: std::marker::PhantomData, prefix: CowArg::Owned(prefix), keep: keep,
        }
    }
    #[inline]
    pub fn prefix(self, prefix: impl AsArg < GString > + 'a) -> Self {
        Self {
            prefix: prefix.into_arg(), .. self
        }
    }
    #[inline]
    pub fn keep(self, keep: bool) -> Self {
        Self {
            keep: keep, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::DirAccess > > {
        let Self {
            _phantom, prefix, keep,
        }
        = self;
        re_export::DirAccess::create_temp_full(prefix, keep,)
    }
}
#[doc = "Default-param extender for [`DirAccess::get_current_dir_ex`][super::DirAccess::get_current_dir_ex]."]
#[must_use]
pub struct ExGetCurrentDir < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::DirAccess, include_drive: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCurrentDir < 'a > {
    fn new(surround_object: &'a re_export::DirAccess,) -> Self {
        let include_drive = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, include_drive: include_drive,
        }
    }
    #[inline]
    pub fn include_drive(self, include_drive: bool) -> Self {
        Self {
            include_drive: include_drive, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, include_drive,
        }
        = self;
        re_export::DirAccess::get_current_dir_full(surround_object, include_drive,)
    }
}
#[doc = "Default-param extender for [`DirAccess::copy_ex`][super::DirAccess::copy_ex]."]
#[must_use]
pub struct ExCopy < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::DirAccess, from: CowArg < 'a, GString >, to: CowArg < 'a, GString >, chmod_flags: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCopy < 'a > {
    fn new(surround_object: &'a mut re_export::DirAccess, from: impl AsArg < GString > + 'a, to: impl AsArg < GString > + 'a,) -> Self {
        let chmod_flags = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from.into_arg(), to: to.into_arg(), chmod_flags: chmod_flags,
        }
    }
    #[inline]
    pub fn chmod_flags(self, chmod_flags: i32) -> Self {
        Self {
            chmod_flags: chmod_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, from, to, chmod_flags,
        }
        = self;
        re_export::DirAccess::copy_full(surround_object, from, to, chmod_flags,)
    }
}
#[doc = "Default-param extender for [`DirAccess::copy_absolute_ex`][super::DirAccess::copy_absolute_ex]."]
#[must_use]
pub struct ExCopyAbsolute < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, from: CowArg < 'a, GString >, to: CowArg < 'a, GString >, chmod_flags: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCopyAbsolute < 'a > {
    fn new(from: impl AsArg < GString > + 'a, to: impl AsArg < GString > + 'a,) -> Self {
        let chmod_flags = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, from: from.into_arg(), to: to.into_arg(), chmod_flags: chmod_flags,
        }
    }
    #[inline]
    pub fn chmod_flags(self, chmod_flags: i32) -> Self {
        Self {
            chmod_flags: chmod_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, from, to, chmod_flags,
        }
        = self;
        re_export::DirAccess::copy_absolute_full(from, to, chmod_flags,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::DirAccess;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for DirAccess {
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