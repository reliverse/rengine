#![doc = "Sidecar module for class [`VisualShader`][crate::classes::VisualShader].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShader` enums](https://docs.godotengine.org/en/stable/classes/class_visualshader.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShader.`\n\nInherits [`Shader`][crate::classes::Shader].\n\nRelated symbols:\n\n* [`visual_shader`][crate::classes::visual_shader]: sidecar module with related enum/flag types\n* [`IVisualShader`][crate::classes::IVisualShader]: virtual methods\n\n\nSee also [Godot docs for `VisualShader`](https://docs.godotengine.org/en/stable/classes/class_visualshader.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`VisualShader::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShader {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`VisualShader`][crate::classes::VisualShader].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IShader`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `VisualShader` methods](https://docs.godotengine.org/en/stable/classes/class_visualshader.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShader: crate::obj::GodotClass < Base = VisualShader > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
        fn get_rid(&self,) -> Rid {
            unimplemented !()
        }
        fn reset_state(&mut self,) {
            unimplemented !()
        }
        fn set_path_cache(&self, path: GString,) {
            unimplemented !()
        }
    }
    impl VisualShader {
        pub fn set_mode(&mut self, mode: crate::classes::shader::Mode,) {
            type CallRet = ();
            type CallParams = (crate::classes::shader::Mode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10593usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "set_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_node(&mut self, type_: crate::classes::visual_shader::Type, node: impl AsArg < Option < Gd < crate::classes::VisualShaderNode >> >, position: Vector2, id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::visual_shader::Type, CowArg < 'a0, Option < Gd < crate::classes::VisualShaderNode > > >, Vector2, i32,);
            let args = (type_, node.into_arg(), position, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10594usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "add_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node(&self, type_: crate::classes::visual_shader::Type, id: i32,) -> Option < Gd < crate::classes::VisualShaderNode > > {
            type CallRet = Option < Gd < crate::classes::VisualShaderNode > >;
            type CallParams = (crate::classes::visual_shader::Type, i32,);
            let args = (type_, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10595usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "get_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_node_position(&mut self, type_: crate::classes::visual_shader::Type, id: i32, position: Vector2,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader::Type, i32, Vector2,);
            let args = (type_, id, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10596usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "set_node_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_position(&self, type_: crate::classes::visual_shader::Type, id: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (crate::classes::visual_shader::Type, i32,);
            let args = (type_, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10597usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "get_node_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_list(&self, type_: crate::classes::visual_shader::Type,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (crate::classes::visual_shader::Type,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10598usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "get_node_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_valid_node_id(&self, type_: crate::classes::visual_shader::Type,) -> i32 {
            type CallRet = i32;
            type CallParams = (crate::classes::visual_shader::Type,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10599usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "get_valid_node_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_node(&mut self, type_: crate::classes::visual_shader::Type, id: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader::Type, i32,);
            let args = (type_, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10600usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "remove_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn replace_node(&mut self, type_: crate::classes::visual_shader::Type, id: i32, new_class: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::visual_shader::Type, i32, CowArg < 'a0, StringName >,);
            let args = (type_, id, new_class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10601usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "replace_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_node_connection(&self, type_: crate::classes::visual_shader::Type, from_node: i32, from_port: i32, to_node: i32, to_port: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::visual_shader::Type, i32, i32, i32, i32,);
            let args = (type_, from_node, from_port, to_node, to_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10602usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "is_node_connection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn can_connect_nodes(&self, type_: crate::classes::visual_shader::Type, from_node: i32, from_port: i32, to_node: i32, to_port: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::visual_shader::Type, i32, i32, i32, i32,);
            let args = (type_, from_node, from_port, to_node, to_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10603usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "can_connect_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn connect_nodes(&mut self, type_: crate::classes::visual_shader::Type, from_node: i32, from_port: i32, to_node: i32, to_port: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (crate::classes::visual_shader::Type, i32, i32, i32, i32,);
            let args = (type_, from_node, from_port, to_node, to_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10604usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "connect_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn disconnect_nodes(&mut self, type_: crate::classes::visual_shader::Type, from_node: i32, from_port: i32, to_node: i32, to_port: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader::Type, i32, i32, i32, i32,);
            let args = (type_, from_node, from_port, to_node, to_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10605usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "disconnect_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn connect_nodes_forced(&mut self, type_: crate::classes::visual_shader::Type, from_node: i32, from_port: i32, to_node: i32, to_port: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader::Type, i32, i32, i32, i32,);
            let args = (type_, from_node, from_port, to_node, to_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10606usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "connect_nodes_forced", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_connections(&self, type_: crate::classes::visual_shader::Type,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = (crate::classes::visual_shader::Type,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10607usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "get_node_connections", Some(self.__validated_obj()), args,)
            }
        }
        pub fn attach_node_to_frame(&mut self, type_: crate::classes::visual_shader::Type, id: i32, frame: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader::Type, i32, i32,);
            let args = (type_, id, frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10608usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "attach_node_to_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn detach_node_from_frame(&mut self, type_: crate::classes::visual_shader::Type, id: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader::Type, i32,);
            let args = (type_, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10609usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "detach_node_from_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_varying(&mut self, name: impl AsArg < GString >, mode: crate::classes::visual_shader::VaryingMode, type_: crate::classes::visual_shader::VaryingType,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::visual_shader::VaryingMode, crate::classes::visual_shader::VaryingType,);
            let args = (name.into_arg(), mode, type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10610usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "add_varying", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_varying(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10611usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "remove_varying", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_varying(&self, name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10612usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShader", "has_varying", Some(self.__validated_obj()), args,)
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
        pub const NODE_ID_INVALID: i32 = - 1i32;
        pub const NODE_ID_OUTPUT: i32 = 0i32;
        
    }
    impl crate::obj::GodotClass for VisualShader {
        type Base = crate::classes::Shader;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VisualShader"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShader {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Shader > for VisualShader {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShader {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShader {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShader {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShader {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShader {
        type Target = crate::classes::Shader;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShader {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VisualShader`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisualShader__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShader > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Shader > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Type {
    ord: i32
}
impl Type {
    #[doc(alias = "TYPE_VERTEX")]
    #[doc = "Godot enumerator name: `TYPE_VERTEX`"]
    pub const VERTEX: Type = Type {
        ord: 0i32
    };
    #[doc(alias = "TYPE_FRAGMENT")]
    #[doc = "Godot enumerator name: `TYPE_FRAGMENT`"]
    pub const FRAGMENT: Type = Type {
        ord: 1i32
    };
    #[doc(alias = "TYPE_LIGHT")]
    #[doc = "Godot enumerator name: `TYPE_LIGHT`"]
    pub const LIGHT: Type = Type {
        ord: 2i32
    };
    #[doc(alias = "TYPE_START")]
    #[doc = "Godot enumerator name: `TYPE_START`"]
    pub const START: Type = Type {
        ord: 3i32
    };
    #[doc(alias = "TYPE_PROCESS")]
    #[doc = "Godot enumerator name: `TYPE_PROCESS`"]
    pub const PROCESS: Type = Type {
        ord: 4i32
    };
    #[doc(alias = "TYPE_COLLIDE")]
    #[doc = "Godot enumerator name: `TYPE_COLLIDE`"]
    pub const COLLIDE: Type = Type {
        ord: 5i32
    };
    #[doc(alias = "TYPE_START_CUSTOM")]
    #[doc = "Godot enumerator name: `TYPE_START_CUSTOM`"]
    pub const START_CUSTOM: Type = Type {
        ord: 6i32
    };
    #[doc(alias = "TYPE_PROCESS_CUSTOM")]
    #[doc = "Godot enumerator name: `TYPE_PROCESS_CUSTOM`"]
    pub const PROCESS_CUSTOM: Type = Type {
        ord: 7i32
    };
    #[doc(alias = "TYPE_SKY")]
    #[doc = "Godot enumerator name: `TYPE_SKY`"]
    pub const SKY: Type = Type {
        ord: 8i32
    };
    #[doc(alias = "TYPE_FOG")]
    #[doc = "Godot enumerator name: `TYPE_FOG`"]
    pub const FOG: Type = Type {
        ord: 9i32
    };
    #[doc(alias = "TYPE_MAX")]
    #[doc = "Godot enumerator name: `TYPE_MAX`"]
    pub const MAX: Type = Type {
        ord: 10i32
    };
    
}
impl std::fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Type") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Type {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 => Some(Self {
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
            Self::VERTEX => "VERTEX", Self::FRAGMENT => "FRAGMENT", Self::LIGHT => "LIGHT", Self::START => "START", Self::PROCESS => "PROCESS", Self::COLLIDE => "COLLIDE", Self::START_CUSTOM => "START_CUSTOM", Self::PROCESS_CUSTOM => "PROCESS_CUSTOM", Self::SKY => "SKY", Self::FOG => "FOG", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Type::VERTEX, Type::FRAGMENT, Type::LIGHT, Type::START, Type::PROCESS, Type::COLLIDE, Type::START_CUSTOM, Type::PROCESS_CUSTOM, Type::SKY, Type::FOG]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Type >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERTEX", "TYPE_VERTEX", Type::VERTEX), crate::meta::inspect::EnumConstant::new("FRAGMENT", "TYPE_FRAGMENT", Type::FRAGMENT), crate::meta::inspect::EnumConstant::new("LIGHT", "TYPE_LIGHT", Type::LIGHT), crate::meta::inspect::EnumConstant::new("START", "TYPE_START", Type::START), crate::meta::inspect::EnumConstant::new("PROCESS", "TYPE_PROCESS", Type::PROCESS), crate::meta::inspect::EnumConstant::new("COLLIDE", "TYPE_COLLIDE", Type::COLLIDE), crate::meta::inspect::EnumConstant::new("START_CUSTOM", "TYPE_START_CUSTOM", Type::START_CUSTOM), crate::meta::inspect::EnumConstant::new("PROCESS_CUSTOM", "TYPE_PROCESS_CUSTOM", Type::PROCESS_CUSTOM), crate::meta::inspect::EnumConstant::new("SKY", "TYPE_SKY", Type::SKY), crate::meta::inspect::EnumConstant::new("FOG", "TYPE_FOG", Type::FOG), crate::meta::inspect::EnumConstant::new("MAX", "TYPE_MAX", Type::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Type {
    const ENUMERATOR_COUNT: usize = 10usize;
    
}
impl crate::meta::GodotConvert for Type {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Type {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Type {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VaryingMode {
    ord: i32
}
impl VaryingMode {
    #[doc(alias = "VARYING_MODE_VERTEX_TO_FRAG_LIGHT")]
    #[doc = "Godot enumerator name: `VARYING_MODE_VERTEX_TO_FRAG_LIGHT`"]
    pub const VERTEX_TO_FRAG_LIGHT: VaryingMode = VaryingMode {
        ord: 0i32
    };
    #[doc(alias = "VARYING_MODE_FRAG_TO_LIGHT")]
    #[doc = "Godot enumerator name: `VARYING_MODE_FRAG_TO_LIGHT`"]
    pub const FRAG_TO_LIGHT: VaryingMode = VaryingMode {
        ord: 1i32
    };
    #[doc(alias = "VARYING_MODE_MAX")]
    #[doc = "Godot enumerator name: `VARYING_MODE_MAX`"]
    pub const MAX: VaryingMode = VaryingMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for VaryingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VaryingMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VaryingMode {
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
            Self::VERTEX_TO_FRAG_LIGHT => "VERTEX_TO_FRAG_LIGHT", Self::FRAG_TO_LIGHT => "FRAG_TO_LIGHT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VaryingMode::VERTEX_TO_FRAG_LIGHT, VaryingMode::FRAG_TO_LIGHT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VaryingMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERTEX_TO_FRAG_LIGHT", "VARYING_MODE_VERTEX_TO_FRAG_LIGHT", VaryingMode::VERTEX_TO_FRAG_LIGHT), crate::meta::inspect::EnumConstant::new("FRAG_TO_LIGHT", "VARYING_MODE_FRAG_TO_LIGHT", VaryingMode::FRAG_TO_LIGHT), crate::meta::inspect::EnumConstant::new("MAX", "VARYING_MODE_MAX", VaryingMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for VaryingMode {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for VaryingMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VaryingMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VaryingMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VaryingType {
    ord: i32
}
impl VaryingType {
    #[doc(alias = "VARYING_TYPE_FLOAT")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_FLOAT`"]
    pub const FLOAT: VaryingType = VaryingType {
        ord: 0i32
    };
    #[doc(alias = "VARYING_TYPE_INT")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_INT`"]
    pub const INT: VaryingType = VaryingType {
        ord: 1i32
    };
    #[doc(alias = "VARYING_TYPE_UINT")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_UINT`"]
    pub const UINT: VaryingType = VaryingType {
        ord: 2i32
    };
    #[doc(alias = "VARYING_TYPE_VECTOR_2D")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_VECTOR_2D`"]
    pub const VECTOR_2D: VaryingType = VaryingType {
        ord: 3i32
    };
    #[doc(alias = "VARYING_TYPE_VECTOR_3D")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_VECTOR_3D`"]
    pub const VECTOR_3D: VaryingType = VaryingType {
        ord: 4i32
    };
    #[doc(alias = "VARYING_TYPE_VECTOR_4D")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_VECTOR_4D`"]
    pub const VECTOR_4D: VaryingType = VaryingType {
        ord: 5i32
    };
    #[doc(alias = "VARYING_TYPE_BOOLEAN")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_BOOLEAN`"]
    pub const BOOLEAN: VaryingType = VaryingType {
        ord: 6i32
    };
    #[doc(alias = "VARYING_TYPE_TRANSFORM")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_TRANSFORM`"]
    pub const TRANSFORM: VaryingType = VaryingType {
        ord: 7i32
    };
    #[doc(alias = "VARYING_TYPE_MAX")]
    #[doc = "Godot enumerator name: `VARYING_TYPE_MAX`"]
    pub const MAX: VaryingType = VaryingType {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for VaryingType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VaryingType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VaryingType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::FLOAT => "FLOAT", Self::INT => "INT", Self::UINT => "UINT", Self::VECTOR_2D => "VECTOR_2D", Self::VECTOR_3D => "VECTOR_3D", Self::VECTOR_4D => "VECTOR_4D", Self::BOOLEAN => "BOOLEAN", Self::TRANSFORM => "TRANSFORM", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VaryingType::FLOAT, VaryingType::INT, VaryingType::UINT, VaryingType::VECTOR_2D, VaryingType::VECTOR_3D, VaryingType::VECTOR_4D, VaryingType::BOOLEAN, VaryingType::TRANSFORM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VaryingType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FLOAT", "VARYING_TYPE_FLOAT", VaryingType::FLOAT), crate::meta::inspect::EnumConstant::new("INT", "VARYING_TYPE_INT", VaryingType::INT), crate::meta::inspect::EnumConstant::new("UINT", "VARYING_TYPE_UINT", VaryingType::UINT), crate::meta::inspect::EnumConstant::new("VECTOR_2D", "VARYING_TYPE_VECTOR_2D", VaryingType::VECTOR_2D), crate::meta::inspect::EnumConstant::new("VECTOR_3D", "VARYING_TYPE_VECTOR_3D", VaryingType::VECTOR_3D), crate::meta::inspect::EnumConstant::new("VECTOR_4D", "VARYING_TYPE_VECTOR_4D", VaryingType::VECTOR_4D), crate::meta::inspect::EnumConstant::new("BOOLEAN", "VARYING_TYPE_BOOLEAN", VaryingType::BOOLEAN), crate::meta::inspect::EnumConstant::new("TRANSFORM", "VARYING_TYPE_TRANSFORM", VaryingType::TRANSFORM), crate::meta::inspect::EnumConstant::new("MAX", "VARYING_TYPE_MAX", VaryingType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for VaryingType {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for VaryingType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VaryingType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VaryingType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VisualShader;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VisualShader {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfResource < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}