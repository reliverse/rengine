#![doc = "Sidecar module for class [`MeshConvexDecompositionSettings`][crate::classes::MeshConvexDecompositionSettings].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MeshConvexDecompositionSettings` enums](https://docs.godotengine.org/en/stable/classes/class_meshconvexdecompositionsettings.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MeshConvexDecompositionSettings.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`mesh_convex_decomposition_settings`][crate::classes::mesh_convex_decomposition_settings]: sidecar module with related enum/flag types\n* [`IMeshConvexDecompositionSettings`][crate::classes::IMeshConvexDecompositionSettings]: virtual methods\n\n\nSee also [Godot docs for `MeshConvexDecompositionSettings`](https://docs.godotengine.org/en/stable/classes/class_meshconvexdecompositionsettings.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`MeshConvexDecompositionSettings::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MeshConvexDecompositionSettings {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`MeshConvexDecompositionSettings`][crate::classes::MeshConvexDecompositionSettings].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `MeshConvexDecompositionSettings` methods](https://docs.godotengine.org/en/stable/classes/class_meshconvexdecompositionsettings.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMeshConvexDecompositionSettings: crate::obj::GodotClass < Base = MeshConvexDecompositionSettings > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl MeshConvexDecompositionSettings {
        pub fn set_max_concavity(&mut self, max_concavity: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (max_concavity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "set_max_concavity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_concavity(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5280usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "get_max_concavity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_symmetry_planes_clipping_bias(&mut self, symmetry_planes_clipping_bias: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (symmetry_planes_clipping_bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5281usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "set_symmetry_planes_clipping_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_symmetry_planes_clipping_bias(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5282usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "get_symmetry_planes_clipping_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_revolution_axes_clipping_bias(&mut self, revolution_axes_clipping_bias: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (revolution_axes_clipping_bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5283usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "set_revolution_axes_clipping_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_revolution_axes_clipping_bias(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5284usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "get_revolution_axes_clipping_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_min_volume_per_convex_hull(&mut self, min_volume_per_convex_hull: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (min_volume_per_convex_hull,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5285usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "set_min_volume_per_convex_hull", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_min_volume_per_convex_hull(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5286usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "get_min_volume_per_convex_hull", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_resolution(&mut self, min_volume_per_convex_hull: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (min_volume_per_convex_hull,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5287usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "set_resolution", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_resolution(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5288usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "get_resolution", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_num_vertices_per_convex_hull(&mut self, max_num_vertices_per_convex_hull: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (max_num_vertices_per_convex_hull,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5289usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "set_max_num_vertices_per_convex_hull", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_num_vertices_per_convex_hull(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5290usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "get_max_num_vertices_per_convex_hull", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_plane_downsampling(&mut self, plane_downsampling: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (plane_downsampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5291usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "set_plane_downsampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_plane_downsampling(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5292usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "get_plane_downsampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_convex_hull_downsampling(&mut self, convex_hull_downsampling: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (convex_hull_downsampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5293usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "set_convex_hull_downsampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_convex_hull_downsampling(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5294usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "get_convex_hull_downsampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_normalize_mesh(&mut self, normalize_mesh: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (normalize_mesh,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5295usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "set_normalize_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_normalize_mesh(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5296usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "get_normalize_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mode(&mut self, mode: crate::classes::mesh_convex_decomposition_settings::Mode,) {
            type CallRet = ();
            type CallParams = (crate::classes::mesh_convex_decomposition_settings::Mode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5297usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "set_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mode(&self,) -> crate::classes::mesh_convex_decomposition_settings::Mode {
            type CallRet = crate::classes::mesh_convex_decomposition_settings::Mode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5298usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "get_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_convex_hull_approximation(&mut self, convex_hull_approximation: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (convex_hull_approximation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5299usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "set_convex_hull_approximation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_convex_hull_approximation(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5300usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "get_convex_hull_approximation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_convex_hulls(&mut self, max_convex_hulls: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (max_convex_hulls,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5301usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "set_max_convex_hulls", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_convex_hulls(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5302usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "get_max_convex_hulls", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_project_hull_vertices(&mut self, project_hull_vertices: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (project_hull_vertices,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5303usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "set_project_hull_vertices", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_project_hull_vertices(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5304usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshConvexDecompositionSettings", "get_project_hull_vertices", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for MeshConvexDecompositionSettings {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("MeshConvexDecompositionSettings"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MeshConvexDecompositionSettings {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for MeshConvexDecompositionSettings {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MeshConvexDecompositionSettings {
        
    }
    impl crate::obj::cap::GodotDefault for MeshConvexDecompositionSettings {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MeshConvexDecompositionSettings {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MeshConvexDecompositionSettings {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MeshConvexDecompositionSettings`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_MeshConvexDecompositionSettings__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MeshConvexDecompositionSettings > for $Class {
                
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
pub struct Mode {
    ord: i32
}
impl Mode {
    #[doc(alias = "CONVEX_DECOMPOSITION_MODE_VOXEL")]
    #[doc = "Godot enumerator name: `CONVEX_DECOMPOSITION_MODE_VOXEL`"]
    pub const VOXEL: Mode = Mode {
        ord: 0i32
    };
    #[doc(alias = "CONVEX_DECOMPOSITION_MODE_TETRAHEDRON")]
    #[doc = "Godot enumerator name: `CONVEX_DECOMPOSITION_MODE_TETRAHEDRON`"]
    pub const TETRAHEDRON: Mode = Mode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Mode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Mode {
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
            Self::VOXEL => "VOXEL", Self::TETRAHEDRON => "TETRAHEDRON", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Mode::VOXEL, Mode::TETRAHEDRON]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Mode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VOXEL", "CONVEX_DECOMPOSITION_MODE_VOXEL", Mode::VOXEL), crate::meta::inspect::EnumConstant::new("TETRAHEDRON", "CONVEX_DECOMPOSITION_MODE_TETRAHEDRON", Mode::TETRAHEDRON)]
        }
    }
}
impl crate::meta::GodotConvert for Mode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Mode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Mode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::MeshConvexDecompositionSettings;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for MeshConvexDecompositionSettings {
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