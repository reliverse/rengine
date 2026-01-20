#![doc = "Sidecar module for class [`TubeTrailMesh`][crate::classes::TubeTrailMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TubeTrailMesh` enums](https://docs.godotengine.org/en/stable/classes/class_tubetrailmesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TubeTrailMesh.`\n\nInherits [`PrimitiveMesh`][crate::classes::PrimitiveMesh].\n\nRelated symbols:\n\n* [`ITubeTrailMesh`][crate::classes::ITubeTrailMesh]: virtual methods\n\n\nSee also [Godot docs for `TubeTrailMesh`](https://docs.godotengine.org/en/stable/classes/class_tubetrailmesh.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TubeTrailMesh::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TubeTrailMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TubeTrailMesh`][crate::classes::TubeTrailMesh].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IPrimitiveMesh`][crate::classes::IPrimitiveMesh] > [`IMesh`][crate::classes::IMesh] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `TubeTrailMesh` methods](https://docs.godotengine.org/en/stable/classes/class_tubetrailmesh.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITubeTrailMesh: crate::obj::GodotClass < Base = TubeTrailMesh > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TubeTrailMesh {
        pub fn set_radius(&mut self, radius: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TubeTrailMesh", "set_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_radius(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TubeTrailMesh", "get_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_radial_steps(&mut self, radial_steps: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (radial_steps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TubeTrailMesh", "set_radial_steps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_radial_steps(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TubeTrailMesh", "get_radial_steps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sections(&mut self, sections: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (sections,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TubeTrailMesh", "set_sections", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sections(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TubeTrailMesh", "get_sections", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_section_length(&mut self, section_length: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (section_length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TubeTrailMesh", "set_section_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_section_length(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TubeTrailMesh", "get_section_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_section_rings(&mut self, section_rings: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (section_rings,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TubeTrailMesh", "set_section_rings", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_section_rings(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TubeTrailMesh", "get_section_rings", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cap_top(&mut self, cap_top: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (cap_top,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TubeTrailMesh", "set_cap_top", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_cap_top(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TubeTrailMesh", "is_cap_top", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cap_bottom(&mut self, cap_bottom: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (cap_bottom,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TubeTrailMesh", "set_cap_bottom", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_cap_bottom(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TubeTrailMesh", "is_cap_bottom", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_curve(&mut self, curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Curve > > >,);
            let args = (curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TubeTrailMesh", "set_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_curve(&self,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TubeTrailMesh", "get_curve", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TubeTrailMesh {
        type Base = crate::classes::PrimitiveMesh;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TubeTrailMesh"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TubeTrailMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PrimitiveMesh > for TubeTrailMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Mesh > for TubeTrailMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for TubeTrailMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TubeTrailMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TubeTrailMesh {
        
    }
    impl crate::obj::cap::GodotDefault for TubeTrailMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TubeTrailMesh {
        type Target = crate::classes::PrimitiveMesh;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TubeTrailMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TubeTrailMesh`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TubeTrailMesh__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TubeTrailMesh > for $Class {
                
            }
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
    use super::re_export::TubeTrailMesh;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for TubeTrailMesh {
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