#![doc = "Sidecar module for class [`EditorNode3DGizmo`][crate::classes::EditorNode3DGizmo].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorNode3DGizmo` enums](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmo.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorNode3DGizmo.`\n\nInherits [`Node3DGizmo`][crate::classes::Node3DGizmo].\n\nRelated symbols:\n\n* [`editor_node_3d_gizmo`][crate::classes::editor_node_3d_gizmo]: sidecar module with related enum/flag types\n* [`IEditorNode3DGizmo`][crate::classes::IEditorNode3DGizmo]: virtual methods\n\n\nSee also [Godot docs for `EditorNode3DGizmo`](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmo.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`EditorNode3DGizmo::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorNode3DGizmo {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorNode3DGizmo`][crate::classes::EditorNode3DGizmo].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`INode3DGizmo`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `EditorNode3DGizmo` methods](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmo.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorNode3DGizmo: crate::obj::GodotClass < Base = EditorNode3DGizmo > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn redraw(&mut self,) {
            unimplemented !()
        }
        fn get_handle_name(&self, id: i32, secondary: bool,) -> GString {
            unimplemented !()
        }
        fn is_handle_highlighted(&self, id: i32, secondary: bool,) -> bool {
            unimplemented !()
        }
        fn get_handle_value(&self, id: i32, secondary: bool,) -> Variant {
            unimplemented !()
        }
        fn begin_handle_action(&mut self, id: i32, secondary: bool,) {
            unimplemented !()
        }
        fn set_handle(&mut self, id: i32, secondary: bool, camera: Option < Gd < crate::classes::Camera3D > >, point: Vector2,) {
            unimplemented !()
        }
        fn commit_handle(&mut self, id: i32, secondary: bool, restore: Variant, cancel: bool,) {
            unimplemented !()
        }
        fn subgizmos_intersect_ray(&self, camera: Option < Gd < crate::classes::Camera3D > >, point: Vector2,) -> i32 {
            unimplemented !()
        }
        fn subgizmos_intersect_frustum(&self, camera: Option < Gd < crate::classes::Camera3D > >, frustum: Array < Plane >,) -> PackedInt32Array {
            unimplemented !()
        }
        fn set_subgizmo_transform(&mut self, id: i32, transform: Transform3D,) {
            unimplemented !()
        }
        fn get_subgizmo_transform(&self, id: i32,) -> Transform3D {
            unimplemented !()
        }
        fn commit_subgizmos(&mut self, ids: PackedInt32Array, restores: Array < Transform3D >, cancel: bool,) {
            unimplemented !()
        }
    }
    impl EditorNode3DGizmo {
        pub(crate) fn add_lines_full(&mut self, lines: RefArg < PackedVector3Array >, material: CowArg < Option < Gd < crate::classes::Material > > >, billboard: bool, modulate: Color,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, PackedVector3Array >, CowArg < 'a1, Option < Gd < crate::classes::Material > > >, bool, Color,);
            let args = (lines, material, billboard, modulate,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmo", "add_lines", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_lines_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_lines(&mut self, lines: &PackedVector3Array, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            self.add_lines_ex(lines, material,) . done()
        }
        #[inline]
        pub fn add_lines_ex < 'a > (&'a mut self, lines: &'a PackedVector3Array, material: impl AsArg < Option < Gd < crate::classes::Material >> > + 'a,) -> ExAddLines < 'a > {
            ExAddLines::new(self, lines, material,)
        }
        pub(crate) fn add_mesh_full(&mut self, mesh: CowArg < Option < Gd < crate::classes::Mesh > > >, material: CowArg < Option < Gd < crate::classes::Material > > >, transform: Transform3D, skeleton: CowArg < Option < Gd < crate::classes::SkinReference > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::Mesh > > >, CowArg < 'a1, Option < Gd < crate::classes::Material > > >, Transform3D, CowArg < 'a2, Option < Gd < crate::classes::SkinReference > > >,);
            let args = (mesh, material, transform, skeleton,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmo", "add_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_mesh_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_mesh(&mut self, mesh: impl AsArg < Option < Gd < crate::classes::Mesh >> >,) {
            self.add_mesh_ex(mesh,) . done()
        }
        #[inline]
        pub fn add_mesh_ex < 'a > (&'a mut self, mesh: impl AsArg < Option < Gd < crate::classes::Mesh >> > + 'a,) -> ExAddMesh < 'a > {
            ExAddMesh::new(self, mesh,)
        }
        pub fn add_collision_segments(&mut self, segments: &PackedVector3Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector3Array >,);
            let args = (RefArg::new(segments),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmo", "add_collision_segments", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_collision_triangles(&mut self, triangles: impl AsArg < Option < Gd < crate::classes::TriangleMesh >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TriangleMesh > > >,);
            let args = (triangles.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmo", "add_collision_triangles", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_unscaled_billboard_full(&mut self, material: CowArg < Option < Gd < crate::classes::Material > > >, default_scale: f32, modulate: Color,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Material > > >, f32, Color,);
            let args = (material, default_scale, modulate,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmo", "add_unscaled_billboard", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_unscaled_billboard_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_unscaled_billboard(&mut self, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            self.add_unscaled_billboard_ex(material,) . done()
        }
        #[inline]
        pub fn add_unscaled_billboard_ex < 'a > (&'a mut self, material: impl AsArg < Option < Gd < crate::classes::Material >> > + 'a,) -> ExAddUnscaledBillboard < 'a > {
            ExAddUnscaledBillboard::new(self, material,)
        }
        pub(crate) fn add_handles_full(&mut self, handles: RefArg < PackedVector3Array >, material: CowArg < Option < Gd < crate::classes::Material > > >, ids: RefArg < PackedInt32Array >, billboard: bool, secondary: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, PackedVector3Array >, CowArg < 'a1, Option < Gd < crate::classes::Material > > >, RefArg < 'a2, PackedInt32Array >, bool, bool,);
            let args = (handles, material, ids, billboard, secondary,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmo", "add_handles", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_handles_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_handles(&mut self, handles: &PackedVector3Array, material: impl AsArg < Option < Gd < crate::classes::Material >> >, ids: &PackedInt32Array,) {
            self.add_handles_ex(handles, material, ids,) . done()
        }
        #[inline]
        pub fn add_handles_ex < 'a > (&'a mut self, handles: &'a PackedVector3Array, material: impl AsArg < Option < Gd < crate::classes::Material >> > + 'a, ids: &'a PackedInt32Array,) -> ExAddHandles < 'a > {
            ExAddHandles::new(self, handles, material, ids,)
        }
        pub fn set_node_3d(&mut self, node: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmo", "set_node_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_3d(&self,) -> Option < Gd < crate::classes::Node3D > > {
            type CallRet = Option < Gd < crate::classes::Node3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmo", "get_node_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_plugin(&self,) -> Option < Gd < crate::classes::EditorNode3DGizmoPlugin > > {
            type CallRet = Option < Gd < crate::classes::EditorNode3DGizmoPlugin > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmo", "get_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmo", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hidden(&mut self, hidden: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (hidden,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmo", "set_hidden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_subgizmo_selected(&self, id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmo", "is_subgizmo_selected", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_subgizmo_selection(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmo", "get_subgizmo_selection", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EditorNode3DGizmo {
        type Base = crate::classes::Node3DGizmo;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorNode3DGizmo"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorNode3DGizmo {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3DGizmo > for EditorNode3DGizmo {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorNode3DGizmo {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorNode3DGizmo {
        
    }
    impl crate::obj::cap::GodotDefault for EditorNode3DGizmo {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorNode3DGizmo {
        type Target = crate::classes::Node3DGizmo;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorNode3DGizmo {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorNode3DGizmo`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorNode3DGizmo__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorNode3DGizmo > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node3DGizmo > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmo::add_lines_ex`][super::EditorNode3DGizmo::add_lines_ex]."]
#[must_use]
pub struct ExAddLines < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorNode3DGizmo, lines: CowArg < 'a, PackedVector3Array >, material: CowArg < 'a, Option < Gd < crate::classes::Material > > >, billboard: bool, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddLines < 'a > {
    fn new(surround_object: &'a mut re_export::EditorNode3DGizmo, lines: &'a PackedVector3Array, material: impl AsArg < Option < Gd < crate::classes::Material >> > + 'a,) -> Self {
        let billboard = false;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, lines: CowArg::Borrowed(lines), material: material.into_arg(), billboard: billboard, modulate: modulate,
        }
    }
    #[inline]
    pub fn billboard(self, billboard: bool) -> Self {
        Self {
            billboard: billboard, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, lines, material, billboard, modulate,
        }
        = self;
        re_export::EditorNode3DGizmo::add_lines_full(surround_object, lines.cow_as_arg(), material, billboard, modulate,)
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmo::add_mesh_ex`][super::EditorNode3DGizmo::add_mesh_ex]."]
#[must_use]
pub struct ExAddMesh < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorNode3DGizmo, mesh: CowArg < 'a, Option < Gd < crate::classes::Mesh > > >, material: CowArg < 'a, Option < Gd < crate::classes::Material > > >, transform: Transform3D, skeleton: CowArg < 'a, Option < Gd < crate::classes::SkinReference > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddMesh < 'a > {
    fn new(surround_object: &'a mut re_export::EditorNode3DGizmo, mesh: impl AsArg < Option < Gd < crate::classes::Mesh >> > + 'a,) -> Self {
        let material = Gd::null_arg();
        let transform = Transform3D::__internal_codegen(1 as _, 0 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _, 0 as _);
        let skeleton = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mesh: mesh.into_arg(), material: material.into_arg(), transform: transform, skeleton: skeleton.into_arg(),
        }
    }
    #[inline]
    pub fn material(self, material: impl AsArg < Option < Gd < crate::classes::Material >> > + 'a) -> Self {
        Self {
            material: material.into_arg(), .. self
        }
    }
    #[inline]
    pub fn transform(self, transform: Transform3D) -> Self {
        Self {
            transform: transform, .. self
        }
    }
    #[inline]
    pub fn skeleton(self, skeleton: impl AsArg < Option < Gd < crate::classes::SkinReference >> > + 'a) -> Self {
        Self {
            skeleton: skeleton.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, mesh, material, transform, skeleton,
        }
        = self;
        re_export::EditorNode3DGizmo::add_mesh_full(surround_object, mesh, material, transform, skeleton,)
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmo::add_unscaled_billboard_ex`][super::EditorNode3DGizmo::add_unscaled_billboard_ex]."]
#[must_use]
pub struct ExAddUnscaledBillboard < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorNode3DGizmo, material: CowArg < 'a, Option < Gd < crate::classes::Material > > >, default_scale: f32, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddUnscaledBillboard < 'a > {
    fn new(surround_object: &'a mut re_export::EditorNode3DGizmo, material: impl AsArg < Option < Gd < crate::classes::Material >> > + 'a,) -> Self {
        let default_scale = 1f32;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, material: material.into_arg(), default_scale: default_scale, modulate: modulate,
        }
    }
    #[inline]
    pub fn default_scale(self, default_scale: f32) -> Self {
        Self {
            default_scale: default_scale, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, material, default_scale, modulate,
        }
        = self;
        re_export::EditorNode3DGizmo::add_unscaled_billboard_full(surround_object, material, default_scale, modulate,)
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmo::add_handles_ex`][super::EditorNode3DGizmo::add_handles_ex]."]
#[must_use]
pub struct ExAddHandles < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorNode3DGizmo, handles: CowArg < 'a, PackedVector3Array >, material: CowArg < 'a, Option < Gd < crate::classes::Material > > >, ids: CowArg < 'a, PackedInt32Array >, billboard: bool, secondary: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddHandles < 'a > {
    fn new(surround_object: &'a mut re_export::EditorNode3DGizmo, handles: &'a PackedVector3Array, material: impl AsArg < Option < Gd < crate::classes::Material >> > + 'a, ids: &'a PackedInt32Array,) -> Self {
        let billboard = false;
        let secondary = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, handles: CowArg::Borrowed(handles), material: material.into_arg(), ids: CowArg::Borrowed(ids), billboard: billboard, secondary: secondary,
        }
    }
    #[inline]
    pub fn billboard(self, billboard: bool) -> Self {
        Self {
            billboard: billboard, .. self
        }
    }
    #[inline]
    pub fn secondary(self, secondary: bool) -> Self {
        Self {
            secondary: secondary, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, handles, material, ids, billboard, secondary,
        }
        = self;
        re_export::EditorNode3DGizmo::add_handles_full(surround_object, handles.cow_as_arg(), material, ids.cow_as_arg(), billboard, secondary,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorNode3DGizmo;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EditorNode3DGizmo {
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