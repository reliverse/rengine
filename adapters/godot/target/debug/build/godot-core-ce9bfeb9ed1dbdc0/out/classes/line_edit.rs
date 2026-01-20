#![doc = "Sidecar module for class [`LineEdit`][crate::classes::LineEdit].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `LineEdit` enums](https://docs.godotengine.org/en/stable/classes/class_lineedit.html#enumerations).\n\n"]
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
    #[doc = "Godot class `LineEdit.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`line_edit`][crate::classes::line_edit]: sidecar module with related enum/flag types\n* [`ILineEdit`][crate::classes::ILineEdit]: virtual methods\n* [`SignalsOfLineEdit`][crate::classes::line_edit::SignalsOfLineEdit]: signal collection\n\n\nSee also [Godot docs for `LineEdit`](https://docs.godotengine.org/en/stable/classes/class_lineedit.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`LineEdit::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct LineEdit {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`LineEdit`][crate::classes::LineEdit].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `LineEdit` methods](https://docs.godotengine.org/en/stable/classes/class_lineedit.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILineEdit: crate::obj::GodotClass < Base = LineEdit > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl LineEdit {
        pub fn has_ime_text(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5050usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "has_ime_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn cancel_ime(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5051usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "cancel_ime", Some(self.__validated_obj()), args,)
            }
        }
        pub fn apply_ime(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5052usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "apply_ime", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_horizontal_alignment(&mut self, alignment: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::HorizontalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5053usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_horizontal_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallRet = crate::global::HorizontalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5054usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn edit(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5055usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "edit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn unedit(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5056usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "unedit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_editing(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5057usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_editing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_keep_editing_on_text_submit(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5058usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_keep_editing_on_text_submit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_editing_kept_on_text_submit(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5059usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_editing_kept_on_text_submit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5060usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn select_full(&mut self, from: i32, to: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5061usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "select", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::select_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn select(&mut self,) {
            self.select_ex() . done()
        }
        #[inline]
        pub fn select_ex < 'a > (&'a mut self,) -> ExSelect < 'a > {
            ExSelect::new(self,)
        }
        pub fn select_all(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5062usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "select_all", Some(self.__validated_obj()), args,)
            }
        }
        pub fn deselect(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5063usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "deselect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_undo(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5064usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "has_undo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_redo(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5065usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "has_redo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_selection(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5066usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "has_selection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selected_text(&mut self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5067usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_selected_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selection_from_column(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5068usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_selection_from_column", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selection_to_column(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5069usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_selection_to_column", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5070usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5071usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_draw_control_chars(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5072usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_draw_control_chars", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_control_chars(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5073usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_draw_control_chars", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::TextDirection,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5074usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5075usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5076usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5077usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::classes::text_server::StructuredTextParser,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::StructuredTextParser,);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5078usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::classes::text_server::StructuredTextParser {
            type CallRet = crate::classes::text_server::StructuredTextParser;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5079usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: &VarArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
            let args = (RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5080usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5081usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_placeholder(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5082usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_placeholder(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5083usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_caret_column(&mut self, position: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5084usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_caret_column", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_caret_column(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5085usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_caret_column", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_next_composite_character_column(&self, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5086usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_next_composite_character_column", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_previous_composite_character_column(&self, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5087usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_previous_composite_character_column", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scroll_offset(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5088usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_scroll_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_expand_to_text_length_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5089usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_expand_to_text_length_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_expand_to_text_length_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5090usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_expand_to_text_length_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_caret_blink_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5091usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_caret_blink_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_caret_blink_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5092usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_caret_blink_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_caret_mid_grapheme_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5093usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_caret_mid_grapheme_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_caret_mid_grapheme_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5094usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_caret_mid_grapheme_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_caret_force_displayed(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5095usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_caret_force_displayed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_caret_force_displayed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5096usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_caret_force_displayed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_caret_blink_interval(&mut self, interval: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (interval,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5097usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_caret_blink_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_caret_blink_interval(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5098usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_caret_blink_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_length(&mut self, chars: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (chars,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5099usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_max_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_length(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_max_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn insert_text_at_caret(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5101usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "insert_text_at_caret", Some(self.__validated_obj()), args,)
            }
        }
        pub fn delete_char_at_caret(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5102usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "delete_char_at_caret", Some(self.__validated_obj()), args,)
            }
        }
        pub fn delete_text(&mut self, from_column: i32, to_column: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from_column, to_column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "delete_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_editable(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_editable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_editable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_editable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_secret(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_secret", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_secret(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_secret", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_secret_character(&mut self, character: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (character.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_secret_character", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_secret_character(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_secret_character", Some(self.__validated_obj()), args,)
            }
        }
        pub fn menu_option(&mut self, option: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "menu_option", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_menu(&self,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallRet = Option < Gd < crate::classes::PopupMenu > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_menu_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_menu_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_context_menu_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_context_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_context_menu_enabled(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_context_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emoji_menu_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_emoji_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_emoji_menu_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_emoji_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_backspace_deletes_composite_character_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_backspace_deletes_composite_character_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_backspace_deletes_composite_character_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_backspace_deletes_composite_character_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_virtual_keyboard_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_virtual_keyboard_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_virtual_keyboard_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_virtual_keyboard_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_virtual_keyboard_show_on_focus(&mut self, show_on_focus: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (show_on_focus,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_virtual_keyboard_show_on_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_virtual_keyboard_show_on_focus(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_virtual_keyboard_show_on_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_virtual_keyboard_type(&mut self, type_: crate::classes::line_edit::VirtualKeyboardType,) {
            type CallRet = ();
            type CallParams = (crate::classes::line_edit::VirtualKeyboardType,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_virtual_keyboard_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_virtual_keyboard_type(&self,) -> crate::classes::line_edit::VirtualKeyboardType {
            type CallRet = crate::classes::line_edit::VirtualKeyboardType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_virtual_keyboard_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_clear_button_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_clear_button_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_clear_button_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_clear_button_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shortcut_keys_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_shortcut_keys_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_shortcut_keys_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_shortcut_keys_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_middle_mouse_paste_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_middle_mouse_paste_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_middle_mouse_paste_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_middle_mouse_paste_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_selecting_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_selecting_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_selecting_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_selecting_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_deselect_on_focus_loss_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_deselect_on_focus_loss_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_deselect_on_focus_loss_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_deselect_on_focus_loss_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_and_drop_selection_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_drag_and_drop_selection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drag_and_drop_selection_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_drag_and_drop_selection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_right_icon(&mut self, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_right_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_right_icon(&mut self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "get_right_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flat(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_flat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_flat(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_flat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_select_all_on_focus(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "set_select_all_on_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_select_all_on_focus(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LineEdit", "is_select_all_on_focus", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for LineEdit {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("LineEdit"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for LineEdit {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for LineEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for LineEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for LineEdit {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for LineEdit {
        
    }
    impl crate::obj::cap::GodotDefault for LineEdit {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for LineEdit {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for LineEdit {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`LineEdit`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_LineEdit__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::LineEdit > for $Class {
                
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
#[doc = "Default-param extender for [`LineEdit::select_ex`][super::LineEdit::select_ex]."]
#[must_use]
pub struct ExSelect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::LineEdit, from: i32, to: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSelect < 'a > {
    fn new(surround_object: &'a mut re_export::LineEdit,) -> Self {
        let from = 0i32;
        let to = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from, to: to,
        }
    }
    #[inline]
    pub fn from(self, from: i32) -> Self {
        Self {
            from: from, .. self
        }
    }
    #[inline]
    pub fn to(self, to: i32) -> Self {
        Self {
            to: to, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from, to,
        }
        = self;
        re_export::LineEdit::select_full(surround_object, from, to,)
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
pub struct VirtualKeyboardType {
    ord: i32
}
impl VirtualKeyboardType {
    #[doc(alias = "KEYBOARD_TYPE_DEFAULT")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_DEFAULT`"]
    pub const DEFAULT: VirtualKeyboardType = VirtualKeyboardType {
        ord: 0i32
    };
    #[doc(alias = "KEYBOARD_TYPE_MULTILINE")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_MULTILINE`"]
    pub const MULTILINE: VirtualKeyboardType = VirtualKeyboardType {
        ord: 1i32
    };
    #[doc(alias = "KEYBOARD_TYPE_NUMBER")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_NUMBER`"]
    pub const NUMBER: VirtualKeyboardType = VirtualKeyboardType {
        ord: 2i32
    };
    #[doc(alias = "KEYBOARD_TYPE_NUMBER_DECIMAL")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_NUMBER_DECIMAL`"]
    pub const NUMBER_DECIMAL: VirtualKeyboardType = VirtualKeyboardType {
        ord: 3i32
    };
    #[doc(alias = "KEYBOARD_TYPE_PHONE")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_PHONE`"]
    pub const PHONE: VirtualKeyboardType = VirtualKeyboardType {
        ord: 4i32
    };
    #[doc(alias = "KEYBOARD_TYPE_EMAIL_ADDRESS")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_EMAIL_ADDRESS`"]
    pub const EMAIL_ADDRESS: VirtualKeyboardType = VirtualKeyboardType {
        ord: 5i32
    };
    #[doc(alias = "KEYBOARD_TYPE_PASSWORD")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_PASSWORD`"]
    pub const PASSWORD: VirtualKeyboardType = VirtualKeyboardType {
        ord: 6i32
    };
    #[doc(alias = "KEYBOARD_TYPE_URL")]
    #[doc = "Godot enumerator name: `KEYBOARD_TYPE_URL`"]
    pub const URL: VirtualKeyboardType = VirtualKeyboardType {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for VirtualKeyboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VirtualKeyboardType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VirtualKeyboardType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
            Self::DEFAULT => "DEFAULT", Self::MULTILINE => "MULTILINE", Self::NUMBER => "NUMBER", Self::NUMBER_DECIMAL => "NUMBER_DECIMAL", Self::PHONE => "PHONE", Self::EMAIL_ADDRESS => "EMAIL_ADDRESS", Self::PASSWORD => "PASSWORD", Self::URL => "URL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VirtualKeyboardType::DEFAULT, VirtualKeyboardType::MULTILINE, VirtualKeyboardType::NUMBER, VirtualKeyboardType::NUMBER_DECIMAL, VirtualKeyboardType::PHONE, VirtualKeyboardType::EMAIL_ADDRESS, VirtualKeyboardType::PASSWORD, VirtualKeyboardType::URL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VirtualKeyboardType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "KEYBOARD_TYPE_DEFAULT", VirtualKeyboardType::DEFAULT), crate::meta::inspect::EnumConstant::new("MULTILINE", "KEYBOARD_TYPE_MULTILINE", VirtualKeyboardType::MULTILINE), crate::meta::inspect::EnumConstant::new("NUMBER", "KEYBOARD_TYPE_NUMBER", VirtualKeyboardType::NUMBER), crate::meta::inspect::EnumConstant::new("NUMBER_DECIMAL", "KEYBOARD_TYPE_NUMBER_DECIMAL", VirtualKeyboardType::NUMBER_DECIMAL), crate::meta::inspect::EnumConstant::new("PHONE", "KEYBOARD_TYPE_PHONE", VirtualKeyboardType::PHONE), crate::meta::inspect::EnumConstant::new("EMAIL_ADDRESS", "KEYBOARD_TYPE_EMAIL_ADDRESS", VirtualKeyboardType::EMAIL_ADDRESS), crate::meta::inspect::EnumConstant::new("PASSWORD", "KEYBOARD_TYPE_PASSWORD", VirtualKeyboardType::PASSWORD), crate::meta::inspect::EnumConstant::new("URL", "KEYBOARD_TYPE_URL", VirtualKeyboardType::URL)]
        }
    }
}
impl crate::meta::GodotConvert for VirtualKeyboardType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VirtualKeyboardType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VirtualKeyboardType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::LineEdit;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`LineEdit`][crate::classes::LineEdit] class."]
    pub struct SignalsOfLineEdit < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfLineEdit < 'c, C > {
        #[doc = "Signature: `(new_text: GString)`"]
        pub fn text_changed(&mut self) -> SigTextChanged < 'c, C > {
            SigTextChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "text_changed")
            }
        }
        #[doc = "Signature: `(rejected_substring: GString)`"]
        pub fn text_change_rejected(&mut self) -> SigTextChangeRejected < 'c, C > {
            SigTextChangeRejected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "text_change_rejected")
            }
        }
        #[doc = "Signature: `(new_text: GString)`"]
        pub fn text_submitted(&mut self) -> SigTextSubmitted < 'c, C > {
            SigTextSubmitted {
                typed: TypedSignal::extract(&mut self.__internal_obj, "text_submitted")
            }
        }
        #[doc = "Signature: `(toggled_on: bool)`"]
        pub fn editing_toggled(&mut self) -> SigEditingToggled < 'c, C > {
            SigEditingToggled {
                typed: TypedSignal::extract(&mut self.__internal_obj, "editing_toggled")
            }
        }
    }
    type TypedSigTextChanged < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigTextChanged < 'c, C: WithSignals > {
        typed: TypedSigTextChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTextChanged < 'c, C > {
        pub fn emit(&mut self, new_text: GString,) {
            self.typed.emit_tuple((new_text,));
            
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
    type TypedSigTextChangeRejected < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigTextChangeRejected < 'c, C: WithSignals > {
        typed: TypedSigTextChangeRejected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTextChangeRejected < 'c, C > {
        pub fn emit(&mut self, rejected_substring: GString,) {
            self.typed.emit_tuple((rejected_substring,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTextChangeRejected < 'c, C > {
        type Target = TypedSigTextChangeRejected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTextChangeRejected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTextSubmitted < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigTextSubmitted < 'c, C: WithSignals > {
        typed: TypedSigTextSubmitted < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTextSubmitted < 'c, C > {
        pub fn emit(&mut self, new_text: GString,) {
            self.typed.emit_tuple((new_text,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTextSubmitted < 'c, C > {
        type Target = TypedSigTextSubmitted < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTextSubmitted < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigEditingToggled < 'c, C > = TypedSignal < 'c, C, (bool,) >;
    pub struct SigEditingToggled < 'c, C: WithSignals > {
        typed: TypedSigEditingToggled < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigEditingToggled < 'c, C > {
        pub fn emit(&mut self, toggled_on: bool,) {
            self.typed.emit_tuple((toggled_on,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigEditingToggled < 'c, C > {
        type Target = TypedSigEditingToggled < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigEditingToggled < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for LineEdit {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfLineEdit < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfLineEdit < 'c, C > {
        type Target = < < LineEdit as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = LineEdit;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfLineEdit < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = LineEdit;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}