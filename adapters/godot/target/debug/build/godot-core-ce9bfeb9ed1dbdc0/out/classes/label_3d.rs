#![doc = "Sidecar module for class [`Label3D`][crate::classes::Label3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Label3D` enums](https://docs.godotengine.org/en/stable/classes/class_label3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Label3D.`\n\nInherits [`GeometryInstance3D`][crate::classes::GeometryInstance3D].\n\nRelated symbols:\n\n* [`label_3d`][crate::classes::label_3d]: sidecar module with related enum/flag types\n* [`ILabel3D`][crate::classes::ILabel3D]: virtual methods\n\n\nSee also [Godot docs for `Label3D`](https://docs.godotengine.org/en/stable/classes/class_label3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Label3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Label3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Label3D`][crate::classes::Label3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IGeometryInstance3D`][crate::classes::IGeometryInstance3D] > [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Label3D` methods](https://docs.godotengine.org/en/stable/classes/class_label3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILabel3D: crate::obj::GodotClass < Base = Label3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_aabb(&self,) -> Aabb {
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
    impl Label3D {
        pub fn set_horizontal_alignment(&mut self, alignment: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::HorizontalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4790usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_horizontal_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallRet = crate::global::HorizontalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4791usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertical_alignment(&mut self, alignment: crate::global::VerticalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::VerticalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4792usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_vertical_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertical_alignment(&self,) -> crate::global::VerticalAlignment {
            type CallRet = crate::global::VerticalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4793usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_vertical_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_modulate(&mut self, modulate: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4794usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_modulate(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4795usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_outline_modulate(&mut self, modulate: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4796usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_outline_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_outline_modulate(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4797usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_outline_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4798usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4799usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::classes::text_server::Direction,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::Direction,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4800usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::classes::text_server::Direction {
            type CallRet = crate::classes::text_server::Direction;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4801usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4802usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4803usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::classes::text_server::StructuredTextParser,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::StructuredTextParser,);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4804usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::classes::text_server::StructuredTextParser {
            type CallRet = crate::classes::text_server::StructuredTextParser;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4805usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: &VarArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
            let args = (RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4806usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4807usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_uppercase(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4808usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_uppercase", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_uppercase(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4809usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "is_uppercase", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_render_priority(&mut self, priority: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4810usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_render_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_render_priority(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4811usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_render_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_outline_render_priority(&mut self, priority: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4812usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_outline_render_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_outline_render_priority(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4813usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_outline_render_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font(&mut self, font: impl AsArg < Option < Gd < crate::classes::Font >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Font > > >,);
            let args = (font.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4814usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font(&self,) -> Option < Gd < crate::classes::Font > > {
            type CallRet = Option < Gd < crate::classes::Font > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4815usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4816usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4817usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_outline_size(&mut self, outline_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (outline_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4818usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_outline_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_outline_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4819usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_outline_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_spacing(&mut self, line_spacing: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (line_spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4820usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_line_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_spacing(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4821usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_line_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, autowrap_mode: crate::classes::text_server::AutowrapMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::AutowrapMode,);
            let args = (autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4822usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autowrap_mode(&self,) -> crate::classes::text_server::AutowrapMode {
            type CallRet = crate::classes::text_server::AutowrapMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4823usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autowrap_trim_flags(&mut self, autowrap_trim_flags: crate::classes::text_server::LineBreakFlag,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::LineBreakFlag,);
            let args = (autowrap_trim_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4824usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_autowrap_trim_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autowrap_trim_flags(&self,) -> crate::classes::text_server::LineBreakFlag {
            type CallRet = crate::classes::text_server::LineBreakFlag;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4825usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_autowrap_trim_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_justification_flags(&mut self, justification_flags: crate::classes::text_server::JustificationFlag,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::JustificationFlag,);
            let args = (justification_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4826usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_justification_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_justification_flags(&self,) -> crate::classes::text_server::JustificationFlag {
            type CallRet = crate::classes::text_server::JustificationFlag;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4827usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_justification_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_width(&mut self, width: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4828usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_width(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4829usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pixel_size(&mut self, pixel_size: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (pixel_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4830usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_pixel_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pixel_size(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4831usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_pixel_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4832usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4833usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_flag(&mut self, flag: crate::classes::label_3d::DrawFlags, enabled: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::label_3d::DrawFlags, bool,);
            let args = (flag, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4834usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_draw_flag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_draw_flag(&self, flag: crate::classes::label_3d::DrawFlags,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::label_3d::DrawFlags,);
            let args = (flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4835usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_draw_flag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_billboard_mode(&mut self, mode: crate::classes::base_material_3d::BillboardMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::BillboardMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4836usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_billboard_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_billboard_mode(&self,) -> crate::classes::base_material_3d::BillboardMode {
            type CallRet = crate::classes::base_material_3d::BillboardMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4837usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_billboard_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_cut_mode(&mut self, mode: crate::classes::label_3d::AlphaCutMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::label_3d::AlphaCutMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4838usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_alpha_cut_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_cut_mode(&self,) -> crate::classes::label_3d::AlphaCutMode {
            type CallRet = crate::classes::label_3d::AlphaCutMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4839usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_alpha_cut_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_scissor_threshold(&mut self, threshold: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4840usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_alpha_scissor_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_scissor_threshold(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4841usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_alpha_scissor_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_hash_scale(&mut self, threshold: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4842usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_alpha_hash_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_hash_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4843usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_alpha_hash_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_antialiasing(&mut self, alpha_aa: crate::classes::base_material_3d::AlphaAntiAliasing,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::AlphaAntiAliasing,);
            let args = (alpha_aa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4844usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_alpha_antialiasing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_antialiasing(&self,) -> crate::classes::base_material_3d::AlphaAntiAliasing {
            type CallRet = crate::classes::base_material_3d::AlphaAntiAliasing;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4845usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_alpha_antialiasing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_antialiasing_edge(&mut self, edge: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (edge,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4846usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_alpha_antialiasing_edge", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_antialiasing_edge(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4847usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_alpha_antialiasing_edge", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_filter(&mut self, mode: crate::classes::base_material_3d::TextureFilter,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::TextureFilter,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4848usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "set_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_filter(&self,) -> crate::classes::base_material_3d::TextureFilter {
            type CallRet = crate::classes::base_material_3d::TextureFilter;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4849usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "get_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn generate_triangle_mesh(&self,) -> Option < Gd < crate::classes::TriangleMesh > > {
            type CallRet = Option < Gd < crate::classes::TriangleMesh > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4850usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Label3D", "generate_triangle_mesh", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Label3D {
        type Base = crate::classes::GeometryInstance3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Label3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Label3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for Label3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for Label3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for Label3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Label3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Label3D {
        
    }
    impl crate::obj::cap::GodotDefault for Label3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Label3D {
        type Target = crate::classes::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Label3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Label3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Label3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Label3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::GeometryInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node3D > for $Class {
                
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
pub struct DrawFlags {
    ord: i32
}
impl DrawFlags {
    #[doc(alias = "FLAG_SHADED")]
    #[doc = "Godot enumerator name: `FLAG_SHADED`"]
    pub const SHADED: DrawFlags = DrawFlags {
        ord: 0i32
    };
    #[doc(alias = "FLAG_DOUBLE_SIDED")]
    #[doc = "Godot enumerator name: `FLAG_DOUBLE_SIDED`"]
    pub const DOUBLE_SIDED: DrawFlags = DrawFlags {
        ord: 1i32
    };
    #[doc(alias = "FLAG_DISABLE_DEPTH_TEST")]
    #[doc = "Godot enumerator name: `FLAG_DISABLE_DEPTH_TEST`"]
    pub const DISABLE_DEPTH_TEST: DrawFlags = DrawFlags {
        ord: 2i32
    };
    #[doc(alias = "FLAG_FIXED_SIZE")]
    #[doc = "Godot enumerator name: `FLAG_FIXED_SIZE`"]
    pub const FIXED_SIZE: DrawFlags = DrawFlags {
        ord: 3i32
    };
    #[doc(alias = "FLAG_MAX")]
    #[doc = "Godot enumerator name: `FLAG_MAX`"]
    pub const MAX: DrawFlags = DrawFlags {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for DrawFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DrawFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DrawFlags {
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
            Self::SHADED => "SHADED", Self::DOUBLE_SIDED => "DOUBLE_SIDED", Self::DISABLE_DEPTH_TEST => "DISABLE_DEPTH_TEST", Self::FIXED_SIZE => "FIXED_SIZE", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DrawFlags::SHADED, DrawFlags::DOUBLE_SIDED, DrawFlags::DISABLE_DEPTH_TEST, DrawFlags::FIXED_SIZE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DrawFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SHADED", "FLAG_SHADED", DrawFlags::SHADED), crate::meta::inspect::EnumConstant::new("DOUBLE_SIDED", "FLAG_DOUBLE_SIDED", DrawFlags::DOUBLE_SIDED), crate::meta::inspect::EnumConstant::new("DISABLE_DEPTH_TEST", "FLAG_DISABLE_DEPTH_TEST", DrawFlags::DISABLE_DEPTH_TEST), crate::meta::inspect::EnumConstant::new("FIXED_SIZE", "FLAG_FIXED_SIZE", DrawFlags::FIXED_SIZE), crate::meta::inspect::EnumConstant::new("MAX", "FLAG_MAX", DrawFlags::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for DrawFlags {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for DrawFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DrawFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DrawFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AlphaCutMode {
    ord: i32
}
impl AlphaCutMode {
    #[doc(alias = "ALPHA_CUT_DISABLED")]
    #[doc = "Godot enumerator name: `ALPHA_CUT_DISABLED`"]
    pub const DISABLED: AlphaCutMode = AlphaCutMode {
        ord: 0i32
    };
    #[doc(alias = "ALPHA_CUT_DISCARD")]
    #[doc = "Godot enumerator name: `ALPHA_CUT_DISCARD`"]
    pub const DISCARD: AlphaCutMode = AlphaCutMode {
        ord: 1i32
    };
    #[doc(alias = "ALPHA_CUT_OPAQUE_PREPASS")]
    #[doc = "Godot enumerator name: `ALPHA_CUT_OPAQUE_PREPASS`"]
    pub const OPAQUE_PREPASS: AlphaCutMode = AlphaCutMode {
        ord: 2i32
    };
    #[doc(alias = "ALPHA_CUT_HASH")]
    #[doc = "Godot enumerator name: `ALPHA_CUT_HASH`"]
    pub const HASH: AlphaCutMode = AlphaCutMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for AlphaCutMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AlphaCutMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AlphaCutMode {
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
            Self::DISABLED => "DISABLED", Self::DISCARD => "DISCARD", Self::OPAQUE_PREPASS => "OPAQUE_PREPASS", Self::HASH => "HASH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AlphaCutMode::DISABLED, AlphaCutMode::DISCARD, AlphaCutMode::OPAQUE_PREPASS, AlphaCutMode::HASH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AlphaCutMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "ALPHA_CUT_DISABLED", AlphaCutMode::DISABLED), crate::meta::inspect::EnumConstant::new("DISCARD", "ALPHA_CUT_DISCARD", AlphaCutMode::DISCARD), crate::meta::inspect::EnumConstant::new("OPAQUE_PREPASS", "ALPHA_CUT_OPAQUE_PREPASS", AlphaCutMode::OPAQUE_PREPASS), crate::meta::inspect::EnumConstant::new("HASH", "ALPHA_CUT_HASH", AlphaCutMode::HASH)]
        }
    }
}
impl crate::meta::GodotConvert for AlphaCutMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AlphaCutMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AlphaCutMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Label3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for Label3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}