#![doc = "Sidecar module for class [`ResourceLoader`][crate::classes::ResourceLoader].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ResourceLoader` enums](https://docs.godotengine.org/en/stable/classes/class_resourceloader.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ResourceLoader.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`resource_loader`][crate::classes::resource_loader]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `ResourceLoader`](https://docs.godotengine.org/en/stable/classes/class_resourceloader.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ResourceLoader {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl ResourceLoader {
        pub(crate) fn load_full(&mut self, path: CowArg < GString >, type_hint: CowArg < GString >, cache_mode: crate::classes::resource_loader::CacheMode,) -> Option < Gd < crate::classes::Resource > > {
            type CallRet = Option < Gd < crate::classes::Resource > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, crate::classes::resource_loader::CacheMode,);
            let args = (path, type_hint, cache_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7463usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "load", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::load_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn load(&mut self, path: impl AsArg < GString >,) -> Option < Gd < crate::classes::Resource > > {
            self.load_ex(path,) . done()
        }
        #[inline]
        pub fn load_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a,) -> ExLoad < 'a > {
            ExLoad::new(self, path,)
        }
        pub fn get_recognized_extensions_for_type(&mut self, type_: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (type_.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7464usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "get_recognized_extensions_for_type", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_resource_format_loader_full(&mut self, format_loader: CowArg < Option < Gd < crate::classes::ResourceFormatLoader > > >, at_front: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ResourceFormatLoader > > >, bool,);
            let args = (format_loader, at_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7465usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "add_resource_format_loader", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_resource_format_loader_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_resource_format_loader(&mut self, format_loader: impl AsArg < Option < Gd < crate::classes::ResourceFormatLoader >> >,) {
            self.add_resource_format_loader_ex(format_loader,) . done()
        }
        #[inline]
        pub fn add_resource_format_loader_ex < 'a > (&'a mut self, format_loader: impl AsArg < Option < Gd < crate::classes::ResourceFormatLoader >> > + 'a,) -> ExAddResourceFormatLoader < 'a > {
            ExAddResourceFormatLoader::new(self, format_loader,)
        }
        pub fn remove_resource_format_loader(&mut self, format_loader: impl AsArg < Option < Gd < crate::classes::ResourceFormatLoader >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ResourceFormatLoader > > >,);
            let args = (format_loader.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7466usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "remove_resource_format_loader", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_abort_on_missing_resources(&mut self, abort: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (abort,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7467usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "set_abort_on_missing_resources", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_dependencies(&mut self, path: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7468usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "get_dependencies", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_cached(&mut self, path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7469usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "has_cached", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cached_ref(&mut self, path: impl AsArg < GString >,) -> Option < Gd < crate::classes::Resource > > {
            type CallRet = Option < Gd < crate::classes::Resource > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7470usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "get_cached_ref", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn exists_full(&mut self, path: CowArg < GString >, type_hint: CowArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (path, type_hint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7471usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "exists", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::exists_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn exists(&mut self, path: impl AsArg < GString >,) -> bool {
            self.exists_ex(path,) . done()
        }
        #[inline]
        pub fn exists_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a,) -> ExExists < 'a > {
            ExExists::new(self, path,)
        }
        pub fn get_resource_uid(&mut self, path: impl AsArg < GString >,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7472usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "get_resource_uid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn list_directory(&mut self, directory_path: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (directory_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7473usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceLoader", "list_directory", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ResourceLoader {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ResourceLoader"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ResourceLoader {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ResourceLoader {
        
    }
    impl crate::obj::Singleton for ResourceLoader {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"ResourceLoader"))
            }
        }
    }
    impl std::ops::Deref for ResourceLoader {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ResourceLoader {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ResourceLoader__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `ResourceLoader` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`ResourceLoader::load_ex`][super::ResourceLoader::load_ex]."]
#[must_use]
pub struct ExLoad < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ResourceLoader, path: CowArg < 'a, GString >, type_hint: CowArg < 'a, GString >, cache_mode: crate::classes::resource_loader::CacheMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLoad < 'a > {
    fn new(surround_object: &'a mut re_export::ResourceLoader, path: impl AsArg < GString > + 'a,) -> Self {
        let type_hint = GString::from("");
        let cache_mode = crate::obj::EngineEnum::from_ord(1);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), type_hint: CowArg::Owned(type_hint), cache_mode: cache_mode,
        }
    }
    #[inline]
    pub fn type_hint(self, type_hint: impl AsArg < GString > + 'a) -> Self {
        Self {
            type_hint: type_hint.into_arg(), .. self
        }
    }
    #[inline]
    pub fn cache_mode(self, cache_mode: crate::classes::resource_loader::CacheMode) -> Self {
        Self {
            cache_mode: cache_mode, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Resource > > {
        let Self {
            _phantom, surround_object, path, type_hint, cache_mode,
        }
        = self;
        re_export::ResourceLoader::load_full(surround_object, path, type_hint, cache_mode,)
    }
}
#[doc = "Default-param extender for [`ResourceLoader::add_resource_format_loader_ex`][super::ResourceLoader::add_resource_format_loader_ex]."]
#[must_use]
pub struct ExAddResourceFormatLoader < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ResourceLoader, format_loader: CowArg < 'a, Option < Gd < crate::classes::ResourceFormatLoader > > >, at_front: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddResourceFormatLoader < 'a > {
    fn new(surround_object: &'a mut re_export::ResourceLoader, format_loader: impl AsArg < Option < Gd < crate::classes::ResourceFormatLoader >> > + 'a,) -> Self {
        let at_front = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, format_loader: format_loader.into_arg(), at_front: at_front,
        }
    }
    #[inline]
    pub fn at_front(self, at_front: bool) -> Self {
        Self {
            at_front: at_front, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, format_loader, at_front,
        }
        = self;
        re_export::ResourceLoader::add_resource_format_loader_full(surround_object, format_loader, at_front,)
    }
}
#[doc = "Default-param extender for [`ResourceLoader::exists_ex`][super::ResourceLoader::exists_ex]."]
#[must_use]
pub struct ExExists < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ResourceLoader, path: CowArg < 'a, GString >, type_hint: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExExists < 'a > {
    fn new(surround_object: &'a mut re_export::ResourceLoader, path: impl AsArg < GString > + 'a,) -> Self {
        let type_hint = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), type_hint: CowArg::Owned(type_hint),
        }
    }
    #[inline]
    pub fn type_hint(self, type_hint: impl AsArg < GString > + 'a) -> Self {
        Self {
            type_hint: type_hint.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, path, type_hint,
        }
        = self;
        re_export::ResourceLoader::exists_full(surround_object, path, type_hint,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ThreadLoadStatus {
    ord: i32
}
impl ThreadLoadStatus {
    #[doc(alias = "THREAD_LOAD_INVALID_RESOURCE")]
    #[doc = "Godot enumerator name: `THREAD_LOAD_INVALID_RESOURCE`"]
    pub const INVALID_RESOURCE: ThreadLoadStatus = ThreadLoadStatus {
        ord: 0i32
    };
    #[doc(alias = "THREAD_LOAD_IN_PROGRESS")]
    #[doc = "Godot enumerator name: `THREAD_LOAD_IN_PROGRESS`"]
    pub const IN_PROGRESS: ThreadLoadStatus = ThreadLoadStatus {
        ord: 1i32
    };
    #[doc(alias = "THREAD_LOAD_FAILED")]
    #[doc = "Godot enumerator name: `THREAD_LOAD_FAILED`"]
    pub const FAILED: ThreadLoadStatus = ThreadLoadStatus {
        ord: 2i32
    };
    #[doc(alias = "THREAD_LOAD_LOADED")]
    #[doc = "Godot enumerator name: `THREAD_LOAD_LOADED`"]
    pub const LOADED: ThreadLoadStatus = ThreadLoadStatus {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ThreadLoadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ThreadLoadStatus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ThreadLoadStatus {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::INVALID_RESOURCE => "INVALID_RESOURCE", Self::IN_PROGRESS => "IN_PROGRESS", Self::FAILED => "FAILED", Self::LOADED => "LOADED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ThreadLoadStatus::INVALID_RESOURCE, ThreadLoadStatus::IN_PROGRESS, ThreadLoadStatus::FAILED, ThreadLoadStatus::LOADED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ThreadLoadStatus >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INVALID_RESOURCE", "THREAD_LOAD_INVALID_RESOURCE", ThreadLoadStatus::INVALID_RESOURCE), crate::meta::inspect::EnumConstant::new("IN_PROGRESS", "THREAD_LOAD_IN_PROGRESS", ThreadLoadStatus::IN_PROGRESS), crate::meta::inspect::EnumConstant::new("FAILED", "THREAD_LOAD_FAILED", ThreadLoadStatus::FAILED), crate::meta::inspect::EnumConstant::new("LOADED", "THREAD_LOAD_LOADED", ThreadLoadStatus::LOADED)]
        }
    }
}
impl crate::meta::GodotConvert for ThreadLoadStatus {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ThreadLoadStatus {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ThreadLoadStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CacheMode {
    ord: i32
}
impl CacheMode {
    #[doc(alias = "CACHE_MODE_IGNORE")]
    #[doc = "Godot enumerator name: `CACHE_MODE_IGNORE`"]
    pub const IGNORE: CacheMode = CacheMode {
        ord: 0i32
    };
    #[doc(alias = "CACHE_MODE_REUSE")]
    #[doc = "Godot enumerator name: `CACHE_MODE_REUSE`"]
    pub const REUSE: CacheMode = CacheMode {
        ord: 1i32
    };
    #[doc(alias = "CACHE_MODE_REPLACE")]
    #[doc = "Godot enumerator name: `CACHE_MODE_REPLACE`"]
    pub const REPLACE: CacheMode = CacheMode {
        ord: 2i32
    };
    #[doc(alias = "CACHE_MODE_IGNORE_DEEP")]
    #[doc = "Godot enumerator name: `CACHE_MODE_IGNORE_DEEP`"]
    pub const IGNORE_DEEP: CacheMode = CacheMode {
        ord: 3i32
    };
    #[doc(alias = "CACHE_MODE_REPLACE_DEEP")]
    #[doc = "Godot enumerator name: `CACHE_MODE_REPLACE_DEEP`"]
    pub const REPLACE_DEEP: CacheMode = CacheMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for CacheMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CacheMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CacheMode {
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
            Self::IGNORE => "IGNORE", Self::REUSE => "REUSE", Self::REPLACE => "REPLACE", Self::IGNORE_DEEP => "IGNORE_DEEP", Self::REPLACE_DEEP => "REPLACE_DEEP", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CacheMode::IGNORE, CacheMode::REUSE, CacheMode::REPLACE, CacheMode::IGNORE_DEEP, CacheMode::REPLACE_DEEP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CacheMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("IGNORE", "CACHE_MODE_IGNORE", CacheMode::IGNORE), crate::meta::inspect::EnumConstant::new("REUSE", "CACHE_MODE_REUSE", CacheMode::REUSE), crate::meta::inspect::EnumConstant::new("REPLACE", "CACHE_MODE_REPLACE", CacheMode::REPLACE), crate::meta::inspect::EnumConstant::new("IGNORE_DEEP", "CACHE_MODE_IGNORE_DEEP", CacheMode::IGNORE_DEEP), crate::meta::inspect::EnumConstant::new("REPLACE_DEEP", "CACHE_MODE_REPLACE_DEEP", CacheMode::REPLACE_DEEP)]
        }
    }
}
impl crate::meta::GodotConvert for CacheMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CacheMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CacheMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ResourceLoader;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for ResourceLoader {
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