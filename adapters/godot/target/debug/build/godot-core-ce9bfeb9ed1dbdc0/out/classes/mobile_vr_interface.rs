#![doc = "Sidecar module for class [`MobileVrInterface`][crate::classes::MobileVrInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MobileVRInterface` enums](https://docs.godotengine.org/en/stable/classes/class_mobilevrinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MobileVRInterface.`\n\nInherits [`XrInterface`][crate::classes::XrInterface].\n\nRelated symbols:\n\n* [`IMobileVrInterface`][crate::classes::IMobileVrInterface]: virtual methods\n\n\nSee also [Godot docs for `MobileVRInterface`](https://docs.godotengine.org/en/stable/classes/class_mobilevrinterface.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`MobileVrInterface::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MobileVrInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`MobileVrInterface`][crate::classes::MobileVrInterface].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IXrInterface`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `MobileVRInterface` methods](https://docs.godotengine.org/en/stable/classes/class_mobilevrinterface.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMobileVrInterface: crate::obj::GodotClass < Base = MobileVrInterface > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MobileVrInterface {
        pub fn set_eye_height(&mut self, eye_height: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (eye_height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "set_eye_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_eye_height(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "get_eye_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_iod(&mut self, iod: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (iod,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "set_iod", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_iod(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "get_iod", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_display_width(&mut self, display_width: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (display_width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "set_display_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_display_width(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "get_display_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_display_to_lens(&mut self, display_to_lens: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (display_to_lens,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "set_display_to_lens", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_display_to_lens(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "get_display_to_lens", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_offset_rect(&mut self, offset_rect: Rect2,) {
            type CallRet = ();
            type CallParams = (Rect2,);
            let args = (offset_rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "set_offset_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_offset_rect(&self,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "get_offset_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_oversample(&mut self, oversample: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (oversample,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "set_oversample", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_oversample(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5420usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "get_oversample", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_k1(&mut self, k: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (k,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5421usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "set_k1", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_k1(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5422usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "get_k1", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_k2(&mut self, k: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (k,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5423usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "set_k2", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_k2(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5424usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "get_k2", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vrs_min_radius(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5425usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "get_vrs_min_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vrs_min_radius(&mut self, radius: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5426usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "set_vrs_min_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vrs_strength(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5427usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "get_vrs_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vrs_strength(&mut self, strength: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5428usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MobileVrInterface", "set_vrs_strength", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for MobileVrInterface {
        type Base = crate::classes::XrInterface;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("MobileVRInterface"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MobileVrInterface {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrInterface > for MobileVrInterface {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for MobileVrInterface {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MobileVrInterface {
        
    }
    impl crate::obj::cap::GodotDefault for MobileVrInterface {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MobileVrInterface {
        type Target = crate::classes::XrInterface;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MobileVrInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MobileVrInterface`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_MobileVrInterface__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MobileVrInterface > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::XrInterface > for $Class {
                
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
    use super::re_export::MobileVrInterface;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::xr_interface::SignalsOfXrInterface;
    impl WithSignals for MobileVrInterface {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfXrInterface < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}