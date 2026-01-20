#![doc = "Sidecar module for class [`OpenXrInterface`][crate::classes::OpenXrInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRInterface` enums](https://docs.godotengine.org/en/stable/classes/class_openxrinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRInterface.`\n\nInherits [`XrInterface`][crate::classes::XrInterface].\n\nRelated symbols:\n\n* [`open_xr_interface`][crate::classes::open_xr_interface]: sidecar module with related enum/flag types\n* [`IOpenXrInterface`][crate::classes::IOpenXrInterface]: virtual methods\n* [`SignalsOfOpenXrInterface`][crate::classes::open_xr_interface::SignalsOfOpenXrInterface]: signal collection\n\n\nSee also [Godot docs for `OpenXRInterface`](https://docs.godotengine.org/en/stable/classes/class_openxrinterface.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`OpenXrInterface::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`OpenXrInterface`][crate::classes::OpenXrInterface].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IXrInterface`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `OpenXRInterface` methods](https://docs.godotengine.org/en/stable/classes/class_openxrinterface.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrInterface: crate::obj::GodotClass < Base = OpenXrInterface > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl OpenXrInterface {
        pub fn get_session_state(&mut self,) -> crate::classes::open_xr_interface::SessionState {
            type CallRet = crate::classes::open_xr_interface::SessionState;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_session_state", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_display_refresh_rate(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_display_refresh_rate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_display_refresh_rate(&mut self, refresh_rate: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (refresh_rate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_display_refresh_rate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_render_target_size_multiplier(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_render_target_size_multiplier", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_render_target_size_multiplier(&mut self, multiplier: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (multiplier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_render_target_size_multiplier", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_foveation_supported(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "is_foveation_supported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_foveation_level(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_foveation_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_foveation_level(&mut self, foveation_level: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (foveation_level,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_foveation_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_foveation_dynamic(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_foveation_dynamic", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_foveation_dynamic(&mut self, foveation_dynamic: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (foveation_dynamic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_foveation_dynamic", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_action_set_active(&self, name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "is_action_set_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_action_set_active(&mut self, name: impl AsArg < GString >, active: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (name.into_arg(), active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_action_set_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_action_sets(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_action_sets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_available_display_refresh_rates(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_available_display_refresh_rates", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_motion_range(&mut self, hand: crate::classes::open_xr_interface::Hand, motion_range: crate::classes::open_xr_interface::HandMotionRange,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_interface::Hand, crate::classes::open_xr_interface::HandMotionRange,);
            let args = (hand, motion_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_motion_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_motion_range(&self, hand: crate::classes::open_xr_interface::Hand,) -> crate::classes::open_xr_interface::HandMotionRange {
            type CallRet = crate::classes::open_xr_interface::HandMotionRange;
            type CallParams = (crate::classes::open_xr_interface::Hand,);
            let args = (hand,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_motion_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hand_tracking_source(&self, hand: crate::classes::open_xr_interface::Hand,) -> crate::classes::open_xr_interface::HandTrackedSource {
            type CallRet = crate::classes::open_xr_interface::HandTrackedSource;
            type CallParams = (crate::classes::open_xr_interface::Hand,);
            let args = (hand,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_hand_tracking_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hand_joint_flags(&self, hand: crate::classes::open_xr_interface::Hand, joint: crate::classes::open_xr_interface::HandJoints,) -> crate::classes::open_xr_interface::HandJointFlags {
            type CallRet = crate::classes::open_xr_interface::HandJointFlags;
            type CallParams = (crate::classes::open_xr_interface::Hand, crate::classes::open_xr_interface::HandJoints,);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_hand_joint_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hand_joint_rotation(&self, hand: crate::classes::open_xr_interface::Hand, joint: crate::classes::open_xr_interface::HandJoints,) -> Quaternion {
            type CallRet = Quaternion;
            type CallParams = (crate::classes::open_xr_interface::Hand, crate::classes::open_xr_interface::HandJoints,);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_hand_joint_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hand_joint_position(&self, hand: crate::classes::open_xr_interface::Hand, joint: crate::classes::open_xr_interface::HandJoints,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (crate::classes::open_xr_interface::Hand, crate::classes::open_xr_interface::HandJoints,);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_hand_joint_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hand_joint_radius(&self, hand: crate::classes::open_xr_interface::Hand, joint: crate::classes::open_xr_interface::HandJoints,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::classes::open_xr_interface::Hand, crate::classes::open_xr_interface::HandJoints,);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_hand_joint_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hand_joint_linear_velocity(&self, hand: crate::classes::open_xr_interface::Hand, joint: crate::classes::open_xr_interface::HandJoints,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (crate::classes::open_xr_interface::Hand, crate::classes::open_xr_interface::HandJoints,);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_hand_joint_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hand_joint_angular_velocity(&self, hand: crate::classes::open_xr_interface::Hand, joint: crate::classes::open_xr_interface::HandJoints,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (crate::classes::open_xr_interface::Hand, crate::classes::open_xr_interface::HandJoints,);
            let args = (hand, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_hand_joint_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_hand_tracking_supported(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "is_hand_tracking_supported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_hand_interaction_supported(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "is_hand_interaction_supported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_eye_gaze_interaction_supported(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "is_eye_gaze_interaction_supported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vrs_min_radius(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_vrs_min_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vrs_min_radius(&mut self, radius: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_vrs_min_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vrs_strength(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "get_vrs_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vrs_strength(&mut self, strength: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_vrs_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cpu_level(&mut self, level: crate::classes::open_xr_interface::PerfSettingsLevel,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_interface::PerfSettingsLevel,);
            let args = (level,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_cpu_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gpu_level(&mut self, level: crate::classes::open_xr_interface::PerfSettingsLevel,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_interface::PerfSettingsLevel,);
            let args = (level,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInterface", "set_gpu_level", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for OpenXrInterface {
        type Base = crate::classes::XrInterface;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OpenXRInterface"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrInterface {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrInterface > for OpenXrInterface {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for OpenXrInterface {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrInterface {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrInterface {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrInterface {
        type Target = crate::classes::XrInterface;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrInterface`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrInterface__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrInterface > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SessionState {
    ord: i32
}
impl SessionState {
    #[doc(alias = "SESSION_STATE_UNKNOWN")]
    #[doc = "Godot enumerator name: `SESSION_STATE_UNKNOWN`"]
    pub const UNKNOWN: SessionState = SessionState {
        ord: 0i32
    };
    #[doc(alias = "SESSION_STATE_IDLE")]
    #[doc = "Godot enumerator name: `SESSION_STATE_IDLE`"]
    pub const IDLE: SessionState = SessionState {
        ord: 1i32
    };
    #[doc(alias = "SESSION_STATE_READY")]
    #[doc = "Godot enumerator name: `SESSION_STATE_READY`"]
    pub const READY: SessionState = SessionState {
        ord: 2i32
    };
    #[doc(alias = "SESSION_STATE_SYNCHRONIZED")]
    #[doc = "Godot enumerator name: `SESSION_STATE_SYNCHRONIZED`"]
    pub const SYNCHRONIZED: SessionState = SessionState {
        ord: 3i32
    };
    #[doc(alias = "SESSION_STATE_VISIBLE")]
    #[doc = "Godot enumerator name: `SESSION_STATE_VISIBLE`"]
    pub const VISIBLE: SessionState = SessionState {
        ord: 4i32
    };
    #[doc(alias = "SESSION_STATE_FOCUSED")]
    #[doc = "Godot enumerator name: `SESSION_STATE_FOCUSED`"]
    pub const FOCUSED: SessionState = SessionState {
        ord: 5i32
    };
    #[doc(alias = "SESSION_STATE_STOPPING")]
    #[doc = "Godot enumerator name: `SESSION_STATE_STOPPING`"]
    pub const STOPPING: SessionState = SessionState {
        ord: 6i32
    };
    #[doc(alias = "SESSION_STATE_LOSS_PENDING")]
    #[doc = "Godot enumerator name: `SESSION_STATE_LOSS_PENDING`"]
    pub const LOSS_PENDING: SessionState = SessionState {
        ord: 7i32
    };
    #[doc(alias = "SESSION_STATE_EXITING")]
    #[doc = "Godot enumerator name: `SESSION_STATE_EXITING`"]
    pub const EXITING: SessionState = SessionState {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for SessionState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SessionState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SessionState {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::UNKNOWN => "UNKNOWN", Self::IDLE => "IDLE", Self::READY => "READY", Self::SYNCHRONIZED => "SYNCHRONIZED", Self::VISIBLE => "VISIBLE", Self::FOCUSED => "FOCUSED", Self::STOPPING => "STOPPING", Self::LOSS_PENDING => "LOSS_PENDING", Self::EXITING => "EXITING", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SessionState::UNKNOWN, SessionState::IDLE, SessionState::READY, SessionState::SYNCHRONIZED, SessionState::VISIBLE, SessionState::FOCUSED, SessionState::STOPPING, SessionState::LOSS_PENDING, SessionState::EXITING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SessionState >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNKNOWN", "SESSION_STATE_UNKNOWN", SessionState::UNKNOWN), crate::meta::inspect::EnumConstant::new("IDLE", "SESSION_STATE_IDLE", SessionState::IDLE), crate::meta::inspect::EnumConstant::new("READY", "SESSION_STATE_READY", SessionState::READY), crate::meta::inspect::EnumConstant::new("SYNCHRONIZED", "SESSION_STATE_SYNCHRONIZED", SessionState::SYNCHRONIZED), crate::meta::inspect::EnumConstant::new("VISIBLE", "SESSION_STATE_VISIBLE", SessionState::VISIBLE), crate::meta::inspect::EnumConstant::new("FOCUSED", "SESSION_STATE_FOCUSED", SessionState::FOCUSED), crate::meta::inspect::EnumConstant::new("STOPPING", "SESSION_STATE_STOPPING", SessionState::STOPPING), crate::meta::inspect::EnumConstant::new("LOSS_PENDING", "SESSION_STATE_LOSS_PENDING", SessionState::LOSS_PENDING), crate::meta::inspect::EnumConstant::new("EXITING", "SESSION_STATE_EXITING", SessionState::EXITING)]
        }
    }
}
impl crate::meta::GodotConvert for SessionState {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SessionState {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SessionState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Hand {
    ord: i32
}
impl Hand {
    #[doc(alias = "HAND_LEFT")]
    #[doc = "Godot enumerator name: `HAND_LEFT`"]
    pub const LEFT: Hand = Hand {
        ord: 0i32
    };
    #[doc(alias = "HAND_RIGHT")]
    #[doc = "Godot enumerator name: `HAND_RIGHT`"]
    pub const RIGHT: Hand = Hand {
        ord: 1i32
    };
    #[doc(alias = "HAND_MAX")]
    #[doc = "Godot enumerator name: `HAND_MAX`"]
    pub const MAX: Hand = Hand {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Hand") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Hand {
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
            Self::LEFT => "LEFT", Self::RIGHT => "RIGHT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Hand::LEFT, Hand::RIGHT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Hand >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LEFT", "HAND_LEFT", Hand::LEFT), crate::meta::inspect::EnumConstant::new("RIGHT", "HAND_RIGHT", Hand::RIGHT), crate::meta::inspect::EnumConstant::new("MAX", "HAND_MAX", Hand::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Hand {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for Hand {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Hand {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Hand {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HandMotionRange {
    ord: i32
}
impl HandMotionRange {
    #[doc(alias = "HAND_MOTION_RANGE_UNOBSTRUCTED")]
    #[doc = "Godot enumerator name: `HAND_MOTION_RANGE_UNOBSTRUCTED`"]
    pub const UNOBSTRUCTED: HandMotionRange = HandMotionRange {
        ord: 0i32
    };
    #[doc(alias = "HAND_MOTION_RANGE_CONFORM_TO_CONTROLLER")]
    #[doc = "Godot enumerator name: `HAND_MOTION_RANGE_CONFORM_TO_CONTROLLER`"]
    pub const CONFORM_TO_CONTROLLER: HandMotionRange = HandMotionRange {
        ord: 1i32
    };
    #[doc(alias = "HAND_MOTION_RANGE_MAX")]
    #[doc = "Godot enumerator name: `HAND_MOTION_RANGE_MAX`"]
    pub const MAX: HandMotionRange = HandMotionRange {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for HandMotionRange {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HandMotionRange") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HandMotionRange {
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
            Self::UNOBSTRUCTED => "UNOBSTRUCTED", Self::CONFORM_TO_CONTROLLER => "CONFORM_TO_CONTROLLER", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[HandMotionRange::UNOBSTRUCTED, HandMotionRange::CONFORM_TO_CONTROLLER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < HandMotionRange >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNOBSTRUCTED", "HAND_MOTION_RANGE_UNOBSTRUCTED", HandMotionRange::UNOBSTRUCTED), crate::meta::inspect::EnumConstant::new("CONFORM_TO_CONTROLLER", "HAND_MOTION_RANGE_CONFORM_TO_CONTROLLER", HandMotionRange::CONFORM_TO_CONTROLLER), crate::meta::inspect::EnumConstant::new("MAX", "HAND_MOTION_RANGE_MAX", HandMotionRange::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for HandMotionRange {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for HandMotionRange {
    type Via = i32;
    
}
impl crate::meta::ToGodot for HandMotionRange {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HandMotionRange {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HandTrackedSource {
    ord: i32
}
impl HandTrackedSource {
    #[doc(alias = "HAND_TRACKED_SOURCE_UNKNOWN")]
    #[doc = "Godot enumerator name: `HAND_TRACKED_SOURCE_UNKNOWN`"]
    pub const UNKNOWN: HandTrackedSource = HandTrackedSource {
        ord: 0i32
    };
    #[doc(alias = "HAND_TRACKED_SOURCE_UNOBSTRUCTED")]
    #[doc = "Godot enumerator name: `HAND_TRACKED_SOURCE_UNOBSTRUCTED`"]
    pub const UNOBSTRUCTED: HandTrackedSource = HandTrackedSource {
        ord: 1i32
    };
    #[doc(alias = "HAND_TRACKED_SOURCE_CONTROLLER")]
    #[doc = "Godot enumerator name: `HAND_TRACKED_SOURCE_CONTROLLER`"]
    pub const CONTROLLER: HandTrackedSource = HandTrackedSource {
        ord: 2i32
    };
    #[doc(alias = "HAND_TRACKED_SOURCE_MAX")]
    #[doc = "Godot enumerator name: `HAND_TRACKED_SOURCE_MAX`"]
    pub const MAX: HandTrackedSource = HandTrackedSource {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for HandTrackedSource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HandTrackedSource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HandTrackedSource {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::UNKNOWN => "UNKNOWN", Self::UNOBSTRUCTED => "UNOBSTRUCTED", Self::CONTROLLER => "CONTROLLER", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[HandTrackedSource::UNKNOWN, HandTrackedSource::UNOBSTRUCTED, HandTrackedSource::CONTROLLER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < HandTrackedSource >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNKNOWN", "HAND_TRACKED_SOURCE_UNKNOWN", HandTrackedSource::UNKNOWN), crate::meta::inspect::EnumConstant::new("UNOBSTRUCTED", "HAND_TRACKED_SOURCE_UNOBSTRUCTED", HandTrackedSource::UNOBSTRUCTED), crate::meta::inspect::EnumConstant::new("CONTROLLER", "HAND_TRACKED_SOURCE_CONTROLLER", HandTrackedSource::CONTROLLER), crate::meta::inspect::EnumConstant::new("MAX", "HAND_TRACKED_SOURCE_MAX", HandTrackedSource::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for HandTrackedSource {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for HandTrackedSource {
    type Via = i32;
    
}
impl crate::meta::ToGodot for HandTrackedSource {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HandTrackedSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HandJoints {
    ord: i32
}
impl HandJoints {
    #[doc(alias = "HAND_JOINT_PALM")]
    #[doc = "Godot enumerator name: `HAND_JOINT_PALM`"]
    pub const PALM: HandJoints = HandJoints {
        ord: 0i32
    };
    #[doc(alias = "HAND_JOINT_WRIST")]
    #[doc = "Godot enumerator name: `HAND_JOINT_WRIST`"]
    pub const WRIST: HandJoints = HandJoints {
        ord: 1i32
    };
    #[doc(alias = "HAND_JOINT_THUMB_METACARPAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_THUMB_METACARPAL`"]
    pub const THUMB_METACARPAL: HandJoints = HandJoints {
        ord: 2i32
    };
    #[doc(alias = "HAND_JOINT_THUMB_PROXIMAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_THUMB_PROXIMAL`"]
    pub const THUMB_PROXIMAL: HandJoints = HandJoints {
        ord: 3i32
    };
    #[doc(alias = "HAND_JOINT_THUMB_DISTAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_THUMB_DISTAL`"]
    pub const THUMB_DISTAL: HandJoints = HandJoints {
        ord: 4i32
    };
    #[doc(alias = "HAND_JOINT_THUMB_TIP")]
    #[doc = "Godot enumerator name: `HAND_JOINT_THUMB_TIP`"]
    pub const THUMB_TIP: HandJoints = HandJoints {
        ord: 5i32
    };
    #[doc(alias = "HAND_JOINT_INDEX_METACARPAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_INDEX_METACARPAL`"]
    pub const INDEX_METACARPAL: HandJoints = HandJoints {
        ord: 6i32
    };
    #[doc(alias = "HAND_JOINT_INDEX_PROXIMAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_INDEX_PROXIMAL`"]
    pub const INDEX_PROXIMAL: HandJoints = HandJoints {
        ord: 7i32
    };
    #[doc(alias = "HAND_JOINT_INDEX_INTERMEDIATE")]
    #[doc = "Godot enumerator name: `HAND_JOINT_INDEX_INTERMEDIATE`"]
    pub const INDEX_INTERMEDIATE: HandJoints = HandJoints {
        ord: 8i32
    };
    #[doc(alias = "HAND_JOINT_INDEX_DISTAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_INDEX_DISTAL`"]
    pub const INDEX_DISTAL: HandJoints = HandJoints {
        ord: 9i32
    };
    #[doc(alias = "HAND_JOINT_INDEX_TIP")]
    #[doc = "Godot enumerator name: `HAND_JOINT_INDEX_TIP`"]
    pub const INDEX_TIP: HandJoints = HandJoints {
        ord: 10i32
    };
    #[doc(alias = "HAND_JOINT_MIDDLE_METACARPAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MIDDLE_METACARPAL`"]
    pub const MIDDLE_METACARPAL: HandJoints = HandJoints {
        ord: 11i32
    };
    #[doc(alias = "HAND_JOINT_MIDDLE_PROXIMAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MIDDLE_PROXIMAL`"]
    pub const MIDDLE_PROXIMAL: HandJoints = HandJoints {
        ord: 12i32
    };
    #[doc(alias = "HAND_JOINT_MIDDLE_INTERMEDIATE")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MIDDLE_INTERMEDIATE`"]
    pub const MIDDLE_INTERMEDIATE: HandJoints = HandJoints {
        ord: 13i32
    };
    #[doc(alias = "HAND_JOINT_MIDDLE_DISTAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MIDDLE_DISTAL`"]
    pub const MIDDLE_DISTAL: HandJoints = HandJoints {
        ord: 14i32
    };
    #[doc(alias = "HAND_JOINT_MIDDLE_TIP")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MIDDLE_TIP`"]
    pub const MIDDLE_TIP: HandJoints = HandJoints {
        ord: 15i32
    };
    #[doc(alias = "HAND_JOINT_RING_METACARPAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_RING_METACARPAL`"]
    pub const RING_METACARPAL: HandJoints = HandJoints {
        ord: 16i32
    };
    #[doc(alias = "HAND_JOINT_RING_PROXIMAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_RING_PROXIMAL`"]
    pub const RING_PROXIMAL: HandJoints = HandJoints {
        ord: 17i32
    };
    #[doc(alias = "HAND_JOINT_RING_INTERMEDIATE")]
    #[doc = "Godot enumerator name: `HAND_JOINT_RING_INTERMEDIATE`"]
    pub const RING_INTERMEDIATE: HandJoints = HandJoints {
        ord: 18i32
    };
    #[doc(alias = "HAND_JOINT_RING_DISTAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_RING_DISTAL`"]
    pub const RING_DISTAL: HandJoints = HandJoints {
        ord: 19i32
    };
    #[doc(alias = "HAND_JOINT_RING_TIP")]
    #[doc = "Godot enumerator name: `HAND_JOINT_RING_TIP`"]
    pub const RING_TIP: HandJoints = HandJoints {
        ord: 20i32
    };
    #[doc(alias = "HAND_JOINT_LITTLE_METACARPAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_LITTLE_METACARPAL`"]
    pub const LITTLE_METACARPAL: HandJoints = HandJoints {
        ord: 21i32
    };
    #[doc(alias = "HAND_JOINT_LITTLE_PROXIMAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_LITTLE_PROXIMAL`"]
    pub const LITTLE_PROXIMAL: HandJoints = HandJoints {
        ord: 22i32
    };
    #[doc(alias = "HAND_JOINT_LITTLE_INTERMEDIATE")]
    #[doc = "Godot enumerator name: `HAND_JOINT_LITTLE_INTERMEDIATE`"]
    pub const LITTLE_INTERMEDIATE: HandJoints = HandJoints {
        ord: 23i32
    };
    #[doc(alias = "HAND_JOINT_LITTLE_DISTAL")]
    #[doc = "Godot enumerator name: `HAND_JOINT_LITTLE_DISTAL`"]
    pub const LITTLE_DISTAL: HandJoints = HandJoints {
        ord: 24i32
    };
    #[doc(alias = "HAND_JOINT_LITTLE_TIP")]
    #[doc = "Godot enumerator name: `HAND_JOINT_LITTLE_TIP`"]
    pub const LITTLE_TIP: HandJoints = HandJoints {
        ord: 25i32
    };
    #[doc(alias = "HAND_JOINT_MAX")]
    #[doc = "Godot enumerator name: `HAND_JOINT_MAX`"]
    pub const MAX: HandJoints = HandJoints {
        ord: 26i32
    };
    
}
impl std::fmt::Debug for HandJoints {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HandJoints") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HandJoints {
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
            Self::PALM => "PALM", Self::WRIST => "WRIST", Self::THUMB_METACARPAL => "THUMB_METACARPAL", Self::THUMB_PROXIMAL => "THUMB_PROXIMAL", Self::THUMB_DISTAL => "THUMB_DISTAL", Self::THUMB_TIP => "THUMB_TIP", Self::INDEX_METACARPAL => "INDEX_METACARPAL", Self::INDEX_PROXIMAL => "INDEX_PROXIMAL", Self::INDEX_INTERMEDIATE => "INDEX_INTERMEDIATE", Self::INDEX_DISTAL => "INDEX_DISTAL", Self::INDEX_TIP => "INDEX_TIP", Self::MIDDLE_METACARPAL => "MIDDLE_METACARPAL", Self::MIDDLE_PROXIMAL => "MIDDLE_PROXIMAL", Self::MIDDLE_INTERMEDIATE => "MIDDLE_INTERMEDIATE", Self::MIDDLE_DISTAL => "MIDDLE_DISTAL", Self::MIDDLE_TIP => "MIDDLE_TIP", Self::RING_METACARPAL => "RING_METACARPAL", Self::RING_PROXIMAL => "RING_PROXIMAL", Self::RING_INTERMEDIATE => "RING_INTERMEDIATE", Self::RING_DISTAL => "RING_DISTAL", Self::RING_TIP => "RING_TIP", Self::LITTLE_METACARPAL => "LITTLE_METACARPAL", Self::LITTLE_PROXIMAL => "LITTLE_PROXIMAL", Self::LITTLE_INTERMEDIATE => "LITTLE_INTERMEDIATE", Self::LITTLE_DISTAL => "LITTLE_DISTAL", Self::LITTLE_TIP => "LITTLE_TIP", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[HandJoints::PALM, HandJoints::WRIST, HandJoints::THUMB_METACARPAL, HandJoints::THUMB_PROXIMAL, HandJoints::THUMB_DISTAL, HandJoints::THUMB_TIP, HandJoints::INDEX_METACARPAL, HandJoints::INDEX_PROXIMAL, HandJoints::INDEX_INTERMEDIATE, HandJoints::INDEX_DISTAL, HandJoints::INDEX_TIP, HandJoints::MIDDLE_METACARPAL, HandJoints::MIDDLE_PROXIMAL, HandJoints::MIDDLE_INTERMEDIATE, HandJoints::MIDDLE_DISTAL, HandJoints::MIDDLE_TIP, HandJoints::RING_METACARPAL, HandJoints::RING_PROXIMAL, HandJoints::RING_INTERMEDIATE, HandJoints::RING_DISTAL, HandJoints::RING_TIP, HandJoints::LITTLE_METACARPAL, HandJoints::LITTLE_PROXIMAL, HandJoints::LITTLE_INTERMEDIATE, HandJoints::LITTLE_DISTAL, HandJoints::LITTLE_TIP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < HandJoints >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PALM", "HAND_JOINT_PALM", HandJoints::PALM), crate::meta::inspect::EnumConstant::new("WRIST", "HAND_JOINT_WRIST", HandJoints::WRIST), crate::meta::inspect::EnumConstant::new("THUMB_METACARPAL", "HAND_JOINT_THUMB_METACARPAL", HandJoints::THUMB_METACARPAL), crate::meta::inspect::EnumConstant::new("THUMB_PROXIMAL", "HAND_JOINT_THUMB_PROXIMAL", HandJoints::THUMB_PROXIMAL), crate::meta::inspect::EnumConstant::new("THUMB_DISTAL", "HAND_JOINT_THUMB_DISTAL", HandJoints::THUMB_DISTAL), crate::meta::inspect::EnumConstant::new("THUMB_TIP", "HAND_JOINT_THUMB_TIP", HandJoints::THUMB_TIP), crate::meta::inspect::EnumConstant::new("INDEX_METACARPAL", "HAND_JOINT_INDEX_METACARPAL", HandJoints::INDEX_METACARPAL), crate::meta::inspect::EnumConstant::new("INDEX_PROXIMAL", "HAND_JOINT_INDEX_PROXIMAL", HandJoints::INDEX_PROXIMAL), crate::meta::inspect::EnumConstant::new("INDEX_INTERMEDIATE", "HAND_JOINT_INDEX_INTERMEDIATE", HandJoints::INDEX_INTERMEDIATE), crate::meta::inspect::EnumConstant::new("INDEX_DISTAL", "HAND_JOINT_INDEX_DISTAL", HandJoints::INDEX_DISTAL), crate::meta::inspect::EnumConstant::new("INDEX_TIP", "HAND_JOINT_INDEX_TIP", HandJoints::INDEX_TIP), crate::meta::inspect::EnumConstant::new("MIDDLE_METACARPAL", "HAND_JOINT_MIDDLE_METACARPAL", HandJoints::MIDDLE_METACARPAL), crate::meta::inspect::EnumConstant::new("MIDDLE_PROXIMAL", "HAND_JOINT_MIDDLE_PROXIMAL", HandJoints::MIDDLE_PROXIMAL), crate::meta::inspect::EnumConstant::new("MIDDLE_INTERMEDIATE", "HAND_JOINT_MIDDLE_INTERMEDIATE", HandJoints::MIDDLE_INTERMEDIATE), crate::meta::inspect::EnumConstant::new("MIDDLE_DISTAL", "HAND_JOINT_MIDDLE_DISTAL", HandJoints::MIDDLE_DISTAL), crate::meta::inspect::EnumConstant::new("MIDDLE_TIP", "HAND_JOINT_MIDDLE_TIP", HandJoints::MIDDLE_TIP), crate::meta::inspect::EnumConstant::new("RING_METACARPAL", "HAND_JOINT_RING_METACARPAL", HandJoints::RING_METACARPAL), crate::meta::inspect::EnumConstant::new("RING_PROXIMAL", "HAND_JOINT_RING_PROXIMAL", HandJoints::RING_PROXIMAL), crate::meta::inspect::EnumConstant::new("RING_INTERMEDIATE", "HAND_JOINT_RING_INTERMEDIATE", HandJoints::RING_INTERMEDIATE), crate::meta::inspect::EnumConstant::new("RING_DISTAL", "HAND_JOINT_RING_DISTAL", HandJoints::RING_DISTAL), crate::meta::inspect::EnumConstant::new("RING_TIP", "HAND_JOINT_RING_TIP", HandJoints::RING_TIP), crate::meta::inspect::EnumConstant::new("LITTLE_METACARPAL", "HAND_JOINT_LITTLE_METACARPAL", HandJoints::LITTLE_METACARPAL), crate::meta::inspect::EnumConstant::new("LITTLE_PROXIMAL", "HAND_JOINT_LITTLE_PROXIMAL", HandJoints::LITTLE_PROXIMAL), crate::meta::inspect::EnumConstant::new("LITTLE_INTERMEDIATE", "HAND_JOINT_LITTLE_INTERMEDIATE", HandJoints::LITTLE_INTERMEDIATE), crate::meta::inspect::EnumConstant::new("LITTLE_DISTAL", "HAND_JOINT_LITTLE_DISTAL", HandJoints::LITTLE_DISTAL), crate::meta::inspect::EnumConstant::new("LITTLE_TIP", "HAND_JOINT_LITTLE_TIP", HandJoints::LITTLE_TIP), crate::meta::inspect::EnumConstant::new("MAX", "HAND_JOINT_MAX", HandJoints::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for HandJoints {
    const ENUMERATOR_COUNT: usize = 26usize;
    
}
impl crate::meta::GodotConvert for HandJoints {
    type Via = i32;
    
}
impl crate::meta::ToGodot for HandJoints {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HandJoints {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PerfSettingsLevel {
    ord: i32
}
impl PerfSettingsLevel {
    #[doc(alias = "PERF_SETTINGS_LEVEL_POWER_SAVINGS")]
    #[doc = "Godot enumerator name: `PERF_SETTINGS_LEVEL_POWER_SAVINGS`"]
    pub const POWER_SAVINGS: PerfSettingsLevel = PerfSettingsLevel {
        ord: 0i32
    };
    #[doc(alias = "PERF_SETTINGS_LEVEL_SUSTAINED_LOW")]
    #[doc = "Godot enumerator name: `PERF_SETTINGS_LEVEL_SUSTAINED_LOW`"]
    pub const SUSTAINED_LOW: PerfSettingsLevel = PerfSettingsLevel {
        ord: 1i32
    };
    #[doc(alias = "PERF_SETTINGS_LEVEL_SUSTAINED_HIGH")]
    #[doc = "Godot enumerator name: `PERF_SETTINGS_LEVEL_SUSTAINED_HIGH`"]
    pub const SUSTAINED_HIGH: PerfSettingsLevel = PerfSettingsLevel {
        ord: 2i32
    };
    #[doc(alias = "PERF_SETTINGS_LEVEL_BOOST")]
    #[doc = "Godot enumerator name: `PERF_SETTINGS_LEVEL_BOOST`"]
    pub const BOOST: PerfSettingsLevel = PerfSettingsLevel {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for PerfSettingsLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PerfSettingsLevel") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PerfSettingsLevel {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::POWER_SAVINGS => "POWER_SAVINGS", Self::SUSTAINED_LOW => "SUSTAINED_LOW", Self::SUSTAINED_HIGH => "SUSTAINED_HIGH", Self::BOOST => "BOOST", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PerfSettingsLevel::POWER_SAVINGS, PerfSettingsLevel::SUSTAINED_LOW, PerfSettingsLevel::SUSTAINED_HIGH, PerfSettingsLevel::BOOST]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PerfSettingsLevel >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("POWER_SAVINGS", "PERF_SETTINGS_LEVEL_POWER_SAVINGS", PerfSettingsLevel::POWER_SAVINGS), crate::meta::inspect::EnumConstant::new("SUSTAINED_LOW", "PERF_SETTINGS_LEVEL_SUSTAINED_LOW", PerfSettingsLevel::SUSTAINED_LOW), crate::meta::inspect::EnumConstant::new("SUSTAINED_HIGH", "PERF_SETTINGS_LEVEL_SUSTAINED_HIGH", PerfSettingsLevel::SUSTAINED_HIGH), crate::meta::inspect::EnumConstant::new("BOOST", "PERF_SETTINGS_LEVEL_BOOST", PerfSettingsLevel::BOOST)]
        }
    }
}
impl crate::meta::GodotConvert for PerfSettingsLevel {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PerfSettingsLevel {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PerfSettingsLevel {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PerfSettingsSubDomain {
    ord: i32
}
impl PerfSettingsSubDomain {
    #[doc(alias = "PERF_SETTINGS_SUB_DOMAIN_COMPOSITING")]
    #[doc = "Godot enumerator name: `PERF_SETTINGS_SUB_DOMAIN_COMPOSITING`"]
    pub const COMPOSITING: PerfSettingsSubDomain = PerfSettingsSubDomain {
        ord: 0i32
    };
    #[doc(alias = "PERF_SETTINGS_SUB_DOMAIN_RENDERING")]
    #[doc = "Godot enumerator name: `PERF_SETTINGS_SUB_DOMAIN_RENDERING`"]
    pub const RENDERING: PerfSettingsSubDomain = PerfSettingsSubDomain {
        ord: 1i32
    };
    #[doc(alias = "PERF_SETTINGS_SUB_DOMAIN_THERMAL")]
    #[doc = "Godot enumerator name: `PERF_SETTINGS_SUB_DOMAIN_THERMAL`"]
    pub const THERMAL: PerfSettingsSubDomain = PerfSettingsSubDomain {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PerfSettingsSubDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PerfSettingsSubDomain") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PerfSettingsSubDomain {
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
            Self::COMPOSITING => "COMPOSITING", Self::RENDERING => "RENDERING", Self::THERMAL => "THERMAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PerfSettingsSubDomain::COMPOSITING, PerfSettingsSubDomain::RENDERING, PerfSettingsSubDomain::THERMAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PerfSettingsSubDomain >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("COMPOSITING", "PERF_SETTINGS_SUB_DOMAIN_COMPOSITING", PerfSettingsSubDomain::COMPOSITING), crate::meta::inspect::EnumConstant::new("RENDERING", "PERF_SETTINGS_SUB_DOMAIN_RENDERING", PerfSettingsSubDomain::RENDERING), crate::meta::inspect::EnumConstant::new("THERMAL", "PERF_SETTINGS_SUB_DOMAIN_THERMAL", PerfSettingsSubDomain::THERMAL)]
        }
    }
}
impl crate::meta::GodotConvert for PerfSettingsSubDomain {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PerfSettingsSubDomain {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PerfSettingsSubDomain {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PerfSettingsNotificationLevel {
    ord: i32
}
impl PerfSettingsNotificationLevel {
    #[doc(alias = "PERF_SETTINGS_NOTIF_LEVEL_NORMAL")]
    #[doc = "Godot enumerator name: `PERF_SETTINGS_NOTIF_LEVEL_NORMAL`"]
    pub const NORMAL: PerfSettingsNotificationLevel = PerfSettingsNotificationLevel {
        ord: 0i32
    };
    #[doc(alias = "PERF_SETTINGS_NOTIF_LEVEL_WARNING")]
    #[doc = "Godot enumerator name: `PERF_SETTINGS_NOTIF_LEVEL_WARNING`"]
    pub const WARNING: PerfSettingsNotificationLevel = PerfSettingsNotificationLevel {
        ord: 1i32
    };
    #[doc(alias = "PERF_SETTINGS_NOTIF_LEVEL_IMPAIRED")]
    #[doc = "Godot enumerator name: `PERF_SETTINGS_NOTIF_LEVEL_IMPAIRED`"]
    pub const IMPAIRED: PerfSettingsNotificationLevel = PerfSettingsNotificationLevel {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PerfSettingsNotificationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PerfSettingsNotificationLevel") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PerfSettingsNotificationLevel {
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
            Self::NORMAL => "NORMAL", Self::WARNING => "WARNING", Self::IMPAIRED => "IMPAIRED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PerfSettingsNotificationLevel::NORMAL, PerfSettingsNotificationLevel::WARNING, PerfSettingsNotificationLevel::IMPAIRED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PerfSettingsNotificationLevel >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NORMAL", "PERF_SETTINGS_NOTIF_LEVEL_NORMAL", PerfSettingsNotificationLevel::NORMAL), crate::meta::inspect::EnumConstant::new("WARNING", "PERF_SETTINGS_NOTIF_LEVEL_WARNING", PerfSettingsNotificationLevel::WARNING), crate::meta::inspect::EnumConstant::new("IMPAIRED", "PERF_SETTINGS_NOTIF_LEVEL_IMPAIRED", PerfSettingsNotificationLevel::IMPAIRED)]
        }
    }
}
impl crate::meta::GodotConvert for PerfSettingsNotificationLevel {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PerfSettingsNotificationLevel {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PerfSettingsNotificationLevel {
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
    #[doc(alias = "HAND_JOINT_NONE")]
    #[doc = "Godot enumerator name: `HAND_JOINT_NONE`"]
    pub const NONE: HandJointFlags = HandJointFlags {
        ord: 0u64
    };
    #[doc(alias = "HAND_JOINT_ORIENTATION_VALID")]
    #[doc = "Godot enumerator name: `HAND_JOINT_ORIENTATION_VALID`"]
    pub const ORIENTATION_VALID: HandJointFlags = HandJointFlags {
        ord: 1u64
    };
    #[doc(alias = "HAND_JOINT_ORIENTATION_TRACKED")]
    #[doc = "Godot enumerator name: `HAND_JOINT_ORIENTATION_TRACKED`"]
    pub const ORIENTATION_TRACKED: HandJointFlags = HandJointFlags {
        ord: 2u64
    };
    #[doc(alias = "HAND_JOINT_POSITION_VALID")]
    #[doc = "Godot enumerator name: `HAND_JOINT_POSITION_VALID`"]
    pub const POSITION_VALID: HandJointFlags = HandJointFlags {
        ord: 4u64
    };
    #[doc(alias = "HAND_JOINT_POSITION_TRACKED")]
    #[doc = "Godot enumerator name: `HAND_JOINT_POSITION_TRACKED`"]
    pub const POSITION_TRACKED: HandJointFlags = HandJointFlags {
        ord: 8u64
    };
    #[doc(alias = "HAND_JOINT_LINEAR_VELOCITY_VALID")]
    #[doc = "Godot enumerator name: `HAND_JOINT_LINEAR_VELOCITY_VALID`"]
    pub const LINEAR_VELOCITY_VALID: HandJointFlags = HandJointFlags {
        ord: 16u64
    };
    #[doc(alias = "HAND_JOINT_ANGULAR_VELOCITY_VALID")]
    #[doc = "Godot enumerator name: `HAND_JOINT_ANGULAR_VELOCITY_VALID`"]
    pub const ANGULAR_VELOCITY_VALID: HandJointFlags = HandJointFlags {
        ord: 32u64
    };
    
}
impl std::fmt::Debug for HandJointFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::NONE => "NONE", Self::ORIENTATION_VALID => "ORIENTATION_VALID", Self::ORIENTATION_TRACKED => "ORIENTATION_TRACKED", Self::POSITION_VALID => "POSITION_VALID", Self::POSITION_TRACKED => "POSITION_TRACKED", Self::LINEAR_VELOCITY_VALID => "LINEAR_VELOCITY_VALID", Self::ANGULAR_VELOCITY_VALID => "ANGULAR_VELOCITY_VALID", _ => {
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
            &[crate::meta::inspect::EnumConstant::new("NONE", "HAND_JOINT_NONE", HandJointFlags::NONE), crate::meta::inspect::EnumConstant::new("ORIENTATION_VALID", "HAND_JOINT_ORIENTATION_VALID", HandJointFlags::ORIENTATION_VALID), crate::meta::inspect::EnumConstant::new("ORIENTATION_TRACKED", "HAND_JOINT_ORIENTATION_TRACKED", HandJointFlags::ORIENTATION_TRACKED), crate::meta::inspect::EnumConstant::new("POSITION_VALID", "HAND_JOINT_POSITION_VALID", HandJointFlags::POSITION_VALID), crate::meta::inspect::EnumConstant::new("POSITION_TRACKED", "HAND_JOINT_POSITION_TRACKED", HandJointFlags::POSITION_TRACKED), crate::meta::inspect::EnumConstant::new("LINEAR_VELOCITY_VALID", "HAND_JOINT_LINEAR_VELOCITY_VALID", HandJointFlags::LINEAR_VELOCITY_VALID), crate::meta::inspect::EnumConstant::new("ANGULAR_VELOCITY_VALID", "HAND_JOINT_ANGULAR_VELOCITY_VALID", HandJointFlags::ANGULAR_VELOCITY_VALID)]
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
    use super::re_export::OpenXrInterface;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`OpenXrInterface`][crate::classes::OpenXrInterface] class."]
    pub struct SignalsOfOpenXrInterface < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfOpenXrInterface < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn session_begun(&mut self) -> SigSessionBegun < 'c, C > {
            SigSessionBegun {
                typed: TypedSignal::extract(&mut self.__internal_obj, "session_begun")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn session_stopping(&mut self) -> SigSessionStopping < 'c, C > {
            SigSessionStopping {
                typed: TypedSignal::extract(&mut self.__internal_obj, "session_stopping")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn session_synchronized(&mut self) -> SigSessionSynchronized < 'c, C > {
            SigSessionSynchronized {
                typed: TypedSignal::extract(&mut self.__internal_obj, "session_synchronized")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn session_focussed(&mut self) -> SigSessionFocussed < 'c, C > {
            SigSessionFocussed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "session_focussed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn session_visible(&mut self) -> SigSessionVisible < 'c, C > {
            SigSessionVisible {
                typed: TypedSignal::extract(&mut self.__internal_obj, "session_visible")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn session_loss_pending(&mut self) -> SigSessionLossPending < 'c, C > {
            SigSessionLossPending {
                typed: TypedSignal::extract(&mut self.__internal_obj, "session_loss_pending")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn instance_exiting(&mut self) -> SigInstanceExiting < 'c, C > {
            SigInstanceExiting {
                typed: TypedSignal::extract(&mut self.__internal_obj, "instance_exiting")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn pose_recentered(&mut self) -> SigPoseRecentered < 'c, C > {
            SigPoseRecentered {
                typed: TypedSignal::extract(&mut self.__internal_obj, "pose_recentered")
            }
        }
        #[doc = "Signature: `(refresh_rate: f64)`"]
        pub fn refresh_rate_changed(&mut self) -> SigRefreshRateChanged < 'c, C > {
            SigRefreshRateChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "refresh_rate_changed")
            }
        }
        #[doc = "Signature: `(sub_domain: i64, from_level: i64, to_level: i64)`"]
        pub fn cpu_level_changed(&mut self) -> SigCpuLevelChanged < 'c, C > {
            SigCpuLevelChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "cpu_level_changed")
            }
        }
        #[doc = "Signature: `(sub_domain: i64, from_level: i64, to_level: i64)`"]
        pub fn gpu_level_changed(&mut self) -> SigGpuLevelChanged < 'c, C > {
            SigGpuLevelChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "gpu_level_changed")
            }
        }
    }
    type TypedSigSessionBegun < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSessionBegun < 'c, C: WithSignals > {
        typed: TypedSigSessionBegun < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSessionBegun < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSessionBegun < 'c, C > {
        type Target = TypedSigSessionBegun < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSessionBegun < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSessionStopping < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSessionStopping < 'c, C: WithSignals > {
        typed: TypedSigSessionStopping < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSessionStopping < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSessionStopping < 'c, C > {
        type Target = TypedSigSessionStopping < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSessionStopping < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSessionSynchronized < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSessionSynchronized < 'c, C: WithSignals > {
        typed: TypedSigSessionSynchronized < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSessionSynchronized < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSessionSynchronized < 'c, C > {
        type Target = TypedSigSessionSynchronized < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSessionSynchronized < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSessionFocussed < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSessionFocussed < 'c, C: WithSignals > {
        typed: TypedSigSessionFocussed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSessionFocussed < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSessionFocussed < 'c, C > {
        type Target = TypedSigSessionFocussed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSessionFocussed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSessionVisible < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSessionVisible < 'c, C: WithSignals > {
        typed: TypedSigSessionVisible < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSessionVisible < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSessionVisible < 'c, C > {
        type Target = TypedSigSessionVisible < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSessionVisible < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSessionLossPending < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSessionLossPending < 'c, C: WithSignals > {
        typed: TypedSigSessionLossPending < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSessionLossPending < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSessionLossPending < 'c, C > {
        type Target = TypedSigSessionLossPending < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSessionLossPending < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigInstanceExiting < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigInstanceExiting < 'c, C: WithSignals > {
        typed: TypedSigInstanceExiting < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigInstanceExiting < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigInstanceExiting < 'c, C > {
        type Target = TypedSigInstanceExiting < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigInstanceExiting < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigPoseRecentered < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigPoseRecentered < 'c, C: WithSignals > {
        typed: TypedSigPoseRecentered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPoseRecentered < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPoseRecentered < 'c, C > {
        type Target = TypedSigPoseRecentered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPoseRecentered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigRefreshRateChanged < 'c, C > = TypedSignal < 'c, C, (f64,) >;
    pub struct SigRefreshRateChanged < 'c, C: WithSignals > {
        typed: TypedSigRefreshRateChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigRefreshRateChanged < 'c, C > {
        pub fn emit(&mut self, refresh_rate: f64,) {
            self.typed.emit_tuple((refresh_rate,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigRefreshRateChanged < 'c, C > {
        type Target = TypedSigRefreshRateChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigRefreshRateChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCpuLevelChanged < 'c, C > = TypedSignal < 'c, C, (i64, i64, i64,) >;
    pub struct SigCpuLevelChanged < 'c, C: WithSignals > {
        typed: TypedSigCpuLevelChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCpuLevelChanged < 'c, C > {
        pub fn emit(&mut self, sub_domain: i64, from_level: i64, to_level: i64,) {
            self.typed.emit_tuple((sub_domain, from_level, to_level,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCpuLevelChanged < 'c, C > {
        type Target = TypedSigCpuLevelChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCpuLevelChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigGpuLevelChanged < 'c, C > = TypedSignal < 'c, C, (i64, i64, i64,) >;
    pub struct SigGpuLevelChanged < 'c, C: WithSignals > {
        typed: TypedSigGpuLevelChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigGpuLevelChanged < 'c, C > {
        pub fn emit(&mut self, sub_domain: i64, from_level: i64, to_level: i64,) {
            self.typed.emit_tuple((sub_domain, from_level, to_level,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigGpuLevelChanged < 'c, C > {
        type Target = TypedSigGpuLevelChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigGpuLevelChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for OpenXrInterface {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfOpenXrInterface < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfOpenXrInterface < 'c, C > {
        type Target = < < OpenXrInterface as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = OpenXrInterface;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfOpenXrInterface < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = OpenXrInterface;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}