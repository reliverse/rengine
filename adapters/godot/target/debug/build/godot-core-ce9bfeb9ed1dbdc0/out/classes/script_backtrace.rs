#![doc = "Sidecar module for class [`ScriptBacktrace`][crate::classes::ScriptBacktrace].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ScriptBacktrace` enums](https://docs.godotengine.org/en/stable/classes/class_scriptbacktrace.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ScriptBacktrace.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`script_backtrace`][crate::classes::script_backtrace]: sidecar module with related enum/flag types\n* [`IScriptBacktrace`][crate::classes::IScriptBacktrace]: virtual methods\n\n\nSee also [Godot docs for `ScriptBacktrace`](https://docs.godotengine.org/en/stable/classes/class_scriptbacktrace.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ScriptBacktrace::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ScriptBacktrace {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ScriptBacktrace`][crate::classes::ScriptBacktrace].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `ScriptBacktrace` methods](https://docs.godotengine.org/en/stable/classes/class_scriptbacktrace.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IScriptBacktrace: crate::obj::GodotClass < Base = ScriptBacktrace > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl ScriptBacktrace {
        pub fn get_language_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7911usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptBacktrace", "get_language_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_empty(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7912usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptBacktrace", "is_empty", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frame_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7913usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptBacktrace", "get_frame_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frame_function(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7914usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptBacktrace", "get_frame_function", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frame_file(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7915usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptBacktrace", "get_frame_file", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frame_line(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7916usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptBacktrace", "get_frame_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_variable_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7917usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptBacktrace", "get_global_variable_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_variable_name(&self, variable_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (variable_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7918usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptBacktrace", "get_global_variable_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_variable_value(&self, variable_index: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (variable_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7919usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptBacktrace", "get_global_variable_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_local_variable_count(&self, frame_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (frame_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7920usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptBacktrace", "get_local_variable_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_local_variable_name(&self, frame_index: i32, variable_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32, i32,);
            let args = (frame_index, variable_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7921usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptBacktrace", "get_local_variable_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_local_variable_value(&self, frame_index: i32, variable_index: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32, i32,);
            let args = (frame_index, variable_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7922usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptBacktrace", "get_local_variable_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_member_variable_count(&self, frame_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (frame_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7923usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptBacktrace", "get_member_variable_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_member_variable_name(&self, frame_index: i32, variable_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32, i32,);
            let args = (frame_index, variable_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7924usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptBacktrace", "get_member_variable_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_member_variable_value(&self, frame_index: i32, variable_index: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32, i32,);
            let args = (frame_index, variable_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7925usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptBacktrace", "get_member_variable_value", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn format_full(&self, indent_all: i32, indent_frames: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32, i32,);
            let args = (indent_all, indent_frames,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7926usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptBacktrace", "format", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::format_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn format(&self,) -> GString {
            self.format_ex() . done()
        }
        #[inline]
        pub fn format_ex < 'a > (&'a self,) -> ExFormat < 'a > {
            ExFormat::new(self,)
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
    impl crate::obj::GodotClass for ScriptBacktrace {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ScriptBacktrace"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ScriptBacktrace {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ScriptBacktrace {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ScriptBacktrace {
        
    }
    impl crate::obj::cap::GodotDefault for ScriptBacktrace {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ScriptBacktrace {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ScriptBacktrace {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ScriptBacktrace`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ScriptBacktrace__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ScriptBacktrace > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ScriptBacktrace::format_ex`][super::ScriptBacktrace::format_ex]."]
#[must_use]
pub struct ExFormat < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ScriptBacktrace, indent_all: i32, indent_frames: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFormat < 'a > {
    fn new(surround_object: &'a re_export::ScriptBacktrace,) -> Self {
        let indent_all = 0i32;
        let indent_frames = 4i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, indent_all: indent_all, indent_frames: indent_frames,
        }
    }
    #[inline]
    pub fn indent_all(self, indent_all: i32) -> Self {
        Self {
            indent_all: indent_all, .. self
        }
    }
    #[inline]
    pub fn indent_frames(self, indent_frames: i32) -> Self {
        Self {
            indent_frames: indent_frames, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, indent_all, indent_frames,
        }
        = self;
        re_export::ScriptBacktrace::format_full(surround_object, indent_all, indent_frames,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ScriptBacktrace;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for ScriptBacktrace {
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