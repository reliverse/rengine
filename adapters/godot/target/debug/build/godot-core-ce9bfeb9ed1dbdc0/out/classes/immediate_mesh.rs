#![doc = "Sidecar module for class [`ImmediateMesh`][crate::classes::ImmediateMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ImmediateMesh` enums](https://docs.godotengine.org/en/stable/classes/class_immediatemesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ImmediateMesh.`\n\nInherits [`Mesh`][crate::classes::Mesh].\n\nRelated symbols:\n\n* [`immediate_mesh`][crate::classes::immediate_mesh]: sidecar module with related enum/flag types\n* [`IImmediateMesh`][crate::classes::IImmediateMesh]: virtual methods\n\n\nSee also [Godot docs for `ImmediateMesh`](https://docs.godotengine.org/en/stable/classes/class_immediatemesh.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ImmediateMesh::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ImmediateMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ImmediateMesh`][crate::classes::ImmediateMesh].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IMesh`][crate::classes::IMesh] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `ImmediateMesh` methods](https://docs.godotengine.org/en/stable/classes/class_immediatemesh.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IImmediateMesh: crate::obj::GodotClass < Base = ImmediateMesh > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_surface_count(&self,) -> i32;
        fn surface_get_array_len(&self, index: i32,) -> i32;
        fn surface_get_array_index_len(&self, index: i32,) -> i32;
        fn surface_get_arrays(&self, index: i32,) -> VarArray;
        fn surface_get_blend_shape_arrays(&self, index: i32,) -> Array < VarArray >;
        fn surface_get_lods(&self, index: i32,) -> VarDictionary;
        fn surface_get_format(&self, index: i32,) -> u32;
        fn surface_get_primitive_type(&self, index: i32,) -> u32;
        fn surface_set_material(&mut self, index: i32, material: Option < Gd < crate::classes::Material > >,);
        fn surface_get_material(&self, index: i32,) -> Option < Gd < crate::classes::Material > >;
        fn get_blend_shape_count(&self,) -> i32;
        fn get_blend_shape_name(&self, index: i32,) -> StringName;
        fn set_blend_shape_name(&mut self, index: i32, name: StringName,);
        fn get_aabb(&self,) -> Aabb;
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
    impl ImmediateMesh {
        pub(crate) fn surface_begin_full(&mut self, primitive: crate::classes::mesh::PrimitiveType, material: CowArg < Option < Gd < crate::classes::Material > > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::mesh::PrimitiveType, CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (primitive, material,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImmediateMesh", "surface_begin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::surface_begin_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn surface_begin(&mut self, primitive: crate::classes::mesh::PrimitiveType,) {
            self.surface_begin_ex(primitive,) . done()
        }
        #[inline]
        pub fn surface_begin_ex < 'a > (&'a mut self, primitive: crate::classes::mesh::PrimitiveType,) -> ExSurfaceBegin < 'a > {
            ExSurfaceBegin::new(self, primitive,)
        }
        pub fn surface_set_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImmediateMesh", "surface_set_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_set_normal(&mut self, normal: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (normal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImmediateMesh", "surface_set_normal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_set_tangent(&mut self, tangent: Plane,) {
            type CallRet = ();
            type CallParams = (Plane,);
            let args = (tangent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImmediateMesh", "surface_set_tangent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_set_uv(&mut self, uv: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImmediateMesh", "surface_set_uv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_set_uv2(&mut self, uv2: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (uv2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImmediateMesh", "surface_set_uv2", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_add_vertex(&mut self, vertex: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImmediateMesh", "surface_add_vertex", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_add_vertex_2d(&mut self, vertex: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImmediateMesh", "surface_add_vertex_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_end(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImmediateMesh", "surface_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_surfaces(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ImmediateMesh", "clear_surfaces", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ImmediateMesh {
        type Base = crate::classes::Mesh;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ImmediateMesh"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ImmediateMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Mesh > for ImmediateMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for ImmediateMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ImmediateMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ImmediateMesh {
        
    }
    impl crate::obj::cap::GodotDefault for ImmediateMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ImmediateMesh {
        type Target = crate::classes::Mesh;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ImmediateMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ImmediateMesh`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ImmediateMesh__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ImmediateMesh > for $Class {
                
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
#[doc = "Default-param extender for [`ImmediateMesh::surface_begin_ex`][super::ImmediateMesh::surface_begin_ex]."]
#[must_use]
pub struct ExSurfaceBegin < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ImmediateMesh, primitive: crate::classes::mesh::PrimitiveType, material: CowArg < 'a, Option < Gd < crate::classes::Material > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSurfaceBegin < 'a > {
    fn new(surround_object: &'a mut re_export::ImmediateMesh, primitive: crate::classes::mesh::PrimitiveType,) -> Self {
        let material = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, primitive: primitive, material: material.into_arg(),
        }
    }
    #[inline]
    pub fn material(self, material: impl AsArg < Option < Gd < crate::classes::Material >> > + 'a) -> Self {
        Self {
            material: material.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, primitive, material,
        }
        = self;
        re_export::ImmediateMesh::surface_begin_full(surround_object, primitive, material,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ImmediateMesh;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for ImmediateMesh {
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