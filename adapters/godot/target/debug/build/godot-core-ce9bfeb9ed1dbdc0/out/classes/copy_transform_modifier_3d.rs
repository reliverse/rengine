#![doc = "Sidecar module for class [`CopyTransformModifier3D`][crate::classes::CopyTransformModifier3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CopyTransformModifier3D` enums](https://docs.godotengine.org/en/stable/classes/class_copytransformmodifier3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CopyTransformModifier3D.`\n\nInherits [`BoneConstraint3D`][crate::classes::BoneConstraint3D].\n\nRelated symbols:\n\n* [`copy_transform_modifier_3d`][crate::classes::copy_transform_modifier_3d]: sidecar module with related enum/flag types\n* [`ICopyTransformModifier3D`][crate::classes::ICopyTransformModifier3D]: virtual methods\n\n\nSee also [Godot docs for `CopyTransformModifier3D`](https://docs.godotengine.org/en/stable/classes/class_copytransformmodifier3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`CopyTransformModifier3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CopyTransformModifier3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`CopyTransformModifier3D`][crate::classes::CopyTransformModifier3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IBoneConstraint3D`][crate::classes::IBoneConstraint3D] > [`ISkeletonModifier3D`][crate::classes::ISkeletonModifier3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `CopyTransformModifier3D` methods](https://docs.godotengine.org/en/stable/classes/class_copytransformmodifier3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICopyTransformModifier3D: crate::obj::GodotClass < Base = CopyTransformModifier3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn process_modification_with_delta(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn process_modification(&mut self,) {
            unimplemented !()
        }
        fn skeleton_changed(&mut self, old_skeleton: Option < Gd < crate::classes::Skeleton3D > >, new_skeleton: Option < Gd < crate::classes::Skeleton3D > >,) {
            unimplemented !()
        }
        fn validate_bone_names(&mut self,) {
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
    impl CopyTransformModifier3D {
        pub fn set_copy_flags(&mut self, index: i32, copy_flags: crate::classes::copy_transform_modifier_3d::TransformFlag,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::copy_transform_modifier_3d::TransformFlag,);
            let args = (index, copy_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2635usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_copy_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_copy_flags(&self, index: i32,) -> crate::classes::copy_transform_modifier_3d::TransformFlag {
            type CallRet = crate::classes::copy_transform_modifier_3d::TransformFlag;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2636usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "get_copy_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_axis_flags(&mut self, index: i32, axis_flags: crate::classes::copy_transform_modifier_3d::AxisFlag,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::copy_transform_modifier_3d::AxisFlag,);
            let args = (index, axis_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2637usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_axis_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_axis_flags(&self, index: i32,) -> crate::classes::copy_transform_modifier_3d::AxisFlag {
            type CallRet = crate::classes::copy_transform_modifier_3d::AxisFlag;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2638usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "get_axis_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_invert_flags(&mut self, index: i32, axis_flags: crate::classes::copy_transform_modifier_3d::AxisFlag,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::copy_transform_modifier_3d::AxisFlag,);
            let args = (index, axis_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2639usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_invert_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_invert_flags(&self, index: i32,) -> crate::classes::copy_transform_modifier_3d::AxisFlag {
            type CallRet = crate::classes::copy_transform_modifier_3d::AxisFlag;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2640usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "get_invert_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_copy_position(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2641usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_copy_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_position_copying(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2642usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_position_copying", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_copy_rotation(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2643usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_copy_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_rotation_copying(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2644usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_rotation_copying", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_copy_scale(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2645usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_copy_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_scale_copying(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2646usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_scale_copying", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_axis_x_enabled(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2647usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_axis_x_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_axis_x_enabled(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2648usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_axis_x_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_axis_y_enabled(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2649usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_axis_y_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_axis_y_enabled(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2650usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_axis_y_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_axis_z_enabled(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2651usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_axis_z_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_axis_z_enabled(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2652usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_axis_z_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_axis_x_inverted(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2653usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_axis_x_inverted", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_axis_x_inverted(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2654usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_axis_x_inverted", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_axis_y_inverted(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2655usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_axis_y_inverted", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_axis_y_inverted(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2656usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_axis_y_inverted", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_axis_z_inverted(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2657usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_axis_z_inverted", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_axis_z_inverted(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2658usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_axis_z_inverted", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_relative(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2659usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_relative", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_relative(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2660usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_relative", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_additive(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2661usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "set_additive", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_additive(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2662usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CopyTransformModifier3D", "is_additive", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CopyTransformModifier3D {
        type Base = crate::classes::BoneConstraint3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CopyTransformModifier3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CopyTransformModifier3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::BoneConstraint3D > for CopyTransformModifier3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::SkeletonModifier3D > for CopyTransformModifier3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for CopyTransformModifier3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CopyTransformModifier3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CopyTransformModifier3D {
        
    }
    impl crate::obj::cap::GodotDefault for CopyTransformModifier3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CopyTransformModifier3D {
        type Target = crate::classes::BoneConstraint3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CopyTransformModifier3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CopyTransformModifier3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CopyTransformModifier3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CopyTransformModifier3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::BoneConstraint3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::SkeletonModifier3D > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct TransformFlag {
    ord: u64
}
impl TransformFlag {
    #[doc(alias = "TRANSFORM_FLAG_POSITION")]
    #[doc = "Godot enumerator name: `TRANSFORM_FLAG_POSITION`"]
    pub const POSITION: TransformFlag = TransformFlag {
        ord: 1u64
    };
    #[doc(alias = "TRANSFORM_FLAG_ROTATION")]
    #[doc = "Godot enumerator name: `TRANSFORM_FLAG_ROTATION`"]
    pub const ROTATION: TransformFlag = TransformFlag {
        ord: 2u64
    };
    #[doc(alias = "TRANSFORM_FLAG_SCALE")]
    #[doc = "Godot enumerator name: `TRANSFORM_FLAG_SCALE`"]
    pub const SCALE: TransformFlag = TransformFlag {
        ord: 4u64
    };
    #[doc(alias = "TRANSFORM_FLAG_ALL")]
    #[doc = "Godot enumerator name: `TRANSFORM_FLAG_ALL`"]
    pub const ALL: TransformFlag = TransformFlag {
        ord: 7u64
    };
    
}
impl std::fmt::Debug for TransformFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::POSITION => "POSITION", Self::ROTATION => "ROTATION", Self::SCALE => "SCALE", Self::ALL => "ALL", _ => {
                f.debug_struct("TransformFlag") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for TransformFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TransformFlag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("POSITION", "TRANSFORM_FLAG_POSITION", TransformFlag::POSITION), crate::meta::inspect::EnumConstant::new("ROTATION", "TRANSFORM_FLAG_ROTATION", TransformFlag::ROTATION), crate::meta::inspect::EnumConstant::new("SCALE", "TRANSFORM_FLAG_SCALE", TransformFlag::SCALE), crate::meta::inspect::EnumConstant::new("ALL", "TRANSFORM_FLAG_ALL", TransformFlag::ALL)]
        }
    }
}
impl std::ops::BitOr for TransformFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for TransformFlag {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for TransformFlag {
    type Via = u64;
    
}
impl crate::meta::ToGodot for TransformFlag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TransformFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct AxisFlag {
    ord: u64
}
impl AxisFlag {
    #[doc(alias = "AXIS_FLAG_X")]
    #[doc = "Godot enumerator name: `AXIS_FLAG_X`"]
    pub const X: AxisFlag = AxisFlag {
        ord: 1u64
    };
    #[doc(alias = "AXIS_FLAG_Y")]
    #[doc = "Godot enumerator name: `AXIS_FLAG_Y`"]
    pub const Y: AxisFlag = AxisFlag {
        ord: 2u64
    };
    #[doc(alias = "AXIS_FLAG_Z")]
    #[doc = "Godot enumerator name: `AXIS_FLAG_Z`"]
    pub const Z: AxisFlag = AxisFlag {
        ord: 4u64
    };
    #[doc(alias = "AXIS_FLAG_ALL")]
    #[doc = "Godot enumerator name: `AXIS_FLAG_ALL`"]
    pub const ALL: AxisFlag = AxisFlag {
        ord: 7u64
    };
    
}
impl std::fmt::Debug for AxisFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::X => "X", Self::Y => "Y", Self::Z => "Z", Self::ALL => "ALL", _ => {
                f.debug_struct("AxisFlag") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for AxisFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AxisFlag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("X", "AXIS_FLAG_X", AxisFlag::X), crate::meta::inspect::EnumConstant::new("Y", "AXIS_FLAG_Y", AxisFlag::Y), crate::meta::inspect::EnumConstant::new("Z", "AXIS_FLAG_Z", AxisFlag::Z), crate::meta::inspect::EnumConstant::new("ALL", "AXIS_FLAG_ALL", AxisFlag::ALL)]
        }
    }
}
impl std::ops::BitOr for AxisFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for AxisFlag {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for AxisFlag {
    type Via = u64;
    
}
impl crate::meta::ToGodot for AxisFlag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AxisFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CopyTransformModifier3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::skeleton_modifier_3d::SignalsOfSkeletonModifier3D;
    impl WithSignals for CopyTransformModifier3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfSkeletonModifier3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}