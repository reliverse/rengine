#![doc = "Sidecar module for class [`SpringBoneSimulator3D`][crate::classes::SpringBoneSimulator3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SpringBoneSimulator3D` enums](https://docs.godotengine.org/en/stable/classes/class_springbonesimulator3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SpringBoneSimulator3D.`\n\nInherits [`SkeletonModifier3D`][crate::classes::SkeletonModifier3D].\n\nRelated symbols:\n\n* [`spring_bone_simulator_3d`][crate::classes::spring_bone_simulator_3d]: sidecar module with related enum/flag types\n* [`ISpringBoneSimulator3D`][crate::classes::ISpringBoneSimulator3D]: virtual methods\n\n\nSee also [Godot docs for `SpringBoneSimulator3D`](https://docs.godotengine.org/en/stable/classes/class_springbonesimulator3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`SpringBoneSimulator3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SpringBoneSimulator3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`SpringBoneSimulator3D`][crate::classes::SpringBoneSimulator3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`ISkeletonModifier3D`][crate::classes::ISkeletonModifier3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `SpringBoneSimulator3D` methods](https://docs.godotengine.org/en/stable/classes/class_springbonesimulator3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISpringBoneSimulator3D: crate::obj::GodotClass < Base = SpringBoneSimulator3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SpringBoneSimulator3D {
        pub fn set_root_bone_name(&mut self, index: i32, bone_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (index, bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8344usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_root_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_bone_name(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8345usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_root_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_root_bone(&mut self, index: i32, bone: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, bone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8346usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_root_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_bone(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8347usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_root_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_end_bone_name(&mut self, index: i32, bone_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (index, bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8348usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_end_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_end_bone_name(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8349usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_end_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_end_bone(&mut self, index: i32, bone: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, bone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8350usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_end_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_end_bone(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8351usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_end_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_extend_end_bone(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8352usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_extend_end_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_end_bone_extended(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8353usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "is_end_bone_extended", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_end_bone_direction(&mut self, index: i32, bone_direction: crate::classes::spring_bone_simulator_3d::BoneDirection,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::spring_bone_simulator_3d::BoneDirection,);
            let args = (index, bone_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8354usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_end_bone_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_end_bone_direction(&self, index: i32,) -> crate::classes::spring_bone_simulator_3d::BoneDirection {
            type CallRet = crate::classes::spring_bone_simulator_3d::BoneDirection;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8355usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_end_bone_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_end_bone_length(&mut self, index: i32, length: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (index, length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8356usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_end_bone_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_end_bone_length(&self, index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8357usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_end_bone_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_center_from(&mut self, index: i32, center_from: crate::classes::spring_bone_simulator_3d::CenterFrom,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::spring_bone_simulator_3d::CenterFrom,);
            let args = (index, center_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8358usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_center_from", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_from(&self, index: i32,) -> crate::classes::spring_bone_simulator_3d::CenterFrom {
            type CallRet = crate::classes::spring_bone_simulator_3d::CenterFrom;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_center_from", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_center_node(&mut self, index: i32, node_path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, NodePath >,);
            let args = (index, node_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_center_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_node(&self, index: i32,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8361usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_center_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_center_bone_name(&mut self, index: i32, bone_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (index, bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8362usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_center_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_bone_name(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8363usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_center_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_center_bone(&mut self, index: i32, bone: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, bone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8364usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_center_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_bone(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8365usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_center_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_radius(&mut self, index: i32, radius: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (index, radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8366usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_radius(&self, index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8367usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rotation_axis(&mut self, index: i32, axis: crate::classes::spring_bone_simulator_3d::RotationAxis,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::spring_bone_simulator_3d::RotationAxis,);
            let args = (index, axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8368usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_rotation_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rotation_axis(&self, index: i32,) -> crate::classes::spring_bone_simulator_3d::RotationAxis {
            type CallRet = crate::classes::spring_bone_simulator_3d::RotationAxis;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8369usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_rotation_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rotation_axis_vector(&mut self, index: i32, vector: Vector3,) {
            type CallRet = ();
            type CallParams = (i32, Vector3,);
            let args = (index, vector,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8370usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_rotation_axis_vector", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rotation_axis_vector(&self, index: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8371usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_rotation_axis_vector", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_radius_damping_curve(&mut self, index: i32, curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Curve > > >,);
            let args = (index, curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8372usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_radius_damping_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_radius_damping_curve(&self, index: i32,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8373usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_radius_damping_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stiffness(&mut self, index: i32, stiffness: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (index, stiffness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8374usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_stiffness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stiffness(&self, index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8375usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_stiffness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stiffness_damping_curve(&mut self, index: i32, curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Curve > > >,);
            let args = (index, curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8376usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_stiffness_damping_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stiffness_damping_curve(&self, index: i32,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8377usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_stiffness_damping_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag(&mut self, index: i32, drag: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (index, drag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8378usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_drag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_drag(&self, index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8379usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_drag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_damping_curve(&mut self, index: i32, curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Curve > > >,);
            let args = (index, curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8380usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_drag_damping_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_drag_damping_curve(&self, index: i32,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8381usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_drag_damping_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity(&mut self, index: i32, gravity: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (index, gravity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8382usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity(&self, index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8383usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity_damping_curve(&mut self, index: i32, curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Curve > > >,);
            let args = (index, curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8384usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_gravity_damping_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity_damping_curve(&self, index: i32,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8385usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_gravity_damping_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity_direction(&mut self, index: i32, gravity_direction: Vector3,) {
            type CallRet = ();
            type CallParams = (i32, Vector3,);
            let args = (index, gravity_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8386usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_gravity_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity_direction(&self, index: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8387usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_gravity_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_setting_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8388usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_setting_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_setting_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8389usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_setting_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_settings(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8390usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "clear_settings", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_individual_config(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8391usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_individual_config", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_config_individual(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8392usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "is_config_individual", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_bone_name(&self, index: i32, joint: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32, i32,);
            let args = (index, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8393usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_joint_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_bone(&self, index: i32, joint: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (index, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8394usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_joint_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_joint_rotation_axis(&mut self, index: i32, joint: i32, axis: crate::classes::spring_bone_simulator_3d::RotationAxis,) {
            type CallRet = ();
            type CallParams = (i32, i32, crate::classes::spring_bone_simulator_3d::RotationAxis,);
            let args = (index, joint, axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8395usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_joint_rotation_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_rotation_axis(&self, index: i32, joint: i32,) -> crate::classes::spring_bone_simulator_3d::RotationAxis {
            type CallRet = crate::classes::spring_bone_simulator_3d::RotationAxis;
            type CallParams = (i32, i32,);
            let args = (index, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8396usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_joint_rotation_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_joint_rotation_axis_vector(&mut self, index: i32, joint: i32, vector: Vector3,) {
            type CallRet = ();
            type CallParams = (i32, i32, Vector3,);
            let args = (index, joint, vector,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8397usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_joint_rotation_axis_vector", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_rotation_axis_vector(&self, index: i32, joint: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32, i32,);
            let args = (index, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8398usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_joint_rotation_axis_vector", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_joint_radius(&mut self, index: i32, joint: i32, radius: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (index, joint, radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8399usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_joint_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_radius(&self, index: i32, joint: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (index, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8400usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_joint_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_joint_stiffness(&mut self, index: i32, joint: i32, stiffness: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (index, joint, stiffness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8401usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_joint_stiffness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_stiffness(&self, index: i32, joint: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (index, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8402usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_joint_stiffness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_joint_drag(&mut self, index: i32, joint: i32, drag: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (index, joint, drag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8403usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_joint_drag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_drag(&self, index: i32, joint: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (index, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_joint_drag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_joint_gravity(&mut self, index: i32, joint: i32, gravity: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (index, joint, gravity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_joint_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_gravity(&self, index: i32, joint: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (index, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_joint_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_joint_gravity_direction(&mut self, index: i32, joint: i32, gravity_direction: Vector3,) {
            type CallRet = ();
            type CallParams = (i32, i32, Vector3,);
            let args = (index, joint, gravity_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_joint_gravity_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_gravity_direction(&self, index: i32, joint: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32, i32,);
            let args = (index, joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_joint_gravity_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_count(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_joint_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enable_all_child_collisions(&mut self, index: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (index, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_enable_all_child_collisions", Some(self.__validated_obj()), args,)
            }
        }
        pub fn are_all_child_collisions_enabled(&self, index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "are_all_child_collisions_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_exclude_collision_path(&mut self, index: i32, collision: i32, node_path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, NodePath >,);
            let args = (index, collision, node_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_exclude_collision_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_exclude_collision_path(&self, index: i32, collision: i32,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = (i32, i32,);
            let args = (index, collision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_exclude_collision_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_exclude_collision_count(&mut self, index: i32, count: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_exclude_collision_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_exclude_collision_count(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_exclude_collision_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_exclude_collisions(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "clear_exclude_collisions", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_path(&mut self, index: i32, collision: i32, node_path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, NodePath >,);
            let args = (index, collision, node_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_collision_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_path(&self, index: i32, collision: i32,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = (i32, i32,);
            let args = (index, collision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_collision_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_count(&mut self, index: i32, count: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_collision_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_count(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8420usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_collision_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_collisions(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8421usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "clear_collisions", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_external_force(&mut self, force: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8422usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "set_external_force", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_external_force(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8423usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "get_external_force", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reset(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8424usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpringBoneSimulator3D", "reset", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for SpringBoneSimulator3D {
        type Base = crate::classes::SkeletonModifier3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("SpringBoneSimulator3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SpringBoneSimulator3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::SkeletonModifier3D > for SpringBoneSimulator3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for SpringBoneSimulator3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for SpringBoneSimulator3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SpringBoneSimulator3D {
        
    }
    impl crate::obj::cap::GodotDefault for SpringBoneSimulator3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SpringBoneSimulator3D {
        type Target = crate::classes::SkeletonModifier3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SpringBoneSimulator3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SpringBoneSimulator3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_SpringBoneSimulator3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SpringBoneSimulator3D > for $Class {
                
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
pub struct BoneDirection {
    ord: i32
}
impl BoneDirection {
    #[doc(alias = "BONE_DIRECTION_PLUS_X")]
    #[doc = "Godot enumerator name: `BONE_DIRECTION_PLUS_X`"]
    pub const PLUS_X: BoneDirection = BoneDirection {
        ord: 0i32
    };
    #[doc(alias = "BONE_DIRECTION_MINUS_X")]
    #[doc = "Godot enumerator name: `BONE_DIRECTION_MINUS_X`"]
    pub const MINUS_X: BoneDirection = BoneDirection {
        ord: 1i32
    };
    #[doc(alias = "BONE_DIRECTION_PLUS_Y")]
    #[doc = "Godot enumerator name: `BONE_DIRECTION_PLUS_Y`"]
    pub const PLUS_Y: BoneDirection = BoneDirection {
        ord: 2i32
    };
    #[doc(alias = "BONE_DIRECTION_MINUS_Y")]
    #[doc = "Godot enumerator name: `BONE_DIRECTION_MINUS_Y`"]
    pub const MINUS_Y: BoneDirection = BoneDirection {
        ord: 3i32
    };
    #[doc(alias = "BONE_DIRECTION_PLUS_Z")]
    #[doc = "Godot enumerator name: `BONE_DIRECTION_PLUS_Z`"]
    pub const PLUS_Z: BoneDirection = BoneDirection {
        ord: 4i32
    };
    #[doc(alias = "BONE_DIRECTION_MINUS_Z")]
    #[doc = "Godot enumerator name: `BONE_DIRECTION_MINUS_Z`"]
    pub const MINUS_Z: BoneDirection = BoneDirection {
        ord: 5i32
    };
    #[doc(alias = "BONE_DIRECTION_FROM_PARENT")]
    #[doc = "Godot enumerator name: `BONE_DIRECTION_FROM_PARENT`"]
    pub const FROM_PARENT: BoneDirection = BoneDirection {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for BoneDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BoneDirection") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BoneDirection {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::PLUS_X => "PLUS_X", Self::MINUS_X => "MINUS_X", Self::PLUS_Y => "PLUS_Y", Self::MINUS_Y => "MINUS_Y", Self::PLUS_Z => "PLUS_Z", Self::MINUS_Z => "MINUS_Z", Self::FROM_PARENT => "FROM_PARENT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BoneDirection::PLUS_X, BoneDirection::MINUS_X, BoneDirection::PLUS_Y, BoneDirection::MINUS_Y, BoneDirection::PLUS_Z, BoneDirection::MINUS_Z, BoneDirection::FROM_PARENT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BoneDirection >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PLUS_X", "BONE_DIRECTION_PLUS_X", BoneDirection::PLUS_X), crate::meta::inspect::EnumConstant::new("MINUS_X", "BONE_DIRECTION_MINUS_X", BoneDirection::MINUS_X), crate::meta::inspect::EnumConstant::new("PLUS_Y", "BONE_DIRECTION_PLUS_Y", BoneDirection::PLUS_Y), crate::meta::inspect::EnumConstant::new("MINUS_Y", "BONE_DIRECTION_MINUS_Y", BoneDirection::MINUS_Y), crate::meta::inspect::EnumConstant::new("PLUS_Z", "BONE_DIRECTION_PLUS_Z", BoneDirection::PLUS_Z), crate::meta::inspect::EnumConstant::new("MINUS_Z", "BONE_DIRECTION_MINUS_Z", BoneDirection::MINUS_Z), crate::meta::inspect::EnumConstant::new("FROM_PARENT", "BONE_DIRECTION_FROM_PARENT", BoneDirection::FROM_PARENT)]
        }
    }
}
impl crate::meta::GodotConvert for BoneDirection {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BoneDirection {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BoneDirection {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CenterFrom {
    ord: i32
}
impl CenterFrom {
    #[doc(alias = "CENTER_FROM_WORLD_ORIGIN")]
    #[doc = "Godot enumerator name: `CENTER_FROM_WORLD_ORIGIN`"]
    pub const WORLD_ORIGIN: CenterFrom = CenterFrom {
        ord: 0i32
    };
    #[doc(alias = "CENTER_FROM_NODE")]
    #[doc = "Godot enumerator name: `CENTER_FROM_NODE`"]
    pub const NODE: CenterFrom = CenterFrom {
        ord: 1i32
    };
    #[doc(alias = "CENTER_FROM_BONE")]
    #[doc = "Godot enumerator name: `CENTER_FROM_BONE`"]
    pub const BONE: CenterFrom = CenterFrom {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for CenterFrom {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CenterFrom") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CenterFrom {
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
            Self::WORLD_ORIGIN => "WORLD_ORIGIN", Self::NODE => "NODE", Self::BONE => "BONE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CenterFrom::WORLD_ORIGIN, CenterFrom::NODE, CenterFrom::BONE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CenterFrom >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("WORLD_ORIGIN", "CENTER_FROM_WORLD_ORIGIN", CenterFrom::WORLD_ORIGIN), crate::meta::inspect::EnumConstant::new("NODE", "CENTER_FROM_NODE", CenterFrom::NODE), crate::meta::inspect::EnumConstant::new("BONE", "CENTER_FROM_BONE", CenterFrom::BONE)]
        }
    }
}
impl crate::meta::GodotConvert for CenterFrom {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CenterFrom {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CenterFrom {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RotationAxis {
    ord: i32
}
impl RotationAxis {
    #[doc(alias = "ROTATION_AXIS_X")]
    #[doc = "Godot enumerator name: `ROTATION_AXIS_X`"]
    pub const X: RotationAxis = RotationAxis {
        ord: 0i32
    };
    #[doc(alias = "ROTATION_AXIS_Y")]
    #[doc = "Godot enumerator name: `ROTATION_AXIS_Y`"]
    pub const Y: RotationAxis = RotationAxis {
        ord: 1i32
    };
    #[doc(alias = "ROTATION_AXIS_Z")]
    #[doc = "Godot enumerator name: `ROTATION_AXIS_Z`"]
    pub const Z: RotationAxis = RotationAxis {
        ord: 2i32
    };
    #[doc(alias = "ROTATION_AXIS_ALL")]
    #[doc = "Godot enumerator name: `ROTATION_AXIS_ALL`"]
    pub const ALL: RotationAxis = RotationAxis {
        ord: 3i32
    };
    #[doc(alias = "ROTATION_AXIS_CUSTOM")]
    #[doc = "Godot enumerator name: `ROTATION_AXIS_CUSTOM`"]
    pub const CUSTOM: RotationAxis = RotationAxis {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for RotationAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RotationAxis") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RotationAxis {
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
            Self::X => "X", Self::Y => "Y", Self::Z => "Z", Self::ALL => "ALL", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[RotationAxis::X, RotationAxis::Y, RotationAxis::Z, RotationAxis::ALL, RotationAxis::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < RotationAxis >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("X", "ROTATION_AXIS_X", RotationAxis::X), crate::meta::inspect::EnumConstant::new("Y", "ROTATION_AXIS_Y", RotationAxis::Y), crate::meta::inspect::EnumConstant::new("Z", "ROTATION_AXIS_Z", RotationAxis::Z), crate::meta::inspect::EnumConstant::new("ALL", "ROTATION_AXIS_ALL", RotationAxis::ALL), crate::meta::inspect::EnumConstant::new("CUSTOM", "ROTATION_AXIS_CUSTOM", RotationAxis::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for RotationAxis {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RotationAxis {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RotationAxis {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::SpringBoneSimulator3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::skeleton_modifier_3d::SignalsOfSkeletonModifier3D;
    impl WithSignals for SpringBoneSimulator3D {
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