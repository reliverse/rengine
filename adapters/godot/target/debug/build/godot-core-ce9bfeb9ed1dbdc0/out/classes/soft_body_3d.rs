#![doc = "Sidecar module for class [`SoftBody3D`][crate::classes::SoftBody3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SoftBody3D` enums](https://docs.godotengine.org/en/stable/classes/class_softbody3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SoftBody3D.`\n\nInherits [`MeshInstance3D`][crate::classes::MeshInstance3D].\n\nRelated symbols:\n\n* [`soft_body_3d`][crate::classes::soft_body_3d]: sidecar module with related enum/flag types\n* [`ISoftBody3D`][crate::classes::ISoftBody3D]: virtual methods\n\n\nSee also [Godot docs for `SoftBody3D`](https://docs.godotengine.org/en/stable/classes/class_softbody3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`SoftBody3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SoftBody3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`SoftBody3D`][crate::classes::SoftBody3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IMeshInstance3D`][crate::classes::IMeshInstance3D] > [`IGeometryInstance3D`][crate::classes::IGeometryInstance3D] > [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `SoftBody3D` methods](https://docs.godotengine.org/en/stable/classes/class_softbody3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISoftBody3D: crate::obj::GodotClass < Base = SoftBody3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_aabb(&self,) -> Aabb {
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
    impl SoftBody3D {
        pub fn get_physics_rid(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_physics_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_mask(&mut self, collision_mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (collision_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_layer(&mut self, collision_layer: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (collision_layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_layer(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_mask_value(&mut self, layer_number: i32, value: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_collision_mask_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_mask_value(&self, layer_number: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_collision_mask_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_layer_value(&mut self, layer_number: i32, value: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_collision_layer_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_layer_value(&self, layer_number: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_collision_layer_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_parent_collision_ignore(&mut self, parent_collision_ignore: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (parent_collision_ignore.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_parent_collision_ignore", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_parent_collision_ignore(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_parent_collision_ignore", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_disable_mode(&mut self, mode: crate::classes::soft_body_3d::DisableMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::soft_body_3d::DisableMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_disable_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_disable_mode(&self,) -> crate::classes::soft_body_3d::DisableMode {
            type CallRet = crate::classes::soft_body_3d::DisableMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_disable_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_exceptions(&mut self,) -> Array < Gd < crate::classes::PhysicsBody3D > > {
            type CallRet = Array < Gd < crate::classes::PhysicsBody3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_collision_exceptions", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_collision_exception_with(&mut self, body: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (body.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "add_collision_exception_with", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_collision_exception_with(&mut self, body: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (body.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "remove_collision_exception_with", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_simulation_precision(&mut self, simulation_precision: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (simulation_precision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_simulation_precision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_simulation_precision(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_simulation_precision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_total_mass(&mut self, mass: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (mass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_total_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_total_mass(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_total_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_stiffness(&mut self, linear_stiffness: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (linear_stiffness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_linear_stiffness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_stiffness(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_linear_stiffness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shrinking_factor(&mut self, shrinking_factor: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (shrinking_factor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_shrinking_factor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shrinking_factor(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_shrinking_factor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pressure_coefficient(&mut self, pressure_coefficient: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (pressure_coefficient,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_pressure_coefficient", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pressure_coefficient(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_pressure_coefficient", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_damping_coefficient(&mut self, damping_coefficient: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (damping_coefficient,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_damping_coefficient", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_damping_coefficient(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_damping_coefficient", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_coefficient(&mut self, drag_coefficient: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (drag_coefficient,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_drag_coefficient", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_drag_coefficient(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_drag_coefficient", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_point_transform(&mut self, point_index: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (point_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "get_point_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn apply_impulse(&mut self, point_index: i32, impulse: Vector3,) {
            type CallRet = ();
            type CallParams = (i32, Vector3,);
            let args = (point_index, impulse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "apply_impulse", Some(self.__validated_obj()), args,)
            }
        }
        pub fn apply_force(&mut self, point_index: i32, force: Vector3,) {
            type CallRet = ();
            type CallParams = (i32, Vector3,);
            let args = (point_index, force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "apply_force", Some(self.__validated_obj()), args,)
            }
        }
        pub fn apply_central_impulse(&mut self, impulse: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "apply_central_impulse", Some(self.__validated_obj()), args,)
            }
        }
        pub fn apply_central_force(&mut self, force: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "apply_central_force", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn set_point_pinned_full(&mut self, point_index: i32, pinned: bool, attachment_path: CowArg < NodePath >, insert_at: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, bool, CowArg < 'a0, NodePath >, i32,);
            let args = (point_index, pinned, attachment_path, insert_at,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_point_pinned", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_point_pinned_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_point_pinned(&mut self, point_index: i32, pinned: bool,) {
            self.set_point_pinned_ex(point_index, pinned,) . done()
        }
        #[inline]
        pub fn set_point_pinned_ex < 'a > (&'a mut self, point_index: i32, pinned: bool,) -> ExSetPointPinned < 'a > {
            ExSetPointPinned::new(self, point_index, pinned,)
        }
        pub fn is_point_pinned(&self, point_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (point_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "is_point_pinned", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ray_pickable(&mut self, ray_pickable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (ray_pickable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "set_ray_pickable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_ray_pickable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SoftBody3D", "is_ray_pickable", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for SoftBody3D {
        type Base = crate::classes::MeshInstance3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("SoftBody3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SoftBody3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::MeshInstance3D > for SoftBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for SoftBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for SoftBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for SoftBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for SoftBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SoftBody3D {
        
    }
    impl crate::obj::cap::GodotDefault for SoftBody3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SoftBody3D {
        type Target = crate::classes::MeshInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SoftBody3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SoftBody3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_SoftBody3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SoftBody3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::MeshInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::GeometryInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualInstance3D > for $Class {
                
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
#[doc = "Default-param extender for [`SoftBody3D::set_point_pinned_ex`][super::SoftBody3D::set_point_pinned_ex]."]
#[must_use]
pub struct ExSetPointPinned < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SoftBody3D, point_index: i32, pinned: bool, attachment_path: CowArg < 'a, NodePath >, insert_at: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetPointPinned < 'a > {
    fn new(surround_object: &'a mut re_export::SoftBody3D, point_index: i32, pinned: bool,) -> Self {
        let attachment_path = NodePath::from("");
        let insert_at = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, point_index: point_index, pinned: pinned, attachment_path: CowArg::Owned(attachment_path), insert_at: insert_at,
        }
    }
    #[inline]
    pub fn attachment_path(self, attachment_path: impl AsArg < NodePath > + 'a) -> Self {
        Self {
            attachment_path: attachment_path.into_arg(), .. self
        }
    }
    #[inline]
    pub fn insert_at(self, insert_at: i32) -> Self {
        Self {
            insert_at: insert_at, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, point_index, pinned, attachment_path, insert_at,
        }
        = self;
        re_export::SoftBody3D::set_point_pinned_full(surround_object, point_index, pinned, attachment_path, insert_at,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DisableMode {
    ord: i32
}
impl DisableMode {
    #[doc(alias = "DISABLE_MODE_REMOVE")]
    #[doc = "Godot enumerator name: `DISABLE_MODE_REMOVE`"]
    pub const REMOVE: DisableMode = DisableMode {
        ord: 0i32
    };
    #[doc(alias = "DISABLE_MODE_KEEP_ACTIVE")]
    #[doc = "Godot enumerator name: `DISABLE_MODE_KEEP_ACTIVE`"]
    pub const KEEP_ACTIVE: DisableMode = DisableMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for DisableMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DisableMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DisableMode {
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
            Self::REMOVE => "REMOVE", Self::KEEP_ACTIVE => "KEEP_ACTIVE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DisableMode::REMOVE, DisableMode::KEEP_ACTIVE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DisableMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("REMOVE", "DISABLE_MODE_REMOVE", DisableMode::REMOVE), crate::meta::inspect::EnumConstant::new("KEEP_ACTIVE", "DISABLE_MODE_KEEP_ACTIVE", DisableMode::KEEP_ACTIVE)]
        }
    }
}
impl crate::meta::GodotConvert for DisableMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DisableMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DisableMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::SoftBody3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for SoftBody3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}