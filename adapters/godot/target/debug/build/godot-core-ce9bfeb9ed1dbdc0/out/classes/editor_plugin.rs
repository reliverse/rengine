#![doc = "Sidecar module for class [`EditorPlugin`][crate::classes::EditorPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorPlugin.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`editor_plugin`][crate::classes::editor_plugin]: sidecar module with related enum/flag types\n* [`IEditorPlugin`][crate::classes::IEditorPlugin]: virtual methods\n* [`SignalsOfEditorPlugin`][crate::classes::editor_plugin::SignalsOfEditorPlugin]: signal collection\n\n\nSee also [Godot docs for `EditorPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorplugin.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`EditorPlugin::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorPlugin`][crate::classes::EditorPlugin].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `EditorPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorplugin.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorPlugin: crate::obj::GodotClass < Base = EditorPlugin > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn forward_canvas_gui_input(&mut self, event: Option < Gd < crate::classes::InputEvent > >,) -> bool {
            unimplemented !()
        }
        fn forward_canvas_draw_over_viewport(&mut self, viewport_control: Option < Gd < crate::classes::Control > >,) {
            unimplemented !()
        }
        fn forward_canvas_force_draw_over_viewport(&mut self, viewport_control: Option < Gd < crate::classes::Control > >,) {
            unimplemented !()
        }
        fn forward_3d_gui_input(&mut self, viewport_camera: Option < Gd < crate::classes::Camera3D > >, event: Option < Gd < crate::classes::InputEvent > >,) -> i32 {
            unimplemented !()
        }
        fn forward_3d_draw_over_viewport(&mut self, viewport_control: Option < Gd < crate::classes::Control > >,) {
            unimplemented !()
        }
        fn forward_3d_force_draw_over_viewport(&mut self, viewport_control: Option < Gd < crate::classes::Control > >,) {
            unimplemented !()
        }
        fn get_plugin_name(&self,) -> GString {
            unimplemented !()
        }
        fn get_plugin_icon(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            unimplemented !()
        }
        fn has_main_screen(&self,) -> bool {
            unimplemented !()
        }
        fn make_visible(&mut self, visible: bool,) {
            unimplemented !()
        }
        fn edit(&mut self, object: Option < Gd < crate::classes::Object > >,) {
            unimplemented !()
        }
        fn handles(&self, object: Gd < crate::classes::Object >,) -> bool {
            unimplemented !()
        }
        fn get_state(&self,) -> VarDictionary {
            unimplemented !()
        }
        fn set_state(&mut self, state: VarDictionary,) {
            unimplemented !()
        }
        fn clear(&mut self,) {
            unimplemented !()
        }
        fn get_unsaved_status(&self, for_scene: GString,) -> GString {
            unimplemented !()
        }
        fn save_external_data(&mut self,) {
            unimplemented !()
        }
        fn apply_changes(&mut self,) {
            unimplemented !()
        }
        fn get_breakpoints(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn set_window_layout(&mut self, configuration: Option < Gd < crate::classes::ConfigFile > >,) {
            unimplemented !()
        }
        fn get_window_layout(&mut self, configuration: Option < Gd < crate::classes::ConfigFile > >,) {
            unimplemented !()
        }
        fn build(&mut self,) -> bool {
            unimplemented !()
        }
        fn enable_plugin(&mut self,) {
            unimplemented !()
        }
        fn disable_plugin(&mut self,) {
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
    impl EditorPlugin {
        pub fn add_control_to_container(&mut self, container: crate::classes::editor_plugin::CustomControlContainer, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::editor_plugin::CustomControlContainer, CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (container, control.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_control_to_container", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_control_to_bottom_panel_full(&mut self, control: CowArg < Option < Gd < crate::classes::Control > > >, title: CowArg < GString >, shortcut: CowArg < Option < Gd < crate::classes::Shortcut > > >,) -> Option < Gd < crate::classes::Button > > {
            type CallRet = Option < Gd < crate::classes::Button > >;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >, CowArg < 'a1, GString >, CowArg < 'a2, Option < Gd < crate::classes::Shortcut > > >,);
            let args = (control, title, shortcut,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_control_to_bottom_panel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_control_to_bottom_panel_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_control_to_bottom_panel(&mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> >, title: impl AsArg < GString >,) -> Option < Gd < crate::classes::Button > > {
            self.add_control_to_bottom_panel_ex(control, title,) . done()
        }
        #[inline]
        pub fn add_control_to_bottom_panel_ex < 'a > (&'a mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> > + 'a, title: impl AsArg < GString > + 'a,) -> ExAddControlToBottomPanel < 'a > {
            ExAddControlToBottomPanel::new(self, control, title,)
        }
        pub(crate) fn add_control_to_dock_full(&mut self, slot: crate::classes::editor_plugin::DockSlot, control: CowArg < Option < Gd < crate::classes::Control > > >, shortcut: CowArg < Option < Gd < crate::classes::Shortcut > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (crate::classes::editor_plugin::DockSlot, CowArg < 'a0, Option < Gd < crate::classes::Control > > >, CowArg < 'a1, Option < Gd < crate::classes::Shortcut > > >,);
            let args = (slot, control, shortcut,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_control_to_dock", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_control_to_dock_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_control_to_dock(&mut self, slot: crate::classes::editor_plugin::DockSlot, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            self.add_control_to_dock_ex(slot, control,) . done()
        }
        #[inline]
        pub fn add_control_to_dock_ex < 'a > (&'a mut self, slot: crate::classes::editor_plugin::DockSlot, control: impl AsArg < Option < Gd < crate::classes::Control >> > + 'a,) -> ExAddControlToDock < 'a > {
            ExAddControlToDock::new(self, slot, control,)
        }
        pub fn remove_control_from_docks(&mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (control.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_control_from_docks", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_control_from_bottom_panel(&mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (control.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_control_from_bottom_panel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_control_from_container(&mut self, container: crate::classes::editor_plugin::CustomControlContainer, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::editor_plugin::CustomControlContainer, CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (container, control.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_control_from_container", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_dock_tab_icon(&mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> >, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (control.into_arg(), icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "set_dock_tab_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_tool_menu_item(&mut self, name: impl AsArg < GString >, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, Callable >,);
            let args = (name.into_arg(), RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_tool_menu_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_tool_submenu_item(&mut self, name: impl AsArg < GString >, submenu: impl AsArg < Option < Gd < crate::classes::PopupMenu >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::PopupMenu > > >,);
            let args = (name.into_arg(), submenu.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_tool_submenu_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_tool_menu_item(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_tool_menu_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_export_as_menu(&mut self,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallRet = Option < Gd < crate::classes::PopupMenu > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "get_export_as_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_custom_type(&mut self, type_: impl AsArg < GString >, base: impl AsArg < GString >, script: impl AsArg < Option < Gd < crate::classes::Script >> >, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, Option < Gd < crate::classes::Script > > >, CowArg < 'a3, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (type_.into_arg(), base.into_arg(), script.into_arg(), icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_custom_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_custom_type(&mut self, type_: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (type_.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_custom_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_autoload_singleton(&mut self, name: impl AsArg < GString >, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (name.into_arg(), path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_autoload_singleton", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_autoload_singleton(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_autoload_singleton", Some(self.__validated_obj()), args,)
            }
        }
        pub fn update_overlays(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "update_overlays", Some(self.__validated_obj()), args,)
            }
        }
        pub fn make_bottom_panel_item_visible(&mut self, item: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (item.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "make_bottom_panel_item_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn hide_bottom_panel(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "hide_bottom_panel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_undo_redo(&mut self,) -> Option < Gd < crate::classes::EditorUndoRedoManager > > {
            type CallRet = Option < Gd < crate::classes::EditorUndoRedoManager > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "get_undo_redo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_undo_redo_inspector_hook_callback(&mut self, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_undo_redo_inspector_hook_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_undo_redo_inspector_hook_callback(&mut self, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_undo_redo_inspector_hook_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn queue_save_layout(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(280usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "queue_save_layout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_translation_parser_plugin(&mut self, parser: impl AsArg < Option < Gd < crate::classes::EditorTranslationParserPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorTranslationParserPlugin > > >,);
            let args = (parser.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(281usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_translation_parser_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_translation_parser_plugin(&mut self, parser: impl AsArg < Option < Gd < crate::classes::EditorTranslationParserPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorTranslationParserPlugin > > >,);
            let args = (parser.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(282usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_translation_parser_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_import_plugin_full(&mut self, importer: CowArg < Option < Gd < crate::classes::EditorImportPlugin > > >, first_priority: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorImportPlugin > > >, bool,);
            let args = (importer, first_priority,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(283usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_import_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_import_plugin_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_import_plugin(&mut self, importer: impl AsArg < Option < Gd < crate::classes::EditorImportPlugin >> >,) {
            self.add_import_plugin_ex(importer,) . done()
        }
        #[inline]
        pub fn add_import_plugin_ex < 'a > (&'a mut self, importer: impl AsArg < Option < Gd < crate::classes::EditorImportPlugin >> > + 'a,) -> ExAddImportPlugin < 'a > {
            ExAddImportPlugin::new(self, importer,)
        }
        pub fn remove_import_plugin(&mut self, importer: impl AsArg < Option < Gd < crate::classes::EditorImportPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorImportPlugin > > >,);
            let args = (importer.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(284usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_import_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_scene_format_importer_plugin_full(&mut self, scene_format_importer: CowArg < Option < Gd < crate::classes::EditorSceneFormatImporter > > >, first_priority: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorSceneFormatImporter > > >, bool,);
            let args = (scene_format_importer, first_priority,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(285usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_scene_format_importer_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_scene_format_importer_plugin_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_scene_format_importer_plugin(&mut self, scene_format_importer: impl AsArg < Option < Gd < crate::classes::EditorSceneFormatImporter >> >,) {
            self.add_scene_format_importer_plugin_ex(scene_format_importer,) . done()
        }
        #[inline]
        pub fn add_scene_format_importer_plugin_ex < 'a > (&'a mut self, scene_format_importer: impl AsArg < Option < Gd < crate::classes::EditorSceneFormatImporter >> > + 'a,) -> ExAddSceneFormatImporterPlugin < 'a > {
            ExAddSceneFormatImporterPlugin::new(self, scene_format_importer,)
        }
        pub fn remove_scene_format_importer_plugin(&mut self, scene_format_importer: impl AsArg < Option < Gd < crate::classes::EditorSceneFormatImporter >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorSceneFormatImporter > > >,);
            let args = (scene_format_importer.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(286usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_scene_format_importer_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_scene_post_import_plugin_full(&mut self, scene_import_plugin: CowArg < Option < Gd < crate::classes::EditorScenePostImportPlugin > > >, first_priority: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorScenePostImportPlugin > > >, bool,);
            let args = (scene_import_plugin, first_priority,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(287usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_scene_post_import_plugin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_scene_post_import_plugin_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_scene_post_import_plugin(&mut self, scene_import_plugin: impl AsArg < Option < Gd < crate::classes::EditorScenePostImportPlugin >> >,) {
            self.add_scene_post_import_plugin_ex(scene_import_plugin,) . done()
        }
        #[inline]
        pub fn add_scene_post_import_plugin_ex < 'a > (&'a mut self, scene_import_plugin: impl AsArg < Option < Gd < crate::classes::EditorScenePostImportPlugin >> > + 'a,) -> ExAddScenePostImportPlugin < 'a > {
            ExAddScenePostImportPlugin::new(self, scene_import_plugin,)
        }
        pub fn remove_scene_post_import_plugin(&mut self, scene_import_plugin: impl AsArg < Option < Gd < crate::classes::EditorScenePostImportPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorScenePostImportPlugin > > >,);
            let args = (scene_import_plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(288usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_scene_post_import_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_export_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorExportPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(289usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_export_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_export_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorExportPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(290usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_export_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_export_platform(&mut self, platform: impl AsArg < Option < Gd < crate::classes::EditorExportPlatform >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPlatform > > >,);
            let args = (platform.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(291usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_export_platform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_export_platform(&mut self, platform: impl AsArg < Option < Gd < crate::classes::EditorExportPlatform >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPlatform > > >,);
            let args = (platform.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(292usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_export_platform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_node_3d_gizmo_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorNode3DGizmoPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorNode3DGizmoPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(293usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_node_3d_gizmo_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_node_3d_gizmo_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorNode3DGizmoPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorNode3DGizmoPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(294usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_node_3d_gizmo_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_inspector_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorInspectorPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorInspectorPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(295usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_inspector_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_inspector_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorInspectorPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorInspectorPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(296usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_inspector_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_resource_conversion_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorResourceConversionPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorResourceConversionPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(297usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_resource_conversion_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_resource_conversion_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorResourceConversionPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorResourceConversionPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(298usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_resource_conversion_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_input_event_forwarding_always_enabled(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(299usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "set_input_event_forwarding_always_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_force_draw_over_forwarding_enabled(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(300usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "set_force_draw_over_forwarding_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_context_menu_plugin(&mut self, slot: crate::classes::editor_context_menu_plugin::ContextMenuSlot, plugin: impl AsArg < Option < Gd < crate::classes::EditorContextMenuPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::editor_context_menu_plugin::ContextMenuSlot, CowArg < 'a0, Option < Gd < crate::classes::EditorContextMenuPlugin > > >,);
            let args = (slot, plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(301usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_context_menu_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_context_menu_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorContextMenuPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorContextMenuPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(302usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_context_menu_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_editor_interface(&mut self,) -> Option < Gd < crate::classes::EditorInterface > > {
            type CallRet = Option < Gd < crate::classes::EditorInterface > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(303usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "get_editor_interface", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_script_create_dialog(&mut self,) -> Option < Gd < crate::classes::ScriptCreateDialog > > {
            type CallRet = Option < Gd < crate::classes::ScriptCreateDialog > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(304usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "get_script_create_dialog", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_debugger_plugin(&mut self, script: impl AsArg < Option < Gd < crate::classes::EditorDebuggerPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorDebuggerPlugin > > >,);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(305usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "add_debugger_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_debugger_plugin(&mut self, script: impl AsArg < Option < Gd < crate::classes::EditorDebuggerPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorDebuggerPlugin > > >,);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "remove_debugger_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_plugin_version(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorPlugin", "get_plugin_version", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EditorPlugin {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorPlugin"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorPlugin {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for EditorPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorPlugin {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorPlugin`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorPlugin__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorPlugin > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_control_to_bottom_panel_ex`][super::EditorPlugin::add_control_to_bottom_panel_ex]."]
#[must_use]
pub struct ExAddControlToBottomPanel < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorPlugin, control: CowArg < 'a, Option < Gd < crate::classes::Control > > >, title: CowArg < 'a, GString >, shortcut: CowArg < 'a, Option < Gd < crate::classes::Shortcut > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddControlToBottomPanel < 'a > {
    fn new(surround_object: &'a mut re_export::EditorPlugin, control: impl AsArg < Option < Gd < crate::classes::Control >> > + 'a, title: impl AsArg < GString > + 'a,) -> Self {
        let shortcut = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, control: control.into_arg(), title: title.into_arg(), shortcut: shortcut.into_arg(),
        }
    }
    #[inline]
    pub fn shortcut(self, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'a) -> Self {
        Self {
            shortcut: shortcut.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Button > > {
        let Self {
            _phantom, surround_object, control, title, shortcut,
        }
        = self;
        re_export::EditorPlugin::add_control_to_bottom_panel_full(surround_object, control, title, shortcut,)
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_control_to_dock_ex`][super::EditorPlugin::add_control_to_dock_ex]."]
#[must_use]
pub struct ExAddControlToDock < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorPlugin, slot: crate::classes::editor_plugin::DockSlot, control: CowArg < 'a, Option < Gd < crate::classes::Control > > >, shortcut: CowArg < 'a, Option < Gd < crate::classes::Shortcut > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddControlToDock < 'a > {
    fn new(surround_object: &'a mut re_export::EditorPlugin, slot: crate::classes::editor_plugin::DockSlot, control: impl AsArg < Option < Gd < crate::classes::Control >> > + 'a,) -> Self {
        let shortcut = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, slot: slot, control: control.into_arg(), shortcut: shortcut.into_arg(),
        }
    }
    #[inline]
    pub fn shortcut(self, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'a) -> Self {
        Self {
            shortcut: shortcut.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, slot, control, shortcut,
        }
        = self;
        re_export::EditorPlugin::add_control_to_dock_full(surround_object, slot, control, shortcut,)
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_import_plugin_ex`][super::EditorPlugin::add_import_plugin_ex]."]
#[must_use]
pub struct ExAddImportPlugin < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorPlugin, importer: CowArg < 'a, Option < Gd < crate::classes::EditorImportPlugin > > >, first_priority: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddImportPlugin < 'a > {
    fn new(surround_object: &'a mut re_export::EditorPlugin, importer: impl AsArg < Option < Gd < crate::classes::EditorImportPlugin >> > + 'a,) -> Self {
        let first_priority = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, importer: importer.into_arg(), first_priority: first_priority,
        }
    }
    #[inline]
    pub fn first_priority(self, first_priority: bool) -> Self {
        Self {
            first_priority: first_priority, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, importer, first_priority,
        }
        = self;
        re_export::EditorPlugin::add_import_plugin_full(surround_object, importer, first_priority,)
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_scene_format_importer_plugin_ex`][super::EditorPlugin::add_scene_format_importer_plugin_ex]."]
#[must_use]
pub struct ExAddSceneFormatImporterPlugin < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorPlugin, scene_format_importer: CowArg < 'a, Option < Gd < crate::classes::EditorSceneFormatImporter > > >, first_priority: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSceneFormatImporterPlugin < 'a > {
    fn new(surround_object: &'a mut re_export::EditorPlugin, scene_format_importer: impl AsArg < Option < Gd < crate::classes::EditorSceneFormatImporter >> > + 'a,) -> Self {
        let first_priority = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, scene_format_importer: scene_format_importer.into_arg(), first_priority: first_priority,
        }
    }
    #[inline]
    pub fn first_priority(self, first_priority: bool) -> Self {
        Self {
            first_priority: first_priority, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, scene_format_importer, first_priority,
        }
        = self;
        re_export::EditorPlugin::add_scene_format_importer_plugin_full(surround_object, scene_format_importer, first_priority,)
    }
}
#[doc = "Default-param extender for [`EditorPlugin::add_scene_post_import_plugin_ex`][super::EditorPlugin::add_scene_post_import_plugin_ex]."]
#[must_use]
pub struct ExAddScenePostImportPlugin < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorPlugin, scene_import_plugin: CowArg < 'a, Option < Gd < crate::classes::EditorScenePostImportPlugin > > >, first_priority: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddScenePostImportPlugin < 'a > {
    fn new(surround_object: &'a mut re_export::EditorPlugin, scene_import_plugin: impl AsArg < Option < Gd < crate::classes::EditorScenePostImportPlugin >> > + 'a,) -> Self {
        let first_priority = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, scene_import_plugin: scene_import_plugin.into_arg(), first_priority: first_priority,
        }
    }
    #[inline]
    pub fn first_priority(self, first_priority: bool) -> Self {
        Self {
            first_priority: first_priority, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, scene_import_plugin, first_priority,
        }
        = self;
        re_export::EditorPlugin::add_scene_post_import_plugin_full(surround_object, scene_import_plugin, first_priority,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CustomControlContainer {
    ord: i32
}
impl CustomControlContainer {
    #[doc(alias = "CONTAINER_TOOLBAR")]
    #[doc = "Godot enumerator name: `CONTAINER_TOOLBAR`"]
    pub const TOOLBAR: CustomControlContainer = CustomControlContainer {
        ord: 0i32
    };
    #[doc(alias = "CONTAINER_SPATIAL_EDITOR_MENU")]
    #[doc = "Godot enumerator name: `CONTAINER_SPATIAL_EDITOR_MENU`"]
    pub const SPATIAL_EDITOR_MENU: CustomControlContainer = CustomControlContainer {
        ord: 1i32
    };
    #[doc(alias = "CONTAINER_SPATIAL_EDITOR_SIDE_LEFT")]
    #[doc = "Godot enumerator name: `CONTAINER_SPATIAL_EDITOR_SIDE_LEFT`"]
    pub const SPATIAL_EDITOR_SIDE_LEFT: CustomControlContainer = CustomControlContainer {
        ord: 2i32
    };
    #[doc(alias = "CONTAINER_SPATIAL_EDITOR_SIDE_RIGHT")]
    #[doc = "Godot enumerator name: `CONTAINER_SPATIAL_EDITOR_SIDE_RIGHT`"]
    pub const SPATIAL_EDITOR_SIDE_RIGHT: CustomControlContainer = CustomControlContainer {
        ord: 3i32
    };
    #[doc(alias = "CONTAINER_SPATIAL_EDITOR_BOTTOM")]
    #[doc = "Godot enumerator name: `CONTAINER_SPATIAL_EDITOR_BOTTOM`"]
    pub const SPATIAL_EDITOR_BOTTOM: CustomControlContainer = CustomControlContainer {
        ord: 4i32
    };
    #[doc(alias = "CONTAINER_CANVAS_EDITOR_MENU")]
    #[doc = "Godot enumerator name: `CONTAINER_CANVAS_EDITOR_MENU`"]
    pub const CANVAS_EDITOR_MENU: CustomControlContainer = CustomControlContainer {
        ord: 5i32
    };
    #[doc(alias = "CONTAINER_CANVAS_EDITOR_SIDE_LEFT")]
    #[doc = "Godot enumerator name: `CONTAINER_CANVAS_EDITOR_SIDE_LEFT`"]
    pub const CANVAS_EDITOR_SIDE_LEFT: CustomControlContainer = CustomControlContainer {
        ord: 6i32
    };
    #[doc(alias = "CONTAINER_CANVAS_EDITOR_SIDE_RIGHT")]
    #[doc = "Godot enumerator name: `CONTAINER_CANVAS_EDITOR_SIDE_RIGHT`"]
    pub const CANVAS_EDITOR_SIDE_RIGHT: CustomControlContainer = CustomControlContainer {
        ord: 7i32
    };
    #[doc(alias = "CONTAINER_CANVAS_EDITOR_BOTTOM")]
    #[doc = "Godot enumerator name: `CONTAINER_CANVAS_EDITOR_BOTTOM`"]
    pub const CANVAS_EDITOR_BOTTOM: CustomControlContainer = CustomControlContainer {
        ord: 8i32
    };
    #[doc(alias = "CONTAINER_INSPECTOR_BOTTOM")]
    #[doc = "Godot enumerator name: `CONTAINER_INSPECTOR_BOTTOM`"]
    pub const INSPECTOR_BOTTOM: CustomControlContainer = CustomControlContainer {
        ord: 9i32
    };
    #[doc(alias = "CONTAINER_PROJECT_SETTING_TAB_LEFT")]
    #[doc = "Godot enumerator name: `CONTAINER_PROJECT_SETTING_TAB_LEFT`"]
    pub const PROJECT_SETTING_TAB_LEFT: CustomControlContainer = CustomControlContainer {
        ord: 10i32
    };
    #[doc(alias = "CONTAINER_PROJECT_SETTING_TAB_RIGHT")]
    #[doc = "Godot enumerator name: `CONTAINER_PROJECT_SETTING_TAB_RIGHT`"]
    pub const PROJECT_SETTING_TAB_RIGHT: CustomControlContainer = CustomControlContainer {
        ord: 11i32
    };
    
}
impl std::fmt::Debug for CustomControlContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CustomControlContainer") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CustomControlContainer {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 => Some(Self {
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
            Self::TOOLBAR => "TOOLBAR", Self::SPATIAL_EDITOR_MENU => "SPATIAL_EDITOR_MENU", Self::SPATIAL_EDITOR_SIDE_LEFT => "SPATIAL_EDITOR_SIDE_LEFT", Self::SPATIAL_EDITOR_SIDE_RIGHT => "SPATIAL_EDITOR_SIDE_RIGHT", Self::SPATIAL_EDITOR_BOTTOM => "SPATIAL_EDITOR_BOTTOM", Self::CANVAS_EDITOR_MENU => "CANVAS_EDITOR_MENU", Self::CANVAS_EDITOR_SIDE_LEFT => "CANVAS_EDITOR_SIDE_LEFT", Self::CANVAS_EDITOR_SIDE_RIGHT => "CANVAS_EDITOR_SIDE_RIGHT", Self::CANVAS_EDITOR_BOTTOM => "CANVAS_EDITOR_BOTTOM", Self::INSPECTOR_BOTTOM => "INSPECTOR_BOTTOM", Self::PROJECT_SETTING_TAB_LEFT => "PROJECT_SETTING_TAB_LEFT", Self::PROJECT_SETTING_TAB_RIGHT => "PROJECT_SETTING_TAB_RIGHT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CustomControlContainer::TOOLBAR, CustomControlContainer::SPATIAL_EDITOR_MENU, CustomControlContainer::SPATIAL_EDITOR_SIDE_LEFT, CustomControlContainer::SPATIAL_EDITOR_SIDE_RIGHT, CustomControlContainer::SPATIAL_EDITOR_BOTTOM, CustomControlContainer::CANVAS_EDITOR_MENU, CustomControlContainer::CANVAS_EDITOR_SIDE_LEFT, CustomControlContainer::CANVAS_EDITOR_SIDE_RIGHT, CustomControlContainer::CANVAS_EDITOR_BOTTOM, CustomControlContainer::INSPECTOR_BOTTOM, CustomControlContainer::PROJECT_SETTING_TAB_LEFT, CustomControlContainer::PROJECT_SETTING_TAB_RIGHT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CustomControlContainer >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TOOLBAR", "CONTAINER_TOOLBAR", CustomControlContainer::TOOLBAR), crate::meta::inspect::EnumConstant::new("SPATIAL_EDITOR_MENU", "CONTAINER_SPATIAL_EDITOR_MENU", CustomControlContainer::SPATIAL_EDITOR_MENU), crate::meta::inspect::EnumConstant::new("SPATIAL_EDITOR_SIDE_LEFT", "CONTAINER_SPATIAL_EDITOR_SIDE_LEFT", CustomControlContainer::SPATIAL_EDITOR_SIDE_LEFT), crate::meta::inspect::EnumConstant::new("SPATIAL_EDITOR_SIDE_RIGHT", "CONTAINER_SPATIAL_EDITOR_SIDE_RIGHT", CustomControlContainer::SPATIAL_EDITOR_SIDE_RIGHT), crate::meta::inspect::EnumConstant::new("SPATIAL_EDITOR_BOTTOM", "CONTAINER_SPATIAL_EDITOR_BOTTOM", CustomControlContainer::SPATIAL_EDITOR_BOTTOM), crate::meta::inspect::EnumConstant::new("CANVAS_EDITOR_MENU", "CONTAINER_CANVAS_EDITOR_MENU", CustomControlContainer::CANVAS_EDITOR_MENU), crate::meta::inspect::EnumConstant::new("CANVAS_EDITOR_SIDE_LEFT", "CONTAINER_CANVAS_EDITOR_SIDE_LEFT", CustomControlContainer::CANVAS_EDITOR_SIDE_LEFT), crate::meta::inspect::EnumConstant::new("CANVAS_EDITOR_SIDE_RIGHT", "CONTAINER_CANVAS_EDITOR_SIDE_RIGHT", CustomControlContainer::CANVAS_EDITOR_SIDE_RIGHT), crate::meta::inspect::EnumConstant::new("CANVAS_EDITOR_BOTTOM", "CONTAINER_CANVAS_EDITOR_BOTTOM", CustomControlContainer::CANVAS_EDITOR_BOTTOM), crate::meta::inspect::EnumConstant::new("INSPECTOR_BOTTOM", "CONTAINER_INSPECTOR_BOTTOM", CustomControlContainer::INSPECTOR_BOTTOM), crate::meta::inspect::EnumConstant::new("PROJECT_SETTING_TAB_LEFT", "CONTAINER_PROJECT_SETTING_TAB_LEFT", CustomControlContainer::PROJECT_SETTING_TAB_LEFT), crate::meta::inspect::EnumConstant::new("PROJECT_SETTING_TAB_RIGHT", "CONTAINER_PROJECT_SETTING_TAB_RIGHT", CustomControlContainer::PROJECT_SETTING_TAB_RIGHT)]
        }
    }
}
impl crate::meta::GodotConvert for CustomControlContainer {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CustomControlContainer {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CustomControlContainer {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DockSlot {
    ord: i32
}
impl DockSlot {
    #[doc(alias = "DOCK_SLOT_LEFT_UL")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_LEFT_UL`"]
    pub const LEFT_UL: DockSlot = DockSlot {
        ord: 0i32
    };
    #[doc(alias = "DOCK_SLOT_LEFT_BL")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_LEFT_BL`"]
    pub const LEFT_BL: DockSlot = DockSlot {
        ord: 1i32
    };
    #[doc(alias = "DOCK_SLOT_LEFT_UR")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_LEFT_UR`"]
    pub const LEFT_UR: DockSlot = DockSlot {
        ord: 2i32
    };
    #[doc(alias = "DOCK_SLOT_LEFT_BR")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_LEFT_BR`"]
    pub const LEFT_BR: DockSlot = DockSlot {
        ord: 3i32
    };
    #[doc(alias = "DOCK_SLOT_RIGHT_UL")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_RIGHT_UL`"]
    pub const RIGHT_UL: DockSlot = DockSlot {
        ord: 4i32
    };
    #[doc(alias = "DOCK_SLOT_RIGHT_BL")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_RIGHT_BL`"]
    pub const RIGHT_BL: DockSlot = DockSlot {
        ord: 5i32
    };
    #[doc(alias = "DOCK_SLOT_RIGHT_UR")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_RIGHT_UR`"]
    pub const RIGHT_UR: DockSlot = DockSlot {
        ord: 6i32
    };
    #[doc(alias = "DOCK_SLOT_RIGHT_BR")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_RIGHT_BR`"]
    pub const RIGHT_BR: DockSlot = DockSlot {
        ord: 7i32
    };
    #[doc(alias = "DOCK_SLOT_MAX")]
    #[doc = "Godot enumerator name: `DOCK_SLOT_MAX`"]
    pub const MAX: DockSlot = DockSlot {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for DockSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DockSlot") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DockSlot {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::LEFT_UL => "LEFT_UL", Self::LEFT_BL => "LEFT_BL", Self::LEFT_UR => "LEFT_UR", Self::LEFT_BR => "LEFT_BR", Self::RIGHT_UL => "RIGHT_UL", Self::RIGHT_BL => "RIGHT_BL", Self::RIGHT_UR => "RIGHT_UR", Self::RIGHT_BR => "RIGHT_BR", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DockSlot::LEFT_UL, DockSlot::LEFT_BL, DockSlot::LEFT_UR, DockSlot::LEFT_BR, DockSlot::RIGHT_UL, DockSlot::RIGHT_BL, DockSlot::RIGHT_UR, DockSlot::RIGHT_BR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DockSlot >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LEFT_UL", "DOCK_SLOT_LEFT_UL", DockSlot::LEFT_UL), crate::meta::inspect::EnumConstant::new("LEFT_BL", "DOCK_SLOT_LEFT_BL", DockSlot::LEFT_BL), crate::meta::inspect::EnumConstant::new("LEFT_UR", "DOCK_SLOT_LEFT_UR", DockSlot::LEFT_UR), crate::meta::inspect::EnumConstant::new("LEFT_BR", "DOCK_SLOT_LEFT_BR", DockSlot::LEFT_BR), crate::meta::inspect::EnumConstant::new("RIGHT_UL", "DOCK_SLOT_RIGHT_UL", DockSlot::RIGHT_UL), crate::meta::inspect::EnumConstant::new("RIGHT_BL", "DOCK_SLOT_RIGHT_BL", DockSlot::RIGHT_BL), crate::meta::inspect::EnumConstant::new("RIGHT_UR", "DOCK_SLOT_RIGHT_UR", DockSlot::RIGHT_UR), crate::meta::inspect::EnumConstant::new("RIGHT_BR", "DOCK_SLOT_RIGHT_BR", DockSlot::RIGHT_BR), crate::meta::inspect::EnumConstant::new("MAX", "DOCK_SLOT_MAX", DockSlot::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for DockSlot {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for DockSlot {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DockSlot {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DockSlot {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `AfterGUIInput`."]
pub struct AfterGuiInput {
    ord: i32
}
impl AfterGuiInput {
    #[doc(alias = "AFTER_GUI_INPUT_PASS")]
    #[doc = "Godot enumerator name: `AFTER_GUI_INPUT_PASS`"]
    pub const PASS: AfterGuiInput = AfterGuiInput {
        ord: 0i32
    };
    #[doc(alias = "AFTER_GUI_INPUT_STOP")]
    #[doc = "Godot enumerator name: `AFTER_GUI_INPUT_STOP`"]
    pub const STOP: AfterGuiInput = AfterGuiInput {
        ord: 1i32
    };
    #[doc(alias = "AFTER_GUI_INPUT_CUSTOM")]
    #[doc = "Godot enumerator name: `AFTER_GUI_INPUT_CUSTOM`"]
    pub const CUSTOM: AfterGuiInput = AfterGuiInput {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AfterGuiInput {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AfterGuiInput") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AfterGuiInput {
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
            Self::PASS => "PASS", Self::STOP => "STOP", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AfterGuiInput::PASS, AfterGuiInput::STOP, AfterGuiInput::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AfterGuiInput >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PASS", "AFTER_GUI_INPUT_PASS", AfterGuiInput::PASS), crate::meta::inspect::EnumConstant::new("STOP", "AFTER_GUI_INPUT_STOP", AfterGuiInput::STOP), crate::meta::inspect::EnumConstant::new("CUSTOM", "AFTER_GUI_INPUT_CUSTOM", AfterGuiInput::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for AfterGuiInput {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AfterGuiInput {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AfterGuiInput {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorPlugin;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`EditorPlugin`][crate::classes::EditorPlugin] class."]
    pub struct SignalsOfEditorPlugin < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfEditorPlugin < 'c, C > {
        #[doc = "Signature: `(scene_root: Gd<Node>)`"]
        pub fn scene_changed(&mut self) -> SigSceneChanged < 'c, C > {
            SigSceneChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "scene_changed")
            }
        }
        #[doc = "Signature: `(filepath: GString)`"]
        pub fn scene_closed(&mut self) -> SigSceneClosed < 'c, C > {
            SigSceneClosed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "scene_closed")
            }
        }
        #[doc = "Signature: `(screen_name: GString)`"]
        pub fn main_screen_changed(&mut self) -> SigMainScreenChanged < 'c, C > {
            SigMainScreenChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "main_screen_changed")
            }
        }
        #[doc = "Signature: `(resource: Gd<Resource>)`"]
        pub fn resource_saved(&mut self) -> SigResourceSaved < 'c, C > {
            SigResourceSaved {
                typed: TypedSignal::extract(&mut self.__internal_obj, "resource_saved")
            }
        }
        #[doc = "Signature: `(filepath: GString)`"]
        pub fn scene_saved(&mut self) -> SigSceneSaved < 'c, C > {
            SigSceneSaved {
                typed: TypedSignal::extract(&mut self.__internal_obj, "scene_saved")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn project_settings_changed(&mut self) -> SigProjectSettingsChanged < 'c, C > {
            SigProjectSettingsChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "project_settings_changed")
            }
        }
    }
    type TypedSigSceneChanged < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigSceneChanged < 'c, C: WithSignals > {
        typed: TypedSigSceneChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSceneChanged < 'c, C > {
        pub fn emit(&mut self, scene_root: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((scene_root,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSceneChanged < 'c, C > {
        type Target = TypedSigSceneChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSceneChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSceneClosed < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigSceneClosed < 'c, C: WithSignals > {
        typed: TypedSigSceneClosed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSceneClosed < 'c, C > {
        pub fn emit(&mut self, filepath: GString,) {
            self.typed.emit_tuple((filepath,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSceneClosed < 'c, C > {
        type Target = TypedSigSceneClosed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSceneClosed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMainScreenChanged < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigMainScreenChanged < 'c, C: WithSignals > {
        typed: TypedSigMainScreenChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMainScreenChanged < 'c, C > {
        pub fn emit(&mut self, screen_name: GString,) {
            self.typed.emit_tuple((screen_name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMainScreenChanged < 'c, C > {
        type Target = TypedSigMainScreenChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMainScreenChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigResourceSaved < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Resource >,) >;
    pub struct SigResourceSaved < 'c, C: WithSignals > {
        typed: TypedSigResourceSaved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigResourceSaved < 'c, C > {
        pub fn emit(&mut self, resource: Gd < crate::classes::Resource >,) {
            self.typed.emit_tuple((resource,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigResourceSaved < 'c, C > {
        type Target = TypedSigResourceSaved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigResourceSaved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSceneSaved < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigSceneSaved < 'c, C: WithSignals > {
        typed: TypedSigSceneSaved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSceneSaved < 'c, C > {
        pub fn emit(&mut self, filepath: GString,) {
            self.typed.emit_tuple((filepath,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSceneSaved < 'c, C > {
        type Target = TypedSigSceneSaved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSceneSaved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigProjectSettingsChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigProjectSettingsChanged < 'c, C: WithSignals > {
        typed: TypedSigProjectSettingsChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigProjectSettingsChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigProjectSettingsChanged < 'c, C > {
        type Target = TypedSigProjectSettingsChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigProjectSettingsChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for EditorPlugin {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfEditorPlugin < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfEditorPlugin < 'c, C > {
        type Target = < < EditorPlugin as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = EditorPlugin;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfEditorPlugin < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = EditorPlugin;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}