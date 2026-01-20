#![doc = "Sidecar module for class [`Camera3D`][crate::classes::Camera3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Camera3D` enums](https://docs.godotengine.org/en/stable/classes/class_camera3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Camera3D.`\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`camera_3d`][crate::classes::camera_3d]: sidecar module with related enum/flag types\n* [`ICamera3D`][crate::classes::ICamera3D]: virtual methods\n\n\nSee also [Godot docs for `Camera3D`](https://docs.godotengine.org/en/stable/classes/class_camera3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Camera3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Camera3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Camera3D`][crate::classes::Camera3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Camera3D` methods](https://docs.godotengine.org/en/stable/classes/class_camera3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICamera3D: crate::obj::GodotClass < Base = Camera3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
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
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_accessibility_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn get_focused_accessibility_element(&self,) -> Rid {
            unimplemented !()
        }
    }
    impl Camera3D {
        pub fn project_ray_normal(&self, screen_point: Vector2,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector2,);
            let args = (screen_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "project_ray_normal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn project_local_ray_normal(&self, screen_point: Vector2,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector2,);
            let args = (screen_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "project_local_ray_normal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn project_ray_origin(&self, screen_point: Vector2,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector2,);
            let args = (screen_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "project_ray_origin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn unproject_position(&self, world_point: Vector3,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector3,);
            let args = (world_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "unproject_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_position_behind(&self, world_point: Vector3,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector3,);
            let args = (world_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "is_position_behind", Some(self.__validated_obj()), args,)
            }
        }
        pub fn project_position(&self, screen_point: Vector2, z_depth: f32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector2, f32,);
            let args = (screen_point, z_depth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1702usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "project_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_perspective(&mut self, fov: f32, z_near: f32, z_far: f32,) {
            type CallRet = ();
            type CallParams = (f32, f32, f32,);
            let args = (fov, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1703usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_perspective", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_orthogonal(&mut self, size: f32, z_near: f32, z_far: f32,) {
            type CallRet = ();
            type CallParams = (f32, f32, f32,);
            let args = (size, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_orthogonal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_frustum(&mut self, size: f32, offset: Vector2, z_near: f32, z_far: f32,) {
            type CallRet = ();
            type CallParams = (f32, Vector2, f32, f32,);
            let args = (size, offset, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_frustum", Some(self.__validated_obj()), args,)
            }
        }
        pub fn make_current(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "make_current", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn clear_current_full(&mut self, enable_next: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable_next,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1707usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "clear_current", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::clear_current_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn clear_current(&mut self,) {
            self.clear_current_ex() . done()
        }
        #[inline]
        pub fn clear_current_ex < 'a > (&'a mut self,) -> ExClearCurrent < 'a > {
            ExClearCurrent::new(self,)
        }
        pub fn set_current(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1708usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_current", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_current(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1709usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "is_current", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_camera_transform(&self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1710usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_camera_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_camera_projection(&self,) -> Projection {
            type CallRet = Projection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1711usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_camera_projection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fov(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1712usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_fov", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frustum_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1713usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_frustum_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1714usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_far(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1715usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_far", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_near(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1716usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_near", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fov(&mut self, fov: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (fov,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1717usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_fov", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_frustum_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1718usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_frustum_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_size(&mut self, size: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1719usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_far(&mut self, far: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (far,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1720usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_far", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_near(&mut self, near: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (near,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1721usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_near", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_projection(&self,) -> crate::classes::camera_3d::ProjectionType {
            type CallRet = crate::classes::camera_3d::ProjectionType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1722usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_projection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_projection(&mut self, mode: crate::classes::camera_3d::ProjectionType,) {
            type CallRet = ();
            type CallParams = (crate::classes::camera_3d::ProjectionType,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1723usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_projection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_h_offset(&mut self, offset: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1724usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_h_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_h_offset(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1725usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_h_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_v_offset(&mut self, offset: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1726usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_v_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_v_offset(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1727usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_v_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cull_mask(&mut self, mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1728usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cull_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1729usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_environment(&mut self, env: impl AsArg < Option < Gd < crate::classes::Environment >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Environment > > >,);
            let args = (env.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1730usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_environment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_environment(&self,) -> Option < Gd < crate::classes::Environment > > {
            type CallRet = Option < Gd < crate::classes::Environment > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1731usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_environment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_attributes(&mut self, env: impl AsArg < Option < Gd < crate::classes::CameraAttributes >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::CameraAttributes > > >,);
            let args = (env.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1732usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_attributes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_attributes(&self,) -> Option < Gd < crate::classes::CameraAttributes > > {
            type CallRet = Option < Gd < crate::classes::CameraAttributes > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1733usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_attributes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_keep_aspect_mode(&mut self, mode: crate::classes::camera_3d::KeepAspect,) {
            type CallRet = ();
            type CallParams = (crate::classes::camera_3d::KeepAspect,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1734usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_keep_aspect_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_keep_aspect_mode(&self,) -> crate::classes::camera_3d::KeepAspect {
            type CallRet = crate::classes::camera_3d::KeepAspect;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1735usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_keep_aspect_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_doppler_tracking(&mut self, mode: crate::classes::camera_3d::DopplerTracking,) {
            type CallRet = ();
            type CallParams = (crate::classes::camera_3d::DopplerTracking,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1736usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_doppler_tracking", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_doppler_tracking(&self,) -> crate::classes::camera_3d::DopplerTracking {
            type CallRet = crate::classes::camera_3d::DopplerTracking;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1737usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_doppler_tracking", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frustum(&self,) -> Array < Plane > {
            type CallRet = Array < Plane >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1738usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_frustum", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_position_in_frustum(&self, world_point: Vector3,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector3,);
            let args = (world_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1739usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "is_position_in_frustum", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_camera_rid(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1740usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_camera_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pyramid_shape_rid(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1741usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_pyramid_shape_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cull_mask_value(&mut self, layer_number: i32, value: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1742usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "set_cull_mask_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cull_mask_value(&self, layer_number: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1743usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera3D", "get_cull_mask_value", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Camera3D {
        type Base = crate::classes::Node3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Camera3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Camera3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for Camera3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Camera3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Camera3D {
        
    }
    impl crate::obj::cap::GodotDefault for Camera3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Camera3D {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Camera3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Camera3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Camera3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Camera3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Camera3D::clear_current_ex`][super::Camera3D::clear_current_ex]."]
#[must_use]
pub struct ExClearCurrent < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Camera3D, enable_next: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClearCurrent < 'a > {
    fn new(surround_object: &'a mut re_export::Camera3D,) -> Self {
        let enable_next = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, enable_next: enable_next,
        }
    }
    #[inline]
    pub fn enable_next(self, enable_next: bool) -> Self {
        Self {
            enable_next: enable_next, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, enable_next,
        }
        = self;
        re_export::Camera3D::clear_current_full(surround_object, enable_next,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ProjectionType {
    ord: i32
}
impl ProjectionType {
    #[doc(alias = "PROJECTION_PERSPECTIVE")]
    #[doc = "Godot enumerator name: `PROJECTION_PERSPECTIVE`"]
    pub const PERSPECTIVE: ProjectionType = ProjectionType {
        ord: 0i32
    };
    #[doc(alias = "PROJECTION_ORTHOGONAL")]
    #[doc = "Godot enumerator name: `PROJECTION_ORTHOGONAL`"]
    pub const ORTHOGONAL: ProjectionType = ProjectionType {
        ord: 1i32
    };
    #[doc(alias = "PROJECTION_FRUSTUM")]
    #[doc = "Godot enumerator name: `PROJECTION_FRUSTUM`"]
    pub const FRUSTUM: ProjectionType = ProjectionType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ProjectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ProjectionType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ProjectionType {
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
            Self::PERSPECTIVE => "PERSPECTIVE", Self::ORTHOGONAL => "ORTHOGONAL", Self::FRUSTUM => "FRUSTUM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ProjectionType::PERSPECTIVE, ProjectionType::ORTHOGONAL, ProjectionType::FRUSTUM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ProjectionType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PERSPECTIVE", "PROJECTION_PERSPECTIVE", ProjectionType::PERSPECTIVE), crate::meta::inspect::EnumConstant::new("ORTHOGONAL", "PROJECTION_ORTHOGONAL", ProjectionType::ORTHOGONAL), crate::meta::inspect::EnumConstant::new("FRUSTUM", "PROJECTION_FRUSTUM", ProjectionType::FRUSTUM)]
        }
    }
}
impl crate::meta::GodotConvert for ProjectionType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ProjectionType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ProjectionType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct KeepAspect {
    ord: i32
}
impl KeepAspect {
    #[doc(alias = "KEEP_WIDTH")]
    #[doc = "Godot enumerator name: `KEEP_WIDTH`"]
    pub const WIDTH: KeepAspect = KeepAspect {
        ord: 0i32
    };
    #[doc(alias = "KEEP_HEIGHT")]
    #[doc = "Godot enumerator name: `KEEP_HEIGHT`"]
    pub const HEIGHT: KeepAspect = KeepAspect {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for KeepAspect {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("KeepAspect") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for KeepAspect {
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
            Self::WIDTH => "WIDTH", Self::HEIGHT => "HEIGHT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[KeepAspect::WIDTH, KeepAspect::HEIGHT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < KeepAspect >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("WIDTH", "KEEP_WIDTH", KeepAspect::WIDTH), crate::meta::inspect::EnumConstant::new("HEIGHT", "KEEP_HEIGHT", KeepAspect::HEIGHT)]
        }
    }
}
impl crate::meta::GodotConvert for KeepAspect {
    type Via = i32;
    
}
impl crate::meta::ToGodot for KeepAspect {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for KeepAspect {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DopplerTracking {
    ord: i32
}
impl DopplerTracking {
    #[doc(alias = "DOPPLER_TRACKING_DISABLED")]
    #[doc = "Godot enumerator name: `DOPPLER_TRACKING_DISABLED`"]
    pub const DISABLED: DopplerTracking = DopplerTracking {
        ord: 0i32
    };
    #[doc(alias = "DOPPLER_TRACKING_IDLE_STEP")]
    #[doc = "Godot enumerator name: `DOPPLER_TRACKING_IDLE_STEP`"]
    pub const IDLE_STEP: DopplerTracking = DopplerTracking {
        ord: 1i32
    };
    #[doc(alias = "DOPPLER_TRACKING_PHYSICS_STEP")]
    #[doc = "Godot enumerator name: `DOPPLER_TRACKING_PHYSICS_STEP`"]
    pub const PHYSICS_STEP: DopplerTracking = DopplerTracking {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DopplerTracking {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DopplerTracking") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DopplerTracking {
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
            Self::DISABLED => "DISABLED", Self::IDLE_STEP => "IDLE_STEP", Self::PHYSICS_STEP => "PHYSICS_STEP", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DopplerTracking::DISABLED, DopplerTracking::IDLE_STEP, DopplerTracking::PHYSICS_STEP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DopplerTracking >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "DOPPLER_TRACKING_DISABLED", DopplerTracking::DISABLED), crate::meta::inspect::EnumConstant::new("IDLE_STEP", "DOPPLER_TRACKING_IDLE_STEP", DopplerTracking::IDLE_STEP), crate::meta::inspect::EnumConstant::new("PHYSICS_STEP", "DOPPLER_TRACKING_PHYSICS_STEP", DopplerTracking::PHYSICS_STEP)]
        }
    }
}
impl crate::meta::GodotConvert for DopplerTracking {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DopplerTracking {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DopplerTracking {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Camera3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for Camera3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}