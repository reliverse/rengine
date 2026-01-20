#![doc = "Sidecar module for class [`ResourceSaver`][crate::classes::ResourceSaver].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ResourceSaver` enums](https://docs.godotengine.org/en/stable/classes/class_resourcesaver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ResourceSaver.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`resource_saver`][crate::classes::resource_saver]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `ResourceSaver`](https://docs.godotengine.org/en/stable/classes/class_resourcesaver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ResourceSaver {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl ResourceSaver {
        pub(crate) fn save_full(&mut self, resource: CowArg < Option < Gd < crate::classes::Resource > > >, path: CowArg < GString >, flags: crate::classes::resource_saver::SaverFlags,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Resource > > >, CowArg < 'a1, GString >, crate::classes::resource_saver::SaverFlags,);
            let args = (resource, path, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7480usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceSaver", "save", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::save_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn save(&mut self, resource: impl AsArg < Option < Gd < crate::classes::Resource >> >,) -> crate::global::Error {
            self.save_ex(resource,) . done()
        }
        #[inline]
        pub fn save_ex < 'a > (&'a mut self, resource: impl AsArg < Option < Gd < crate::classes::Resource >> > + 'a,) -> ExSave < 'a > {
            ExSave::new(self, resource,)
        }
        pub fn set_uid(&mut self, resource: impl AsArg < GString >, uid: i64,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i64,);
            let args = (resource.into_arg(), uid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7481usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceSaver", "set_uid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_recognized_extensions(&mut self, type_: impl AsArg < Option < Gd < crate::classes::Resource >> >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Resource > > >,);
            let args = (type_.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7482usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceSaver", "get_recognized_extensions", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_resource_format_saver_full(&mut self, format_saver: CowArg < Option < Gd < crate::classes::ResourceFormatSaver > > >, at_front: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ResourceFormatSaver > > >, bool,);
            let args = (format_saver, at_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7483usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceSaver", "add_resource_format_saver", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_resource_format_saver_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_resource_format_saver(&mut self, format_saver: impl AsArg < Option < Gd < crate::classes::ResourceFormatSaver >> >,) {
            self.add_resource_format_saver_ex(format_saver,) . done()
        }
        #[inline]
        pub fn add_resource_format_saver_ex < 'a > (&'a mut self, format_saver: impl AsArg < Option < Gd < crate::classes::ResourceFormatSaver >> > + 'a,) -> ExAddResourceFormatSaver < 'a > {
            ExAddResourceFormatSaver::new(self, format_saver,)
        }
        pub fn remove_resource_format_saver(&mut self, format_saver: impl AsArg < Option < Gd < crate::classes::ResourceFormatSaver >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ResourceFormatSaver > > >,);
            let args = (format_saver.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7484usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceSaver", "remove_resource_format_saver", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_resource_id_for_path_full(&mut self, path: CowArg < GString >, generate: bool,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (path, generate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7485usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ResourceSaver", "get_resource_id_for_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_resource_id_for_path_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_resource_id_for_path(&mut self, path: impl AsArg < GString >,) -> i64 {
            self.get_resource_id_for_path_ex(path,) . done()
        }
        #[inline]
        pub fn get_resource_id_for_path_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a,) -> ExGetResourceIdForPath < 'a > {
            ExGetResourceIdForPath::new(self, path,)
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
    impl crate::obj::GodotClass for ResourceSaver {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ResourceSaver"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ResourceSaver {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ResourceSaver {
        
    }
    impl crate::obj::Singleton for ResourceSaver {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"ResourceSaver"))
            }
        }
    }
    impl std::ops::Deref for ResourceSaver {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ResourceSaver {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ResourceSaver__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `ResourceSaver` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`ResourceSaver::save_ex`][super::ResourceSaver::save_ex]."]
#[must_use]
pub struct ExSave < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ResourceSaver, resource: CowArg < 'a, Option < Gd < crate::classes::Resource > > >, path: CowArg < 'a, GString >, flags: crate::classes::resource_saver::SaverFlags,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSave < 'a > {
    fn new(surround_object: &'a mut re_export::ResourceSaver, resource: impl AsArg < Option < Gd < crate::classes::Resource >> > + 'a,) -> Self {
        let path = GString::from("");
        let flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, resource: resource.into_arg(), path: CowArg::Owned(path), flags: flags,
        }
    }
    #[inline]
    pub fn path(self, path: impl AsArg < GString > + 'a) -> Self {
        Self {
            path: path.into_arg(), .. self
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::resource_saver::SaverFlags) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, resource, path, flags,
        }
        = self;
        re_export::ResourceSaver::save_full(surround_object, resource, path, flags,)
    }
}
#[doc = "Default-param extender for [`ResourceSaver::add_resource_format_saver_ex`][super::ResourceSaver::add_resource_format_saver_ex]."]
#[must_use]
pub struct ExAddResourceFormatSaver < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ResourceSaver, format_saver: CowArg < 'a, Option < Gd < crate::classes::ResourceFormatSaver > > >, at_front: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddResourceFormatSaver < 'a > {
    fn new(surround_object: &'a mut re_export::ResourceSaver, format_saver: impl AsArg < Option < Gd < crate::classes::ResourceFormatSaver >> > + 'a,) -> Self {
        let at_front = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, format_saver: format_saver.into_arg(), at_front: at_front,
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
            _phantom, surround_object, format_saver, at_front,
        }
        = self;
        re_export::ResourceSaver::add_resource_format_saver_full(surround_object, format_saver, at_front,)
    }
}
#[doc = "Default-param extender for [`ResourceSaver::get_resource_id_for_path_ex`][super::ResourceSaver::get_resource_id_for_path_ex]."]
#[must_use]
pub struct ExGetResourceIdForPath < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ResourceSaver, path: CowArg < 'a, GString >, generate: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetResourceIdForPath < 'a > {
    fn new(surround_object: &'a mut re_export::ResourceSaver, path: impl AsArg < GString > + 'a,) -> Self {
        let generate = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), generate: generate,
        }
    }
    #[inline]
    pub fn generate(self, generate: bool) -> Self {
        Self {
            generate: generate, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, path, generate,
        }
        = self;
        re_export::ResourceSaver::get_resource_id_for_path_full(surround_object, path, generate,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct SaverFlags {
    ord: u64
}
impl SaverFlags {
    #[doc(alias = "FLAG_NONE")]
    #[doc = "Godot enumerator name: `FLAG_NONE`"]
    pub const NONE: SaverFlags = SaverFlags {
        ord: 0u64
    };
    #[doc(alias = "FLAG_RELATIVE_PATHS")]
    #[doc = "Godot enumerator name: `FLAG_RELATIVE_PATHS`"]
    pub const RELATIVE_PATHS: SaverFlags = SaverFlags {
        ord: 1u64
    };
    #[doc(alias = "FLAG_BUNDLE_RESOURCES")]
    #[doc = "Godot enumerator name: `FLAG_BUNDLE_RESOURCES`"]
    pub const BUNDLE_RESOURCES: SaverFlags = SaverFlags {
        ord: 2u64
    };
    #[doc(alias = "FLAG_CHANGE_PATH")]
    #[doc = "Godot enumerator name: `FLAG_CHANGE_PATH`"]
    pub const CHANGE_PATH: SaverFlags = SaverFlags {
        ord: 4u64
    };
    #[doc(alias = "FLAG_OMIT_EDITOR_PROPERTIES")]
    #[doc = "Godot enumerator name: `FLAG_OMIT_EDITOR_PROPERTIES`"]
    pub const OMIT_EDITOR_PROPERTIES: SaverFlags = SaverFlags {
        ord: 8u64
    };
    #[doc(alias = "FLAG_SAVE_BIG_ENDIAN")]
    #[doc = "Godot enumerator name: `FLAG_SAVE_BIG_ENDIAN`"]
    pub const SAVE_BIG_ENDIAN: SaverFlags = SaverFlags {
        ord: 16u64
    };
    #[doc(alias = "FLAG_COMPRESS")]
    #[doc = "Godot enumerator name: `FLAG_COMPRESS`"]
    pub const COMPRESS: SaverFlags = SaverFlags {
        ord: 32u64
    };
    #[doc(alias = "FLAG_REPLACE_SUBRESOURCE_PATHS")]
    #[doc = "Godot enumerator name: `FLAG_REPLACE_SUBRESOURCE_PATHS`"]
    pub const REPLACE_SUBRESOURCE_PATHS: SaverFlags = SaverFlags {
        ord: 64u64
    };
    
}
impl std::fmt::Debug for SaverFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::NONE => "NONE", Self::RELATIVE_PATHS => "RELATIVE_PATHS", Self::BUNDLE_RESOURCES => "BUNDLE_RESOURCES", Self::CHANGE_PATH => "CHANGE_PATH", Self::OMIT_EDITOR_PROPERTIES => "OMIT_EDITOR_PROPERTIES", Self::SAVE_BIG_ENDIAN => "SAVE_BIG_ENDIAN", Self::COMPRESS => "COMPRESS", Self::REPLACE_SUBRESOURCE_PATHS => "REPLACE_SUBRESOURCE_PATHS", _ => {
                f.debug_struct("SaverFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for SaverFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SaverFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "FLAG_NONE", SaverFlags::NONE), crate::meta::inspect::EnumConstant::new("RELATIVE_PATHS", "FLAG_RELATIVE_PATHS", SaverFlags::RELATIVE_PATHS), crate::meta::inspect::EnumConstant::new("BUNDLE_RESOURCES", "FLAG_BUNDLE_RESOURCES", SaverFlags::BUNDLE_RESOURCES), crate::meta::inspect::EnumConstant::new("CHANGE_PATH", "FLAG_CHANGE_PATH", SaverFlags::CHANGE_PATH), crate::meta::inspect::EnumConstant::new("OMIT_EDITOR_PROPERTIES", "FLAG_OMIT_EDITOR_PROPERTIES", SaverFlags::OMIT_EDITOR_PROPERTIES), crate::meta::inspect::EnumConstant::new("SAVE_BIG_ENDIAN", "FLAG_SAVE_BIG_ENDIAN", SaverFlags::SAVE_BIG_ENDIAN), crate::meta::inspect::EnumConstant::new("COMPRESS", "FLAG_COMPRESS", SaverFlags::COMPRESS), crate::meta::inspect::EnumConstant::new("REPLACE_SUBRESOURCE_PATHS", "FLAG_REPLACE_SUBRESOURCE_PATHS", SaverFlags::REPLACE_SUBRESOURCE_PATHS)]
        }
    }
}
impl std::ops::BitOr for SaverFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for SaverFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for SaverFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for SaverFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SaverFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ResourceSaver;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for ResourceSaver {
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