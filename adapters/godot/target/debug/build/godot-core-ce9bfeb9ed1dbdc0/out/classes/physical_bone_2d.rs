#![doc = "Sidecar module for class [`PhysicalBone2D`][crate::classes::PhysicalBone2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicalBone2D` enums](https://docs.godotengine.org/en/stable/classes/class_physicalbone2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicalBone2D.`\n\nInherits [`RigidBody2D`][crate::classes::RigidBody2D].\n\nRelated symbols:\n\n* [`IPhysicalBone2D`][crate::classes::IPhysicalBone2D]: virtual methods\n\n\nSee also [Godot docs for `PhysicalBone2D`](https://docs.godotengine.org/en/stable/classes/class_physicalbone2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`PhysicalBone2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicalBone2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`PhysicalBone2D`][crate::classes::PhysicalBone2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRigidBody2D`][crate::classes::IRigidBody2D] > ~~`IPhysicsBody2D`~~ > ~~`ICollisionObject2D`~~ > [`INode2D`][crate::classes::INode2D] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `PhysicalBone2D` methods](https://docs.godotengine.org/en/stable/classes/class_physicalbone2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPhysicalBone2D: crate::obj::GodotClass < Base = PhysicalBone2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PhysicalBone2D {
        pub fn get_joint(&self,) -> Option < Gd < crate::classes::Joint2D > > {
            type CallRet = Option < Gd < crate::classes::Joint2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone2D", "get_joint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_auto_configure_joint(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone2D", "get_auto_configure_joint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_configure_joint(&mut self, auto_configure_joint: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (auto_configure_joint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone2D", "set_auto_configure_joint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_simulate_physics(&mut self, simulate_physics: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (simulate_physics,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone2D", "set_simulate_physics", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_simulate_physics(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone2D", "get_simulate_physics", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_simulating_physics(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone2D", "is_simulating_physics", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bone2d_nodepath(&mut self, nodepath: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (nodepath.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone2D", "set_bone2d_nodepath", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bone2d_nodepath(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone2D", "get_bone2d_nodepath", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bone2d_index(&mut self, bone_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (bone_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone2D", "set_bone2d_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bone2d_index(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone2D", "get_bone2d_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_follow_bone_when_simulating(&mut self, follow_bone: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (follow_bone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone2D", "set_follow_bone_when_simulating", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_follow_bone_when_simulating(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicalBone2D", "get_follow_bone_when_simulating", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PhysicalBone2D {
        type Base = crate::classes::RigidBody2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PhysicalBone2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicalBone2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RigidBody2D > for PhysicalBone2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PhysicsBody2D > for PhysicalBone2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject2D > for PhysicalBone2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for PhysicalBone2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for PhysicalBone2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for PhysicalBone2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicalBone2D {
        
    }
    impl crate::obj::cap::GodotDefault for PhysicalBone2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PhysicalBone2D {
        type Target = crate::classes::RigidBody2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicalBone2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PhysicalBone2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PhysicalBone2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicalBone2D > for $Class {
                
            }
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PhysicalBone2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::rigid_body_2d::SignalsOfRigidBody2D;
    impl WithSignals for PhysicalBone2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfRigidBody2D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}