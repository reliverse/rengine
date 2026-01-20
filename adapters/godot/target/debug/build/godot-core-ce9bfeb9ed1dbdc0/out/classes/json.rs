#![doc = "Sidecar module for class [`Json`][crate::classes::Json].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `JSON` enums](https://docs.godotengine.org/en/stable/classes/class_json.html#enumerations).\n\n"]
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
    #[doc = "Godot class `JSON.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`json`][crate::classes::json]: sidecar module with related enum/flag types\n* [`IJson`][crate::classes::IJson]: virtual methods\n\n\nSee also [Godot docs for `JSON`](https://docs.godotengine.org/en/stable/classes/class_json.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Json::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Json {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Json`][crate::classes::Json].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `JSON` methods](https://docs.godotengine.org/en/stable/classes/class_json.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IJson: crate::obj::GodotClass < Base = Json > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Json {
        pub(crate) fn stringify_full(data: RefArg < Variant >, indent: CowArg < GString >, sort_keys: bool, full_precision: bool,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, CowArg < 'a1, GString >, bool, bool,);
            let args = (data, indent, sort_keys, full_precision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4679usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Json", "stringify", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::stringify_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn stringify(data: &Variant,) -> GString {
            Self::stringify_ex(data,) . done()
        }
        #[inline]
        pub fn stringify_ex < 'a > (data: &'a Variant,) -> ExStringify < 'a > {
            ExStringify::new(data,)
        }
        pub fn parse_string(json_string: impl AsArg < GString >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (json_string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4680usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Json", "parse_string", None, args,)
            }
        }
        pub(crate) fn parse_full(&mut self, json_text: CowArg < GString >, keep_text: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (json_text, keep_text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4681usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Json", "parse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::parse_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn parse(&mut self, json_text: impl AsArg < GString >,) -> crate::global::Error {
            self.parse_ex(json_text,) . done()
        }
        #[inline]
        pub fn parse_ex < 'a > (&'a mut self, json_text: impl AsArg < GString > + 'a,) -> ExParse < 'a > {
            ExParse::new(self, json_text,)
        }
        pub fn get_data(&self,) -> Variant {
            type CallRet = Variant;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4682usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Json", "get_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_data(&mut self, data: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
            let args = (RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4683usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Json", "set_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_parsed_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4684usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Json", "get_parsed_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_error_line(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4685usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Json", "get_error_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_error_message(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4686usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Json", "get_error_message", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn from_native_full(variant: RefArg < Variant >, full_objects: bool,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >, bool,);
            let args = (variant, full_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4687usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Json", "from_native", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::from_native_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn from_native(variant: &Variant,) -> Variant {
            Self::from_native_ex(variant,) . done()
        }
        #[inline]
        pub fn from_native_ex < 'a > (variant: &'a Variant,) -> ExFromNative < 'a > {
            ExFromNative::new(variant,)
        }
        pub(crate) fn to_native_full(json: RefArg < Variant >, allow_objects: bool,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >, bool,);
            let args = (json, allow_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4688usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Json", "to_native", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::to_native_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn to_native(json: &Variant,) -> Variant {
            Self::to_native_ex(json,) . done()
        }
        #[inline]
        pub fn to_native_ex < 'a > (json: &'a Variant,) -> ExToNative < 'a > {
            ExToNative::new(json,)
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
    impl crate::obj::GodotClass for Json {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("JSON"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Json {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Json {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Json {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Json {
        
    }
    impl crate::obj::cap::GodotDefault for Json {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Json {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Json {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Json`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Json__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Json > for $Class {
                
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
#[doc = "Default-param extender for [`Json::stringify_ex`][super::Json::stringify_ex]."]
#[must_use]
pub struct ExStringify < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, data: CowArg < 'a, Variant >, indent: CowArg < 'a, GString >, sort_keys: bool, full_precision: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStringify < 'a > {
    fn new(data: &'a Variant,) -> Self {
        let indent = GString::from("");
        let sort_keys = true;
        let full_precision = false;
        Self {
            _phantom: std::marker::PhantomData, data: CowArg::Borrowed(data), indent: CowArg::Owned(indent), sort_keys: sort_keys, full_precision: full_precision,
        }
    }
    #[inline]
    pub fn indent(self, indent: impl AsArg < GString > + 'a) -> Self {
        Self {
            indent: indent.into_arg(), .. self
        }
    }
    #[inline]
    pub fn sort_keys(self, sort_keys: bool) -> Self {
        Self {
            sort_keys: sort_keys, .. self
        }
    }
    #[inline]
    pub fn full_precision(self, full_precision: bool) -> Self {
        Self {
            full_precision: full_precision, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, data, indent, sort_keys, full_precision,
        }
        = self;
        re_export::Json::stringify_full(data.cow_as_arg(), indent, sort_keys, full_precision,)
    }
}
#[doc = "Default-param extender for [`Json::parse_ex`][super::Json::parse_ex]."]
#[must_use]
pub struct ExParse < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Json, json_text: CowArg < 'a, GString >, keep_text: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExParse < 'a > {
    fn new(surround_object: &'a mut re_export::Json, json_text: impl AsArg < GString > + 'a,) -> Self {
        let keep_text = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, json_text: json_text.into_arg(), keep_text: keep_text,
        }
    }
    #[inline]
    pub fn keep_text(self, keep_text: bool) -> Self {
        Self {
            keep_text: keep_text, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, json_text, keep_text,
        }
        = self;
        re_export::Json::parse_full(surround_object, json_text, keep_text,)
    }
}
#[doc = "Default-param extender for [`Json::from_native_ex`][super::Json::from_native_ex]."]
#[must_use]
pub struct ExFromNative < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, variant: CowArg < 'a, Variant >, full_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFromNative < 'a > {
    fn new(variant: &'a Variant,) -> Self {
        let full_objects = false;
        Self {
            _phantom: std::marker::PhantomData, variant: CowArg::Borrowed(variant), full_objects: full_objects,
        }
    }
    #[inline]
    pub fn full_objects(self, full_objects: bool) -> Self {
        Self {
            full_objects: full_objects, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, variant, full_objects,
        }
        = self;
        re_export::Json::from_native_full(variant.cow_as_arg(), full_objects,)
    }
}
#[doc = "Default-param extender for [`Json::to_native_ex`][super::Json::to_native_ex]."]
#[must_use]
pub struct ExToNative < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, json: CowArg < 'a, Variant >, allow_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExToNative < 'a > {
    fn new(json: &'a Variant,) -> Self {
        let allow_objects = false;
        Self {
            _phantom: std::marker::PhantomData, json: CowArg::Borrowed(json), allow_objects: allow_objects,
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
            _phantom, json, allow_objects,
        }
        = self;
        re_export::Json::to_native_full(json.cow_as_arg(), allow_objects,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Json;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Json {
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