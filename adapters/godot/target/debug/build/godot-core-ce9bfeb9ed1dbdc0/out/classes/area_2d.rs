#![doc = "Sidecar module for class [`Area2D`][crate::classes::Area2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Area2D` enums](https://docs.godotengine.org/en/stable/classes/class_area2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Area2D.`\n\nInherits [`CollisionObject2D`][crate::classes::CollisionObject2D].\n\nRelated symbols:\n\n* [`area_2d`][crate::classes::area_2d]: sidecar module with related enum/flag types\n* [`IArea2D`][crate::classes::IArea2D]: virtual methods\n* [`SignalsOfArea2D`][crate::classes::area_2d::SignalsOfArea2D]: signal collection\n\n\nSee also [Godot docs for `Area2D`](https://docs.godotengine.org/en/stable/classes/class_area2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Area2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Area2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Area2D`][crate::classes::Area2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`ICollisionObject2D`~~ > [`INode2D`][crate::classes::INode2D] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Area2D` methods](https://docs.godotengine.org/en/stable/classes/class_area2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IArea2D: crate::obj::GodotClass < Base = Area2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
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
        fn input_event(&mut self, viewport: Gd < crate::classes::Viewport >, event: Gd < crate::classes::InputEvent >, shape_idx: i32,) {
            unimplemented !()
        }
        fn mouse_enter(&mut self,) {
            unimplemented !()
        }
        fn mouse_exit(&mut self,) {
            unimplemented !()
        }
        fn mouse_shape_enter(&mut self, shape_idx: i32,) {
            unimplemented !()
        }
        fn mouse_shape_exit(&mut self, shape_idx: i32,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
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
    impl Area2D {
        pub fn set_gravity_space_override_mode(&mut self, space_override_mode: crate::classes::area_2d::SpaceOverride,) {
            type CallRet = ();
            type CallParams = (crate::classes::area_2d::SpaceOverride,);
            let args = (space_override_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(550usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_gravity_space_override_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity_space_override_mode(&self,) -> crate::classes::area_2d::SpaceOverride {
            type CallRet = crate::classes::area_2d::SpaceOverride;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(551usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_gravity_space_override_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity_is_point(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(552usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_gravity_is_point", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_gravity_a_point(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(553usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "is_gravity_a_point", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity_point_unit_distance(&mut self, distance_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(554usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_gravity_point_unit_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity_point_unit_distance(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(555usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_gravity_point_unit_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity_point_center(&mut self, center: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (center,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(556usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_gravity_point_center", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity_point_center(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(557usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_gravity_point_center", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity_direction(&mut self, direction: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(558usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_gravity_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity_direction(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(559usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_gravity_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity(&mut self, gravity: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (gravity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(560usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(561usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_damp_space_override_mode(&mut self, space_override_mode: crate::classes::area_2d::SpaceOverride,) {
            type CallRet = ();
            type CallParams = (crate::classes::area_2d::SpaceOverride,);
            let args = (space_override_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(562usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_linear_damp_space_override_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_damp_space_override_mode(&self,) -> crate::classes::area_2d::SpaceOverride {
            type CallRet = crate::classes::area_2d::SpaceOverride;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(563usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_linear_damp_space_override_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_damp_space_override_mode(&mut self, space_override_mode: crate::classes::area_2d::SpaceOverride,) {
            type CallRet = ();
            type CallParams = (crate::classes::area_2d::SpaceOverride,);
            let args = (space_override_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(564usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_angular_damp_space_override_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_damp_space_override_mode(&self,) -> crate::classes::area_2d::SpaceOverride {
            type CallRet = crate::classes::area_2d::SpaceOverride;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(565usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_angular_damp_space_override_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_damp(&mut self, linear_damp: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (linear_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(566usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_linear_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(567usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_linear_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_damp(&mut self, angular_damp: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angular_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(568usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_angular_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(569usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_angular_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_priority(&mut self, priority: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(570usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_priority(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(571usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_monitoring(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(572usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_monitoring", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_monitoring(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(573usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "is_monitoring", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_monitorable(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(574usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_monitorable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_monitorable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(575usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "is_monitorable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_overlapping_bodies(&self,) -> Array < Gd < crate::classes::Node2D > > {
            type CallRet = Array < Gd < crate::classes::Node2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(576usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_overlapping_bodies", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_overlapping_areas(&self,) -> Array < Gd < crate::classes::Area2D > > {
            type CallRet = Array < Gd < crate::classes::Area2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(577usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_overlapping_areas", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_overlapping_bodies(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(578usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "has_overlapping_bodies", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_overlapping_areas(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(579usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "has_overlapping_areas", Some(self.__validated_obj()), args,)
            }
        }
        pub fn overlaps_body(&self, body: impl AsArg < Option < Gd < crate::classes::Node >> >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (body.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(580usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "overlaps_body", Some(self.__validated_obj()), args,)
            }
        }
        pub fn overlaps_area(&self, area: impl AsArg < Option < Gd < crate::classes::Node >> >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (area.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(581usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "overlaps_area", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_audio_bus_name(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(582usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_audio_bus_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_audio_bus_name(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(583usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "get_audio_bus_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_audio_bus_override(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(584usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "set_audio_bus_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_overriding_audio_bus(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(585usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Area2D", "is_overriding_audio_bus", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Area2D {
        type Base = crate::classes::CollisionObject2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Area2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Area2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject2D > for Area2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for Area2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Area2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Area2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Area2D {
        
    }
    impl crate::obj::cap::GodotDefault for Area2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Area2D {
        type Target = crate::classes::CollisionObject2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Area2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Area2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Area2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Area2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CollisionObject2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
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
pub struct SpaceOverride {
    ord: i32
}
impl SpaceOverride {
    #[doc(alias = "SPACE_OVERRIDE_DISABLED")]
    #[doc = "Godot enumerator name: `SPACE_OVERRIDE_DISABLED`"]
    pub const DISABLED: SpaceOverride = SpaceOverride {
        ord: 0i32
    };
    #[doc(alias = "SPACE_OVERRIDE_COMBINE")]
    #[doc = "Godot enumerator name: `SPACE_OVERRIDE_COMBINE`"]
    pub const COMBINE: SpaceOverride = SpaceOverride {
        ord: 1i32
    };
    #[doc(alias = "SPACE_OVERRIDE_COMBINE_REPLACE")]
    #[doc = "Godot enumerator name: `SPACE_OVERRIDE_COMBINE_REPLACE`"]
    pub const COMBINE_REPLACE: SpaceOverride = SpaceOverride {
        ord: 2i32
    };
    #[doc(alias = "SPACE_OVERRIDE_REPLACE")]
    #[doc = "Godot enumerator name: `SPACE_OVERRIDE_REPLACE`"]
    pub const REPLACE: SpaceOverride = SpaceOverride {
        ord: 3i32
    };
    #[doc(alias = "SPACE_OVERRIDE_REPLACE_COMBINE")]
    #[doc = "Godot enumerator name: `SPACE_OVERRIDE_REPLACE_COMBINE`"]
    pub const REPLACE_COMBINE: SpaceOverride = SpaceOverride {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for SpaceOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SpaceOverride") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SpaceOverride {
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
            Self::DISABLED => "DISABLED", Self::COMBINE => "COMBINE", Self::COMBINE_REPLACE => "COMBINE_REPLACE", Self::REPLACE => "REPLACE", Self::REPLACE_COMBINE => "REPLACE_COMBINE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SpaceOverride::DISABLED, SpaceOverride::COMBINE, SpaceOverride::COMBINE_REPLACE, SpaceOverride::REPLACE, SpaceOverride::REPLACE_COMBINE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SpaceOverride >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "SPACE_OVERRIDE_DISABLED", SpaceOverride::DISABLED), crate::meta::inspect::EnumConstant::new("COMBINE", "SPACE_OVERRIDE_COMBINE", SpaceOverride::COMBINE), crate::meta::inspect::EnumConstant::new("COMBINE_REPLACE", "SPACE_OVERRIDE_COMBINE_REPLACE", SpaceOverride::COMBINE_REPLACE), crate::meta::inspect::EnumConstant::new("REPLACE", "SPACE_OVERRIDE_REPLACE", SpaceOverride::REPLACE), crate::meta::inspect::EnumConstant::new("REPLACE_COMBINE", "SPACE_OVERRIDE_REPLACE_COMBINE", SpaceOverride::REPLACE_COMBINE)]
        }
    }
}
impl crate::meta::GodotConvert for SpaceOverride {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SpaceOverride {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SpaceOverride {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Area2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Area2D`][crate::classes::Area2D] class."]
    pub struct SignalsOfArea2D < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfArea2D < 'c, C > {
        #[doc = "Signature: `(body_rid: Rid, body: Gd<Node2D>, body_shape_index: i64, local_shape_index: i64)`"]
        pub fn body_shape_entered(&mut self) -> SigBodyShapeEntered < 'c, C > {
            SigBodyShapeEntered {
                typed: TypedSignal::extract(&mut self.__internal_obj, "body_shape_entered")
            }
        }
        #[doc = "Signature: `(body_rid: Rid, body: Gd<Node2D>, body_shape_index: i64, local_shape_index: i64)`"]
        pub fn body_shape_exited(&mut self) -> SigBodyShapeExited < 'c, C > {
            SigBodyShapeExited {
                typed: TypedSignal::extract(&mut self.__internal_obj, "body_shape_exited")
            }
        }
        #[doc = "Signature: `(body: Gd<Node2D>)`"]
        pub fn body_entered(&mut self) -> SigBodyEntered < 'c, C > {
            SigBodyEntered {
                typed: TypedSignal::extract(&mut self.__internal_obj, "body_entered")
            }
        }
        #[doc = "Signature: `(body: Gd<Node2D>)`"]
        pub fn body_exited(&mut self) -> SigBodyExited < 'c, C > {
            SigBodyExited {
                typed: TypedSignal::extract(&mut self.__internal_obj, "body_exited")
            }
        }
        #[doc = "Signature: `(area_rid: Rid, area: Gd<Area2D>, area_shape_index: i64, local_shape_index: i64)`"]
        pub fn area_shape_entered(&mut self) -> SigAreaShapeEntered < 'c, C > {
            SigAreaShapeEntered {
                typed: TypedSignal::extract(&mut self.__internal_obj, "area_shape_entered")
            }
        }
        #[doc = "Signature: `(area_rid: Rid, area: Gd<Area2D>, area_shape_index: i64, local_shape_index: i64)`"]
        pub fn area_shape_exited(&mut self) -> SigAreaShapeExited < 'c, C > {
            SigAreaShapeExited {
                typed: TypedSignal::extract(&mut self.__internal_obj, "area_shape_exited")
            }
        }
        #[doc = "Signature: `(area: Gd<Area2D>)`"]
        pub fn area_entered(&mut self) -> SigAreaEntered < 'c, C > {
            SigAreaEntered {
                typed: TypedSignal::extract(&mut self.__internal_obj, "area_entered")
            }
        }
        #[doc = "Signature: `(area: Gd<Area2D>)`"]
        pub fn area_exited(&mut self) -> SigAreaExited < 'c, C > {
            SigAreaExited {
                typed: TypedSignal::extract(&mut self.__internal_obj, "area_exited")
            }
        }
    }
    type TypedSigBodyShapeEntered < 'c, C > = TypedSignal < 'c, C, (Rid, Gd < crate::classes::Node2D >, i64, i64,) >;
    pub struct SigBodyShapeEntered < 'c, C: WithSignals > {
        typed: TypedSigBodyShapeEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBodyShapeEntered < 'c, C > {
        pub fn emit(&mut self, body_rid: Rid, body: Gd < crate::classes::Node2D >, body_shape_index: i64, local_shape_index: i64,) {
            self.typed.emit_tuple((body_rid, body, body_shape_index, local_shape_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigBodyShapeEntered < 'c, C > {
        type Target = TypedSigBodyShapeEntered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigBodyShapeEntered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigBodyShapeExited < 'c, C > = TypedSignal < 'c, C, (Rid, Gd < crate::classes::Node2D >, i64, i64,) >;
    pub struct SigBodyShapeExited < 'c, C: WithSignals > {
        typed: TypedSigBodyShapeExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBodyShapeExited < 'c, C > {
        pub fn emit(&mut self, body_rid: Rid, body: Gd < crate::classes::Node2D >, body_shape_index: i64, local_shape_index: i64,) {
            self.typed.emit_tuple((body_rid, body, body_shape_index, local_shape_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigBodyShapeExited < 'c, C > {
        type Target = TypedSigBodyShapeExited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigBodyShapeExited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigBodyEntered < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node2D >,) >;
    pub struct SigBodyEntered < 'c, C: WithSignals > {
        typed: TypedSigBodyEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBodyEntered < 'c, C > {
        pub fn emit(&mut self, body: Gd < crate::classes::Node2D >,) {
            self.typed.emit_tuple((body,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigBodyEntered < 'c, C > {
        type Target = TypedSigBodyEntered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigBodyEntered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigBodyExited < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node2D >,) >;
    pub struct SigBodyExited < 'c, C: WithSignals > {
        typed: TypedSigBodyExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBodyExited < 'c, C > {
        pub fn emit(&mut self, body: Gd < crate::classes::Node2D >,) {
            self.typed.emit_tuple((body,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigBodyExited < 'c, C > {
        type Target = TypedSigBodyExited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigBodyExited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAreaShapeEntered < 'c, C > = TypedSignal < 'c, C, (Rid, Gd < crate::classes::Area2D >, i64, i64,) >;
    pub struct SigAreaShapeEntered < 'c, C: WithSignals > {
        typed: TypedSigAreaShapeEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAreaShapeEntered < 'c, C > {
        pub fn emit(&mut self, area_rid: Rid, area: Gd < crate::classes::Area2D >, area_shape_index: i64, local_shape_index: i64,) {
            self.typed.emit_tuple((area_rid, area, area_shape_index, local_shape_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAreaShapeEntered < 'c, C > {
        type Target = TypedSigAreaShapeEntered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAreaShapeEntered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAreaShapeExited < 'c, C > = TypedSignal < 'c, C, (Rid, Gd < crate::classes::Area2D >, i64, i64,) >;
    pub struct SigAreaShapeExited < 'c, C: WithSignals > {
        typed: TypedSigAreaShapeExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAreaShapeExited < 'c, C > {
        pub fn emit(&mut self, area_rid: Rid, area: Gd < crate::classes::Area2D >, area_shape_index: i64, local_shape_index: i64,) {
            self.typed.emit_tuple((area_rid, area, area_shape_index, local_shape_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAreaShapeExited < 'c, C > {
        type Target = TypedSigAreaShapeExited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAreaShapeExited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAreaEntered < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Area2D >,) >;
    pub struct SigAreaEntered < 'c, C: WithSignals > {
        typed: TypedSigAreaEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAreaEntered < 'c, C > {
        pub fn emit(&mut self, area: Gd < crate::classes::Area2D >,) {
            self.typed.emit_tuple((area,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAreaEntered < 'c, C > {
        type Target = TypedSigAreaEntered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAreaEntered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAreaExited < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Area2D >,) >;
    pub struct SigAreaExited < 'c, C: WithSignals > {
        typed: TypedSigAreaExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAreaExited < 'c, C > {
        pub fn emit(&mut self, area: Gd < crate::classes::Area2D >,) {
            self.typed.emit_tuple((area,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAreaExited < 'c, C > {
        type Target = TypedSigAreaExited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAreaExited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Area2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfArea2D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfArea2D < 'c, C > {
        type Target = < < Area2D as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Area2D;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfArea2D < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Area2D;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}