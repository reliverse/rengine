#![doc = "Sidecar module for class [`Container`][crate::classes::Container].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Container` enums](https://docs.godotengine.org/en/stable/classes/class_container.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Container.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`container`][crate::classes::container]: sidecar module with related enum/flag types\n* [`IContainer`][crate::classes::IContainer]: virtual methods\n* [`SignalsOfContainer`][crate::classes::container::SignalsOfContainer]: signal collection\n* [`ContainerNotification`][crate::classes::notify::ContainerNotification]: notification type\n\n\nSee also [Godot docs for `Container`](https://docs.godotengine.org/en/stable/classes/class_container.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Container::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Container {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Container`][crate::classes::Container].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Container` methods](https://docs.godotengine.org/en/stable/classes/class_container.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IContainer: crate::obj::GodotClass < Base = Container > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ContainerNotification) {
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
        fn get_allowed_size_flags_horizontal(&self,) -> PackedInt32Array {
            unimplemented !()
        }
        fn get_allowed_size_flags_vertical(&self,) -> PackedInt32Array {
            unimplemented !()
        }
        fn has_point(&self, point: Vector2,) -> bool {
            unimplemented !()
        }
        fn structured_text_parser(&self, args: VarArray, text: GString,) -> Array < Vector3i > {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_tooltip(&self, at_position: Vector2,) -> GString {
            unimplemented !()
        }
        fn get_drag_data(&mut self, at_position: Vector2,) -> Variant {
            unimplemented !()
        }
        fn can_drop_data(&self, at_position: Vector2, data: Variant,) -> bool {
            unimplemented !()
        }
        fn drop_data(&mut self, at_position: Vector2, data: Variant,) {
            unimplemented !()
        }
        fn make_custom_tooltip(&self, for_text: GString,) -> Option < Gd < crate::classes::Object > > {
            unimplemented !()
        }
        fn accessibility_get_contextual_info(&self,) -> GString {
            unimplemented !()
        }
        fn get_accessibility_container_name(&self, node: Option < Gd < crate::classes::Node > >,) -> GString {
            unimplemented !()
        }
        fn gui_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
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
    #[doc = "Notification type for class [`Container`][crate::classes::Container]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[doc = r""]
    #[doc = r" Contains the [`Unknown`][Self::Unknown] variant for forward compatibility."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum ContainerNotification {
        PRE_SORT_CHILDREN = 50i32, SORT_CHILDREN = 51i32, RESIZED = 40i32, MOUSE_ENTER = 41i32, MOUSE_EXIT = 42i32, MOUSE_ENTER_SELF = 60i32, MOUSE_EXIT_SELF = 61i32, FOCUS_ENTER = 43i32, FOCUS_EXIT = 44i32, THEME_CHANGED = 45i32, SCROLL_BEGIN = 47i32, SCROLL_END = 48i32, LAYOUT_DIRECTION_CHANGED = 49i32, TRANSFORM_CHANGED = 2000i32, LOCAL_TRANSFORM_CHANGED = 35i32, DRAW = 30i32, VISIBILITY_CHANGED = 31i32, ENTER_CANVAS = 32i32, EXIT_CANVAS = 33i32, WORLD_2D_CHANGED = 36i32, ENTER_TREE = 10i32, EXIT_TREE = 11i32, MOVED_IN_PARENT = 12i32, READY = 13i32, PAUSED = 14i32, UNPAUSED = 15i32, PHYSICS_PROCESS = 16i32, PROCESS = 17i32, PARENTED = 18i32, UNPARENTED = 19i32, SCENE_INSTANTIATED = 20i32, DRAG_BEGIN = 21i32, DRAG_END = 22i32, PATH_RENAMED = 23i32, CHILD_ORDER_CHANGED = 24i32, INTERNAL_PROCESS = 25i32, INTERNAL_PHYSICS_PROCESS = 26i32, POST_ENTER_TREE = 27i32, DISABLED = 28i32, ENABLED = 29i32, RESET_PHYSICS_INTERPOLATION = 2001i32, EDITOR_PRE_SAVE = 9001i32, EDITOR_POST_SAVE = 9002i32, WM_MOUSE_ENTER = 1002i32, WM_MOUSE_EXIT = 1003i32, WM_WINDOW_FOCUS_IN = 1004i32, WM_WINDOW_FOCUS_OUT = 1005i32, WM_CLOSE_REQUEST = 1006i32, WM_GO_BACK_REQUEST = 1007i32, WM_SIZE_CHANGED = 1008i32, WM_DPI_CHANGE = 1009i32, VP_MOUSE_ENTER = 1010i32, VP_MOUSE_EXIT = 1011i32, WM_POSITION_CHANGED = 1012i32, OS_MEMORY_WARNING = 2009i32, TRANSLATION_CHANGED = 2010i32, WM_ABOUT = 2011i32, CRASH = 2012i32, OS_IME_UPDATE = 2013i32, APPLICATION_RESUMED = 2014i32, APPLICATION_PAUSED = 2015i32, APPLICATION_FOCUS_IN = 2016i32, APPLICATION_FOCUS_OUT = 2017i32, TEXT_SERVER_CHANGED = 2018i32, ACCESSIBILITY_UPDATE = 3000i32, ACCESSIBILITY_INVALIDATE = 3001i32, POSTINITIALIZE = 0i32, PREDELETE = 1i32, EXTENSION_RELOADED = 2i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        #[doc = r""]
        #[doc = r" This is also necessary if you develop an extension on a Godot version and want to be forward-compatible with newer"]
        #[doc = r" versions. If Godot adds new notifications, they will be unknown to your extension, but you can still handle them."]
        Unknown(i32),
    }
    impl From < i32 > for ContainerNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                50i32 => Self::PRE_SORT_CHILDREN, 51i32 => Self::SORT_CHILDREN, 40i32 => Self::RESIZED, 41i32 => Self::MOUSE_ENTER, 42i32 => Self::MOUSE_EXIT, 60i32 => Self::MOUSE_ENTER_SELF, 61i32 => Self::MOUSE_EXIT_SELF, 43i32 => Self::FOCUS_ENTER, 44i32 => Self::FOCUS_EXIT, 45i32 => Self::THEME_CHANGED, 47i32 => Self::SCROLL_BEGIN, 48i32 => Self::SCROLL_END, 49i32 => Self::LAYOUT_DIRECTION_CHANGED, 2000i32 => Self::TRANSFORM_CHANGED, 35i32 => Self::LOCAL_TRANSFORM_CHANGED, 30i32 => Self::DRAW, 31i32 => Self::VISIBILITY_CHANGED, 32i32 => Self::ENTER_CANVAS, 33i32 => Self::EXIT_CANVAS, 36i32 => Self::WORLD_2D_CHANGED, 10i32 => Self::ENTER_TREE, 11i32 => Self::EXIT_TREE, 12i32 => Self::MOVED_IN_PARENT, 13i32 => Self::READY, 14i32 => Self::PAUSED, 15i32 => Self::UNPAUSED, 16i32 => Self::PHYSICS_PROCESS, 17i32 => Self::PROCESS, 18i32 => Self::PARENTED, 19i32 => Self::UNPARENTED, 20i32 => Self::SCENE_INSTANTIATED, 21i32 => Self::DRAG_BEGIN, 22i32 => Self::DRAG_END, 23i32 => Self::PATH_RENAMED, 24i32 => Self::CHILD_ORDER_CHANGED, 25i32 => Self::INTERNAL_PROCESS, 26i32 => Self::INTERNAL_PHYSICS_PROCESS, 27i32 => Self::POST_ENTER_TREE, 28i32 => Self::DISABLED, 29i32 => Self::ENABLED, 2001i32 => Self::RESET_PHYSICS_INTERPOLATION, 9001i32 => Self::EDITOR_PRE_SAVE, 9002i32 => Self::EDITOR_POST_SAVE, 1002i32 => Self::WM_MOUSE_ENTER, 1003i32 => Self::WM_MOUSE_EXIT, 1004i32 => Self::WM_WINDOW_FOCUS_IN, 1005i32 => Self::WM_WINDOW_FOCUS_OUT, 1006i32 => Self::WM_CLOSE_REQUEST, 1007i32 => Self::WM_GO_BACK_REQUEST, 1008i32 => Self::WM_SIZE_CHANGED, 1009i32 => Self::WM_DPI_CHANGE, 1010i32 => Self::VP_MOUSE_ENTER, 1011i32 => Self::VP_MOUSE_EXIT, 1012i32 => Self::WM_POSITION_CHANGED, 2009i32 => Self::OS_MEMORY_WARNING, 2010i32 => Self::TRANSLATION_CHANGED, 2011i32 => Self::WM_ABOUT, 2012i32 => Self::CRASH, 2013i32 => Self::OS_IME_UPDATE, 2014i32 => Self::APPLICATION_RESUMED, 2015i32 => Self::APPLICATION_PAUSED, 2016i32 => Self::APPLICATION_FOCUS_IN, 2017i32 => Self::APPLICATION_FOCUS_OUT, 2018i32 => Self::TEXT_SERVER_CHANGED, 3000i32 => Self::ACCESSIBILITY_UPDATE, 3001i32 => Self::ACCESSIBILITY_INVALIDATE, 0i32 => Self::POSTINITIALIZE, 1i32 => Self::PREDELETE, 2i32 => Self::EXTENSION_RELOADED, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < ContainerNotification > for i32 {
        fn from(notification: ContainerNotification) -> i32 {
            match notification {
                ContainerNotification::PRE_SORT_CHILDREN => 50i32, ContainerNotification::SORT_CHILDREN => 51i32, ContainerNotification::RESIZED => 40i32, ContainerNotification::MOUSE_ENTER => 41i32, ContainerNotification::MOUSE_EXIT => 42i32, ContainerNotification::MOUSE_ENTER_SELF => 60i32, ContainerNotification::MOUSE_EXIT_SELF => 61i32, ContainerNotification::FOCUS_ENTER => 43i32, ContainerNotification::FOCUS_EXIT => 44i32, ContainerNotification::THEME_CHANGED => 45i32, ContainerNotification::SCROLL_BEGIN => 47i32, ContainerNotification::SCROLL_END => 48i32, ContainerNotification::LAYOUT_DIRECTION_CHANGED => 49i32, ContainerNotification::TRANSFORM_CHANGED => 2000i32, ContainerNotification::LOCAL_TRANSFORM_CHANGED => 35i32, ContainerNotification::DRAW => 30i32, ContainerNotification::VISIBILITY_CHANGED => 31i32, ContainerNotification::ENTER_CANVAS => 32i32, ContainerNotification::EXIT_CANVAS => 33i32, ContainerNotification::WORLD_2D_CHANGED => 36i32, ContainerNotification::ENTER_TREE => 10i32, ContainerNotification::EXIT_TREE => 11i32, ContainerNotification::MOVED_IN_PARENT => 12i32, ContainerNotification::READY => 13i32, ContainerNotification::PAUSED => 14i32, ContainerNotification::UNPAUSED => 15i32, ContainerNotification::PHYSICS_PROCESS => 16i32, ContainerNotification::PROCESS => 17i32, ContainerNotification::PARENTED => 18i32, ContainerNotification::UNPARENTED => 19i32, ContainerNotification::SCENE_INSTANTIATED => 20i32, ContainerNotification::DRAG_BEGIN => 21i32, ContainerNotification::DRAG_END => 22i32, ContainerNotification::PATH_RENAMED => 23i32, ContainerNotification::CHILD_ORDER_CHANGED => 24i32, ContainerNotification::INTERNAL_PROCESS => 25i32, ContainerNotification::INTERNAL_PHYSICS_PROCESS => 26i32, ContainerNotification::POST_ENTER_TREE => 27i32, ContainerNotification::DISABLED => 28i32, ContainerNotification::ENABLED => 29i32, ContainerNotification::RESET_PHYSICS_INTERPOLATION => 2001i32, ContainerNotification::EDITOR_PRE_SAVE => 9001i32, ContainerNotification::EDITOR_POST_SAVE => 9002i32, ContainerNotification::WM_MOUSE_ENTER => 1002i32, ContainerNotification::WM_MOUSE_EXIT => 1003i32, ContainerNotification::WM_WINDOW_FOCUS_IN => 1004i32, ContainerNotification::WM_WINDOW_FOCUS_OUT => 1005i32, ContainerNotification::WM_CLOSE_REQUEST => 1006i32, ContainerNotification::WM_GO_BACK_REQUEST => 1007i32, ContainerNotification::WM_SIZE_CHANGED => 1008i32, ContainerNotification::WM_DPI_CHANGE => 1009i32, ContainerNotification::VP_MOUSE_ENTER => 1010i32, ContainerNotification::VP_MOUSE_EXIT => 1011i32, ContainerNotification::WM_POSITION_CHANGED => 1012i32, ContainerNotification::OS_MEMORY_WARNING => 2009i32, ContainerNotification::TRANSLATION_CHANGED => 2010i32, ContainerNotification::WM_ABOUT => 2011i32, ContainerNotification::CRASH => 2012i32, ContainerNotification::OS_IME_UPDATE => 2013i32, ContainerNotification::APPLICATION_RESUMED => 2014i32, ContainerNotification::APPLICATION_PAUSED => 2015i32, ContainerNotification::APPLICATION_FOCUS_IN => 2016i32, ContainerNotification::APPLICATION_FOCUS_OUT => 2017i32, ContainerNotification::TEXT_SERVER_CHANGED => 2018i32, ContainerNotification::ACCESSIBILITY_UPDATE => 3000i32, ContainerNotification::ACCESSIBILITY_INVALIDATE => 3001i32, ContainerNotification::POSTINITIALIZE => 0i32, ContainerNotification::PREDELETE => 1i32, ContainerNotification::EXTENSION_RELOADED => 2i32, ContainerNotification::Unknown(int) => int,
            }
        }
    }
    impl Container {
        pub fn queue_sort(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2456usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Container", "queue_sort", Some(self.__validated_obj()), args,)
            }
        }
        pub fn fit_child_in_rect(&mut self, child: impl AsArg < Option < Gd < crate::classes::Control >> >, rect: Rect2,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >, Rect2,);
            let args = (child.into_arg(), rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2457usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Container", "fit_child_in_rect", Some(self.__validated_obj()), args,)
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
        pub fn notify(&mut self, what: ContainerNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: ContainerNotification) {
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
        pub(crate) const NOTIFICATION_PRE_SORT_CHILDREN: i32 = 50i32;
        pub(crate) const NOTIFICATION_SORT_CHILDREN: i32 = 51i32;
        
    }
    impl crate::obj::GodotClass for Container {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Container"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Container {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for Container {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Container {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Container {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Container {
        
    }
    impl crate::obj::cap::GodotDefault for Container {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Container {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Container {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Container`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Container__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Container > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Control > for $Class {
                
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Container;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Container`][crate::classes::Container] class."]
    pub struct SignalsOfContainer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfContainer < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn pre_sort_children(&mut self) -> SigPreSortChildren < 'c, C > {
            SigPreSortChildren {
                typed: TypedSignal::extract(&mut self.__internal_obj, "pre_sort_children")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn sort_children(&mut self) -> SigSortChildren < 'c, C > {
            SigSortChildren {
                typed: TypedSignal::extract(&mut self.__internal_obj, "sort_children")
            }
        }
    }
    type TypedSigPreSortChildren < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigPreSortChildren < 'c, C: WithSignals > {
        typed: TypedSigPreSortChildren < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPreSortChildren < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPreSortChildren < 'c, C > {
        type Target = TypedSigPreSortChildren < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPreSortChildren < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSortChildren < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSortChildren < 'c, C: WithSignals > {
        typed: TypedSigSortChildren < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSortChildren < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSortChildren < 'c, C > {
        type Target = TypedSigSortChildren < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSortChildren < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Container {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfContainer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfContainer < 'c, C > {
        type Target = < < Container as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Container;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfContainer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Container;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}