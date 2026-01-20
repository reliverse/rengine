#![doc = "Sidecar module for class [`CharacterBody3D`][crate::classes::CharacterBody3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CharacterBody3D` enums](https://docs.godotengine.org/en/stable/classes/class_characterbody3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CharacterBody3D.`\n\nInherits [`PhysicsBody3D`][crate::classes::PhysicsBody3D].\n\nRelated symbols:\n\n* [`character_body_3d`][crate::classes::character_body_3d]: sidecar module with related enum/flag types\n* [`ICharacterBody3D`][crate::classes::ICharacterBody3D]: virtual methods\n\n\nSee also [Godot docs for `CharacterBody3D`](https://docs.godotengine.org/en/stable/classes/class_characterbody3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`CharacterBody3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CharacterBody3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`CharacterBody3D`][crate::classes::CharacterBody3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IPhysicsBody3D`~~ > ~~`ICollisionObject3D`~~ > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `CharacterBody3D` methods](https://docs.godotengine.org/en/stable/classes/class_characterbody3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICharacterBody3D: crate::obj::GodotClass < Base = CharacterBody3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn input_event(&mut self, camera: Option < Gd < crate::classes::Camera3D > >, event: Option < Gd < crate::classes::InputEvent > >, event_position: Vector3, normal: Vector3, shape_idx: i32,) {
            unimplemented !()
        }
        fn mouse_enter(&mut self,) {
            unimplemented !()
        }
        fn mouse_exit(&mut self,) {
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
    impl CharacterBody3D {
        pub fn move_and_slide(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2052usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "move_and_slide", Some(self.__validated_obj()), args,)
            }
        }
        pub fn apply_floor_snap(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2053usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "apply_floor_snap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_velocity(&mut self, velocity: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2054usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_velocity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2055usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_safe_margin(&mut self, margin: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2056usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_safe_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_safe_margin(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2057usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_safe_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_floor_stop_on_slope_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2058usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_floor_stop_on_slope_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_floor_stop_on_slope_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2059usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_floor_stop_on_slope_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_floor_constant_speed_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2060usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_floor_constant_speed_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_floor_constant_speed_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2061usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_floor_constant_speed_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_floor_block_on_wall_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2062usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_floor_block_on_wall_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_floor_block_on_wall_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2063usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_floor_block_on_wall_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_slide_on_ceiling_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2064usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_slide_on_ceiling_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_slide_on_ceiling_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2065usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_slide_on_ceiling_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_platform_floor_layers(&mut self, exclude_layer: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (exclude_layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2066usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_platform_floor_layers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_platform_floor_layers(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2067usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_platform_floor_layers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_platform_wall_layers(&mut self, exclude_layer: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (exclude_layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2068usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_platform_wall_layers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_platform_wall_layers(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2069usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_platform_wall_layers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_slides(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2070usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_max_slides", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_slides(&mut self, max_slides: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_slides,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2071usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_max_slides", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_floor_max_angle(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2072usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_floor_max_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_floor_max_angle(&mut self, radians: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2073usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_floor_max_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_floor_snap_length(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2074usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_floor_snap_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_floor_snap_length(&mut self, floor_snap_length: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (floor_snap_length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2075usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_floor_snap_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_wall_min_slide_angle(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2076usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_wall_min_slide_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_wall_min_slide_angle(&mut self, radians: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2077usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_wall_min_slide_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_up_direction(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2078usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_up_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_up_direction(&mut self, up_direction: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (up_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2079usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_up_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_motion_mode(&mut self, mode: crate::classes::character_body_3d::MotionMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::character_body_3d::MotionMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2080usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_motion_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_motion_mode(&self,) -> crate::classes::character_body_3d::MotionMode {
            type CallRet = crate::classes::character_body_3d::MotionMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2081usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_motion_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_platform_on_leave(&mut self, on_leave_apply_velocity: crate::classes::character_body_3d::PlatformOnLeave,) {
            type CallRet = ();
            type CallParams = (crate::classes::character_body_3d::PlatformOnLeave,);
            let args = (on_leave_apply_velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2082usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "set_platform_on_leave", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_platform_on_leave(&self,) -> crate::classes::character_body_3d::PlatformOnLeave {
            type CallRet = crate::classes::character_body_3d::PlatformOnLeave;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2083usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_platform_on_leave", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_on_floor(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2084usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_on_floor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_on_floor_only(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2085usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_on_floor_only", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_on_ceiling(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2086usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_on_ceiling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_on_ceiling_only(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2087usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_on_ceiling_only", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_on_wall(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2088usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_on_wall", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_on_wall_only(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2089usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "is_on_wall_only", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_floor_normal(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2090usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_floor_normal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_wall_normal(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2091usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_wall_normal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_last_motion(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2092usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_last_motion", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_position_delta(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2093usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_position_delta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_real_velocity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2094usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_real_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_floor_angle_full(&self, up_direction: Vector3,) -> f32 {
            type CallRet = f32;
            type CallParams = (Vector3,);
            let args = (up_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2095usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_floor_angle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_floor_angle_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_floor_angle(&self,) -> f32 {
            self.get_floor_angle_ex() . done()
        }
        #[inline]
        pub fn get_floor_angle_ex < 'a > (&'a self,) -> ExGetFloorAngle < 'a > {
            ExGetFloorAngle::new(self,)
        }
        pub fn get_platform_velocity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2096usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_platform_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_platform_angular_velocity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2097usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_platform_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_slide_collision_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2098usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_slide_collision_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_slide_collision(&mut self, slide_idx: i32,) -> Option < Gd < crate::classes::KinematicCollision3D > > {
            type CallRet = Option < Gd < crate::classes::KinematicCollision3D > >;
            type CallParams = (i32,);
            let args = (slide_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2099usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_slide_collision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_last_slide_collision(&mut self,) -> Option < Gd < crate::classes::KinematicCollision3D > > {
            type CallRet = Option < Gd < crate::classes::KinematicCollision3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharacterBody3D", "get_last_slide_collision", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CharacterBody3D {
        type Base = crate::classes::PhysicsBody3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CharacterBody3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CharacterBody3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PhysicsBody3D > for CharacterBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject3D > for CharacterBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for CharacterBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CharacterBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CharacterBody3D {
        
    }
    impl crate::obj::cap::GodotDefault for CharacterBody3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CharacterBody3D {
        type Target = crate::classes::PhysicsBody3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CharacterBody3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CharacterBody3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CharacterBody3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CharacterBody3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PhysicsBody3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CollisionObject3D > for $Class {
                
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
#[doc = "Default-param extender for [`CharacterBody3D::get_floor_angle_ex`][super::CharacterBody3D::get_floor_angle_ex]."]
#[must_use]
pub struct ExGetFloorAngle < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::CharacterBody3D, up_direction: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetFloorAngle < 'a > {
    fn new(surround_object: &'a re_export::CharacterBody3D,) -> Self {
        let up_direction = Vector3::new(0 as _, 1 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, up_direction: up_direction,
        }
    }
    #[inline]
    pub fn up_direction(self, up_direction: Vector3) -> Self {
        Self {
            up_direction: up_direction, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, up_direction,
        }
        = self;
        re_export::CharacterBody3D::get_floor_angle_full(surround_object, up_direction,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MotionMode {
    ord: i32
}
impl MotionMode {
    #[doc(alias = "MOTION_MODE_GROUNDED")]
    #[doc = "Godot enumerator name: `MOTION_MODE_GROUNDED`"]
    pub const GROUNDED: MotionMode = MotionMode {
        ord: 0i32
    };
    #[doc(alias = "MOTION_MODE_FLOATING")]
    #[doc = "Godot enumerator name: `MOTION_MODE_FLOATING`"]
    pub const FLOATING: MotionMode = MotionMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for MotionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MotionMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MotionMode {
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
            Self::GROUNDED => "GROUNDED", Self::FLOATING => "FLOATING", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MotionMode::GROUNDED, MotionMode::FLOATING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MotionMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("GROUNDED", "MOTION_MODE_GROUNDED", MotionMode::GROUNDED), crate::meta::inspect::EnumConstant::new("FLOATING", "MOTION_MODE_FLOATING", MotionMode::FLOATING)]
        }
    }
}
impl crate::meta::GodotConvert for MotionMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MotionMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MotionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PlatformOnLeave {
    ord: i32
}
impl PlatformOnLeave {
    #[doc(alias = "PLATFORM_ON_LEAVE_ADD_VELOCITY")]
    #[doc = "Godot enumerator name: `PLATFORM_ON_LEAVE_ADD_VELOCITY`"]
    pub const ADD_VELOCITY: PlatformOnLeave = PlatformOnLeave {
        ord: 0i32
    };
    #[doc(alias = "PLATFORM_ON_LEAVE_ADD_UPWARD_VELOCITY")]
    #[doc = "Godot enumerator name: `PLATFORM_ON_LEAVE_ADD_UPWARD_VELOCITY`"]
    pub const ADD_UPWARD_VELOCITY: PlatformOnLeave = PlatformOnLeave {
        ord: 1i32
    };
    #[doc(alias = "PLATFORM_ON_LEAVE_DO_NOTHING")]
    #[doc = "Godot enumerator name: `PLATFORM_ON_LEAVE_DO_NOTHING`"]
    pub const DO_NOTHING: PlatformOnLeave = PlatformOnLeave {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PlatformOnLeave {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PlatformOnLeave") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PlatformOnLeave {
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
            Self::ADD_VELOCITY => "ADD_VELOCITY", Self::ADD_UPWARD_VELOCITY => "ADD_UPWARD_VELOCITY", Self::DO_NOTHING => "DO_NOTHING", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PlatformOnLeave::ADD_VELOCITY, PlatformOnLeave::ADD_UPWARD_VELOCITY, PlatformOnLeave::DO_NOTHING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PlatformOnLeave >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ADD_VELOCITY", "PLATFORM_ON_LEAVE_ADD_VELOCITY", PlatformOnLeave::ADD_VELOCITY), crate::meta::inspect::EnumConstant::new("ADD_UPWARD_VELOCITY", "PLATFORM_ON_LEAVE_ADD_UPWARD_VELOCITY", PlatformOnLeave::ADD_UPWARD_VELOCITY), crate::meta::inspect::EnumConstant::new("DO_NOTHING", "PLATFORM_ON_LEAVE_DO_NOTHING", PlatformOnLeave::DO_NOTHING)]
        }
    }
}
impl crate::meta::GodotConvert for PlatformOnLeave {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PlatformOnLeave {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PlatformOnLeave {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CharacterBody3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::collision_object_3d::SignalsOfCollisionObject3D;
    impl WithSignals for CharacterBody3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCollisionObject3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}