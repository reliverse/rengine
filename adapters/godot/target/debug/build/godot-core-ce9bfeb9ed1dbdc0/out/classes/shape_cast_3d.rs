#![doc = "Sidecar module for class [`ShapeCast3D`][crate::classes::ShapeCast3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ShapeCast3D` enums](https://docs.godotengine.org/en/stable/classes/class_shapecast3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ShapeCast3D.`\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`IShapeCast3D`][crate::classes::IShapeCast3D]: virtual methods\n\n\nSee also [Godot docs for `ShapeCast3D`](https://docs.godotengine.org/en/stable/classes/class_shapecast3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`ShapeCast3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ShapeCast3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ShapeCast3D`][crate::classes::ShapeCast3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `ShapeCast3D` methods](https://docs.godotengine.org/en/stable/classes/class_shapecast3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IShapeCast3D: crate::obj::GodotClass < Base = ShapeCast3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ShapeCast3D {
        pub fn resource_changed(&mut self, resource: impl AsArg < Option < Gd < crate::classes::Resource >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Resource > > >,);
            let args = (resource.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8027usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "resource_changed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8028usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8029usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "is_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shape(&mut self, shape: impl AsArg < Option < Gd < crate::classes::Shape3D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Shape3D > > >,);
            let args = (shape.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8030usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shape(&self,) -> Option < Gd < crate::classes::Shape3D > > {
            type CallRet = Option < Gd < crate::classes::Shape3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8031usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_target_position(&mut self, local_point: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (local_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8032usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_target_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_target_position(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8033usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_target_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_margin(&mut self, margin: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8034usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_margin(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8035usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_results(&mut self, max_results: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_results,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8036usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_max_results", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_results(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8037usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_max_results", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_colliding(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8038usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "is_colliding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8039usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collision_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn force_shapecast_update(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8040usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "force_shapecast_update", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collider(&self, index: i32,) -> Option < Gd < crate::classes::Object > > {
            type CallRet = Option < Gd < crate::classes::Object > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8041usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collider", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collider_rid(&self, index: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8042usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collider_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collider_shape(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8043usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collider_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_point(&self, index: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8044usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collision_point", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_normal(&self, index: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8045usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collision_normal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_closest_collision_safe_fraction(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8046usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_closest_collision_safe_fraction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_closest_collision_unsafe_fraction(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8047usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_closest_collision_unsafe_fraction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_exception_rid(&mut self, rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8048usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "add_exception_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_exception(&mut self, node: impl AsArg < Option < Gd < crate::classes::CollisionObject3D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::CollisionObject3D > > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8049usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "add_exception", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_exception_rid(&mut self, rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8050usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "remove_exception_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_exception(&mut self, node: impl AsArg < Option < Gd < crate::classes::CollisionObject3D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::CollisionObject3D > > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8051usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "remove_exception", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_exceptions(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8052usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "clear_exceptions", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_mask(&mut self, mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8053usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8054usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_mask_value(&mut self, layer_number: i32, value: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8055usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_collision_mask_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_mask_value(&self, layer_number: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8056usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collision_mask_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_exclude_parent_body(&mut self, mask: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8057usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_exclude_parent_body", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_exclude_parent_body(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8058usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_exclude_parent_body", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collide_with_areas(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8059usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_collide_with_areas", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_collide_with_areas_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8060usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "is_collide_with_areas_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collide_with_bodies(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8061usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_collide_with_bodies", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_collide_with_bodies_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8062usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "is_collide_with_bodies_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_result(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8063usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_collision_result", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_debug_shape_custom_color(&mut self, debug_shape_custom_color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (debug_shape_custom_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8064usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "set_debug_shape_custom_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_debug_shape_custom_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8065usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ShapeCast3D", "get_debug_shape_custom_color", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ShapeCast3D {
        type Base = crate::classes::Node3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ShapeCast3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ShapeCast3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for ShapeCast3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for ShapeCast3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ShapeCast3D {
        
    }
    impl crate::obj::cap::GodotDefault for ShapeCast3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ShapeCast3D {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ShapeCast3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ShapeCast3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ShapeCast3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ShapeCast3D > for $Class {
                
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
    use super::re_export::ShapeCast3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for ShapeCast3D {
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