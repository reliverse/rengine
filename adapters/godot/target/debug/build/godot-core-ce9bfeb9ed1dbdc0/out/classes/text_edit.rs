#![doc = "Sidecar module for class [`TextEdit`][crate::classes::TextEdit].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextEdit` enums](https://docs.godotengine.org/en/stable/classes/class_textedit.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextEdit.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`text_edit`][crate::classes::text_edit]: sidecar module with related enum/flag types\n* [`ITextEdit`][crate::classes::ITextEdit]: virtual methods\n* [`SignalsOfTextEdit`][crate::classes::text_edit::SignalsOfTextEdit]: signal collection\n\n\nSee also [Godot docs for `TextEdit`](https://docs.godotengine.org/en/stable/classes/class_textedit.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`TextEdit::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextEdit {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TextEdit`][crate::classes::TextEdit].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `TextEdit` methods](https://docs.godotengine.org/en/stable/classes/class_textedit.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextEdit: crate::obj::GodotClass < Base = TextEdit > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ControlNotification) {
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
        fn handle_unicode_input(&mut self, unicode_char: i32, caret_index: i32,) {
            unimplemented !()
        }
        fn backspace(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        fn cut(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        fn copy(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        fn paste(&mut self, caret_index: i32,) {
            unimplemented !()
        }
        fn paste_primary_clipboard(&mut self, caret_index: i32,) {
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
    impl TextEdit {
        pub fn has_ime_text(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8873usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "has_ime_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn cancel_ime(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8874usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "cancel_ime", Some(self.__validated_obj()), args,)
            }
        }
        pub fn apply_ime(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8875usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "apply_ime", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_editable(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8876usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_editable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_editable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8877usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_editable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::TextDirection,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8878usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8879usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8880usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8881usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::classes::text_server::StructuredTextParser,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::StructuredTextParser,);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8882usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::classes::text_server::StructuredTextParser {
            type CallRet = crate::classes::text_server::StructuredTextParser;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8883usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: &VarArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
            let args = (RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8884usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8885usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8886usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_tab_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8887usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_tab_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_indent_wrapped_lines(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8888usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_indent_wrapped_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_indent_wrapped_lines(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8889usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_indent_wrapped_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_input_mode(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8890usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_tab_input_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_input_mode(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8891usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_tab_input_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_overtype_mode_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8892usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_overtype_mode_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_overtype_mode_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8893usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_overtype_mode_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_context_menu_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8894usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_context_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_context_menu_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8895usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_context_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emoji_menu_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8896usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_emoji_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_emoji_menu_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8897usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_emoji_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_backspace_deletes_composite_character_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8898usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_backspace_deletes_composite_character_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_backspace_deletes_composite_character_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8899usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_backspace_deletes_composite_character_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shortcut_keys_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8900usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_shortcut_keys_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_shortcut_keys_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8901usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_shortcut_keys_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_virtual_keyboard_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8902usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_virtual_keyboard_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_virtual_keyboard_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8903usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_virtual_keyboard_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_virtual_keyboard_show_on_focus(&mut self, show_on_focus: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (show_on_focus,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8904usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_virtual_keyboard_show_on_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_virtual_keyboard_show_on_focus(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8905usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_virtual_keyboard_show_on_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_middle_mouse_paste_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8906usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_middle_mouse_paste_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_middle_mouse_paste_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8907usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_middle_mouse_paste_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_empty_selection_clipboard_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8908usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_empty_selection_clipboard_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_empty_selection_clipboard_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8909usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_empty_selection_clipboard_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8910usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8911usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8912usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8913usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_placeholder(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8914usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_placeholder(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8915usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line(&mut self, line: i32, new_text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (line, new_text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8916usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line(&self, line: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8917usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_with_ime(&self, line: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8918usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_with_ime", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_line_width_full(&self, line: i32, wrap_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (line, wrap_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8919usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_line_width_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_line_width(&self, line: i32,) -> i32 {
            self.get_line_width_ex(line,) . done()
        }
        #[inline]
        pub fn get_line_width_ex < 'a > (&'a self, line: i32,) -> ExGetLineWidth < 'a > {
            ExGetLineWidth::new(self, line,)
        }
        pub fn get_line_height(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8920usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_indent_level(&self, line: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8921usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_indent_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_first_non_whitespace_column(&self, line: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8922usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_first_non_whitespace_column", Some(self.__validated_obj()), args,)
            }
        }
        pub fn swap_lines(&mut self, from_line: i32, to_line: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from_line, to_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8923usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "swap_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn insert_line_at(&mut self, line: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (line, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8924usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "insert_line_at", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn remove_line_at_full(&mut self, line: i32, move_carets_down: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (line, move_carets_down,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8925usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "remove_line_at", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::remove_line_at_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn remove_line_at(&mut self, line: i32,) {
            self.remove_line_at_ex(line,) . done()
        }
        #[inline]
        pub fn remove_line_at_ex < 'a > (&'a mut self, line: i32,) -> ExRemoveLineAt < 'a > {
            ExRemoveLineAt::new(self, line,)
        }
        pub(crate) fn insert_text_at_caret_full(&mut self, text: CowArg < GString >, caret_index: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (text, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8926usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "insert_text_at_caret", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::insert_text_at_caret_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn insert_text_at_caret(&mut self, text: impl AsArg < GString >,) {
            self.insert_text_at_caret_ex(text,) . done()
        }
        #[inline]
        pub fn insert_text_at_caret_ex < 'a > (&'a mut self, text: impl AsArg < GString > + 'a,) -> ExInsertTextAtCaret < 'a > {
            ExInsertTextAtCaret::new(self, text,)
        }
        pub(crate) fn insert_text_full(&mut self, text: CowArg < GString >, line: i32, column: i32, before_selection_begin: bool, before_selection_end: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, i32, bool, bool,);
            let args = (text, line, column, before_selection_begin, before_selection_end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8927usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "insert_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::insert_text_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn insert_text(&mut self, text: impl AsArg < GString >, line: i32, column: i32,) {
            self.insert_text_ex(text, line, column,) . done()
        }
        #[inline]
        pub fn insert_text_ex < 'a > (&'a mut self, text: impl AsArg < GString > + 'a, line: i32, column: i32,) -> ExInsertText < 'a > {
            ExInsertText::new(self, text, line, column,)
        }
        pub fn remove_text(&mut self, from_line: i32, from_column: i32, to_line: i32, to_column: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32, i32, i32,);
            let args = (from_line, from_column, to_line, to_column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8928usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "remove_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_last_unhidden_line(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8929usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_last_unhidden_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_next_visible_line_offset_from(&self, line: i32, visible_amount: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (line, visible_amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8930usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_next_visible_line_offset_from", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_next_visible_line_index_offset_from(&self, line: i32, wrap_index: i32, visible_amount: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32, i32, i32,);
            let args = (line, wrap_index, visible_amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8931usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_next_visible_line_index_offset_from", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn backspace_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8932usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "backspace", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::backspace_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn backspace(&mut self,) {
            self.backspace_ex() . done()
        }
        #[inline]
        pub fn backspace_ex < 'a > (&'a mut self,) -> ExBackspace < 'a > {
            ExBackspace::new(self,)
        }
        pub(crate) fn cut_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8933usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "cut", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::cut_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn cut(&mut self,) {
            self.cut_ex() . done()
        }
        #[inline]
        pub fn cut_ex < 'a > (&'a mut self,) -> ExCut < 'a > {
            ExCut::new(self,)
        }
        pub(crate) fn copy_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8934usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "copy", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::copy_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn copy(&mut self,) {
            self.copy_ex() . done()
        }
        #[inline]
        pub fn copy_ex < 'a > (&'a mut self,) -> ExCopy < 'a > {
            ExCopy::new(self,)
        }
        pub(crate) fn paste_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8935usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "paste", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::paste_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn paste(&mut self,) {
            self.paste_ex() . done()
        }
        #[inline]
        pub fn paste_ex < 'a > (&'a mut self,) -> ExPaste < 'a > {
            ExPaste::new(self,)
        }
        pub(crate) fn paste_primary_clipboard_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8936usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "paste_primary_clipboard", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::paste_primary_clipboard_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn paste_primary_clipboard(&mut self,) {
            self.paste_primary_clipboard_ex() . done()
        }
        #[inline]
        pub fn paste_primary_clipboard_ex < 'a > (&'a mut self,) -> ExPastePrimaryClipboard < 'a > {
            ExPastePrimaryClipboard::new(self,)
        }
        pub fn start_action(&mut self, action: crate::classes::text_edit::EditAction,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_edit::EditAction,);
            let args = (action,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8937usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "start_action", Some(self.__validated_obj()), args,)
            }
        }
        pub fn end_action(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8938usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "end_action", Some(self.__validated_obj()), args,)
            }
        }
        pub fn begin_complex_operation(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8939usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "begin_complex_operation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn end_complex_operation(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8940usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "end_complex_operation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_undo(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8941usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "has_undo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_redo(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8942usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "has_redo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn undo(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8943usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "undo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn redo(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8944usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "redo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_undo_history(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8945usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "clear_undo_history", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tag_saved_version(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8946usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "tag_saved_version", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_version(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8947usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_version", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_saved_version(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8948usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_saved_version", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_search_text(&mut self, search_text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (search_text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8949usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_search_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_search_flags(&mut self, flags: crate::classes::text_edit::SearchFlags,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_edit::SearchFlags,);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8950usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_search_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn search(&self, text: impl AsArg < GString >, flags: crate::classes::text_edit::SearchFlags, from_line: i32, from_column: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::text_edit::SearchFlags, i32, i32,);
            let args = (text.into_arg(), flags, from_line, from_column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8951usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "search", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tooltip_request_func(&mut self, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8952usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_tooltip_request_func", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_local_mouse_pos(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8953usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_local_mouse_pos", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_word_at_pos(&self, position: Vector2,) -> GString {
            type CallRet = GString;
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8954usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_word_at_pos", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_line_column_at_pos_full(&self, position: Vector2i, clamp_line: bool, clamp_column: bool,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Vector2i, bool, bool,);
            let args = (position, clamp_line, clamp_column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8955usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_column_at_pos", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_line_column_at_pos_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_line_column_at_pos(&self, position: Vector2i,) -> Vector2i {
            self.get_line_column_at_pos_ex(position,) . done()
        }
        #[inline]
        pub fn get_line_column_at_pos_ex < 'a > (&'a self, position: Vector2i,) -> ExGetLineColumnAtPos < 'a > {
            ExGetLineColumnAtPos::new(self, position,)
        }
        pub fn get_pos_at_line_column(&self, line: i32, column: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8956usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_pos_at_line_column", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rect_at_line_column(&self, line: i32, column: i32,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8957usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_rect_at_line_column", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_minimap_line_at_pos(&self, position: Vector2i,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2i,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8958usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_minimap_line_at_pos", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_dragging_cursor(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8959usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_dragging_cursor", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn is_mouse_over_selection_full(&self, edges: bool, caret_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (bool, i32,);
            let args = (edges, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8960usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_mouse_over_selection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_mouse_over_selection_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_mouse_over_selection(&self, edges: bool,) -> bool {
            self.is_mouse_over_selection_ex(edges,) . done()
        }
        #[inline]
        pub fn is_mouse_over_selection_ex < 'a > (&'a self, edges: bool,) -> ExIsMouseOverSelection < 'a > {
            ExIsMouseOverSelection::new(self, edges,)
        }
        pub fn set_caret_type(&mut self, type_: crate::classes::text_edit::CaretType,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_edit::CaretType,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8961usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_caret_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_caret_type(&self,) -> crate::classes::text_edit::CaretType {
            type CallRet = crate::classes::text_edit::CaretType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8962usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_caret_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_caret_blink_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8963usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_caret_blink_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_caret_blink_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8964usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_caret_blink_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_caret_blink_interval(&mut self, interval: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (interval,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8965usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_caret_blink_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_caret_blink_interval(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8966usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_caret_blink_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_caret_when_editable_disabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8967usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_draw_caret_when_editable_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drawing_caret_when_editable_disabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8968usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_drawing_caret_when_editable_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_move_caret_on_right_click_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8969usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_move_caret_on_right_click_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_move_caret_on_right_click_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8970usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_move_caret_on_right_click_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_caret_mid_grapheme_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8971usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_caret_mid_grapheme_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_caret_mid_grapheme_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8972usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_caret_mid_grapheme_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_multiple_carets_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8973usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_multiple_carets_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_multiple_carets_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8974usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_multiple_carets_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_caret(&mut self, line: i32, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8975usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "add_caret", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_caret(&mut self, caret: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8976usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "remove_caret", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_secondary_carets(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8977usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "remove_secondary_carets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_caret_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8978usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_caret_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_caret_at_carets(&mut self, below: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (below,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8979usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "add_caret_at_carets", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_sorted_carets_full(&self, include_ignored_carets: bool,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (bool,);
            let args = (include_ignored_carets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8980usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_sorted_carets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_sorted_carets_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_sorted_carets(&self,) -> PackedInt32Array {
            self.get_sorted_carets_ex() . done()
        }
        #[inline]
        pub fn get_sorted_carets_ex < 'a > (&'a self,) -> ExGetSortedCarets < 'a > {
            ExGetSortedCarets::new(self,)
        }
        pub(crate) fn collapse_carets_full(&mut self, from_line: i32, from_column: i32, to_line: i32, to_column: i32, inclusive: bool,) {
            type CallRet = ();
            type CallParams = (i32, i32, i32, i32, bool,);
            let args = (from_line, from_column, to_line, to_column, inclusive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8981usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "collapse_carets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::collapse_carets_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn collapse_carets(&mut self, from_line: i32, from_column: i32, to_line: i32, to_column: i32,) {
            self.collapse_carets_ex(from_line, from_column, to_line, to_column,) . done()
        }
        #[inline]
        pub fn collapse_carets_ex < 'a > (&'a mut self, from_line: i32, from_column: i32, to_line: i32, to_column: i32,) -> ExCollapseCarets < 'a > {
            ExCollapseCarets::new(self, from_line, from_column, to_line, to_column,)
        }
        pub fn merge_overlapping_carets(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8982usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "merge_overlapping_carets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn begin_multicaret_edit(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8983usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "begin_multicaret_edit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn end_multicaret_edit(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8984usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "end_multicaret_edit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_in_mulitcaret_edit(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8985usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_in_mulitcaret_edit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multicaret_edit_ignore_caret(&self, caret_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8986usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "multicaret_edit_ignore_caret", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn is_caret_visible_full(&self, caret_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8987usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_caret_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_caret_visible_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_caret_visible(&self,) -> bool {
            self.is_caret_visible_ex() . done()
        }
        #[inline]
        pub fn is_caret_visible_ex < 'a > (&'a self,) -> ExIsCaretVisible < 'a > {
            ExIsCaretVisible::new(self,)
        }
        pub(crate) fn get_caret_draw_pos_full(&self, caret_index: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8988usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_caret_draw_pos", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_caret_draw_pos_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_caret_draw_pos(&self,) -> Vector2 {
            self.get_caret_draw_pos_ex() . done()
        }
        #[inline]
        pub fn get_caret_draw_pos_ex < 'a > (&'a self,) -> ExGetCaretDrawPos < 'a > {
            ExGetCaretDrawPos::new(self,)
        }
        pub(crate) fn set_caret_line_full(&mut self, line: i32, adjust_viewport: bool, can_be_hidden: bool, wrap_index: i32, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, bool, bool, i32, i32,);
            let args = (line, adjust_viewport, can_be_hidden, wrap_index, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8989usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_caret_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_caret_line_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_caret_line(&mut self, line: i32,) {
            self.set_caret_line_ex(line,) . done()
        }
        #[inline]
        pub fn set_caret_line_ex < 'a > (&'a mut self, line: i32,) -> ExSetCaretLine < 'a > {
            ExSetCaretLine::new(self, line,)
        }
        pub(crate) fn get_caret_line_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8990usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_caret_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_caret_line_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_caret_line(&self,) -> i32 {
            self.get_caret_line_ex() . done()
        }
        #[inline]
        pub fn get_caret_line_ex < 'a > (&'a self,) -> ExGetCaretLine < 'a > {
            ExGetCaretLine::new(self,)
        }
        pub(crate) fn set_caret_column_full(&mut self, column: i32, adjust_viewport: bool, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, bool, i32,);
            let args = (column, adjust_viewport, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8991usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_caret_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_caret_column_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_caret_column(&mut self, column: i32,) {
            self.set_caret_column_ex(column,) . done()
        }
        #[inline]
        pub fn set_caret_column_ex < 'a > (&'a mut self, column: i32,) -> ExSetCaretColumn < 'a > {
            ExSetCaretColumn::new(self, column,)
        }
        pub(crate) fn get_caret_column_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8992usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_caret_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_caret_column_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_caret_column(&self,) -> i32 {
            self.get_caret_column_ex() . done()
        }
        #[inline]
        pub fn get_caret_column_ex < 'a > (&'a self,) -> ExGetCaretColumn < 'a > {
            ExGetCaretColumn::new(self,)
        }
        pub fn get_next_composite_character_column(&self, line: i32, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8993usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_next_composite_character_column", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_previous_composite_character_column(&self, line: i32, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8994usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_previous_composite_character_column", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_caret_wrap_index_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8995usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_caret_wrap_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_caret_wrap_index_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_caret_wrap_index(&self,) -> i32 {
            self.get_caret_wrap_index_ex() . done()
        }
        #[inline]
        pub fn get_caret_wrap_index_ex < 'a > (&'a self,) -> ExGetCaretWrapIndex < 'a > {
            ExGetCaretWrapIndex::new(self,)
        }
        pub(crate) fn get_word_under_caret_full(&self, caret_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8996usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_word_under_caret", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_word_under_caret_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_word_under_caret(&self,) -> GString {
            self.get_word_under_caret_ex() . done()
        }
        #[inline]
        pub fn get_word_under_caret_ex < 'a > (&'a self,) -> ExGetWordUnderCaret < 'a > {
            ExGetWordUnderCaret::new(self,)
        }
        pub fn set_use_default_word_separators(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8997usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_use_default_word_separators", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_default_word_separators_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8998usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_default_word_separators_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_custom_word_separators(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8999usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_use_custom_word_separators", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_custom_word_separators_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9000usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_custom_word_separators_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_word_separators(&mut self, custom_word_separators: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (custom_word_separators.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9001usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_custom_word_separators", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_word_separators(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9002usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_custom_word_separators", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_selecting_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9003usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_selecting_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_selecting_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9004usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_selecting_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_deselect_on_focus_loss_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9005usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_deselect_on_focus_loss_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_deselect_on_focus_loss_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9006usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_deselect_on_focus_loss_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_and_drop_selection_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9007usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_drag_and_drop_selection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drag_and_drop_selection_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9008usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_drag_and_drop_selection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_selection_mode(&mut self, mode: crate::classes::text_edit::SelectionMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_edit::SelectionMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9009usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_selection_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selection_mode(&self,) -> crate::classes::text_edit::SelectionMode {
            type CallRet = crate::classes::text_edit::SelectionMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9010usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn select_all(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9011usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "select_all", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn select_word_under_caret_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9012usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "select_word_under_caret", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::select_word_under_caret_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn select_word_under_caret(&mut self,) {
            self.select_word_under_caret_ex() . done()
        }
        #[inline]
        pub fn select_word_under_caret_ex < 'a > (&'a mut self,) -> ExSelectWordUnderCaret < 'a > {
            ExSelectWordUnderCaret::new(self,)
        }
        pub fn add_selection_for_next_occurrence(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9013usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "add_selection_for_next_occurrence", Some(self.__validated_obj()), args,)
            }
        }
        pub fn skip_selection_for_next_occurrence(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9014usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "skip_selection_for_next_occurrence", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn select_full(&mut self, origin_line: i32, origin_column: i32, caret_line: i32, caret_column: i32, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32, i32, i32, i32,);
            let args = (origin_line, origin_column, caret_line, caret_column, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9015usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "select", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::select_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn select(&mut self, origin_line: i32, origin_column: i32, caret_line: i32, caret_column: i32,) {
            self.select_ex(origin_line, origin_column, caret_line, caret_column,) . done()
        }
        #[inline]
        pub fn select_ex < 'a > (&'a mut self, origin_line: i32, origin_column: i32, caret_line: i32, caret_column: i32,) -> ExSelect < 'a > {
            ExSelect::new(self, origin_line, origin_column, caret_line, caret_column,)
        }
        pub(crate) fn has_selection_full(&self, caret_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9016usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "has_selection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::has_selection_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn has_selection(&self,) -> bool {
            self.has_selection_ex() . done()
        }
        #[inline]
        pub fn has_selection_ex < 'a > (&'a self,) -> ExHasSelection < 'a > {
            ExHasSelection::new(self,)
        }
        pub(crate) fn get_selected_text_full(&mut self, caret_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9017usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selected_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_selected_text_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_selected_text(&mut self,) -> GString {
            self.get_selected_text_ex() . done()
        }
        #[inline]
        pub fn get_selected_text_ex < 'a > (&'a mut self,) -> ExGetSelectedText < 'a > {
            ExGetSelectedText::new(self,)
        }
        pub(crate) fn get_selection_at_line_column_full(&self, line: i32, column: i32, include_edges: bool, only_selections: bool,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32, bool, bool,);
            let args = (line, column, include_edges, only_selections,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9018usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_at_line_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_selection_at_line_column_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_selection_at_line_column(&self, line: i32, column: i32,) -> i32 {
            self.get_selection_at_line_column_ex(line, column,) . done()
        }
        #[inline]
        pub fn get_selection_at_line_column_ex < 'a > (&'a self, line: i32, column: i32,) -> ExGetSelectionAtLineColumn < 'a > {
            ExGetSelectionAtLineColumn::new(self, line, column,)
        }
        pub(crate) fn get_line_ranges_from_carets_full(&self, only_selections: bool, merge_adjacent: bool,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (bool, bool,);
            let args = (only_selections, merge_adjacent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9019usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_ranges_from_carets", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_line_ranges_from_carets_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_line_ranges_from_carets(&self,) -> Array < Vector2i > {
            self.get_line_ranges_from_carets_ex() . done()
        }
        #[inline]
        pub fn get_line_ranges_from_carets_ex < 'a > (&'a self,) -> ExGetLineRangesFromCarets < 'a > {
            ExGetLineRangesFromCarets::new(self,)
        }
        pub(crate) fn get_selection_origin_line_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9020usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_origin_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_selection_origin_line_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_selection_origin_line(&self,) -> i32 {
            self.get_selection_origin_line_ex() . done()
        }
        #[inline]
        pub fn get_selection_origin_line_ex < 'a > (&'a self,) -> ExGetSelectionOriginLine < 'a > {
            ExGetSelectionOriginLine::new(self,)
        }
        pub(crate) fn get_selection_origin_column_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9021usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_origin_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_selection_origin_column_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_selection_origin_column(&self,) -> i32 {
            self.get_selection_origin_column_ex() . done()
        }
        #[inline]
        pub fn get_selection_origin_column_ex < 'a > (&'a self,) -> ExGetSelectionOriginColumn < 'a > {
            ExGetSelectionOriginColumn::new(self,)
        }
        pub(crate) fn set_selection_origin_line_full(&mut self, line: i32, can_be_hidden: bool, wrap_index: i32, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, bool, i32, i32,);
            let args = (line, can_be_hidden, wrap_index, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9022usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_selection_origin_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_selection_origin_line_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_selection_origin_line(&mut self, line: i32,) {
            self.set_selection_origin_line_ex(line,) . done()
        }
        #[inline]
        pub fn set_selection_origin_line_ex < 'a > (&'a mut self, line: i32,) -> ExSetSelectionOriginLine < 'a > {
            ExSetSelectionOriginLine::new(self, line,)
        }
        pub(crate) fn set_selection_origin_column_full(&mut self, column: i32, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (column, caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9023usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_selection_origin_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_selection_origin_column_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_selection_origin_column(&mut self, column: i32,) {
            self.set_selection_origin_column_ex(column,) . done()
        }
        #[inline]
        pub fn set_selection_origin_column_ex < 'a > (&'a mut self, column: i32,) -> ExSetSelectionOriginColumn < 'a > {
            ExSetSelectionOriginColumn::new(self, column,)
        }
        pub(crate) fn get_selection_from_line_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9024usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_from_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_selection_from_line_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_selection_from_line(&self,) -> i32 {
            self.get_selection_from_line_ex() . done()
        }
        #[inline]
        pub fn get_selection_from_line_ex < 'a > (&'a self,) -> ExGetSelectionFromLine < 'a > {
            ExGetSelectionFromLine::new(self,)
        }
        pub(crate) fn get_selection_from_column_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9025usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_from_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_selection_from_column_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_selection_from_column(&self,) -> i32 {
            self.get_selection_from_column_ex() . done()
        }
        #[inline]
        pub fn get_selection_from_column_ex < 'a > (&'a self,) -> ExGetSelectionFromColumn < 'a > {
            ExGetSelectionFromColumn::new(self,)
        }
        pub(crate) fn get_selection_to_line_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9026usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_to_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_selection_to_line_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_selection_to_line(&self,) -> i32 {
            self.get_selection_to_line_ex() . done()
        }
        #[inline]
        pub fn get_selection_to_line_ex < 'a > (&'a self,) -> ExGetSelectionToLine < 'a > {
            ExGetSelectionToLine::new(self,)
        }
        pub(crate) fn get_selection_to_column_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9027usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_to_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_selection_to_column_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_selection_to_column(&self,) -> i32 {
            self.get_selection_to_column_ex() . done()
        }
        #[inline]
        pub fn get_selection_to_column_ex < 'a > (&'a self,) -> ExGetSelectionToColumn < 'a > {
            ExGetSelectionToColumn::new(self,)
        }
        pub(crate) fn is_caret_after_selection_origin_full(&self, caret_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9028usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_caret_after_selection_origin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_caret_after_selection_origin_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_caret_after_selection_origin(&self,) -> bool {
            self.is_caret_after_selection_origin_ex() . done()
        }
        #[inline]
        pub fn is_caret_after_selection_origin_ex < 'a > (&'a self,) -> ExIsCaretAfterSelectionOrigin < 'a > {
            ExIsCaretAfterSelectionOrigin::new(self,)
        }
        pub(crate) fn deselect_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9029usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "deselect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::deselect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn deselect(&mut self,) {
            self.deselect_ex() . done()
        }
        #[inline]
        pub fn deselect_ex < 'a > (&'a mut self,) -> ExDeselect < 'a > {
            ExDeselect::new(self,)
        }
        pub(crate) fn delete_selection_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9030usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "delete_selection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::delete_selection_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn delete_selection(&mut self,) {
            self.delete_selection_ex() . done()
        }
        #[inline]
        pub fn delete_selection_ex < 'a > (&'a mut self,) -> ExDeleteSelection < 'a > {
            ExDeleteSelection::new(self,)
        }
        pub fn set_line_wrapping_mode(&mut self, mode: crate::classes::text_edit::LineWrappingMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_edit::LineWrappingMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9031usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_wrapping_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_wrapping_mode(&self,) -> crate::classes::text_edit::LineWrappingMode {
            type CallRet = crate::classes::text_edit::LineWrappingMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9032usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_wrapping_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, autowrap_mode: crate::classes::text_server::AutowrapMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::AutowrapMode,);
            let args = (autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9033usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autowrap_mode(&self,) -> crate::classes::text_server::AutowrapMode {
            type CallRet = crate::classes::text_server::AutowrapMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9034usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_line_wrapped(&self, line: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9035usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_line_wrapped", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_wrap_count(&self, line: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9036usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_wrap_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_wrap_index_at_column(&self, line: i32, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9037usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_wrap_index_at_column", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_wrapped_text(&self, line: i32,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9038usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_wrapped_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_smooth_scroll_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9039usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_smooth_scroll_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_smooth_scroll_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9040usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_smooth_scroll_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_v_scroll_bar(&self,) -> Option < Gd < crate::classes::VScrollBar > > {
            type CallRet = Option < Gd < crate::classes::VScrollBar > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9041usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_v_scroll_bar", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_h_scroll_bar(&self,) -> Option < Gd < crate::classes::HScrollBar > > {
            type CallRet = Option < Gd < crate::classes::HScrollBar > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9042usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_h_scroll_bar", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_v_scroll(&mut self, value: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9043usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_v_scroll", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_v_scroll(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9044usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_v_scroll", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_h_scroll(&mut self, value: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9045usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_h_scroll", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_h_scroll(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9046usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_h_scroll", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scroll_past_end_of_file_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9047usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_scroll_past_end_of_file_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_scroll_past_end_of_file_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9048usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_scroll_past_end_of_file_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_v_scroll_speed(&mut self, speed: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9049usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_v_scroll_speed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_v_scroll_speed(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9050usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_v_scroll_speed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fit_content_height_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9051usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_fit_content_height_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_fit_content_height_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9052usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_fit_content_height_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fit_content_width_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9053usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_fit_content_width_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_fit_content_width_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9054usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_fit_content_width_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_scroll_pos_for_line_full(&self, line: i32, wrap_index: i32,) -> f64 {
            type CallRet = f64;
            type CallParams = (i32, i32,);
            let args = (line, wrap_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9055usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_scroll_pos_for_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_scroll_pos_for_line_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_scroll_pos_for_line(&self, line: i32,) -> f64 {
            self.get_scroll_pos_for_line_ex(line,) . done()
        }
        #[inline]
        pub fn get_scroll_pos_for_line_ex < 'a > (&'a self, line: i32,) -> ExGetScrollPosForLine < 'a > {
            ExGetScrollPosForLine::new(self, line,)
        }
        pub(crate) fn set_line_as_first_visible_full(&mut self, line: i32, wrap_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (line, wrap_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9056usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_as_first_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_line_as_first_visible_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_line_as_first_visible(&mut self, line: i32,) {
            self.set_line_as_first_visible_ex(line,) . done()
        }
        #[inline]
        pub fn set_line_as_first_visible_ex < 'a > (&'a mut self, line: i32,) -> ExSetLineAsFirstVisible < 'a > {
            ExSetLineAsFirstVisible::new(self, line,)
        }
        pub fn get_first_visible_line(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9057usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_first_visible_line", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn set_line_as_center_visible_full(&mut self, line: i32, wrap_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (line, wrap_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9058usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_as_center_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_line_as_center_visible_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_line_as_center_visible(&mut self, line: i32,) {
            self.set_line_as_center_visible_ex(line,) . done()
        }
        #[inline]
        pub fn set_line_as_center_visible_ex < 'a > (&'a mut self, line: i32,) -> ExSetLineAsCenterVisible < 'a > {
            ExSetLineAsCenterVisible::new(self, line,)
        }
        pub(crate) fn set_line_as_last_visible_full(&mut self, line: i32, wrap_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (line, wrap_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9059usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_as_last_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_line_as_last_visible_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_line_as_last_visible(&mut self, line: i32,) {
            self.set_line_as_last_visible_ex(line,) . done()
        }
        #[inline]
        pub fn set_line_as_last_visible_ex < 'a > (&'a mut self, line: i32,) -> ExSetLineAsLastVisible < 'a > {
            ExSetLineAsLastVisible::new(self, line,)
        }
        pub fn get_last_full_visible_line(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9060usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_last_full_visible_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_last_full_visible_line_wrap_index(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9061usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_last_full_visible_line_wrap_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_line_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9062usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_visible_line_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_line_count_in_range(&self, from_line: i32, to_line: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (from_line, to_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9063usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_visible_line_count_in_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_total_visible_line_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9064usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_total_visible_line_count", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn adjust_viewport_to_caret_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9065usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "adjust_viewport_to_caret", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::adjust_viewport_to_caret_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn adjust_viewport_to_caret(&mut self,) {
            self.adjust_viewport_to_caret_ex() . done()
        }
        #[inline]
        pub fn adjust_viewport_to_caret_ex < 'a > (&'a mut self,) -> ExAdjustViewportToCaret < 'a > {
            ExAdjustViewportToCaret::new(self,)
        }
        pub(crate) fn center_viewport_to_caret_full(&mut self, caret_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9066usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "center_viewport_to_caret", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::center_viewport_to_caret_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn center_viewport_to_caret(&mut self,) {
            self.center_viewport_to_caret_ex() . done()
        }
        #[inline]
        pub fn center_viewport_to_caret_ex < 'a > (&'a mut self,) -> ExCenterViewportToCaret < 'a > {
            ExCenterViewportToCaret::new(self,)
        }
        pub fn set_draw_minimap(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9067usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_draw_minimap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drawing_minimap(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9068usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_drawing_minimap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_minimap_width(&mut self, width: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9069usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_minimap_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_minimap_width(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9070usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_minimap_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_minimap_visible_lines(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9071usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_minimap_visible_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_gutter_full(&mut self, at: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (at,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9072usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "add_gutter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_gutter_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_gutter(&mut self,) {
            self.add_gutter_ex() . done()
        }
        #[inline]
        pub fn add_gutter_ex < 'a > (&'a mut self,) -> ExAddGutter < 'a > {
            ExAddGutter::new(self,)
        }
        pub fn remove_gutter(&mut self, gutter: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9073usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "remove_gutter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gutter_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9074usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_gutter_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gutter_name(&mut self, gutter: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (gutter, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9075usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_gutter_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gutter_name(&self, gutter: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9076usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_gutter_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gutter_type(&mut self, gutter: i32, type_: crate::classes::text_edit::GutterType,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::text_edit::GutterType,);
            let args = (gutter, type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9077usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_gutter_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gutter_type(&self, gutter: i32,) -> crate::classes::text_edit::GutterType {
            type CallRet = crate::classes::text_edit::GutterType;
            type CallParams = (i32,);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9078usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_gutter_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gutter_width(&mut self, gutter: i32, width: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (gutter, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9079usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_gutter_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gutter_width(&self, gutter: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9080usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_gutter_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gutter_draw(&mut self, gutter: i32, draw: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (gutter, draw,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9081usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_gutter_draw", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_gutter_drawn(&self, gutter: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9082usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_gutter_drawn", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gutter_clickable(&mut self, gutter: i32, clickable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (gutter, clickable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9083usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_gutter_clickable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_gutter_clickable(&self, gutter: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9084usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_gutter_clickable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gutter_overwritable(&mut self, gutter: i32, overwritable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (gutter, overwritable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9085usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_gutter_overwritable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_gutter_overwritable(&self, gutter: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9086usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_gutter_overwritable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn merge_gutters(&mut self, from_line: i32, to_line: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from_line, to_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9087usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "merge_gutters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gutter_custom_draw(&mut self, column: i32, draw_callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Callable >,);
            let args = (column, RefArg::new(draw_callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9088usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_gutter_custom_draw", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_total_gutter_width(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9089usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_total_gutter_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_gutter_metadata(&mut self, line: i32, gutter: i32, metadata: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, RefArg < 'a0, Variant >,);
            let args = (line, gutter, RefArg::new(metadata),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9090usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_gutter_metadata", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_gutter_metadata(&self, line: i32, gutter: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32, i32,);
            let args = (line, gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9091usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_gutter_metadata", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_gutter_text(&mut self, line: i32, gutter: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, GString >,);
            let args = (line, gutter, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9092usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_gutter_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_gutter_text(&self, line: i32, gutter: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32, i32,);
            let args = (line, gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9093usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_gutter_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_gutter_icon(&mut self, line: i32, gutter: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (line, gutter, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9094usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_gutter_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_gutter_icon(&self, line: i32, gutter: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32, i32,);
            let args = (line, gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9095usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_gutter_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_gutter_item_color(&mut self, line: i32, gutter: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, i32, Color,);
            let args = (line, gutter, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9096usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_gutter_item_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_gutter_item_color(&self, line: i32, gutter: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32, i32,);
            let args = (line, gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9097usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_gutter_item_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_gutter_clickable(&mut self, line: i32, gutter: i32, clickable: bool,) {
            type CallRet = ();
            type CallParams = (i32, i32, bool,);
            let args = (line, gutter, clickable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9098usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_gutter_clickable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_line_gutter_clickable(&self, line: i32, gutter: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, i32,);
            let args = (line, gutter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9099usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_line_gutter_clickable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_background_color(&mut self, line: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (line, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_line_background_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_background_color(&self, line: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9101usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_line_background_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_syntax_highlighter(&mut self, syntax_highlighter: impl AsArg < Option < Gd < crate::classes::SyntaxHighlighter >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::SyntaxHighlighter > > >,);
            let args = (syntax_highlighter.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9102usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_syntax_highlighter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_syntax_highlighter(&self,) -> Option < Gd < crate::classes::SyntaxHighlighter > > {
            type CallRet = Option < Gd < crate::classes::SyntaxHighlighter > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_syntax_highlighter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_highlight_current_line(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_highlight_current_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_highlight_current_line_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_highlight_current_line_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_highlight_all_occurrences(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_highlight_all_occurrences", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_highlight_all_occurrences_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_highlight_all_occurrences_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_draw_control_chars(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_draw_control_chars", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_control_chars(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_draw_control_chars", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_tabs(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_draw_tabs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drawing_tabs(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_drawing_tabs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_spaces(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "set_draw_spaces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drawing_spaces(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_drawing_spaces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_menu(&self,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallRet = Option < Gd < crate::classes::PopupMenu > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_menu_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "is_menu_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn menu_option(&mut self, option: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "menu_option", Some(self.__validated_obj()), args,)
            }
        }
        pub fn adjust_carets_after_edit(&mut self, caret: i32, from_line: i32, from_col: i32, to_line: i32, to_col: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32, i32, i32, i32,);
            let args = (caret, from_line, from_col, to_line, to_col,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "adjust_carets_after_edit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_caret_index_edit_order(&mut self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_caret_index_edit_order", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_selection_line_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_selection_line_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_selection_line(&self,) -> i32 {
            self.get_selection_line_ex() . done()
        }
        #[inline]
        pub fn get_selection_line_ex < 'a > (&'a self,) -> ExGetSelectionLine < 'a > {
            ExGetSelectionLine::new(self,)
        }
        pub(crate) fn get_selection_column_full(&self, caret_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (caret_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextEdit", "get_selection_column", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_selection_column_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_selection_column(&self,) -> i32 {
            self.get_selection_column_ex() . done()
        }
        #[inline]
        pub fn get_selection_column_ex < 'a > (&'a self,) -> ExGetSelectionColumn < 'a > {
            ExGetSelectionColumn::new(self,)
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
    impl crate::obj::GodotClass for TextEdit {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TextEdit"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextEdit {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for TextEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for TextEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for TextEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextEdit {
        
    }
    impl crate::obj::cap::GodotDefault for TextEdit {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TextEdit {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextEdit {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TextEdit`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TextEdit__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TextEdit > for $Class {
                
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
#[doc = "Default-param extender for [`TextEdit::get_line_width_ex`][super::TextEdit::get_line_width_ex]."]
#[must_use]
pub struct ExGetLineWidth < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, line: i32, wrap_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetLineWidth < 'a > {
    fn new(surround_object: &'a re_export::TextEdit, line: i32,) -> Self {
        let wrap_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, wrap_index: wrap_index,
        }
    }
    #[inline]
    pub fn wrap_index(self, wrap_index: i32) -> Self {
        Self {
            wrap_index: wrap_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, line, wrap_index,
        }
        = self;
        re_export::TextEdit::get_line_width_full(surround_object, line, wrap_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::remove_line_at_ex`][super::TextEdit::remove_line_at_ex]."]
#[must_use]
pub struct ExRemoveLineAt < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, line: i32, move_carets_down: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRemoveLineAt < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, line: i32,) -> Self {
        let move_carets_down = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, move_carets_down: move_carets_down,
        }
    }
    #[inline]
    pub fn move_carets_down(self, move_carets_down: bool) -> Self {
        Self {
            move_carets_down: move_carets_down, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, line, move_carets_down,
        }
        = self;
        re_export::TextEdit::remove_line_at_full(surround_object, line, move_carets_down,)
    }
}
#[doc = "Default-param extender for [`TextEdit::insert_text_at_caret_ex`][super::TextEdit::insert_text_at_caret_ex]."]
#[must_use]
pub struct ExInsertTextAtCaret < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, text: CowArg < 'a, GString >, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExInsertTextAtCaret < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, text: impl AsArg < GString > + 'a,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, text, caret_index,
        }
        = self;
        re_export::TextEdit::insert_text_at_caret_full(surround_object, text, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::insert_text_ex`][super::TextEdit::insert_text_ex]."]
#[must_use]
pub struct ExInsertText < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, text: CowArg < 'a, GString >, line: i32, column: i32, before_selection_begin: bool, before_selection_end: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExInsertText < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, text: impl AsArg < GString > + 'a, line: i32, column: i32,) -> Self {
        let before_selection_begin = true;
        let before_selection_end = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), line: line, column: column, before_selection_begin: before_selection_begin, before_selection_end: before_selection_end,
        }
    }
    #[inline]
    pub fn before_selection_begin(self, before_selection_begin: bool) -> Self {
        Self {
            before_selection_begin: before_selection_begin, .. self
        }
    }
    #[inline]
    pub fn before_selection_end(self, before_selection_end: bool) -> Self {
        Self {
            before_selection_end: before_selection_end, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, text, line, column, before_selection_begin, before_selection_end,
        }
        = self;
        re_export::TextEdit::insert_text_full(surround_object, text, line, column, before_selection_begin, before_selection_end,)
    }
}
#[doc = "Default-param extender for [`TextEdit::backspace_ex`][super::TextEdit::backspace_ex]."]
#[must_use]
pub struct ExBackspace < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBackspace < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::backspace_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::cut_ex`][super::TextEdit::cut_ex]."]
#[must_use]
pub struct ExCut < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCut < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::cut_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::copy_ex`][super::TextEdit::copy_ex]."]
#[must_use]
pub struct ExCopy < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCopy < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::copy_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::paste_ex`][super::TextEdit::paste_ex]."]
#[must_use]
pub struct ExPaste < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPaste < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::paste_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::paste_primary_clipboard_ex`][super::TextEdit::paste_primary_clipboard_ex]."]
#[must_use]
pub struct ExPastePrimaryClipboard < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPastePrimaryClipboard < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::paste_primary_clipboard_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_line_column_at_pos_ex`][super::TextEdit::get_line_column_at_pos_ex]."]
#[must_use]
pub struct ExGetLineColumnAtPos < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, position: Vector2i, clamp_line: bool, clamp_column: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetLineColumnAtPos < 'a > {
    fn new(surround_object: &'a re_export::TextEdit, position: Vector2i,) -> Self {
        let clamp_line = true;
        let clamp_column = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, clamp_line: clamp_line, clamp_column: clamp_column,
        }
    }
    #[inline]
    pub fn clamp_line(self, clamp_line: bool) -> Self {
        Self {
            clamp_line: clamp_line, .. self
        }
    }
    #[inline]
    pub fn clamp_column(self, clamp_column: bool) -> Self {
        Self {
            clamp_column: clamp_column, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, position, clamp_line, clamp_column,
        }
        = self;
        re_export::TextEdit::get_line_column_at_pos_full(surround_object, position, clamp_line, clamp_column,)
    }
}
#[doc = "Default-param extender for [`TextEdit::is_mouse_over_selection_ex`][super::TextEdit::is_mouse_over_selection_ex]."]
#[must_use]
pub struct ExIsMouseOverSelection < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, edges: bool, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsMouseOverSelection < 'a > {
    fn new(surround_object: &'a re_export::TextEdit, edges: bool,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, edges: edges, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, edges, caret_index,
        }
        = self;
        re_export::TextEdit::is_mouse_over_selection_full(surround_object, edges, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_sorted_carets_ex`][super::TextEdit::get_sorted_carets_ex]."]
#[must_use]
pub struct ExGetSortedCarets < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, include_ignored_carets: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSortedCarets < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let include_ignored_carets = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, include_ignored_carets: include_ignored_carets,
        }
    }
    #[inline]
    pub fn include_ignored_carets(self, include_ignored_carets: bool) -> Self {
        Self {
            include_ignored_carets: include_ignored_carets, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, include_ignored_carets,
        }
        = self;
        re_export::TextEdit::get_sorted_carets_full(surround_object, include_ignored_carets,)
    }
}
#[doc = "Default-param extender for [`TextEdit::collapse_carets_ex`][super::TextEdit::collapse_carets_ex]."]
#[must_use]
pub struct ExCollapseCarets < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, from_line: i32, from_column: i32, to_line: i32, to_column: i32, inclusive: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCollapseCarets < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, from_line: i32, from_column: i32, to_line: i32, to_column: i32,) -> Self {
        let inclusive = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_line: from_line, from_column: from_column, to_line: to_line, to_column: to_column, inclusive: inclusive,
        }
    }
    #[inline]
    pub fn inclusive(self, inclusive: bool) -> Self {
        Self {
            inclusive: inclusive, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from_line, from_column, to_line, to_column, inclusive,
        }
        = self;
        re_export::TextEdit::collapse_carets_full(surround_object, from_line, from_column, to_line, to_column, inclusive,)
    }
}
#[doc = "Default-param extender for [`TextEdit::is_caret_visible_ex`][super::TextEdit::is_caret_visible_ex]."]
#[must_use]
pub struct ExIsCaretVisible < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsCaretVisible < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::is_caret_visible_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_caret_draw_pos_ex`][super::TextEdit::get_caret_draw_pos_ex]."]
#[must_use]
pub struct ExGetCaretDrawPos < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCaretDrawPos < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_caret_draw_pos_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_caret_line_ex`][super::TextEdit::set_caret_line_ex]."]
#[must_use]
pub struct ExSetCaretLine < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, line: i32, adjust_viewport: bool, can_be_hidden: bool, wrap_index: i32, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCaretLine < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, line: i32,) -> Self {
        let adjust_viewport = true;
        let can_be_hidden = true;
        let wrap_index = 0i32;
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, adjust_viewport: adjust_viewport, can_be_hidden: can_be_hidden, wrap_index: wrap_index, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn adjust_viewport(self, adjust_viewport: bool) -> Self {
        Self {
            adjust_viewport: adjust_viewport, .. self
        }
    }
    #[inline]
    pub fn can_be_hidden(self, can_be_hidden: bool) -> Self {
        Self {
            can_be_hidden: can_be_hidden, .. self
        }
    }
    #[inline]
    pub fn wrap_index(self, wrap_index: i32) -> Self {
        Self {
            wrap_index: wrap_index, .. self
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, line, adjust_viewport, can_be_hidden, wrap_index, caret_index,
        }
        = self;
        re_export::TextEdit::set_caret_line_full(surround_object, line, adjust_viewport, can_be_hidden, wrap_index, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_caret_line_ex`][super::TextEdit::get_caret_line_ex]."]
#[must_use]
pub struct ExGetCaretLine < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCaretLine < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_caret_line_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_caret_column_ex`][super::TextEdit::set_caret_column_ex]."]
#[must_use]
pub struct ExSetCaretColumn < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, column: i32, adjust_viewport: bool, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCaretColumn < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, column: i32,) -> Self {
        let adjust_viewport = true;
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, adjust_viewport: adjust_viewport, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn adjust_viewport(self, adjust_viewport: bool) -> Self {
        Self {
            adjust_viewport: adjust_viewport, .. self
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, adjust_viewport, caret_index,
        }
        = self;
        re_export::TextEdit::set_caret_column_full(surround_object, column, adjust_viewport, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_caret_column_ex`][super::TextEdit::get_caret_column_ex]."]
#[must_use]
pub struct ExGetCaretColumn < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCaretColumn < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_caret_column_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_caret_wrap_index_ex`][super::TextEdit::get_caret_wrap_index_ex]."]
#[must_use]
pub struct ExGetCaretWrapIndex < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCaretWrapIndex < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_caret_wrap_index_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_word_under_caret_ex`][super::TextEdit::get_word_under_caret_ex]."]
#[must_use]
pub struct ExGetWordUnderCaret < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetWordUnderCaret < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_word_under_caret_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::select_word_under_caret_ex`][super::TextEdit::select_word_under_caret_ex]."]
#[must_use]
pub struct ExSelectWordUnderCaret < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSelectWordUnderCaret < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::select_word_under_caret_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::select_ex`][super::TextEdit::select_ex]."]
#[must_use]
pub struct ExSelect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, origin_line: i32, origin_column: i32, caret_line: i32, caret_column: i32, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSelect < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, origin_line: i32, origin_column: i32, caret_line: i32, caret_column: i32,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, origin_line: origin_line, origin_column: origin_column, caret_line: caret_line, caret_column: caret_column, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, origin_line, origin_column, caret_line, caret_column, caret_index,
        }
        = self;
        re_export::TextEdit::select_full(surround_object, origin_line, origin_column, caret_line, caret_column, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::has_selection_ex`][super::TextEdit::has_selection_ex]."]
#[must_use]
pub struct ExHasSelection < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExHasSelection < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::has_selection_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selected_text_ex`][super::TextEdit::get_selected_text_ex]."]
#[must_use]
pub struct ExGetSelectedText < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectedText < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selected_text_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_at_line_column_ex`][super::TextEdit::get_selection_at_line_column_ex]."]
#[must_use]
pub struct ExGetSelectionAtLineColumn < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, line: i32, column: i32, include_edges: bool, only_selections: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectionAtLineColumn < 'a > {
    fn new(surround_object: &'a re_export::TextEdit, line: i32, column: i32,) -> Self {
        let include_edges = true;
        let only_selections = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, column: column, include_edges: include_edges, only_selections: only_selections,
        }
    }
    #[inline]
    pub fn include_edges(self, include_edges: bool) -> Self {
        Self {
            include_edges: include_edges, .. self
        }
    }
    #[inline]
    pub fn only_selections(self, only_selections: bool) -> Self {
        Self {
            only_selections: only_selections, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, line, column, include_edges, only_selections,
        }
        = self;
        re_export::TextEdit::get_selection_at_line_column_full(surround_object, line, column, include_edges, only_selections,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_line_ranges_from_carets_ex`][super::TextEdit::get_line_ranges_from_carets_ex]."]
#[must_use]
pub struct ExGetLineRangesFromCarets < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, only_selections: bool, merge_adjacent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetLineRangesFromCarets < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let only_selections = false;
        let merge_adjacent = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, only_selections: only_selections, merge_adjacent: merge_adjacent,
        }
    }
    #[inline]
    pub fn only_selections(self, only_selections: bool) -> Self {
        Self {
            only_selections: only_selections, .. self
        }
    }
    #[inline]
    pub fn merge_adjacent(self, merge_adjacent: bool) -> Self {
        Self {
            merge_adjacent: merge_adjacent, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Vector2i > {
        let Self {
            _phantom, surround_object, only_selections, merge_adjacent,
        }
        = self;
        re_export::TextEdit::get_line_ranges_from_carets_full(surround_object, only_selections, merge_adjacent,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_origin_line_ex`][super::TextEdit::get_selection_origin_line_ex]."]
#[must_use]
pub struct ExGetSelectionOriginLine < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectionOriginLine < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selection_origin_line_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_origin_column_ex`][super::TextEdit::get_selection_origin_column_ex]."]
#[must_use]
pub struct ExGetSelectionOriginColumn < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectionOriginColumn < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selection_origin_column_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_selection_origin_line_ex`][super::TextEdit::set_selection_origin_line_ex]."]
#[must_use]
pub struct ExSetSelectionOriginLine < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, line: i32, can_be_hidden: bool, wrap_index: i32, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetSelectionOriginLine < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, line: i32,) -> Self {
        let can_be_hidden = true;
        let wrap_index = - 1i32;
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, can_be_hidden: can_be_hidden, wrap_index: wrap_index, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn can_be_hidden(self, can_be_hidden: bool) -> Self {
        Self {
            can_be_hidden: can_be_hidden, .. self
        }
    }
    #[inline]
    pub fn wrap_index(self, wrap_index: i32) -> Self {
        Self {
            wrap_index: wrap_index, .. self
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, line, can_be_hidden, wrap_index, caret_index,
        }
        = self;
        re_export::TextEdit::set_selection_origin_line_full(surround_object, line, can_be_hidden, wrap_index, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_selection_origin_column_ex`][super::TextEdit::set_selection_origin_column_ex]."]
#[must_use]
pub struct ExSetSelectionOriginColumn < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, column: i32, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetSelectionOriginColumn < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, column: i32,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, caret_index,
        }
        = self;
        re_export::TextEdit::set_selection_origin_column_full(surround_object, column, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_from_line_ex`][super::TextEdit::get_selection_from_line_ex]."]
#[must_use]
pub struct ExGetSelectionFromLine < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectionFromLine < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selection_from_line_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_from_column_ex`][super::TextEdit::get_selection_from_column_ex]."]
#[must_use]
pub struct ExGetSelectionFromColumn < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectionFromColumn < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selection_from_column_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_to_line_ex`][super::TextEdit::get_selection_to_line_ex]."]
#[must_use]
pub struct ExGetSelectionToLine < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectionToLine < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selection_to_line_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_to_column_ex`][super::TextEdit::get_selection_to_column_ex]."]
#[must_use]
pub struct ExGetSelectionToColumn < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectionToColumn < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selection_to_column_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::is_caret_after_selection_origin_ex`][super::TextEdit::is_caret_after_selection_origin_ex]."]
#[must_use]
pub struct ExIsCaretAfterSelectionOrigin < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsCaretAfterSelectionOrigin < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::is_caret_after_selection_origin_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::deselect_ex`][super::TextEdit::deselect_ex]."]
#[must_use]
pub struct ExDeselect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDeselect < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::deselect_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::delete_selection_ex`][super::TextEdit::delete_selection_ex]."]
#[must_use]
pub struct ExDeleteSelection < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDeleteSelection < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        let caret_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::delete_selection_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_scroll_pos_for_line_ex`][super::TextEdit::get_scroll_pos_for_line_ex]."]
#[must_use]
pub struct ExGetScrollPosForLine < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, line: i32, wrap_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetScrollPosForLine < 'a > {
    fn new(surround_object: &'a re_export::TextEdit, line: i32,) -> Self {
        let wrap_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, wrap_index: wrap_index,
        }
    }
    #[inline]
    pub fn wrap_index(self, wrap_index: i32) -> Self {
        Self {
            wrap_index: wrap_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f64 {
        let Self {
            _phantom, surround_object, line, wrap_index,
        }
        = self;
        re_export::TextEdit::get_scroll_pos_for_line_full(surround_object, line, wrap_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_line_as_first_visible_ex`][super::TextEdit::set_line_as_first_visible_ex]."]
#[must_use]
pub struct ExSetLineAsFirstVisible < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, line: i32, wrap_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetLineAsFirstVisible < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, line: i32,) -> Self {
        let wrap_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, wrap_index: wrap_index,
        }
    }
    #[inline]
    pub fn wrap_index(self, wrap_index: i32) -> Self {
        Self {
            wrap_index: wrap_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, line, wrap_index,
        }
        = self;
        re_export::TextEdit::set_line_as_first_visible_full(surround_object, line, wrap_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_line_as_center_visible_ex`][super::TextEdit::set_line_as_center_visible_ex]."]
#[must_use]
pub struct ExSetLineAsCenterVisible < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, line: i32, wrap_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetLineAsCenterVisible < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, line: i32,) -> Self {
        let wrap_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, wrap_index: wrap_index,
        }
    }
    #[inline]
    pub fn wrap_index(self, wrap_index: i32) -> Self {
        Self {
            wrap_index: wrap_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, line, wrap_index,
        }
        = self;
        re_export::TextEdit::set_line_as_center_visible_full(surround_object, line, wrap_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::set_line_as_last_visible_ex`][super::TextEdit::set_line_as_last_visible_ex]."]
#[must_use]
pub struct ExSetLineAsLastVisible < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, line: i32, wrap_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetLineAsLastVisible < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit, line: i32,) -> Self {
        let wrap_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, wrap_index: wrap_index,
        }
    }
    #[inline]
    pub fn wrap_index(self, wrap_index: i32) -> Self {
        Self {
            wrap_index: wrap_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, line, wrap_index,
        }
        = self;
        re_export::TextEdit::set_line_as_last_visible_full(surround_object, line, wrap_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::adjust_viewport_to_caret_ex`][super::TextEdit::adjust_viewport_to_caret_ex]."]
#[must_use]
pub struct ExAdjustViewportToCaret < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAdjustViewportToCaret < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::adjust_viewport_to_caret_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::center_viewport_to_caret_ex`][super::TextEdit::center_viewport_to_caret_ex]."]
#[must_use]
pub struct ExCenterViewportToCaret < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCenterViewportToCaret < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::center_viewport_to_caret_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::add_gutter_ex`][super::TextEdit::add_gutter_ex]."]
#[must_use]
pub struct ExAddGutter < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextEdit, at: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddGutter < 'a > {
    fn new(surround_object: &'a mut re_export::TextEdit,) -> Self {
        let at = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, at: at,
        }
    }
    #[inline]
    pub fn at(self, at: i32) -> Self {
        Self {
            at: at, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, at,
        }
        = self;
        re_export::TextEdit::add_gutter_full(surround_object, at,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_line_ex`][super::TextEdit::get_selection_line_ex]."]
#[must_use]
pub struct ExGetSelectionLine < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectionLine < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selection_line_full(surround_object, caret_index,)
    }
}
#[doc = "Default-param extender for [`TextEdit::get_selection_column_ex`][super::TextEdit::get_selection_column_ex]."]
#[must_use]
pub struct ExGetSelectionColumn < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextEdit, caret_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectionColumn < 'a > {
    fn new(surround_object: &'a re_export::TextEdit,) -> Self {
        let caret_index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, caret_index: caret_index,
        }
    }
    #[inline]
    pub fn caret_index(self, caret_index: i32) -> Self {
        Self {
            caret_index: caret_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, caret_index,
        }
        = self;
        re_export::TextEdit::get_selection_column_full(surround_object, caret_index,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MenuItems {
    ord: i32
}
impl MenuItems {
    #[doc(alias = "MENU_CUT")]
    #[doc = "Godot enumerator name: `MENU_CUT`"]
    pub const CUT: MenuItems = MenuItems {
        ord: 0i32
    };
    #[doc(alias = "MENU_COPY")]
    #[doc = "Godot enumerator name: `MENU_COPY`"]
    pub const COPY: MenuItems = MenuItems {
        ord: 1i32
    };
    #[doc(alias = "MENU_PASTE")]
    #[doc = "Godot enumerator name: `MENU_PASTE`"]
    pub const PASTE: MenuItems = MenuItems {
        ord: 2i32
    };
    #[doc(alias = "MENU_CLEAR")]
    #[doc = "Godot enumerator name: `MENU_CLEAR`"]
    pub const CLEAR: MenuItems = MenuItems {
        ord: 3i32
    };
    #[doc(alias = "MENU_SELECT_ALL")]
    #[doc = "Godot enumerator name: `MENU_SELECT_ALL`"]
    pub const SELECT_ALL: MenuItems = MenuItems {
        ord: 4i32
    };
    #[doc(alias = "MENU_UNDO")]
    #[doc = "Godot enumerator name: `MENU_UNDO`"]
    pub const UNDO: MenuItems = MenuItems {
        ord: 5i32
    };
    #[doc(alias = "MENU_REDO")]
    #[doc = "Godot enumerator name: `MENU_REDO`"]
    pub const REDO: MenuItems = MenuItems {
        ord: 6i32
    };
    #[doc(alias = "MENU_SUBMENU_TEXT_DIR")]
    #[doc = "Godot enumerator name: `MENU_SUBMENU_TEXT_DIR`"]
    pub const SUBMENU_TEXT_DIR: MenuItems = MenuItems {
        ord: 7i32
    };
    #[doc(alias = "MENU_DIR_INHERITED")]
    #[doc = "Godot enumerator name: `MENU_DIR_INHERITED`"]
    pub const DIR_INHERITED: MenuItems = MenuItems {
        ord: 8i32
    };
    #[doc(alias = "MENU_DIR_AUTO")]
    #[doc = "Godot enumerator name: `MENU_DIR_AUTO`"]
    pub const DIR_AUTO: MenuItems = MenuItems {
        ord: 9i32
    };
    #[doc(alias = "MENU_DIR_LTR")]
    #[doc = "Godot enumerator name: `MENU_DIR_LTR`"]
    pub const DIR_LTR: MenuItems = MenuItems {
        ord: 10i32
    };
    #[doc(alias = "MENU_DIR_RTL")]
    #[doc = "Godot enumerator name: `MENU_DIR_RTL`"]
    pub const DIR_RTL: MenuItems = MenuItems {
        ord: 11i32
    };
    #[doc(alias = "MENU_DISPLAY_UCC")]
    #[doc = "Godot enumerator name: `MENU_DISPLAY_UCC`"]
    pub const DISPLAY_UCC: MenuItems = MenuItems {
        ord: 12i32
    };
    #[doc(alias = "MENU_SUBMENU_INSERT_UCC")]
    #[doc = "Godot enumerator name: `MENU_SUBMENU_INSERT_UCC`"]
    pub const SUBMENU_INSERT_UCC: MenuItems = MenuItems {
        ord: 13i32
    };
    #[doc(alias = "MENU_INSERT_LRM")]
    #[doc = "Godot enumerator name: `MENU_INSERT_LRM`"]
    pub const INSERT_LRM: MenuItems = MenuItems {
        ord: 14i32
    };
    #[doc(alias = "MENU_INSERT_RLM")]
    #[doc = "Godot enumerator name: `MENU_INSERT_RLM`"]
    pub const INSERT_RLM: MenuItems = MenuItems {
        ord: 15i32
    };
    #[doc(alias = "MENU_INSERT_LRE")]
    #[doc = "Godot enumerator name: `MENU_INSERT_LRE`"]
    pub const INSERT_LRE: MenuItems = MenuItems {
        ord: 16i32
    };
    #[doc(alias = "MENU_INSERT_RLE")]
    #[doc = "Godot enumerator name: `MENU_INSERT_RLE`"]
    pub const INSERT_RLE: MenuItems = MenuItems {
        ord: 17i32
    };
    #[doc(alias = "MENU_INSERT_LRO")]
    #[doc = "Godot enumerator name: `MENU_INSERT_LRO`"]
    pub const INSERT_LRO: MenuItems = MenuItems {
        ord: 18i32
    };
    #[doc(alias = "MENU_INSERT_RLO")]
    #[doc = "Godot enumerator name: `MENU_INSERT_RLO`"]
    pub const INSERT_RLO: MenuItems = MenuItems {
        ord: 19i32
    };
    #[doc(alias = "MENU_INSERT_PDF")]
    #[doc = "Godot enumerator name: `MENU_INSERT_PDF`"]
    pub const INSERT_PDF: MenuItems = MenuItems {
        ord: 20i32
    };
    #[doc(alias = "MENU_INSERT_ALM")]
    #[doc = "Godot enumerator name: `MENU_INSERT_ALM`"]
    pub const INSERT_ALM: MenuItems = MenuItems {
        ord: 21i32
    };
    #[doc(alias = "MENU_INSERT_LRI")]
    #[doc = "Godot enumerator name: `MENU_INSERT_LRI`"]
    pub const INSERT_LRI: MenuItems = MenuItems {
        ord: 22i32
    };
    #[doc(alias = "MENU_INSERT_RLI")]
    #[doc = "Godot enumerator name: `MENU_INSERT_RLI`"]
    pub const INSERT_RLI: MenuItems = MenuItems {
        ord: 23i32
    };
    #[doc(alias = "MENU_INSERT_FSI")]
    #[doc = "Godot enumerator name: `MENU_INSERT_FSI`"]
    pub const INSERT_FSI: MenuItems = MenuItems {
        ord: 24i32
    };
    #[doc(alias = "MENU_INSERT_PDI")]
    #[doc = "Godot enumerator name: `MENU_INSERT_PDI`"]
    pub const INSERT_PDI: MenuItems = MenuItems {
        ord: 25i32
    };
    #[doc(alias = "MENU_INSERT_ZWJ")]
    #[doc = "Godot enumerator name: `MENU_INSERT_ZWJ`"]
    pub const INSERT_ZWJ: MenuItems = MenuItems {
        ord: 26i32
    };
    #[doc(alias = "MENU_INSERT_ZWNJ")]
    #[doc = "Godot enumerator name: `MENU_INSERT_ZWNJ`"]
    pub const INSERT_ZWNJ: MenuItems = MenuItems {
        ord: 27i32
    };
    #[doc(alias = "MENU_INSERT_WJ")]
    #[doc = "Godot enumerator name: `MENU_INSERT_WJ`"]
    pub const INSERT_WJ: MenuItems = MenuItems {
        ord: 28i32
    };
    #[doc(alias = "MENU_INSERT_SHY")]
    #[doc = "Godot enumerator name: `MENU_INSERT_SHY`"]
    pub const INSERT_SHY: MenuItems = MenuItems {
        ord: 29i32
    };
    #[doc(alias = "MENU_EMOJI_AND_SYMBOL")]
    #[doc = "Godot enumerator name: `MENU_EMOJI_AND_SYMBOL`"]
    pub const EMOJI_AND_SYMBOL: MenuItems = MenuItems {
        ord: 30i32
    };
    #[doc(alias = "MENU_MAX")]
    #[doc = "Godot enumerator name: `MENU_MAX`"]
    pub const MAX: MenuItems = MenuItems {
        ord: 31i32
    };
    
}
impl std::fmt::Debug for MenuItems {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MenuItems") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MenuItems {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 => Some(Self {
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
            Self::CUT => "CUT", Self::COPY => "COPY", Self::PASTE => "PASTE", Self::CLEAR => "CLEAR", Self::SELECT_ALL => "SELECT_ALL", Self::UNDO => "UNDO", Self::REDO => "REDO", Self::SUBMENU_TEXT_DIR => "SUBMENU_TEXT_DIR", Self::DIR_INHERITED => "DIR_INHERITED", Self::DIR_AUTO => "DIR_AUTO", Self::DIR_LTR => "DIR_LTR", Self::DIR_RTL => "DIR_RTL", Self::DISPLAY_UCC => "DISPLAY_UCC", Self::SUBMENU_INSERT_UCC => "SUBMENU_INSERT_UCC", Self::INSERT_LRM => "INSERT_LRM", Self::INSERT_RLM => "INSERT_RLM", Self::INSERT_LRE => "INSERT_LRE", Self::INSERT_RLE => "INSERT_RLE", Self::INSERT_LRO => "INSERT_LRO", Self::INSERT_RLO => "INSERT_RLO", Self::INSERT_PDF => "INSERT_PDF", Self::INSERT_ALM => "INSERT_ALM", Self::INSERT_LRI => "INSERT_LRI", Self::INSERT_RLI => "INSERT_RLI", Self::INSERT_FSI => "INSERT_FSI", Self::INSERT_PDI => "INSERT_PDI", Self::INSERT_ZWJ => "INSERT_ZWJ", Self::INSERT_ZWNJ => "INSERT_ZWNJ", Self::INSERT_WJ => "INSERT_WJ", Self::INSERT_SHY => "INSERT_SHY", Self::EMOJI_AND_SYMBOL => "EMOJI_AND_SYMBOL", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MenuItems::CUT, MenuItems::COPY, MenuItems::PASTE, MenuItems::CLEAR, MenuItems::SELECT_ALL, MenuItems::UNDO, MenuItems::REDO, MenuItems::SUBMENU_TEXT_DIR, MenuItems::DIR_INHERITED, MenuItems::DIR_AUTO, MenuItems::DIR_LTR, MenuItems::DIR_RTL, MenuItems::DISPLAY_UCC, MenuItems::SUBMENU_INSERT_UCC, MenuItems::INSERT_LRM, MenuItems::INSERT_RLM, MenuItems::INSERT_LRE, MenuItems::INSERT_RLE, MenuItems::INSERT_LRO, MenuItems::INSERT_RLO, MenuItems::INSERT_PDF, MenuItems::INSERT_ALM, MenuItems::INSERT_LRI, MenuItems::INSERT_RLI, MenuItems::INSERT_FSI, MenuItems::INSERT_PDI, MenuItems::INSERT_ZWJ, MenuItems::INSERT_ZWNJ, MenuItems::INSERT_WJ, MenuItems::INSERT_SHY, MenuItems::EMOJI_AND_SYMBOL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MenuItems >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CUT", "MENU_CUT", MenuItems::CUT), crate::meta::inspect::EnumConstant::new("COPY", "MENU_COPY", MenuItems::COPY), crate::meta::inspect::EnumConstant::new("PASTE", "MENU_PASTE", MenuItems::PASTE), crate::meta::inspect::EnumConstant::new("CLEAR", "MENU_CLEAR", MenuItems::CLEAR), crate::meta::inspect::EnumConstant::new("SELECT_ALL", "MENU_SELECT_ALL", MenuItems::SELECT_ALL), crate::meta::inspect::EnumConstant::new("UNDO", "MENU_UNDO", MenuItems::UNDO), crate::meta::inspect::EnumConstant::new("REDO", "MENU_REDO", MenuItems::REDO), crate::meta::inspect::EnumConstant::new("SUBMENU_TEXT_DIR", "MENU_SUBMENU_TEXT_DIR", MenuItems::SUBMENU_TEXT_DIR), crate::meta::inspect::EnumConstant::new("DIR_INHERITED", "MENU_DIR_INHERITED", MenuItems::DIR_INHERITED), crate::meta::inspect::EnumConstant::new("DIR_AUTO", "MENU_DIR_AUTO", MenuItems::DIR_AUTO), crate::meta::inspect::EnumConstant::new("DIR_LTR", "MENU_DIR_LTR", MenuItems::DIR_LTR), crate::meta::inspect::EnumConstant::new("DIR_RTL", "MENU_DIR_RTL", MenuItems::DIR_RTL), crate::meta::inspect::EnumConstant::new("DISPLAY_UCC", "MENU_DISPLAY_UCC", MenuItems::DISPLAY_UCC), crate::meta::inspect::EnumConstant::new("SUBMENU_INSERT_UCC", "MENU_SUBMENU_INSERT_UCC", MenuItems::SUBMENU_INSERT_UCC), crate::meta::inspect::EnumConstant::new("INSERT_LRM", "MENU_INSERT_LRM", MenuItems::INSERT_LRM), crate::meta::inspect::EnumConstant::new("INSERT_RLM", "MENU_INSERT_RLM", MenuItems::INSERT_RLM), crate::meta::inspect::EnumConstant::new("INSERT_LRE", "MENU_INSERT_LRE", MenuItems::INSERT_LRE), crate::meta::inspect::EnumConstant::new("INSERT_RLE", "MENU_INSERT_RLE", MenuItems::INSERT_RLE), crate::meta::inspect::EnumConstant::new("INSERT_LRO", "MENU_INSERT_LRO", MenuItems::INSERT_LRO), crate::meta::inspect::EnumConstant::new("INSERT_RLO", "MENU_INSERT_RLO", MenuItems::INSERT_RLO), crate::meta::inspect::EnumConstant::new("INSERT_PDF", "MENU_INSERT_PDF", MenuItems::INSERT_PDF), crate::meta::inspect::EnumConstant::new("INSERT_ALM", "MENU_INSERT_ALM", MenuItems::INSERT_ALM), crate::meta::inspect::EnumConstant::new("INSERT_LRI", "MENU_INSERT_LRI", MenuItems::INSERT_LRI), crate::meta::inspect::EnumConstant::new("INSERT_RLI", "MENU_INSERT_RLI", MenuItems::INSERT_RLI), crate::meta::inspect::EnumConstant::new("INSERT_FSI", "MENU_INSERT_FSI", MenuItems::INSERT_FSI), crate::meta::inspect::EnumConstant::new("INSERT_PDI", "MENU_INSERT_PDI", MenuItems::INSERT_PDI), crate::meta::inspect::EnumConstant::new("INSERT_ZWJ", "MENU_INSERT_ZWJ", MenuItems::INSERT_ZWJ), crate::meta::inspect::EnumConstant::new("INSERT_ZWNJ", "MENU_INSERT_ZWNJ", MenuItems::INSERT_ZWNJ), crate::meta::inspect::EnumConstant::new("INSERT_WJ", "MENU_INSERT_WJ", MenuItems::INSERT_WJ), crate::meta::inspect::EnumConstant::new("INSERT_SHY", "MENU_INSERT_SHY", MenuItems::INSERT_SHY), crate::meta::inspect::EnumConstant::new("EMOJI_AND_SYMBOL", "MENU_EMOJI_AND_SYMBOL", MenuItems::EMOJI_AND_SYMBOL), crate::meta::inspect::EnumConstant::new("MAX", "MENU_MAX", MenuItems::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for MenuItems {
    const ENUMERATOR_COUNT: usize = 31usize;
    
}
impl crate::meta::GodotConvert for MenuItems {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MenuItems {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MenuItems {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EditAction {
    ord: i32
}
impl EditAction {
    #[doc(alias = "ACTION_NONE")]
    #[doc = "Godot enumerator name: `ACTION_NONE`"]
    pub const NONE: EditAction = EditAction {
        ord: 0i32
    };
    #[doc(alias = "ACTION_TYPING")]
    #[doc = "Godot enumerator name: `ACTION_TYPING`"]
    pub const TYPING: EditAction = EditAction {
        ord: 1i32
    };
    #[doc(alias = "ACTION_BACKSPACE")]
    #[doc = "Godot enumerator name: `ACTION_BACKSPACE`"]
    pub const BACKSPACE: EditAction = EditAction {
        ord: 2i32
    };
    #[doc(alias = "ACTION_DELETE")]
    #[doc = "Godot enumerator name: `ACTION_DELETE`"]
    pub const DELETE: EditAction = EditAction {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for EditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EditAction") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EditAction {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::NONE => "NONE", Self::TYPING => "TYPING", Self::BACKSPACE => "BACKSPACE", Self::DELETE => "DELETE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EditAction::NONE, EditAction::TYPING, EditAction::BACKSPACE, EditAction::DELETE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EditAction >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "ACTION_NONE", EditAction::NONE), crate::meta::inspect::EnumConstant::new("TYPING", "ACTION_TYPING", EditAction::TYPING), crate::meta::inspect::EnumConstant::new("BACKSPACE", "ACTION_BACKSPACE", EditAction::BACKSPACE), crate::meta::inspect::EnumConstant::new("DELETE", "ACTION_DELETE", EditAction::DELETE)]
        }
    }
}
impl crate::meta::GodotConvert for EditAction {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EditAction {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EditAction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct SearchFlags {
    ord: u64
}
impl SearchFlags {
    #[doc(alias = "SEARCH_MATCH_CASE")]
    #[doc = "Godot enumerator name: `SEARCH_MATCH_CASE`"]
    pub const MATCH_CASE: SearchFlags = SearchFlags {
        ord: 1u64
    };
    #[doc(alias = "SEARCH_WHOLE_WORDS")]
    #[doc = "Godot enumerator name: `SEARCH_WHOLE_WORDS`"]
    pub const WHOLE_WORDS: SearchFlags = SearchFlags {
        ord: 2u64
    };
    #[doc(alias = "SEARCH_BACKWARDS")]
    #[doc = "Godot enumerator name: `SEARCH_BACKWARDS`"]
    pub const BACKWARDS: SearchFlags = SearchFlags {
        ord: 4u64
    };
    
}
impl std::fmt::Debug for SearchFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::MATCH_CASE => "MATCH_CASE", Self::WHOLE_WORDS => "WHOLE_WORDS", Self::BACKWARDS => "BACKWARDS", _ => {
                f.debug_struct("SearchFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for SearchFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SearchFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("MATCH_CASE", "SEARCH_MATCH_CASE", SearchFlags::MATCH_CASE), crate::meta::inspect::EnumConstant::new("WHOLE_WORDS", "SEARCH_WHOLE_WORDS", SearchFlags::WHOLE_WORDS), crate::meta::inspect::EnumConstant::new("BACKWARDS", "SEARCH_BACKWARDS", SearchFlags::BACKWARDS)]
        }
    }
}
impl std::ops::BitOr for SearchFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for SearchFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for SearchFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for SearchFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SearchFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CaretType {
    ord: i32
}
impl CaretType {
    #[doc(alias = "CARET_TYPE_LINE")]
    #[doc = "Godot enumerator name: `CARET_TYPE_LINE`"]
    pub const LINE: CaretType = CaretType {
        ord: 0i32
    };
    #[doc(alias = "CARET_TYPE_BLOCK")]
    #[doc = "Godot enumerator name: `CARET_TYPE_BLOCK`"]
    pub const BLOCK: CaretType = CaretType {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for CaretType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CaretType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CaretType {
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
            Self::LINE => "LINE", Self::BLOCK => "BLOCK", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CaretType::LINE, CaretType::BLOCK]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CaretType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LINE", "CARET_TYPE_LINE", CaretType::LINE), crate::meta::inspect::EnumConstant::new("BLOCK", "CARET_TYPE_BLOCK", CaretType::BLOCK)]
        }
    }
}
impl crate::meta::GodotConvert for CaretType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CaretType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CaretType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SelectionMode {
    ord: i32
}
impl SelectionMode {
    #[doc(alias = "SELECTION_MODE_NONE")]
    #[doc = "Godot enumerator name: `SELECTION_MODE_NONE`"]
    pub const NONE: SelectionMode = SelectionMode {
        ord: 0i32
    };
    #[doc(alias = "SELECTION_MODE_SHIFT")]
    #[doc = "Godot enumerator name: `SELECTION_MODE_SHIFT`"]
    pub const SHIFT: SelectionMode = SelectionMode {
        ord: 1i32
    };
    #[doc(alias = "SELECTION_MODE_POINTER")]
    #[doc = "Godot enumerator name: `SELECTION_MODE_POINTER`"]
    pub const POINTER: SelectionMode = SelectionMode {
        ord: 2i32
    };
    #[doc(alias = "SELECTION_MODE_WORD")]
    #[doc = "Godot enumerator name: `SELECTION_MODE_WORD`"]
    pub const WORD: SelectionMode = SelectionMode {
        ord: 3i32
    };
    #[doc(alias = "SELECTION_MODE_LINE")]
    #[doc = "Godot enumerator name: `SELECTION_MODE_LINE`"]
    pub const LINE: SelectionMode = SelectionMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for SelectionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SelectionMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SelectionMode {
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
            Self::NONE => "NONE", Self::SHIFT => "SHIFT", Self::POINTER => "POINTER", Self::WORD => "WORD", Self::LINE => "LINE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SelectionMode::NONE, SelectionMode::SHIFT, SelectionMode::POINTER, SelectionMode::WORD, SelectionMode::LINE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SelectionMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "SELECTION_MODE_NONE", SelectionMode::NONE), crate::meta::inspect::EnumConstant::new("SHIFT", "SELECTION_MODE_SHIFT", SelectionMode::SHIFT), crate::meta::inspect::EnumConstant::new("POINTER", "SELECTION_MODE_POINTER", SelectionMode::POINTER), crate::meta::inspect::EnumConstant::new("WORD", "SELECTION_MODE_WORD", SelectionMode::WORD), crate::meta::inspect::EnumConstant::new("LINE", "SELECTION_MODE_LINE", SelectionMode::LINE)]
        }
    }
}
impl crate::meta::GodotConvert for SelectionMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SelectionMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SelectionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LineWrappingMode {
    ord: i32
}
impl LineWrappingMode {
    #[doc(alias = "LINE_WRAPPING_NONE")]
    #[doc = "Godot enumerator name: `LINE_WRAPPING_NONE`"]
    pub const NONE: LineWrappingMode = LineWrappingMode {
        ord: 0i32
    };
    #[doc(alias = "LINE_WRAPPING_BOUNDARY")]
    #[doc = "Godot enumerator name: `LINE_WRAPPING_BOUNDARY`"]
    pub const BOUNDARY: LineWrappingMode = LineWrappingMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for LineWrappingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LineWrappingMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LineWrappingMode {
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
            Self::NONE => "NONE", Self::BOUNDARY => "BOUNDARY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LineWrappingMode::NONE, LineWrappingMode::BOUNDARY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LineWrappingMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "LINE_WRAPPING_NONE", LineWrappingMode::NONE), crate::meta::inspect::EnumConstant::new("BOUNDARY", "LINE_WRAPPING_BOUNDARY", LineWrappingMode::BOUNDARY)]
        }
    }
}
impl crate::meta::GodotConvert for LineWrappingMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LineWrappingMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LineWrappingMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GutterType {
    ord: i32
}
impl GutterType {
    #[doc(alias = "GUTTER_TYPE_STRING")]
    #[doc = "Godot enumerator name: `GUTTER_TYPE_STRING`"]
    pub const STRING: GutterType = GutterType {
        ord: 0i32
    };
    #[doc(alias = "GUTTER_TYPE_ICON")]
    #[doc = "Godot enumerator name: `GUTTER_TYPE_ICON`"]
    pub const ICON: GutterType = GutterType {
        ord: 1i32
    };
    #[doc(alias = "GUTTER_TYPE_CUSTOM")]
    #[doc = "Godot enumerator name: `GUTTER_TYPE_CUSTOM`"]
    pub const CUSTOM: GutterType = GutterType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for GutterType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GutterType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GutterType {
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
            Self::STRING => "STRING", Self::ICON => "ICON", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[GutterType::STRING, GutterType::ICON, GutterType::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GutterType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STRING", "GUTTER_TYPE_STRING", GutterType::STRING), crate::meta::inspect::EnumConstant::new("ICON", "GUTTER_TYPE_ICON", GutterType::ICON), crate::meta::inspect::EnumConstant::new("CUSTOM", "GUTTER_TYPE_CUSTOM", GutterType::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for GutterType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for GutterType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GutterType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TextEdit;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`TextEdit`][crate::classes::TextEdit] class."]
    pub struct SignalsOfTextEdit < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfTextEdit < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn text_set(&mut self) -> SigTextSet < 'c, C > {
            SigTextSet {
                typed: TypedSignal::extract(&mut self.__internal_obj, "text_set")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn text_changed(&mut self) -> SigTextChanged < 'c, C > {
            SigTextChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "text_changed")
            }
        }
        #[doc = "Signature: `(from_line: i64, to_line: i64)`"]
        pub fn lines_edited_from(&mut self) -> SigLinesEditedFrom < 'c, C > {
            SigLinesEditedFrom {
                typed: TypedSignal::extract(&mut self.__internal_obj, "lines_edited_from")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn caret_changed(&mut self) -> SigCaretChanged < 'c, C > {
            SigCaretChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "caret_changed")
            }
        }
        #[doc = "Signature: `(line: i64, gutter: i64)`"]
        pub fn gutter_clicked(&mut self) -> SigGutterClicked < 'c, C > {
            SigGutterClicked {
                typed: TypedSignal::extract(&mut self.__internal_obj, "gutter_clicked")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn gutter_added(&mut self) -> SigGutterAdded < 'c, C > {
            SigGutterAdded {
                typed: TypedSignal::extract(&mut self.__internal_obj, "gutter_added")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn gutter_removed(&mut self) -> SigGutterRemoved < 'c, C > {
            SigGutterRemoved {
                typed: TypedSignal::extract(&mut self.__internal_obj, "gutter_removed")
            }
        }
    }
    type TypedSigTextSet < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTextSet < 'c, C: WithSignals > {
        typed: TypedSigTextSet < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTextSet < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTextSet < 'c, C > {
        type Target = TypedSigTextSet < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTextSet < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTextChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTextChanged < 'c, C: WithSignals > {
        typed: TypedSigTextChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTextChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTextChanged < 'c, C > {
        type Target = TypedSigTextChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTextChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigLinesEditedFrom < 'c, C > = TypedSignal < 'c, C, (i64, i64,) >;
    pub struct SigLinesEditedFrom < 'c, C: WithSignals > {
        typed: TypedSigLinesEditedFrom < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigLinesEditedFrom < 'c, C > {
        pub fn emit(&mut self, from_line: i64, to_line: i64,) {
            self.typed.emit_tuple((from_line, to_line,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigLinesEditedFrom < 'c, C > {
        type Target = TypedSigLinesEditedFrom < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigLinesEditedFrom < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCaretChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigCaretChanged < 'c, C: WithSignals > {
        typed: TypedSigCaretChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCaretChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCaretChanged < 'c, C > {
        type Target = TypedSigCaretChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCaretChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigGutterClicked < 'c, C > = TypedSignal < 'c, C, (i64, i64,) >;
    pub struct SigGutterClicked < 'c, C: WithSignals > {
        typed: TypedSigGutterClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigGutterClicked < 'c, C > {
        pub fn emit(&mut self, line: i64, gutter: i64,) {
            self.typed.emit_tuple((line, gutter,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigGutterClicked < 'c, C > {
        type Target = TypedSigGutterClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigGutterClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigGutterAdded < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigGutterAdded < 'c, C: WithSignals > {
        typed: TypedSigGutterAdded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigGutterAdded < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigGutterAdded < 'c, C > {
        type Target = TypedSigGutterAdded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigGutterAdded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigGutterRemoved < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigGutterRemoved < 'c, C: WithSignals > {
        typed: TypedSigGutterRemoved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigGutterRemoved < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigGutterRemoved < 'c, C > {
        type Target = TypedSigGutterRemoved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigGutterRemoved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for TextEdit {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfTextEdit < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfTextEdit < 'c, C > {
        type Target = < < TextEdit as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = TextEdit;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfTextEdit < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = TextEdit;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}