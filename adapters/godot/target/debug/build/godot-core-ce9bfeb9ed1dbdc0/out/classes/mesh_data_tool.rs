#![doc = "Sidecar module for class [`MeshDataTool`][crate::classes::MeshDataTool].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MeshDataTool` enums](https://docs.godotengine.org/en/stable/classes/class_meshdatatool.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MeshDataTool.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`mesh_data_tool`][crate::classes::mesh_data_tool]: sidecar module with related enum/flag types\n* [`IMeshDataTool`][crate::classes::IMeshDataTool]: virtual methods\n\n\nSee also [Godot docs for `MeshDataTool`](https://docs.godotengine.org/en/stable/classes/class_meshdatatool.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`MeshDataTool::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MeshDataTool {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`MeshDataTool`][crate::classes::MeshDataTool].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `MeshDataTool` methods](https://docs.godotengine.org/en/stable/classes/class_meshdatatool.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMeshDataTool: crate::obj::GodotClass < Base = MeshDataTool > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MeshDataTool {
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5305usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_from_surface(&mut self, mesh: impl AsArg < Option < Gd < crate::classes::ArrayMesh >> >, surface: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ArrayMesh > > >, i32,);
            let args = (mesh.into_arg(), surface,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "create_from_surface", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn commit_to_surface_full(&mut self, mesh: CowArg < Option < Gd < crate::classes::ArrayMesh > > >, compression_flags: u64,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ArrayMesh > > >, u64,);
            let args = (mesh, compression_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "commit_to_surface", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::commit_to_surface_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn commit_to_surface(&mut self, mesh: impl AsArg < Option < Gd < crate::classes::ArrayMesh >> >,) -> crate::global::Error {
            self.commit_to_surface_ex(mesh,) . done()
        }
        #[inline]
        pub fn commit_to_surface_ex < 'a > (&'a mut self, mesh: impl AsArg < Option < Gd < crate::classes::ArrayMesh >> > + 'a,) -> ExCommitToSurface < 'a > {
            ExCommitToSurface::new(self, mesh,)
        }
        pub fn get_format(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5308usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertex_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5309usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_edge_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5310usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_edge_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_face_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5311usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_face_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertex(&mut self, idx: i32, vertex: Vector3,) {
            type CallRet = ();
            type CallParams = (i32, Vector3,);
            let args = (idx, vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5312usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertex(&self, idx: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5313usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertex_normal(&mut self, idx: i32, normal: Vector3,) {
            type CallRet = ();
            type CallParams = (i32, Vector3,);
            let args = (idx, normal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5314usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_normal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertex_normal(&self, idx: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5315usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_normal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertex_tangent(&mut self, idx: i32, tangent: Plane,) {
            type CallRet = ();
            type CallParams = (i32, Plane,);
            let args = (idx, tangent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5316usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_tangent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertex_tangent(&self, idx: i32,) -> Plane {
            type CallRet = Plane;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5317usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_tangent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertex_uv(&mut self, idx: i32, uv: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2,);
            let args = (idx, uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5318usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_uv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertex_uv(&self, idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5319usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_uv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertex_uv2(&mut self, idx: i32, uv2: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2,);
            let args = (idx, uv2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5320usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_uv2", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertex_uv2(&self, idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5321usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_uv2", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertex_color(&mut self, idx: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (idx, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertex_color(&self, idx: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5323usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertex_bones(&mut self, idx: i32, bones: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, PackedInt32Array >,);
            let args = (idx, RefArg::new(bones),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_bones", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertex_bones(&self, idx: i32,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_bones", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertex_weights(&mut self, idx: i32, weights: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, PackedFloat32Array >,);
            let args = (idx, RefArg::new(weights),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_weights", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertex_weights(&self, idx: i32,) -> PackedFloat32Array {
            type CallRet = PackedFloat32Array;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_weights", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertex_meta(&mut self, idx: i32, meta: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (idx, RefArg::new(meta),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_vertex_meta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertex_meta(&self, idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_meta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertex_edges(&self, idx: i32,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_edges", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertex_faces(&self, idx: i32,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_vertex_faces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_edge_vertex(&self, idx: i32, vertex: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (idx, vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_edge_vertex", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_edge_faces(&self, idx: i32,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_edge_faces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_edge_meta(&mut self, idx: i32, meta: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (idx, RefArg::new(meta),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_edge_meta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_edge_meta(&self, idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_edge_meta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_face_vertex(&self, idx: i32, vertex: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (idx, vertex,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_face_vertex", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_face_edge(&self, idx: i32, edge: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (idx, edge,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_face_edge", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_face_meta(&mut self, idx: i32, meta: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (idx, RefArg::new(meta),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_face_meta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_face_meta(&self, idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_face_meta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_face_normal(&self, idx: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_face_normal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_material(&mut self, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "set_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_material(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5342usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MeshDataTool", "get_material", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for MeshDataTool {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("MeshDataTool"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MeshDataTool {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for MeshDataTool {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MeshDataTool {
        
    }
    impl crate::obj::cap::GodotDefault for MeshDataTool {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MeshDataTool {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MeshDataTool {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MeshDataTool`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_MeshDataTool__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MeshDataTool > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`MeshDataTool::commit_to_surface_ex`][super::MeshDataTool::commit_to_surface_ex]."]
#[must_use]
pub struct ExCommitToSurface < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::MeshDataTool, mesh: CowArg < 'a, Option < Gd < crate::classes::ArrayMesh > > >, compression_flags: u64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCommitToSurface < 'a > {
    fn new(surround_object: &'a mut re_export::MeshDataTool, mesh: impl AsArg < Option < Gd < crate::classes::ArrayMesh >> > + 'a,) -> Self {
        let compression_flags = 0u64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mesh: mesh.into_arg(), compression_flags: compression_flags,
        }
    }
    #[inline]
    pub fn compression_flags(self, compression_flags: u64) -> Self {
        Self {
            compression_flags: compression_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, mesh, compression_flags,
        }
        = self;
        re_export::MeshDataTool::commit_to_surface_full(surround_object, mesh, compression_flags,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::MeshDataTool;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for MeshDataTool {
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