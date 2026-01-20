#![doc = "Sidecar module for class [`Label`][crate::classes::Label].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Label` enums](https://docs.godotengine.org/en/stable/classes/class_label.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Label.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`label`][crate::classes::label]: sidecar module with related enum/flag types\n* [`ILabel`][crate::classes::ILabel]: virtual methods\n\n\nSee also [Godot docs for `Label`](https://docs.godotengine.org/en/stable/classes/class_label.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Label::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Label {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Label`][crate::classes::Label].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Label` methods](https://docs.godotengine.org/en/stable/classes/class_label.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILabel: crate::obj::GodotClass < Base = Label > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Label {
        pub fn set_horizontal_alignment(&mut self, alignment: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::HorizontalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4741usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_horizontal_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallRet = crate::global::HorizontalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4742usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertical_alignment(&mut self, alignment: crate::global::VerticalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::VerticalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4743usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_vertical_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertical_alignment(&self,) -> crate::global::VerticalAlignment {
            type CallRet = crate::global::VerticalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4744usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_vertical_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4745usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4746usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_label_settings(&mut self, settings: impl AsArg < Option < Gd < crate::classes::LabelSettings >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::LabelSettings > > >,);
            let args = (settings.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4747usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_label_settings", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_label_settings(&self,) -> Option < Gd < crate::classes::LabelSettings > > {
            type CallRet = Option < Gd < crate::classes::LabelSettings > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4748usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_label_settings", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::TextDirection,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4749usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4750usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4751usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4752usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_paragraph_separator(&mut self, paragraph_separator: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (paragraph_separator.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4753usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_paragraph_separator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_paragraph_separator(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4754usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_paragraph_separator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, autowrap_mode: crate::classes::text_server::AutowrapMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::AutowrapMode,);
            let args = (autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4755usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autowrap_mode(&self,) -> crate::classes::text_server::AutowrapMode {
            type CallRet = crate::classes::text_server::AutowrapMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4756usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autowrap_trim_flags(&mut self, autowrap_trim_flags: crate::classes::text_server::LineBreakFlag,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::LineBreakFlag,);
            let args = (autowrap_trim_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4757usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_autowrap_trim_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autowrap_trim_flags(&self,) -> crate::classes::text_server::LineBreakFlag {
            type CallRet = crate::classes::text_server::LineBreakFlag;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4758usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_autowrap_trim_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_justification_flags(&mut self, justification_flags: crate::classes::text_server::JustificationFlag,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::JustificationFlag,);
            let args = (justification_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4759usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_justification_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_justification_flags(&self,) -> crate::classes::text_server::JustificationFlag {
            type CallRet = crate::classes::text_server::JustificationFlag;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4760usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_justification_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_clip_text(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4761usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_clip_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_clipping_text(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4762usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "is_clipping_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_stops(&mut self, tab_stops: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedFloat32Array >,);
            let args = (RefArg::new(tab_stops),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4763usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_tab_stops", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_stops(&self,) -> PackedFloat32Array {
            type CallRet = PackedFloat32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4764usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_tab_stops", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_overrun_behavior(&mut self, overrun_behavior: crate::classes::text_server::OverrunBehavior,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::OverrunBehavior,);
            let args = (overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4765usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_overrun_behavior(&self,) -> crate::classes::text_server::OverrunBehavior {
            type CallRet = crate::classes::text_server::OverrunBehavior;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4766usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ellipsis_char(&mut self, char: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (char.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4767usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_ellipsis_char", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ellipsis_char(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4768usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_ellipsis_char", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_uppercase(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4769usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_uppercase", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_uppercase(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4770usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "is_uppercase", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_line_height_full(&self, line: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4771usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_line_height", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_line_height_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_line_height(&self,) -> i32 {
            self.get_line_height_ex() . done()
        }
        #[inline]
        pub fn get_line_height_ex < 'a > (&'a self,) -> ExGetLineHeight < 'a > {
            ExGetLineHeight::new(self,)
        }
        pub fn get_line_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4772usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_line_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_line_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4773usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_visible_line_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_total_character_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4774usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_total_character_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible_characters(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4775usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_visible_characters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_characters(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4776usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_visible_characters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_characters_behavior(&self,) -> crate::classes::text_server::VisibleCharactersBehavior {
            type CallRet = crate::classes::text_server::VisibleCharactersBehavior;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4777usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_visible_characters_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible_characters_behavior(&mut self, behavior: crate::classes::text_server::VisibleCharactersBehavior,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::VisibleCharactersBehavior,);
            let args = (behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4778usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_visible_characters_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible_ratio(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4779usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_visible_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_ratio(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4780usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_visible_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lines_skipped(&mut self, lines_skipped: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (lines_skipped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4781usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_lines_skipped", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lines_skipped(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4782usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_lines_skipped", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_lines_visible(&mut self, lines_visible: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (lines_visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4783usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_max_lines_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_lines_visible(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4784usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_max_lines_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::classes::text_server::StructuredTextParser,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::StructuredTextParser,);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4785usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::classes::text_server::StructuredTextParser {
            type CallRet = crate::classes::text_server::StructuredTextParser;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4786usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: &VarArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
            let args = (RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4787usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "set_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4788usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_character_bounds(&self, pos: i32,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (i32,);
            let args = (pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4789usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label", "get_character_bounds", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Label {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Label"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Label {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for Label {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Label {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Label {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Label {
        
    }
    impl crate::obj::cap::GodotDefault for Label {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Label {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Label {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Label`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Label__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Label > for $Class {
                
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
#[doc = "Default-param extender for [`Label::get_line_height_ex`][super::Label::get_line_height_ex]."]
#[must_use]
pub struct ExGetLineHeight < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Label, line: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetLineHeight < 'a > {
    fn new(surround_object: &'a re_export::Label,) -> Self {
        let line = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, line: line,
        }
    }
    #[inline]
    pub fn line(self, line: i32) -> Self {
        Self {
            line: line, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, line,
        }
        = self;
        re_export::Label::get_line_height_full(surround_object, line,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Label;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::control::SignalsOfControl;
    impl WithSignals for Label {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfControl < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}