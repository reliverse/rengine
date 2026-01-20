#![doc = "Sidecar module for class [`Window`][crate::classes::Window].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Window` enums](https://docs.godotengine.org/en/stable/classes/class_window.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Window.`\n\nInherits [`Viewport`][crate::classes::Viewport].\n\nRelated symbols:\n\n* [`window`][crate::classes::window]: sidecar module with related enum/flag types\n* [`IWindow`][crate::classes::IWindow]: virtual methods\n* [`SignalsOfWindow`][crate::classes::window::SignalsOfWindow]: signal collection\n* [`WindowNotification`][crate::classes::notify::WindowNotification]: notification type\n\n\nSee also [Godot docs for `Window`](https://docs.godotengine.org/en/stable/classes/class_window.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Window::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Window {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Window`][crate::classes::Window].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IViewport`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Window` methods](https://docs.godotengine.org/en/stable/classes/class_window.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWindow: crate::obj::GodotClass < Base = Window > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: WindowNotification) {
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
        fn get_contents_minimum_size(&self,) -> Vector2 {
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
    #[doc = "Notification type for class [`Window`][crate::classes::Window]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[doc = r""]
    #[doc = r" Contains the [`Unknown`][Self::Unknown] variant for forward compatibility."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum WindowNotification {
        VISIBILITY_CHANGED = 30i32, THEME_CHANGED = 32i32, ENTER_TREE = 10i32, EXIT_TREE = 11i32, MOVED_IN_PARENT = 12i32, READY = 13i32, PAUSED = 14i32, UNPAUSED = 15i32, PHYSICS_PROCESS = 16i32, PROCESS = 17i32, PARENTED = 18i32, UNPARENTED = 19i32, SCENE_INSTANTIATED = 20i32, DRAG_BEGIN = 21i32, DRAG_END = 22i32, PATH_RENAMED = 23i32, CHILD_ORDER_CHANGED = 24i32, INTERNAL_PROCESS = 25i32, INTERNAL_PHYSICS_PROCESS = 26i32, POST_ENTER_TREE = 27i32, DISABLED = 28i32, ENABLED = 29i32, RESET_PHYSICS_INTERPOLATION = 2001i32, EDITOR_PRE_SAVE = 9001i32, EDITOR_POST_SAVE = 9002i32, WM_MOUSE_ENTER = 1002i32, WM_MOUSE_EXIT = 1003i32, WM_WINDOW_FOCUS_IN = 1004i32, WM_WINDOW_FOCUS_OUT = 1005i32, WM_CLOSE_REQUEST = 1006i32, WM_GO_BACK_REQUEST = 1007i32, WM_SIZE_CHANGED = 1008i32, WM_DPI_CHANGE = 1009i32, VP_MOUSE_ENTER = 1010i32, VP_MOUSE_EXIT = 1011i32, WM_POSITION_CHANGED = 1012i32, OS_MEMORY_WARNING = 2009i32, TRANSLATION_CHANGED = 2010i32, WM_ABOUT = 2011i32, CRASH = 2012i32, OS_IME_UPDATE = 2013i32, APPLICATION_RESUMED = 2014i32, APPLICATION_PAUSED = 2015i32, APPLICATION_FOCUS_IN = 2016i32, APPLICATION_FOCUS_OUT = 2017i32, TEXT_SERVER_CHANGED = 2018i32, ACCESSIBILITY_UPDATE = 3000i32, ACCESSIBILITY_INVALIDATE = 3001i32, POSTINITIALIZE = 0i32, PREDELETE = 1i32, EXTENSION_RELOADED = 2i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        #[doc = r""]
        #[doc = r" This is also necessary if you develop an extension on a Godot version and want to be forward-compatible with newer"]
        #[doc = r" versions. If Godot adds new notifications, they will be unknown to your extension, but you can still handle them."]
        Unknown(i32),
    }
    impl From < i32 > for WindowNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                30i32 => Self::VISIBILITY_CHANGED, 32i32 => Self::THEME_CHANGED, 10i32 => Self::ENTER_TREE, 11i32 => Self::EXIT_TREE, 12i32 => Self::MOVED_IN_PARENT, 13i32 => Self::READY, 14i32 => Self::PAUSED, 15i32 => Self::UNPAUSED, 16i32 => Self::PHYSICS_PROCESS, 17i32 => Self::PROCESS, 18i32 => Self::PARENTED, 19i32 => Self::UNPARENTED, 20i32 => Self::SCENE_INSTANTIATED, 21i32 => Self::DRAG_BEGIN, 22i32 => Self::DRAG_END, 23i32 => Self::PATH_RENAMED, 24i32 => Self::CHILD_ORDER_CHANGED, 25i32 => Self::INTERNAL_PROCESS, 26i32 => Self::INTERNAL_PHYSICS_PROCESS, 27i32 => Self::POST_ENTER_TREE, 28i32 => Self::DISABLED, 29i32 => Self::ENABLED, 2001i32 => Self::RESET_PHYSICS_INTERPOLATION, 9001i32 => Self::EDITOR_PRE_SAVE, 9002i32 => Self::EDITOR_POST_SAVE, 1002i32 => Self::WM_MOUSE_ENTER, 1003i32 => Self::WM_MOUSE_EXIT, 1004i32 => Self::WM_WINDOW_FOCUS_IN, 1005i32 => Self::WM_WINDOW_FOCUS_OUT, 1006i32 => Self::WM_CLOSE_REQUEST, 1007i32 => Self::WM_GO_BACK_REQUEST, 1008i32 => Self::WM_SIZE_CHANGED, 1009i32 => Self::WM_DPI_CHANGE, 1010i32 => Self::VP_MOUSE_ENTER, 1011i32 => Self::VP_MOUSE_EXIT, 1012i32 => Self::WM_POSITION_CHANGED, 2009i32 => Self::OS_MEMORY_WARNING, 2010i32 => Self::TRANSLATION_CHANGED, 2011i32 => Self::WM_ABOUT, 2012i32 => Self::CRASH, 2013i32 => Self::OS_IME_UPDATE, 2014i32 => Self::APPLICATION_RESUMED, 2015i32 => Self::APPLICATION_PAUSED, 2016i32 => Self::APPLICATION_FOCUS_IN, 2017i32 => Self::APPLICATION_FOCUS_OUT, 2018i32 => Self::TEXT_SERVER_CHANGED, 3000i32 => Self::ACCESSIBILITY_UPDATE, 3001i32 => Self::ACCESSIBILITY_INVALIDATE, 0i32 => Self::POSTINITIALIZE, 1i32 => Self::PREDELETE, 2i32 => Self::EXTENSION_RELOADED, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < WindowNotification > for i32 {
        fn from(notification: WindowNotification) -> i32 {
            match notification {
                WindowNotification::VISIBILITY_CHANGED => 30i32, WindowNotification::THEME_CHANGED => 32i32, WindowNotification::ENTER_TREE => 10i32, WindowNotification::EXIT_TREE => 11i32, WindowNotification::MOVED_IN_PARENT => 12i32, WindowNotification::READY => 13i32, WindowNotification::PAUSED => 14i32, WindowNotification::UNPAUSED => 15i32, WindowNotification::PHYSICS_PROCESS => 16i32, WindowNotification::PROCESS => 17i32, WindowNotification::PARENTED => 18i32, WindowNotification::UNPARENTED => 19i32, WindowNotification::SCENE_INSTANTIATED => 20i32, WindowNotification::DRAG_BEGIN => 21i32, WindowNotification::DRAG_END => 22i32, WindowNotification::PATH_RENAMED => 23i32, WindowNotification::CHILD_ORDER_CHANGED => 24i32, WindowNotification::INTERNAL_PROCESS => 25i32, WindowNotification::INTERNAL_PHYSICS_PROCESS => 26i32, WindowNotification::POST_ENTER_TREE => 27i32, WindowNotification::DISABLED => 28i32, WindowNotification::ENABLED => 29i32, WindowNotification::RESET_PHYSICS_INTERPOLATION => 2001i32, WindowNotification::EDITOR_PRE_SAVE => 9001i32, WindowNotification::EDITOR_POST_SAVE => 9002i32, WindowNotification::WM_MOUSE_ENTER => 1002i32, WindowNotification::WM_MOUSE_EXIT => 1003i32, WindowNotification::WM_WINDOW_FOCUS_IN => 1004i32, WindowNotification::WM_WINDOW_FOCUS_OUT => 1005i32, WindowNotification::WM_CLOSE_REQUEST => 1006i32, WindowNotification::WM_GO_BACK_REQUEST => 1007i32, WindowNotification::WM_SIZE_CHANGED => 1008i32, WindowNotification::WM_DPI_CHANGE => 1009i32, WindowNotification::VP_MOUSE_ENTER => 1010i32, WindowNotification::VP_MOUSE_EXIT => 1011i32, WindowNotification::WM_POSITION_CHANGED => 1012i32, WindowNotification::OS_MEMORY_WARNING => 2009i32, WindowNotification::TRANSLATION_CHANGED => 2010i32, WindowNotification::WM_ABOUT => 2011i32, WindowNotification::CRASH => 2012i32, WindowNotification::OS_IME_UPDATE => 2013i32, WindowNotification::APPLICATION_RESUMED => 2014i32, WindowNotification::APPLICATION_PAUSED => 2015i32, WindowNotification::APPLICATION_FOCUS_IN => 2016i32, WindowNotification::APPLICATION_FOCUS_OUT => 2017i32, WindowNotification::TEXT_SERVER_CHANGED => 2018i32, WindowNotification::ACCESSIBILITY_UPDATE => 3000i32, WindowNotification::ACCESSIBILITY_INVALIDATE => 3001i32, WindowNotification::POSTINITIALIZE => 0i32, WindowNotification::PREDELETE => 1i32, WindowNotification::EXTENSION_RELOADED => 2i32, WindowNotification::Unknown(int) => int,
            }
        }
    }
    impl Window {
        pub fn set_title(&mut self, title: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (title.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10987usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_title", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_title(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10988usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_title", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_initial_position(&mut self, initial_position: crate::classes::window::WindowInitialPosition,) {
            type CallRet = ();
            type CallParams = (crate::classes::window::WindowInitialPosition,);
            let args = (initial_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10989usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_initial_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_initial_position(&self,) -> crate::classes::window::WindowInitialPosition {
            type CallRet = crate::classes::window::WindowInitialPosition;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10990usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_initial_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_current_screen(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10991usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_current_screen", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_screen(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10992usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_current_screen", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_position(&mut self, position: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10993usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_position(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10994usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn move_to_center(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10995usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "move_to_center", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_size(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10996usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10997usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reset_size(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10998usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "reset_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_position_with_decorations(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10999usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_position_with_decorations", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size_with_decorations(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11000usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_size_with_decorations", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_size(&mut self, max_size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (max_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11001usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_max_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11002usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_max_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_min_size(&mut self, min_size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (min_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11003usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_min_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_min_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11004usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_min_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mode(&mut self, mode: crate::classes::window::Mode,) {
            type CallRet = ();
            type CallParams = (crate::classes::window::Mode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11005usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mode(&self,) -> crate::classes::window::Mode {
            type CallRet = crate::classes::window::Mode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11006usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flag(&mut self, flag: crate::classes::window::Flags, enabled: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::window::Flags, bool,);
            let args = (flag, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11007usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_flag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_flag(&self, flag: crate::classes::window::Flags,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::window::Flags,);
            let args = (flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11008usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_flag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_maximize_allowed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11009usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "is_maximize_allowed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn request_attention(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11010usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "request_attention", Some(self.__validated_obj()), args,)
            }
        }
        pub fn move_to_foreground(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11011usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "move_to_foreground", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible(&mut self, visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11012usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11013usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "is_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn hide(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11014usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "hide", Some(self.__validated_obj()), args,)
            }
        }
        pub fn show(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11015usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "show", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transient(&mut self, transient: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (transient,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11016usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_transient", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_transient(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11017usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "is_transient", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transient_to_focused(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11018usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_transient_to_focused", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_transient_to_focused(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11019usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "is_transient_to_focused", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_exclusive(&mut self, exclusive: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (exclusive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11020usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_exclusive", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_exclusive(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11021usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "is_exclusive", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_unparent_when_invisible(&mut self, unparent: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (unparent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11022usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_unparent_when_invisible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn can_draw(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11023usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "can_draw", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_focus(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11024usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "has_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn grab_focus(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11025usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "grab_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn start_drag(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11026usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "start_drag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn start_resize(&mut self, edge: crate::classes::display_server::WindowResizeEdge,) {
            type CallRet = ();
            type CallParams = (crate::classes::display_server::WindowResizeEdge,);
            let args = (edge,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11027usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "start_resize", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ime_active(&mut self, active: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11028usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_ime_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ime_position(&mut self, position: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11029usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_ime_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_embedded(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11030usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "is_embedded", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contents_minimum_size(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11031usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_contents_minimum_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_force_native(&mut self, force_native: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (force_native,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11032usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_force_native", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_force_native(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11033usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_force_native", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_content_scale_size(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11034usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_content_scale_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_content_scale_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11035usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_content_scale_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_content_scale_mode(&mut self, mode: crate::classes::window::ContentScaleMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::window::ContentScaleMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11036usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_content_scale_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_content_scale_mode(&self,) -> crate::classes::window::ContentScaleMode {
            type CallRet = crate::classes::window::ContentScaleMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11037usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_content_scale_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_content_scale_aspect(&mut self, aspect: crate::classes::window::ContentScaleAspect,) {
            type CallRet = ();
            type CallParams = (crate::classes::window::ContentScaleAspect,);
            let args = (aspect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11038usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_content_scale_aspect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_content_scale_aspect(&self,) -> crate::classes::window::ContentScaleAspect {
            type CallRet = crate::classes::window::ContentScaleAspect;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11039usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_content_scale_aspect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_content_scale_stretch(&mut self, stretch: crate::classes::window::ContentScaleStretch,) {
            type CallRet = ();
            type CallParams = (crate::classes::window::ContentScaleStretch,);
            let args = (stretch,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11040usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_content_scale_stretch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_content_scale_stretch(&self,) -> crate::classes::window::ContentScaleStretch {
            type CallRet = crate::classes::window::ContentScaleStretch;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11041usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_content_scale_stretch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_keep_title_visible(&mut self, title_visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (title_visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11042usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_keep_title_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_keep_title_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11043usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_keep_title_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_content_scale_factor(&mut self, factor: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (factor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11044usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_content_scale_factor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_content_scale_factor(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11045usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_content_scale_factor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mouse_passthrough_polygon(&mut self, polygon: &PackedVector2Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >,);
            let args = (RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11046usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_mouse_passthrough_polygon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mouse_passthrough_polygon(&self,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11047usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_mouse_passthrough_polygon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_wrap_controls(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11048usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_wrap_controls", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_wrapping_controls(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11049usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "is_wrapping_controls", Some(self.__validated_obj()), args,)
            }
        }
        pub fn child_controls_changed(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11050usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "child_controls_changed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_theme(&mut self, theme: impl AsArg < Option < Gd < crate::classes::Theme >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Theme > > >,);
            let args = (theme.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11051usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_theme", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_theme(&self,) -> Option < Gd < crate::classes::Theme > > {
            type CallRet = Option < Gd < crate::classes::Theme > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11052usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_theme", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_theme_type_variation(&mut self, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11053usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_theme_type_variation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_theme_type_variation(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11054usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_theme_type_variation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn begin_bulk_theme_override(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11055usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "begin_bulk_theme_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn end_bulk_theme_override(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11056usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "end_bulk_theme_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_theme_icon_override(&mut self, name: impl AsArg < StringName >, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (name.into_arg(), texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11057usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "add_theme_icon_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_theme_stylebox_override(&mut self, name: impl AsArg < StringName >, stylebox: impl AsArg < Option < Gd < crate::classes::StyleBox >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::StyleBox > > >,);
            let args = (name.into_arg(), stylebox.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11058usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "add_theme_stylebox_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_theme_font_override(&mut self, name: impl AsArg < StringName >, font: impl AsArg < Option < Gd < crate::classes::Font >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::Font > > >,);
            let args = (name.into_arg(), font.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11059usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "add_theme_font_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_theme_font_size_override(&mut self, name: impl AsArg < StringName >, font_size: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, i32,);
            let args = (name.into_arg(), font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11060usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "add_theme_font_size_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_theme_color_override(&mut self, name: impl AsArg < StringName >, color: Color,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, Color,);
            let args = (name.into_arg(), color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11061usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "add_theme_color_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_theme_constant_override(&mut self, name: impl AsArg < StringName >, constant: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, i32,);
            let args = (name.into_arg(), constant,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11062usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "add_theme_constant_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_theme_icon_override(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11063usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "remove_theme_icon_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_theme_stylebox_override(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11064usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "remove_theme_stylebox_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_theme_font_override(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11065usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "remove_theme_font_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_theme_font_size_override(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11066usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "remove_theme_font_size_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_theme_color_override(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11067usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "remove_theme_color_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_theme_constant_override(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11068usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "remove_theme_constant_override", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_theme_icon_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11069usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_theme_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_theme_icon_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_theme_icon(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Texture2D > > {
            self.get_theme_icon_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_icon_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExGetThemeIcon < 'a > {
            ExGetThemeIcon::new(self, name,)
        }
        pub(crate) fn get_theme_stylebox_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> Option < Gd < crate::classes::StyleBox > > {
            type CallRet = Option < Gd < crate::classes::StyleBox > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11070usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_theme_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_theme_stylebox_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_theme_stylebox(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::StyleBox > > {
            self.get_theme_stylebox_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_stylebox_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExGetThemeStylebox < 'a > {
            ExGetThemeStylebox::new(self, name,)
        }
        pub(crate) fn get_theme_font_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> Option < Gd < crate::classes::Font > > {
            type CallRet = Option < Gd < crate::classes::Font > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11071usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_theme_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_theme_font_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_theme_font(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Font > > {
            self.get_theme_font_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_font_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExGetThemeFont < 'a > {
            ExGetThemeFont::new(self, name,)
        }
        pub(crate) fn get_theme_font_size_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11072usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_theme_font_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_theme_font_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_theme_font_size(&self, name: impl AsArg < StringName >,) -> i32 {
            self.get_theme_font_size_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_font_size_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExGetThemeFontSize < 'a > {
            ExGetThemeFontSize::new(self, name,)
        }
        pub(crate) fn get_theme_color_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> Color {
            type CallRet = Color;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11073usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_theme_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_theme_color_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_theme_color(&self, name: impl AsArg < StringName >,) -> Color {
            self.get_theme_color_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_color_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExGetThemeColor < 'a > {
            ExGetThemeColor::new(self, name,)
        }
        pub(crate) fn get_theme_constant_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11074usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_theme_constant", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_theme_constant_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_theme_constant(&self, name: impl AsArg < StringName >,) -> i32 {
            self.get_theme_constant_ex(name,) . done()
        }
        #[inline]
        pub fn get_theme_constant_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExGetThemeConstant < 'a > {
            ExGetThemeConstant::new(self, name,)
        }
        pub fn has_theme_icon_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11075usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "has_theme_icon_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_theme_stylebox_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11076usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "has_theme_stylebox_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_theme_font_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11077usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "has_theme_font_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_theme_font_size_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11078usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "has_theme_font_size_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_theme_color_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11079usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "has_theme_color_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_theme_constant_override(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11080usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "has_theme_constant_override", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn has_theme_icon_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11081usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "has_theme_icon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::has_theme_icon_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn has_theme_icon(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_icon_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_icon_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExHasThemeIcon < 'a > {
            ExHasThemeIcon::new(self, name,)
        }
        pub(crate) fn has_theme_stylebox_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11082usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "has_theme_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::has_theme_stylebox_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn has_theme_stylebox(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_stylebox_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_stylebox_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExHasThemeStylebox < 'a > {
            ExHasThemeStylebox::new(self, name,)
        }
        pub(crate) fn has_theme_font_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11083usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "has_theme_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::has_theme_font_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn has_theme_font(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_font_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_font_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExHasThemeFont < 'a > {
            ExHasThemeFont::new(self, name,)
        }
        pub(crate) fn has_theme_font_size_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11084usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "has_theme_font_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::has_theme_font_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn has_theme_font_size(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_font_size_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_font_size_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExHasThemeFontSize < 'a > {
            ExHasThemeFontSize::new(self, name,)
        }
        pub(crate) fn has_theme_color_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11085usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "has_theme_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::has_theme_color_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn has_theme_color(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_color_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_color_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExHasThemeColor < 'a > {
            ExHasThemeColor::new(self, name,)
        }
        pub(crate) fn has_theme_constant_full(&self, name: CowArg < StringName >, theme_type: CowArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name, theme_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11086usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "has_theme_constant", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::has_theme_constant_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn has_theme_constant(&self, name: impl AsArg < StringName >,) -> bool {
            self.has_theme_constant_ex(name,) . done()
        }
        #[inline]
        pub fn has_theme_constant_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExHasThemeConstant < 'a > {
            ExHasThemeConstant::new(self, name,)
        }
        pub fn get_theme_default_base_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11087usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_theme_default_base_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_theme_default_font(&self,) -> Option < Gd < crate::classes::Font > > {
            type CallRet = Option < Gd < crate::classes::Font > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11088usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_theme_default_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_theme_default_font_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11089usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_theme_default_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_window_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11090usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_window_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_accessibility_name(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11091usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_accessibility_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_accessibility_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11092usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_accessibility_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_accessibility_description(&mut self, description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (description.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11093usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_accessibility_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_accessibility_description(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11094usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_accessibility_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_focused_window() -> Option < Gd < crate::classes::Window > > {
            type CallRet = Option < Gd < crate::classes::Window > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11095usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_focused_window", None, args,)
            }
        }
        pub fn set_layout_direction(&mut self, direction: crate::classes::window::LayoutDirection,) {
            type CallRet = ();
            type CallParams = (crate::classes::window::LayoutDirection,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11096usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_layout_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_layout_direction(&self,) -> crate::classes::window::LayoutDirection {
            type CallRet = crate::classes::window::LayoutDirection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11097usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "get_layout_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_layout_rtl(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11098usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "is_layout_rtl", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_translate(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11099usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_auto_translate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_auto_translating(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "is_auto_translating", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_font_oversampling(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11101usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "set_use_font_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_font_oversampling(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11102usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "is_using_font_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn popup_full(&mut self, rect: Rect2i,) {
            type CallRet = ();
            type CallParams = (Rect2i,);
            let args = (rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "popup", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup(&mut self,) {
            self.popup_ex() . done()
        }
        #[inline]
        pub fn popup_ex < 'a > (&'a mut self,) -> ExPopup < 'a > {
            ExPopup::new(self,)
        }
        pub fn popup_on_parent(&mut self, parent_rect: Rect2i,) {
            type CallRet = ();
            type CallParams = (Rect2i,);
            let args = (parent_rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "popup_on_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn popup_centered_full(&mut self, minsize: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (minsize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "popup_centered", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_centered_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_centered(&mut self,) {
            self.popup_centered_ex() . done()
        }
        #[inline]
        pub fn popup_centered_ex < 'a > (&'a mut self,) -> ExPopupCentered < 'a > {
            ExPopupCentered::new(self,)
        }
        pub(crate) fn popup_centered_ratio_full(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "popup_centered_ratio", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_centered_ratio_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_centered_ratio(&mut self,) {
            self.popup_centered_ratio_ex() . done()
        }
        #[inline]
        pub fn popup_centered_ratio_ex < 'a > (&'a mut self,) -> ExPopupCenteredRatio < 'a > {
            ExPopupCenteredRatio::new(self,)
        }
        pub(crate) fn popup_centered_clamped_full(&mut self, minsize: Vector2i, fallback_ratio: f32,) {
            type CallRet = ();
            type CallParams = (Vector2i, f32,);
            let args = (minsize, fallback_ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "popup_centered_clamped", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_centered_clamped_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_centered_clamped(&mut self,) {
            self.popup_centered_clamped_ex() . done()
        }
        #[inline]
        pub fn popup_centered_clamped_ex < 'a > (&'a mut self,) -> ExPopupCenteredClamped < 'a > {
            ExPopupCenteredClamped::new(self,)
        }
        pub(crate) fn popup_exclusive_full(&mut self, from_node: CowArg < Option < Gd < crate::classes::Node > > >, rect: Rect2i,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >, Rect2i,);
            let args = (from_node, rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "popup_exclusive", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_exclusive_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_exclusive(&mut self, from_node: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            self.popup_exclusive_ex(from_node,) . done()
        }
        #[inline]
        pub fn popup_exclusive_ex < 'a > (&'a mut self, from_node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> ExPopupExclusive < 'a > {
            ExPopupExclusive::new(self, from_node,)
        }
        pub fn popup_exclusive_on_parent(&mut self, from_node: impl AsArg < Option < Gd < crate::classes::Node >> >, parent_rect: Rect2i,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >, Rect2i,);
            let args = (from_node.into_arg(), parent_rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "popup_exclusive_on_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn popup_exclusive_centered_full(&mut self, from_node: CowArg < Option < Gd < crate::classes::Node > > >, minsize: Vector2i,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >, Vector2i,);
            let args = (from_node, minsize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "popup_exclusive_centered", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_exclusive_centered_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_exclusive_centered(&mut self, from_node: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            self.popup_exclusive_centered_ex(from_node,) . done()
        }
        #[inline]
        pub fn popup_exclusive_centered_ex < 'a > (&'a mut self, from_node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> ExPopupExclusiveCentered < 'a > {
            ExPopupExclusiveCentered::new(self, from_node,)
        }
        pub(crate) fn popup_exclusive_centered_ratio_full(&mut self, from_node: CowArg < Option < Gd < crate::classes::Node > > >, ratio: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >, f32,);
            let args = (from_node, ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "popup_exclusive_centered_ratio", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_exclusive_centered_ratio_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_exclusive_centered_ratio(&mut self, from_node: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            self.popup_exclusive_centered_ratio_ex(from_node,) . done()
        }
        #[inline]
        pub fn popup_exclusive_centered_ratio_ex < 'a > (&'a mut self, from_node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> ExPopupExclusiveCenteredRatio < 'a > {
            ExPopupExclusiveCenteredRatio::new(self, from_node,)
        }
        pub(crate) fn popup_exclusive_centered_clamped_full(&mut self, from_node: CowArg < Option < Gd < crate::classes::Node > > >, minsize: Vector2i, fallback_ratio: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >, Vector2i, f32,);
            let args = (from_node, minsize, fallback_ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Window", "popup_exclusive_centered_clamped", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_exclusive_centered_clamped_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_exclusive_centered_clamped(&mut self, from_node: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            self.popup_exclusive_centered_clamped_ex(from_node,) . done()
        }
        #[inline]
        pub fn popup_exclusive_centered_clamped_ex < 'a > (&'a mut self, from_node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> ExPopupExclusiveCenteredClamped < 'a > {
            ExPopupExclusiveCenteredClamped::new(self, from_node,)
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
        pub fn notify(&mut self, what: WindowNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: WindowNotification) {
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
        pub(crate) const NOTIFICATION_VISIBILITY_CHANGED: i32 = 30i32;
        pub(crate) const NOTIFICATION_THEME_CHANGED: i32 = 32i32;
        
    }
    impl crate::obj::GodotClass for Window {
        type Base = crate::classes::Viewport;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Window"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Window {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Viewport > for Window {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Window {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Window {
        
    }
    impl crate::obj::cap::GodotDefault for Window {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Window {
        type Target = crate::classes::Viewport;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Window {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Window`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Window__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Window > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Viewport > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Window::get_theme_icon_ex`][super::Window::get_theme_icon_ex]."]
#[must_use]
pub struct ExGetThemeIcon < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Window, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeIcon < 'a > {
    fn new(surround_object: &'a re_export::Window, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Texture2D > > {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Window::get_theme_icon_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::get_theme_stylebox_ex`][super::Window::get_theme_stylebox_ex]."]
#[must_use]
pub struct ExGetThemeStylebox < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Window, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeStylebox < 'a > {
    fn new(surround_object: &'a re_export::Window, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::StyleBox > > {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Window::get_theme_stylebox_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::get_theme_font_ex`][super::Window::get_theme_font_ex]."]
#[must_use]
pub struct ExGetThemeFont < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Window, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeFont < 'a > {
    fn new(surround_object: &'a re_export::Window, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Font > > {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Window::get_theme_font_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::get_theme_font_size_ex`][super::Window::get_theme_font_size_ex]."]
#[must_use]
pub struct ExGetThemeFontSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Window, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeFontSize < 'a > {
    fn new(surround_object: &'a re_export::Window, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Window::get_theme_font_size_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::get_theme_color_ex`][super::Window::get_theme_color_ex]."]
#[must_use]
pub struct ExGetThemeColor < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Window, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeColor < 'a > {
    fn new(surround_object: &'a re_export::Window, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Color {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Window::get_theme_color_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::get_theme_constant_ex`][super::Window::get_theme_constant_ex]."]
#[must_use]
pub struct ExGetThemeConstant < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Window, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetThemeConstant < 'a > {
    fn new(surround_object: &'a re_export::Window, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Window::get_theme_constant_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::has_theme_icon_ex`][super::Window::has_theme_icon_ex]."]
#[must_use]
pub struct ExHasThemeIcon < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Window, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeIcon < 'a > {
    fn new(surround_object: &'a re_export::Window, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Window::has_theme_icon_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::has_theme_stylebox_ex`][super::Window::has_theme_stylebox_ex]."]
#[must_use]
pub struct ExHasThemeStylebox < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Window, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeStylebox < 'a > {
    fn new(surround_object: &'a re_export::Window, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Window::has_theme_stylebox_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::has_theme_font_ex`][super::Window::has_theme_font_ex]."]
#[must_use]
pub struct ExHasThemeFont < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Window, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeFont < 'a > {
    fn new(surround_object: &'a re_export::Window, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Window::has_theme_font_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::has_theme_font_size_ex`][super::Window::has_theme_font_size_ex]."]
#[must_use]
pub struct ExHasThemeFontSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Window, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeFontSize < 'a > {
    fn new(surround_object: &'a re_export::Window, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Window::has_theme_font_size_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::has_theme_color_ex`][super::Window::has_theme_color_ex]."]
#[must_use]
pub struct ExHasThemeColor < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Window, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeColor < 'a > {
    fn new(surround_object: &'a re_export::Window, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Window::has_theme_color_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::has_theme_constant_ex`][super::Window::has_theme_constant_ex]."]
#[must_use]
pub struct ExHasThemeConstant < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Window, name: CowArg < 'a, StringName >, theme_type: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasThemeConstant < 'a > {
    fn new(surround_object: &'a re_export::Window, name: impl AsArg < StringName > + 'a,) -> Self {
        let theme_type = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), theme_type: CowArg::Owned(theme_type),
        }
    }
    #[inline]
    pub fn theme_type(self, theme_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            theme_type: theme_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, name, theme_type,
        }
        = self;
        re_export::Window::has_theme_constant_full(surround_object, name, theme_type,)
    }
}
#[doc = "Default-param extender for [`Window::popup_ex`][super::Window::popup_ex]."]
#[must_use]
pub struct ExPopup < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Window, rect: Rect2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopup < 'a > {
    fn new(surround_object: &'a mut re_export::Window,) -> Self {
        let rect = Rect2i::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rect: rect,
        }
    }
    #[inline]
    pub fn rect(self, rect: Rect2i) -> Self {
        Self {
            rect: rect, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, rect,
        }
        = self;
        re_export::Window::popup_full(surround_object, rect,)
    }
}
#[doc = "Default-param extender for [`Window::popup_centered_ex`][super::Window::popup_centered_ex]."]
#[must_use]
pub struct ExPopupCentered < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Window, minsize: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupCentered < 'a > {
    fn new(surround_object: &'a mut re_export::Window,) -> Self {
        let minsize = Vector2i::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, minsize: minsize,
        }
    }
    #[inline]
    pub fn minsize(self, minsize: Vector2i) -> Self {
        Self {
            minsize: minsize, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, minsize,
        }
        = self;
        re_export::Window::popup_centered_full(surround_object, minsize,)
    }
}
#[doc = "Default-param extender for [`Window::popup_centered_ratio_ex`][super::Window::popup_centered_ratio_ex]."]
#[must_use]
pub struct ExPopupCenteredRatio < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Window, ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupCenteredRatio < 'a > {
    fn new(surround_object: &'a mut re_export::Window,) -> Self {
        let ratio = 0.8f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, ratio: ratio,
        }
    }
    #[inline]
    pub fn ratio(self, ratio: f32) -> Self {
        Self {
            ratio: ratio, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, ratio,
        }
        = self;
        re_export::Window::popup_centered_ratio_full(surround_object, ratio,)
    }
}
#[doc = "Default-param extender for [`Window::popup_centered_clamped_ex`][super::Window::popup_centered_clamped_ex]."]
#[must_use]
pub struct ExPopupCenteredClamped < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Window, minsize: Vector2i, fallback_ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupCenteredClamped < 'a > {
    fn new(surround_object: &'a mut re_export::Window,) -> Self {
        let minsize = Vector2i::new(0 as _, 0 as _);
        let fallback_ratio = 0.75f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, minsize: minsize, fallback_ratio: fallback_ratio,
        }
    }
    #[inline]
    pub fn minsize(self, minsize: Vector2i) -> Self {
        Self {
            minsize: minsize, .. self
        }
    }
    #[inline]
    pub fn fallback_ratio(self, fallback_ratio: f32) -> Self {
        Self {
            fallback_ratio: fallback_ratio, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, minsize, fallback_ratio,
        }
        = self;
        re_export::Window::popup_centered_clamped_full(surround_object, minsize, fallback_ratio,)
    }
}
#[doc = "Default-param extender for [`Window::popup_exclusive_ex`][super::Window::popup_exclusive_ex]."]
#[must_use]
pub struct ExPopupExclusive < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Window, from_node: CowArg < 'a, Option < Gd < crate::classes::Node > > >, rect: Rect2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupExclusive < 'a > {
    fn new(surround_object: &'a mut re_export::Window, from_node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> Self {
        let rect = Rect2i::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_node: from_node.into_arg(), rect: rect,
        }
    }
    #[inline]
    pub fn rect(self, rect: Rect2i) -> Self {
        Self {
            rect: rect, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from_node, rect,
        }
        = self;
        re_export::Window::popup_exclusive_full(surround_object, from_node, rect,)
    }
}
#[doc = "Default-param extender for [`Window::popup_exclusive_centered_ex`][super::Window::popup_exclusive_centered_ex]."]
#[must_use]
pub struct ExPopupExclusiveCentered < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Window, from_node: CowArg < 'a, Option < Gd < crate::classes::Node > > >, minsize: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupExclusiveCentered < 'a > {
    fn new(surround_object: &'a mut re_export::Window, from_node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> Self {
        let minsize = Vector2i::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_node: from_node.into_arg(), minsize: minsize,
        }
    }
    #[inline]
    pub fn minsize(self, minsize: Vector2i) -> Self {
        Self {
            minsize: minsize, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from_node, minsize,
        }
        = self;
        re_export::Window::popup_exclusive_centered_full(surround_object, from_node, minsize,)
    }
}
#[doc = "Default-param extender for [`Window::popup_exclusive_centered_ratio_ex`][super::Window::popup_exclusive_centered_ratio_ex]."]
#[must_use]
pub struct ExPopupExclusiveCenteredRatio < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Window, from_node: CowArg < 'a, Option < Gd < crate::classes::Node > > >, ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupExclusiveCenteredRatio < 'a > {
    fn new(surround_object: &'a mut re_export::Window, from_node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> Self {
        let ratio = 0.8f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_node: from_node.into_arg(), ratio: ratio,
        }
    }
    #[inline]
    pub fn ratio(self, ratio: f32) -> Self {
        Self {
            ratio: ratio, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from_node, ratio,
        }
        = self;
        re_export::Window::popup_exclusive_centered_ratio_full(surround_object, from_node, ratio,)
    }
}
#[doc = "Default-param extender for [`Window::popup_exclusive_centered_clamped_ex`][super::Window::popup_exclusive_centered_clamped_ex]."]
#[must_use]
pub struct ExPopupExclusiveCenteredClamped < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Window, from_node: CowArg < 'a, Option < Gd < crate::classes::Node > > >, minsize: Vector2i, fallback_ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupExclusiveCenteredClamped < 'a > {
    fn new(surround_object: &'a mut re_export::Window, from_node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> Self {
        let minsize = Vector2i::new(0 as _, 0 as _);
        let fallback_ratio = 0.75f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_node: from_node.into_arg(), minsize: minsize, fallback_ratio: fallback_ratio,
        }
    }
    #[inline]
    pub fn minsize(self, minsize: Vector2i) -> Self {
        Self {
            minsize: minsize, .. self
        }
    }
    #[inline]
    pub fn fallback_ratio(self, fallback_ratio: f32) -> Self {
        Self {
            fallback_ratio: fallback_ratio, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from_node, minsize, fallback_ratio,
        }
        = self;
        re_export::Window::popup_exclusive_centered_clamped_full(surround_object, from_node, minsize, fallback_ratio,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Mode {
    ord: i32
}
impl Mode {
    #[doc(alias = "MODE_WINDOWED")]
    #[doc = "Godot enumerator name: `MODE_WINDOWED`"]
    pub const WINDOWED: Mode = Mode {
        ord: 0i32
    };
    #[doc(alias = "MODE_MINIMIZED")]
    #[doc = "Godot enumerator name: `MODE_MINIMIZED`"]
    pub const MINIMIZED: Mode = Mode {
        ord: 1i32
    };
    #[doc(alias = "MODE_MAXIMIZED")]
    #[doc = "Godot enumerator name: `MODE_MAXIMIZED`"]
    pub const MAXIMIZED: Mode = Mode {
        ord: 2i32
    };
    #[doc(alias = "MODE_FULLSCREEN")]
    #[doc = "Godot enumerator name: `MODE_FULLSCREEN`"]
    pub const FULLSCREEN: Mode = Mode {
        ord: 3i32
    };
    #[doc(alias = "MODE_EXCLUSIVE_FULLSCREEN")]
    #[doc = "Godot enumerator name: `MODE_EXCLUSIVE_FULLSCREEN`"]
    pub const EXCLUSIVE_FULLSCREEN: Mode = Mode {
        ord: 4i32
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
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::WINDOWED => "WINDOWED", Self::MINIMIZED => "MINIMIZED", Self::MAXIMIZED => "MAXIMIZED", Self::FULLSCREEN => "FULLSCREEN", Self::EXCLUSIVE_FULLSCREEN => "EXCLUSIVE_FULLSCREEN", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Mode::WINDOWED, Mode::MINIMIZED, Mode::MAXIMIZED, Mode::FULLSCREEN, Mode::EXCLUSIVE_FULLSCREEN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Mode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("WINDOWED", "MODE_WINDOWED", Mode::WINDOWED), crate::meta::inspect::EnumConstant::new("MINIMIZED", "MODE_MINIMIZED", Mode::MINIMIZED), crate::meta::inspect::EnumConstant::new("MAXIMIZED", "MODE_MAXIMIZED", Mode::MAXIMIZED), crate::meta::inspect::EnumConstant::new("FULLSCREEN", "MODE_FULLSCREEN", Mode::FULLSCREEN), crate::meta::inspect::EnumConstant::new("EXCLUSIVE_FULLSCREEN", "MODE_EXCLUSIVE_FULLSCREEN", Mode::EXCLUSIVE_FULLSCREEN)]
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Flags {
    ord: i32
}
impl Flags {
    #[doc(alias = "FLAG_RESIZE_DISABLED")]
    #[doc = "Godot enumerator name: `FLAG_RESIZE_DISABLED`"]
    pub const RESIZE_DISABLED: Flags = Flags {
        ord: 0i32
    };
    #[doc(alias = "FLAG_BORDERLESS")]
    #[doc = "Godot enumerator name: `FLAG_BORDERLESS`"]
    pub const BORDERLESS: Flags = Flags {
        ord: 1i32
    };
    #[doc(alias = "FLAG_ALWAYS_ON_TOP")]
    #[doc = "Godot enumerator name: `FLAG_ALWAYS_ON_TOP`"]
    pub const ALWAYS_ON_TOP: Flags = Flags {
        ord: 2i32
    };
    #[doc(alias = "FLAG_TRANSPARENT")]
    #[doc = "Godot enumerator name: `FLAG_TRANSPARENT`"]
    pub const TRANSPARENT: Flags = Flags {
        ord: 3i32
    };
    #[doc(alias = "FLAG_NO_FOCUS")]
    #[doc = "Godot enumerator name: `FLAG_NO_FOCUS`"]
    pub const NO_FOCUS: Flags = Flags {
        ord: 4i32
    };
    #[doc(alias = "FLAG_POPUP")]
    #[doc = "Godot enumerator name: `FLAG_POPUP`"]
    pub const POPUP: Flags = Flags {
        ord: 5i32
    };
    #[doc(alias = "FLAG_EXTEND_TO_TITLE")]
    #[doc = "Godot enumerator name: `FLAG_EXTEND_TO_TITLE`"]
    pub const EXTEND_TO_TITLE: Flags = Flags {
        ord: 6i32
    };
    #[doc(alias = "FLAG_MOUSE_PASSTHROUGH")]
    #[doc = "Godot enumerator name: `FLAG_MOUSE_PASSTHROUGH`"]
    pub const MOUSE_PASSTHROUGH: Flags = Flags {
        ord: 7i32
    };
    #[doc(alias = "FLAG_SHARP_CORNERS")]
    #[doc = "Godot enumerator name: `FLAG_SHARP_CORNERS`"]
    pub const SHARP_CORNERS: Flags = Flags {
        ord: 8i32
    };
    #[doc(alias = "FLAG_EXCLUDE_FROM_CAPTURE")]
    #[doc = "Godot enumerator name: `FLAG_EXCLUDE_FROM_CAPTURE`"]
    pub const EXCLUDE_FROM_CAPTURE: Flags = Flags {
        ord: 9i32
    };
    #[doc(alias = "FLAG_POPUP_WM_HINT")]
    #[doc = "Godot enumerator name: `FLAG_POPUP_WM_HINT`"]
    pub const POPUP_WM_HINT: Flags = Flags {
        ord: 10i32
    };
    #[doc(alias = "FLAG_MINIMIZE_DISABLED")]
    #[doc = "Godot enumerator name: `FLAG_MINIMIZE_DISABLED`"]
    pub const MINIMIZE_DISABLED: Flags = Flags {
        ord: 11i32
    };
    #[doc(alias = "FLAG_MAXIMIZE_DISABLED")]
    #[doc = "Godot enumerator name: `FLAG_MAXIMIZE_DISABLED`"]
    pub const MAXIMIZE_DISABLED: Flags = Flags {
        ord: 12i32
    };
    #[doc(alias = "FLAG_MAX")]
    #[doc = "Godot enumerator name: `FLAG_MAX`"]
    pub const MAX: Flags = Flags {
        ord: 13i32
    };
    
}
impl std::fmt::Debug for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Flags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Flags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
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
            Self::RESIZE_DISABLED => "RESIZE_DISABLED", Self::BORDERLESS => "BORDERLESS", Self::ALWAYS_ON_TOP => "ALWAYS_ON_TOP", Self::TRANSPARENT => "TRANSPARENT", Self::NO_FOCUS => "NO_FOCUS", Self::POPUP => "POPUP", Self::EXTEND_TO_TITLE => "EXTEND_TO_TITLE", Self::MOUSE_PASSTHROUGH => "MOUSE_PASSTHROUGH", Self::SHARP_CORNERS => "SHARP_CORNERS", Self::EXCLUDE_FROM_CAPTURE => "EXCLUDE_FROM_CAPTURE", Self::POPUP_WM_HINT => "POPUP_WM_HINT", Self::MINIMIZE_DISABLED => "MINIMIZE_DISABLED", Self::MAXIMIZE_DISABLED => "MAXIMIZE_DISABLED", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Flags::RESIZE_DISABLED, Flags::BORDERLESS, Flags::ALWAYS_ON_TOP, Flags::TRANSPARENT, Flags::NO_FOCUS, Flags::POPUP, Flags::EXTEND_TO_TITLE, Flags::MOUSE_PASSTHROUGH, Flags::SHARP_CORNERS, Flags::EXCLUDE_FROM_CAPTURE, Flags::POPUP_WM_HINT, Flags::MINIMIZE_DISABLED, Flags::MAXIMIZE_DISABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Flags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RESIZE_DISABLED", "FLAG_RESIZE_DISABLED", Flags::RESIZE_DISABLED), crate::meta::inspect::EnumConstant::new("BORDERLESS", "FLAG_BORDERLESS", Flags::BORDERLESS), crate::meta::inspect::EnumConstant::new("ALWAYS_ON_TOP", "FLAG_ALWAYS_ON_TOP", Flags::ALWAYS_ON_TOP), crate::meta::inspect::EnumConstant::new("TRANSPARENT", "FLAG_TRANSPARENT", Flags::TRANSPARENT), crate::meta::inspect::EnumConstant::new("NO_FOCUS", "FLAG_NO_FOCUS", Flags::NO_FOCUS), crate::meta::inspect::EnumConstant::new("POPUP", "FLAG_POPUP", Flags::POPUP), crate::meta::inspect::EnumConstant::new("EXTEND_TO_TITLE", "FLAG_EXTEND_TO_TITLE", Flags::EXTEND_TO_TITLE), crate::meta::inspect::EnumConstant::new("MOUSE_PASSTHROUGH", "FLAG_MOUSE_PASSTHROUGH", Flags::MOUSE_PASSTHROUGH), crate::meta::inspect::EnumConstant::new("SHARP_CORNERS", "FLAG_SHARP_CORNERS", Flags::SHARP_CORNERS), crate::meta::inspect::EnumConstant::new("EXCLUDE_FROM_CAPTURE", "FLAG_EXCLUDE_FROM_CAPTURE", Flags::EXCLUDE_FROM_CAPTURE), crate::meta::inspect::EnumConstant::new("POPUP_WM_HINT", "FLAG_POPUP_WM_HINT", Flags::POPUP_WM_HINT), crate::meta::inspect::EnumConstant::new("MINIMIZE_DISABLED", "FLAG_MINIMIZE_DISABLED", Flags::MINIMIZE_DISABLED), crate::meta::inspect::EnumConstant::new("MAXIMIZE_DISABLED", "FLAG_MAXIMIZE_DISABLED", Flags::MAXIMIZE_DISABLED), crate::meta::inspect::EnumConstant::new("MAX", "FLAG_MAX", Flags::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Flags {
    const ENUMERATOR_COUNT: usize = 13usize;
    
}
impl crate::meta::GodotConvert for Flags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Flags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Flags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ContentScaleMode {
    ord: i32
}
impl ContentScaleMode {
    #[doc(alias = "CONTENT_SCALE_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `CONTENT_SCALE_MODE_DISABLED`"]
    pub const DISABLED: ContentScaleMode = ContentScaleMode {
        ord: 0i32
    };
    #[doc(alias = "CONTENT_SCALE_MODE_CANVAS_ITEMS")]
    #[doc = "Godot enumerator name: `CONTENT_SCALE_MODE_CANVAS_ITEMS`"]
    pub const CANVAS_ITEMS: ContentScaleMode = ContentScaleMode {
        ord: 1i32
    };
    #[doc(alias = "CONTENT_SCALE_MODE_VIEWPORT")]
    #[doc = "Godot enumerator name: `CONTENT_SCALE_MODE_VIEWPORT`"]
    pub const VIEWPORT: ContentScaleMode = ContentScaleMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ContentScaleMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ContentScaleMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ContentScaleMode {
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
            Self::DISABLED => "DISABLED", Self::CANVAS_ITEMS => "CANVAS_ITEMS", Self::VIEWPORT => "VIEWPORT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ContentScaleMode::DISABLED, ContentScaleMode::CANVAS_ITEMS, ContentScaleMode::VIEWPORT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ContentScaleMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "CONTENT_SCALE_MODE_DISABLED", ContentScaleMode::DISABLED), crate::meta::inspect::EnumConstant::new("CANVAS_ITEMS", "CONTENT_SCALE_MODE_CANVAS_ITEMS", ContentScaleMode::CANVAS_ITEMS), crate::meta::inspect::EnumConstant::new("VIEWPORT", "CONTENT_SCALE_MODE_VIEWPORT", ContentScaleMode::VIEWPORT)]
        }
    }
}
impl crate::meta::GodotConvert for ContentScaleMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ContentScaleMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ContentScaleMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ContentScaleAspect {
    ord: i32
}
impl ContentScaleAspect {
    #[doc(alias = "CONTENT_SCALE_ASPECT_IGNORE")]
    #[doc = "Godot enumerator name: `CONTENT_SCALE_ASPECT_IGNORE`"]
    pub const IGNORE: ContentScaleAspect = ContentScaleAspect {
        ord: 0i32
    };
    #[doc(alias = "CONTENT_SCALE_ASPECT_KEEP")]
    #[doc = "Godot enumerator name: `CONTENT_SCALE_ASPECT_KEEP`"]
    pub const KEEP: ContentScaleAspect = ContentScaleAspect {
        ord: 1i32
    };
    #[doc(alias = "CONTENT_SCALE_ASPECT_KEEP_WIDTH")]
    #[doc = "Godot enumerator name: `CONTENT_SCALE_ASPECT_KEEP_WIDTH`"]
    pub const KEEP_WIDTH: ContentScaleAspect = ContentScaleAspect {
        ord: 2i32
    };
    #[doc(alias = "CONTENT_SCALE_ASPECT_KEEP_HEIGHT")]
    #[doc = "Godot enumerator name: `CONTENT_SCALE_ASPECT_KEEP_HEIGHT`"]
    pub const KEEP_HEIGHT: ContentScaleAspect = ContentScaleAspect {
        ord: 3i32
    };
    #[doc(alias = "CONTENT_SCALE_ASPECT_EXPAND")]
    #[doc = "Godot enumerator name: `CONTENT_SCALE_ASPECT_EXPAND`"]
    pub const EXPAND: ContentScaleAspect = ContentScaleAspect {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ContentScaleAspect {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ContentScaleAspect") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ContentScaleAspect {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::IGNORE => "IGNORE", Self::KEEP => "KEEP", Self::KEEP_WIDTH => "KEEP_WIDTH", Self::KEEP_HEIGHT => "KEEP_HEIGHT", Self::EXPAND => "EXPAND", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ContentScaleAspect::IGNORE, ContentScaleAspect::KEEP, ContentScaleAspect::KEEP_WIDTH, ContentScaleAspect::KEEP_HEIGHT, ContentScaleAspect::EXPAND]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ContentScaleAspect >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("IGNORE", "CONTENT_SCALE_ASPECT_IGNORE", ContentScaleAspect::IGNORE), crate::meta::inspect::EnumConstant::new("KEEP", "CONTENT_SCALE_ASPECT_KEEP", ContentScaleAspect::KEEP), crate::meta::inspect::EnumConstant::new("KEEP_WIDTH", "CONTENT_SCALE_ASPECT_KEEP_WIDTH", ContentScaleAspect::KEEP_WIDTH), crate::meta::inspect::EnumConstant::new("KEEP_HEIGHT", "CONTENT_SCALE_ASPECT_KEEP_HEIGHT", ContentScaleAspect::KEEP_HEIGHT), crate::meta::inspect::EnumConstant::new("EXPAND", "CONTENT_SCALE_ASPECT_EXPAND", ContentScaleAspect::EXPAND)]
        }
    }
}
impl crate::meta::GodotConvert for ContentScaleAspect {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ContentScaleAspect {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ContentScaleAspect {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ContentScaleStretch {
    ord: i32
}
impl ContentScaleStretch {
    #[doc(alias = "CONTENT_SCALE_STRETCH_FRACTIONAL")]
    #[doc = "Godot enumerator name: `CONTENT_SCALE_STRETCH_FRACTIONAL`"]
    pub const FRACTIONAL: ContentScaleStretch = ContentScaleStretch {
        ord: 0i32
    };
    #[doc(alias = "CONTENT_SCALE_STRETCH_INTEGER")]
    #[doc = "Godot enumerator name: `CONTENT_SCALE_STRETCH_INTEGER`"]
    pub const INTEGER: ContentScaleStretch = ContentScaleStretch {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for ContentScaleStretch {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ContentScaleStretch") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ContentScaleStretch {
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
            Self::FRACTIONAL => "FRACTIONAL", Self::INTEGER => "INTEGER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ContentScaleStretch::FRACTIONAL, ContentScaleStretch::INTEGER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ContentScaleStretch >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FRACTIONAL", "CONTENT_SCALE_STRETCH_FRACTIONAL", ContentScaleStretch::FRACTIONAL), crate::meta::inspect::EnumConstant::new("INTEGER", "CONTENT_SCALE_STRETCH_INTEGER", ContentScaleStretch::INTEGER)]
        }
    }
}
impl crate::meta::GodotConvert for ContentScaleStretch {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ContentScaleStretch {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ContentScaleStretch {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LayoutDirection {
    ord: i32
}
impl LayoutDirection {
    #[doc(alias = "LAYOUT_DIRECTION_INHERITED")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_INHERITED`"]
    pub const INHERITED: LayoutDirection = LayoutDirection {
        ord: 0i32
    };
    #[doc(alias = "LAYOUT_DIRECTION_APPLICATION_LOCALE")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_APPLICATION_LOCALE`"]
    pub const APPLICATION_LOCALE: LayoutDirection = LayoutDirection {
        ord: 1i32
    };
    #[doc(alias = "LAYOUT_DIRECTION_LTR")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_LTR`"]
    pub const LTR: LayoutDirection = LayoutDirection {
        ord: 2i32
    };
    #[doc(alias = "LAYOUT_DIRECTION_RTL")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_RTL`"]
    pub const RTL: LayoutDirection = LayoutDirection {
        ord: 3i32
    };
    #[doc(alias = "LAYOUT_DIRECTION_SYSTEM_LOCALE")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_SYSTEM_LOCALE`"]
    pub const SYSTEM_LOCALE: LayoutDirection = LayoutDirection {
        ord: 4i32
    };
    #[doc(alias = "LAYOUT_DIRECTION_MAX")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_MAX`"]
    pub const MAX: LayoutDirection = LayoutDirection {
        ord: 5i32
    };
    #[doc(alias = "LAYOUT_DIRECTION_LOCALE")]
    #[doc = "Godot enumerator name: `LAYOUT_DIRECTION_LOCALE`"]
    pub const LOCALE: LayoutDirection = LayoutDirection {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for LayoutDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LayoutDirection") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LayoutDirection {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::INHERITED => "INHERITED", Self::APPLICATION_LOCALE => "APPLICATION_LOCALE", Self::LTR => "LTR", Self::RTL => "RTL", Self::SYSTEM_LOCALE => "SYSTEM_LOCALE", Self::MAX => "MAX", Self::LOCALE => "LOCALE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LayoutDirection::INHERITED, LayoutDirection::APPLICATION_LOCALE, LayoutDirection::LTR, LayoutDirection::RTL, LayoutDirection::SYSTEM_LOCALE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LayoutDirection >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INHERITED", "LAYOUT_DIRECTION_INHERITED", LayoutDirection::INHERITED), crate::meta::inspect::EnumConstant::new("APPLICATION_LOCALE", "LAYOUT_DIRECTION_APPLICATION_LOCALE", LayoutDirection::APPLICATION_LOCALE), crate::meta::inspect::EnumConstant::new("LTR", "LAYOUT_DIRECTION_LTR", LayoutDirection::LTR), crate::meta::inspect::EnumConstant::new("RTL", "LAYOUT_DIRECTION_RTL", LayoutDirection::RTL), crate::meta::inspect::EnumConstant::new("SYSTEM_LOCALE", "LAYOUT_DIRECTION_SYSTEM_LOCALE", LayoutDirection::SYSTEM_LOCALE), crate::meta::inspect::EnumConstant::new("MAX", "LAYOUT_DIRECTION_MAX", LayoutDirection::MAX), crate::meta::inspect::EnumConstant::new("LOCALE", "LAYOUT_DIRECTION_LOCALE", LayoutDirection::LOCALE)]
        }
    }
}
impl crate::obj::IndexEnum for LayoutDirection {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for LayoutDirection {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LayoutDirection {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LayoutDirection {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct WindowInitialPosition {
    ord: i32
}
impl WindowInitialPosition {
    #[doc(alias = "WINDOW_INITIAL_POSITION_ABSOLUTE")]
    #[doc = "Godot enumerator name: `WINDOW_INITIAL_POSITION_ABSOLUTE`"]
    pub const ABSOLUTE: WindowInitialPosition = WindowInitialPosition {
        ord: 0i32
    };
    #[doc(alias = "WINDOW_INITIAL_POSITION_CENTER_PRIMARY_SCREEN")]
    #[doc = "Godot enumerator name: `WINDOW_INITIAL_POSITION_CENTER_PRIMARY_SCREEN`"]
    pub const CENTER_PRIMARY_SCREEN: WindowInitialPosition = WindowInitialPosition {
        ord: 1i32
    };
    #[doc(alias = "WINDOW_INITIAL_POSITION_CENTER_MAIN_WINDOW_SCREEN")]
    #[doc = "Godot enumerator name: `WINDOW_INITIAL_POSITION_CENTER_MAIN_WINDOW_SCREEN`"]
    pub const CENTER_MAIN_WINDOW_SCREEN: WindowInitialPosition = WindowInitialPosition {
        ord: 2i32
    };
    #[doc(alias = "WINDOW_INITIAL_POSITION_CENTER_OTHER_SCREEN")]
    #[doc = "Godot enumerator name: `WINDOW_INITIAL_POSITION_CENTER_OTHER_SCREEN`"]
    pub const CENTER_OTHER_SCREEN: WindowInitialPosition = WindowInitialPosition {
        ord: 3i32
    };
    #[doc(alias = "WINDOW_INITIAL_POSITION_CENTER_SCREEN_WITH_MOUSE_FOCUS")]
    #[doc = "Godot enumerator name: `WINDOW_INITIAL_POSITION_CENTER_SCREEN_WITH_MOUSE_FOCUS`"]
    pub const CENTER_SCREEN_WITH_MOUSE_FOCUS: WindowInitialPosition = WindowInitialPosition {
        ord: 4i32
    };
    #[doc(alias = "WINDOW_INITIAL_POSITION_CENTER_SCREEN_WITH_KEYBOARD_FOCUS")]
    #[doc = "Godot enumerator name: `WINDOW_INITIAL_POSITION_CENTER_SCREEN_WITH_KEYBOARD_FOCUS`"]
    pub const CENTER_SCREEN_WITH_KEYBOARD_FOCUS: WindowInitialPosition = WindowInitialPosition {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for WindowInitialPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("WindowInitialPosition") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for WindowInitialPosition {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::ABSOLUTE => "ABSOLUTE", Self::CENTER_PRIMARY_SCREEN => "CENTER_PRIMARY_SCREEN", Self::CENTER_MAIN_WINDOW_SCREEN => "CENTER_MAIN_WINDOW_SCREEN", Self::CENTER_OTHER_SCREEN => "CENTER_OTHER_SCREEN", Self::CENTER_SCREEN_WITH_MOUSE_FOCUS => "CENTER_SCREEN_WITH_MOUSE_FOCUS", Self::CENTER_SCREEN_WITH_KEYBOARD_FOCUS => "CENTER_SCREEN_WITH_KEYBOARD_FOCUS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[WindowInitialPosition::ABSOLUTE, WindowInitialPosition::CENTER_PRIMARY_SCREEN, WindowInitialPosition::CENTER_MAIN_WINDOW_SCREEN, WindowInitialPosition::CENTER_OTHER_SCREEN, WindowInitialPosition::CENTER_SCREEN_WITH_MOUSE_FOCUS, WindowInitialPosition::CENTER_SCREEN_WITH_KEYBOARD_FOCUS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < WindowInitialPosition >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ABSOLUTE", "WINDOW_INITIAL_POSITION_ABSOLUTE", WindowInitialPosition::ABSOLUTE), crate::meta::inspect::EnumConstant::new("CENTER_PRIMARY_SCREEN", "WINDOW_INITIAL_POSITION_CENTER_PRIMARY_SCREEN", WindowInitialPosition::CENTER_PRIMARY_SCREEN), crate::meta::inspect::EnumConstant::new("CENTER_MAIN_WINDOW_SCREEN", "WINDOW_INITIAL_POSITION_CENTER_MAIN_WINDOW_SCREEN", WindowInitialPosition::CENTER_MAIN_WINDOW_SCREEN), crate::meta::inspect::EnumConstant::new("CENTER_OTHER_SCREEN", "WINDOW_INITIAL_POSITION_CENTER_OTHER_SCREEN", WindowInitialPosition::CENTER_OTHER_SCREEN), crate::meta::inspect::EnumConstant::new("CENTER_SCREEN_WITH_MOUSE_FOCUS", "WINDOW_INITIAL_POSITION_CENTER_SCREEN_WITH_MOUSE_FOCUS", WindowInitialPosition::CENTER_SCREEN_WITH_MOUSE_FOCUS), crate::meta::inspect::EnumConstant::new("CENTER_SCREEN_WITH_KEYBOARD_FOCUS", "WINDOW_INITIAL_POSITION_CENTER_SCREEN_WITH_KEYBOARD_FOCUS", WindowInitialPosition::CENTER_SCREEN_WITH_KEYBOARD_FOCUS)]
        }
    }
}
impl crate::meta::GodotConvert for WindowInitialPosition {
    type Via = i32;
    
}
impl crate::meta::ToGodot for WindowInitialPosition {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for WindowInitialPosition {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Window;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Window`][crate::classes::Window] class."]
    pub struct SignalsOfWindow < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfWindow < 'c, C > {
        #[doc = "Signature: `(event: Gd<InputEvent>)`"]
        pub fn window_input(&mut self) -> SigWindowInput < 'c, C > {
            SigWindowInput {
                typed: TypedSignal::extract(&mut self.__internal_obj, "window_input")
            }
        }
        #[doc = "Signature: `(files: PackedStringArray)`"]
        pub fn files_dropped(&mut self) -> SigFilesDropped < 'c, C > {
            SigFilesDropped {
                typed: TypedSignal::extract(&mut self.__internal_obj, "files_dropped")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn mouse_entered(&mut self) -> SigMouseEntered < 'c, C > {
            SigMouseEntered {
                typed: TypedSignal::extract(&mut self.__internal_obj, "mouse_entered")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn mouse_exited(&mut self) -> SigMouseExited < 'c, C > {
            SigMouseExited {
                typed: TypedSignal::extract(&mut self.__internal_obj, "mouse_exited")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn focus_entered(&mut self) -> SigFocusEntered < 'c, C > {
            SigFocusEntered {
                typed: TypedSignal::extract(&mut self.__internal_obj, "focus_entered")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn focus_exited(&mut self) -> SigFocusExited < 'c, C > {
            SigFocusExited {
                typed: TypedSignal::extract(&mut self.__internal_obj, "focus_exited")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn close_requested(&mut self) -> SigCloseRequested < 'c, C > {
            SigCloseRequested {
                typed: TypedSignal::extract(&mut self.__internal_obj, "close_requested")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn go_back_requested(&mut self) -> SigGoBackRequested < 'c, C > {
            SigGoBackRequested {
                typed: TypedSignal::extract(&mut self.__internal_obj, "go_back_requested")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn visibility_changed(&mut self) -> SigVisibilityChanged < 'c, C > {
            SigVisibilityChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "visibility_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn about_to_popup(&mut self) -> SigAboutToPopup < 'c, C > {
            SigAboutToPopup {
                typed: TypedSignal::extract(&mut self.__internal_obj, "about_to_popup")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn theme_changed(&mut self) -> SigThemeChanged < 'c, C > {
            SigThemeChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "theme_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn dpi_changed(&mut self) -> SigDpiChanged < 'c, C > {
            SigDpiChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "dpi_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn titlebar_changed(&mut self) -> SigTitlebarChanged < 'c, C > {
            SigTitlebarChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "titlebar_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn title_changed(&mut self) -> SigTitleChanged < 'c, C > {
            SigTitleChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "title_changed")
            }
        }
    }
    type TypedSigWindowInput < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::InputEvent >,) >;
    pub struct SigWindowInput < 'c, C: WithSignals > {
        typed: TypedSigWindowInput < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigWindowInput < 'c, C > {
        pub fn emit(&mut self, event: Gd < crate::classes::InputEvent >,) {
            self.typed.emit_tuple((event,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigWindowInput < 'c, C > {
        type Target = TypedSigWindowInput < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigWindowInput < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFilesDropped < 'c, C > = TypedSignal < 'c, C, (PackedStringArray,) >;
    pub struct SigFilesDropped < 'c, C: WithSignals > {
        typed: TypedSigFilesDropped < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFilesDropped < 'c, C > {
        pub fn emit(&mut self, files: PackedStringArray,) {
            self.typed.emit_tuple((files,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFilesDropped < 'c, C > {
        type Target = TypedSigFilesDropped < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFilesDropped < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMouseEntered < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigMouseEntered < 'c, C: WithSignals > {
        typed: TypedSigMouseEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMouseEntered < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMouseEntered < 'c, C > {
        type Target = TypedSigMouseEntered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMouseEntered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMouseExited < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigMouseExited < 'c, C: WithSignals > {
        typed: TypedSigMouseExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMouseExited < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMouseExited < 'c, C > {
        type Target = TypedSigMouseExited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMouseExited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFocusEntered < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFocusEntered < 'c, C: WithSignals > {
        typed: TypedSigFocusEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFocusEntered < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFocusEntered < 'c, C > {
        type Target = TypedSigFocusEntered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFocusEntered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFocusExited < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFocusExited < 'c, C: WithSignals > {
        typed: TypedSigFocusExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFocusExited < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFocusExited < 'c, C > {
        type Target = TypedSigFocusExited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFocusExited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCloseRequested < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigCloseRequested < 'c, C: WithSignals > {
        typed: TypedSigCloseRequested < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCloseRequested < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCloseRequested < 'c, C > {
        type Target = TypedSigCloseRequested < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCloseRequested < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigGoBackRequested < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigGoBackRequested < 'c, C: WithSignals > {
        typed: TypedSigGoBackRequested < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigGoBackRequested < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigGoBackRequested < 'c, C > {
        type Target = TypedSigGoBackRequested < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigGoBackRequested < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
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
    type TypedSigAboutToPopup < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigAboutToPopup < 'c, C: WithSignals > {
        typed: TypedSigAboutToPopup < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAboutToPopup < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAboutToPopup < 'c, C > {
        type Target = TypedSigAboutToPopup < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAboutToPopup < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigThemeChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigThemeChanged < 'c, C: WithSignals > {
        typed: TypedSigThemeChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigThemeChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigThemeChanged < 'c, C > {
        type Target = TypedSigThemeChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigThemeChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigDpiChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigDpiChanged < 'c, C: WithSignals > {
        typed: TypedSigDpiChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDpiChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDpiChanged < 'c, C > {
        type Target = TypedSigDpiChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDpiChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTitlebarChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTitlebarChanged < 'c, C: WithSignals > {
        typed: TypedSigTitlebarChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTitlebarChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTitlebarChanged < 'c, C > {
        type Target = TypedSigTitlebarChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTitlebarChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTitleChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTitleChanged < 'c, C: WithSignals > {
        typed: TypedSigTitleChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTitleChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTitleChanged < 'c, C > {
        type Target = TypedSigTitleChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTitleChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Window {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfWindow < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfWindow < 'c, C > {
        type Target = < < Window as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Window;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfWindow < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Window;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}