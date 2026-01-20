#![doc = "Sidecar module for class [`XrHandTracker`][crate::classes::XrHandTracker].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `XRHandTracker` enums](https://docs.godotengine.org/en/stable/classes/class_xrhandtracker.html#enumerations).\n\n"]
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
    #[doc = "Godot class `XRHandTracker.`\n\nInherits [`XrPositionalTracker`][crate::classes::XrPositionalTracker].\n\nRelated symbols:\n\n* [`xr_hand_tracker`][crate::classes::xr_hand_tracker]: sidecar module with related enum/flag types\n* [`IXrHandTracker`][crate::classes::IXrHandTracker]: virtual methods\n\n\nSee also [Godot docs for `XRHandTracker`](https://docs.godotengine.org/en/stable/classes/class_xrhandtracker.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`XrHandTracker::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct XrHandTracker {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`XrHandTracker`][crate::classes::XrHandTracker].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IXrPositionalTracker`][crate::classes::IXrPositionalTracker] > ~~`IXrTracker`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `XRHandTracker` methods](https://docs.godotengine.org/en/stable/classes/class_xrhandtracker.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IXrHandTracker: crate::obj::GodotClass < Base = XrHandTracker > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl XrHandTracker {
        pub fn set_has_tracking_data(&mut self, has_data: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (has_data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrHandTracker", "set_has_tracking_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_has_tracking_data(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrHandTracker", "get_has_tracking_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hand_tracking_source(&mut self, source: crate::classes::xr_hand_tracker::HandTrackingSource,) {
            type CallRet = ();
            type CallParams = (crate::classes::xr_hand_tracker::HandTrackingSource,);
            let args = (source,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrHandTracker", "set_hand_tracking_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hand_tracking_source(&self,) -> crate::classes::xr_hand_tracker::HandTrackingSource {
            type CallRet = crate::classes::xr_hand_tracker::HandTrackingSource;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrHandTracker", "get_hand_tracking_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hand_joint_flags(&mut self, joint: crate::classes::xr_hand_tracker::HandJoint, flags: crate::classes::xr_hand_tracker::HandJointFlags,) {
            type CallRet = ();
            type CallParams = (crate::classes::xr_hand_tracker::HandJoint, crate::classes::xr_hand_tracker::HandJointFlags,);
            let args = (joint, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrHandTracker", "set_hand_joint_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hand_joint_flags(&self, joint: crate::classes::xr_hand_tracker::HandJoint,) -> crate::classes::xr_hand_tracker::HandJointFlags {
            type CallRet = crate::classes::xr_hand_tracker::HandJointFlags;
            type CallParams = (crate::classes::xr_hand_tracker::HandJoint,);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrHandTracker", "get_hand_joint_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hand_joint_transform(&mut self, joint: crate::classes::xr_hand_tracker::HandJoint, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (crate::classes::xr_hand_tracker::HandJoint, Transform3D,);
            let args = (joint, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrHandTracker", "set_hand_joint_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hand_joint_transform(&self, joint: crate::classes::xr_hand_tracker::HandJoint,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (crate::classes::xr_hand_tracker::HandJoint,);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrHandTracker", "get_hand_joint_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hand_joint_radius(&mut self, joint: crate::classes::xr_hand_tracker::HandJoint, radius: f32,) {
            type CallRet = ();
            type CallParams = (crate::classes::xr_hand_tracker::HandJoint, f32,);
            let args = (joint, radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrHandTracker", "set_hand_joint_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hand_joint_radius(&self, joint: crate::classes::xr_hand_tracker::HandJoint,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::classes::xr_hand_tracker::HandJoint,);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrHandTracker", "get_hand_joint_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hand_joint_linear_velocity(&mut self, joint: crate::classes::xr_hand_tracker::HandJoint, linear_velocity: Vector3,) {
            type CallRet = ();
            type CallParams = (crate::classes::xr_hand_tracker::HandJoint, Vector3,);
            let args = (joint, linear_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrHandTracker", "set_hand_joint_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hand_joint_linear_velocity(&self, joint: crate::classes::xr_hand_tracker::HandJoint,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (crate::classes::xr_hand_tracker::HandJoint,);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrHandTracker", "get_hand_joint_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hand_joint_angular_velocity(&mut self, joint: crate::classes::xr_hand_tracker::HandJoint, angular_velocity: Vector3,) {
            type CallRet = ();
            type CallParams = (crate::classes::xr_hand_tracker::HandJoint, Vector3,);
            let args = (joint, angular_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrHandTracker", "set_hand_joint_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hand_joint_angular_velocity(&self, joint: crate::classes::xr_hand_tracker::HandJoint,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (crate::classes::xr_hand_tracker::HandJoint,);
            let args = (joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrHandTracker", "get_hand_joint_angular_velocity", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for XrHandTracker {
        type Base = crate::classes::XrPositionalTracker;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("XRHandTracker"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for XrHandTracker {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrPositionalTracker > for XrHandTracker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrTracker > for XrHandTracker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for XrHandTracker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for XrHandTracker {
        
    }
    impl crate::obj::cap::GodotDefault for XrHandTracker {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for XrHandTracker {
        type Target = crate::classes::XrPositionalTracker;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for XrHandTracker {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`XrHandTracker`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_XrHandTracker__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::XrHandTracker > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::XrPositionalTracker > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::XrTracker > for $Class {
                
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
pub struct HandTrackingSource {
    ord: i32
}
impl HandTrackingSource {
    #[doc(alias = "HAND_TRACKING_SOURCE_UNKNOWN")]
    #[doc = "Godot enumerator name: `HAND_TRACKING_SOURCE_UNKNOWN`"]
    pub const UNKNOWN: HandTrackingSource = HandTrackingSource {
        ord: 0i32
    };
    #[doc(alias = "HAND_TRACKING_SOURCE_UNOBSTRUCTED")]
    #[doc = "Godot enumerator name: `HAND_TRACKING_SOURCE_UNOBSTRUCTED`"]
    pub const UNOBSTRUCTED: HandTrackingSource = HandTrackingSource {
        ord: 1i32
    };
    #[doc(alias = "HAND_TRACKING_SOURCE_CONTROLLER")]
    #[doc = "Godot enumerator name: `HAND_TRACKING_SOURCE_CONTROLLER`"]
    pub const CONTROLLER: HandTrackingSource = HandTrackingSource {
        ord: 2i32
    };
    #[doc(alias = "HAND_TRACKING_SOURCE_NOT_TRACKED")]
    #[doc = "Godot enumerator name: `HAND_TRACKING_SOURCE_NOT_TRACKED`"]
    pub const NOT_TRACKED: HandTrackingSource = HandTrackingSource {
        ord: 3i32
    };
    #[doc(alias = "HAND_TRACKING_SOURCE_MAX")]
    #[doc = "Godot enumerator name: `HAND_TRACKING_SOURCE_MAX`"]
    pub const MAX: HandTrackingSource = HandTrackingSource {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for HandTrackingSource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HandTrackingSource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HandTrackingSource {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::UNKNOWN => "UNKNOWN", Self::UNOBSTRUCTED => "UNOBSTRUCTED", Self::CONTROLLER => "CONTROLLER", Self::NOT_TRACKED => "NOT_TRACKED", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[HandTrackingSource::UNKNOWN, HandTrackingSource::UNOBSTRUCTED, HandTrackingSource::CONTROLLER, HandTrackingSource::NOT_TRACKED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < HandTrackingSource >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNKNOWN", "HAND_TRACKING_SOURCE_UNKNOWN", HandTrackingSource::UNKNOWN), crate::meta::inspect::EnumConstant::new("UNOBSTRUCTED", "HAND_TRACKING_SOURCE_UNOBSTRUCTED", HandTrackingSource::UNOBSTRUCTED), crate::meta::inspect::EnumConstant::new("CONTROLLER", "HAND_TRACKING_SOURCE_CONTROLLER", HandTrackingSource::CONTROLLER), crate::meta::inspect::EnumConstant::new("NOT_TRACKED", "HAND_TRACKING_SOURCE_NOT_TRACKED", HandTrackingSource::NOT_TRACKED), crate::meta::inspect::EnumConstant::new("MAX", "HAND_TRACKING_SOURCE_MAX", HandTrackingSource::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for HandTrackingSource {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for HandTrackingSource {
    type Via = i32;
    
}
impl crate::meta::ToGodot for HandTrackingSource {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HandTrackingSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HandJoint {
    ord: i32
}
impl HandJoint {
    #[doc(alias = "HAND_JOINT_PALM")]
    #[doc = "Godot enumerator name: `HAND_JOINT_PALM`"]
    pub const PALM: HandJoint = HandJoint {
        ord: 0i32
    };
    #[doc(alias = "HAND_JOINT_WRIST")]
    #[doc = "Godot enumerator name: `HAND_JOINT_WRIST`"]
    pub const WRIST: HandJoint = HandJoint {
        ord: 1i32
    };
    #[doc(alias = "HAND_JOINT_THUMB_METACARPAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_THUMB_METACARPAL`"]
    pub const THUMB_METACARPAL: HandJoint = HandJoint {
        ord: 2i32
    };
    #[doc(alias = "HAND_JOINT_THUMB_PHALANX_PROXIMAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_THUMB_PHALANX_PROXIMAL`"]
    pub const THUMB_PHALANX_PROXIMAL: HandJoint = HandJoint {
        ord: 3i32
    };
    #[doc(alias = "HAND_JOINT_THUMB_PHALANX_DISTAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_THUMB_PHALANX_DISTAL`"]
    pub const THUMB_PHALANX_DISTAL: HandJoint = HandJoint {
        ord: 4i32
    };
    #[doc(alias = "HAND_JOINT_THUMB_TIP")]
    #[doc = "Godot enumerator name: `HAND_JOINT_THUMB_TIP`"]
    pub const THUMB_TIP: HandJoint = HandJoint {
        ord: 5i32
    };
    #[doc(alias = "HAND_JOINT_INDEX_FINGER_METACARPAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_INDEX_FINGER_METACARPAL`"]
    pub const INDEX_FINGER_METACARPAL: HandJoint = HandJoint {
        ord: 6i32
    };
    #[doc(alias = "HAND_JOINT_INDEX_FINGER_PHALANX_PROXIMAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_INDEX_FINGER_PHALANX_PROXIMAL`"]
    pub const INDEX_FINGER_PHALANX_PROXIMAL: HandJoint = HandJoint {
        ord: 7i32
    };
    #[doc(alias = "HAND_JOINT_INDEX_FINGER_PHALANX_INTERMEDIATE")]
    #[doc = "Godot enumerator name: `HAND_JOINT_INDEX_FINGER_PHALANX_INTERMEDIATE`"]
    pub const INDEX_FINGER_PHALANX_INTERMEDIATE: HandJoint = HandJoint {
        ord: 8i32
    };
    #[doc(alias = "HAND_JOINT_INDEX_FINGER_PHALANX_DISTAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_INDEX_FINGER_PHALANX_DISTAL`"]
    pub const INDEX_FINGER_PHALANX_DISTAL: HandJoint = HandJoint {
        ord: 9i32
    };
    #[doc(alias = "HAND_JOINT_INDEX_FINGER_TIP")]
    #[doc = "Godot enumerator name: `HAND_JOINT_INDEX_FINGER_TIP`"]
    pub const INDEX_FINGER_TIP: HandJoint = HandJoint {
        ord: 10i32
    };
    #[doc(alias = "HAND_JOINT_MIDDLE_FINGER_METACARPAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MIDDLE_FINGER_METACARPAL`"]
    pub const MIDDLE_FINGER_METACARPAL: HandJoint = HandJoint {
        ord: 11i32
    };
    #[doc(alias = "HAND_JOINT_MIDDLE_FINGER_PHALANX_PROXIMAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MIDDLE_FINGER_PHALANX_PROXIMAL`"]
    pub const MIDDLE_FINGER_PHALANX_PROXIMAL: HandJoint = HandJoint {
        ord: 12i32
    };
    #[doc(alias = "HAND_JOINT_MIDDLE_FINGER_PHALANX_INTERMEDIATE")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MIDDLE_FINGER_PHALANX_INTERMEDIATE`"]
    pub const MIDDLE_FINGER_PHALANX_INTERMEDIATE: HandJoint = HandJoint {
        ord: 13i32
    };
    #[doc(alias = "HAND_JOINT_MIDDLE_FINGER_PHALANX_DISTAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MIDDLE_FINGER_PHALANX_DISTAL`"]
    pub const MIDDLE_FINGER_PHALANX_DISTAL: HandJoint = HandJoint {
        ord: 14i32
    };
    #[doc(alias = "HAND_JOINT_MIDDLE_FINGER_TIP")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MIDDLE_FINGER_TIP`"]
    pub const MIDDLE_FINGER_TIP: HandJoint = HandJoint {
        ord: 15i32
    };
    #[doc(alias = "HAND_JOINT_RING_FINGER_METACARPAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_RING_FINGER_METACARPAL`"]
    pub const RING_FINGER_METACARPAL: HandJoint = HandJoint {
        ord: 16i32
    };
    #[doc(alias = "HAND_JOINT_RING_FINGER_PHALANX_PROXIMAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_RING_FINGER_PHALANX_PROXIMAL`"]
    pub const RING_FINGER_PHALANX_PROXIMAL: HandJoint = HandJoint {
        ord: 17i32
    };
    #[doc(alias = "HAND_JOINT_RING_FINGER_PHALANX_INTERMEDIATE")]
    #[doc = "Godot enumerator name: `HAND_JOINT_RING_FINGER_PHALANX_INTERMEDIATE`"]
    pub const RING_FINGER_PHALANX_INTERMEDIATE: HandJoint = HandJoint {
        ord: 18i32
    };
    #[doc(alias = "HAND_JOINT_RING_FINGER_PHALANX_DISTAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_RING_FINGER_PHALANX_DISTAL`"]
    pub const RING_FINGER_PHALANX_DISTAL: HandJoint = HandJoint {
        ord: 19i32
    };
    #[doc(alias = "HAND_JOINT_RING_FINGER_TIP")]
    #[doc = "Godot enumerator name: `HAND_JOINT_RING_FINGER_TIP`"]
    pub const RING_FINGER_TIP: HandJoint = HandJoint {
        ord: 20i32
    };
    #[doc(alias = "HAND_JOINT_PINKY_FINGER_METACARPAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_PINKY_FINGER_METACARPAL`"]
    pub const PINKY_FINGER_METACARPAL: HandJoint = HandJoint {
        ord: 21i32
    };
    #[doc(alias = "HAND_JOINT_PINKY_FINGER_PHALANX_PROXIMAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_PINKY_FINGER_PHALANX_PROXIMAL`"]
    pub const PINKY_FINGER_PHALANX_PROXIMAL: HandJoint = HandJoint {
        ord: 22i32
    };
    #[doc(alias = "HAND_JOINT_PINKY_FINGER_PHALANX_INTERMEDIATE")]
    #[doc = "Godot enumerator name: `HAND_JOINT_PINKY_FINGER_PHALANX_INTERMEDIATE`"]
    pub const PINKY_FINGER_PHALANX_INTERMEDIATE: HandJoint = HandJoint {
        ord: 23i32
    };
    #[doc(alias = "HAND_JOINT_PINKY_FINGER_PHALANX_DISTAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_PINKY_FINGER_PHALANX_DISTAL`"]
    pub const PINKY_FINGER_PHALANX_DISTAL: HandJoint = HandJoint {
        ord: 24i32
    };
    #[doc(alias = "HAND_JOINT_PINKY_FINGER_TIP")]
    #[doc = "Godot enumerator name: `HAND_JOINT_PINKY_FINGER_TIP`"]
    pub const PINKY_FINGER_TIP: HandJoint = HandJoint {
        ord: 25i32
    };
    #[doc(alias = "HAND_JOINT_MAX")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MAX`"]
    pub const MAX: HandJoint = HandJoint {
        ord: 26i32
    };
    
}
impl std::fmt::Debug for HandJoint {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HandJoint") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HandJoint {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 => Some(Self {
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
            Self::PALM => "PALM", Self::WRIST => "WRIST", Self::THUMB_METACARPAL => "THUMB_METACARPAL", Self::THUMB_PHALANX_PROXIMAL => "THUMB_PHALANX_PROXIMAL", Self::THUMB_PHALANX_DISTAL => "THUMB_PHALANX_DISTAL", Self::THUMB_TIP => "THUMB_TIP", Self::INDEX_FINGER_METACARPAL => "INDEX_FINGER_METACARPAL", Self::INDEX_FINGER_PHALANX_PROXIMAL => "INDEX_FINGER_PHALANX_PROXIMAL", Self::INDEX_FINGER_PHALANX_INTERMEDIATE => "INDEX_FINGER_PHALANX_INTERMEDIATE", Self::INDEX_FINGER_PHALANX_DISTAL => "INDEX_FINGER_PHALANX_DISTAL", Self::INDEX_FINGER_TIP => "INDEX_FINGER_TIP", Self::MIDDLE_FINGER_METACARPAL => "MIDDLE_FINGER_METACARPAL", Self::MIDDLE_FINGER_PHALANX_PROXIMAL => "MIDDLE_FINGER_PHALANX_PROXIMAL", Self::MIDDLE_FINGER_PHALANX_INTERMEDIATE => "MIDDLE_FINGER_PHALANX_INTERMEDIATE", Self::MIDDLE_FINGER_PHALANX_DISTAL => "MIDDLE_FINGER_PHALANX_DISTAL", Self::MIDDLE_FINGER_TIP => "MIDDLE_FINGER_TIP", Self::RING_FINGER_METACARPAL => "RING_FINGER_METACARPAL", Self::RING_FINGER_PHALANX_PROXIMAL => "RING_FINGER_PHALANX_PROXIMAL", Self::RING_FINGER_PHALANX_INTERMEDIATE => "RING_FINGER_PHALANX_INTERMEDIATE", Self::RING_FINGER_PHALANX_DISTAL => "RING_FINGER_PHALANX_DISTAL", Self::RING_FINGER_TIP => "RING_FINGER_TIP", Self::PINKY_FINGER_METACARPAL => "PINKY_FINGER_METACARPAL", Self::PINKY_FINGER_PHALANX_PROXIMAL => "PINKY_FINGER_PHALANX_PROXIMAL", Self::PINKY_FINGER_PHALANX_INTERMEDIATE => "PINKY_FINGER_PHALANX_INTERMEDIATE", Self::PINKY_FINGER_PHALANX_DISTAL => "PINKY_FINGER_PHALANX_DISTAL", Self::PINKY_FINGER_TIP => "PINKY_FINGER_TIP", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[HandJoint::PALM, HandJoint::WRIST, HandJoint::THUMB_METACARPAL, HandJoint::THUMB_PHALANX_PROXIMAL, HandJoint::THUMB_PHALANX_DISTAL, HandJoint::THUMB_TIP, HandJoint::INDEX_FINGER_METACARPAL, HandJoint::INDEX_FINGER_PHALANX_PROXIMAL, HandJoint::INDEX_FINGER_PHALANX_INTERMEDIATE, HandJoint::INDEX_FINGER_PHALANX_DISTAL, HandJoint::INDEX_FINGER_TIP, HandJoint::MIDDLE_FINGER_METACARPAL, HandJoint::MIDDLE_FINGER_PHALANX_PROXIMAL, HandJoint::MIDDLE_FINGER_PHALANX_INTERMEDIATE, HandJoint::MIDDLE_FINGER_PHALANX_DISTAL, HandJoint::MIDDLE_FINGER_TIP, HandJoint::RING_FINGER_METACARPAL, HandJoint::RING_FINGER_PHALANX_PROXIMAL, HandJoint::RING_FINGER_PHALANX_INTERMEDIATE, HandJoint::RING_FINGER_PHALANX_DISTAL, HandJoint::RING_FINGER_TIP, HandJoint::PINKY_FINGER_METACARPAL, HandJoint::PINKY_FINGER_PHALANX_PROXIMAL, HandJoint::PINKY_FINGER_PHALANX_INTERMEDIATE, HandJoint::PINKY_FINGER_PHALANX_DISTAL, HandJoint::PINKY_FINGER_TIP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < HandJoint >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PALM", "HAND_JOINT_PALM", HandJoint::PALM), crate::meta::inspect::EnumConstant::new("WRIST", "HAND_JOINT_WRIST", HandJoint::WRIST), crate::meta::inspect::EnumConstant::new("THUMB_METACARPAL", "HAND_JOINT_THUMB_METACARPAL", HandJoint::THUMB_METACARPAL), crate::meta::inspect::EnumConstant::new("THUMB_PHALANX_PROXIMAL", "HAND_JOINT_THUMB_PHALANX_PROXIMAL", HandJoint::THUMB_PHALANX_PROXIMAL), crate::meta::inspect::EnumConstant::new("THUMB_PHALANX_DISTAL", "HAND_JOINT_THUMB_PHALANX_DISTAL", HandJoint::THUMB_PHALANX_DISTAL), crate::meta::inspect::EnumConstant::new("THUMB_TIP", "HAND_JOINT_THUMB_TIP", HandJoint::THUMB_TIP), crate::meta::inspect::EnumConstant::new("INDEX_FINGER_METACARPAL", "HAND_JOINT_INDEX_FINGER_METACARPAL", HandJoint::INDEX_FINGER_METACARPAL), crate::meta::inspect::EnumConstant::new("INDEX_FINGER_PHALANX_PROXIMAL", "HAND_JOINT_INDEX_FINGER_PHALANX_PROXIMAL", HandJoint::INDEX_FINGER_PHALANX_PROXIMAL), crate::meta::inspect::EnumConstant::new("INDEX_FINGER_PHALANX_INTERMEDIATE", "HAND_JOINT_INDEX_FINGER_PHALANX_INTERMEDIATE", HandJoint::INDEX_FINGER_PHALANX_INTERMEDIATE), crate::meta::inspect::EnumConstant::new("INDEX_FINGER_PHALANX_DISTAL", "HAND_JOINT_INDEX_FINGER_PHALANX_DISTAL", HandJoint::INDEX_FINGER_PHALANX_DISTAL), crate::meta::inspect::EnumConstant::new("INDEX_FINGER_TIP", "HAND_JOINT_INDEX_FINGER_TIP", HandJoint::INDEX_FINGER_TIP), crate::meta::inspect::EnumConstant::new("MIDDLE_FINGER_METACARPAL", "HAND_JOINT_MIDDLE_FINGER_METACARPAL", HandJoint::MIDDLE_FINGER_METACARPAL), crate::meta::inspect::EnumConstant::new("MIDDLE_FINGER_PHALANX_PROXIMAL", "HAND_JOINT_MIDDLE_FINGER_PHALANX_PROXIMAL", HandJoint::MIDDLE_FINGER_PHALANX_PROXIMAL), crate::meta::inspect::EnumConstant::new("MIDDLE_FINGER_PHALANX_INTERMEDIATE", "HAND_JOINT_MIDDLE_FINGER_PHALANX_INTERMEDIATE", HandJoint::MIDDLE_FINGER_PHALANX_INTERMEDIATE), crate::meta::inspect::EnumConstant::new("MIDDLE_FINGER_PHALANX_DISTAL", "HAND_JOINT_MIDDLE_FINGER_PHALANX_DISTAL", HandJoint::MIDDLE_FINGER_PHALANX_DISTAL), crate::meta::inspect::EnumConstant::new("MIDDLE_FINGER_TIP", "HAND_JOINT_MIDDLE_FINGER_TIP", HandJoint::MIDDLE_FINGER_TIP), crate::meta::inspect::EnumConstant::new("RING_FINGER_METACARPAL", "HAND_JOINT_RING_FINGER_METACARPAL", HandJoint::RING_FINGER_METACARPAL), crate::meta::inspect::EnumConstant::new("RING_FINGER_PHALANX_PROXIMAL", "HAND_JOINT_RING_FINGER_PHALANX_PROXIMAL", HandJoint::RING_FINGER_PHALANX_PROXIMAL), crate::meta::inspect::EnumConstant::new("RING_FINGER_PHALANX_INTERMEDIATE", "HAND_JOINT_RING_FINGER_PHALANX_INTERMEDIATE", HandJoint::RING_FINGER_PHALANX_INTERMEDIATE), crate::meta::inspect::EnumConstant::new("RING_FINGER_PHALANX_DISTAL", "HAND_JOINT_RING_FINGER_PHALANX_DISTAL", HandJoint::RING_FINGER_PHALANX_DISTAL), crate::meta::inspect::EnumConstant::new("RING_FINGER_TIP", "HAND_JOINT_RING_FINGER_TIP", HandJoint::RING_FINGER_TIP), crate::meta::inspect::EnumConstant::new("PINKY_FINGER_METACARPAL", "HAND_JOINT_PINKY_FINGER_METACARPAL", HandJoint::PINKY_FINGER_METACARPAL), crate::meta::inspect::EnumConstant::new("PINKY_FINGER_PHALANX_PROXIMAL", "HAND_JOINT_PINKY_FINGER_PHALANX_PROXIMAL", HandJoint::PINKY_FINGER_PHALANX_PROXIMAL), crate::meta::inspect::EnumConstant::new("PINKY_FINGER_PHALANX_INTERMEDIATE", "HAND_JOINT_PINKY_FINGER_PHALANX_INTERMEDIATE", HandJoint::PINKY_FINGER_PHALANX_INTERMEDIATE), crate::meta::inspect::EnumConstant::new("PINKY_FINGER_PHALANX_DISTAL", "HAND_JOINT_PINKY_FINGER_PHALANX_DISTAL", HandJoint::PINKY_FINGER_PHALANX_DISTAL), crate::meta::inspect::EnumConstant::new("PINKY_FINGER_TIP", "HAND_JOINT_PINKY_FINGER_TIP", HandJoint::PINKY_FINGER_TIP), crate::meta::inspect::EnumConstant::new("MAX", "HAND_JOINT_MAX", HandJoint::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for HandJoint {
    const ENUMERATOR_COUNT: usize = 26usize;
    
}
impl crate::meta::GodotConvert for HandJoint {
    type Via = i32;
    
}
impl crate::meta::ToGodot for HandJoint {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HandJoint {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct HandJointFlags {
    ord: u64
}
impl HandJointFlags {
    #[doc(alias = "HAND_JOINT_FLAG_ORIENTATION_VALID")]
    #[doc = "Godot enumerator name: `HAND_JOINT_FLAG_ORIENTATION_VALID`"]
    pub const ORIENTATION_VALID: HandJointFlags = HandJointFlags {
        ord: 1u64
    };
    #[doc(alias = "HAND_JOINT_FLAG_ORIENTATION_TRACKED")]
    #[doc = "Godot enumerator name: `HAND_JOINT_FLAG_ORIENTATION_TRACKED`"]
    pub const ORIENTATION_TRACKED: HandJointFlags = HandJointFlags {
        ord: 2u64
    };
    #[doc(alias = "HAND_JOINT_FLAG_POSITION_VALID")]
    #[doc = "Godot enumerator name: `HAND_JOINT_FLAG_POSITION_VALID`"]
    pub const POSITION_VALID: HandJointFlags = HandJointFlags {
        ord: 4u64
    };
    #[doc(alias = "HAND_JOINT_FLAG_POSITION_TRACKED")]
    #[doc = "Godot enumerator name: `HAND_JOINT_FLAG_POSITION_TRACKED`"]
    pub const POSITION_TRACKED: HandJointFlags = HandJointFlags {
        ord: 8u64
    };
    #[doc(alias = "HAND_JOINT_FLAG_LINEAR_VELOCITY_VALID")]
    #[doc = "Godot enumerator name: `HAND_JOINT_FLAG_LINEAR_VELOCITY_VALID`"]
    pub const LINEAR_VELOCITY_VALID: HandJointFlags = HandJointFlags {
        ord: 16u64
    };
    #[doc(alias = "HAND_JOINT_FLAG_ANGULAR_VELOCITY_VALID")]
    #[doc = "Godot enumerator name: `HAND_JOINT_FLAG_ANGULAR_VELOCITY_VALID`"]
    pub const ANGULAR_VELOCITY_VALID: HandJointFlags = HandJointFlags {
        ord: 32u64
    };
    
}
impl std::fmt::Debug for HandJointFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::ORIENTATION_VALID => "ORIENTATION_VALID", Self::ORIENTATION_TRACKED => "ORIENTATION_TRACKED", Self::POSITION_VALID => "POSITION_VALID", Self::POSITION_TRACKED => "POSITION_TRACKED", Self::LINEAR_VELOCITY_VALID => "LINEAR_VELOCITY_VALID", Self::ANGULAR_VELOCITY_VALID => "ANGULAR_VELOCITY_VALID", _ => {
                f.debug_struct("HandJointFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for HandJointFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < HandJointFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ORIENTATION_VALID", "HAND_JOINT_FLAG_ORIENTATION_VALID", HandJointFlags::ORIENTATION_VALID), crate::meta::inspect::EnumConstant::new("ORIENTATION_TRACKED", "HAND_JOINT_FLAG_ORIENTATION_TRACKED", HandJointFlags::ORIENTATION_TRACKED), crate::meta::inspect::EnumConstant::new("POSITION_VALID", "HAND_JOINT_FLAG_POSITION_VALID", HandJointFlags::POSITION_VALID), crate::meta::inspect::EnumConstant::new("POSITION_TRACKED", "HAND_JOINT_FLAG_POSITION_TRACKED", HandJointFlags::POSITION_TRACKED), crate::meta::inspect::EnumConstant::new("LINEAR_VELOCITY_VALID", "HAND_JOINT_FLAG_LINEAR_VELOCITY_VALID", HandJointFlags::LINEAR_VELOCITY_VALID), crate::meta::inspect::EnumConstant::new("ANGULAR_VELOCITY_VALID", "HAND_JOINT_FLAG_ANGULAR_VELOCITY_VALID", HandJointFlags::ANGULAR_VELOCITY_VALID)]
        }
    }
}
impl std::ops::BitOr for HandJointFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for HandJointFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for HandJointFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for HandJointFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HandJointFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::XrHandTracker;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::xr_positional_tracker::SignalsOfXrPositionalTracker;
    impl WithSignals for XrHandTracker {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfXrPositionalTracker < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}