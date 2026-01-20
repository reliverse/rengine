#![doc = "Sidecar module for class [`Node`][crate::classes::Node].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Node` enums](https://docs.godotengine.org/en/stable/classes/class_node.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Node.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`node`][crate::classes::node]: sidecar module with related enum/flag types\n* [`INode`][crate::classes::INode]: virtual methods\n* [`SignalsOfNode`][crate::classes::node::SignalsOfNode]: signal collection\n* [`NodeNotification`][crate::classes::notify::NodeNotification]: notification type\n\n\nSee also [Godot docs for `Node`](https://docs.godotengine.org/en/stable/classes/class_node.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Node::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Node {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Node`][crate::classes::Node].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Node` methods](https://docs.godotengine.org/en/stable/classes/class_node.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INode: crate::obj::GodotClass < Base = Node > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: NodeNotification) {
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
    #[doc = "Notification type for class [`Node`][crate::classes::Node]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[doc = r""]
    #[doc = r" Contains the [`Unknown`][Self::Unknown] variant for forward compatibility."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum NodeNotification {
        ENTER_TREE = 10i32, EXIT_TREE = 11i32, MOVED_IN_PARENT = 12i32, READY = 13i32, PAUSED = 14i32, UNPAUSED = 15i32, PHYSICS_PROCESS = 16i32, PROCESS = 17i32, PARENTED = 18i32, UNPARENTED = 19i32, SCENE_INSTANTIATED = 20i32, DRAG_BEGIN = 21i32, DRAG_END = 22i32, PATH_RENAMED = 23i32, CHILD_ORDER_CHANGED = 24i32, INTERNAL_PROCESS = 25i32, INTERNAL_PHYSICS_PROCESS = 26i32, POST_ENTER_TREE = 27i32, DISABLED = 28i32, ENABLED = 29i32, RESET_PHYSICS_INTERPOLATION = 2001i32, EDITOR_PRE_SAVE = 9001i32, EDITOR_POST_SAVE = 9002i32, WM_MOUSE_ENTER = 1002i32, WM_MOUSE_EXIT = 1003i32, WM_WINDOW_FOCUS_IN = 1004i32, WM_WINDOW_FOCUS_OUT = 1005i32, WM_CLOSE_REQUEST = 1006i32, WM_GO_BACK_REQUEST = 1007i32, WM_SIZE_CHANGED = 1008i32, WM_DPI_CHANGE = 1009i32, VP_MOUSE_ENTER = 1010i32, VP_MOUSE_EXIT = 1011i32, WM_POSITION_CHANGED = 1012i32, OS_MEMORY_WARNING = 2009i32, TRANSLATION_CHANGED = 2010i32, WM_ABOUT = 2011i32, CRASH = 2012i32, OS_IME_UPDATE = 2013i32, APPLICATION_RESUMED = 2014i32, APPLICATION_PAUSED = 2015i32, APPLICATION_FOCUS_IN = 2016i32, APPLICATION_FOCUS_OUT = 2017i32, TEXT_SERVER_CHANGED = 2018i32, ACCESSIBILITY_UPDATE = 3000i32, ACCESSIBILITY_INVALIDATE = 3001i32, POSTINITIALIZE = 0i32, PREDELETE = 1i32, EXTENSION_RELOADED = 2i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        #[doc = r""]
        #[doc = r" This is also necessary if you develop an extension on a Godot version and want to be forward-compatible with newer"]
        #[doc = r" versions. If Godot adds new notifications, they will be unknown to your extension, but you can still handle them."]
        Unknown(i32),
    }
    impl From < i32 > for NodeNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                10i32 => Self::ENTER_TREE, 11i32 => Self::EXIT_TREE, 12i32 => Self::MOVED_IN_PARENT, 13i32 => Self::READY, 14i32 => Self::PAUSED, 15i32 => Self::UNPAUSED, 16i32 => Self::PHYSICS_PROCESS, 17i32 => Self::PROCESS, 18i32 => Self::PARENTED, 19i32 => Self::UNPARENTED, 20i32 => Self::SCENE_INSTANTIATED, 21i32 => Self::DRAG_BEGIN, 22i32 => Self::DRAG_END, 23i32 => Self::PATH_RENAMED, 24i32 => Self::CHILD_ORDER_CHANGED, 25i32 => Self::INTERNAL_PROCESS, 26i32 => Self::INTERNAL_PHYSICS_PROCESS, 27i32 => Self::POST_ENTER_TREE, 28i32 => Self::DISABLED, 29i32 => Self::ENABLED, 2001i32 => Self::RESET_PHYSICS_INTERPOLATION, 9001i32 => Self::EDITOR_PRE_SAVE, 9002i32 => Self::EDITOR_POST_SAVE, 1002i32 => Self::WM_MOUSE_ENTER, 1003i32 => Self::WM_MOUSE_EXIT, 1004i32 => Self::WM_WINDOW_FOCUS_IN, 1005i32 => Self::WM_WINDOW_FOCUS_OUT, 1006i32 => Self::WM_CLOSE_REQUEST, 1007i32 => Self::WM_GO_BACK_REQUEST, 1008i32 => Self::WM_SIZE_CHANGED, 1009i32 => Self::WM_DPI_CHANGE, 1010i32 => Self::VP_MOUSE_ENTER, 1011i32 => Self::VP_MOUSE_EXIT, 1012i32 => Self::WM_POSITION_CHANGED, 2009i32 => Self::OS_MEMORY_WARNING, 2010i32 => Self::TRANSLATION_CHANGED, 2011i32 => Self::WM_ABOUT, 2012i32 => Self::CRASH, 2013i32 => Self::OS_IME_UPDATE, 2014i32 => Self::APPLICATION_RESUMED, 2015i32 => Self::APPLICATION_PAUSED, 2016i32 => Self::APPLICATION_FOCUS_IN, 2017i32 => Self::APPLICATION_FOCUS_OUT, 2018i32 => Self::TEXT_SERVER_CHANGED, 3000i32 => Self::ACCESSIBILITY_UPDATE, 3001i32 => Self::ACCESSIBILITY_INVALIDATE, 0i32 => Self::POSTINITIALIZE, 1i32 => Self::PREDELETE, 2i32 => Self::EXTENSION_RELOADED, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < NodeNotification > for i32 {
        fn from(notification: NodeNotification) -> i32 {
            match notification {
                NodeNotification::ENTER_TREE => 10i32, NodeNotification::EXIT_TREE => 11i32, NodeNotification::MOVED_IN_PARENT => 12i32, NodeNotification::READY => 13i32, NodeNotification::PAUSED => 14i32, NodeNotification::UNPAUSED => 15i32, NodeNotification::PHYSICS_PROCESS => 16i32, NodeNotification::PROCESS => 17i32, NodeNotification::PARENTED => 18i32, NodeNotification::UNPARENTED => 19i32, NodeNotification::SCENE_INSTANTIATED => 20i32, NodeNotification::DRAG_BEGIN => 21i32, NodeNotification::DRAG_END => 22i32, NodeNotification::PATH_RENAMED => 23i32, NodeNotification::CHILD_ORDER_CHANGED => 24i32, NodeNotification::INTERNAL_PROCESS => 25i32, NodeNotification::INTERNAL_PHYSICS_PROCESS => 26i32, NodeNotification::POST_ENTER_TREE => 27i32, NodeNotification::DISABLED => 28i32, NodeNotification::ENABLED => 29i32, NodeNotification::RESET_PHYSICS_INTERPOLATION => 2001i32, NodeNotification::EDITOR_PRE_SAVE => 9001i32, NodeNotification::EDITOR_POST_SAVE => 9002i32, NodeNotification::WM_MOUSE_ENTER => 1002i32, NodeNotification::WM_MOUSE_EXIT => 1003i32, NodeNotification::WM_WINDOW_FOCUS_IN => 1004i32, NodeNotification::WM_WINDOW_FOCUS_OUT => 1005i32, NodeNotification::WM_CLOSE_REQUEST => 1006i32, NodeNotification::WM_GO_BACK_REQUEST => 1007i32, NodeNotification::WM_SIZE_CHANGED => 1008i32, NodeNotification::WM_DPI_CHANGE => 1009i32, NodeNotification::VP_MOUSE_ENTER => 1010i32, NodeNotification::VP_MOUSE_EXIT => 1011i32, NodeNotification::WM_POSITION_CHANGED => 1012i32, NodeNotification::OS_MEMORY_WARNING => 2009i32, NodeNotification::TRANSLATION_CHANGED => 2010i32, NodeNotification::WM_ABOUT => 2011i32, NodeNotification::CRASH => 2012i32, NodeNotification::OS_IME_UPDATE => 2013i32, NodeNotification::APPLICATION_RESUMED => 2014i32, NodeNotification::APPLICATION_PAUSED => 2015i32, NodeNotification::APPLICATION_FOCUS_IN => 2016i32, NodeNotification::APPLICATION_FOCUS_OUT => 2017i32, NodeNotification::TEXT_SERVER_CHANGED => 2018i32, NodeNotification::ACCESSIBILITY_UPDATE => 3000i32, NodeNotification::ACCESSIBILITY_INVALIDATE => 3001i32, NodeNotification::POSTINITIALIZE => 0i32, NodeNotification::PREDELETE => 1i32, NodeNotification::EXTENSION_RELOADED => 2i32, NodeNotification::Unknown(int) => int,
            }
        }
    }
    impl Node {
        pub fn print_orphan_nodes() {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5606usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "print_orphan_nodes", None, args,)
            }
        }
        pub fn get_orphan_node_ids() -> Array < i64 > {
            type CallRet = Array < i64 >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5607usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_orphan_node_ids", None, args,)
            }
        }
        pub(crate) fn add_sibling_full(&mut self, sibling: CowArg < Option < Gd < crate::classes::Node > > >, force_readable_name: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >, bool,);
            let args = (sibling, force_readable_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5608usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "add_sibling", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_sibling_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_sibling(&mut self, sibling: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            self.add_sibling_ex(sibling,) . done()
        }
        #[inline]
        pub fn add_sibling_ex < 'a > (&'a mut self, sibling: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> ExAddSibling < 'a > {
            ExAddSibling::new(self, sibling,)
        }
        pub fn set_name(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5609usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_name(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5610usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_name", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_child_full(&mut self, node: CowArg < Option < Gd < crate::classes::Node > > >, force_readable_name: bool, internal: crate::classes::node::InternalMode,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >, bool, crate::classes::node::InternalMode,);
            let args = (node, force_readable_name, internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5611usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "add_child", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_child_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_child(&mut self, node: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            self.add_child_ex(node,) . done()
        }
        #[inline]
        pub fn add_child_ex < 'a > (&'a mut self, node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> ExAddChild < 'a > {
            ExAddChild::new(self, node,)
        }
        pub fn remove_child(&mut self, node: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5612usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "remove_child", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn reparent_full(&mut self, new_parent: CowArg < Option < Gd < crate::classes::Node > > >, keep_global_transform: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >, bool,);
            let args = (new_parent, keep_global_transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5613usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "reparent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::reparent_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn reparent(&mut self, new_parent: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            self.reparent_ex(new_parent,) . done()
        }
        #[inline]
        pub fn reparent_ex < 'a > (&'a mut self, new_parent: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> ExReparent < 'a > {
            ExReparent::new(self, new_parent,)
        }
        pub(crate) fn get_child_count_full(&self, include_internal: bool,) -> i32 {
            type CallRet = i32;
            type CallParams = (bool,);
            let args = (include_internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5614usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_child_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_child_count_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_child_count(&self,) -> i32 {
            self.get_child_count_ex() . done()
        }
        #[inline]
        pub fn get_child_count_ex < 'a > (&'a self,) -> ExGetChildCount < 'a > {
            ExGetChildCount::new(self,)
        }
        pub(crate) fn get_children_full(&self, include_internal: bool,) -> Array < Gd < crate::classes::Node > > {
            type CallRet = Array < Gd < crate::classes::Node > >;
            type CallParams = (bool,);
            let args = (include_internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5615usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_children", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_children_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_children(&self,) -> Array < Gd < crate::classes::Node > > {
            self.get_children_ex() . done()
        }
        #[inline]
        pub fn get_children_ex < 'a > (&'a self,) -> ExGetChildren < 'a > {
            ExGetChildren::new(self,)
        }
        pub(crate) fn get_child_full(&self, idx: i32, include_internal: bool,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = (i32, bool,);
            let args = (idx, include_internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5616usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_child", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_child_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_child(&self, idx: i32,) -> Option < Gd < crate::classes::Node > > {
            self.get_child_ex(idx,) . done()
        }
        #[inline]
        pub fn get_child_ex < 'a > (&'a self, idx: i32,) -> ExGetChild < 'a > {
            ExGetChild::new(self, idx,)
        }
        pub fn has_node(&self, path: impl AsArg < NodePath >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5617usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "has_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_or_null(&self, path: impl AsArg < NodePath >,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5618usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_node_or_null", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_parent(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5619usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn find_child_full(&self, pattern: CowArg < GString >, recursive: bool, owned: bool,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool, bool,);
            let args = (pattern, recursive, owned,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5620usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "find_child", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::find_child_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn find_child(&self, pattern: impl AsArg < GString >,) -> Option < Gd < crate::classes::Node > > {
            self.find_child_ex(pattern,) . done()
        }
        #[inline]
        pub fn find_child_ex < 'a > (&'a self, pattern: impl AsArg < GString > + 'a,) -> ExFindChild < 'a > {
            ExFindChild::new(self, pattern,)
        }
        pub(crate) fn find_children_full(&self, pattern: CowArg < GString >, type_: CowArg < GString >, recursive: bool, owned: bool,) -> Array < Gd < crate::classes::Node > > {
            type CallRet = Array < Gd < crate::classes::Node > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, bool, bool,);
            let args = (pattern, type_, recursive, owned,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5621usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "find_children", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::find_children_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn find_children(&self, pattern: impl AsArg < GString >,) -> Array < Gd < crate::classes::Node > > {
            self.find_children_ex(pattern,) . done()
        }
        #[inline]
        pub fn find_children_ex < 'a > (&'a self, pattern: impl AsArg < GString > + 'a,) -> ExFindChildren < 'a > {
            ExFindChildren::new(self, pattern,)
        }
        pub fn find_parent(&self, pattern: impl AsArg < GString >,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (pattern.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5622usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "find_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_node_and_resource(&self, path: impl AsArg < NodePath >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5623usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "has_node_and_resource", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_and_resource(&mut self, path: impl AsArg < NodePath >,) -> VarArray {
            type CallRet = VarArray;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5624usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_node_and_resource", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_inside_tree(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5625usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_inside_tree", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_part_of_edited_scene(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5626usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_part_of_edited_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_ancestor_of(&self, node: impl AsArg < Option < Gd < crate::classes::Node >> >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5627usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_ancestor_of", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_greater_than(&self, node: impl AsArg < Option < Gd < crate::classes::Node >> >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5628usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_greater_than", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_path(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5629usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_path", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_path_to_full(&self, node: CowArg < Option < Gd < crate::classes::Node > > >, use_unique_path: bool,) -> NodePath {
            type CallRet = NodePath;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >, bool,);
            let args = (node, use_unique_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5630usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_path_to", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_path_to_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_path_to(&self, node: impl AsArg < Option < Gd < crate::classes::Node >> >,) -> NodePath {
            self.get_path_to_ex(node,) . done()
        }
        #[inline]
        pub fn get_path_to_ex < 'a > (&'a self, node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> ExGetPathTo < 'a > {
            ExGetPathTo::new(self, node,)
        }
        pub(crate) fn add_to_group_full(&mut self, group: CowArg < StringName >, persistent: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (group, persistent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5631usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "add_to_group", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_to_group_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_to_group(&mut self, group: impl AsArg < StringName >,) {
            self.add_to_group_ex(group,) . done()
        }
        #[inline]
        pub fn add_to_group_ex < 'a > (&'a mut self, group: impl AsArg < StringName > + 'a,) -> ExAddToGroup < 'a > {
            ExAddToGroup::new(self, group,)
        }
        pub fn remove_from_group(&mut self, group: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (group.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5632usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "remove_from_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_in_group(&self, group: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (group.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5633usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_in_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn move_child(&mut self, child_node: impl AsArg < Option < Gd < crate::classes::Node >> >, to_index: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >, i32,);
            let args = (child_node.into_arg(), to_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5634usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "move_child", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_groups(&self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5635usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_groups", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_owner(&mut self, owner: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (owner.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5636usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_owner", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_owner(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5637usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_owner", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_index_full(&self, include_internal: bool,) -> i32 {
            type CallRet = i32;
            type CallParams = (bool,);
            let args = (include_internal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5638usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_index_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_index(&self,) -> i32 {
            self.get_index_ex() . done()
        }
        #[inline]
        pub fn get_index_ex < 'a > (&'a self,) -> ExGetIndex < 'a > {
            ExGetIndex::new(self,)
        }
        pub fn print_tree(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5639usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "print_tree", Some(self.__validated_obj()), args,)
            }
        }
        pub fn print_tree_pretty(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5640usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "print_tree_pretty", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tree_string(&mut self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5641usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_tree_string", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tree_string_pretty(&mut self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5642usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_tree_string_pretty", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scene_file_path(&mut self, scene_file_path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (scene_file_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5643usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_scene_file_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scene_file_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5644usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_scene_file_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn propagate_notification(&mut self, what: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (what,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5645usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "propagate_notification", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn propagate_call_full(&mut self, method: CowArg < StringName >, args: RefArg < VarArray >, parent_first: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, VarArray >, bool,);
            let args = (method, args, parent_first,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5646usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "propagate_call", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::propagate_call_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn propagate_call(&mut self, method: impl AsArg < StringName >,) {
            self.propagate_call_ex(method,) . done()
        }
        #[inline]
        pub fn propagate_call_ex < 'a > (&'a mut self, method: impl AsArg < StringName > + 'a,) -> ExPropagateCall < 'a > {
            ExPropagateCall::new(self, method,)
        }
        pub fn set_physics_process(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5647usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_physics_process", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_process_delta_time(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5648usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_physics_process_delta_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_physics_processing(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5649usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_physics_processing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_delta_time(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5650usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_process_delta_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5651usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_priority(&mut self, priority: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5652usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_priority(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5653usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_process_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_process_priority(&mut self, priority: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5654usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_physics_process_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_process_priority(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5655usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_physics_process_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_processing(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5656usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_processing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_input(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5657usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_input", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_processing_input(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5658usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_processing_input", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_shortcut_input(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5659usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_shortcut_input", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_processing_shortcut_input(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5660usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_processing_shortcut_input", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_unhandled_input(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5661usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_unhandled_input", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_processing_unhandled_input(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5662usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_processing_unhandled_input", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_unhandled_key_input(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5663usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_unhandled_key_input", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_processing_unhandled_key_input(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5664usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_processing_unhandled_key_input", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_mode(&mut self, mode: crate::classes::node::ProcessMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::node::ProcessMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5665usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_mode(&self,) -> crate::classes::node::ProcessMode {
            type CallRet = crate::classes::node::ProcessMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5666usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_process_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn can_process(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5667usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "can_process", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_thread_group(&mut self, mode: crate::classes::node::ProcessThreadGroup,) {
            type CallRet = ();
            type CallParams = (crate::classes::node::ProcessThreadGroup,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_thread_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_thread_group(&self,) -> crate::classes::node::ProcessThreadGroup {
            type CallRet = crate::classes::node::ProcessThreadGroup;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5669usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_process_thread_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_thread_messages(&mut self, flags: crate::classes::node::ProcessThreadMessages,) {
            type CallRet = ();
            type CallParams = (crate::classes::node::ProcessThreadMessages,);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_thread_messages", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_thread_messages(&self,) -> crate::classes::node::ProcessThreadMessages {
            type CallRet = crate::classes::node::ProcessThreadMessages;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_process_thread_messages", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_thread_group_order(&mut self, order: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_thread_group_order", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_thread_group_order(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5673usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_process_thread_group_order", Some(self.__validated_obj()), args,)
            }
        }
        pub fn queue_accessibility_update(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5674usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "queue_accessibility_update", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_accessibility_element(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5675usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_accessibility_element", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_display_folded(&mut self, fold: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (fold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5676usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_display_folded", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_displayed_folded(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5677usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_displayed_folded", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_internal(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5678usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_process_internal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_processing_internal(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5679usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_processing_internal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_process_internal(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5680usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_physics_process_internal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_physics_processing_internal(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5681usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_physics_processing_internal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_interpolation_mode(&mut self, mode: crate::classes::node::PhysicsInterpolationMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::node::PhysicsInterpolationMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5682usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_physics_interpolation_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_interpolation_mode(&self,) -> crate::classes::node::PhysicsInterpolationMode {
            type CallRet = crate::classes::node::PhysicsInterpolationMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5683usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_physics_interpolation_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_physics_interpolated(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5684usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_physics_interpolated", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_physics_interpolated_and_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5685usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_physics_interpolated_and_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reset_physics_interpolation(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5686usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "reset_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_translate_mode(&mut self, mode: crate::classes::node::AutoTranslateMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::node::AutoTranslateMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5687usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_auto_translate_mode(&self,) -> crate::classes::node::AutoTranslateMode {
            type CallRet = crate::classes::node::AutoTranslateMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5688usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn can_auto_translate(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5689usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "can_auto_translate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_translation_domain_inherited(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5690usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_translation_domain_inherited", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_window(&self,) -> Option < Gd < crate::classes::Window > > {
            type CallRet = Option < Gd < crate::classes::Window > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_window", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_last_exclusive_window(&self,) -> Option < Gd < crate::classes::Window > > {
            type CallRet = Option < Gd < crate::classes::Window > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_last_exclusive_window", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tree(&self,) -> Option < Gd < crate::classes::SceneTree > > {
            type CallRet = Option < Gd < crate::classes::SceneTree > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_tree", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_tween(&mut self,) -> Option < Gd < crate::classes::Tween > > {
            type CallRet = Option < Gd < crate::classes::Tween > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "create_tween", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn duplicate_full(&self, flags: crate::classes::node::DuplicateFlags,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = (crate::classes::node::DuplicateFlags,);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "duplicate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::duplicate_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn duplicate(&self,) -> Option < Gd < crate::classes::Node > > {
            self.duplicate_ex() . done()
        }
        #[inline]
        pub fn duplicate_ex < 'a > (&'a self,) -> ExDuplicate < 'a > {
            ExDuplicate::new(self,)
        }
        pub(crate) fn replace_by_full(&mut self, node: CowArg < Option < Gd < crate::classes::Node > > >, keep_groups: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >, bool,);
            let args = (node, keep_groups,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "replace_by", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::replace_by_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn replace_by(&mut self, node: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            self.replace_by_ex(node,) . done()
        }
        #[inline]
        pub fn replace_by_ex < 'a > (&'a mut self, node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> ExReplaceBy < 'a > {
            ExReplaceBy::new(self, node,)
        }
        pub fn set_scene_instance_load_placeholder(&mut self, load_placeholder: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (load_placeholder,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_scene_instance_load_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scene_instance_load_placeholder(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_scene_instance_load_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_editable_instance(&mut self, node: impl AsArg < Option < Gd < crate::classes::Node >> >, is_editable: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >, bool,);
            let args = (node.into_arg(), is_editable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_editable_instance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_editable_instance(&self, node: impl AsArg < Option < Gd < crate::classes::Node >> >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_editable_instance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_viewport(&self,) -> Option < Gd < crate::classes::Viewport > > {
            type CallRet = Option < Gd < crate::classes::Viewport > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_viewport", Some(self.__validated_obj()), args,)
            }
        }
        pub fn queue_free(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5702usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "queue_free", Some(self.__validated_obj()), args,)
            }
        }
        pub fn request_ready(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5703usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "request_ready", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_node_ready(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_node_ready", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn set_multiplayer_authority_full(&mut self, id: i32, recursive: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (id, recursive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_multiplayer_authority", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_multiplayer_authority_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_multiplayer_authority(&mut self, id: i32,) {
            self.set_multiplayer_authority_ex(id,) . done()
        }
        #[inline]
        pub fn set_multiplayer_authority_ex < 'a > (&'a mut self, id: i32,) -> ExSetMultiplayerAuthority < 'a > {
            ExSetMultiplayerAuthority::new(self, id,)
        }
        pub fn get_multiplayer_authority(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_multiplayer_authority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_multiplayer_authority(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5707usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_multiplayer_authority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_multiplayer(&self,) -> Option < Gd < crate::classes::MultiplayerApi > > {
            type CallRet = Option < Gd < crate::classes::MultiplayerApi > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5708usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_multiplayer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rpc_config(&mut self, method: impl AsArg < StringName >, config: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (method.into_arg(), RefArg::new(config),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5709usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "rpc_config", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_rpc_config(&self,) -> Variant {
            type CallRet = Variant;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5710usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_node_rpc_config", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_editor_description(&mut self, editor_description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (editor_description.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5711usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_editor_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_editor_description(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5712usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "get_editor_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_unique_name_in_owner(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5713usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_unique_name_in_owner", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_unique_name_in_owner(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5714usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "is_unique_name_in_owner", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn atr_full(&self, message: CowArg < GString >, context: CowArg < StringName >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, StringName >,);
            let args = (message, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5715usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "atr", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::atr_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn atr(&self, message: impl AsArg < GString >,) -> GString {
            self.atr_ex(message,) . done()
        }
        #[inline]
        pub fn atr_ex < 'a > (&'a self, message: impl AsArg < GString > + 'a,) -> ExAtr < 'a > {
            ExAtr::new(self, message,)
        }
        pub(crate) fn atr_n_full(&self, message: CowArg < GString >, plural_message: CowArg < StringName >, n: i32, context: CowArg < StringName >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, StringName >, i32, CowArg < 'a2, StringName >,);
            let args = (message, plural_message, n, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5716usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "atr_n", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::atr_n_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn atr_n(&self, message: impl AsArg < GString >, plural_message: impl AsArg < StringName >, n: i32,) -> GString {
            self.atr_n_ex(message, plural_message, n,) . done()
        }
        #[inline]
        pub fn atr_n_ex < 'a > (&'a self, message: impl AsArg < GString > + 'a, plural_message: impl AsArg < StringName > + 'a, n: i32,) -> ExAtrN < 'a > {
            ExAtrN::new(self, message, plural_message, n,)
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn rpc(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> crate::global::Error {
            Self::try_rpc(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_rpc(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < crate::global::Error, crate::meta::error::CallError > {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5717usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "Node", "rpc", Some(self.__validated_obj()), args, varargs)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn rpc_id(&mut self, peer_id: i64, method: impl AsArg < StringName >, varargs: &[Variant]) -> crate::global::Error {
            Self::try_rpc_id(self, peer_id, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_rpc_id(&mut self, peer_id: i64, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < crate::global::Error, crate::meta::error::CallError > {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (i64, CowArg < 'a0, StringName >,);
            let args = (peer_id, method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5718usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "Node", "rpc_id", Some(self.__validated_obj()), args, varargs)
            }
        }
        pub fn update_configuration_warnings(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5719usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "update_configuration_warnings", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn call_deferred_thread_group(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Variant {
            Self::try_call_deferred_thread_group(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_call_deferred_thread_group(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < Variant, crate::meta::error::CallError > {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5720usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "Node", "call_deferred_thread_group", Some(self.__validated_obj()), args, varargs)
            }
        }
        pub fn set_deferred_thread_group(&mut self, property: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5721usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_deferred_thread_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn notify_deferred_thread_group(&mut self, what: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (what,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5722usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "notify_deferred_thread_group", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn call_thread_safe(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Variant {
            Self::try_call_thread_safe(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_call_thread_safe(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < Variant, crate::meta::error::CallError > {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5723usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "Node", "call_thread_safe", Some(self.__validated_obj()), args, varargs)
            }
        }
        pub fn set_thread_safe(&mut self, property: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5724usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "set_thread_safe", Some(self.__validated_obj()), args,)
            }
        }
        pub fn notify_thread_safe(&mut self, what: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (what,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5725usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Node", "notify_thread_safe", Some(self.__validated_obj()), args,)
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
        pub fn notify(&mut self, what: NodeNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: NodeNotification) {
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
        pub(crate) const NOTIFICATION_ENTER_TREE: i32 = 10i32;
        pub(crate) const NOTIFICATION_EXIT_TREE: i32 = 11i32;
        pub(crate) const NOTIFICATION_MOVED_IN_PARENT: i32 = 12i32;
        pub(crate) const NOTIFICATION_READY: i32 = 13i32;
        pub(crate) const NOTIFICATION_PAUSED: i32 = 14i32;
        pub(crate) const NOTIFICATION_UNPAUSED: i32 = 15i32;
        pub(crate) const NOTIFICATION_PHYSICS_PROCESS: i32 = 16i32;
        pub(crate) const NOTIFICATION_PROCESS: i32 = 17i32;
        pub(crate) const NOTIFICATION_PARENTED: i32 = 18i32;
        pub(crate) const NOTIFICATION_UNPARENTED: i32 = 19i32;
        pub(crate) const NOTIFICATION_SCENE_INSTANTIATED: i32 = 20i32;
        pub(crate) const NOTIFICATION_DRAG_BEGIN: i32 = 21i32;
        pub(crate) const NOTIFICATION_DRAG_END: i32 = 22i32;
        pub(crate) const NOTIFICATION_PATH_RENAMED: i32 = 23i32;
        pub(crate) const NOTIFICATION_CHILD_ORDER_CHANGED: i32 = 24i32;
        pub(crate) const NOTIFICATION_INTERNAL_PROCESS: i32 = 25i32;
        pub(crate) const NOTIFICATION_INTERNAL_PHYSICS_PROCESS: i32 = 26i32;
        pub(crate) const NOTIFICATION_POST_ENTER_TREE: i32 = 27i32;
        pub(crate) const NOTIFICATION_DISABLED: i32 = 28i32;
        pub(crate) const NOTIFICATION_ENABLED: i32 = 29i32;
        pub(crate) const NOTIFICATION_RESET_PHYSICS_INTERPOLATION: i32 = 2001i32;
        pub(crate) const NOTIFICATION_EDITOR_PRE_SAVE: i32 = 9001i32;
        pub(crate) const NOTIFICATION_EDITOR_POST_SAVE: i32 = 9002i32;
        pub(crate) const NOTIFICATION_WM_MOUSE_ENTER: i32 = 1002i32;
        pub(crate) const NOTIFICATION_WM_MOUSE_EXIT: i32 = 1003i32;
        pub(crate) const NOTIFICATION_WM_WINDOW_FOCUS_IN: i32 = 1004i32;
        pub(crate) const NOTIFICATION_WM_WINDOW_FOCUS_OUT: i32 = 1005i32;
        pub(crate) const NOTIFICATION_WM_CLOSE_REQUEST: i32 = 1006i32;
        pub(crate) const NOTIFICATION_WM_GO_BACK_REQUEST: i32 = 1007i32;
        pub(crate) const NOTIFICATION_WM_SIZE_CHANGED: i32 = 1008i32;
        pub(crate) const NOTIFICATION_WM_DPI_CHANGE: i32 = 1009i32;
        pub(crate) const NOTIFICATION_VP_MOUSE_ENTER: i32 = 1010i32;
        pub(crate) const NOTIFICATION_VP_MOUSE_EXIT: i32 = 1011i32;
        pub(crate) const NOTIFICATION_WM_POSITION_CHANGED: i32 = 1012i32;
        pub(crate) const NOTIFICATION_OS_MEMORY_WARNING: i32 = 2009i32;
        pub(crate) const NOTIFICATION_TRANSLATION_CHANGED: i32 = 2010i32;
        pub(crate) const NOTIFICATION_WM_ABOUT: i32 = 2011i32;
        pub(crate) const NOTIFICATION_CRASH: i32 = 2012i32;
        pub(crate) const NOTIFICATION_OS_IME_UPDATE: i32 = 2013i32;
        pub(crate) const NOTIFICATION_APPLICATION_RESUMED: i32 = 2014i32;
        pub(crate) const NOTIFICATION_APPLICATION_PAUSED: i32 = 2015i32;
        pub(crate) const NOTIFICATION_APPLICATION_FOCUS_IN: i32 = 2016i32;
        pub(crate) const NOTIFICATION_APPLICATION_FOCUS_OUT: i32 = 2017i32;
        pub(crate) const NOTIFICATION_TEXT_SERVER_CHANGED: i32 = 2018i32;
        pub(crate) const NOTIFICATION_ACCESSIBILITY_UPDATE: i32 = 3000i32;
        pub(crate) const NOTIFICATION_ACCESSIBILITY_INVALIDATE: i32 = 3001i32;
        
    }
    impl crate::obj::GodotClass for Node {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Node"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Node {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Node {
        
    }
    impl crate::obj::cap::GodotDefault for Node {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Node {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Node {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Node`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Node__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Node::add_sibling_ex`][super::Node::add_sibling_ex]."]
#[must_use]
pub struct ExAddSibling < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node, sibling: CowArg < 'a, Option < Gd < crate::classes::Node > > >, force_readable_name: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSibling < 'a > {
    fn new(surround_object: &'a mut re_export::Node, sibling: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> Self {
        let force_readable_name = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, sibling: sibling.into_arg(), force_readable_name: force_readable_name,
        }
    }
    #[inline]
    pub fn force_readable_name(self, force_readable_name: bool) -> Self {
        Self {
            force_readable_name: force_readable_name, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, sibling, force_readable_name,
        }
        = self;
        re_export::Node::add_sibling_full(surround_object, sibling, force_readable_name,)
    }
}
#[doc = "Default-param extender for [`Node::add_child_ex`][super::Node::add_child_ex]."]
#[must_use]
pub struct ExAddChild < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node, node: CowArg < 'a, Option < Gd < crate::classes::Node > > >, force_readable_name: bool, internal: crate::classes::node::InternalMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddChild < 'a > {
    fn new(surround_object: &'a mut re_export::Node, node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> Self {
        let force_readable_name = false;
        let internal = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, node: node.into_arg(), force_readable_name: force_readable_name, internal: internal,
        }
    }
    #[inline]
    pub fn force_readable_name(self, force_readable_name: bool) -> Self {
        Self {
            force_readable_name: force_readable_name, .. self
        }
    }
    #[inline]
    pub fn internal(self, internal: crate::classes::node::InternalMode) -> Self {
        Self {
            internal: internal, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, node, force_readable_name, internal,
        }
        = self;
        re_export::Node::add_child_full(surround_object, node, force_readable_name, internal,)
    }
}
#[doc = "Default-param extender for [`Node::reparent_ex`][super::Node::reparent_ex]."]
#[must_use]
pub struct ExReparent < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node, new_parent: CowArg < 'a, Option < Gd < crate::classes::Node > > >, keep_global_transform: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExReparent < 'a > {
    fn new(surround_object: &'a mut re_export::Node, new_parent: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> Self {
        let keep_global_transform = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, new_parent: new_parent.into_arg(), keep_global_transform: keep_global_transform,
        }
    }
    #[inline]
    pub fn keep_global_transform(self, keep_global_transform: bool) -> Self {
        Self {
            keep_global_transform: keep_global_transform, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, new_parent, keep_global_transform,
        }
        = self;
        re_export::Node::reparent_full(surround_object, new_parent, keep_global_transform,)
    }
}
#[doc = "Default-param extender for [`Node::get_child_count_ex`][super::Node::get_child_count_ex]."]
#[must_use]
pub struct ExGetChildCount < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, include_internal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetChildCount < 'a > {
    fn new(surround_object: &'a re_export::Node,) -> Self {
        let include_internal = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, include_internal: include_internal,
        }
    }
    #[inline]
    pub fn include_internal(self, include_internal: bool) -> Self {
        Self {
            include_internal: include_internal, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, include_internal,
        }
        = self;
        re_export::Node::get_child_count_full(surround_object, include_internal,)
    }
}
#[doc = "Default-param extender for [`Node::get_children_ex`][super::Node::get_children_ex]."]
#[must_use]
pub struct ExGetChildren < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, include_internal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetChildren < 'a > {
    fn new(surround_object: &'a re_export::Node,) -> Self {
        let include_internal = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, include_internal: include_internal,
        }
    }
    #[inline]
    pub fn include_internal(self, include_internal: bool) -> Self {
        Self {
            include_internal: include_internal, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, include_internal,
        }
        = self;
        re_export::Node::get_children_full(surround_object, include_internal,)
    }
}
#[doc = "Default-param extender for [`Node::get_child_ex`][super::Node::get_child_ex]."]
#[must_use]
pub struct ExGetChild < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, idx: i32, include_internal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetChild < 'a > {
    fn new(surround_object: &'a re_export::Node, idx: i32,) -> Self {
        let include_internal = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, idx: idx, include_internal: include_internal,
        }
    }
    #[inline]
    pub fn include_internal(self, include_internal: bool) -> Self {
        Self {
            include_internal: include_internal, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, idx, include_internal,
        }
        = self;
        re_export::Node::get_child_full(surround_object, idx, include_internal,)
    }
}
#[doc = "Default-param extender for [`Node::find_child_ex`][super::Node::find_child_ex]."]
#[must_use]
pub struct ExFindChild < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, pattern: CowArg < 'a, GString >, recursive: bool, owned: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFindChild < 'a > {
    fn new(surround_object: &'a re_export::Node, pattern: impl AsArg < GString > + 'a,) -> Self {
        let recursive = true;
        let owned = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, pattern: pattern.into_arg(), recursive: recursive, owned: owned,
        }
    }
    #[inline]
    pub fn recursive(self, recursive: bool) -> Self {
        Self {
            recursive: recursive, .. self
        }
    }
    #[inline]
    pub fn owned(self, owned: bool) -> Self {
        Self {
            owned: owned, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, pattern, recursive, owned,
        }
        = self;
        re_export::Node::find_child_full(surround_object, pattern, recursive, owned,)
    }
}
#[doc = "Default-param extender for [`Node::find_children_ex`][super::Node::find_children_ex]."]
#[must_use]
pub struct ExFindChildren < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, pattern: CowArg < 'a, GString >, type_: CowArg < 'a, GString >, recursive: bool, owned: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFindChildren < 'a > {
    fn new(surround_object: &'a re_export::Node, pattern: impl AsArg < GString > + 'a,) -> Self {
        let type_ = GString::from("");
        let recursive = true;
        let owned = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, pattern: pattern.into_arg(), type_: CowArg::Owned(type_), recursive: recursive, owned: owned,
        }
    }
    #[inline]
    pub fn type_(self, type_: impl AsArg < GString > + 'a) -> Self {
        Self {
            type_: type_.into_arg(), .. self
        }
    }
    #[inline]
    pub fn recursive(self, recursive: bool) -> Self {
        Self {
            recursive: recursive, .. self
        }
    }
    #[inline]
    pub fn owned(self, owned: bool) -> Self {
        Self {
            owned: owned, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, pattern, type_, recursive, owned,
        }
        = self;
        re_export::Node::find_children_full(surround_object, pattern, type_, recursive, owned,)
    }
}
#[doc = "Default-param extender for [`Node::get_path_to_ex`][super::Node::get_path_to_ex]."]
#[must_use]
pub struct ExGetPathTo < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, node: CowArg < 'a, Option < Gd < crate::classes::Node > > >, use_unique_path: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetPathTo < 'a > {
    fn new(surround_object: &'a re_export::Node, node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> Self {
        let use_unique_path = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, node: node.into_arg(), use_unique_path: use_unique_path,
        }
    }
    #[inline]
    pub fn use_unique_path(self, use_unique_path: bool) -> Self {
        Self {
            use_unique_path: use_unique_path, .. self
        }
    }
    #[inline]
    pub fn done(self) -> NodePath {
        let Self {
            _phantom, surround_object, node, use_unique_path,
        }
        = self;
        re_export::Node::get_path_to_full(surround_object, node, use_unique_path,)
    }
}
#[doc = "Default-param extender for [`Node::add_to_group_ex`][super::Node::add_to_group_ex]."]
#[must_use]
pub struct ExAddToGroup < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node, group: CowArg < 'a, StringName >, persistent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddToGroup < 'a > {
    fn new(surround_object: &'a mut re_export::Node, group: impl AsArg < StringName > + 'a,) -> Self {
        let persistent = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, group: group.into_arg(), persistent: persistent,
        }
    }
    #[inline]
    pub fn persistent(self, persistent: bool) -> Self {
        Self {
            persistent: persistent, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, group, persistent,
        }
        = self;
        re_export::Node::add_to_group_full(surround_object, group, persistent,)
    }
}
#[doc = "Default-param extender for [`Node::get_index_ex`][super::Node::get_index_ex]."]
#[must_use]
pub struct ExGetIndex < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, include_internal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetIndex < 'a > {
    fn new(surround_object: &'a re_export::Node,) -> Self {
        let include_internal = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, include_internal: include_internal,
        }
    }
    #[inline]
    pub fn include_internal(self, include_internal: bool) -> Self {
        Self {
            include_internal: include_internal, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, include_internal,
        }
        = self;
        re_export::Node::get_index_full(surround_object, include_internal,)
    }
}
#[doc = "Default-param extender for [`Node::propagate_call_ex`][super::Node::propagate_call_ex]."]
#[must_use]
pub struct ExPropagateCall < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node, method: CowArg < 'a, StringName >, args: CowArg < 'a, VarArray >, parent_first: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPropagateCall < 'a > {
    fn new(surround_object: &'a mut re_export::Node, method: impl AsArg < StringName > + 'a,) -> Self {
        let args = Array::new();
        let parent_first = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, method: method.into_arg(), args: CowArg::Owned(args), parent_first: parent_first,
        }
    }
    #[inline]
    pub fn args(self, args: &'a VarArray) -> Self {
        Self {
            args: CowArg::Borrowed(args), .. self
        }
    }
    #[inline]
    pub fn parent_first(self, parent_first: bool) -> Self {
        Self {
            parent_first: parent_first, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, method, args, parent_first,
        }
        = self;
        re_export::Node::propagate_call_full(surround_object, method, args.cow_as_arg(), parent_first,)
    }
}
#[doc = "Default-param extender for [`Node::duplicate_ex`][super::Node::duplicate_ex]."]
#[must_use]
pub struct ExDuplicate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, flags: crate::classes::node::DuplicateFlags,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDuplicate < 'a > {
    fn new(surround_object: &'a re_export::Node,) -> Self {
        let flags = crate::obj::EngineBitfield::from_ord(15);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, flags: flags,
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::node::DuplicateFlags) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, flags,
        }
        = self;
        re_export::Node::duplicate_full(surround_object, flags,)
    }
}
#[doc = "Default-param extender for [`Node::replace_by_ex`][super::Node::replace_by_ex]."]
#[must_use]
pub struct ExReplaceBy < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node, node: CowArg < 'a, Option < Gd < crate::classes::Node > > >, keep_groups: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExReplaceBy < 'a > {
    fn new(surround_object: &'a mut re_export::Node, node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a,) -> Self {
        let keep_groups = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, node: node.into_arg(), keep_groups: keep_groups,
        }
    }
    #[inline]
    pub fn keep_groups(self, keep_groups: bool) -> Self {
        Self {
            keep_groups: keep_groups, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, node, keep_groups,
        }
        = self;
        re_export::Node::replace_by_full(surround_object, node, keep_groups,)
    }
}
#[doc = "Default-param extender for [`Node::set_multiplayer_authority_ex`][super::Node::set_multiplayer_authority_ex]."]
#[must_use]
pub struct ExSetMultiplayerAuthority < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Node, id: i32, recursive: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetMultiplayerAuthority < 'a > {
    fn new(surround_object: &'a mut re_export::Node, id: i32,) -> Self {
        let recursive = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id, recursive: recursive,
        }
    }
    #[inline]
    pub fn recursive(self, recursive: bool) -> Self {
        Self {
            recursive: recursive, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, id, recursive,
        }
        = self;
        re_export::Node::set_multiplayer_authority_full(surround_object, id, recursive,)
    }
}
#[doc = "Default-param extender for [`Node::atr_ex`][super::Node::atr_ex]."]
#[must_use]
pub struct ExAtr < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, message: CowArg < 'a, GString >, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAtr < 'a > {
    fn new(surround_object: &'a re_export::Node, message: impl AsArg < GString > + 'a,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, message, context,
        }
        = self;
        re_export::Node::atr_full(surround_object, message, context,)
    }
}
#[doc = "Default-param extender for [`Node::atr_n_ex`][super::Node::atr_n_ex]."]
#[must_use]
pub struct ExAtrN < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Node, message: CowArg < 'a, GString >, plural_message: CowArg < 'a, StringName >, n: i32, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAtrN < 'a > {
    fn new(surround_object: &'a re_export::Node, message: impl AsArg < GString > + 'a, plural_message: impl AsArg < StringName > + 'a, n: i32,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), plural_message: plural_message.into_arg(), n: n, context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, message, plural_message, n, context,
        }
        = self;
        re_export::Node::atr_n_full(surround_object, message, plural_message, n, context,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ProcessMode {
    ord: i32
}
impl ProcessMode {
    #[doc(alias = "PROCESS_MODE_INHERIT")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_INHERIT`"]
    pub const INHERIT: ProcessMode = ProcessMode {
        ord: 0i32
    };
    #[doc(alias = "PROCESS_MODE_PAUSABLE")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_PAUSABLE`"]
    pub const PAUSABLE: ProcessMode = ProcessMode {
        ord: 1i32
    };
    #[doc(alias = "PROCESS_MODE_WHEN_PAUSED")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_WHEN_PAUSED`"]
    pub const WHEN_PAUSED: ProcessMode = ProcessMode {
        ord: 2i32
    };
    #[doc(alias = "PROCESS_MODE_ALWAYS")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_ALWAYS`"]
    pub const ALWAYS: ProcessMode = ProcessMode {
        ord: 3i32
    };
    #[doc(alias = "PROCESS_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `PROCESS_MODE_DISABLED`"]
    pub const DISABLED: ProcessMode = ProcessMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ProcessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ProcessMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ProcessMode {
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
            Self::INHERIT => "INHERIT", Self::PAUSABLE => "PAUSABLE", Self::WHEN_PAUSED => "WHEN_PAUSED", Self::ALWAYS => "ALWAYS", Self::DISABLED => "DISABLED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ProcessMode::INHERIT, ProcessMode::PAUSABLE, ProcessMode::WHEN_PAUSED, ProcessMode::ALWAYS, ProcessMode::DISABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ProcessMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INHERIT", "PROCESS_MODE_INHERIT", ProcessMode::INHERIT), crate::meta::inspect::EnumConstant::new("PAUSABLE", "PROCESS_MODE_PAUSABLE", ProcessMode::PAUSABLE), crate::meta::inspect::EnumConstant::new("WHEN_PAUSED", "PROCESS_MODE_WHEN_PAUSED", ProcessMode::WHEN_PAUSED), crate::meta::inspect::EnumConstant::new("ALWAYS", "PROCESS_MODE_ALWAYS", ProcessMode::ALWAYS), crate::meta::inspect::EnumConstant::new("DISABLED", "PROCESS_MODE_DISABLED", ProcessMode::DISABLED)]
        }
    }
}
impl crate::meta::GodotConvert for ProcessMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ProcessMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ProcessMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ProcessThreadGroup {
    ord: i32
}
impl ProcessThreadGroup {
    #[doc(alias = "PROCESS_THREAD_GROUP_INHERIT")]
    #[doc = "Godot enumerator name: `PROCESS_THREAD_GROUP_INHERIT`"]
    pub const INHERIT: ProcessThreadGroup = ProcessThreadGroup {
        ord: 0i32
    };
    #[doc(alias = "PROCESS_THREAD_GROUP_MAIN_THREAD")]
    #[doc = "Godot enumerator name: `PROCESS_THREAD_GROUP_MAIN_THREAD`"]
    pub const MAIN_THREAD: ProcessThreadGroup = ProcessThreadGroup {
        ord: 1i32
    };
    #[doc(alias = "PROCESS_THREAD_GROUP_SUB_THREAD")]
    #[doc = "Godot enumerator name: `PROCESS_THREAD_GROUP_SUB_THREAD`"]
    pub const SUB_THREAD: ProcessThreadGroup = ProcessThreadGroup {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ProcessThreadGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ProcessThreadGroup") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ProcessThreadGroup {
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
            Self::INHERIT => "INHERIT", Self::MAIN_THREAD => "MAIN_THREAD", Self::SUB_THREAD => "SUB_THREAD", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ProcessThreadGroup::INHERIT, ProcessThreadGroup::MAIN_THREAD, ProcessThreadGroup::SUB_THREAD]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ProcessThreadGroup >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INHERIT", "PROCESS_THREAD_GROUP_INHERIT", ProcessThreadGroup::INHERIT), crate::meta::inspect::EnumConstant::new("MAIN_THREAD", "PROCESS_THREAD_GROUP_MAIN_THREAD", ProcessThreadGroup::MAIN_THREAD), crate::meta::inspect::EnumConstant::new("SUB_THREAD", "PROCESS_THREAD_GROUP_SUB_THREAD", ProcessThreadGroup::SUB_THREAD)]
        }
    }
}
impl crate::meta::GodotConvert for ProcessThreadGroup {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ProcessThreadGroup {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ProcessThreadGroup {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ProcessThreadMessages {
    ord: u64
}
impl ProcessThreadMessages {
    #[doc(alias = "FLAG_PROCESS_THREAD_MESSAGES")]
    #[doc = "Godot enumerator name: `FLAG_PROCESS_THREAD_MESSAGES`"]
    pub const MESSAGES: ProcessThreadMessages = ProcessThreadMessages {
        ord: 1u64
    };
    #[doc(alias = "FLAG_PROCESS_THREAD_MESSAGES_PHYSICS")]
    #[doc = "Godot enumerator name: `FLAG_PROCESS_THREAD_MESSAGES_PHYSICS`"]
    pub const MESSAGES_PHYSICS: ProcessThreadMessages = ProcessThreadMessages {
        ord: 2u64
    };
    #[doc(alias = "FLAG_PROCESS_THREAD_MESSAGES_ALL")]
    #[doc = "Godot enumerator name: `FLAG_PROCESS_THREAD_MESSAGES_ALL`"]
    pub const MESSAGES_ALL: ProcessThreadMessages = ProcessThreadMessages {
        ord: 3u64
    };
    
}
impl std::fmt::Debug for ProcessThreadMessages {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::MESSAGES => "MESSAGES", Self::MESSAGES_PHYSICS => "MESSAGES_PHYSICS", Self::MESSAGES_ALL => "MESSAGES_ALL", _ => {
                f.debug_struct("ProcessThreadMessages") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for ProcessThreadMessages {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ProcessThreadMessages >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("MESSAGES", "FLAG_PROCESS_THREAD_MESSAGES", ProcessThreadMessages::MESSAGES), crate::meta::inspect::EnumConstant::new("MESSAGES_PHYSICS", "FLAG_PROCESS_THREAD_MESSAGES_PHYSICS", ProcessThreadMessages::MESSAGES_PHYSICS), crate::meta::inspect::EnumConstant::new("MESSAGES_ALL", "FLAG_PROCESS_THREAD_MESSAGES_ALL", ProcessThreadMessages::MESSAGES_ALL)]
        }
    }
}
impl std::ops::BitOr for ProcessThreadMessages {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for ProcessThreadMessages {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for ProcessThreadMessages {
    type Via = u64;
    
}
impl crate::meta::ToGodot for ProcessThreadMessages {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ProcessThreadMessages {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PhysicsInterpolationMode {
    ord: i32
}
impl PhysicsInterpolationMode {
    #[doc(alias = "PHYSICS_INTERPOLATION_MODE_INHERIT")]
    #[doc = "Godot enumerator name: `PHYSICS_INTERPOLATION_MODE_INHERIT`"]
    pub const INHERIT: PhysicsInterpolationMode = PhysicsInterpolationMode {
        ord: 0i32
    };
    #[doc(alias = "PHYSICS_INTERPOLATION_MODE_ON")]
    #[doc = "Godot enumerator name: `PHYSICS_INTERPOLATION_MODE_ON`"]
    pub const ON: PhysicsInterpolationMode = PhysicsInterpolationMode {
        ord: 1i32
    };
    #[doc(alias = "PHYSICS_INTERPOLATION_MODE_OFF")]
    #[doc = "Godot enumerator name: `PHYSICS_INTERPOLATION_MODE_OFF`"]
    pub const OFF: PhysicsInterpolationMode = PhysicsInterpolationMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PhysicsInterpolationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PhysicsInterpolationMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PhysicsInterpolationMode {
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
            Self::INHERIT => "INHERIT", Self::ON => "ON", Self::OFF => "OFF", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PhysicsInterpolationMode::INHERIT, PhysicsInterpolationMode::ON, PhysicsInterpolationMode::OFF]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PhysicsInterpolationMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INHERIT", "PHYSICS_INTERPOLATION_MODE_INHERIT", PhysicsInterpolationMode::INHERIT), crate::meta::inspect::EnumConstant::new("ON", "PHYSICS_INTERPOLATION_MODE_ON", PhysicsInterpolationMode::ON), crate::meta::inspect::EnumConstant::new("OFF", "PHYSICS_INTERPOLATION_MODE_OFF", PhysicsInterpolationMode::OFF)]
        }
    }
}
impl crate::meta::GodotConvert for PhysicsInterpolationMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PhysicsInterpolationMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PhysicsInterpolationMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct DuplicateFlags {
    ord: u64
}
impl DuplicateFlags {
    #[doc(alias = "DUPLICATE_SIGNALS")]
    #[doc = "Godot enumerator name: `DUPLICATE_SIGNALS`"]
    pub const SIGNALS: DuplicateFlags = DuplicateFlags {
        ord: 1u64
    };
    #[doc(alias = "DUPLICATE_GROUPS")]
    #[doc = "Godot enumerator name: `DUPLICATE_GROUPS`"]
    pub const GROUPS: DuplicateFlags = DuplicateFlags {
        ord: 2u64
    };
    #[doc(alias = "DUPLICATE_SCRIPTS")]
    #[doc = "Godot enumerator name: `DUPLICATE_SCRIPTS`"]
    pub const SCRIPTS: DuplicateFlags = DuplicateFlags {
        ord: 4u64
    };
    #[doc(alias = "DUPLICATE_USE_INSTANTIATION")]
    #[doc = "Godot enumerator name: `DUPLICATE_USE_INSTANTIATION`"]
    pub const USE_INSTANTIATION: DuplicateFlags = DuplicateFlags {
        ord: 8u64
    };
    
}
impl std::fmt::Debug for DuplicateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::SIGNALS => "SIGNALS", Self::GROUPS => "GROUPS", Self::SCRIPTS => "SCRIPTS", Self::USE_INSTANTIATION => "USE_INSTANTIATION", _ => {
                f.debug_struct("DuplicateFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for DuplicateFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DuplicateFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SIGNALS", "DUPLICATE_SIGNALS", DuplicateFlags::SIGNALS), crate::meta::inspect::EnumConstant::new("GROUPS", "DUPLICATE_GROUPS", DuplicateFlags::GROUPS), crate::meta::inspect::EnumConstant::new("SCRIPTS", "DUPLICATE_SCRIPTS", DuplicateFlags::SCRIPTS), crate::meta::inspect::EnumConstant::new("USE_INSTANTIATION", "DUPLICATE_USE_INSTANTIATION", DuplicateFlags::USE_INSTANTIATION)]
        }
    }
}
impl std::ops::BitOr for DuplicateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for DuplicateFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for DuplicateFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for DuplicateFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DuplicateFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct InternalMode {
    ord: i32
}
impl InternalMode {
    #[doc(alias = "INTERNAL_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `INTERNAL_MODE_DISABLED`"]
    pub const DISABLED: InternalMode = InternalMode {
        ord: 0i32
    };
    #[doc(alias = "INTERNAL_MODE_FRONT")]
    #[doc = "Godot enumerator name: `INTERNAL_MODE_FRONT`"]
    pub const FRONT: InternalMode = InternalMode {
        ord: 1i32
    };
    #[doc(alias = "INTERNAL_MODE_BACK")]
    #[doc = "Godot enumerator name: `INTERNAL_MODE_BACK`"]
    pub const BACK: InternalMode = InternalMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for InternalMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("InternalMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for InternalMode {
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
            Self::DISABLED => "DISABLED", Self::FRONT => "FRONT", Self::BACK => "BACK", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[InternalMode::DISABLED, InternalMode::FRONT, InternalMode::BACK]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < InternalMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "INTERNAL_MODE_DISABLED", InternalMode::DISABLED), crate::meta::inspect::EnumConstant::new("FRONT", "INTERNAL_MODE_FRONT", InternalMode::FRONT), crate::meta::inspect::EnumConstant::new("BACK", "INTERNAL_MODE_BACK", InternalMode::BACK)]
        }
    }
}
impl crate::meta::GodotConvert for InternalMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for InternalMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for InternalMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AutoTranslateMode {
    ord: i32
}
impl AutoTranslateMode {
    #[doc(alias = "AUTO_TRANSLATE_MODE_INHERIT")]
    #[doc = "Godot enumerator name: `AUTO_TRANSLATE_MODE_INHERIT`"]
    pub const INHERIT: AutoTranslateMode = AutoTranslateMode {
        ord: 0i32
    };
    #[doc(alias = "AUTO_TRANSLATE_MODE_ALWAYS")]
    #[doc = "Godot enumerator name: `AUTO_TRANSLATE_MODE_ALWAYS`"]
    pub const ALWAYS: AutoTranslateMode = AutoTranslateMode {
        ord: 1i32
    };
    #[doc(alias = "AUTO_TRANSLATE_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `AUTO_TRANSLATE_MODE_DISABLED`"]
    pub const DISABLED: AutoTranslateMode = AutoTranslateMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AutoTranslateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AutoTranslateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AutoTranslateMode {
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
            Self::INHERIT => "INHERIT", Self::ALWAYS => "ALWAYS", Self::DISABLED => "DISABLED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AutoTranslateMode::INHERIT, AutoTranslateMode::ALWAYS, AutoTranslateMode::DISABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AutoTranslateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INHERIT", "AUTO_TRANSLATE_MODE_INHERIT", AutoTranslateMode::INHERIT), crate::meta::inspect::EnumConstant::new("ALWAYS", "AUTO_TRANSLATE_MODE_ALWAYS", AutoTranslateMode::ALWAYS), crate::meta::inspect::EnumConstant::new("DISABLED", "AUTO_TRANSLATE_MODE_DISABLED", AutoTranslateMode::DISABLED)]
        }
    }
}
impl crate::meta::GodotConvert for AutoTranslateMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AutoTranslateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AutoTranslateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Node;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Node`][crate::classes::Node] class."]
    pub struct SignalsOfNode < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfNode < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn ready(&mut self) -> SigReady < 'c, C > {
            SigReady {
                typed: TypedSignal::extract(&mut self.__internal_obj, "ready")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn renamed(&mut self) -> SigRenamed < 'c, C > {
            SigRenamed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "renamed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn tree_entered(&mut self) -> SigTreeEntered < 'c, C > {
            SigTreeEntered {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tree_entered")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn tree_exiting(&mut self) -> SigTreeExiting < 'c, C > {
            SigTreeExiting {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tree_exiting")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn tree_exited(&mut self) -> SigTreeExited < 'c, C > {
            SigTreeExited {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tree_exited")
            }
        }
        #[doc = "Signature: `(node: Gd<Node>)`"]
        pub fn child_entered_tree(&mut self) -> SigChildEnteredTree < 'c, C > {
            SigChildEnteredTree {
                typed: TypedSignal::extract(&mut self.__internal_obj, "child_entered_tree")
            }
        }
        #[doc = "Signature: `(node: Gd<Node>)`"]
        pub fn child_exiting_tree(&mut self) -> SigChildExitingTree < 'c, C > {
            SigChildExitingTree {
                typed: TypedSignal::extract(&mut self.__internal_obj, "child_exiting_tree")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn child_order_changed(&mut self) -> SigChildOrderChanged < 'c, C > {
            SigChildOrderChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "child_order_changed")
            }
        }
        #[doc = "Signature: `(node: Gd<Node>)`"]
        pub fn replacing_by(&mut self) -> SigReplacingBy < 'c, C > {
            SigReplacingBy {
                typed: TypedSignal::extract(&mut self.__internal_obj, "replacing_by")
            }
        }
        #[doc = "Signature: `(node: Gd<Node>)`"]
        pub fn editor_description_changed(&mut self) -> SigEditorDescriptionChanged < 'c, C > {
            SigEditorDescriptionChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "editor_description_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn editor_state_changed(&mut self) -> SigEditorStateChanged < 'c, C > {
            SigEditorStateChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "editor_state_changed")
            }
        }
    }
    type TypedSigReady < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigReady < 'c, C: WithSignals > {
        typed: TypedSigReady < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigReady < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigReady < 'c, C > {
        type Target = TypedSigReady < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigReady < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigRenamed < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigRenamed < 'c, C: WithSignals > {
        typed: TypedSigRenamed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigRenamed < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigRenamed < 'c, C > {
        type Target = TypedSigRenamed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigRenamed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTreeEntered < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTreeEntered < 'c, C: WithSignals > {
        typed: TypedSigTreeEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTreeEntered < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTreeEntered < 'c, C > {
        type Target = TypedSigTreeEntered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTreeEntered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTreeExiting < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTreeExiting < 'c, C: WithSignals > {
        typed: TypedSigTreeExiting < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTreeExiting < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTreeExiting < 'c, C > {
        type Target = TypedSigTreeExiting < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTreeExiting < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTreeExited < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTreeExited < 'c, C: WithSignals > {
        typed: TypedSigTreeExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTreeExited < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTreeExited < 'c, C > {
        type Target = TypedSigTreeExited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTreeExited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigChildEnteredTree < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigChildEnteredTree < 'c, C: WithSignals > {
        typed: TypedSigChildEnteredTree < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigChildEnteredTree < 'c, C > {
        pub fn emit(&mut self, node: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((node,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigChildEnteredTree < 'c, C > {
        type Target = TypedSigChildEnteredTree < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigChildEnteredTree < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigChildExitingTree < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigChildExitingTree < 'c, C: WithSignals > {
        typed: TypedSigChildExitingTree < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigChildExitingTree < 'c, C > {
        pub fn emit(&mut self, node: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((node,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigChildExitingTree < 'c, C > {
        type Target = TypedSigChildExitingTree < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigChildExitingTree < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigChildOrderChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigChildOrderChanged < 'c, C: WithSignals > {
        typed: TypedSigChildOrderChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigChildOrderChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigChildOrderChanged < 'c, C > {
        type Target = TypedSigChildOrderChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigChildOrderChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigReplacingBy < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigReplacingBy < 'c, C: WithSignals > {
        typed: TypedSigReplacingBy < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigReplacingBy < 'c, C > {
        pub fn emit(&mut self, node: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((node,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigReplacingBy < 'c, C > {
        type Target = TypedSigReplacingBy < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigReplacingBy < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigEditorDescriptionChanged < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigEditorDescriptionChanged < 'c, C: WithSignals > {
        typed: TypedSigEditorDescriptionChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigEditorDescriptionChanged < 'c, C > {
        pub fn emit(&mut self, node: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((node,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigEditorDescriptionChanged < 'c, C > {
        type Target = TypedSigEditorDescriptionChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigEditorDescriptionChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigEditorStateChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigEditorStateChanged < 'c, C: WithSignals > {
        typed: TypedSigEditorStateChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigEditorStateChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigEditorStateChanged < 'c, C > {
        type Target = TypedSigEditorStateChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigEditorStateChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Node {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfNode < 'c, C > {
        type Target = < < Node as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Node;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfNode < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Node;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}