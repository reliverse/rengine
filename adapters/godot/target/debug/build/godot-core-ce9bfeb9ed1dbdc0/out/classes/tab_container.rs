#![doc = "Sidecar module for class [`TabContainer`][crate::classes::TabContainer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TabContainer` enums](https://docs.godotengine.org/en/stable/classes/class_tabcontainer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TabContainer.`\n\nInherits [`Container`][crate::classes::Container].\n\nRelated symbols:\n\n* [`tab_container`][crate::classes::tab_container]: sidecar module with related enum/flag types\n* [`ITabContainer`][crate::classes::ITabContainer]: virtual methods\n* [`SignalsOfTabContainer`][crate::classes::tab_container::SignalsOfTabContainer]: signal collection\n\n\nSee also [Godot docs for `TabContainer`](https://docs.godotengine.org/en/stable/classes/class_tabcontainer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`TabContainer::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TabContainer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TabContainer`][crate::classes::TabContainer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IContainer`][crate::classes::IContainer] > [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `TabContainer` methods](https://docs.godotengine.org/en/stable/classes/class_tabcontainer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITabContainer: crate::obj::GodotClass < Base = TabContainer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TabContainer {
        pub fn get_tab_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8824usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_current_tab(&mut self, tab_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8825usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_current_tab", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_tab(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8826usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_current_tab", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_previous_tab(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8827usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_previous_tab", Some(self.__validated_obj()), args,)
            }
        }
        pub fn select_previous_available(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8828usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "select_previous_available", Some(self.__validated_obj()), args,)
            }
        }
        pub fn select_next_available(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8829usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "select_next_available", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_tab_control(&self,) -> Option < Gd < crate::classes::Control > > {
            type CallRet = Option < Gd < crate::classes::Control > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8830usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_current_tab_control", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_bar(&self,) -> Option < Gd < crate::classes::TabBar > > {
            type CallRet = Option < Gd < crate::classes::TabBar > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8831usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_bar", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_control(&self, tab_idx: i32,) -> Option < Gd < crate::classes::Control > > {
            type CallRet = Option < Gd < crate::classes::Control > >;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8832usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_control", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_alignment(&mut self, alignment: crate::classes::tab_bar::AlignmentMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::tab_bar::AlignmentMode,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8833usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_alignment(&self,) -> crate::classes::tab_bar::AlignmentMode {
            type CallRet = crate::classes::tab_bar::AlignmentMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8834usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tabs_position(&mut self, tabs_position: crate::classes::tab_container::TabPosition,) {
            type CallRet = ();
            type CallParams = (crate::classes::tab_container::TabPosition,);
            let args = (tabs_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8835usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_tabs_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tabs_position(&self,) -> crate::classes::tab_container::TabPosition {
            type CallRet = crate::classes::tab_container::TabPosition;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8836usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_tabs_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_clip_tabs(&mut self, clip_tabs: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (clip_tabs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8837usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_clip_tabs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_clip_tabs(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8838usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_clip_tabs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tabs_visible(&mut self, visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8839usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_tabs_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn are_tabs_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8840usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "are_tabs_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_all_tabs_in_front(&mut self, is_front: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (is_front,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8841usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_all_tabs_in_front", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_all_tabs_in_front(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8842usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "is_all_tabs_in_front", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_title(&mut self, tab_idx: i32, title: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (tab_idx, title.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8843usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_title", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_title(&self, tab_idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8844usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_title", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_tooltip(&mut self, tab_idx: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (tab_idx, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8845usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_tooltip(&self, tab_idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8846usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_icon(&mut self, tab_idx: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (tab_idx, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8847usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_icon(&self, tab_idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8848usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_icon_max_width(&mut self, tab_idx: i32, width: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (tab_idx, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8849usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_icon_max_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_icon_max_width(&self, tab_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8850usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_icon_max_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_disabled(&mut self, tab_idx: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (tab_idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8851usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_tab_disabled(&self, tab_idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8852usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "is_tab_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_hidden(&mut self, tab_idx: i32, hidden: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (tab_idx, hidden,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8853usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_hidden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_tab_hidden(&self, tab_idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8854usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "is_tab_hidden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_metadata(&mut self, tab_idx: i32, metadata: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (tab_idx, RefArg::new(metadata),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8855usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_metadata", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_metadata(&self, tab_idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8856usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_metadata", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_button_icon(&mut self, tab_idx: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (tab_idx, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8857usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_button_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_button_icon(&self, tab_idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (tab_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8858usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_button_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_idx_at_point(&self, point: Vector2,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2,);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_idx_at_point", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_idx_from_control(&self, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (control.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_idx_from_control", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_popup(&mut self, popup: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (popup.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8861usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_popup", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_popup(&self,) -> Option < Gd < crate::classes::Popup > > {
            type CallRet = Option < Gd < crate::classes::Popup > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8862usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_popup", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_to_rearrange_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8863usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_drag_to_rearrange_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_drag_to_rearrange_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8864usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_drag_to_rearrange_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tabs_rearrange_group(&mut self, group_id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8865usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_tabs_rearrange_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tabs_rearrange_group(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8866usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_tabs_rearrange_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_hidden_tabs_for_min_size(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8867usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_use_hidden_tabs_for_min_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_use_hidden_tabs_for_min_size(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8868usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_use_hidden_tabs_for_min_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_focus_mode(&mut self, focus_mode: crate::classes::control::FocusMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::FocusMode,);
            let args = (focus_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8869usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_tab_focus_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_focus_mode(&self,) -> crate::classes::control::FocusMode {
            type CallRet = crate::classes::control::FocusMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8870usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_tab_focus_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_deselect_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8871usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "set_deselect_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_deselect_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8872usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TabContainer", "get_deselect_enabled", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TabContainer {
        type Base = crate::classes::Container;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TabContainer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TabContainer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Container > for TabContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for TabContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for TabContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for TabContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TabContainer {
        
    }
    impl crate::obj::cap::GodotDefault for TabContainer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TabContainer {
        type Target = crate::classes::Container;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TabContainer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TabContainer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TabContainer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TabContainer > for $Class {
                
            }
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TabPosition {
    ord: i32
}
impl TabPosition {
    #[doc(alias = "POSITION_TOP")]
    #[doc = "Godot enumerator name: `POSITION_TOP`"]
    pub const TOP: TabPosition = TabPosition {
        ord: 0i32
    };
    #[doc(alias = "POSITION_BOTTOM")]
    #[doc = "Godot enumerator name: `POSITION_BOTTOM`"]
    pub const BOTTOM: TabPosition = TabPosition {
        ord: 1i32
    };
    #[doc(alias = "POSITION_MAX")]
    #[doc = "Godot enumerator name: `POSITION_MAX`"]
    pub const MAX: TabPosition = TabPosition {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TabPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TabPosition") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TabPosition {
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
            Self::TOP => "TOP", Self::BOTTOM => "BOTTOM", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TabPosition::TOP, TabPosition::BOTTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TabPosition >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TOP", "POSITION_TOP", TabPosition::TOP), crate::meta::inspect::EnumConstant::new("BOTTOM", "POSITION_BOTTOM", TabPosition::BOTTOM), crate::meta::inspect::EnumConstant::new("MAX", "POSITION_MAX", TabPosition::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for TabPosition {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for TabPosition {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TabPosition {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TabPosition {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TabContainer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`TabContainer`][crate::classes::TabContainer] class."]
    pub struct SignalsOfTabContainer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfTabContainer < 'c, C > {
        #[doc = "Signature: `(idx_to: i64)`"]
        pub fn active_tab_rearranged(&mut self) -> SigActiveTabRearranged < 'c, C > {
            SigActiveTabRearranged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "active_tab_rearranged")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_changed(&mut self) -> SigTabChanged < 'c, C > {
            SigTabChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tab_changed")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_clicked(&mut self) -> SigTabClicked < 'c, C > {
            SigTabClicked {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tab_clicked")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_hovered(&mut self) -> SigTabHovered < 'c, C > {
            SigTabHovered {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tab_hovered")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_selected(&mut self) -> SigTabSelected < 'c, C > {
            SigTabSelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tab_selected")
            }
        }
        #[doc = "Signature: `(tab: i64)`"]
        pub fn tab_button_pressed(&mut self) -> SigTabButtonPressed < 'c, C > {
            SigTabButtonPressed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tab_button_pressed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn pre_popup_pressed(&mut self) -> SigPrePopupPressed < 'c, C > {
            SigPrePopupPressed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "pre_popup_pressed")
            }
        }
    }
    type TypedSigActiveTabRearranged < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigActiveTabRearranged < 'c, C: WithSignals > {
        typed: TypedSigActiveTabRearranged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigActiveTabRearranged < 'c, C > {
        pub fn emit(&mut self, idx_to: i64,) {
            self.typed.emit_tuple((idx_to,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigActiveTabRearranged < 'c, C > {
        type Target = TypedSigActiveTabRearranged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigActiveTabRearranged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabChanged < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabChanged < 'c, C: WithSignals > {
        typed: TypedSigTabChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabChanged < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabChanged < 'c, C > {
        type Target = TypedSigTabChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabClicked < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabClicked < 'c, C: WithSignals > {
        typed: TypedSigTabClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabClicked < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabClicked < 'c, C > {
        type Target = TypedSigTabClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabHovered < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabHovered < 'c, C: WithSignals > {
        typed: TypedSigTabHovered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabHovered < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabHovered < 'c, C > {
        type Target = TypedSigTabHovered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabHovered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabSelected < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabSelected < 'c, C: WithSignals > {
        typed: TypedSigTabSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabSelected < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabSelected < 'c, C > {
        type Target = TypedSigTabSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTabButtonPressed < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigTabButtonPressed < 'c, C: WithSignals > {
        typed: TypedSigTabButtonPressed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTabButtonPressed < 'c, C > {
        pub fn emit(&mut self, tab: i64,) {
            self.typed.emit_tuple((tab,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTabButtonPressed < 'c, C > {
        type Target = TypedSigTabButtonPressed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTabButtonPressed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigPrePopupPressed < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigPrePopupPressed < 'c, C: WithSignals > {
        typed: TypedSigPrePopupPressed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPrePopupPressed < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPrePopupPressed < 'c, C > {
        type Target = TypedSigPrePopupPressed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPrePopupPressed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for TabContainer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfTabContainer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfTabContainer < 'c, C > {
        type Target = < < TabContainer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = TabContainer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfTabContainer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = TabContainer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}