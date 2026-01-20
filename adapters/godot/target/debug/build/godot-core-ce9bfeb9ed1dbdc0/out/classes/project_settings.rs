#![doc = "Sidecar module for class [`ProjectSettings`][crate::classes::ProjectSettings].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ProjectSettings` enums](https://docs.godotengine.org/en/stable/classes/class_projectsettings.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ProjectSettings.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`project_settings`][crate::classes::project_settings]: sidecar module with related enum/flag types\n* [`SignalsOfProjectSettings`][crate::classes::project_settings::SignalsOfProjectSettings]: signal collection\n\n\nSee also [Godot docs for `ProjectSettings`](https://docs.godotengine.org/en/stable/classes/class_projectsettings.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ProjectSettings {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl ProjectSettings {
        pub fn has_setting(&self, name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "has_setting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_setting(&mut self, name: impl AsArg < GString >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_setting", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_setting_full(&self, name: CowArg < GString >, default_value: RefArg < Variant >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, Variant >,);
            let args = (name, default_value,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "get_setting", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_setting_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_setting(&self, name: impl AsArg < GString >,) -> Variant {
            self.get_setting_ex(name,) . done()
        }
        #[inline]
        pub fn get_setting_ex < 'a > (&'a self, name: impl AsArg < GString > + 'a,) -> ExGetSetting < 'a > {
            ExGetSetting::new(self, name,)
        }
        pub fn get_setting_with_override(&self, name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "get_setting_with_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_class_list(&mut self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "get_global_class_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_setting_with_override_and_custom_features(&self, name: impl AsArg < StringName >, features: &PackedStringArray,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, PackedStringArray >,);
            let args = (name.into_arg(), RefArg::new(features),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "get_setting_with_override_and_custom_features", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_order(&mut self, name: impl AsArg < GString >, position: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (name.into_arg(), position,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_order", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_order(&self, name: impl AsArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "get_order", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_initial_value(&mut self, name: impl AsArg < GString >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_initial_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_as_basic(&mut self, name: impl AsArg < GString >, basic: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (name.into_arg(), basic,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_as_basic", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_as_internal(&mut self, name: impl AsArg < GString >, internal: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (name.into_arg(), internal,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_as_internal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_property_info(&mut self, hint: &VarDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >,);
            let args = (RefArg::new(hint),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "add_property_info", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_restart_if_changed(&mut self, name: impl AsArg < GString >, restart: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (name.into_arg(), restart,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "set_restart_if_changed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn localize_path(&self, path: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "localize_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn globalize_path(&self, path: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "globalize_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn save(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "save", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn load_resource_pack_full(&mut self, pack: CowArg < GString >, replace_files: bool, offset: i32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool, i32,);
            let args = (pack, replace_files, offset,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "load_resource_pack", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::load_resource_pack_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn load_resource_pack(&mut self, pack: impl AsArg < GString >,) -> bool {
            self.load_resource_pack_ex(pack,) . done()
        }
        #[inline]
        pub fn load_resource_pack_ex < 'a > (&'a mut self, pack: impl AsArg < GString > + 'a,) -> ExLoadResourcePack < 'a > {
            ExLoadResourcePack::new(self, pack,)
        }
        pub fn save_custom(&mut self, file: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProjectSettings", "save_custom", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ProjectSettings {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ProjectSettings"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Core;
        
    }
    unsafe impl crate::obj::Bounds for ProjectSettings {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ProjectSettings {
        
    }
    impl crate::obj::Singleton for ProjectSettings {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"ProjectSettings"))
            }
        }
    }
    impl std::ops::Deref for ProjectSettings {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ProjectSettings {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ProjectSettings__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `ProjectSettings` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`ProjectSettings::get_setting_ex`][super::ProjectSettings::get_setting_ex]."]
#[must_use]
pub struct ExGetSetting < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ProjectSettings, name: CowArg < 'a, GString >, default_value: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSetting < 'a > {
    fn new(surround_object: &'a re_export::ProjectSettings, name: impl AsArg < GString > + 'a,) -> Self {
        let default_value = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), default_value: CowArg::Owned(default_value),
        }
    }
    #[inline]
    pub fn default_value(self, default_value: &'a Variant) -> Self {
        Self {
            default_value: CowArg::Borrowed(default_value), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, name, default_value,
        }
        = self;
        re_export::ProjectSettings::get_setting_full(surround_object, name, default_value.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`ProjectSettings::load_resource_pack_ex`][super::ProjectSettings::load_resource_pack_ex]."]
#[must_use]
pub struct ExLoadResourcePack < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ProjectSettings, pack: CowArg < 'a, GString >, replace_files: bool, offset: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLoadResourcePack < 'a > {
    fn new(surround_object: &'a mut re_export::ProjectSettings, pack: impl AsArg < GString > + 'a,) -> Self {
        let replace_files = true;
        let offset = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, pack: pack.into_arg(), replace_files: replace_files, offset: offset,
        }
    }
    #[inline]
    pub fn replace_files(self, replace_files: bool) -> Self {
        Self {
            replace_files: replace_files, .. self
        }
    }
    #[inline]
    pub fn offset(self, offset: i32) -> Self {
        Self {
            offset: offset, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, pack, replace_files, offset,
        }
        = self;
        re_export::ProjectSettings::load_resource_pack_full(surround_object, pack, replace_files, offset,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ProjectSettings;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`ProjectSettings`][crate::classes::ProjectSettings] class."]
    pub struct SignalsOfProjectSettings < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfProjectSettings < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn settings_changed(&mut self) -> SigSettingsChanged < 'c, C > {
            SigSettingsChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "settings_changed")
            }
        }
    }
    type TypedSigSettingsChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSettingsChanged < 'c, C: WithSignals > {
        typed: TypedSigSettingsChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSettingsChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSettingsChanged < 'c, C > {
        type Target = TypedSigSettingsChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSettingsChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for ProjectSettings {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfProjectSettings < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfProjectSettings < 'c, C > {
        type Target = < < ProjectSettings as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = ProjectSettings;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfProjectSettings < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = ProjectSettings;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}