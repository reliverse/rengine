#![doc = "Sidecar module for class [`PhysicalBone3D`][crate::classes::PhysicalBone3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicalBone3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicalbone3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicalBone3D.`\n\nInherits [`PhysicsBody3D`][crate::classes::PhysicsBody3D].\n\nRelated symbols:\n\n* [`physical_bone_3d`][crate::classes::physical_bone_3d]: sidecar module with related enum/flag types\n* [`IPhysicalBone3D`][crate::classes::IPhysicalBone3D]: virtual methods\n\n\nSee also [Godot docs for `PhysicalBone3D`](https://docs.godotengine.org/en/stable/classes/class_physicalbone3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`PhysicalBone3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicalBone3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`PhysicalBone3D`][crate::classes::PhysicalBone3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IPhysicsBody3D`~~ > ~~`ICollisionObject3D`~~ > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `PhysicalBone3D` methods](https://docs.godotengine.org/en/stable/classes/class_physicalbone3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicalBone3D: crate::obj::GodotClass < Base = PhysicalBone3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn integrate_forces(&mut self, state: Option < Gd < crate::classes::PhysicsDirectBodyState3D > >,) {
            unimplemented !()
        }
        fn input_event(&mut self, camera: Option < Gd < crate::classes::Camera3D > >, event: Option < Gd < crate::classes::InputEvent > >, event_position: Vector3, normal: Vector3, shape_idx: i32,) {
            unimplemented !()
        }
        fn mouse_enter(&mut self,) {
            unimplemented !()
        }
        fn mouse_exit(&mut self,) {
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
    impl PhysicalBone3D {
        pub fn apply_central_impulse(&mut self, impulse: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "apply_central_impulse", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn apply_impulse_full(&mut self, impulse: Vector3, position: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3, Vector3,);
            let args = (impulse, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6420usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "apply_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::apply_impulse_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn apply_impulse(&mut self, impulse: Vector3,) {
            self.apply_impulse_ex(impulse,) . done()
        }
        #[inline]
        pub fn apply_impulse_ex < 'a > (&'a mut self, impulse: Vector3,) -> ExApplyImpulse < 'a > {
            ExApplyImpulse::new(self, impulse,)
        }
        pub fn set_joint_type(&mut self, joint_type: crate::classes::physical_bone_3d::JointType,) {
            type CallRet = ();
            type CallParams = (crate::classes::physical_bone_3d::JointType,);
            let args = (joint_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6421usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_joint_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_type(&self,) -> crate::classes::physical_bone_3d::JointType {
            type CallRet = crate::classes::physical_bone_3d::JointType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6422usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_joint_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_joint_offset(&mut self, offset: Transform3D,) {
            type CallRet = ();
            type CallParams = (Transform3D,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6423usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_joint_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_offset(&self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6424usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_joint_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_joint_rotation(&mut self, euler: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (euler,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6425usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_joint_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_rotation(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6426usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_joint_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_body_offset(&mut self, offset: Transform3D,) {
            type CallRet = ();
            type CallParams = (Transform3D,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6427usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_body_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_body_offset(&self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6428usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_body_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_simulate_physics(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6429usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_simulate_physics", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_simulating_physics(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6430usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "is_simulating_physics", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bone_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6431usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_bone_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mass(&mut self, mass: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (mass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6432usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mass(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6433usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_friction(&mut self, friction: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (friction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6434usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_friction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_friction(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6435usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_friction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bounce(&mut self, bounce: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (bounce,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6436usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_bounce", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bounce(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6437usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_bounce", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity_scale(&mut self, gravity_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (gravity_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6438usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_gravity_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6439usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_gravity_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_damp_mode(&mut self, linear_damp_mode: crate::classes::physical_bone_3d::DampMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::physical_bone_3d::DampMode,);
            let args = (linear_damp_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6440usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_linear_damp_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_damp_mode(&self,) -> crate::classes::physical_bone_3d::DampMode {
            type CallRet = crate::classes::physical_bone_3d::DampMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6441usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_linear_damp_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_damp_mode(&mut self, angular_damp_mode: crate::classes::physical_bone_3d::DampMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::physical_bone_3d::DampMode,);
            let args = (angular_damp_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6442usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_angular_damp_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_damp_mode(&self,) -> crate::classes::physical_bone_3d::DampMode {
            type CallRet = crate::classes::physical_bone_3d::DampMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6443usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_angular_damp_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_damp(&mut self, linear_damp: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (linear_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6444usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_linear_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6445usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_linear_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_damp(&mut self, angular_damp: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angular_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6446usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_angular_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6447usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_angular_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_velocity(&mut self, linear_velocity: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (linear_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6448usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_velocity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6449usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_velocity(&mut self, angular_velocity: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (angular_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6450usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_velocity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6451usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "get_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_custom_integrator(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6452usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_use_custom_integrator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_custom_integrator(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6453usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "is_using_custom_integrator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_can_sleep(&mut self, able_to_sleep: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (able_to_sleep,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6454usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "set_can_sleep", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_able_to_sleep(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6455usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone3D", "is_able_to_sleep", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PhysicalBone3D {
        type Base = crate::classes::PhysicsBody3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PhysicalBone3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicalBone3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PhysicsBody3D > for PhysicalBone3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject3D > for PhysicalBone3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for PhysicalBone3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for PhysicalBone3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicalBone3D {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicalBone3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicalBone3D {
        type Target = crate::classes::PhysicsBody3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicalBone3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PhysicalBone3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PhysicalBone3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicalBone3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsBody3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CollisionObject3D > for $Class {
                
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
#[doc = "Default-param extender for [`PhysicalBone3D::apply_impulse_ex`][super::PhysicalBone3D::apply_impulse_ex]."]
#[must_use]
pub struct ExApplyImpulse < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicalBone3D, impulse: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicalBone3D, impulse: Vector3,) -> Self {
        let position = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, impulse: impulse, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector3) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, impulse, position,
        }
        = self;
        re_export::PhysicalBone3D::apply_impulse_full(surround_object, impulse, position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DampMode {
    ord: i32
}
impl DampMode {
    #[doc(alias = "DAMP_MODE_COMBINE")]
    #[doc = "Godot enumerator name: `DAMP_MODE_COMBINE`"]
    pub const COMBINE: DampMode = DampMode {
        ord: 0i32
    };
    #[doc(alias = "DAMP_MODE_REPLACE")]
    #[doc = "Godot enumerator name: `DAMP_MODE_REPLACE`"]
    pub const REPLACE: DampMode = DampMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for DampMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DampMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DampMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::COMBINE => "COMBINE", Self::REPLACE => "REPLACE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DampMode::COMBINE, DampMode::REPLACE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DampMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("COMBINE", "DAMP_MODE_COMBINE", DampMode::COMBINE), crate::meta::inspect::EnumConstant::new("REPLACE", "DAMP_MODE_REPLACE", DampMode::REPLACE)]
        }
    }
}
impl crate::meta::GodotConvert for DampMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DampMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DampMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct JointType {
    ord: i32
}
impl JointType {
    #[doc(alias = "JOINT_TYPE_NONE")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_NONE`"]
    pub const NONE: JointType = JointType {
        ord: 0i32
    };
    #[doc(alias = "JOINT_TYPE_PIN")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_PIN`"]
    pub const PIN: JointType = JointType {
        ord: 1i32
    };
    #[doc(alias = "JOINT_TYPE_CONE")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_CONE`"]
    pub const CONE: JointType = JointType {
        ord: 2i32
    };
    #[doc(alias = "JOINT_TYPE_HINGE")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_HINGE`"]
    pub const HINGE: JointType = JointType {
        ord: 3i32
    };
    #[doc(alias = "JOINT_TYPE_SLIDER")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_SLIDER`"]
    pub const SLIDER: JointType = JointType {
        ord: 4i32
    };
    #[doc(alias = "JOINT_TYPE_6DOF")]
    #[doc = "Godot enumerator name: `JOINT_TYPE_6DOF`"]
    pub const TYPE_6DOF: JointType = JointType {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for JointType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("JointType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for JointType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::NONE => "NONE", Self::PIN => "PIN", Self::CONE => "CONE", Self::HINGE => "HINGE", Self::SLIDER => "SLIDER", Self::TYPE_6DOF => "TYPE_6DOF", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[JointType::NONE, JointType::PIN, JointType::CONE, JointType::HINGE, JointType::SLIDER, JointType::TYPE_6DOF]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < JointType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "JOINT_TYPE_NONE", JointType::NONE), crate::meta::inspect::EnumConstant::new("PIN", "JOINT_TYPE_PIN", JointType::PIN), crate::meta::inspect::EnumConstant::new("CONE", "JOINT_TYPE_CONE", JointType::CONE), crate::meta::inspect::EnumConstant::new("HINGE", "JOINT_TYPE_HINGE", JointType::HINGE), crate::meta::inspect::EnumConstant::new("SLIDER", "JOINT_TYPE_SLIDER", JointType::SLIDER), crate::meta::inspect::EnumConstant::new("TYPE_6DOF", "JOINT_TYPE_6DOF", JointType::TYPE_6DOF)]
        }
    }
}
impl crate::meta::GodotConvert for JointType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for JointType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for JointType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PhysicalBone3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::collision_object_3d::SignalsOfCollisionObject3D;
    impl WithSignals for PhysicalBone3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCollisionObject3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}