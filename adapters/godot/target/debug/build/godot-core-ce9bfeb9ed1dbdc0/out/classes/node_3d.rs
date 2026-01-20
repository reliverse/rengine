#![doc = "Sidecar module for class [`Node3D`][crate::classes::Node3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Node3D` enums](https://docs.godotengine.org/en/stable/classes/class_node3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Node3D.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`node_3d`][crate::classes::node_3d]: sidecar module with related enum/flag types\n* [`INode3D`][crate::classes::INode3D]: virtual methods\n* [`SignalsOfNode3D`][crate::classes::node_3d::SignalsOfNode3D]: signal collection\n* [`Node3DNotification`][crate::classes::notify::Node3DNotification]: notification type\n\n\nSee also [Godot docs for `Node3D`](https://docs.godotengine.org/en/stable/classes/class_node3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Node3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Node3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Node3D`][crate::classes::Node3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Node3D` methods](https://docs.godotengine.org/en/stable/classes/class_node3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INode3D: crate::obj::GodotClass < Base = Node3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    #[doc = "Notification type for class [`Node3D`][crate::classes::Node3D]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[doc = r""]
    #[doc = r" Contains the [`Unknown`][Self::Unknown] variant for forward compatibility."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum Node3DNotification {
        TRANSFORM_CHANGED = 2000i32, ENTER_WORLD = 41i32, EXIT_WORLD = 42i32, VISIBILITY_CHANGED = 43i32, LOCAL_TRANSFORM_CHANGED = 44i32, ENTER_TREE = 10i32, EXIT_TREE = 11i32, MOVED_IN_PARENT = 12i32, READY = 13i32, PAUSED = 14i32, UNPAUSED = 15i32, PHYSICS_PROCESS = 16i32, PROCESS = 17i32, PARENTED = 18i32, UNPARENTED = 19i32, SCENE_INSTANTIATED = 20i32, DRAG_BEGIN = 21i32, DRAG_END = 22i32, PATH_RENAMED = 23i32, CHILD_ORDER_CHANGED = 24i32, INTERNAL_PROCESS = 25i32, INTERNAL_PHYSICS_PROCESS = 26i32, POST_ENTER_TREE = 27i32, DISABLED = 28i32, ENABLED = 29i32, RESET_PHYSICS_INTERPOLATION = 2001i32, EDITOR_PRE_SAVE = 9001i32, EDITOR_POST_SAVE = 9002i32, WM_MOUSE_ENTER = 1002i32, WM_MOUSE_EXIT = 1003i32, WM_WINDOW_FOCUS_IN = 1004i32, WM_WINDOW_FOCUS_OUT = 1005i32, WM_CLOSE_REQUEST = 1006i32, WM_GO_BACK_REQUEST = 1007i32, WM_SIZE_CHANGED = 1008i32, WM_DPI_CHANGE = 1009i32, VP_MOUSE_ENTER = 1010i32, VP_MOUSE_EXIT = 1011i32, WM_POSITION_CHANGED = 1012i32, OS_MEMORY_WARNING = 2009i32, TRANSLATION_CHANGED = 2010i32, WM_ABOUT = 2011i32, CRASH = 2012i32, OS_IME_UPDATE = 2013i32, APPLICATION_RESUMED = 2014i32, APPLICATION_PAUSED = 2015i32, APPLICATION_FOCUS_IN = 2016i32, APPLICATION_FOCUS_OUT = 2017i32, TEXT_SERVER_CHANGED = 2018i32, ACCESSIBILITY_UPDATE = 3000i32, ACCESSIBILITY_INVALIDATE = 3001i32, POSTINITIALIZE = 0i32, PREDELETE = 1i32, EXTENSION_RELOADED = 2i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        #[doc = r""]
        #[doc = r" This is also necessary if you develop an extension on a Godot version and want to be forward-compatible with newer"]
        #[doc = r" versions. If Godot adds new notifications, they will be unknown to your extension, but you can still handle them."]
        Unknown(i32),
    }
    impl From < i32 > for Node3DNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                2000i32 => Self::TRANSFORM_CHANGED, 41i32 => Self::ENTER_WORLD, 42i32 => Self::EXIT_WORLD, 43i32 => Self::VISIBILITY_CHANGED, 44i32 => Self::LOCAL_TRANSFORM_CHANGED, 10i32 => Self::ENTER_TREE, 11i32 => Self::EXIT_TREE, 12i32 => Self::MOVED_IN_PARENT, 13i32 => Self::READY, 14i32 => Self::PAUSED, 15i32 => Self::UNPAUSED, 16i32 => Self::PHYSICS_PROCESS, 17i32 => Self::PROCESS, 18i32 => Self::PARENTED, 19i32 => Self::UNPARENTED, 20i32 => Self::SCENE_INSTANTIATED, 21i32 => Self::DRAG_BEGIN, 22i32 => Self::DRAG_END, 23i32 => Self::PATH_RENAMED, 24i32 => Self::CHILD_ORDER_CHANGED, 25i32 => Self::INTERNAL_PROCESS, 26i32 => Self::INTERNAL_PHYSICS_PROCESS, 27i32 => Self::POST_ENTER_TREE, 28i32 => Self::DISABLED, 29i32 => Self::ENABLED, 2001i32 => Self::RESET_PHYSICS_INTERPOLATION, 9001i32 => Self::EDITOR_PRE_SAVE, 9002i32 => Self::EDITOR_POST_SAVE, 1002i32 => Self::WM_MOUSE_ENTER, 1003i32 => Self::WM_MOUSE_EXIT, 1004i32 => Self::WM_WINDOW_FOCUS_IN, 1005i32 => Self::WM_WINDOW_FOCUS_OUT, 1006i32 => Self::WM_CLOSE_REQUEST, 1007i32 => Self::WM_GO_BACK_REQUEST, 1008i32 => Self::WM_SIZE_CHANGED, 1009i32 => Self::WM_DPI_CHANGE, 1010i32 => Self::VP_MOUSE_ENTER, 1011i32 => Self::VP_MOUSE_EXIT, 1012i32 => Self::WM_POSITION_CHANGED, 2009i32 => Self::OS_MEMORY_WARNING, 2010i32 => Self::TRANSLATION_CHANGED, 2011i32 => Self::WM_ABOUT, 2012i32 => Self::CRASH, 2013i32 => Self::OS_IME_UPDATE, 2014i32 => Self::APPLICATION_RESUMED, 2015i32 => Self::APPLICATION_PAUSED, 2016i32 => Self::APPLICATION_FOCUS_IN, 2017i32 => Self::APPLICATION_FOCUS_OUT, 2018i32 => Self::TEXT_SERVER_CHANGED, 3000i32 => Self::ACCESSIBILITY_UPDATE, 3001i32 => Self::ACCESSIBILITY_INVALIDATE, 0i32 => Self::POSTINITIALIZE, 1i32 => Self::PREDELETE, 2i32 => Self::EXTENSION_RELOADED, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < Node3DNotification > for i32 {
        fn from(notification: Node3DNotification) -> i32 {
            match notification {
                Node3DNotification::TRANSFORM_CHANGED => 2000i32, Node3DNotification::ENTER_WORLD => 41i32, Node3DNotification::EXIT_WORLD => 42i32, Node3DNotification::VISIBILITY_CHANGED => 43i32, Node3DNotification::LOCAL_TRANSFORM_CHANGED => 44i32, Node3DNotification::ENTER_TREE => 10i32, Node3DNotification::EXIT_TREE => 11i32, Node3DNotification::MOVED_IN_PARENT => 12i32, Node3DNotification::READY => 13i32, Node3DNotification::PAUSED => 14i32, Node3DNotification::UNPAUSED => 15i32, Node3DNotification::PHYSICS_PROCESS => 16i32, Node3DNotification::PROCESS => 17i32, Node3DNotification::PARENTED => 18i32, Node3DNotification::UNPARENTED => 19i32, Node3DNotification::SCENE_INSTANTIATED => 20i32, Node3DNotification::DRAG_BEGIN => 21i32, Node3DNotification::DRAG_END => 22i32, Node3DNotification::PATH_RENAMED => 23i32, Node3DNotification::CHILD_ORDER_CHANGED => 24i32, Node3DNotification::INTERNAL_PROCESS => 25i32, Node3DNotification::INTERNAL_PHYSICS_PROCESS => 26i32, Node3DNotification::POST_ENTER_TREE => 27i32, Node3DNotification::DISABLED => 28i32, Node3DNotification::ENABLED => 29i32, Node3DNotification::RESET_PHYSICS_INTERPOLATION => 2001i32, Node3DNotification::EDITOR_PRE_SAVE => 9001i32, Node3DNotification::EDITOR_POST_SAVE => 9002i32, Node3DNotification::WM_MOUSE_ENTER => 1002i32, Node3DNotification::WM_MOUSE_EXIT => 1003i32, Node3DNotification::WM_WINDOW_FOCUS_IN => 1004i32, Node3DNotification::WM_WINDOW_FOCUS_OUT => 1005i32, Node3DNotification::WM_CLOSE_REQUEST => 1006i32, Node3DNotification::WM_GO_BACK_REQUEST => 1007i32, Node3DNotification::WM_SIZE_CHANGED => 1008i32, Node3DNotification::WM_DPI_CHANGE => 1009i32, Node3DNotification::VP_MOUSE_ENTER => 1010i32, Node3DNotification::VP_MOUSE_EXIT => 1011i32, Node3DNotification::WM_POSITION_CHANGED => 1012i32, Node3DNotification::OS_MEMORY_WARNING => 2009i32, Node3DNotification::TRANSLATION_CHANGED => 2010i32, Node3DNotification::WM_ABOUT => 2011i32, Node3DNotification::CRASH => 2012i32, Node3DNotification::OS_IME_UPDATE => 2013i32, Node3DNotification::APPLICATION_RESUMED => 2014i32, Node3DNotification::APPLICATION_PAUSED => 2015i32, Node3DNotification::APPLICATION_FOCUS_IN => 2016i32, Node3DNotification::APPLICATION_FOCUS_OUT => 2017i32, Node3DNotification::TEXT_SERVER_CHANGED => 2018i32, Node3DNotification::ACCESSIBILITY_UPDATE => 3000i32, Node3DNotification::ACCESSIBILITY_INVALIDATE => 3001i32, Node3DNotification::POSTINITIALIZE => 0i32, Node3DNotification::PREDELETE => 1i32, Node3DNotification::EXTENSION_RELOADED => 2i32, Node3DNotification::Unknown(int) => int,
            }
        }
    }
    impl Node3D {
        pub fn set_transform(&mut self, local: Transform3D,) {
            type CallRet = ();
            type CallParams = (Transform3D,);
            let args = (local,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5759usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transform(&self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5760usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_position(&mut self, position: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5761usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_position(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5762usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rotation(&mut self, euler_radians: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (euler_radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5763usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rotation(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5764usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rotation_degrees(&mut self, euler_degrees: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (euler_degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5765usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_rotation_degrees", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rotation_degrees(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5766usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_rotation_degrees", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rotation_order(&mut self, order: crate::global::EulerOrder,) {
            type CallRet = ();
            type CallParams = (crate::global::EulerOrder,);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5767usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_rotation_order", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rotation_order(&self,) -> crate::global::EulerOrder {
            type CallRet = crate::global::EulerOrder;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5768usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_rotation_order", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rotation_edit_mode(&mut self, edit_mode: crate::classes::node_3d::RotationEditMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::node_3d::RotationEditMode,);
            let args = (edit_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5769usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_rotation_edit_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rotation_edit_mode(&self,) -> crate::classes::node_3d::RotationEditMode {
            type CallRet = crate::classes::node_3d::RotationEditMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5770usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_rotation_edit_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scale(&mut self, scale: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5771usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scale(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5772usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_quaternion(&mut self, quaternion: Quaternion,) {
            type CallRet = ();
            type CallParams = (Quaternion,);
            let args = (quaternion,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5773usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_quaternion", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_quaternion(&self,) -> Quaternion {
            type CallRet = Quaternion;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5774usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_quaternion", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_basis(&mut self, basis: Basis,) {
            type CallRet = ();
            type CallParams = (Basis,);
            let args = (basis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5775usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_basis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_basis(&self,) -> Basis {
            type CallRet = Basis;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5776usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_basis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_global_transform(&mut self, global: Transform3D,) {
            type CallRet = ();
            type CallParams = (Transform3D,);
            let args = (global,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5777usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_global_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_transform(&self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5778usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_global_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_transform_interpolated(&mut self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5779usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_global_transform_interpolated", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_global_position(&mut self, position: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5780usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_global_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_position(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5781usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_global_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_global_basis(&mut self, basis: Basis,) {
            type CallRet = ();
            type CallParams = (Basis,);
            let args = (basis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5782usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_global_basis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_basis(&self,) -> Basis {
            type CallRet = Basis;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5783usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_global_basis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_global_rotation(&mut self, euler_radians: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (euler_radians,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5784usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_global_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_rotation(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5785usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_global_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_global_rotation_degrees(&mut self, euler_degrees: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (euler_degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5786usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_global_rotation_degrees", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_rotation_degrees(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5787usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_global_rotation_degrees", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_parent_node_3d(&self,) -> Option < Gd < crate::classes::Node3D > > {
            type CallRet = Option < Gd < crate::classes::Node3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5788usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_parent_node_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ignore_transform_notification(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5789usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_ignore_transform_notification", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_as_top_level(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5790usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_as_top_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_set_as_top_level(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5791usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "is_set_as_top_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_disable_scale(&mut self, disable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5792usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_disable_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_scale_disabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5793usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "is_scale_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_world_3d(&self,) -> Option < Gd < crate::classes::World3D > > {
            type CallRet = Option < Gd < crate::classes::World3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5794usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_world_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn force_update_transform(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5795usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "force_update_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_parent(&mut self, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5796usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_visibility_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_parent(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5797usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_visibility_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn update_gizmos(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5798usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "update_gizmos", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_gizmo(&mut self, gizmo: impl AsArg < Option < Gd < crate::classes::Node3DGizmo >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node3DGizmo > > >,);
            let args = (gizmo.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5799usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "add_gizmo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gizmos(&self,) -> Array < Gd < crate::classes::Node3DGizmo > > {
            type CallRet = Array < Gd < crate::classes::Node3DGizmo > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5800usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "get_gizmos", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_gizmos(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5801usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "clear_gizmos", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_subgizmo_selection(&mut self, gizmo: impl AsArg < Option < Gd < crate::classes::Node3DGizmo >> >, id: i32, transform: Transform3D,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node3DGizmo > > >, i32, Transform3D,);
            let args = (gizmo.into_arg(), id, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5802usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_subgizmo_selection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_subgizmo_selection(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5803usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "clear_subgizmo_selection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible(&mut self, visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5804usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5805usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "is_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_visible_in_tree(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5806usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "is_visible_in_tree", Some(self.__validated_obj()), args,)
            }
        }
        pub fn show(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5807usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "show", Some(self.__validated_obj()), args,)
            }
        }
        pub fn hide(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5808usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "hide", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_notify_local_transform(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5809usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_notify_local_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_local_transform_notification_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5810usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "is_local_transform_notification_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_notify_transform(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5811usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_notify_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_transform_notification_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5812usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "is_transform_notification_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rotate(&mut self, axis: Vector3, angle: f32,) {
            type CallRet = ();
            type CallParams = (Vector3, f32,);
            let args = (axis, angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5813usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "rotate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_rotate(&mut self, axis: Vector3, angle: f32,) {
            type CallRet = ();
            type CallParams = (Vector3, f32,);
            let args = (axis, angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5814usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "global_rotate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_scale(&mut self, scale: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5815usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "global_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_translate(&mut self, offset: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5816usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "global_translate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rotate_object_local(&mut self, axis: Vector3, angle: f32,) {
            type CallRet = ();
            type CallParams = (Vector3, f32,);
            let args = (axis, angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5817usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "rotate_object_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn scale_object_local(&mut self, scale: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5818usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "scale_object_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn translate_object_local(&mut self, offset: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5819usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "translate_object_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rotate_x(&mut self, angle: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5820usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "rotate_x", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rotate_y(&mut self, angle: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5821usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "rotate_y", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rotate_z(&mut self, angle: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5822usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "rotate_z", Some(self.__validated_obj()), args,)
            }
        }
        pub fn translate(&mut self, offset: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5823usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "translate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn orthonormalize(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5824usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "orthonormalize", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_identity(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5825usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "set_identity", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn look_at_full(&mut self, target: Vector3, up: Vector3, use_model_front: bool,) {
            type CallRet = ();
            type CallParams = (Vector3, Vector3, bool,);
            let args = (target, up, use_model_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5826usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "look_at", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::look_at_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn look_at(&mut self, target: Vector3,) {
            self.look_at_ex(target,) . done()
        }
        #[inline]
        pub fn look_at_ex < 'a > (&'a mut self, target: Vector3,) -> ExLookAt < 'a > {
            ExLookAt::new(self, target,)
        }
        pub(crate) fn look_at_from_position_full(&mut self, position: Vector3, target: Vector3, up: Vector3, use_model_front: bool,) {
            type CallRet = ();
            type CallParams = (Vector3, Vector3, Vector3, bool,);
            let args = (position, target, up, use_model_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5827usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "look_at_from_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::look_at_from_position_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn look_at_from_position(&mut self, position: Vector3, target: Vector3,) {
            self.look_at_from_position_ex(position, target,) . done()
        }
        #[inline]
        pub fn look_at_from_position_ex < 'a > (&'a mut self, position: Vector3, target: Vector3,) -> ExLookAtFromPosition < 'a > {
            ExLookAtFromPosition::new(self, position, target,)
        }
        pub fn to_local(&self, global_point: Vector3,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector3,);
            let args = (global_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5828usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "to_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn to_global(&self, local_point: Vector3,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector3,);
            let args = (local_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5829usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node3D", "to_global", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: Node3DNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: Node3DNotification) {
            self.notification(i32::from(what), true);
            
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
        pub(crate) const NOTIFICATION_TRANSFORM_CHANGED: i32 = 2000i32;
        pub(crate) const NOTIFICATION_ENTER_WORLD: i32 = 41i32;
        pub(crate) const NOTIFICATION_EXIT_WORLD: i32 = 42i32;
        pub(crate) const NOTIFICATION_VISIBILITY_CHANGED: i32 = 43i32;
        pub(crate) const NOTIFICATION_LOCAL_TRANSFORM_CHANGED: i32 = 44i32;
        
    }
    impl crate::obj::GodotClass for Node3D {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Node3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Node3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Node3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Node3D {
        
    }
    impl crate::obj::cap::GodotDefault for Node3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Node3D {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Node3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Node3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Node3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Node3D::look_at_ex`][super::Node3D::look_at_ex]."]
#[must_use]
pub struct ExLookAt < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node3D, target: Vector3, up: Vector3, use_model_front: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLookAt < 'a > {
    fn new(surround_object: &'a mut re_export::Node3D, target: Vector3,) -> Self {
        let up = Vector3::new(0 as _, 1 as _, 0 as _);
        let use_model_front = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, target: target, up: up, use_model_front: use_model_front,
        }
    }
    #[inline]
    pub fn up(self, up: Vector3) -> Self {
        Self {
            up: up, .. self
        }
    }
    #[inline]
    pub fn use_model_front(self, use_model_front: bool) -> Self {
        Self {
            use_model_front: use_model_front, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, target, up, use_model_front,
        }
        = self;
        re_export::Node3D::look_at_full(surround_object, target, up, use_model_front,)
    }
}
#[doc = "Default-param extender for [`Node3D::look_at_from_position_ex`][super::Node3D::look_at_from_position_ex]."]
#[must_use]
pub struct ExLookAtFromPosition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node3D, position: Vector3, target: Vector3, up: Vector3, use_model_front: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLookAtFromPosition < 'a > {
    fn new(surround_object: &'a mut re_export::Node3D, position: Vector3, target: Vector3,) -> Self {
        let up = Vector3::new(0 as _, 1 as _, 0 as _);
        let use_model_front = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, target: target, up: up, use_model_front: use_model_front,
        }
    }
    #[inline]
    pub fn up(self, up: Vector3) -> Self {
        Self {
            up: up, .. self
        }
    }
    #[inline]
    pub fn use_model_front(self, use_model_front: bool) -> Self {
        Self {
            use_model_front: use_model_front, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, target, up, use_model_front,
        }
        = self;
        re_export::Node3D::look_at_from_position_full(surround_object, position, target, up, use_model_front,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RotationEditMode {
    ord: i32
}
impl RotationEditMode {
    #[doc(alias = "ROTATION_EDIT_MODE_EULER")]
    #[doc = "Godot enumerator name: `ROTATION_EDIT_MODE_EULER`"]
    pub const EULER: RotationEditMode = RotationEditMode {
        ord: 0i32
    };
    #[doc(alias = "ROTATION_EDIT_MODE_QUATERNION")]
    #[doc = "Godot enumerator name: `ROTATION_EDIT_MODE_QUATERNION`"]
    pub const QUATERNION: RotationEditMode = RotationEditMode {
        ord: 1i32
    };
    #[doc(alias = "ROTATION_EDIT_MODE_BASIS")]
    #[doc = "Godot enumerator name: `ROTATION_EDIT_MODE_BASIS`"]
    pub const BASIS: RotationEditMode = RotationEditMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for RotationEditMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RotationEditMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RotationEditMode {
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
            Self::EULER => "EULER", Self::QUATERNION => "QUATERNION", Self::BASIS => "BASIS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[RotationEditMode::EULER, RotationEditMode::QUATERNION, RotationEditMode::BASIS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < RotationEditMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("EULER", "ROTATION_EDIT_MODE_EULER", RotationEditMode::EULER), crate::meta::inspect::EnumConstant::new("QUATERNION", "ROTATION_EDIT_MODE_QUATERNION", RotationEditMode::QUATERNION), crate::meta::inspect::EnumConstant::new("BASIS", "ROTATION_EDIT_MODE_BASIS", RotationEditMode::BASIS)]
        }
    }
}
impl crate::meta::GodotConvert for RotationEditMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RotationEditMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RotationEditMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Node3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Node3D`][crate::classes::Node3D] class."]
    pub struct SignalsOfNode3D < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfNode3D < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn visibility_changed(&mut self) -> SigVisibilityChanged < 'c, C > {
            SigVisibilityChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "visibility_changed")
            }
        }
    }
    type TypedSigVisibilityChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigVisibilityChanged < 'c, C: WithSignals > {
        typed: TypedSigVisibilityChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigVisibilityChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigVisibilityChanged < 'c, C > {
        type Target = TypedSigVisibilityChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigVisibilityChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Node3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfNode3D < 'c, C > {
        type Target = < < Node3D as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Node3D;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfNode3D < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Node3D;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}