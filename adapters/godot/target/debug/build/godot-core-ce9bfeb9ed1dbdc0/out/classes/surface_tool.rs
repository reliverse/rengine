#![doc = "Sidecar module for class [`SurfaceTool`][crate::classes::SurfaceTool].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SurfaceTool` enums](https://docs.godotengine.org/en/stable/classes/class_surfacetool.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SurfaceTool.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`surface_tool`][crate::classes::surface_tool]: sidecar module with related enum/flag types\n* [`ISurfaceTool`][crate::classes::ISurfaceTool]: virtual methods\n\n\nSee also [Godot docs for `SurfaceTool`](https://docs.godotengine.org/en/stable/classes/class_surfacetool.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`SurfaceTool::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SurfaceTool {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`SurfaceTool`][crate::classes::SurfaceTool].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `SurfaceTool` methods](https://docs.godotengine.org/en/stable/classes/class_surfacetool.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISurfaceTool: crate::obj::GodotClass < Base = SurfaceTool > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SurfaceTool {
        pub fn set_skin_weight_count(&mut self, count: crate::classes::surface_tool::SkinWeightCount,) {
            type CallRet = ();
            type CallParams = (crate::classes::surface_tool::SkinWeightCount,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8682usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_skin_weight_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_skin_weight_count(&self,) -> crate::classes::surface_tool::SkinWeightCount {
            type CallRet = crate::classes::surface_tool::SkinWeightCount;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8683usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "get_skin_weight_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_format(&mut self, channel_index: i32, format: crate::classes::surface_tool::CustomFormat,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::surface_tool::CustomFormat,);
            let args = (channel_index, format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8684usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_custom_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_format(&self, channel_index: i32,) -> crate::classes::surface_tool::CustomFormat {
            type CallRet = crate::classes::surface_tool::CustomFormat;
            type CallParams = (i32,);
            let args = (channel_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8685usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "get_custom_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn begin(&mut self, primitive: crate::classes::mesh::PrimitiveType,) {
            type CallRet = ();
            type CallParams = (crate::classes::mesh::PrimitiveType,);
            let args = (primitive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8686usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "begin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_vertex(&mut self, vertex: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8687usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "add_vertex", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8688usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_normal(&mut self, normal: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (normal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8689usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_normal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tangent(&mut self, tangent: Plane,) {
            type CallRet = ();
            type CallParams = (Plane,);
            let args = (tangent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8690usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_tangent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_uv(&mut self, uv: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_uv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_uv2(&mut self, uv2: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (uv2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_uv2", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bones(&mut self, bones: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedInt32Array >,);
            let args = (RefArg::new(bones),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_bones", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_weights(&mut self, weights: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedFloat32Array >,);
            let args = (RefArg::new(weights),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_weights", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom(&mut self, channel_index: i32, custom_color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (channel_index, custom_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_custom", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_smooth_group(&mut self, index: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_smooth_group", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_triangle_fan_full(&mut self, vertices: RefArg < PackedVector3Array >, uvs: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, uv2s: RefArg < PackedVector2Array >, normals: RefArg < PackedVector3Array >, tangents: RefArg < Array < Plane > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, 'a5, > = (RefArg < 'a0, PackedVector3Array >, RefArg < 'a1, PackedVector2Array >, RefArg < 'a2, PackedColorArray >, RefArg < 'a3, PackedVector2Array >, RefArg < 'a4, PackedVector3Array >, RefArg < 'a5, Array < Plane > >,);
            let args = (vertices, uvs, colors, uv2s, normals, tangents,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "add_triangle_fan", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_triangle_fan_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_triangle_fan(&mut self, vertices: &PackedVector3Array,) {
            self.add_triangle_fan_ex(vertices,) . done()
        }
        #[inline]
        pub fn add_triangle_fan_ex < 'a > (&'a mut self, vertices: &'a PackedVector3Array,) -> ExAddTriangleFan < 'a > {
            ExAddTriangleFan::new(self, vertices,)
        }
        pub fn add_index(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "add_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn index(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn deindex(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "deindex", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn generate_normals_full(&mut self, flip: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (flip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "generate_normals", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::generate_normals_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn generate_normals(&mut self,) {
            self.generate_normals_ex() . done()
        }
        #[inline]
        pub fn generate_normals_ex < 'a > (&'a mut self,) -> ExGenerateNormals < 'a > {
            ExGenerateNormals::new(self,)
        }
        pub fn generate_tangents(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8702usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "generate_tangents", Some(self.__validated_obj()), args,)
            }
        }
        pub fn optimize_indices_for_cache(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8703usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "optimize_indices_for_cache", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_aabb(&self,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "get_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn generate_lod_full(&mut self, nd_threshold: f32, target_index_count: i32,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (f32, i32,);
            let args = (nd_threshold, target_index_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "generate_lod", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::generate_lod_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn generate_lod(&mut self, nd_threshold: f32,) -> PackedInt32Array {
            self.generate_lod_ex(nd_threshold,) . done()
        }
        #[inline]
        pub fn generate_lod_ex < 'a > (&'a mut self, nd_threshold: f32,) -> ExGenerateLod < 'a > {
            ExGenerateLod::new(self, nd_threshold,)
        }
        pub fn set_material(&mut self, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "set_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_primitive_type(&self,) -> crate::classes::mesh::PrimitiveType {
            type CallRet = crate::classes::mesh::PrimitiveType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8707usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "get_primitive_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8708usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_from(&mut self, existing: impl AsArg < Option < Gd < crate::classes::Mesh >> >, surface: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Mesh > > >, i32,);
            let args = (existing.into_arg(), surface,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8709usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "create_from", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn create_from_arrays_full(&mut self, arrays: RefArg < VarArray >, primitive_type: crate::classes::mesh::PrimitiveType,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarArray >, crate::classes::mesh::PrimitiveType,);
            let args = (arrays, primitive_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8710usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "create_from_arrays", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_from_arrays_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_from_arrays(&mut self, arrays: &VarArray,) {
            self.create_from_arrays_ex(arrays,) . done()
        }
        #[inline]
        pub fn create_from_arrays_ex < 'a > (&'a mut self, arrays: &'a VarArray,) -> ExCreateFromArrays < 'a > {
            ExCreateFromArrays::new(self, arrays,)
        }
        pub fn create_from_blend_shape(&mut self, existing: impl AsArg < Option < Gd < crate::classes::Mesh >> >, surface: i32, blend_shape: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Mesh > > >, i32, CowArg < 'a1, GString >,);
            let args = (existing.into_arg(), surface, blend_shape.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8711usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "create_from_blend_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn append_from(&mut self, existing: impl AsArg < Option < Gd < crate::classes::Mesh >> >, surface: i32, transform: Transform3D,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Mesh > > >, i32, Transform3D,);
            let args = (existing.into_arg(), surface, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8712usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "append_from", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn commit_full(&mut self, existing: CowArg < Option < Gd < crate::classes::ArrayMesh > > >, flags: u64,) -> Option < Gd < crate::classes::ArrayMesh > > {
            type CallRet = Option < Gd < crate::classes::ArrayMesh > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ArrayMesh > > >, u64,);
            let args = (existing, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8713usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "commit", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::commit_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn commit(&mut self,) -> Option < Gd < crate::classes::ArrayMesh > > {
            self.commit_ex() . done()
        }
        #[inline]
        pub fn commit_ex < 'a > (&'a mut self,) -> ExCommit < 'a > {
            ExCommit::new(self,)
        }
        pub fn commit_to_arrays(&mut self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8714usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SurfaceTool", "commit_to_arrays", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for SurfaceTool {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("SurfaceTool"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SurfaceTool {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for SurfaceTool {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SurfaceTool {
        
    }
    impl crate::obj::cap::GodotDefault for SurfaceTool {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SurfaceTool {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SurfaceTool {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SurfaceTool`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_SurfaceTool__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SurfaceTool > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`SurfaceTool::add_triangle_fan_ex`][super::SurfaceTool::add_triangle_fan_ex]."]
#[must_use]
pub struct ExAddTriangleFan < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SurfaceTool, vertices: CowArg < 'a, PackedVector3Array >, uvs: CowArg < 'a, PackedVector2Array >, colors: CowArg < 'a, PackedColorArray >, uv2s: CowArg < 'a, PackedVector2Array >, normals: CowArg < 'a, PackedVector3Array >, tangents: CowArg < 'a, Array < Plane > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTriangleFan < 'a > {
    fn new(surround_object: &'a mut re_export::SurfaceTool, vertices: &'a PackedVector3Array,) -> Self {
        let uvs = PackedVector2Array::new();
        let colors = PackedColorArray::new();
        let uv2s = PackedVector2Array::new();
        let normals = PackedVector3Array::new();
        let tangents = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, vertices: CowArg::Borrowed(vertices), uvs: CowArg::Owned(uvs), colors: CowArg::Owned(colors), uv2s: CowArg::Owned(uv2s), normals: CowArg::Owned(normals), tangents: CowArg::Owned(tangents),
        }
    }
    #[inline]
    pub fn uvs(self, uvs: &'a PackedVector2Array) -> Self {
        Self {
            uvs: CowArg::Borrowed(uvs), .. self
        }
    }
    #[inline]
    pub fn colors(self, colors: &'a PackedColorArray) -> Self {
        Self {
            colors: CowArg::Borrowed(colors), .. self
        }
    }
    #[inline]
    pub fn uv2s(self, uv2s: &'a PackedVector2Array) -> Self {
        Self {
            uv2s: CowArg::Borrowed(uv2s), .. self
        }
    }
    #[inline]
    pub fn normals(self, normals: &'a PackedVector3Array) -> Self {
        Self {
            normals: CowArg::Borrowed(normals), .. self
        }
    }
    #[inline]
    pub fn tangents(self, tangents: &'a Array < Plane >) -> Self {
        Self {
            tangents: CowArg::Borrowed(tangents), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, vertices, uvs, colors, uv2s, normals, tangents,
        }
        = self;
        re_export::SurfaceTool::add_triangle_fan_full(surround_object, vertices.cow_as_arg(), uvs.cow_as_arg(), colors.cow_as_arg(), uv2s.cow_as_arg(), normals.cow_as_arg(), tangents.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`SurfaceTool::generate_normals_ex`][super::SurfaceTool::generate_normals_ex]."]
#[must_use]
pub struct ExGenerateNormals < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SurfaceTool, flip: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGenerateNormals < 'a > {
    fn new(surround_object: &'a mut re_export::SurfaceTool,) -> Self {
        let flip = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, flip: flip,
        }
    }
    #[inline]
    pub fn flip(self, flip: bool) -> Self {
        Self {
            flip: flip, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, flip,
        }
        = self;
        re_export::SurfaceTool::generate_normals_full(surround_object, flip,)
    }
}
#[doc = "Default-param extender for [`SurfaceTool::generate_lod_ex`][super::SurfaceTool::generate_lod_ex]."]
#[must_use]
pub struct ExGenerateLod < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SurfaceTool, nd_threshold: f32, target_index_count: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGenerateLod < 'a > {
    fn new(surround_object: &'a mut re_export::SurfaceTool, nd_threshold: f32,) -> Self {
        let target_index_count = 3i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, nd_threshold: nd_threshold, target_index_count: target_index_count,
        }
    }
    #[inline]
    pub fn target_index_count(self, target_index_count: i32) -> Self {
        Self {
            target_index_count: target_index_count, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, nd_threshold, target_index_count,
        }
        = self;
        re_export::SurfaceTool::generate_lod_full(surround_object, nd_threshold, target_index_count,)
    }
}
#[doc = "Default-param extender for [`SurfaceTool::create_from_arrays_ex`][super::SurfaceTool::create_from_arrays_ex]."]
#[must_use]
pub struct ExCreateFromArrays < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SurfaceTool, arrays: CowArg < 'a, VarArray >, primitive_type: crate::classes::mesh::PrimitiveType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateFromArrays < 'a > {
    fn new(surround_object: &'a mut re_export::SurfaceTool, arrays: &'a VarArray,) -> Self {
        let primitive_type = crate::obj::EngineEnum::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, arrays: CowArg::Borrowed(arrays), primitive_type: primitive_type,
        }
    }
    #[inline]
    pub fn primitive_type(self, primitive_type: crate::classes::mesh::PrimitiveType) -> Self {
        Self {
            primitive_type: primitive_type, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, arrays, primitive_type,
        }
        = self;
        re_export::SurfaceTool::create_from_arrays_full(surround_object, arrays.cow_as_arg(), primitive_type,)
    }
}
#[doc = "Default-param extender for [`SurfaceTool::commit_ex`][super::SurfaceTool::commit_ex]."]
#[must_use]
pub struct ExCommit < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SurfaceTool, existing: CowArg < 'a, Option < Gd < crate::classes::ArrayMesh > > >, flags: u64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCommit < 'a > {
    fn new(surround_object: &'a mut re_export::SurfaceTool,) -> Self {
        let existing = Gd::null_arg();
        let flags = 0u64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, existing: existing.into_arg(), flags: flags,
        }
    }
    #[inline]
    pub fn existing(self, existing: impl AsArg < Option < Gd < crate::classes::ArrayMesh >> > + 'a) -> Self {
        Self {
            existing: existing.into_arg(), .. self
        }
    }
    #[inline]
    pub fn flags(self, flags: u64) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::ArrayMesh > > {
        let Self {
            _phantom, surround_object, existing, flags,
        }
        = self;
        re_export::SurfaceTool::commit_full(surround_object, existing, flags,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CustomFormat {
    ord: i32
}
impl CustomFormat {
    #[doc(alias = "CUSTOM_RGBA8_UNORM")]
    #[doc = "Godot enumerator name: `CUSTOM_RGBA8_UNORM`"]
    pub const RGBA8_UNORM: CustomFormat = CustomFormat {
        ord: 0i32
    };
    #[doc(alias = "CUSTOM_RGBA8_SNORM")]
    #[doc = "Godot enumerator name: `CUSTOM_RGBA8_SNORM`"]
    pub const RGBA8_SNORM: CustomFormat = CustomFormat {
        ord: 1i32
    };
    #[doc(alias = "CUSTOM_RG_HALF")]
    #[doc = "Godot enumerator name: `CUSTOM_RG_HALF`"]
    pub const RG_HALF: CustomFormat = CustomFormat {
        ord: 2i32
    };
    #[doc(alias = "CUSTOM_RGBA_HALF")]
    #[doc = "Godot enumerator name: `CUSTOM_RGBA_HALF`"]
    pub const RGBA_HALF: CustomFormat = CustomFormat {
        ord: 3i32
    };
    #[doc(alias = "CUSTOM_R_FLOAT")]
    #[doc = "Godot enumerator name: `CUSTOM_R_FLOAT`"]
    pub const R_FLOAT: CustomFormat = CustomFormat {
        ord: 4i32
    };
    #[doc(alias = "CUSTOM_RG_FLOAT")]
    #[doc = "Godot enumerator name: `CUSTOM_RG_FLOAT`"]
    pub const RG_FLOAT: CustomFormat = CustomFormat {
        ord: 5i32
    };
    #[doc(alias = "CUSTOM_RGB_FLOAT")]
    #[doc = "Godot enumerator name: `CUSTOM_RGB_FLOAT`"]
    pub const RGB_FLOAT: CustomFormat = CustomFormat {
        ord: 6i32
    };
    #[doc(alias = "CUSTOM_RGBA_FLOAT")]
    #[doc = "Godot enumerator name: `CUSTOM_RGBA_FLOAT`"]
    pub const RGBA_FLOAT: CustomFormat = CustomFormat {
        ord: 7i32
    };
    #[doc(alias = "CUSTOM_MAX")]
    #[doc = "Godot enumerator name: `CUSTOM_MAX`"]
    pub const MAX: CustomFormat = CustomFormat {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for CustomFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CustomFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CustomFormat {
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
            Self::RGBA8_UNORM => "RGBA8_UNORM", Self::RGBA8_SNORM => "RGBA8_SNORM", Self::RG_HALF => "RG_HALF", Self::RGBA_HALF => "RGBA_HALF", Self::R_FLOAT => "R_FLOAT", Self::RG_FLOAT => "RG_FLOAT", Self::RGB_FLOAT => "RGB_FLOAT", Self::RGBA_FLOAT => "RGBA_FLOAT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CustomFormat::RGBA8_UNORM, CustomFormat::RGBA8_SNORM, CustomFormat::RG_HALF, CustomFormat::RGBA_HALF, CustomFormat::R_FLOAT, CustomFormat::RG_FLOAT, CustomFormat::RGB_FLOAT, CustomFormat::RGBA_FLOAT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CustomFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RGBA8_UNORM", "CUSTOM_RGBA8_UNORM", CustomFormat::RGBA8_UNORM), crate::meta::inspect::EnumConstant::new("RGBA8_SNORM", "CUSTOM_RGBA8_SNORM", CustomFormat::RGBA8_SNORM), crate::meta::inspect::EnumConstant::new("RG_HALF", "CUSTOM_RG_HALF", CustomFormat::RG_HALF), crate::meta::inspect::EnumConstant::new("RGBA_HALF", "CUSTOM_RGBA_HALF", CustomFormat::RGBA_HALF), crate::meta::inspect::EnumConstant::new("R_FLOAT", "CUSTOM_R_FLOAT", CustomFormat::R_FLOAT), crate::meta::inspect::EnumConstant::new("RG_FLOAT", "CUSTOM_RG_FLOAT", CustomFormat::RG_FLOAT), crate::meta::inspect::EnumConstant::new("RGB_FLOAT", "CUSTOM_RGB_FLOAT", CustomFormat::RGB_FLOAT), crate::meta::inspect::EnumConstant::new("RGBA_FLOAT", "CUSTOM_RGBA_FLOAT", CustomFormat::RGBA_FLOAT), crate::meta::inspect::EnumConstant::new("MAX", "CUSTOM_MAX", CustomFormat::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CustomFormat {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for CustomFormat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CustomFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CustomFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SkinWeightCount {
    ord: i32
}
impl SkinWeightCount {
    pub const SKIN_4_WEIGHTS: SkinWeightCount = SkinWeightCount {
        ord: 0i32
    };
    pub const SKIN_8_WEIGHTS: SkinWeightCount = SkinWeightCount {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for SkinWeightCount {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SkinWeightCount") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SkinWeightCount {
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
            Self::SKIN_4_WEIGHTS => "SKIN_4_WEIGHTS", Self::SKIN_8_WEIGHTS => "SKIN_8_WEIGHTS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SkinWeightCount::SKIN_4_WEIGHTS, SkinWeightCount::SKIN_8_WEIGHTS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SkinWeightCount >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SKIN_4_WEIGHTS", "SKIN_4_WEIGHTS", SkinWeightCount::SKIN_4_WEIGHTS), crate::meta::inspect::EnumConstant::new("SKIN_8_WEIGHTS", "SKIN_8_WEIGHTS", SkinWeightCount::SKIN_8_WEIGHTS)]
        }
    }
}
impl crate::meta::GodotConvert for SkinWeightCount {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SkinWeightCount {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SkinWeightCount {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::SurfaceTool;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for SurfaceTool {
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