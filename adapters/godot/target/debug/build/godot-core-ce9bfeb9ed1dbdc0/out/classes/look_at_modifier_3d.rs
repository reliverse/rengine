#![doc = "Sidecar module for class [`LookAtModifier3D`][crate::classes::LookAtModifier3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `LookAtModifier3D` enums](https://docs.godotengine.org/en/stable/classes/class_lookatmodifier3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `LookAtModifier3D.`\n\nInherits [`SkeletonModifier3D`][crate::classes::SkeletonModifier3D].\n\nRelated symbols:\n\n* [`look_at_modifier_3d`][crate::classes::look_at_modifier_3d]: sidecar module with related enum/flag types\n* [`ILookAtModifier3D`][crate::classes::ILookAtModifier3D]: virtual methods\n\n\nSee also [Godot docs for `LookAtModifier3D`](https://docs.godotengine.org/en/stable/classes/class_lookatmodifier3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`LookAtModifier3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct LookAtModifier3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`LookAtModifier3D`][crate::classes::LookAtModifier3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`ISkeletonModifier3D`][crate::classes::ISkeletonModifier3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `LookAtModifier3D` methods](https://docs.godotengine.org/en/stable/classes/class_lookatmodifier3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILookAtModifier3D: crate::obj::GodotClass < Base = LookAtModifier3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl LookAtModifier3D {
        pub fn set_target_node(&mut self, target_node: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (target_node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_target_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_target_node(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_target_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bone_name(&mut self, bone_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bone_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bone(&mut self, bone: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (bone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bone(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_forward_axis(&mut self, forward_axis: crate::classes::skeleton_modifier_3d::BoneAxis,) {
            type CallRet = ();
            type CallParams = (crate::classes::skeleton_modifier_3d::BoneAxis,);
            let args = (forward_axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_forward_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_forward_axis(&self,) -> crate::classes::skeleton_modifier_3d::BoneAxis {
            type CallRet = crate::classes::skeleton_modifier_3d::BoneAxis;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_forward_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_primary_rotation_axis(&mut self, axis: Vector3Axis,) {
            type CallRet = ();
            type CallParams = (Vector3Axis,);
            let args = (axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_primary_rotation_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_primary_rotation_axis(&self,) -> Vector3Axis {
            type CallRet = Vector3Axis;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_primary_rotation_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_secondary_rotation(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_use_secondary_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_secondary_rotation(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "is_using_secondary_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_origin_safe_margin(&mut self, margin: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_origin_safe_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_origin_safe_margin(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_origin_safe_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_origin_from(&mut self, origin_from: crate::classes::look_at_modifier_3d::OriginFrom,) {
            type CallRet = ();
            type CallParams = (crate::classes::look_at_modifier_3d::OriginFrom,);
            let args = (origin_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_origin_from", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_origin_from(&self,) -> crate::classes::look_at_modifier_3d::OriginFrom {
            type CallRet = crate::classes::look_at_modifier_3d::OriginFrom;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_origin_from", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_origin_bone_name(&mut self, bone_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_origin_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_origin_bone_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_origin_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_origin_bone(&mut self, bone: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (bone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_origin_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_origin_bone(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_origin_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_origin_external_node(&mut self, external_node: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (external_node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_origin_external_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_origin_external_node(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_origin_external_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_origin_offset(&mut self, offset: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_origin_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_origin_offset(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_origin_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_duration(&mut self, duration: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_duration", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_duration(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_duration", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transition_type(&mut self, transition_type: crate::classes::tween::TransitionType,) {
            type CallRet = ();
            type CallParams = (crate::classes::tween::TransitionType,);
            let args = (transition_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_transition_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transition_type(&self,) -> crate::classes::tween::TransitionType {
            type CallRet = crate::classes::tween::TransitionType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_transition_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ease_type(&mut self, ease_type: crate::classes::tween::EaseType,) {
            type CallRet = ();
            type CallParams = (crate::classes::tween::EaseType,);
            let args = (ease_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_ease_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ease_type(&self,) -> crate::classes::tween::EaseType {
            type CallRet = crate::classes::tween::EaseType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_ease_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_angle_limitation(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_use_angle_limitation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_angle_limitation(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "is_using_angle_limitation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_symmetry_limitation(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_symmetry_limitation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_limitation_symmetry(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "is_limitation_symmetry", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_primary_limit_angle(&mut self, angle: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_primary_limit_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_primary_limit_angle(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_primary_limit_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_primary_damp_threshold(&mut self, power: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (power,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_primary_damp_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_primary_damp_threshold(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_primary_damp_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_primary_positive_limit_angle(&mut self, angle: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_primary_positive_limit_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_primary_positive_limit_angle(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_primary_positive_limit_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_primary_positive_damp_threshold(&mut self, power: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (power,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_primary_positive_damp_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_primary_positive_damp_threshold(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_primary_positive_damp_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_primary_negative_limit_angle(&mut self, angle: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_primary_negative_limit_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_primary_negative_limit_angle(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_primary_negative_limit_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_primary_negative_damp_threshold(&mut self, power: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (power,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_primary_negative_damp_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_primary_negative_damp_threshold(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_primary_negative_damp_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_secondary_limit_angle(&mut self, angle: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_secondary_limit_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_secondary_limit_angle(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_secondary_limit_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_secondary_damp_threshold(&mut self, power: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (power,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_secondary_damp_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_secondary_damp_threshold(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_secondary_damp_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_secondary_positive_limit_angle(&mut self, angle: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_secondary_positive_limit_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_secondary_positive_limit_angle(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_secondary_positive_limit_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_secondary_positive_damp_threshold(&mut self, power: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (power,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_secondary_positive_damp_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_secondary_positive_damp_threshold(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_secondary_positive_damp_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_secondary_negative_limit_angle(&mut self, angle: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_secondary_negative_limit_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_secondary_negative_limit_angle(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_secondary_negative_limit_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_secondary_negative_damp_threshold(&mut self, power: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (power,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "set_secondary_negative_damp_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_secondary_negative_damp_threshold(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_secondary_negative_damp_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_interpolation_remaining(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "get_interpolation_remaining", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_interpolating(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "is_interpolating", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_target_within_limitation(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LookAtModifier3D", "is_target_within_limitation", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for LookAtModifier3D {
        type Base = crate::classes::SkeletonModifier3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("LookAtModifier3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for LookAtModifier3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::SkeletonModifier3D > for LookAtModifier3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for LookAtModifier3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for LookAtModifier3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for LookAtModifier3D {
        
    }
    impl crate::obj::cap::GodotDefault for LookAtModifier3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for LookAtModifier3D {
        type Target = crate::classes::SkeletonModifier3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for LookAtModifier3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`LookAtModifier3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_LookAtModifier3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::LookAtModifier3D > for $Class {
                
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
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct OriginFrom {
    ord: i32
}
impl OriginFrom {
    #[doc(alias = "ORIGIN_FROM_SELF")]
    #[doc = "Godot enumerator name: `ORIGIN_FROM_SELF`"]
    pub const SELF: OriginFrom = OriginFrom {
        ord: 0i32
    };
    #[doc(alias = "ORIGIN_FROM_SPECIFIC_BONE")]
    #[doc = "Godot enumerator name: `ORIGIN_FROM_SPECIFIC_BONE`"]
    pub const SPECIFIC_BONE: OriginFrom = OriginFrom {
        ord: 1i32
    };
    #[doc(alias = "ORIGIN_FROM_EXTERNAL_NODE")]
    #[doc = "Godot enumerator name: `ORIGIN_FROM_EXTERNAL_NODE`"]
    pub const EXTERNAL_NODE: OriginFrom = OriginFrom {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for OriginFrom {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("OriginFrom") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for OriginFrom {
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
            Self::SELF => "SELF", Self::SPECIFIC_BONE => "SPECIFIC_BONE", Self::EXTERNAL_NODE => "EXTERNAL_NODE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[OriginFrom::SELF, OriginFrom::SPECIFIC_BONE, OriginFrom::EXTERNAL_NODE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < OriginFrom >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SELF", "ORIGIN_FROM_SELF", OriginFrom::SELF), crate::meta::inspect::EnumConstant::new("SPECIFIC_BONE", "ORIGIN_FROM_SPECIFIC_BONE", OriginFrom::SPECIFIC_BONE), crate::meta::inspect::EnumConstant::new("EXTERNAL_NODE", "ORIGIN_FROM_EXTERNAL_NODE", OriginFrom::EXTERNAL_NODE)]
        }
    }
}
impl crate::meta::GodotConvert for OriginFrom {
    type Via = i32;
    
}
impl crate::meta::ToGodot for OriginFrom {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for OriginFrom {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::LookAtModifier3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::skeleton_modifier_3d::SignalsOfSkeletonModifier3D;
    impl WithSignals for LookAtModifier3D {
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