#![doc = "Sidecar module for class [`Generic6DofJoint3D`][crate::classes::Generic6DofJoint3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Generic6DOFJoint3D` enums](https://docs.godotengine.org/en/stable/classes/class_generic6dofjoint3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Generic6DOFJoint3D.`\n\nInherits [`Joint3D`][crate::classes::Joint3D].\n\nRelated symbols:\n\n* [`generic6_dof_joint_3d`][crate::classes::generic6_dof_joint_3d]: sidecar module with related enum/flag types\n* [`IGeneric6DofJoint3D`][crate::classes::IGeneric6DofJoint3D]: virtual methods\n\n\nSee also [Godot docs for `Generic6DOFJoint3D`](https://docs.godotengine.org/en/stable/classes/class_generic6dofjoint3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Generic6DofJoint3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Generic6DofJoint3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Generic6DofJoint3D`][crate::classes::Generic6DofJoint3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IJoint3D`~~ > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Generic6DOFJoint3D` methods](https://docs.godotengine.org/en/stable/classes/class_generic6dofjoint3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGeneric6DofJoint3D: crate::obj::GodotClass < Base = Generic6DofJoint3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Generic6DofJoint3D {
        pub fn set_param_x(&mut self, param: crate::classes::generic6_dof_joint_3d::Param, value: f32,) {
            type CallRet = ();
            type CallParams = (crate::classes::generic6_dof_joint_3d::Param, f32,);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3997usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Generic6DofJoint3D", "set_param_x", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_param_x(&self, param: crate::classes::generic6_dof_joint_3d::Param,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::classes::generic6_dof_joint_3d::Param,);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3998usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Generic6DofJoint3D", "get_param_x", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_param_y(&mut self, param: crate::classes::generic6_dof_joint_3d::Param, value: f32,) {
            type CallRet = ();
            type CallParams = (crate::classes::generic6_dof_joint_3d::Param, f32,);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3999usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Generic6DofJoint3D", "set_param_y", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_param_y(&self, param: crate::classes::generic6_dof_joint_3d::Param,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::classes::generic6_dof_joint_3d::Param,);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4000usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Generic6DofJoint3D", "get_param_y", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_param_z(&mut self, param: crate::classes::generic6_dof_joint_3d::Param, value: f32,) {
            type CallRet = ();
            type CallParams = (crate::classes::generic6_dof_joint_3d::Param, f32,);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4001usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Generic6DofJoint3D", "set_param_z", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_param_z(&self, param: crate::classes::generic6_dof_joint_3d::Param,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::classes::generic6_dof_joint_3d::Param,);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4002usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Generic6DofJoint3D", "get_param_z", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flag_x(&mut self, flag: crate::classes::generic6_dof_joint_3d::Flag, value: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::generic6_dof_joint_3d::Flag, bool,);
            let args = (flag, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4003usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Generic6DofJoint3D", "set_flag_x", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_flag_x(&self, flag: crate::classes::generic6_dof_joint_3d::Flag,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::generic6_dof_joint_3d::Flag,);
            let args = (flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4004usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Generic6DofJoint3D", "get_flag_x", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flag_y(&mut self, flag: crate::classes::generic6_dof_joint_3d::Flag, value: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::generic6_dof_joint_3d::Flag, bool,);
            let args = (flag, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4005usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Generic6DofJoint3D", "set_flag_y", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_flag_y(&self, flag: crate::classes::generic6_dof_joint_3d::Flag,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::generic6_dof_joint_3d::Flag,);
            let args = (flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4006usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Generic6DofJoint3D", "get_flag_y", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flag_z(&mut self, flag: crate::classes::generic6_dof_joint_3d::Flag, value: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::generic6_dof_joint_3d::Flag, bool,);
            let args = (flag, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4007usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Generic6DofJoint3D", "set_flag_z", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_flag_z(&self, flag: crate::classes::generic6_dof_joint_3d::Flag,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::generic6_dof_joint_3d::Flag,);
            let args = (flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4008usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Generic6DofJoint3D", "get_flag_z", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Generic6DofJoint3D {
        type Base = crate::classes::Joint3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Generic6DOFJoint3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Generic6DofJoint3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Joint3D > for Generic6DofJoint3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for Generic6DofJoint3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Generic6DofJoint3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Generic6DofJoint3D {
        
    }
    impl crate::obj::cap::GodotDefault for Generic6DofJoint3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Generic6DofJoint3D {
        type Target = crate::classes::Joint3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Generic6DofJoint3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Generic6DofJoint3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Generic6DofJoint3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Generic6DofJoint3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Joint3D > for $Class {
                
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
pub struct Param {
    ord: i32
}
impl Param {
    #[doc(alias = "PARAM_LINEAR_LOWER_LIMIT")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_LOWER_LIMIT`"]
    pub const LINEAR_LOWER_LIMIT: Param = Param {
        ord: 0i32
    };
    #[doc(alias = "PARAM_LINEAR_UPPER_LIMIT")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_UPPER_LIMIT`"]
    pub const LINEAR_UPPER_LIMIT: Param = Param {
        ord: 1i32
    };
    #[doc(alias = "PARAM_LINEAR_LIMIT_SOFTNESS")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_LIMIT_SOFTNESS`"]
    pub const LINEAR_LIMIT_SOFTNESS: Param = Param {
        ord: 2i32
    };
    #[doc(alias = "PARAM_LINEAR_RESTITUTION")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_RESTITUTION`"]
    pub const LINEAR_RESTITUTION: Param = Param {
        ord: 3i32
    };
    #[doc(alias = "PARAM_LINEAR_DAMPING")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_DAMPING`"]
    pub const LINEAR_DAMPING: Param = Param {
        ord: 4i32
    };
    #[doc(alias = "PARAM_LINEAR_MOTOR_TARGET_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_MOTOR_TARGET_VELOCITY`"]
    pub const LINEAR_MOTOR_TARGET_VELOCITY: Param = Param {
        ord: 5i32
    };
    #[doc(alias = "PARAM_LINEAR_MOTOR_FORCE_LIMIT")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_MOTOR_FORCE_LIMIT`"]
    pub const LINEAR_MOTOR_FORCE_LIMIT: Param = Param {
        ord: 6i32
    };
    #[doc(alias = "PARAM_LINEAR_SPRING_STIFFNESS")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_SPRING_STIFFNESS`"]
    pub const LINEAR_SPRING_STIFFNESS: Param = Param {
        ord: 7i32
    };
    #[doc(alias = "PARAM_LINEAR_SPRING_DAMPING")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_SPRING_DAMPING`"]
    pub const LINEAR_SPRING_DAMPING: Param = Param {
        ord: 8i32
    };
    #[doc(alias = "PARAM_LINEAR_SPRING_EQUILIBRIUM_POINT")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_SPRING_EQUILIBRIUM_POINT`"]
    pub const LINEAR_SPRING_EQUILIBRIUM_POINT: Param = Param {
        ord: 9i32
    };
    #[doc(alias = "PARAM_ANGULAR_LOWER_LIMIT")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_LOWER_LIMIT`"]
    pub const ANGULAR_LOWER_LIMIT: Param = Param {
        ord: 10i32
    };
    #[doc(alias = "PARAM_ANGULAR_UPPER_LIMIT")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_UPPER_LIMIT`"]
    pub const ANGULAR_UPPER_LIMIT: Param = Param {
        ord: 11i32
    };
    #[doc(alias = "PARAM_ANGULAR_LIMIT_SOFTNESS")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_LIMIT_SOFTNESS`"]
    pub const ANGULAR_LIMIT_SOFTNESS: Param = Param {
        ord: 12i32
    };
    #[doc(alias = "PARAM_ANGULAR_DAMPING")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_DAMPING`"]
    pub const ANGULAR_DAMPING: Param = Param {
        ord: 13i32
    };
    #[doc(alias = "PARAM_ANGULAR_RESTITUTION")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_RESTITUTION`"]
    pub const ANGULAR_RESTITUTION: Param = Param {
        ord: 14i32
    };
    #[doc(alias = "PARAM_ANGULAR_FORCE_LIMIT")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_FORCE_LIMIT`"]
    pub const ANGULAR_FORCE_LIMIT: Param = Param {
        ord: 15i32
    };
    #[doc(alias = "PARAM_ANGULAR_ERP")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_ERP`"]
    pub const ANGULAR_ERP: Param = Param {
        ord: 16i32
    };
    #[doc(alias = "PARAM_ANGULAR_MOTOR_TARGET_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_MOTOR_TARGET_VELOCITY`"]
    pub const ANGULAR_MOTOR_TARGET_VELOCITY: Param = Param {
        ord: 17i32
    };
    #[doc(alias = "PARAM_ANGULAR_MOTOR_FORCE_LIMIT")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_MOTOR_FORCE_LIMIT`"]
    pub const ANGULAR_MOTOR_FORCE_LIMIT: Param = Param {
        ord: 18i32
    };
    #[doc(alias = "PARAM_ANGULAR_SPRING_STIFFNESS")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_SPRING_STIFFNESS`"]
    pub const ANGULAR_SPRING_STIFFNESS: Param = Param {
        ord: 19i32
    };
    #[doc(alias = "PARAM_ANGULAR_SPRING_DAMPING")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_SPRING_DAMPING`"]
    pub const ANGULAR_SPRING_DAMPING: Param = Param {
        ord: 20i32
    };
    #[doc(alias = "PARAM_ANGULAR_SPRING_EQUILIBRIUM_POINT")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_SPRING_EQUILIBRIUM_POINT`"]
    pub const ANGULAR_SPRING_EQUILIBRIUM_POINT: Param = Param {
        ord: 21i32
    };
    #[doc(alias = "PARAM_MAX")]
    #[doc = "Godot enumerator name: `PARAM_MAX`"]
    pub const MAX: Param = Param {
        ord: 22i32
    };
    
}
impl std::fmt::Debug for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Param") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Param {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 => Some(Self {
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
            Self::LINEAR_LOWER_LIMIT => "LINEAR_LOWER_LIMIT", Self::LINEAR_UPPER_LIMIT => "LINEAR_UPPER_LIMIT", Self::LINEAR_LIMIT_SOFTNESS => "LINEAR_LIMIT_SOFTNESS", Self::LINEAR_RESTITUTION => "LINEAR_RESTITUTION", Self::LINEAR_DAMPING => "LINEAR_DAMPING", Self::LINEAR_MOTOR_TARGET_VELOCITY => "LINEAR_MOTOR_TARGET_VELOCITY", Self::LINEAR_MOTOR_FORCE_LIMIT => "LINEAR_MOTOR_FORCE_LIMIT", Self::LINEAR_SPRING_STIFFNESS => "LINEAR_SPRING_STIFFNESS", Self::LINEAR_SPRING_DAMPING => "LINEAR_SPRING_DAMPING", Self::LINEAR_SPRING_EQUILIBRIUM_POINT => "LINEAR_SPRING_EQUILIBRIUM_POINT", Self::ANGULAR_LOWER_LIMIT => "ANGULAR_LOWER_LIMIT", Self::ANGULAR_UPPER_LIMIT => "ANGULAR_UPPER_LIMIT", Self::ANGULAR_LIMIT_SOFTNESS => "ANGULAR_LIMIT_SOFTNESS", Self::ANGULAR_DAMPING => "ANGULAR_DAMPING", Self::ANGULAR_RESTITUTION => "ANGULAR_RESTITUTION", Self::ANGULAR_FORCE_LIMIT => "ANGULAR_FORCE_LIMIT", Self::ANGULAR_ERP => "ANGULAR_ERP", Self::ANGULAR_MOTOR_TARGET_VELOCITY => "ANGULAR_MOTOR_TARGET_VELOCITY", Self::ANGULAR_MOTOR_FORCE_LIMIT => "ANGULAR_MOTOR_FORCE_LIMIT", Self::ANGULAR_SPRING_STIFFNESS => "ANGULAR_SPRING_STIFFNESS", Self::ANGULAR_SPRING_DAMPING => "ANGULAR_SPRING_DAMPING", Self::ANGULAR_SPRING_EQUILIBRIUM_POINT => "ANGULAR_SPRING_EQUILIBRIUM_POINT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Param::LINEAR_LOWER_LIMIT, Param::LINEAR_UPPER_LIMIT, Param::LINEAR_LIMIT_SOFTNESS, Param::LINEAR_RESTITUTION, Param::LINEAR_DAMPING, Param::LINEAR_MOTOR_TARGET_VELOCITY, Param::LINEAR_MOTOR_FORCE_LIMIT, Param::LINEAR_SPRING_STIFFNESS, Param::LINEAR_SPRING_DAMPING, Param::LINEAR_SPRING_EQUILIBRIUM_POINT, Param::ANGULAR_LOWER_LIMIT, Param::ANGULAR_UPPER_LIMIT, Param::ANGULAR_LIMIT_SOFTNESS, Param::ANGULAR_DAMPING, Param::ANGULAR_RESTITUTION, Param::ANGULAR_FORCE_LIMIT, Param::ANGULAR_ERP, Param::ANGULAR_MOTOR_TARGET_VELOCITY, Param::ANGULAR_MOTOR_FORCE_LIMIT, Param::ANGULAR_SPRING_STIFFNESS, Param::ANGULAR_SPRING_DAMPING, Param::ANGULAR_SPRING_EQUILIBRIUM_POINT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Param >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LINEAR_LOWER_LIMIT", "PARAM_LINEAR_LOWER_LIMIT", Param::LINEAR_LOWER_LIMIT), crate::meta::inspect::EnumConstant::new("LINEAR_UPPER_LIMIT", "PARAM_LINEAR_UPPER_LIMIT", Param::LINEAR_UPPER_LIMIT), crate::meta::inspect::EnumConstant::new("LINEAR_LIMIT_SOFTNESS", "PARAM_LINEAR_LIMIT_SOFTNESS", Param::LINEAR_LIMIT_SOFTNESS), crate::meta::inspect::EnumConstant::new("LINEAR_RESTITUTION", "PARAM_LINEAR_RESTITUTION", Param::LINEAR_RESTITUTION), crate::meta::inspect::EnumConstant::new("LINEAR_DAMPING", "PARAM_LINEAR_DAMPING", Param::LINEAR_DAMPING), crate::meta::inspect::EnumConstant::new("LINEAR_MOTOR_TARGET_VELOCITY", "PARAM_LINEAR_MOTOR_TARGET_VELOCITY", Param::LINEAR_MOTOR_TARGET_VELOCITY), crate::meta::inspect::EnumConstant::new("LINEAR_MOTOR_FORCE_LIMIT", "PARAM_LINEAR_MOTOR_FORCE_LIMIT", Param::LINEAR_MOTOR_FORCE_LIMIT), crate::meta::inspect::EnumConstant::new("LINEAR_SPRING_STIFFNESS", "PARAM_LINEAR_SPRING_STIFFNESS", Param::LINEAR_SPRING_STIFFNESS), crate::meta::inspect::EnumConstant::new("LINEAR_SPRING_DAMPING", "PARAM_LINEAR_SPRING_DAMPING", Param::LINEAR_SPRING_DAMPING), crate::meta::inspect::EnumConstant::new("LINEAR_SPRING_EQUILIBRIUM_POINT", "PARAM_LINEAR_SPRING_EQUILIBRIUM_POINT", Param::LINEAR_SPRING_EQUILIBRIUM_POINT), crate::meta::inspect::EnumConstant::new("ANGULAR_LOWER_LIMIT", "PARAM_ANGULAR_LOWER_LIMIT", Param::ANGULAR_LOWER_LIMIT), crate::meta::inspect::EnumConstant::new("ANGULAR_UPPER_LIMIT", "PARAM_ANGULAR_UPPER_LIMIT", Param::ANGULAR_UPPER_LIMIT), crate::meta::inspect::EnumConstant::new("ANGULAR_LIMIT_SOFTNESS", "PARAM_ANGULAR_LIMIT_SOFTNESS", Param::ANGULAR_LIMIT_SOFTNESS), crate::meta::inspect::EnumConstant::new("ANGULAR_DAMPING", "PARAM_ANGULAR_DAMPING", Param::ANGULAR_DAMPING), crate::meta::inspect::EnumConstant::new("ANGULAR_RESTITUTION", "PARAM_ANGULAR_RESTITUTION", Param::ANGULAR_RESTITUTION), crate::meta::inspect::EnumConstant::new("ANGULAR_FORCE_LIMIT", "PARAM_ANGULAR_FORCE_LIMIT", Param::ANGULAR_FORCE_LIMIT), crate::meta::inspect::EnumConstant::new("ANGULAR_ERP", "PARAM_ANGULAR_ERP", Param::ANGULAR_ERP), crate::meta::inspect::EnumConstant::new("ANGULAR_MOTOR_TARGET_VELOCITY", "PARAM_ANGULAR_MOTOR_TARGET_VELOCITY", Param::ANGULAR_MOTOR_TARGET_VELOCITY), crate::meta::inspect::EnumConstant::new("ANGULAR_MOTOR_FORCE_LIMIT", "PARAM_ANGULAR_MOTOR_FORCE_LIMIT", Param::ANGULAR_MOTOR_FORCE_LIMIT), crate::meta::inspect::EnumConstant::new("ANGULAR_SPRING_STIFFNESS", "PARAM_ANGULAR_SPRING_STIFFNESS", Param::ANGULAR_SPRING_STIFFNESS), crate::meta::inspect::EnumConstant::new("ANGULAR_SPRING_DAMPING", "PARAM_ANGULAR_SPRING_DAMPING", Param::ANGULAR_SPRING_DAMPING), crate::meta::inspect::EnumConstant::new("ANGULAR_SPRING_EQUILIBRIUM_POINT", "PARAM_ANGULAR_SPRING_EQUILIBRIUM_POINT", Param::ANGULAR_SPRING_EQUILIBRIUM_POINT), crate::meta::inspect::EnumConstant::new("MAX", "PARAM_MAX", Param::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Param {
    const ENUMERATOR_COUNT: usize = 22usize;
    
}
impl crate::meta::GodotConvert for Param {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Param {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Param {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Flag {
    ord: i32
}
impl Flag {
    #[doc(alias = "FLAG_ENABLE_LINEAR_LIMIT")]
    #[doc = "Godot enumerator name: `FLAG_ENABLE_LINEAR_LIMIT`"]
    pub const ENABLE_LINEAR_LIMIT: Flag = Flag {
        ord: 0i32
    };
    #[doc(alias = "FLAG_ENABLE_ANGULAR_LIMIT")]
    #[doc = "Godot enumerator name: `FLAG_ENABLE_ANGULAR_LIMIT`"]
    pub const ENABLE_ANGULAR_LIMIT: Flag = Flag {
        ord: 1i32
    };
    #[doc(alias = "FLAG_ENABLE_LINEAR_SPRING")]
    #[doc = "Godot enumerator name: `FLAG_ENABLE_LINEAR_SPRING`"]
    pub const ENABLE_LINEAR_SPRING: Flag = Flag {
        ord: 3i32
    };
    #[doc(alias = "FLAG_ENABLE_ANGULAR_SPRING")]
    #[doc = "Godot enumerator name: `FLAG_ENABLE_ANGULAR_SPRING`"]
    pub const ENABLE_ANGULAR_SPRING: Flag = Flag {
        ord: 2i32
    };
    #[doc(alias = "FLAG_ENABLE_MOTOR")]
    #[doc = "Godot enumerator name: `FLAG_ENABLE_MOTOR`"]
    pub const ENABLE_MOTOR: Flag = Flag {
        ord: 4i32
    };
    #[doc(alias = "FLAG_ENABLE_LINEAR_MOTOR")]
    #[doc = "Godot enumerator name: `FLAG_ENABLE_LINEAR_MOTOR`"]
    pub const ENABLE_LINEAR_MOTOR: Flag = Flag {
        ord: 5i32
    };
    #[doc(alias = "FLAG_MAX")]
    #[doc = "Godot enumerator name: `FLAG_MAX`"]
    pub const MAX: Flag = Flag {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Flag") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Flag {
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
            Self::ENABLE_LINEAR_LIMIT => "ENABLE_LINEAR_LIMIT", Self::ENABLE_ANGULAR_LIMIT => "ENABLE_ANGULAR_LIMIT", Self::ENABLE_LINEAR_SPRING => "ENABLE_LINEAR_SPRING", Self::ENABLE_ANGULAR_SPRING => "ENABLE_ANGULAR_SPRING", Self::ENABLE_MOTOR => "ENABLE_MOTOR", Self::ENABLE_LINEAR_MOTOR => "ENABLE_LINEAR_MOTOR", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Flag::ENABLE_LINEAR_LIMIT, Flag::ENABLE_ANGULAR_LIMIT, Flag::ENABLE_LINEAR_SPRING, Flag::ENABLE_ANGULAR_SPRING, Flag::ENABLE_MOTOR, Flag::ENABLE_LINEAR_MOTOR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Flag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ENABLE_LINEAR_LIMIT", "FLAG_ENABLE_LINEAR_LIMIT", Flag::ENABLE_LINEAR_LIMIT), crate::meta::inspect::EnumConstant::new("ENABLE_ANGULAR_LIMIT", "FLAG_ENABLE_ANGULAR_LIMIT", Flag::ENABLE_ANGULAR_LIMIT), crate::meta::inspect::EnumConstant::new("ENABLE_LINEAR_SPRING", "FLAG_ENABLE_LINEAR_SPRING", Flag::ENABLE_LINEAR_SPRING), crate::meta::inspect::EnumConstant::new("ENABLE_ANGULAR_SPRING", "FLAG_ENABLE_ANGULAR_SPRING", Flag::ENABLE_ANGULAR_SPRING), crate::meta::inspect::EnumConstant::new("ENABLE_MOTOR", "FLAG_ENABLE_MOTOR", Flag::ENABLE_MOTOR), crate::meta::inspect::EnumConstant::new("ENABLE_LINEAR_MOTOR", "FLAG_ENABLE_LINEAR_MOTOR", Flag::ENABLE_LINEAR_MOTOR), crate::meta::inspect::EnumConstant::new("MAX", "FLAG_MAX", Flag::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Flag {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for Flag {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Flag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Flag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Generic6DofJoint3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for Generic6DofJoint3D {
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