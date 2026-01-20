#![doc = "Sidecar module for class [`PhysicsServer2DExtension`][crate::classes::PhysicsServer2DExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsServer2DExtension` enums](https://docs.godotengine.org/en/stable/classes/class_physicsserver2dextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsServer2DExtension.`\n\nInherits [`PhysicsServer2D`][crate::classes::PhysicsServer2D].\n\nRelated symbols:\n\n* [`IPhysicsServer2DExtension`][crate::classes::IPhysicsServer2DExtension]: virtual methods\n\n\nSee also [Godot docs for `PhysicsServer2DExtension`](https://docs.godotengine.org/en/stable/classes/class_physicsserver2dextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`PhysicsServer2DExtension::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsServer2DExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`PhysicsServer2DExtension`][crate::classes::PhysicsServer2DExtension].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IPhysicsServer2D`~~ > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `PhysicsServer2DExtension` methods](https://docs.godotengine.org/en/stable/classes/class_physicsserver2dextension.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicsServer2DExtension: crate::obj::GodotClass < Base = PhysicsServer2DExtension > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn world_boundary_shape_create(&mut self,) -> Rid;
        fn separation_ray_shape_create(&mut self,) -> Rid;
        fn segment_shape_create(&mut self,) -> Rid;
        fn circle_shape_create(&mut self,) -> Rid;
        fn rectangle_shape_create(&mut self,) -> Rid;
        fn capsule_shape_create(&mut self,) -> Rid;
        fn convex_polygon_shape_create(&mut self,) -> Rid;
        fn concave_polygon_shape_create(&mut self,) -> Rid;
        fn shape_set_data(&mut self, shape: Rid, data: Variant,);
        fn shape_set_custom_solver_bias(&mut self, shape: Rid, bias: f32,);
        fn shape_get_type(&self, shape: Rid,) -> crate::classes::physics_server_2d::ShapeType;
        fn shape_get_data(&self, shape: Rid,) -> Variant;
        fn shape_get_custom_solver_bias(&self, shape: Rid,) -> f32;
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn shape_collide_rawptr(&mut self, shape_A: Rid, xform_A: Transform2D, motion_A: Vector2, shape_B: Rid, xform_B: Transform2D, motion_B: Vector2, results: * mut c_void, result_max: i32, result_count: * mut i32,) -> bool;
        fn space_create(&mut self,) -> Rid;
        fn space_set_active(&mut self, space: Rid, active: bool,);
        fn space_is_active(&self, space: Rid,) -> bool;
        fn space_set_param(&mut self, space: Rid, param: crate::classes::physics_server_2d::SpaceParameter, value: f32,);
        fn space_get_param(&self, space: Rid, param: crate::classes::physics_server_2d::SpaceParameter,) -> f32;
        fn space_get_direct_state(&mut self, space: Rid,) -> Option < Gd < crate::classes::PhysicsDirectSpaceState2D > >;
        fn space_set_debug_contacts(&mut self, space: Rid, max_contacts: i32,);
        fn space_get_contacts(&self, space: Rid,) -> PackedVector2Array;
        fn space_get_contact_count(&self, space: Rid,) -> i32;
        fn area_create(&mut self,) -> Rid;
        fn area_set_space(&mut self, area: Rid, space: Rid,);
        fn area_get_space(&self, area: Rid,) -> Rid;
        fn area_add_shape(&mut self, area: Rid, shape: Rid, transform: Transform2D, disabled: bool,);
        fn area_set_shape(&mut self, area: Rid, shape_idx: i32, shape: Rid,);
        fn area_set_shape_transform(&mut self, area: Rid, shape_idx: i32, transform: Transform2D,);
        fn area_set_shape_disabled(&mut self, area: Rid, shape_idx: i32, disabled: bool,);
        fn area_get_shape_count(&self, area: Rid,) -> i32;
        fn area_get_shape(&self, area: Rid, shape_idx: i32,) -> Rid;
        fn area_get_shape_transform(&self, area: Rid, shape_idx: i32,) -> Transform2D;
        fn area_remove_shape(&mut self, area: Rid, shape_idx: i32,);
        fn area_clear_shapes(&mut self, area: Rid,);
        fn area_attach_object_instance_id(&mut self, area: Rid, id: u64,);
        fn area_get_object_instance_id(&self, area: Rid,) -> u64;
        fn area_attach_canvas_instance_id(&mut self, area: Rid, id: u64,);
        fn area_get_canvas_instance_id(&self, area: Rid,) -> u64;
        fn area_set_param(&mut self, area: Rid, param: crate::classes::physics_server_2d::AreaParameter, value: Variant,);
        fn area_set_transform(&mut self, area: Rid, transform: Transform2D,);
        fn area_get_param(&self, area: Rid, param: crate::classes::physics_server_2d::AreaParameter,) -> Variant;
        fn area_get_transform(&self, area: Rid,) -> Transform2D;
        fn area_set_collision_layer(&mut self, area: Rid, layer: u32,);
        fn area_get_collision_layer(&self, area: Rid,) -> u32;
        fn area_set_collision_mask(&mut self, area: Rid, mask: u32,);
        fn area_get_collision_mask(&self, area: Rid,) -> u32;
        fn area_set_monitorable(&mut self, area: Rid, monitorable: bool,);
        fn area_set_pickable(&mut self, area: Rid, pickable: bool,);
        fn area_set_monitor_callback(&mut self, area: Rid, callback: Callable,);
        fn area_set_area_monitor_callback(&mut self, area: Rid, callback: Callable,);
        fn body_create(&mut self,) -> Rid;
        fn body_set_space(&mut self, body: Rid, space: Rid,);
        fn body_get_space(&self, body: Rid,) -> Rid;
        fn body_set_mode(&mut self, body: Rid, mode: crate::classes::physics_server_2d::BodyMode,);
        fn body_get_mode(&self, body: Rid,) -> crate::classes::physics_server_2d::BodyMode;
        fn body_add_shape(&mut self, body: Rid, shape: Rid, transform: Transform2D, disabled: bool,);
        fn body_set_shape(&mut self, body: Rid, shape_idx: i32, shape: Rid,);
        fn body_set_shape_transform(&mut self, body: Rid, shape_idx: i32, transform: Transform2D,);
        fn body_get_shape_count(&self, body: Rid,) -> i32;
        fn body_get_shape(&self, body: Rid, shape_idx: i32,) -> Rid;
        fn body_get_shape_transform(&self, body: Rid, shape_idx: i32,) -> Transform2D;
        fn body_set_shape_disabled(&mut self, body: Rid, shape_idx: i32, disabled: bool,);
        fn body_set_shape_as_one_way_collision(&mut self, body: Rid, shape_idx: i32, enable: bool, margin: f32,);
        fn body_remove_shape(&mut self, body: Rid, shape_idx: i32,);
        fn body_clear_shapes(&mut self, body: Rid,);
        fn body_attach_object_instance_id(&mut self, body: Rid, id: u64,);
        fn body_get_object_instance_id(&self, body: Rid,) -> u64;
        fn body_attach_canvas_instance_id(&mut self, body: Rid, id: u64,);
        fn body_get_canvas_instance_id(&self, body: Rid,) -> u64;
        fn body_set_continuous_collision_detection_mode(&mut self, body: Rid, mode: crate::classes::physics_server_2d::CcdMode,);
        fn body_get_continuous_collision_detection_mode(&self, body: Rid,) -> crate::classes::physics_server_2d::CcdMode;
        fn body_set_collision_layer(&mut self, body: Rid, layer: u32,);
        fn body_get_collision_layer(&self, body: Rid,) -> u32;
        fn body_set_collision_mask(&mut self, body: Rid, mask: u32,);
        fn body_get_collision_mask(&self, body: Rid,) -> u32;
        fn body_set_collision_priority(&mut self, body: Rid, priority: f32,);
        fn body_get_collision_priority(&self, body: Rid,) -> f32;
        fn body_set_param(&mut self, body: Rid, param: crate::classes::physics_server_2d::BodyParameter, value: Variant,);
        fn body_get_param(&self, body: Rid, param: crate::classes::physics_server_2d::BodyParameter,) -> Variant;
        fn body_reset_mass_properties(&mut self, body: Rid,);
        fn body_set_state(&mut self, body: Rid, state: crate::classes::physics_server_2d::BodyState, value: Variant,);
        fn body_get_state(&self, body: Rid, state: crate::classes::physics_server_2d::BodyState,) -> Variant;
        fn body_apply_central_impulse(&mut self, body: Rid, impulse: Vector2,);
        fn body_apply_torque_impulse(&mut self, body: Rid, impulse: f32,);
        fn body_apply_impulse(&mut self, body: Rid, impulse: Vector2, position: Vector2,);
        fn body_apply_central_force(&mut self, body: Rid, force: Vector2,);
        fn body_apply_force(&mut self, body: Rid, force: Vector2, position: Vector2,);
        fn body_apply_torque(&mut self, body: Rid, torque: f32,);
        fn body_add_constant_central_force(&mut self, body: Rid, force: Vector2,);
        fn body_add_constant_force(&mut self, body: Rid, force: Vector2, position: Vector2,);
        fn body_add_constant_torque(&mut self, body: Rid, torque: f32,);
        fn body_set_constant_force(&mut self, body: Rid, force: Vector2,);
        fn body_get_constant_force(&self, body: Rid,) -> Vector2;
        fn body_set_constant_torque(&mut self, body: Rid, torque: f32,);
        fn body_get_constant_torque(&self, body: Rid,) -> f32;
        fn body_set_axis_velocity(&mut self, body: Rid, axis_velocity: Vector2,);
        fn body_add_collision_exception(&mut self, body: Rid, excepted_body: Rid,);
        fn body_remove_collision_exception(&mut self, body: Rid, excepted_body: Rid,);
        fn body_get_collision_exceptions(&self, body: Rid,) -> Array < Rid >;
        fn body_set_max_contacts_reported(&mut self, body: Rid, amount: i32,);
        fn body_get_max_contacts_reported(&self, body: Rid,) -> i32;
        fn body_set_contacts_reported_depth_threshold(&mut self, body: Rid, threshold: f32,);
        fn body_get_contacts_reported_depth_threshold(&self, body: Rid,) -> f32;
        fn body_set_omit_force_integration(&mut self, body: Rid, enable: bool,);
        fn body_is_omitting_force_integration(&self, body: Rid,) -> bool;
        fn body_set_state_sync_callback(&mut self, body: Rid, callable: Callable,);
        fn body_set_force_integration_callback(&mut self, body: Rid, callable: Callable, userdata: Variant,);
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn body_collide_shape_rawptr(&mut self, body: Rid, body_shape: i32, shape: Rid, shape_xform: Transform2D, motion: Vector2, results: * mut c_void, result_max: i32, result_count: * mut i32,) -> bool;
        fn body_set_pickable(&mut self, body: Rid, pickable: bool,);
        fn body_get_direct_state(&mut self, body: Rid,) -> Option < Gd < crate::classes::PhysicsDirectBodyState2D > >;
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn body_test_motion_rawptr(&self, body: Rid, from: Transform2D, motion: Vector2, margin: f32, collide_separation_ray: bool, recovery_as_collision: bool, result: * mut PhysicsServer2DExtensionMotionResult,) -> bool;
        fn joint_create(&mut self,) -> Rid;
        fn joint_clear(&mut self, joint: Rid,);
        fn joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_2d::JointParam, value: f32,);
        fn joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_2d::JointParam,) -> f32;
        fn joint_disable_collisions_between_bodies(&mut self, joint: Rid, disable: bool,);
        fn joint_is_disabled_collisions_between_bodies(&self, joint: Rid,) -> bool;
        fn joint_make_pin(&mut self, joint: Rid, anchor: Vector2, body_a: Rid, body_b: Rid,);
        fn joint_make_groove(&mut self, joint: Rid, a_groove1: Vector2, a_groove2: Vector2, b_anchor: Vector2, body_a: Rid, body_b: Rid,);
        fn joint_make_damped_spring(&mut self, joint: Rid, anchor_a: Vector2, anchor_b: Vector2, body_a: Rid, body_b: Rid,);
        fn pin_joint_set_flag(&mut self, joint: Rid, flag: crate::classes::physics_server_2d::PinJointFlag, enabled: bool,);
        fn pin_joint_get_flag(&self, joint: Rid, flag: crate::classes::physics_server_2d::PinJointFlag,) -> bool;
        fn pin_joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_2d::PinJointParam, value: f32,);
        fn pin_joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_2d::PinJointParam,) -> f32;
        fn damped_spring_joint_set_param(&mut self, joint: Rid, param: crate::classes::physics_server_2d::DampedSpringParam, value: f32,);
        fn damped_spring_joint_get_param(&self, joint: Rid, param: crate::classes::physics_server_2d::DampedSpringParam,) -> f32;
        fn joint_get_type(&self, joint: Rid,) -> crate::classes::physics_server_2d::JointType;
        fn free_rid(&mut self, rid: Rid,);
        fn set_active(&mut self, active: bool,);
        fn init_ext(&mut self,);
        fn step(&mut self, step: f32,);
        fn sync(&mut self,);
        fn flush_queries(&mut self,);
        fn end_sync(&mut self,);
        fn finish(&mut self,);
        fn is_flushing_queries(&self,) -> bool;
        fn get_process_info(&mut self, process_info: crate::classes::physics_server_2d::ProcessInfo,) -> i32;
        
    }
    impl PhysicsServer2DExtension {
        pub fn body_test_motion_is_excluding_body(&self, body: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(559usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2DExtension", "body_test_motion_is_excluding_body", Some(self.__validated_obj()), args,)
            }
        }
        pub fn body_test_motion_is_excluding_object(&self, object: u64,) -> bool {
            type CallRet = bool;
            type CallParams = (u64,);
            let args = (object,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(560usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsServer2DExtension", "body_test_motion_is_excluding_object", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PhysicsServer2DExtension {
        type Base = crate::classes::PhysicsServer2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PhysicsServer2DExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsServer2DExtension {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PhysicsServer2D > for PhysicsServer2DExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsServer2DExtension {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicsServer2DExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicsServer2DExtension {
        type Target = crate::classes::PhysicsServer2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsServer2DExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PhysicsServer2DExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PhysicsServer2DExtension__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsServer2DExtension > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsServer2D > for $Class {
                
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
    use super::re_export::PhysicsServer2DExtension;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for PhysicsServer2DExtension {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfObject < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}