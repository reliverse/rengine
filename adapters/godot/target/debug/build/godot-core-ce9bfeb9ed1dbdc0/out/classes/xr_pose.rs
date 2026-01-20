#![doc = "Sidecar module for class [`XrPose`][crate::classes::XrPose].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `XRPose` enums](https://docs.godotengine.org/en/stable/classes/class_xrpose.html#enumerations).\n\n"]
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
    #[doc = "Godot class `XRPose.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`xr_pose`][crate::classes::xr_pose]: sidecar module with related enum/flag types\n* [`IXrPose`][crate::classes::IXrPose]: virtual methods\n\n\nSee also [Godot docs for `XRPose`](https://docs.godotengine.org/en/stable/classes/class_xrpose.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`XrPose::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct XrPose {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`XrPose`][crate::classes::XrPose].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `XRPose` methods](https://docs.godotengine.org/en/stable/classes/class_xrpose.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IXrPose: crate::obj::GodotClass < Base = XrPose > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl XrPose {
        pub fn set_has_tracking_data(&mut self, has_tracking_data: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (has_tracking_data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrPose", "set_has_tracking_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_has_tracking_data(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrPose", "get_has_tracking_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_name(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrPose", "set_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_name(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrPose", "get_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transform(&mut self, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Transform3D,);
            let args = (transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrPose", "set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transform(&self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrPose", "get_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_adjusted_transform(&self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrPose", "get_adjusted_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_velocity(&mut self, velocity: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrPose", "set_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_velocity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrPose", "get_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_velocity(&mut self, velocity: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrPose", "set_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_velocity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrPose", "get_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tracking_confidence(&mut self, tracking_confidence: crate::classes::xr_pose::TrackingConfidence,) {
            type CallRet = ();
            type CallParams = (crate::classes::xr_pose::TrackingConfidence,);
            let args = (tracking_confidence,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrPose", "set_tracking_confidence", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tracking_confidence(&self,) -> crate::classes::xr_pose::TrackingConfidence {
            type CallRet = crate::classes::xr_pose::TrackingConfidence;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrPose", "get_tracking_confidence", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for XrPose {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("XRPose"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for XrPose {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for XrPose {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for XrPose {
        
    }
    impl crate::obj::cap::GodotDefault for XrPose {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for XrPose {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for XrPose {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`XrPose`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_XrPose__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::XrPose > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TrackingConfidence {
    ord: i32
}
impl TrackingConfidence {
    #[doc(alias = "XR_TRACKING_CONFIDENCE_NONE")]
    #[doc = "Godot enumerator name: `XR_TRACKING_CONFIDENCE_NONE`"]
    pub const NONE: TrackingConfidence = TrackingConfidence {
        ord: 0i32
    };
    #[doc(alias = "XR_TRACKING_CONFIDENCE_LOW")]
    #[doc = "Godot enumerator name: `XR_TRACKING_CONFIDENCE_LOW`"]
    pub const LOW: TrackingConfidence = TrackingConfidence {
        ord: 1i32
    };
    #[doc(alias = "XR_TRACKING_CONFIDENCE_HIGH")]
    #[doc = "Godot enumerator name: `XR_TRACKING_CONFIDENCE_HIGH`"]
    pub const HIGH: TrackingConfidence = TrackingConfidence {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TrackingConfidence {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TrackingConfidence") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TrackingConfidence {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
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
            Self::NONE => "NONE", Self::LOW => "LOW", Self::HIGH => "HIGH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TrackingConfidence::NONE, TrackingConfidence::LOW, TrackingConfidence::HIGH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TrackingConfidence >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "XR_TRACKING_CONFIDENCE_NONE", TrackingConfidence::NONE), crate::meta::inspect::EnumConstant::new("LOW", "XR_TRACKING_CONFIDENCE_LOW", TrackingConfidence::LOW), crate::meta::inspect::EnumConstant::new("HIGH", "XR_TRACKING_CONFIDENCE_HIGH", TrackingConfidence::HIGH)]
        }
    }
}
impl crate::meta::GodotConvert for TrackingConfidence {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TrackingConfidence {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TrackingConfidence {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::XrPose;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for XrPose {
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