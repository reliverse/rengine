#![doc = "Sidecar module for class [`CodeEdit`][crate::classes::CodeEdit].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CodeEdit` enums](https://docs.godotengine.org/en/stable/classes/class_codeedit.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CodeEdit.`\n\nInherits [`TextEdit`][crate::classes::TextEdit].\n\nRelated symbols:\n\n* [`code_edit`][crate::classes::code_edit]: sidecar module with related enum/flag types\n* [`ICodeEdit`][crate::classes::ICodeEdit]: virtual methods\n* [`SignalsOfCodeEdit`][crate::classes::code_edit::SignalsOfCodeEdit]: signal collection\n\n\nSee also [Godot docs for `CodeEdit`](https://docs.godotengine.org/en/stable/classes/class_codeedit.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`CodeEdit::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CodeEdit {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`CodeEdit`][crate::classes::CodeEdit].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`ITextEdit`][crate::classes::ITextEdit] > [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `CodeEdit` methods](https://docs.godotengine.org/en/stable/classes/class_codeedit.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICodeEdit: crate::obj::GodotClass < Base = CodeEdit > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn confirm_code_completion(&mut self, replace: bool,) {
            unimplemented !()
        }
        fn request_code_completion(&mut self, force: bool,) {
            unimplemented !()
        }
        fn filter_code_completion_candidates(&self, candidates: Array < VarDictionary >,) -> Array < VarDictionary > {
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
    impl CodeEdit {
        pub fn set_indent_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_indent_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_indent_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_indent_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_indent_using_spaces(&mut self, use_spaces: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (use_spaces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_indent_using_spaces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_indent_using_spaces(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_indent_using_spaces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_indent_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_auto_indent_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_auto_indent_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_auto_indent_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_indent_prefixes(&mut self, prefixes: &Array < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < GString > >,);
            let args = (RefArg::new(prefixes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_auto_indent_prefixes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_auto_indent_prefixes(&self,) -> Array < GString > {
            type CallRet = Array < GString >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_auto_indent_prefixes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn do_indent(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "do_indent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn indent_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "indent_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn unindent_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "unindent_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn convert_indent_full(&mut self, from_line: i32, to_line: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from_line, to_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "convert_indent", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::convert_indent_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn convert_indent(&mut self,) {
            self.convert_indent_ex() . done()
        }
        #[inline]
        pub fn convert_indent_ex < 'a > (&'a mut self,) -> ExConvertIndent < 'a > {
            ExConvertIndent::new(self,)
        }
        pub fn set_auto_brace_completion_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_auto_brace_completion_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_auto_brace_completion_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_auto_brace_completion_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_highlight_matching_braces_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_highlight_matching_braces_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_highlight_matching_braces_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2148usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_highlight_matching_braces_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_auto_brace_completion_pair(&mut self, start_key: impl AsArg < GString >, end_key: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (start_key.into_arg(), end_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2149usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "add_auto_brace_completion_pair", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_brace_completion_pairs(&mut self, pairs: &VarDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >,);
            let args = (RefArg::new(pairs),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_auto_brace_completion_pairs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_auto_brace_completion_pairs(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_auto_brace_completion_pairs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_auto_brace_completion_open_key(&self, open_key: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (open_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "has_auto_brace_completion_open_key", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_auto_brace_completion_close_key(&self, close_key: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (close_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "has_auto_brace_completion_close_key", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_auto_brace_completion_close_key(&self, open_key: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (open_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_auto_brace_completion_close_key", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_breakpoints_gutter(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_draw_breakpoints_gutter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drawing_breakpoints_gutter(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_drawing_breakpoints_gutter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_bookmarks_gutter(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_draw_bookmarks_gutter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drawing_bookmarks_gutter(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_drawing_bookmarks_gutter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_executing_lines_gutter(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_draw_executing_lines_gutter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drawing_executing_lines_gutter(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_drawing_executing_lines_gutter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_as_breakpoint(&mut self, line: i32, breakpointed: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (line, breakpointed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_as_breakpoint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_line_breakpointed(&self, line: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_breakpointed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_breakpointed_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "clear_breakpointed_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_breakpointed_lines(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_breakpointed_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_as_bookmarked(&mut self, line: i32, bookmarked: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (line, bookmarked,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_as_bookmarked", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_line_bookmarked(&self, line: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_bookmarked", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_bookmarked_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "clear_bookmarked_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bookmarked_lines(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_bookmarked_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_as_executing(&mut self, line: i32, executing: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (line, executing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_as_executing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_line_executing(&self, line: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_executing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_executing_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "clear_executing_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_executing_lines(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_executing_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_line_numbers(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_draw_line_numbers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_draw_line_numbers_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_draw_line_numbers_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_numbers_zero_padded(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_numbers_zero_padded", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_line_numbers_zero_padded(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_numbers_zero_padded", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_fold_gutter(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_draw_fold_gutter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drawing_fold_gutter(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_drawing_fold_gutter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_folding_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_folding_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_line_folding_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_folding_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn can_fold_line(&self, line: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "can_fold_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn fold_line(&mut self, line: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "fold_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn unfold_line(&mut self, line: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "unfold_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn fold_all_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "fold_all_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn unfold_all_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "unfold_all_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn toggle_foldable_line(&mut self, line: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "toggle_foldable_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn toggle_foldable_lines_at_carets(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "toggle_foldable_lines_at_carets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_line_folded(&self, line: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_folded", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_folded_lines(&self,) -> Array < i64 > {
            type CallRet = Array < i64 >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_folded_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_code_region(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "create_code_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_code_region_start_tag(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_region_start_tag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_code_region_end_tag(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_region_end_tag", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn set_code_region_tags_full(&mut self, start: CowArg < GString >, end: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_region_tags", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_code_region_tags_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_code_region_tags(&mut self,) {
            self.set_code_region_tags_ex() . done()
        }
        #[inline]
        pub fn set_code_region_tags_ex < 'a > (&'a mut self,) -> ExSetCodeRegionTags < 'a > {
            ExSetCodeRegionTags::new(self,)
        }
        pub fn is_line_code_region_start(&self, line: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_code_region_start", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_line_code_region_end(&self, line: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_line_code_region_end", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_string_delimiter_full(&mut self, start_key: CowArg < GString >, end_key: CowArg < GString >, line_only: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, bool,);
            let args = (start_key, end_key, line_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "add_string_delimiter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_string_delimiter_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_string_delimiter(&mut self, start_key: impl AsArg < GString >, end_key: impl AsArg < GString >,) {
            self.add_string_delimiter_ex(start_key, end_key,) . done()
        }
        #[inline]
        pub fn add_string_delimiter_ex < 'a > (&'a mut self, start_key: impl AsArg < GString > + 'a, end_key: impl AsArg < GString > + 'a,) -> ExAddStringDelimiter < 'a > {
            ExAddStringDelimiter::new(self, start_key, end_key,)
        }
        pub fn remove_string_delimiter(&mut self, start_key: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (start_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "remove_string_delimiter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_string_delimiter(&self, start_key: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (start_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "has_string_delimiter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_string_delimiters(&mut self, string_delimiters: &Array < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < GString > >,);
            let args = (RefArg::new(string_delimiters),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_string_delimiters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_string_delimiters(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "clear_string_delimiters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_string_delimiters(&self,) -> Array < GString > {
            type CallRet = Array < GString >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_string_delimiters", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn is_in_string_full(&self, line: i32, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_in_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_in_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_in_string(&self, line: i32,) -> i32 {
            self.is_in_string_ex(line,) . done()
        }
        #[inline]
        pub fn is_in_string_ex < 'a > (&'a self, line: i32,) -> ExIsInString < 'a > {
            ExIsInString::new(self, line,)
        }
        pub(crate) fn add_comment_delimiter_full(&mut self, start_key: CowArg < GString >, end_key: CowArg < GString >, line_only: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, bool,);
            let args = (start_key, end_key, line_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "add_comment_delimiter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_comment_delimiter_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_comment_delimiter(&mut self, start_key: impl AsArg < GString >, end_key: impl AsArg < GString >,) {
            self.add_comment_delimiter_ex(start_key, end_key,) . done()
        }
        #[inline]
        pub fn add_comment_delimiter_ex < 'a > (&'a mut self, start_key: impl AsArg < GString > + 'a, end_key: impl AsArg < GString > + 'a,) -> ExAddCommentDelimiter < 'a > {
            ExAddCommentDelimiter::new(self, start_key, end_key,)
        }
        pub fn remove_comment_delimiter(&mut self, start_key: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (start_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "remove_comment_delimiter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_comment_delimiter(&self, start_key: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (start_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "has_comment_delimiter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_comment_delimiters(&mut self, comment_delimiters: &Array < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < GString > >,);
            let args = (RefArg::new(comment_delimiters),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_comment_delimiters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_comment_delimiters(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "clear_comment_delimiters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_comment_delimiters(&self,) -> Array < GString > {
            type CallRet = Array < GString >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_comment_delimiters", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn is_in_comment_full(&self, line: i32, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_in_comment", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_in_comment_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_in_comment(&self, line: i32,) -> i32 {
            self.is_in_comment_ex(line,) . done()
        }
        #[inline]
        pub fn is_in_comment_ex < 'a > (&'a self, line: i32,) -> ExIsInComment < 'a > {
            ExIsInComment::new(self, line,)
        }
        pub fn get_delimiter_start_key(&self, delimiter_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (delimiter_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_delimiter_start_key", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_delimiter_end_key(&self, delimiter_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (delimiter_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_delimiter_end_key", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_delimiter_start_position(&self, line: i32, column: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_delimiter_start_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_delimiter_end_position(&self, line: i32, column: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_delimiter_end_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_code_hint(&mut self, code_hint: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (code_hint.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_hint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_code_hint_draw_below(&mut self, draw_below: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (draw_below,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_hint_draw_below", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_for_code_completion(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_text_for_code_completion", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn request_code_completion_full(&mut self, force: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "request_code_completion", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::request_code_completion_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn request_code_completion(&mut self,) {
            self.request_code_completion_ex() . done()
        }
        #[inline]
        pub fn request_code_completion_ex < 'a > (&'a mut self,) -> ExRequestCodeCompletion < 'a > {
            ExRequestCodeCompletion::new(self,)
        }
        pub(crate) fn add_code_completion_option_full(&mut self, type_: crate::classes::code_edit::CodeCompletionKind, display_text: CowArg < GString >, insert_text: CowArg < GString >, text_color: Color, icon: CowArg < Option < Gd < crate::classes::Resource > > >, value: RefArg < Variant >, location: crate::classes::code_edit::CodeCompletionLocation,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (crate::classes::code_edit::CodeCompletionKind, CowArg < 'a0, GString >, CowArg < 'a1, GString >, Color, CowArg < 'a2, Option < Gd < crate::classes::Resource > > >, RefArg < 'a3, Variant >, crate::classes::code_edit::CodeCompletionLocation,);
            let args = (type_, display_text, insert_text, text_color, icon, value, location,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "add_code_completion_option", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_code_completion_option_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_code_completion_option(&mut self, type_: crate::classes::code_edit::CodeCompletionKind, display_text: impl AsArg < GString >, insert_text: impl AsArg < GString >,) {
            self.add_code_completion_option_ex(type_, display_text, insert_text,) . done()
        }
        #[inline]
        pub fn add_code_completion_option_ex < 'a > (&'a mut self, type_: crate::classes::code_edit::CodeCompletionKind, display_text: impl AsArg < GString > + 'a, insert_text: impl AsArg < GString > + 'a,) -> ExAddCodeCompletionOption < 'a > {
            ExAddCodeCompletionOption::new(self, type_, display_text, insert_text,)
        }
        pub fn update_code_completion_options(&mut self, force: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "update_code_completion_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_code_completion_options(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_completion_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_code_completion_option(&self, index: i32,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_completion_option", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_code_completion_selected_index(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_completion_selected_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_code_completion_selected_index(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_completion_selected_index", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn confirm_code_completion_full(&mut self, replace: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (replace,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "confirm_code_completion", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::confirm_code_completion_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn confirm_code_completion(&mut self,) {
            self.confirm_code_completion_ex() . done()
        }
        #[inline]
        pub fn confirm_code_completion_ex < 'a > (&'a mut self,) -> ExConfirmCodeCompletion < 'a > {
            ExConfirmCodeCompletion::new(self,)
        }
        pub fn cancel_code_completion(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "cancel_code_completion", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_code_completion_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_completion_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_code_completion_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_code_completion_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_code_completion_prefixes(&mut self, prefixes: &Array < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < GString > >,);
            let args = (RefArg::new(prefixes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_code_completion_prefixes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_code_completion_prefixes(&self,) -> Array < GString > {
            type CallRet = Array < GString >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_code_completion_prefixes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_length_guidelines(&mut self, guideline_columns: &Array < i64 >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < i64 > >,);
            let args = (RefArg::new(guideline_columns),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_line_length_guidelines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_length_guidelines(&self,) -> Array < i64 > {
            type CallRet = Array < i64 >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_line_length_guidelines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_symbol_lookup_on_click_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_symbol_lookup_on_click_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_symbol_lookup_on_click_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_symbol_lookup_on_click_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_for_symbol_lookup(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_text_for_symbol_lookup", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_with_cursor_char(&self, line: i32, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32, i32,);
            let args = (line, column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "get_text_with_cursor_char", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_symbol_lookup_word_as_valid(&mut self, valid: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (valid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_symbol_lookup_word_as_valid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_symbol_tooltip_on_hover_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "set_symbol_tooltip_on_hover_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_symbol_tooltip_on_hover_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "is_symbol_tooltip_on_hover_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn move_lines_up(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "move_lines_up", Some(self.__validated_obj()), args,)
            }
        }
        pub fn move_lines_down(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "move_lines_down", Some(self.__validated_obj()), args,)
            }
        }
        pub fn delete_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "delete_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn duplicate_selection(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "duplicate_selection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn duplicate_lines(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeEdit", "duplicate_lines", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CodeEdit {
        type Base = crate::classes::TextEdit;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CodeEdit"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CodeEdit {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::TextEdit > for CodeEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for CodeEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for CodeEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CodeEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CodeEdit {
        
    }
    impl crate::obj::cap::GodotDefault for CodeEdit {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CodeEdit {
        type Target = crate::classes::TextEdit;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CodeEdit {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CodeEdit`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CodeEdit__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CodeEdit > for $Class {
                
            }
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
#[doc = "Default-param extender for [`CodeEdit::convert_indent_ex`][super::CodeEdit::convert_indent_ex]."]
#[must_use]
pub struct ExConvertIndent < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CodeEdit, from_line: i32, to_line: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConvertIndent < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit,) -> Self {
        let from_line = - 1i32;
        let to_line = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_line: from_line, to_line: to_line,
        }
    }
    #[inline]
    pub fn from_line(self, from_line: i32) -> Self {
        Self {
            from_line: from_line, .. self
        }
    }
    #[inline]
    pub fn to_line(self, to_line: i32) -> Self {
        Self {
            to_line: to_line, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from_line, to_line,
        }
        = self;
        re_export::CodeEdit::convert_indent_full(surround_object, from_line, to_line,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::set_code_region_tags_ex`][super::CodeEdit::set_code_region_tags_ex]."]
#[must_use]
pub struct ExSetCodeRegionTags < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CodeEdit, start: CowArg < 'a, GString >, end: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCodeRegionTags < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit,) -> Self {
        let start = GString::from("region");
        let end = GString::from("endregion");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, start: CowArg::Owned(start), end: CowArg::Owned(end),
        }
    }
    #[inline]
    pub fn start(self, start: impl AsArg < GString > + 'a) -> Self {
        Self {
            start: start.into_arg(), .. self
        }
    }
    #[inline]
    pub fn end(self, end: impl AsArg < GString > + 'a) -> Self {
        Self {
            end: end.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, start, end,
        }
        = self;
        re_export::CodeEdit::set_code_region_tags_full(surround_object, start, end,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::add_string_delimiter_ex`][super::CodeEdit::add_string_delimiter_ex]."]
#[must_use]
pub struct ExAddStringDelimiter < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CodeEdit, start_key: CowArg < 'a, GString >, end_key: CowArg < 'a, GString >, line_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddStringDelimiter < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit, start_key: impl AsArg < GString > + 'a, end_key: impl AsArg < GString > + 'a,) -> Self {
        let line_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, start_key: start_key.into_arg(), end_key: end_key.into_arg(), line_only: line_only,
        }
    }
    #[inline]
    pub fn line_only(self, line_only: bool) -> Self {
        Self {
            line_only: line_only, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, start_key, end_key, line_only,
        }
        = self;
        re_export::CodeEdit::add_string_delimiter_full(surround_object, start_key, end_key, line_only,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::is_in_string_ex`][super::CodeEdit::is_in_string_ex]."]
#[must_use]
pub struct ExIsInString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::CodeEdit, line: i32, column: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsInString < 'a > {
    fn new(surround_object: &'a re_export::CodeEdit, line: i32,) -> Self {
        let column = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, column: column,
        }
    }
    #[inline]
    pub fn column(self, column: i32) -> Self {
        Self {
            column: column, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, line, column,
        }
        = self;
        re_export::CodeEdit::is_in_string_full(surround_object, line, column,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::add_comment_delimiter_ex`][super::CodeEdit::add_comment_delimiter_ex]."]
#[must_use]
pub struct ExAddCommentDelimiter < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CodeEdit, start_key: CowArg < 'a, GString >, end_key: CowArg < 'a, GString >, line_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCommentDelimiter < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit, start_key: impl AsArg < GString > + 'a, end_key: impl AsArg < GString > + 'a,) -> Self {
        let line_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, start_key: start_key.into_arg(), end_key: end_key.into_arg(), line_only: line_only,
        }
    }
    #[inline]
    pub fn line_only(self, line_only: bool) -> Self {
        Self {
            line_only: line_only, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, start_key, end_key, line_only,
        }
        = self;
        re_export::CodeEdit::add_comment_delimiter_full(surround_object, start_key, end_key, line_only,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::is_in_comment_ex`][super::CodeEdit::is_in_comment_ex]."]
#[must_use]
pub struct ExIsInComment < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::CodeEdit, line: i32, column: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsInComment < 'a > {
    fn new(surround_object: &'a re_export::CodeEdit, line: i32,) -> Self {
        let column = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line, column: column,
        }
    }
    #[inline]
    pub fn column(self, column: i32) -> Self {
        Self {
            column: column, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, line, column,
        }
        = self;
        re_export::CodeEdit::is_in_comment_full(surround_object, line, column,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::request_code_completion_ex`][super::CodeEdit::request_code_completion_ex]."]
#[must_use]
pub struct ExRequestCodeCompletion < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CodeEdit, force: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRequestCodeCompletion < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit,) -> Self {
        let force = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force,
        }
    }
    #[inline]
    pub fn force(self, force: bool) -> Self {
        Self {
            force: force, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force,
        }
        = self;
        re_export::CodeEdit::request_code_completion_full(surround_object, force,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::add_code_completion_option_ex`][super::CodeEdit::add_code_completion_option_ex]."]
#[must_use]
pub struct ExAddCodeCompletionOption < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CodeEdit, type_: crate::classes::code_edit::CodeCompletionKind, display_text: CowArg < 'a, GString >, insert_text: CowArg < 'a, GString >, text_color: Color, icon: CowArg < 'a, Option < Gd < crate::classes::Resource > > >, value: CowArg < 'a, Variant >, location: crate::classes::code_edit::CodeCompletionLocation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCodeCompletionOption < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit, type_: crate::classes::code_edit::CodeCompletionKind, display_text: impl AsArg < GString > + 'a, insert_text: impl AsArg < GString > + 'a,) -> Self {
        let text_color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let icon = Gd::null_arg();
        let value = Variant::nil();
        let location = crate::obj::EngineEnum::from_ord(1024);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, type_: type_, display_text: display_text.into_arg(), insert_text: insert_text.into_arg(), text_color: text_color, icon: icon.into_arg(), value: CowArg::Owned(value), location: location,
        }
    }
    #[inline]
    pub fn text_color(self, text_color: Color) -> Self {
        Self {
            text_color: text_color, .. self
        }
    }
    #[inline]
    pub fn icon(self, icon: impl AsArg < Option < Gd < crate::classes::Resource >> > + 'a) -> Self {
        Self {
            icon: icon.into_arg(), .. self
        }
    }
    #[inline]
    pub fn value(self, value: &'a Variant) -> Self {
        Self {
            value: CowArg::Borrowed(value), .. self
        }
    }
    #[inline]
    pub fn location(self, location: crate::classes::code_edit::CodeCompletionLocation) -> Self {
        Self {
            location: location, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, type_, display_text, insert_text, text_color, icon, value, location,
        }
        = self;
        re_export::CodeEdit::add_code_completion_option_full(surround_object, type_, display_text, insert_text, text_color, icon, value.cow_as_arg(), location,)
    }
}
#[doc = "Default-param extender for [`CodeEdit::confirm_code_completion_ex`][super::CodeEdit::confirm_code_completion_ex]."]
#[must_use]
pub struct ExConfirmCodeCompletion < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CodeEdit, replace: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConfirmCodeCompletion < 'a > {
    fn new(surround_object: &'a mut re_export::CodeEdit,) -> Self {
        let replace = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, replace: replace,
        }
    }
    #[inline]
    pub fn replace(self, replace: bool) -> Self {
        Self {
            replace: replace, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, replace,
        }
        = self;
        re_export::CodeEdit::confirm_code_completion_full(surround_object, replace,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CodeCompletionKind {
    ord: i32
}
impl CodeCompletionKind {
    #[doc(alias = "KIND_CLASS")]
    #[doc = "Godot enumerator name: `KIND_CLASS`"]
    pub const CLASS: CodeCompletionKind = CodeCompletionKind {
        ord: 0i32
    };
    #[doc(alias = "KIND_FUNCTION")]
    #[doc = "Godot enumerator name: `KIND_FUNCTION`"]
    pub const FUNCTION: CodeCompletionKind = CodeCompletionKind {
        ord: 1i32
    };
    #[doc(alias = "KIND_SIGNAL")]
    #[doc = "Godot enumerator name: `KIND_SIGNAL`"]
    pub const SIGNAL: CodeCompletionKind = CodeCompletionKind {
        ord: 2i32
    };
    #[doc(alias = "KIND_VARIABLE")]
    #[doc = "Godot enumerator name: `KIND_VARIABLE`"]
    pub const VARIABLE: CodeCompletionKind = CodeCompletionKind {
        ord: 3i32
    };
    #[doc(alias = "KIND_MEMBER")]
    #[doc = "Godot enumerator name: `KIND_MEMBER`"]
    pub const MEMBER: CodeCompletionKind = CodeCompletionKind {
        ord: 4i32
    };
    #[doc(alias = "KIND_ENUM")]
    #[doc = "Godot enumerator name: `KIND_ENUM`"]
    pub const ENUM: CodeCompletionKind = CodeCompletionKind {
        ord: 5i32
    };
    #[doc(alias = "KIND_CONSTANT")]
    #[doc = "Godot enumerator name: `KIND_CONSTANT`"]
    pub const CONSTANT: CodeCompletionKind = CodeCompletionKind {
        ord: 6i32
    };
    #[doc(alias = "KIND_NODE_PATH")]
    #[doc = "Godot enumerator name: `KIND_NODE_PATH`"]
    pub const NODE_PATH: CodeCompletionKind = CodeCompletionKind {
        ord: 7i32
    };
    #[doc(alias = "KIND_FILE_PATH")]
    #[doc = "Godot enumerator name: `KIND_FILE_PATH`"]
    pub const FILE_PATH: CodeCompletionKind = CodeCompletionKind {
        ord: 8i32
    };
    #[doc(alias = "KIND_PLAIN_TEXT")]
    #[doc = "Godot enumerator name: `KIND_PLAIN_TEXT`"]
    pub const PLAIN_TEXT: CodeCompletionKind = CodeCompletionKind {
        ord: 9i32
    };
    
}
impl std::fmt::Debug for CodeCompletionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CodeCompletionKind") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CodeCompletionKind {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
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
            Self::CLASS => "CLASS", Self::FUNCTION => "FUNCTION", Self::SIGNAL => "SIGNAL", Self::VARIABLE => "VARIABLE", Self::MEMBER => "MEMBER", Self::ENUM => "ENUM", Self::CONSTANT => "CONSTANT", Self::NODE_PATH => "NODE_PATH", Self::FILE_PATH => "FILE_PATH", Self::PLAIN_TEXT => "PLAIN_TEXT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CodeCompletionKind::CLASS, CodeCompletionKind::FUNCTION, CodeCompletionKind::SIGNAL, CodeCompletionKind::VARIABLE, CodeCompletionKind::MEMBER, CodeCompletionKind::ENUM, CodeCompletionKind::CONSTANT, CodeCompletionKind::NODE_PATH, CodeCompletionKind::FILE_PATH, CodeCompletionKind::PLAIN_TEXT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CodeCompletionKind >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CLASS", "KIND_CLASS", CodeCompletionKind::CLASS), crate::meta::inspect::EnumConstant::new("FUNCTION", "KIND_FUNCTION", CodeCompletionKind::FUNCTION), crate::meta::inspect::EnumConstant::new("SIGNAL", "KIND_SIGNAL", CodeCompletionKind::SIGNAL), crate::meta::inspect::EnumConstant::new("VARIABLE", "KIND_VARIABLE", CodeCompletionKind::VARIABLE), crate::meta::inspect::EnumConstant::new("MEMBER", "KIND_MEMBER", CodeCompletionKind::MEMBER), crate::meta::inspect::EnumConstant::new("ENUM", "KIND_ENUM", CodeCompletionKind::ENUM), crate::meta::inspect::EnumConstant::new("CONSTANT", "KIND_CONSTANT", CodeCompletionKind::CONSTANT), crate::meta::inspect::EnumConstant::new("NODE_PATH", "KIND_NODE_PATH", CodeCompletionKind::NODE_PATH), crate::meta::inspect::EnumConstant::new("FILE_PATH", "KIND_FILE_PATH", CodeCompletionKind::FILE_PATH), crate::meta::inspect::EnumConstant::new("PLAIN_TEXT", "KIND_PLAIN_TEXT", CodeCompletionKind::PLAIN_TEXT)]
        }
    }
}
impl crate::meta::GodotConvert for CodeCompletionKind {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CodeCompletionKind {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CodeCompletionKind {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CodeCompletionLocation {
    ord: i32
}
impl CodeCompletionLocation {
    #[doc(alias = "LOCATION_LOCAL")]
    #[doc = "Godot enumerator name: `LOCATION_LOCAL`"]
    pub const LOCAL: CodeCompletionLocation = CodeCompletionLocation {
        ord: 0i32
    };
    #[doc(alias = "LOCATION_PARENT_MASK")]
    #[doc = "Godot enumerator name: `LOCATION_PARENT_MASK`"]
    pub const PARENT_MASK: CodeCompletionLocation = CodeCompletionLocation {
        ord: 256i32
    };
    #[doc(alias = "LOCATION_OTHER_USER_CODE")]
    #[doc = "Godot enumerator name: `LOCATION_OTHER_USER_CODE`"]
    pub const OTHER_USER_CODE: CodeCompletionLocation = CodeCompletionLocation {
        ord: 512i32
    };
    #[doc(alias = "LOCATION_OTHER")]
    #[doc = "Godot enumerator name: `LOCATION_OTHER`"]
    pub const OTHER: CodeCompletionLocation = CodeCompletionLocation {
        ord: 1024i32
    };
    
}
impl std::fmt::Debug for CodeCompletionLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CodeCompletionLocation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CodeCompletionLocation {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 256i32 | ord @ 512i32 | ord @ 1024i32 => Some(Self {
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
            Self::LOCAL => "LOCAL", Self::PARENT_MASK => "PARENT_MASK", Self::OTHER_USER_CODE => "OTHER_USER_CODE", Self::OTHER => "OTHER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CodeCompletionLocation::LOCAL, CodeCompletionLocation::PARENT_MASK, CodeCompletionLocation::OTHER_USER_CODE, CodeCompletionLocation::OTHER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CodeCompletionLocation >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LOCAL", "LOCATION_LOCAL", CodeCompletionLocation::LOCAL), crate::meta::inspect::EnumConstant::new("PARENT_MASK", "LOCATION_PARENT_MASK", CodeCompletionLocation::PARENT_MASK), crate::meta::inspect::EnumConstant::new("OTHER_USER_CODE", "LOCATION_OTHER_USER_CODE", CodeCompletionLocation::OTHER_USER_CODE), crate::meta::inspect::EnumConstant::new("OTHER", "LOCATION_OTHER", CodeCompletionLocation::OTHER)]
        }
    }
}
impl crate::meta::GodotConvert for CodeCompletionLocation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CodeCompletionLocation {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CodeCompletionLocation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CodeEdit;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`CodeEdit`][crate::classes::CodeEdit] class."]
    pub struct SignalsOfCodeEdit < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfCodeEdit < 'c, C > {
        #[doc = "Signature: `(line: i64)`"]
        pub fn breakpoint_toggled(&mut self) -> SigBreakpointToggled < 'c, C > {
            SigBreakpointToggled {
                typed: TypedSignal::extract(&mut self.__internal_obj, "breakpoint_toggled")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn code_completion_requested(&mut self) -> SigCodeCompletionRequested < 'c, C > {
            SigCodeCompletionRequested {
                typed: TypedSignal::extract(&mut self.__internal_obj, "code_completion_requested")
            }
        }
        #[doc = "Signature: `(symbol: GString, line: i64, column: i64)`"]
        pub fn symbol_lookup(&mut self) -> SigSymbolLookup < 'c, C > {
            SigSymbolLookup {
                typed: TypedSignal::extract(&mut self.__internal_obj, "symbol_lookup")
            }
        }
        #[doc = "Signature: `(symbol: GString)`"]
        pub fn symbol_validate(&mut self) -> SigSymbolValidate < 'c, C > {
            SigSymbolValidate {
                typed: TypedSignal::extract(&mut self.__internal_obj, "symbol_validate")
            }
        }
        #[doc = "Signature: `(symbol: GString, line: i64, column: i64)`"]
        pub fn symbol_hovered(&mut self) -> SigSymbolHovered < 'c, C > {
            SigSymbolHovered {
                typed: TypedSignal::extract(&mut self.__internal_obj, "symbol_hovered")
            }
        }
    }
    type TypedSigBreakpointToggled < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigBreakpointToggled < 'c, C: WithSignals > {
        typed: TypedSigBreakpointToggled < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBreakpointToggled < 'c, C > {
        pub fn emit(&mut self, line: i64,) {
            self.typed.emit_tuple((line,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigBreakpointToggled < 'c, C > {
        type Target = TypedSigBreakpointToggled < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigBreakpointToggled < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCodeCompletionRequested < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigCodeCompletionRequested < 'c, C: WithSignals > {
        typed: TypedSigCodeCompletionRequested < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCodeCompletionRequested < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCodeCompletionRequested < 'c, C > {
        type Target = TypedSigCodeCompletionRequested < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCodeCompletionRequested < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSymbolLookup < 'c, C > = TypedSignal < 'c, C, (GString, i64, i64,) >;
    pub struct SigSymbolLookup < 'c, C: WithSignals > {
        typed: TypedSigSymbolLookup < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSymbolLookup < 'c, C > {
        pub fn emit(&mut self, symbol: GString, line: i64, column: i64,) {
            self.typed.emit_tuple((symbol, line, column,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSymbolLookup < 'c, C > {
        type Target = TypedSigSymbolLookup < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSymbolLookup < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSymbolValidate < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigSymbolValidate < 'c, C: WithSignals > {
        typed: TypedSigSymbolValidate < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSymbolValidate < 'c, C > {
        pub fn emit(&mut self, symbol: GString,) {
            self.typed.emit_tuple((symbol,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSymbolValidate < 'c, C > {
        type Target = TypedSigSymbolValidate < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSymbolValidate < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSymbolHovered < 'c, C > = TypedSignal < 'c, C, (GString, i64, i64,) >;
    pub struct SigSymbolHovered < 'c, C: WithSignals > {
        typed: TypedSigSymbolHovered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSymbolHovered < 'c, C > {
        pub fn emit(&mut self, symbol: GString, line: i64, column: i64,) {
            self.typed.emit_tuple((symbol, line, column,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSymbolHovered < 'c, C > {
        type Target = TypedSigSymbolHovered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSymbolHovered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for CodeEdit {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCodeEdit < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfCodeEdit < 'c, C > {
        type Target = < < CodeEdit as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = CodeEdit;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfCodeEdit < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = CodeEdit;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}