#![doc = "Sidecar module for class [`TranslationServer`][crate::classes::TranslationServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TranslationServer` enums](https://docs.godotengine.org/en/stable/classes/class_translationserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TranslationServer.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`translation_server`][crate::classes::translation_server]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `TranslationServer`](https://docs.godotengine.org/en/stable/classes/class_translationserver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TranslationServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl TranslationServer {
        pub fn set_locale(&mut self, locale: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "set_locale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_locale(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_locale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tool_locale(&mut self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_tool_locale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn compare_locales(&self, locale_a: impl AsArg < GString >, locale_b: impl AsArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (locale_a.into_arg(), locale_b.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "compare_locales", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn standardize_locale_full(&self, locale: CowArg < GString >, add_defaults: bool,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (locale, add_defaults,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "standardize_locale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::standardize_locale_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn standardize_locale(&self, locale: impl AsArg < GString >,) -> GString {
            self.standardize_locale_ex(locale,) . done()
        }
        #[inline]
        pub fn standardize_locale_ex < 'a > (&'a self, locale: impl AsArg < GString > + 'a,) -> ExStandardizeLocale < 'a > {
            ExStandardizeLocale::new(self, locale,)
        }
        pub fn get_all_languages(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1280usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_all_languages", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_language_name(&self, language: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1281usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_language_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_all_scripts(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1282usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_all_scripts", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_script_name(&self, script: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1283usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_script_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_all_countries(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1284usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_all_countries", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_country_name(&self, country: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (country.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1285usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_country_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_locale_name(&self, locale: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1286usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_locale_name", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn translate_full(&self, message: CowArg < StringName >, context: CowArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (message, context,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1287usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "translate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::translate_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn translate(&self, message: impl AsArg < StringName >,) -> StringName {
            self.translate_ex(message,) . done()
        }
        #[inline]
        pub fn translate_ex < 'a > (&'a self, message: impl AsArg < StringName > + 'a,) -> ExTranslate < 'a > {
            ExTranslate::new(self, message,)
        }
        pub(crate) fn translate_plural_full(&self, message: CowArg < StringName >, plural_message: CowArg < StringName >, n: i32, context: CowArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, i32, CowArg < 'a2, StringName >,);
            let args = (message, plural_message, n, context,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1288usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "translate_plural", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::translate_plural_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn translate_plural(&self, message: impl AsArg < StringName >, plural_message: impl AsArg < StringName >, n: i32,) -> StringName {
            self.translate_plural_ex(message, plural_message, n,) . done()
        }
        #[inline]
        pub fn translate_plural_ex < 'a > (&'a self, message: impl AsArg < StringName > + 'a, plural_message: impl AsArg < StringName > + 'a, n: i32,) -> ExTranslatePlural < 'a > {
            ExTranslatePlural::new(self, message, plural_message, n,)
        }
        pub fn add_translation(&mut self, translation: impl AsArg < Option < Gd < crate::classes::Translation >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Translation > > >,);
            let args = (translation.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1289usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "add_translation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_translation(&mut self, translation: impl AsArg < Option < Gd < crate::classes::Translation >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Translation > > >,);
            let args = (translation.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1290usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "remove_translation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_translation_object(&mut self, locale: impl AsArg < GString >,) -> Option < Gd < crate::classes::Translation > > {
            type CallRet = Option < Gd < crate::classes::Translation > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1291usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_translation_object", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_domain(&self, domain: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (domain.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1292usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "has_domain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_or_add_domain(&mut self, domain: impl AsArg < StringName >,) -> Option < Gd < crate::classes::TranslationDomain > > {
            type CallRet = Option < Gd < crate::classes::TranslationDomain > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (domain.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1293usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_or_add_domain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_domain(&mut self, domain: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (domain.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1294usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "remove_domain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1295usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_loaded_locales(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1296usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "get_loaded_locales", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_pseudolocalization_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1297usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "is_pseudolocalization_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pseudolocalization_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1298usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "set_pseudolocalization_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reload_pseudolocalization(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1299usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "reload_pseudolocalization", Some(self.__validated_obj()), args,)
            }
        }
        pub fn pseudolocalize(&self, message: impl AsArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (message.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1300usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TranslationServer", "pseudolocalize", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TranslationServer {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TranslationServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for TranslationServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TranslationServer {
        
    }
    impl crate::obj::Singleton for TranslationServer {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"TranslationServer"))
            }
        }
    }
    impl std::ops::Deref for TranslationServer {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TranslationServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TranslationServer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `TranslationServer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`TranslationServer::standardize_locale_ex`][super::TranslationServer::standardize_locale_ex]."]
#[must_use]
pub struct ExStandardizeLocale < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TranslationServer, locale: CowArg < 'a, GString >, add_defaults: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStandardizeLocale < 'a > {
    fn new(surround_object: &'a re_export::TranslationServer, locale: impl AsArg < GString > + 'a,) -> Self {
        let add_defaults = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, locale: locale.into_arg(), add_defaults: add_defaults,
        }
    }
    #[inline]
    pub fn add_defaults(self, add_defaults: bool) -> Self {
        Self {
            add_defaults: add_defaults, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, locale, add_defaults,
        }
        = self;
        re_export::TranslationServer::standardize_locale_full(surround_object, locale, add_defaults,)
    }
}
#[doc = "Default-param extender for [`TranslationServer::translate_ex`][super::TranslationServer::translate_ex]."]
#[must_use]
pub struct ExTranslate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TranslationServer, message: CowArg < 'a, StringName >, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTranslate < 'a > {
    fn new(surround_object: &'a re_export::TranslationServer, message: impl AsArg < StringName > + 'a,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> StringName {
        let Self {
            _phantom, surround_object, message, context,
        }
        = self;
        re_export::TranslationServer::translate_full(surround_object, message, context,)
    }
}
#[doc = "Default-param extender for [`TranslationServer::translate_plural_ex`][super::TranslationServer::translate_plural_ex]."]
#[must_use]
pub struct ExTranslatePlural < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TranslationServer, message: CowArg < 'a, StringName >, plural_message: CowArg < 'a, StringName >, n: i32, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTranslatePlural < 'a > {
    fn new(surround_object: &'a re_export::TranslationServer, message: impl AsArg < StringName > + 'a, plural_message: impl AsArg < StringName > + 'a, n: i32,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), plural_message: plural_message.into_arg(), n: n, context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> StringName {
        let Self {
            _phantom, surround_object, message, plural_message, n, context,
        }
        = self;
        re_export::TranslationServer::translate_plural_full(surround_object, message, plural_message, n, context,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TranslationServer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for TranslationServer {
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