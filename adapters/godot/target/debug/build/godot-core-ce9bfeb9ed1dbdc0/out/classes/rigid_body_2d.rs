#![doc = "Sidecar module for class [`RigidBody2D`][crate::classes::RigidBody2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RigidBody2D` enums](https://docs.godotengine.org/en/stable/classes/class_rigidbody2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RigidBody2D.`\n\nInherits [`PhysicsBody2D`][crate::classes::PhysicsBody2D].\n\nRelated symbols:\n\n* [`rigid_body_2d`][crate::classes::rigid_body_2d]: sidecar module with related enum/flag types\n* [`IRigidBody2D`][crate::classes::IRigidBody2D]: virtual methods\n* [`SignalsOfRigidBody2D`][crate::classes::rigid_body_2d::SignalsOfRigidBody2D]: signal collection\n\n\nSee also [Godot docs for `RigidBody2D`](https://docs.godotengine.org/en/stable/classes/class_rigidbody2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`RigidBody2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RigidBody2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`RigidBody2D`][crate::classes::RigidBody2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IPhysicsBody2D`~~ > ~~`ICollisionObject2D`~~ > [`INode2D`][crate::classes::INode2D] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `RigidBody2D` methods](https://docs.godotengine.org/en/stable/classes/class_rigidbody2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRigidBody2D: crate::obj::GodotClass < Base = RigidBody2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn integrate_forces(&mut self, state: Option < Gd < crate::classes::PhysicsDirectBodyState2D > >,) {
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
    impl RigidBody2D {
        pub fn set_mass(&mut self, mass: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (mass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7660usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mass(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7661usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inertia(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7662usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_inertia", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_inertia(&mut self, inertia: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (inertia,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7663usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_inertia", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_center_of_mass_mode(&mut self, mode: crate::classes::rigid_body_2d::CenterOfMassMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::rigid_body_2d::CenterOfMassMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7664usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_center_of_mass_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_of_mass_mode(&self,) -> crate::classes::rigid_body_2d::CenterOfMassMode {
            type CallRet = crate::classes::rigid_body_2d::CenterOfMassMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7665usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_center_of_mass_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_center_of_mass(&mut self, center_of_mass: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (center_of_mass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7666usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_center_of_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_of_mass(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7667usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_center_of_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_material_override(&mut self, physics_material_override: impl AsArg < Option < Gd < crate::classes::PhysicsMaterial >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::PhysicsMaterial > > >,);
            let args = (physics_material_override.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_physics_material_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_material_override(&self,) -> Option < Gd < crate::classes::PhysicsMaterial > > {
            type CallRet = Option < Gd < crate::classes::PhysicsMaterial > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7669usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_physics_material_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity_scale(&mut self, gravity_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (gravity_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_gravity_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_gravity_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_damp_mode(&mut self, linear_damp_mode: crate::classes::rigid_body_2d::DampMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::rigid_body_2d::DampMode,);
            let args = (linear_damp_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_linear_damp_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_damp_mode(&self,) -> crate::classes::rigid_body_2d::DampMode {
            type CallRet = crate::classes::rigid_body_2d::DampMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7673usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_linear_damp_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_damp_mode(&mut self, angular_damp_mode: crate::classes::rigid_body_2d::DampMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::rigid_body_2d::DampMode,);
            let args = (angular_damp_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7674usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_angular_damp_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_damp_mode(&self,) -> crate::classes::rigid_body_2d::DampMode {
            type CallRet = crate::classes::rigid_body_2d::DampMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7675usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_angular_damp_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_damp(&mut self, linear_damp: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (linear_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7676usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_linear_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7677usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_linear_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_damp(&mut self, angular_damp: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angular_damp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7678usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_angular_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7679usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_angular_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_velocity(&mut self, linear_velocity: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (linear_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7680usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_velocity(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7681usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_velocity(&mut self, angular_velocity: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angular_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7682usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_velocity(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7683usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_contacts_reported(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7684usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_max_contacts_reported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_contacts_reported(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7685usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_max_contacts_reported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7686usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_contact_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_custom_integrator(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7687usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_use_custom_integrator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_custom_integrator(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7688usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "is_using_custom_integrator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_contact_monitor(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7689usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_contact_monitor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_contact_monitor_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7690usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "is_contact_monitor_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_continuous_collision_detection_mode(&mut self, mode: crate::classes::rigid_body_2d::CcdMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::rigid_body_2d::CcdMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_continuous_collision_detection_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_continuous_collision_detection_mode(&self,) -> crate::classes::rigid_body_2d::CcdMode {
            type CallRet = crate::classes::rigid_body_2d::CcdMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_continuous_collision_detection_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_axis_velocity(&mut self, axis_velocity: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (axis_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_axis_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn apply_central_impulse_full(&mut self, impulse: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "apply_central_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::apply_central_impulse_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn apply_central_impulse(&mut self,) {
            self.apply_central_impulse_ex() . done()
        }
        #[inline]
        pub fn apply_central_impulse_ex < 'a > (&'a mut self,) -> ExApplyCentralImpulse < 'a > {
            ExApplyCentralImpulse::new(self,)
        }
        pub(crate) fn apply_impulse_full(&mut self, impulse: Vector2, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2,);
            let args = (impulse, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "apply_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::apply_impulse_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn apply_impulse(&mut self, impulse: Vector2,) {
            self.apply_impulse_ex(impulse,) . done()
        }
        #[inline]
        pub fn apply_impulse_ex < 'a > (&'a mut self, impulse: Vector2,) -> ExApplyImpulse < 'a > {
            ExApplyImpulse::new(self, impulse,)
        }
        pub fn apply_torque_impulse(&mut self, torque: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "apply_torque_impulse", Some(self.__validated_obj()), args,)
            }
        }
        pub fn apply_central_force(&mut self, force: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "apply_central_force", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn apply_force_full(&mut self, force: Vector2, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2,);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "apply_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::apply_force_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn apply_force(&mut self, force: Vector2,) {
            self.apply_force_ex(force,) . done()
        }
        #[inline]
        pub fn apply_force_ex < 'a > (&'a mut self, force: Vector2,) -> ExApplyForce < 'a > {
            ExApplyForce::new(self, force,)
        }
        pub fn apply_torque(&mut self, torque: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "apply_torque", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_constant_central_force(&mut self, force: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "add_constant_central_force", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_constant_force_full(&mut self, force: Vector2, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2,);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "add_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_constant_force_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_constant_force(&mut self, force: Vector2,) {
            self.add_constant_force_ex(force,) . done()
        }
        #[inline]
        pub fn add_constant_force_ex < 'a > (&'a mut self, force: Vector2,) -> ExAddConstantForce < 'a > {
            ExAddConstantForce::new(self, force,)
        }
        pub fn add_constant_torque(&mut self, torque: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7702usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "add_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_constant_force(&mut self, force: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7703usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_constant_force(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_constant_torque(&mut self, torque: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_constant_torque(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sleeping(&mut self, sleeping: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (sleeping,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7707usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_sleeping", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_sleeping(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7708usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "is_sleeping", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_can_sleep(&mut self, able_to_sleep: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (able_to_sleep,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7709usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_can_sleep", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_able_to_sleep(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7710usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "is_able_to_sleep", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lock_rotation_enabled(&mut self, lock_rotation: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (lock_rotation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7711usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_lock_rotation_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_lock_rotation_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7712usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "is_lock_rotation_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_freeze_enabled(&mut self, freeze_mode: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (freeze_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7713usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_freeze_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_freeze_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7714usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "is_freeze_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_freeze_mode(&mut self, freeze_mode: crate::classes::rigid_body_2d::FreezeMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::rigid_body_2d::FreezeMode,);
            let args = (freeze_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7715usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "set_freeze_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_freeze_mode(&self,) -> crate::classes::rigid_body_2d::FreezeMode {
            type CallRet = crate::classes::rigid_body_2d::FreezeMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7716usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_freeze_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_colliding_bodies(&self,) -> Array < Gd < crate::classes::Node2D > > {
            type CallRet = Array < Gd < crate::classes::Node2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7717usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RigidBody2D", "get_colliding_bodies", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for RigidBody2D {
        type Base = crate::classes::PhysicsBody2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("RigidBody2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RigidBody2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PhysicsBody2D > for RigidBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject2D > for RigidBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for RigidBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for RigidBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for RigidBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RigidBody2D {
        
    }
    impl crate::obj::cap::GodotDefault for RigidBody2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RigidBody2D {
        type Target = crate::classes::PhysicsBody2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RigidBody2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RigidBody2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RigidBody2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RigidBody2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsBody2D > for $Class {
                
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
#[doc = "Default-param extender for [`RigidBody2D::apply_central_impulse_ex`][super::RigidBody2D::apply_central_impulse_ex]."]
#[must_use]
pub struct ExApplyCentralImpulse < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RigidBody2D, impulse: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyCentralImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::RigidBody2D,) -> Self {
        let impulse = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, impulse: impulse,
        }
    }
    #[inline]
    pub fn impulse(self, impulse: Vector2) -> Self {
        Self {
            impulse: impulse, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, impulse,
        }
        = self;
        re_export::RigidBody2D::apply_central_impulse_full(surround_object, impulse,)
    }
}
#[doc = "Default-param extender for [`RigidBody2D::apply_impulse_ex`][super::RigidBody2D::apply_impulse_ex]."]
#[must_use]
pub struct ExApplyImpulse < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RigidBody2D, impulse: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::RigidBody2D, impulse: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, impulse: impulse, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
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
        re_export::RigidBody2D::apply_impulse_full(surround_object, impulse, position,)
    }
}
#[doc = "Default-param extender for [`RigidBody2D::apply_force_ex`][super::RigidBody2D::apply_force_ex]."]
#[must_use]
pub struct ExApplyForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RigidBody2D, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyForce < 'a > {
    fn new(surround_object: &'a mut re_export::RigidBody2D, force: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force, position,
        }
        = self;
        re_export::RigidBody2D::apply_force_full(surround_object, force, position,)
    }
}
#[doc = "Default-param extender for [`RigidBody2D::add_constant_force_ex`][super::RigidBody2D::add_constant_force_ex]."]
#[must_use]
pub struct ExAddConstantForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RigidBody2D, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddConstantForce < 'a > {
    fn new(surround_object: &'a mut re_export::RigidBody2D, force: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force, position,
        }
        = self;
        re_export::RigidBody2D::add_constant_force_full(surround_object, force, position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FreezeMode {
    ord: i32
}
impl FreezeMode {
    #[doc(alias = "FREEZE_MODE_STATIC")]
    #[doc = "Godot enumerator name: `FREEZE_MODE_STATIC`"]
    pub const STATIC: FreezeMode = FreezeMode {
        ord: 0i32
    };
    #[doc(alias = "FREEZE_MODE_KINEMATIC")]
    #[doc = "Godot enumerator name: `FREEZE_MODE_KINEMATIC`"]
    pub const KINEMATIC: FreezeMode = FreezeMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for FreezeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FreezeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FreezeMode {
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
            Self::STATIC => "STATIC", Self::KINEMATIC => "KINEMATIC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FreezeMode::STATIC, FreezeMode::KINEMATIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FreezeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STATIC", "FREEZE_MODE_STATIC", FreezeMode::STATIC), crate::meta::inspect::EnumConstant::new("KINEMATIC", "FREEZE_MODE_KINEMATIC", FreezeMode::KINEMATIC)]
        }
    }
}
impl crate::meta::GodotConvert for FreezeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FreezeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FreezeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CenterOfMassMode {
    ord: i32
}
impl CenterOfMassMode {
    #[doc(alias = "CENTER_OF_MASS_MODE_AUTO")]
    #[doc = "Godot enumerator name: `CENTER_OF_MASS_MODE_AUTO`"]
    pub const AUTO: CenterOfMassMode = CenterOfMassMode {
        ord: 0i32
    };
    #[doc(alias = "CENTER_OF_MASS_MODE_CUSTOM")]
    #[doc = "Godot enumerator name: `CENTER_OF_MASS_MODE_CUSTOM`"]
    pub const CUSTOM: CenterOfMassMode = CenterOfMassMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for CenterOfMassMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CenterOfMassMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CenterOfMassMode {
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
            Self::AUTO => "AUTO", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CenterOfMassMode::AUTO, CenterOfMassMode::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CenterOfMassMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("AUTO", "CENTER_OF_MASS_MODE_AUTO", CenterOfMassMode::AUTO), crate::meta::inspect::EnumConstant::new("CUSTOM", "CENTER_OF_MASS_MODE_CUSTOM", CenterOfMassMode::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for CenterOfMassMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CenterOfMassMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CenterOfMassMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
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
#[doc = "Godot enum name: `CCDMode`."]
pub struct CcdMode {
    ord: i32
}
impl CcdMode {
    #[doc(alias = "CCD_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `CCD_MODE_DISABLED`"]
    pub const DISABLED: CcdMode = CcdMode {
        ord: 0i32
    };
    #[doc(alias = "CCD_MODE_CAST_RAY")]
    #[doc = "Godot enumerator name: `CCD_MODE_CAST_RAY`"]
    pub const CAST_RAY: CcdMode = CcdMode {
        ord: 1i32
    };
    #[doc(alias = "CCD_MODE_CAST_SHAPE")]
    #[doc = "Godot enumerator name: `CCD_MODE_CAST_SHAPE`"]
    pub const CAST_SHAPE: CcdMode = CcdMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for CcdMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CcdMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CcdMode {
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
            Self::DISABLED => "DISABLED", Self::CAST_RAY => "CAST_RAY", Self::CAST_SHAPE => "CAST_SHAPE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CcdMode::DISABLED, CcdMode::CAST_RAY, CcdMode::CAST_SHAPE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CcdMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "CCD_MODE_DISABLED", CcdMode::DISABLED), crate::meta::inspect::EnumConstant::new("CAST_RAY", "CCD_MODE_CAST_RAY", CcdMode::CAST_RAY), crate::meta::inspect::EnumConstant::new("CAST_SHAPE", "CCD_MODE_CAST_SHAPE", CcdMode::CAST_SHAPE)]
        }
    }
}
impl crate::meta::GodotConvert for CcdMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CcdMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CcdMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::RigidBody2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`RigidBody2D`][crate::classes::RigidBody2D] class."]
    pub struct SignalsOfRigidBody2D < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfRigidBody2D < 'c, C > {
        #[doc = "Signature: `(body_rid: Rid, body: Gd<Node>, body_shape_index: i64, local_shape_index: i64)`"]
        pub fn body_shape_entered(&mut self) -> SigBodyShapeEntered < 'c, C > {
            SigBodyShapeEntered {
                typed: TypedSignal::extract(&mut self.__internal_obj, "body_shape_entered")
            }
        }
        #[doc = "Signature: `(body_rid: Rid, body: Gd<Node>, body_shape_index: i64, local_shape_index: i64)`"]
        pub fn body_shape_exited(&mut self) -> SigBodyShapeExited < 'c, C > {
            SigBodyShapeExited {
                typed: TypedSignal::extract(&mut self.__internal_obj, "body_shape_exited")
            }
        }
        #[doc = "Signature: `(body: Gd<Node>)`"]
        pub fn body_entered(&mut self) -> SigBodyEntered < 'c, C > {
            SigBodyEntered {
                typed: TypedSignal::extract(&mut self.__internal_obj, "body_entered")
            }
        }
        #[doc = "Signature: `(body: Gd<Node>)`"]
        pub fn body_exited(&mut self) -> SigBodyExited < 'c, C > {
            SigBodyExited {
                typed: TypedSignal::extract(&mut self.__internal_obj, "body_exited")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn sleeping_state_changed(&mut self) -> SigSleepingStateChanged < 'c, C > {
            SigSleepingStateChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "sleeping_state_changed")
            }
        }
    }
    type TypedSigBodyShapeEntered < 'c, C > = TypedSignal < 'c, C, (Rid, Gd < crate::classes::Node >, i64, i64,) >;
    pub struct SigBodyShapeEntered < 'c, C: WithSignals > {
        typed: TypedSigBodyShapeEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBodyShapeEntered < 'c, C > {
        pub fn emit(&mut self, body_rid: Rid, body: Gd < crate::classes::Node >, body_shape_index: i64, local_shape_index: i64,) {
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
    type TypedSigBodyShapeExited < 'c, C > = TypedSignal < 'c, C, (Rid, Gd < crate::classes::Node >, i64, i64,) >;
    pub struct SigBodyShapeExited < 'c, C: WithSignals > {
        typed: TypedSigBodyShapeExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBodyShapeExited < 'c, C > {
        pub fn emit(&mut self, body_rid: Rid, body: Gd < crate::classes::Node >, body_shape_index: i64, local_shape_index: i64,) {
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
    type TypedSigBodyEntered < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigBodyEntered < 'c, C: WithSignals > {
        typed: TypedSigBodyEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBodyEntered < 'c, C > {
        pub fn emit(&mut self, body: Gd < crate::classes::Node >,) {
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
    type TypedSigBodyExited < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigBodyExited < 'c, C: WithSignals > {
        typed: TypedSigBodyExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBodyExited < 'c, C > {
        pub fn emit(&mut self, body: Gd < crate::classes::Node >,) {
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
    type TypedSigSleepingStateChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSleepingStateChanged < 'c, C: WithSignals > {
        typed: TypedSigSleepingStateChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSleepingStateChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSleepingStateChanged < 'c, C > {
        type Target = TypedSigSleepingStateChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSleepingStateChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for RigidBody2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfRigidBody2D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfRigidBody2D < 'c, C > {
        type Target = < < RigidBody2D as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = RigidBody2D;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfRigidBody2D < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = RigidBody2D;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}