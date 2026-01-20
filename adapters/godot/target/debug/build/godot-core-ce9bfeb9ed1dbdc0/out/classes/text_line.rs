#![doc = "Sidecar module for class [`TextLine`][crate::classes::TextLine].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextLine` enums](https://docs.godotengine.org/en/stable/classes/class_textline.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextLine.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`text_line`][crate::classes::text_line]: sidecar module with related enum/flag types\n* [`ITextLine`][crate::classes::ITextLine]: virtual methods\n\n\nSee also [Godot docs for `TextLine`](https://docs.godotengine.org/en/stable/classes/class_textline.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TextLine::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextLine {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TextLine`][crate::classes::TextLine].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `TextLine` methods](https://docs.godotengine.org/en/stable/classes/class_textline.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextLine: crate::obj::GodotClass < Base = TextLine > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ObjectNotification) {
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
    }
    impl TextLine {
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_direction(&mut self, direction: crate::classes::text_server::Direction,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::Direction,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_direction(&self,) -> crate::classes::text_server::Direction {
            type CallRet = crate::classes::text_server::Direction;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inferred_direction(&self,) -> crate::classes::text_server::Direction {
            type CallRet = crate::classes::text_server::Direction;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_inferred_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_orientation(&mut self, orientation: crate::classes::text_server::Orientation,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::Orientation,);
            let args = (orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_orientation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_orientation(&self,) -> crate::classes::text_server::Orientation {
            type CallRet = crate::classes::text_server::Orientation;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_orientation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_preserve_invalid(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_preserve_invalid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_preserve_invalid(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_preserve_invalid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_preserve_control(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_preserve_control", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_preserve_control(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_preserve_control", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bidi_override(&mut self, override_: &VarArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
            let args = (RefArg::new(override_),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_string_full(&mut self, text: CowArg < GString >, font: CowArg < Option < Gd < crate::classes::Font > > >, font_size: i32, language: CowArg < GString >, meta: RefArg < Variant >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Font > > >, i32, CowArg < 'a2, GString >, RefArg < 'a3, Variant >,);
            let args = (text, font, font_size, language, meta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "add_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_string(&mut self, text: impl AsArg < GString >, font: impl AsArg < Option < Gd < crate::classes::Font >> >, font_size: i32,) -> bool {
            self.add_string_ex(text, font, font_size,) . done()
        }
        #[inline]
        pub fn add_string_ex < 'a > (&'a mut self, text: impl AsArg < GString > + 'a, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a, font_size: i32,) -> ExAddString < 'a > {
            ExAddString::new(self, text, font, font_size,)
        }
        pub(crate) fn add_object_full(&mut self, key: RefArg < Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, length: i32, baseline: f32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >, Vector2, crate::global::InlineAlignment, i32, f32,);
            let args = (key, size, inline_align, length, baseline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "add_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_object_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_object(&mut self, key: &Variant, size: Vector2,) -> bool {
            self.add_object_ex(key, size,) . done()
        }
        #[inline]
        pub fn add_object_ex < 'a > (&'a mut self, key: &'a Variant, size: Vector2,) -> ExAddObject < 'a > {
            ExAddObject::new(self, key, size,)
        }
        pub(crate) fn resize_object_full(&mut self, key: RefArg < Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, baseline: f32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >, Vector2, crate::global::InlineAlignment, f32,);
            let args = (key, size, inline_align, baseline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "resize_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::resize_object_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn resize_object(&mut self, key: &Variant, size: Vector2,) -> bool {
            self.resize_object_ex(key, size,) . done()
        }
        #[inline]
        pub fn resize_object_ex < 'a > (&'a mut self, key: &'a Variant, size: Vector2,) -> ExResizeObject < 'a > {
            ExResizeObject::new(self, key, size,)
        }
        pub fn set_width(&mut self, width: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_width(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_horizontal_alignment(&mut self, alignment: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::HorizontalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_horizontal_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallRet = crate::global::HorizontalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tab_align(&mut self, tab_stops: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedFloat32Array >,);
            let args = (RefArg::new(tab_stops),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "tab_align", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flags(&mut self, flags: crate::classes::text_server::JustificationFlag,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::JustificationFlag,);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_flags(&self,) -> crate::classes::text_server::JustificationFlag {
            type CallRet = crate::classes::text_server::JustificationFlag;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_overrun_behavior(&mut self, overrun_behavior: crate::classes::text_server::OverrunBehavior,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::OverrunBehavior,);
            let args = (overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_overrun_behavior(&self,) -> crate::classes::text_server::OverrunBehavior {
            type CallRet = crate::classes::text_server::OverrunBehavior;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ellipsis_char(&mut self, char: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (char.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "set_ellipsis_char", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ellipsis_char(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_ellipsis_char", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_objects(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_objects", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_object_rect(&self, key: &Variant,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
            let args = (RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_object_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9148usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rid(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9149usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_ascent(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_line_ascent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_descent(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_line_descent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_width(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_line_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_underline_position(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_line_underline_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_underline_thickness(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "get_line_underline_thickness", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn draw_full(&self, canvas: Rid, pos: Vector2, color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, Color, f32,);
            let args = (canvas, pos, color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "draw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw(&self, canvas: Rid, pos: Vector2,) {
            self.draw_ex(canvas, pos,) . done()
        }
        #[inline]
        pub fn draw_ex < 'a > (&'a self, canvas: Rid, pos: Vector2,) -> ExDraw < 'a > {
            ExDraw::new(self, canvas, pos,)
        }
        pub(crate) fn draw_outline_full(&self, canvas: Rid, pos: Vector2, outline_size: i32, color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, i32, Color, f32,);
            let args = (canvas, pos, outline_size, color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "draw_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_outline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_outline(&self, canvas: Rid, pos: Vector2,) {
            self.draw_outline_ex(canvas, pos,) . done()
        }
        #[inline]
        pub fn draw_outline_ex < 'a > (&'a self, canvas: Rid, pos: Vector2,) -> ExDrawOutline < 'a > {
            ExDrawOutline::new(self, canvas, pos,)
        }
        pub fn hit_test(&self, coords: f32,) -> i32 {
            type CallRet = i32;
            type CallParams = (f32,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextLine", "hit_test", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TextLine {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TextLine"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextLine {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TextLine {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextLine {
        
    }
    impl crate::obj::cap::GodotDefault for TextLine {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TextLine {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextLine {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TextLine`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TextLine__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TextLine > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TextLine::add_string_ex`][super::TextLine::add_string_ex]."]
#[must_use]
pub struct ExAddString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextLine, text: CowArg < 'a, GString >, font: CowArg < 'a, Option < Gd < crate::classes::Font > > >, font_size: i32, language: CowArg < 'a, GString >, meta: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddString < 'a > {
    fn new(surround_object: &'a mut re_export::TextLine, text: impl AsArg < GString > + 'a, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a, font_size: i32,) -> Self {
        let language = GString::from("");
        let meta = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), font: font.into_arg(), font_size: font_size, language: CowArg::Owned(language), meta: CowArg::Owned(meta),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn meta(self, meta: &'a Variant) -> Self {
        Self {
            meta: CowArg::Borrowed(meta), .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, text, font, font_size, language, meta,
        }
        = self;
        re_export::TextLine::add_string_full(surround_object, text, font, font_size, language, meta.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`TextLine::add_object_ex`][super::TextLine::add_object_ex]."]
#[must_use]
pub struct ExAddObject < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextLine, key: CowArg < 'a, Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, length: i32, baseline: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddObject < 'a > {
    fn new(surround_object: &'a mut re_export::TextLine, key: &'a Variant, size: Vector2,) -> Self {
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let length = 1i32;
        let baseline = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, key: CowArg::Borrowed(key), size: size, inline_align: inline_align, length: length, baseline: baseline,
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn length(self, length: i32) -> Self {
        Self {
            length: length, .. self
        }
    }
    #[inline]
    pub fn baseline(self, baseline: f32) -> Self {
        Self {
            baseline: baseline, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, key, size, inline_align, length, baseline,
        }
        = self;
        re_export::TextLine::add_object_full(surround_object, key.cow_as_arg(), size, inline_align, length, baseline,)
    }
}
#[doc = "Default-param extender for [`TextLine::resize_object_ex`][super::TextLine::resize_object_ex]."]
#[must_use]
pub struct ExResizeObject < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextLine, key: CowArg < 'a, Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, baseline: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExResizeObject < 'a > {
    fn new(surround_object: &'a mut re_export::TextLine, key: &'a Variant, size: Vector2,) -> Self {
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let baseline = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, key: CowArg::Borrowed(key), size: size, inline_align: inline_align, baseline: baseline,
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn baseline(self, baseline: f32) -> Self {
        Self {
            baseline: baseline, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, key, size, inline_align, baseline,
        }
        = self;
        re_export::TextLine::resize_object_full(surround_object, key.cow_as_arg(), size, inline_align, baseline,)
    }
}
#[doc = "Default-param extender for [`TextLine::draw_ex`][super::TextLine::draw_ex]."]
#[must_use]
pub struct ExDraw < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextLine, canvas: Rid, pos: Vector2, color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDraw < 'a > {
    fn new(surround_object: &'a re_export::TextLine, canvas: Rid, pos: Vector2,) -> Self {
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas: canvas, pos: pos, color: color, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas, pos, color, oversampling,
        }
        = self;
        re_export::TextLine::draw_full(surround_object, canvas, pos, color, oversampling,)
    }
}
#[doc = "Default-param extender for [`TextLine::draw_outline_ex`][super::TextLine::draw_outline_ex]."]
#[must_use]
pub struct ExDrawOutline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextLine, canvas: Rid, pos: Vector2, outline_size: i32, color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawOutline < 'a > {
    fn new(surround_object: &'a re_export::TextLine, canvas: Rid, pos: Vector2,) -> Self {
        let outline_size = 1i32;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, canvas: canvas, pos: pos, outline_size: outline_size, color: color, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn outline_size(self, outline_size: i32) -> Self {
        Self {
            outline_size: outline_size, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn oversampling(self, oversampling: f32) -> Self {
        Self {
            oversampling: oversampling, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, canvas, pos, outline_size, color, oversampling,
        }
        = self;
        re_export::TextLine::draw_outline_full(surround_object, canvas, pos, outline_size, color, oversampling,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TextLine;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for TextLine {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfObject < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}