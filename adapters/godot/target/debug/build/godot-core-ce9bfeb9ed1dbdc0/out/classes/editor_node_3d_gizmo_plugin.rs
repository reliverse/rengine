#![doc = "Sidecar module for class [`EditorNode3DGizmoPlugin`][crate::classes::EditorNode3DGizmoPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorNode3DGizmoPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmoplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorNode3DGizmoPlugin.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`editor_node_3d_gizmo_plugin`][crate::classes::editor_node_3d_gizmo_plugin]: sidecar module with related enum/flag types\n* [`IEditorNode3DGizmoPlugin`][crate::classes::IEditorNode3DGizmoPlugin]: virtual methods\n\n\nSee also [Godot docs for `EditorNode3DGizmoPlugin`](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmoplugin.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`EditorNode3DGizmoPlugin::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorNode3DGizmoPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorNode3DGizmoPlugin`][crate::classes::EditorNode3DGizmoPlugin].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `EditorNode3DGizmoPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmoplugin.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorNode3DGizmoPlugin: crate::obj::GodotClass < Base = EditorNode3DGizmoPlugin > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn has_gizmo(&self, for_node_3d: Option < Gd < crate::classes::Node3D > >,) -> bool {
            unimplemented !()
        }
        fn create_gizmo(&self, for_node_3d: Option < Gd < crate::classes::Node3D > >,) -> Option < Gd < crate::classes::EditorNode3DGizmo > > {
            unimplemented !()
        }
        fn get_gizmo_name(&self,) -> GString {
            unimplemented !()
        }
        fn get_priority(&self,) -> i32 {
            unimplemented !()
        }
        fn can_be_hidden(&self,) -> bool {
            unimplemented !()
        }
        fn is_selectable_when_hidden(&self,) -> bool {
            unimplemented !()
        }
        fn redraw(&mut self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >,) {
            unimplemented !()
        }
        fn get_handle_name(&self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, handle_id: i32, secondary: bool,) -> GString {
            unimplemented !()
        }
        fn is_handle_highlighted(&self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, handle_id: i32, secondary: bool,) -> bool {
            unimplemented !()
        }
        fn get_handle_value(&self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, handle_id: i32, secondary: bool,) -> Variant {
            unimplemented !()
        }
        fn begin_handle_action(&mut self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, handle_id: i32, secondary: bool,) {
            unimplemented !()
        }
        fn set_handle(&mut self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, handle_id: i32, secondary: bool, camera: Option < Gd < crate::classes::Camera3D > >, screen_pos: Vector2,) {
            unimplemented !()
        }
        fn commit_handle(&mut self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, handle_id: i32, secondary: bool, restore: Variant, cancel: bool,) {
            unimplemented !()
        }
        fn subgizmos_intersect_ray(&self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, camera: Option < Gd < crate::classes::Camera3D > >, screen_pos: Vector2,) -> i32 {
            unimplemented !()
        }
        fn subgizmos_intersect_frustum(&self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, camera: Option < Gd < crate::classes::Camera3D > >, frustum_planes: Array < Plane >,) -> PackedInt32Array {
            unimplemented !()
        }
        fn get_subgizmo_transform(&self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, subgizmo_id: i32,) -> Transform3D {
            unimplemented !()
        }
        fn set_subgizmo_transform(&mut self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, subgizmo_id: i32, transform: Transform3D,) {
            unimplemented !()
        }
        fn commit_subgizmos(&mut self, gizmo: Option < Gd < crate::classes::EditorNode3DGizmo > >, ids: PackedInt32Array, restores: Array < Transform3D >, cancel: bool,) {
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
    impl EditorNode3DGizmoPlugin {
        pub(crate) fn create_material_full(&mut self, name: CowArg < GString >, color: Color, billboard: bool, on_top: bool, use_vertex_color: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, Color, bool, bool, bool,);
            let args = (name, color, billboard, on_top, use_vertex_color,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmoPlugin", "create_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_material_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_material(&mut self, name: impl AsArg < GString >, color: Color,) {
            self.create_material_ex(name, color,) . done()
        }
        #[inline]
        pub fn create_material_ex < 'a > (&'a mut self, name: impl AsArg < GString > + 'a, color: Color,) -> ExCreateMaterial < 'a > {
            ExCreateMaterial::new(self, name, color,)
        }
        pub(crate) fn create_icon_material_full(&mut self, name: CowArg < GString >, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, on_top: bool, color: Color,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >, bool, Color,);
            let args = (name, texture, on_top, color,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmoPlugin", "create_icon_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_icon_material_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_icon_material(&mut self, name: impl AsArg < GString >, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            self.create_icon_material_ex(name, texture,) . done()
        }
        #[inline]
        pub fn create_icon_material_ex < 'a > (&'a mut self, name: impl AsArg < GString > + 'a, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a,) -> ExCreateIconMaterial < 'a > {
            ExCreateIconMaterial::new(self, name, texture,)
        }
        pub(crate) fn create_handle_material_full(&mut self, name: CowArg < GString >, billboard: bool, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, bool, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (name, billboard, texture,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmoPlugin", "create_handle_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_handle_material_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_handle_material(&mut self, name: impl AsArg < GString >,) {
            self.create_handle_material_ex(name,) . done()
        }
        #[inline]
        pub fn create_handle_material_ex < 'a > (&'a mut self, name: impl AsArg < GString > + 'a,) -> ExCreateHandleMaterial < 'a > {
            ExCreateHandleMaterial::new(self, name,)
        }
        pub fn add_material(&mut self, name: impl AsArg < GString >, material: impl AsArg < Option < Gd < crate::classes::StandardMaterial3D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::StandardMaterial3D > > >,);
            let args = (name.into_arg(), material.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmoPlugin", "add_material", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_material_full(&mut self, name: CowArg < GString >, gizmo: CowArg < Option < Gd < crate::classes::EditorNode3DGizmo > > >,) -> Option < Gd < crate::classes::StandardMaterial3D > > {
            type CallRet = Option < Gd < crate::classes::StandardMaterial3D > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::EditorNode3DGizmo > > >,);
            let args = (name, gizmo,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorNode3DGizmoPlugin", "get_material", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_material_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_material(&mut self, name: impl AsArg < GString >,) -> Option < Gd < crate::classes::StandardMaterial3D > > {
            self.get_material_ex(name,) . done()
        }
        #[inline]
        pub fn get_material_ex < 'a > (&'a mut self, name: impl AsArg < GString > + 'a,) -> ExGetMaterial < 'a > {
            ExGetMaterial::new(self, name,)
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
    impl crate::obj::GodotClass for EditorNode3DGizmoPlugin {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorNode3DGizmoPlugin"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorNode3DGizmoPlugin {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for EditorNode3DGizmoPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorNode3DGizmoPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorNode3DGizmoPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorNode3DGizmoPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorNode3DGizmoPlugin {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorNode3DGizmoPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorNode3DGizmoPlugin`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorNode3DGizmoPlugin__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorNode3DGizmoPlugin > for $Class {
                
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
#[doc = "Default-param extender for [`EditorNode3DGizmoPlugin::create_material_ex`][super::EditorNode3DGizmoPlugin::create_material_ex]."]
#[must_use]
pub struct ExCreateMaterial < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorNode3DGizmoPlugin, name: CowArg < 'a, GString >, color: Color, billboard: bool, on_top: bool, use_vertex_color: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateMaterial < 'a > {
    fn new(surround_object: &'a mut re_export::EditorNode3DGizmoPlugin, name: impl AsArg < GString > + 'a, color: Color,) -> Self {
        let billboard = false;
        let on_top = false;
        let use_vertex_color = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), color: color, billboard: billboard, on_top: on_top, use_vertex_color: use_vertex_color,
        }
    }
    #[inline]
    pub fn billboard(self, billboard: bool) -> Self {
        Self {
            billboard: billboard, .. self
        }
    }
    #[inline]
    pub fn on_top(self, on_top: bool) -> Self {
        Self {
            on_top: on_top, .. self
        }
    }
    #[inline]
    pub fn use_vertex_color(self, use_vertex_color: bool) -> Self {
        Self {
            use_vertex_color: use_vertex_color, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, color, billboard, on_top, use_vertex_color,
        }
        = self;
        re_export::EditorNode3DGizmoPlugin::create_material_full(surround_object, name, color, billboard, on_top, use_vertex_color,)
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmoPlugin::create_icon_material_ex`][super::EditorNode3DGizmoPlugin::create_icon_material_ex]."]
#[must_use]
pub struct ExCreateIconMaterial < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorNode3DGizmoPlugin, name: CowArg < 'a, GString >, texture: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, on_top: bool, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateIconMaterial < 'a > {
    fn new(surround_object: &'a mut re_export::EditorNode3DGizmoPlugin, name: impl AsArg < GString > + 'a, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a,) -> Self {
        let on_top = false;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), texture: texture.into_arg(), on_top: on_top, color: color,
        }
    }
    #[inline]
    pub fn on_top(self, on_top: bool) -> Self {
        Self {
            on_top: on_top, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, texture, on_top, color,
        }
        = self;
        re_export::EditorNode3DGizmoPlugin::create_icon_material_full(surround_object, name, texture, on_top, color,)
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmoPlugin::create_handle_material_ex`][super::EditorNode3DGizmoPlugin::create_handle_material_ex]."]
#[must_use]
pub struct ExCreateHandleMaterial < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorNode3DGizmoPlugin, name: CowArg < 'a, GString >, billboard: bool, texture: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateHandleMaterial < 'a > {
    fn new(surround_object: &'a mut re_export::EditorNode3DGizmoPlugin, name: impl AsArg < GString > + 'a,) -> Self {
        let billboard = false;
        let texture = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), billboard: billboard, texture: texture.into_arg(),
        }
    }
    #[inline]
    pub fn billboard(self, billboard: bool) -> Self {
        Self {
            billboard: billboard, .. self
        }
    }
    #[inline]
    pub fn texture(self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a) -> Self {
        Self {
            texture: texture.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, billboard, texture,
        }
        = self;
        re_export::EditorNode3DGizmoPlugin::create_handle_material_full(surround_object, name, billboard, texture,)
    }
}
#[doc = "Default-param extender for [`EditorNode3DGizmoPlugin::get_material_ex`][super::EditorNode3DGizmoPlugin::get_material_ex]."]
#[must_use]
pub struct ExGetMaterial < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorNode3DGizmoPlugin, name: CowArg < 'a, GString >, gizmo: CowArg < 'a, Option < Gd < crate::classes::EditorNode3DGizmo > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetMaterial < 'a > {
    fn new(surround_object: &'a mut re_export::EditorNode3DGizmoPlugin, name: impl AsArg < GString > + 'a,) -> Self {
        let gizmo = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), gizmo: gizmo.into_arg(),
        }
    }
    #[inline]
    pub fn gizmo(self, gizmo: impl AsArg < Option < Gd < crate::classes::EditorNode3DGizmo >> > + 'a) -> Self {
        Self {
            gizmo: gizmo.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::StandardMaterial3D > > {
        let Self {
            _phantom, surround_object, name, gizmo,
        }
        = self;
        re_export::EditorNode3DGizmoPlugin::get_material_full(surround_object, name, gizmo,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorNode3DGizmoPlugin;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for EditorNode3DGizmoPlugin {
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