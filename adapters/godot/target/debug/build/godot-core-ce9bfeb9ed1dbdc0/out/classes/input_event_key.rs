#![doc = "Sidecar module for class [`InputEventKey`][crate::classes::InputEventKey].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputEventKey` enums](https://docs.godotengine.org/en/stable/classes/class_inputeventkey.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputEventKey.`\n\nInherits [`InputEventWithModifiers`][crate::classes::InputEventWithModifiers].\n\nRelated symbols:\n\n* [`IInputEventKey`][crate::classes::IInputEventKey]: virtual methods\n\n\nSee also [Godot docs for `InputEventKey`](https://docs.godotengine.org/en/stable/classes/class_inputeventkey.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`InputEventKey::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputEventKey {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`InputEventKey`][crate::classes::InputEventKey].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IInputEventWithModifiers`~~ > ~~`IInputEventFromWindow`~~ > ~~`IInputEvent`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `InputEventKey` methods](https://docs.godotengine.org/en/stable/classes/class_inputeventkey.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IInputEventKey: crate::obj::GodotClass < Base = InputEventKey > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl InputEventKey {
        pub fn set_pressed(&mut self, pressed: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4475usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "set_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_keycode(&mut self, keycode: crate::global::Key,) {
            type CallRet = ();
            type CallParams = (crate::global::Key,);
            let args = (keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4476usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "set_keycode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_keycode(&self,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4477usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "get_keycode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physical_keycode(&mut self, physical_keycode: crate::global::Key,) {
            type CallRet = ();
            type CallParams = (crate::global::Key,);
            let args = (physical_keycode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4478usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "set_physical_keycode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physical_keycode(&self,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4479usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "get_physical_keycode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_key_label(&mut self, key_label: crate::global::Key,) {
            type CallRet = ();
            type CallParams = (crate::global::Key,);
            let args = (key_label,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4480usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "set_key_label", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_key_label(&self,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4481usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "get_key_label", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_unicode(&mut self, unicode: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (unicode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4482usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "set_unicode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_unicode(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4483usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "get_unicode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_location(&mut self, location: crate::global::KeyLocation,) {
            type CallRet = ();
            type CallParams = (crate::global::KeyLocation,);
            let args = (location,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4484usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "set_location", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_location(&self,) -> crate::global::KeyLocation {
            type CallRet = crate::global::KeyLocation;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4485usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "get_location", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_echo(&mut self, echo: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (echo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4486usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "set_echo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_keycode_with_modifiers(&self,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4487usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "get_keycode_with_modifiers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physical_keycode_with_modifiers(&self,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4488usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "get_physical_keycode_with_modifiers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_key_label_with_modifiers(&self,) -> crate::global::Key {
            type CallRet = crate::global::Key;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4489usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "get_key_label_with_modifiers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn as_text_keycode(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4490usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "as_text_keycode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn as_text_physical_keycode(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4491usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "as_text_physical_keycode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn as_text_key_label(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4492usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "as_text_key_label", Some(self.__validated_obj()), args,)
            }
        }
        pub fn as_text_location(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4493usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventKey", "as_text_location", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for InputEventKey {
        type Base = crate::classes::InputEventWithModifiers;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("InputEventKey"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputEventKey {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::InputEventWithModifiers > for InputEventKey {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::InputEventFromWindow > for InputEventKey {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::InputEvent > for InputEventKey {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for InputEventKey {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for InputEventKey {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for InputEventKey {
        
    }
    impl crate::obj::cap::GodotDefault for InputEventKey {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for InputEventKey {
        type Target = crate::classes::InputEventWithModifiers;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputEventKey {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`InputEventKey`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_InputEventKey__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::InputEventKey > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::InputEventWithModifiers > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::InputEventFromWindow > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::InputEvent > for $Class {
                
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::InputEventKey;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for InputEventKey {
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