#![doc = "Sidecar module for class [`PrimitiveMesh`][crate::classes::PrimitiveMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PrimitiveMesh` enums](https://docs.godotengine.org/en/stable/classes/class_primitivemesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PrimitiveMesh.`\n\nInherits [`Mesh`][crate::classes::Mesh].\n\nRelated symbols:\n\n* [`IPrimitiveMesh`][crate::classes::IPrimitiveMesh]: virtual methods\n\n\nSee also [Godot docs for `PrimitiveMesh`](https://docs.godotengine.org/en/stable/classes/class_primitivemesh.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`PrimitiveMesh::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PrimitiveMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`PrimitiveMesh`][crate::classes::PrimitiveMesh].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IMesh`][crate::classes::IMesh] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `PrimitiveMesh` methods](https://docs.godotengine.org/en/stable/classes/class_primitivemesh.html#methods)."]
    #[doc = "\n\n# Changes from base interface traits\nThe following virtual methods originally declared in direct/indirect base classes have their presence (required/optional) changed in `IPrimitiveMesh`. This can happen if Godot already overrides virtual methods, discouraging the user from further overriding them.\n\n\n| Base method | Original | New |\n| --- | --- | --- |\n| [`IMesh::get_surface_count`](crate::classes::IMesh::get_surface_count) | required | optional |\n| [`IMesh::surface_get_array_len`](crate::classes::IMesh::surface_get_array_len) | required | optional |\n| [`IMesh::surface_get_array_index_len`](crate::classes::IMesh::surface_get_array_index_len) | required | optional |\n| [`IMesh::surface_get_arrays`](crate::classes::IMesh::surface_get_arrays) | required | optional |\n| [`IMesh::surface_get_blend_shape_arrays`](crate::classes::IMesh::surface_get_blend_shape_arrays) | required | optional |\n| [`IMesh::surface_get_lods`](crate::classes::IMesh::surface_get_lods) | required | optional |\n| [`IMesh::surface_get_format`](crate::classes::IMesh::surface_get_format) | required | optional |\n| [`IMesh::surface_get_primitive_type`](crate::classes::IMesh::surface_get_primitive_type) | required | optional |\n| [`IMesh::surface_set_material`](crate::classes::IMesh::surface_set_material) | required | optional |\n| [`IMesh::surface_get_material`](crate::classes::IMesh::surface_get_material) | required | optional |\n| [`IMesh::get_blend_shape_count`](crate::classes::IMesh::get_blend_shape_count) | required | optional |\n| [`IMesh::get_blend_shape_name`](crate::classes::IMesh::get_blend_shape_name) | required | optional |\n| [`IMesh::set_blend_shape_name`](crate::classes::IMesh::set_blend_shape_name) | required | optional |\n| [`IMesh::get_aabb`](crate::classes::IMesh::get_aabb) | required | optional |"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPrimitiveMesh: crate::obj::GodotClass < Base = PrimitiveMesh > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn create_mesh_array(&self,) -> VarArray {
            unimplemented !()
        }
        fn get_surface_count(&self,) -> i32 {
            unimplemented !()
        }
        fn surface_get_array_len(&self, index: i32,) -> i32 {
            unimplemented !()
        }
        fn surface_get_array_index_len(&self, index: i32,) -> i32 {
            unimplemented !()
        }
        fn surface_get_arrays(&self, index: i32,) -> VarArray {
            unimplemented !()
        }
        fn surface_get_blend_shape_arrays(&self, index: i32,) -> Array < VarArray > {
            unimplemented !()
        }
        fn surface_get_lods(&self, index: i32,) -> VarDictionary {
            unimplemented !()
        }
        fn surface_get_format(&self, index: i32,) -> u32 {
            unimplemented !()
        }
        fn surface_get_primitive_type(&self, index: i32,) -> u32 {
            unimplemented !()
        }
        fn surface_set_material(&mut self, index: i32, material: Option < Gd < crate::classes::Material > >,) {
            unimplemented !()
        }
        fn surface_get_material(&self, index: i32,) -> Option < Gd < crate::classes::Material > > {
            unimplemented !()
        }
        fn get_blend_shape_count(&self,) -> i32 {
            unimplemented !()
        }
        fn get_blend_shape_name(&self, index: i32,) -> StringName {
            unimplemented !()
        }
        fn set_blend_shape_name(&mut self, index: i32, name: StringName,) {
            unimplemented !()
        }
        fn get_aabb(&self,) -> Aabb {
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
    impl PrimitiveMesh {
        pub fn set_material(&mut self, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6836usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PrimitiveMesh", "set_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_material(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6837usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PrimitiveMesh", "get_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mesh_arrays(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6838usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PrimitiveMesh", "get_mesh_arrays", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_aabb(&mut self, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Aabb,);
            let args = (aabb,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6839usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PrimitiveMesh", "set_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_aabb(&self,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6840usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PrimitiveMesh", "get_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flip_faces(&mut self, flip_faces: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (flip_faces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6841usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PrimitiveMesh", "set_flip_faces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_flip_faces(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6842usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PrimitiveMesh", "get_flip_faces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_add_uv2(&mut self, add_uv2: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (add_uv2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6843usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PrimitiveMesh", "set_add_uv2", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_add_uv2(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6844usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PrimitiveMesh", "get_add_uv2", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_uv2_padding(&mut self, uv2_padding: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (uv2_padding,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6845usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PrimitiveMesh", "set_uv2_padding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_uv2_padding(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6846usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PrimitiveMesh", "get_uv2_padding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn request_update(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6847usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PrimitiveMesh", "request_update", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PrimitiveMesh {
        type Base = crate::classes::Mesh;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PrimitiveMesh"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PrimitiveMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Mesh > for PrimitiveMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for PrimitiveMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for PrimitiveMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PrimitiveMesh {
        
    }
    impl crate::obj::cap::GodotDefault for PrimitiveMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PrimitiveMesh {
        type Target = crate::classes::Mesh;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PrimitiveMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PrimitiveMesh`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PrimitiveMesh__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PrimitiveMesh > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Mesh > for $Class {
                
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PrimitiveMesh;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for PrimitiveMesh {
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