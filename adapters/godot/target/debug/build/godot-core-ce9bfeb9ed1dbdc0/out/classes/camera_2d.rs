#![doc = "Sidecar module for class [`Camera2D`][crate::classes::Camera2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Camera2D` enums](https://docs.godotengine.org/en/stable/classes/class_camera2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Camera2D.`\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`camera_2d`][crate::classes::camera_2d]: sidecar module with related enum/flag types\n* [`ICamera2D`][crate::classes::ICamera2D]: virtual methods\n\n\nSee also [Godot docs for `Camera2D`](https://docs.godotengine.org/en/stable/classes/class_camera2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Camera2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Camera2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Camera2D`][crate::classes::Camera2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode2D`][crate::classes::INode2D] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Camera2D` methods](https://docs.godotengine.org/en/stable/classes/class_camera2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICamera2D: crate::obj::GodotClass < Base = Camera2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
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
        fn draw(&mut self,) {
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
    impl Camera2D {
        pub fn set_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1645usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1646usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_anchor_mode(&mut self, anchor_mode: crate::classes::camera_2d::AnchorMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::camera_2d::AnchorMode,);
            let args = (anchor_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1647usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_anchor_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_anchor_mode(&self,) -> crate::classes::camera_2d::AnchorMode {
            type CallRet = crate::classes::camera_2d::AnchorMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1648usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_anchor_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ignore_rotation(&mut self, ignore: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (ignore,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1649usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_ignore_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_ignoring_rotation(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1650usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_ignoring_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_callback(&mut self, mode: crate::classes::camera_2d::Camera2DProcessCallback,) {
            type CallRet = ();
            type CallParams = (crate::classes::camera_2d::Camera2DProcessCallback,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1651usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_process_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_callback(&self,) -> crate::classes::camera_2d::Camera2DProcessCallback {
            type CallRet = crate::classes::camera_2d::Camera2DProcessCallback;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1652usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_process_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1653usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1654usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn make_current(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1655usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "make_current", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_current(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1656usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_current", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_limit_enabled(&mut self, limit_enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (limit_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1657usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_limit_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_limit_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1658usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_limit_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_limit(&mut self, margin: crate::global::Side, limit: i32,) {
            type CallRet = ();
            type CallParams = (crate::global::Side, i32,);
            let args = (margin, limit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1659usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_limit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_limit(&self, margin: crate::global::Side,) -> i32 {
            type CallRet = i32;
            type CallParams = (crate::global::Side,);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1660usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_limit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_limit_smoothing_enabled(&mut self, limit_smoothing_enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (limit_smoothing_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1661usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_limit_smoothing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_limit_smoothing_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1662usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_limit_smoothing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_vertical_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1663usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_drag_vertical_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drag_vertical_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1664usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_drag_vertical_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_horizontal_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1665usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_drag_horizontal_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drag_horizontal_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1666usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_drag_horizontal_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_vertical_offset(&mut self, offset: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1667usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_drag_vertical_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_drag_vertical_offset(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_drag_vertical_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_horizontal_offset(&mut self, offset: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1669usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_drag_horizontal_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_drag_horizontal_offset(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_drag_horizontal_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_margin(&mut self, margin: crate::global::Side, drag_margin: f32,) {
            type CallRet = ();
            type CallParams = (crate::global::Side, f32,);
            let args = (margin, drag_margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_drag_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_drag_margin(&self, margin: crate::global::Side,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::global::Side,);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_drag_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_target_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1673usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_target_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_screen_center_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1674usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_screen_center_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_screen_rotation(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1675usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_screen_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_zoom(&mut self, zoom: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (zoom,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1676usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_zoom", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_zoom(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1677usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_zoom", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_viewport(&mut self, viewport: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (viewport.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1678usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_custom_viewport", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_viewport(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1679usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_custom_viewport", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_position_smoothing_speed(&mut self, position_smoothing_speed: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (position_smoothing_speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1680usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_position_smoothing_speed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_position_smoothing_speed(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1681usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_position_smoothing_speed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_position_smoothing_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1682usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_position_smoothing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_position_smoothing_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1683usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_position_smoothing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rotation_smoothing_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1684usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_rotation_smoothing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_rotation_smoothing_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1685usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_rotation_smoothing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rotation_smoothing_speed(&mut self, speed: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1686usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_rotation_smoothing_speed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rotation_smoothing_speed(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1687usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "get_rotation_smoothing_speed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn force_update_scroll(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1688usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "force_update_scroll", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reset_smoothing(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1689usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "reset_smoothing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn align(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1690usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "align", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_screen_drawing_enabled(&mut self, screen_drawing_enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (screen_drawing_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_screen_drawing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_screen_drawing_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_screen_drawing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_limit_drawing_enabled(&mut self, limit_drawing_enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (limit_drawing_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_limit_drawing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_limit_drawing_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_limit_drawing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_margin_drawing_enabled(&mut self, margin_drawing_enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (margin_drawing_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "set_margin_drawing_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_margin_drawing_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Camera2D", "is_margin_drawing_enabled", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Camera2D {
        type Base = crate::classes::Node2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Camera2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Camera2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for Camera2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Camera2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Camera2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Camera2D {
        
    }
    impl crate::obj::cap::GodotDefault for Camera2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Camera2D {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Camera2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Camera2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Camera2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Camera2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnchorMode {
    ord: i32
}
impl AnchorMode {
    #[doc(alias = "ANCHOR_MODE_FIXED_TOP_LEFT")]
    #[doc = "Godot enumerator name: `ANCHOR_MODE_FIXED_TOP_LEFT`"]
    pub const FIXED_TOP_LEFT: AnchorMode = AnchorMode {
        ord: 0i32
    };
    #[doc(alias = "ANCHOR_MODE_DRAG_CENTER")]
    #[doc = "Godot enumerator name: `ANCHOR_MODE_DRAG_CENTER`"]
    pub const DRAG_CENTER: AnchorMode = AnchorMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for AnchorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnchorMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnchorMode {
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
            Self::FIXED_TOP_LEFT => "FIXED_TOP_LEFT", Self::DRAG_CENTER => "DRAG_CENTER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AnchorMode::FIXED_TOP_LEFT, AnchorMode::DRAG_CENTER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AnchorMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FIXED_TOP_LEFT", "ANCHOR_MODE_FIXED_TOP_LEFT", AnchorMode::FIXED_TOP_LEFT), crate::meta::inspect::EnumConstant::new("DRAG_CENTER", "ANCHOR_MODE_DRAG_CENTER", AnchorMode::DRAG_CENTER)]
        }
    }
}
impl crate::meta::GodotConvert for AnchorMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AnchorMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnchorMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Camera2DProcessCallback {
    ord: i32
}
impl Camera2DProcessCallback {
    #[doc(alias = "CAMERA2D_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `CAMERA2D_PROCESS_PHYSICS`"]
    pub const PHYSICS: Camera2DProcessCallback = Camera2DProcessCallback {
        ord: 0i32
    };
    #[doc(alias = "CAMERA2D_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `CAMERA2D_PROCESS_IDLE`"]
    pub const IDLE: Camera2DProcessCallback = Camera2DProcessCallback {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for Camera2DProcessCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Camera2DProcessCallback") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Camera2DProcessCallback {
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
            Self::PHYSICS => "PHYSICS", Self::IDLE => "IDLE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Camera2DProcessCallback::PHYSICS, Camera2DProcessCallback::IDLE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Camera2DProcessCallback >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PHYSICS", "CAMERA2D_PROCESS_PHYSICS", Camera2DProcessCallback::PHYSICS), crate::meta::inspect::EnumConstant::new("IDLE", "CAMERA2D_PROCESS_IDLE", Camera2DProcessCallback::IDLE)]
        }
    }
}
impl crate::meta::GodotConvert for Camera2DProcessCallback {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Camera2DProcessCallback {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Camera2DProcessCallback {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Camera2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::canvas_item::SignalsOfCanvasItem;
    impl WithSignals for Camera2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCanvasItem < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}