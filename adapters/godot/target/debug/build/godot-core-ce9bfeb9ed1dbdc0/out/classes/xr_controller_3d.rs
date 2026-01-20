#![doc = "Sidecar module for class [`XrController3D`][crate::classes::XrController3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `XRController3D` enums](https://docs.godotengine.org/en/stable/classes/class_xrcontroller3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `XRController3D.`\n\nInherits [`XrNode3D`][crate::classes::XrNode3D].\n\nRelated symbols:\n\n* [`xr_controller_3d`][crate::classes::xr_controller_3d]: sidecar module with related enum/flag types\n* [`IXrController3D`][crate::classes::IXrController3D]: virtual methods\n* [`SignalsOfXrController3D`][crate::classes::xr_controller_3d::SignalsOfXrController3D]: signal collection\n\n\nSee also [Godot docs for `XRController3D`](https://docs.godotengine.org/en/stable/classes/class_xrcontroller3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`XrController3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct XrController3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`XrController3D`][crate::classes::XrController3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IXrNode3D`][crate::classes::IXrNode3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `XRController3D` methods](https://docs.godotengine.org/en/stable/classes/class_xrcontroller3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IXrController3D: crate::obj::GodotClass < Base = XrController3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
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
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_accessibility_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn get_focused_accessibility_element(&self,) -> Rid {
            unimplemented !()
        }
    }
    impl XrController3D {
        pub fn is_button_pressed(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrController3D", "is_button_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_input(&self, name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrController3D", "get_input", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_float(&self, name: impl AsArg < StringName >,) -> f32 {
            type CallRet = f32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrController3D", "get_float", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vector2(&self, name: impl AsArg < StringName >,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrController3D", "get_vector2", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tracker_hand(&self,) -> crate::classes::xr_positional_tracker::TrackerHand {
            type CallRet = crate::classes::xr_positional_tracker::TrackerHand;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrController3D", "get_tracker_hand", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for XrController3D {
        type Base = crate::classes::XrNode3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("XRController3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for XrController3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrNode3D > for XrController3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for XrController3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for XrController3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for XrController3D {
        
    }
    impl crate::obj::cap::GodotDefault for XrController3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for XrController3D {
        type Target = crate::classes::XrNode3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for XrController3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`XrController3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_XrController3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::XrController3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::XrNode3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
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
    use super::re_export::XrController3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`XrController3D`][crate::classes::XrController3D] class."]
    pub struct SignalsOfXrController3D < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfXrController3D < 'c, C > {
        #[doc = "Signature: `(name: GString)`"]
        pub fn button_pressed(&mut self) -> SigButtonPressed < 'c, C > {
            SigButtonPressed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "button_pressed")
            }
        }
        #[doc = "Signature: `(name: GString)`"]
        pub fn button_released(&mut self) -> SigButtonReleased < 'c, C > {
            SigButtonReleased {
                typed: TypedSignal::extract(&mut self.__internal_obj, "button_released")
            }
        }
        #[doc = "Signature: `(name: GString, value: f64)`"]
        pub fn input_float_changed(&mut self) -> SigInputFloatChanged < 'c, C > {
            SigInputFloatChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "input_float_changed")
            }
        }
        #[doc = "Signature: `(name: GString, value: Vector2)`"]
        pub fn input_vector2_changed(&mut self) -> SigInputVector2Changed < 'c, C > {
            SigInputVector2Changed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "input_vector2_changed")
            }
        }
        #[doc = "Signature: `(role: GString)`"]
        pub fn profile_changed(&mut self) -> SigProfileChanged < 'c, C > {
            SigProfileChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "profile_changed")
            }
        }
    }
    type TypedSigButtonPressed < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigButtonPressed < 'c, C: WithSignals > {
        typed: TypedSigButtonPressed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigButtonPressed < 'c, C > {
        pub fn emit(&mut self, name: GString,) {
            self.typed.emit_tuple((name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigButtonPressed < 'c, C > {
        type Target = TypedSigButtonPressed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigButtonPressed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigButtonReleased < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigButtonReleased < 'c, C: WithSignals > {
        typed: TypedSigButtonReleased < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigButtonReleased < 'c, C > {
        pub fn emit(&mut self, name: GString,) {
            self.typed.emit_tuple((name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigButtonReleased < 'c, C > {
        type Target = TypedSigButtonReleased < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigButtonReleased < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigInputFloatChanged < 'c, C > = TypedSignal < 'c, C, (GString, f64,) >;
    pub struct SigInputFloatChanged < 'c, C: WithSignals > {
        typed: TypedSigInputFloatChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigInputFloatChanged < 'c, C > {
        pub fn emit(&mut self, name: GString, value: f64,) {
            self.typed.emit_tuple((name, value,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigInputFloatChanged < 'c, C > {
        type Target = TypedSigInputFloatChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigInputFloatChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigInputVector2Changed < 'c, C > = TypedSignal < 'c, C, (GString, Vector2,) >;
    pub struct SigInputVector2Changed < 'c, C: WithSignals > {
        typed: TypedSigInputVector2Changed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigInputVector2Changed < 'c, C > {
        pub fn emit(&mut self, name: GString, value: Vector2,) {
            self.typed.emit_tuple((name, value,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigInputVector2Changed < 'c, C > {
        type Target = TypedSigInputVector2Changed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigInputVector2Changed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigProfileChanged < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigProfileChanged < 'c, C: WithSignals > {
        typed: TypedSigProfileChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigProfileChanged < 'c, C > {
        pub fn emit(&mut self, role: GString,) {
            self.typed.emit_tuple((role,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigProfileChanged < 'c, C > {
        type Target = TypedSigProfileChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigProfileChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for XrController3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfXrController3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfXrController3D < 'c, C > {
        type Target = < < XrController3D as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = XrController3D;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfXrController3D < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = XrController3D;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}