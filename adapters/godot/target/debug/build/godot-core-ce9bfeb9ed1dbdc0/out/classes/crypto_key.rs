#![doc = "Sidecar module for class [`CryptoKey`][crate::classes::CryptoKey].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CryptoKey` enums](https://docs.godotengine.org/en/stable/classes/class_cryptokey.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CryptoKey.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`crypto_key`][crate::classes::crypto_key]: sidecar module with related enum/flag types\n* [`ICryptoKey`][crate::classes::ICryptoKey]: virtual methods\n\n\nSee also [Godot docs for `CryptoKey`](https://docs.godotengine.org/en/stable/classes/class_cryptokey.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`CryptoKey::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CryptoKey {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`CryptoKey`][crate::classes::CryptoKey].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `CryptoKey` methods](https://docs.godotengine.org/en/stable/classes/class_cryptokey.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICryptoKey: crate::obj::GodotClass < Base = CryptoKey > + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `Base<T>` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::classes::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: ObjectNotification) {
            unimplemented !()
        }
        #[doc = r" Called whenever [`get()`](crate::classes::Object::get) is called or Godot gets the value of a property."]
        #[doc = r""]
        #[doc = r" Should return the given `property`'s value as `Some(value)`, or `None` if the property should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-get)."]
        fn get_property(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`set()`](crate::classes::Object::set) is called or Godot sets the value of a property."]
        #[doc = r""]
        #[doc = r" Should set `property` to the given `value` and return `true`, or return `false` to indicate the `property`"]
        #[doc = r" should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_set`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-set)."]
        fn set_property(&mut self, property: StringName, value: Variant) -> bool {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`get_property_list()`](crate::classes::Object::get_property_list) is called, the returned vector here is"]
        #[doc = r" appended to the existing list of properties."]
        #[doc = r""]
        #[doc = r" This should mainly be used for advanced purposes, such as dynamically updating the property list in the editor."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get_property_list`](https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-get-property-list)"]
        #[cfg(since_api = "4.3")]
        #[cfg_attr(published_docs, doc(cfg(since_api = "4.3")))]
        fn get_property_list(&mut self) -> Vec < crate::meta::PropertyInfo > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot retrieves value of property. Allows to customize existing properties."]
        #[doc = r" Every property info goes through this method, except properties **added** with `get_property_list()`."]
        #[doc = r""]
        #[doc = r" Exposed `property` here is a shared mutable reference obtained (and returned to) from Godot."]
        #[doc = r""]
        #[doc = r" See also in the Godot docs:"]
        #[doc = r" * [`Object::_validate_property`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-validate-property)"]
        fn validate_property(&self, property: &mut crate::meta::PropertyInfo) {
            unimplemented !()
        }
        #[doc = r" Called by Godot to tell if a property has a custom revert or not."]
        #[doc = r""]
        #[doc = r" Return `None` for no custom revert, and return `Some(value)` to specify the custom revert."]
        #[doc = r""]
        #[doc = r" This is a combination of Godot's [`Object::_property_get_revert`] and [`Object::_property_can_revert`]. This means that this"]
        #[doc = r" function will usually be called twice by Godot to find the revert."]
        #[doc = r""]
        #[doc = r" Note that this should be a _pure_ function. That is, it should always return the same value for a property as long as `self`"]
        #[doc = r" remains unchanged. Otherwise, this may lead to unexpected (safe) behavior."]
        #[doc = r""]
        #[doc = r" [`Object::_property_get_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-get-revert"]
        #[doc = r" [`Object::_property_can_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-can-revert"]
        #[doc(alias = "property_can_revert")]
        fn property_get_revert(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
        fn get_rid(&self,) -> Rid {
            unimplemented !()
        }
        fn reset_state(&mut self,) {
            unimplemented !()
        }
        fn set_path_cache(&self, path: GString,) {
            unimplemented !()
        }
    }
    impl CryptoKey {
        pub(crate) fn save_full(&mut self, path: CowArg < GString >, public_only: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (path, public_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CryptoKey", "save", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::save_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn save(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            self.save_ex(path,) . done()
        }
        #[inline]
        pub fn save_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a,) -> ExSave < 'a > {
            ExSave::new(self, path,)
        }
        pub(crate) fn load_full(&mut self, path: CowArg < GString >, public_only: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (path, public_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2673usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CryptoKey", "load", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::load_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn load(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            self.load_ex(path,) . done()
        }
        #[inline]
        pub fn load_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a,) -> ExLoad < 'a > {
            ExLoad::new(self, path,)
        }
        pub fn is_public_only(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2674usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CryptoKey", "is_public_only", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn save_to_string_full(&mut self, public_only: bool,) -> GString {
            type CallRet = GString;
            type CallParams = (bool,);
            let args = (public_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2675usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CryptoKey", "save_to_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::save_to_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn save_to_string(&mut self,) -> GString {
            self.save_to_string_ex() . done()
        }
        #[inline]
        pub fn save_to_string_ex < 'a > (&'a mut self,) -> ExSaveToString < 'a > {
            ExSaveToString::new(self,)
        }
        pub(crate) fn load_from_string_full(&mut self, string_key: CowArg < GString >, public_only: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (string_key, public_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2676usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CryptoKey", "load_from_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::load_from_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn load_from_string(&mut self, string_key: impl AsArg < GString >,) -> crate::global::Error {
            self.load_from_string_ex(string_key,) . done()
        }
        #[inline]
        pub fn load_from_string_ex < 'a > (&'a mut self, string_key: impl AsArg < GString > + 'a,) -> ExLoadFromString < 'a > {
            ExLoadFromString::new(self, string_key,)
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
    impl crate::obj::GodotClass for CryptoKey {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CryptoKey"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CryptoKey {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for CryptoKey {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for CryptoKey {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CryptoKey {
        
    }
    impl crate::obj::cap::GodotDefault for CryptoKey {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CryptoKey {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CryptoKey {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CryptoKey`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CryptoKey__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CryptoKey > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`CryptoKey::save_ex`][super::CryptoKey::save_ex]."]
#[must_use]
pub struct ExSave < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CryptoKey, path: CowArg < 'a, GString >, public_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSave < 'a > {
    fn new(surround_object: &'a mut re_export::CryptoKey, path: impl AsArg < GString > + 'a,) -> Self {
        let public_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), public_only: public_only,
        }
    }
    #[inline]
    pub fn public_only(self, public_only: bool) -> Self {
        Self {
            public_only: public_only, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, path, public_only,
        }
        = self;
        re_export::CryptoKey::save_full(surround_object, path, public_only,)
    }
}
#[doc = "Default-param extender for [`CryptoKey::load_ex`][super::CryptoKey::load_ex]."]
#[must_use]
pub struct ExLoad < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CryptoKey, path: CowArg < 'a, GString >, public_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLoad < 'a > {
    fn new(surround_object: &'a mut re_export::CryptoKey, path: impl AsArg < GString > + 'a,) -> Self {
        let public_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), public_only: public_only,
        }
    }
    #[inline]
    pub fn public_only(self, public_only: bool) -> Self {
        Self {
            public_only: public_only, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, path, public_only,
        }
        = self;
        re_export::CryptoKey::load_full(surround_object, path, public_only,)
    }
}
#[doc = "Default-param extender for [`CryptoKey::save_to_string_ex`][super::CryptoKey::save_to_string_ex]."]
#[must_use]
pub struct ExSaveToString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CryptoKey, public_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveToString < 'a > {
    fn new(surround_object: &'a mut re_export::CryptoKey,) -> Self {
        let public_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, public_only: public_only,
        }
    }
    #[inline]
    pub fn public_only(self, public_only: bool) -> Self {
        Self {
            public_only: public_only, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, public_only,
        }
        = self;
        re_export::CryptoKey::save_to_string_full(surround_object, public_only,)
    }
}
#[doc = "Default-param extender for [`CryptoKey::load_from_string_ex`][super::CryptoKey::load_from_string_ex]."]
#[must_use]
pub struct ExLoadFromString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CryptoKey, string_key: CowArg < 'a, GString >, public_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLoadFromString < 'a > {
    fn new(surround_object: &'a mut re_export::CryptoKey, string_key: impl AsArg < GString > + 'a,) -> Self {
        let public_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string_key: string_key.into_arg(), public_only: public_only,
        }
    }
    #[inline]
    pub fn public_only(self, public_only: bool) -> Self {
        Self {
            public_only: public_only, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, string_key, public_only,
        }
        = self;
        re_export::CryptoKey::load_from_string_full(surround_object, string_key, public_only,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CryptoKey;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for CryptoKey {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfResource < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}