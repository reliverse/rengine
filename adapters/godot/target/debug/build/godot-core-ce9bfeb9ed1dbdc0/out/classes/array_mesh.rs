#![doc = "Sidecar module for class [`ArrayMesh`][crate::classes::ArrayMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ArrayMesh` enums](https://docs.godotengine.org/en/stable/classes/class_arraymesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ArrayMesh.`\n\nInherits [`Mesh`][crate::classes::Mesh].\n\nRelated symbols:\n\n* [`array_mesh`][crate::classes::array_mesh]: sidecar module with related enum/flag types\n* [`IArrayMesh`][crate::classes::IArrayMesh]: virtual methods\n\n\nSee also [Godot docs for `ArrayMesh`](https://docs.godotengine.org/en/stable/classes/class_arraymesh.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ArrayMesh::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ArrayMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ArrayMesh`][crate::classes::ArrayMesh].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IMesh`][crate::classes::IMesh] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `ArrayMesh` methods](https://docs.godotengine.org/en/stable/classes/class_arraymesh.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IArrayMesh: crate::obj::GodotClass < Base = ArrayMesh > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ArrayMesh {
        pub fn add_blend_shape(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(636usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "add_blend_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_blend_shape_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(637usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "get_blend_shape_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_blend_shape_name(&self, index: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(638usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "get_blend_shape_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_blend_shape_name(&mut self, index: i32, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, StringName >,);
            let args = (index, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(639usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "set_blend_shape_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_blend_shapes(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(640usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "clear_blend_shapes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_blend_shape_mode(&mut self, mode: crate::classes::mesh::BlendShapeMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::mesh::BlendShapeMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(641usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "set_blend_shape_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_blend_shape_mode(&self,) -> crate::classes::mesh::BlendShapeMode {
            type CallRet = crate::classes::mesh::BlendShapeMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(642usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "get_blend_shape_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_surface_from_arrays_full(&mut self, primitive: crate::classes::mesh::PrimitiveType, arrays: RefArg < VarArray >, blend_shapes: RefArg < Array < VarArray > >, lods: RefArg < VarDictionary >, flags: crate::classes::mesh::ArrayFormat,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (crate::classes::mesh::PrimitiveType, RefArg < 'a0, VarArray >, RefArg < 'a1, Array < VarArray > >, RefArg < 'a2, VarDictionary >, crate::classes::mesh::ArrayFormat,);
            let args = (primitive, arrays, blend_shapes, lods, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(643usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "add_surface_from_arrays", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_surface_from_arrays_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_surface_from_arrays(&mut self, primitive: crate::classes::mesh::PrimitiveType, arrays: &VarArray,) {
            self.add_surface_from_arrays_ex(primitive, arrays,) . done()
        }
        #[inline]
        pub fn add_surface_from_arrays_ex < 'a > (&'a mut self, primitive: crate::classes::mesh::PrimitiveType, arrays: &'a VarArray,) -> ExAddSurfaceFromArrays < 'a > {
            ExAddSurfaceFromArrays::new(self, primitive, arrays,)
        }
        pub fn clear_surfaces(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(644usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "clear_surfaces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_remove(&mut self, surf_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(645usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_remove", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_update_vertex_region(&mut self, surf_idx: i32, offset: i32, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, RefArg < 'a0, PackedByteArray >,);
            let args = (surf_idx, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(646usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_update_vertex_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_update_attribute_region(&mut self, surf_idx: i32, offset: i32, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, RefArg < 'a0, PackedByteArray >,);
            let args = (surf_idx, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(647usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_update_attribute_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_update_skin_region(&mut self, surf_idx: i32, offset: i32, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, RefArg < 'a0, PackedByteArray >,);
            let args = (surf_idx, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(648usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_update_skin_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_get_array_len(&self, surf_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(649usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_get_array_len", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_get_array_index_len(&self, surf_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(650usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_get_array_index_len", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_get_format(&self, surf_idx: i32,) -> crate::classes::mesh::ArrayFormat {
            type CallRet = crate::classes::mesh::ArrayFormat;
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(651usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_get_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_get_primitive_type(&self, surf_idx: i32,) -> crate::classes::mesh::PrimitiveType {
            type CallRet = crate::classes::mesh::PrimitiveType;
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(652usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_get_primitive_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_find_by_name(&self, name: impl AsArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(653usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_find_by_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_set_name(&mut self, surf_idx: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (surf_idx, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(654usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_set_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_get_name(&self, surf_idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(655usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "surface_get_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn regen_normal_maps(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(656usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "regen_normal_maps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_unwrap(&mut self, transform: Transform3D, texel_size: f32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (Transform3D, f32,);
            let args = (transform, texel_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(657usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "lightmap_unwrap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_aabb(&mut self, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Aabb,);
            let args = (aabb,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(658usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "set_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_aabb(&self,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(659usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "get_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_mesh(&mut self, mesh: impl AsArg < Option < Gd < crate::classes::ArrayMesh >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ArrayMesh > > >,);
            let args = (mesh.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(660usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "set_shadow_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadow_mesh(&self,) -> Option < Gd < crate::classes::ArrayMesh > > {
            type CallRet = Option < Gd < crate::classes::ArrayMesh > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(661usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ArrayMesh", "get_shadow_mesh", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ArrayMesh {
        type Base = crate::classes::Mesh;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ArrayMesh"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ArrayMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Mesh > for ArrayMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for ArrayMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ArrayMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ArrayMesh {
        
    }
    impl crate::obj::cap::GodotDefault for ArrayMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ArrayMesh {
        type Target = crate::classes::Mesh;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ArrayMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ArrayMesh`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ArrayMesh__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ArrayMesh > for $Class {
                
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
#[doc = "Default-param extender for [`ArrayMesh::add_surface_from_arrays_ex`][super::ArrayMesh::add_surface_from_arrays_ex]."]
#[must_use]
pub struct ExAddSurfaceFromArrays < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ArrayMesh, primitive: crate::classes::mesh::PrimitiveType, arrays: CowArg < 'a, VarArray >, blend_shapes: CowArg < 'a, Array < VarArray > >, lods: CowArg < 'a, VarDictionary >, flags: crate::classes::mesh::ArrayFormat,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSurfaceFromArrays < 'a > {
    fn new(surround_object: &'a mut re_export::ArrayMesh, primitive: crate::classes::mesh::PrimitiveType, arrays: &'a VarArray,) -> Self {
        let blend_shapes = Array::new();
        let lods = VarDictionary::new();
        let flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, primitive: primitive, arrays: CowArg::Borrowed(arrays), blend_shapes: CowArg::Owned(blend_shapes), lods: CowArg::Owned(lods), flags: flags,
        }
    }
    #[inline]
    pub fn blend_shapes(self, blend_shapes: &'a Array < VarArray >) -> Self {
        Self {
            blend_shapes: CowArg::Borrowed(blend_shapes), .. self
        }
    }
    #[inline]
    pub fn lods(self, lods: &'a VarDictionary) -> Self {
        Self {
            lods: CowArg::Borrowed(lods), .. self
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::mesh::ArrayFormat) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, primitive, arrays, blend_shapes, lods, flags,
        }
        = self;
        re_export::ArrayMesh::add_surface_from_arrays_full(surround_object, primitive, arrays.cow_as_arg(), blend_shapes.cow_as_arg(), lods.cow_as_arg(), flags,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ArrayMesh;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for ArrayMesh {
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