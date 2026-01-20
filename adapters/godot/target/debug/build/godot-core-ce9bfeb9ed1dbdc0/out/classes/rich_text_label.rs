#![doc = "Sidecar module for class [`RichTextLabel`][crate::classes::RichTextLabel].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RichTextLabel` enums](https://docs.godotengine.org/en/stable/classes/class_richtextlabel.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RichTextLabel.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`rich_text_label`][crate::classes::rich_text_label]: sidecar module with related enum/flag types\n* [`IRichTextLabel`][crate::classes::IRichTextLabel]: virtual methods\n* [`SignalsOfRichTextLabel`][crate::classes::rich_text_label::SignalsOfRichTextLabel]: signal collection\n\n\nSee also [Godot docs for `RichTextLabel`](https://docs.godotengine.org/en/stable/classes/class_richtextlabel.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`RichTextLabel::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RichTextLabel {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`RichTextLabel`][crate::classes::RichTextLabel].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `RichTextLabel` methods](https://docs.godotengine.org/en/stable/classes/class_richtextlabel.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRichTextLabel: crate::obj::GodotClass < Base = RichTextLabel > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RichTextLabel {
        pub fn get_parsed_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7522usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_parsed_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_text(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7523usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "add_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7524usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_text", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_hr_full(&mut self, width: i32, height: i32, color: Color, alignment: crate::global::HorizontalAlignment, width_in_percent: bool, height_in_percent: bool,) {
            type CallRet = ();
            type CallParams = (i32, i32, Color, crate::global::HorizontalAlignment, bool, bool,);
            let args = (width, height, color, alignment, width_in_percent, height_in_percent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7525usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "add_hr", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_hr_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_hr(&mut self,) {
            self.add_hr_ex() . done()
        }
        #[inline]
        pub fn add_hr_ex < 'a > (&'a mut self,) -> ExAddHr < 'a > {
            ExAddHr::new(self,)
        }
        pub(crate) fn add_image_full(&mut self, image: CowArg < Option < Gd < crate::classes::Texture2D > > >, width: i32, height: i32, color: Color, inline_align: crate::global::InlineAlignment, region: Rect2, key: RefArg < Variant >, pad: bool, tooltip: CowArg < GString >, width_in_percent: bool, height_in_percent: bool, alt_text: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, i32, i32, Color, crate::global::InlineAlignment, Rect2, RefArg < 'a1, Variant >, bool, CowArg < 'a2, GString >, bool, bool, CowArg < 'a3, GString >,);
            let args = (image, width, height, color, inline_align, region, key, pad, tooltip, width_in_percent, height_in_percent, alt_text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7526usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "add_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_image_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_image(&mut self, image: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            self.add_image_ex(image,) . done()
        }
        #[inline]
        pub fn add_image_ex < 'a > (&'a mut self, image: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a,) -> ExAddImage < 'a > {
            ExAddImage::new(self, image,)
        }
        pub(crate) fn update_image_full(&mut self, key: RefArg < Variant >, mask: crate::classes::rich_text_label::ImageUpdateMask, image: CowArg < Option < Gd < crate::classes::Texture2D > > >, width: i32, height: i32, color: Color, inline_align: crate::global::InlineAlignment, region: Rect2, pad: bool, tooltip: CowArg < GString >, width_in_percent: bool, height_in_percent: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, Variant >, crate::classes::rich_text_label::ImageUpdateMask, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >, i32, i32, Color, crate::global::InlineAlignment, Rect2, bool, CowArg < 'a2, GString >, bool, bool,);
            let args = (key, mask, image, width, height, color, inline_align, region, pad, tooltip, width_in_percent, height_in_percent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7527usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "update_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::update_image_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn update_image(&mut self, key: &Variant, mask: crate::classes::rich_text_label::ImageUpdateMask, image: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            self.update_image_ex(key, mask, image,) . done()
        }
        #[inline]
        pub fn update_image_ex < 'a > (&'a mut self, key: &'a Variant, mask: crate::classes::rich_text_label::ImageUpdateMask, image: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a,) -> ExUpdateImage < 'a > {
            ExUpdateImage::new(self, key, mask, image,)
        }
        pub fn newline(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7528usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "newline", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn remove_paragraph_full(&mut self, paragraph: i32, no_invalidate: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, bool,);
            let args = (paragraph, no_invalidate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7529usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "remove_paragraph", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::remove_paragraph_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn remove_paragraph(&mut self, paragraph: i32,) -> bool {
            self.remove_paragraph_ex(paragraph,) . done()
        }
        #[inline]
        pub fn remove_paragraph_ex < 'a > (&'a mut self, paragraph: i32,) -> ExRemoveParagraph < 'a > {
            ExRemoveParagraph::new(self, paragraph,)
        }
        pub fn invalidate_paragraph(&mut self, paragraph: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (paragraph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7530usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "invalidate_paragraph", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn push_font_full(&mut self, font: CowArg < Option < Gd < crate::classes::Font > > >, font_size: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Font > > >, i32,);
            let args = (font, font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7531usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_font_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_font(&mut self, font: impl AsArg < Option < Gd < crate::classes::Font >> >,) {
            self.push_font_ex(font,) . done()
        }
        #[inline]
        pub fn push_font_ex < 'a > (&'a mut self, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a,) -> ExPushFont < 'a > {
            ExPushFont::new(self, font,)
        }
        pub fn push_font_size(&mut self, font_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7532usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn push_normal(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7533usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_normal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn push_bold(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7534usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_bold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn push_bold_italics(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7535usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_bold_italics", Some(self.__validated_obj()), args,)
            }
        }
        pub fn push_italics(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7536usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_italics", Some(self.__validated_obj()), args,)
            }
        }
        pub fn push_mono(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7537usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_mono", Some(self.__validated_obj()), args,)
            }
        }
        pub fn push_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7538usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn push_outline_size(&mut self, outline_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (outline_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7539usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_outline_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn push_outline_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7540usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_outline_color", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn push_paragraph_full(&mut self, alignment: crate::global::HorizontalAlignment, base_direction: crate::classes::control::TextDirection, language: CowArg < GString >, st_parser: crate::classes::text_server::StructuredTextParser, justification_flags: crate::classes::text_server::JustificationFlag, tab_stops: RefArg < PackedFloat32Array >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (crate::global::HorizontalAlignment, crate::classes::control::TextDirection, CowArg < 'a0, GString >, crate::classes::text_server::StructuredTextParser, crate::classes::text_server::JustificationFlag, RefArg < 'a1, PackedFloat32Array >,);
            let args = (alignment, base_direction, language, st_parser, justification_flags, tab_stops,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7541usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_paragraph", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_paragraph_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_paragraph(&mut self, alignment: crate::global::HorizontalAlignment,) {
            self.push_paragraph_ex(alignment,) . done()
        }
        #[inline]
        pub fn push_paragraph_ex < 'a > (&'a mut self, alignment: crate::global::HorizontalAlignment,) -> ExPushParagraph < 'a > {
            ExPushParagraph::new(self, alignment,)
        }
        pub fn push_indent(&mut self, level: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (level,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7542usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_indent", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn push_list_full(&mut self, level: i32, type_: crate::classes::rich_text_label::ListType, capitalize: bool, bullet: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, crate::classes::rich_text_label::ListType, bool, CowArg < 'a0, GString >,);
            let args = (level, type_, capitalize, bullet,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7543usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_list_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_list(&mut self, level: i32, type_: crate::classes::rich_text_label::ListType, capitalize: bool,) {
            self.push_list_ex(level, type_, capitalize,) . done()
        }
        #[inline]
        pub fn push_list_ex < 'a > (&'a mut self, level: i32, type_: crate::classes::rich_text_label::ListType, capitalize: bool,) -> ExPushList < 'a > {
            ExPushList::new(self, level, type_, capitalize,)
        }
        pub(crate) fn push_meta_full(&mut self, data: RefArg < Variant >, underline_mode: crate::classes::rich_text_label::MetaUnderline, tooltip: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, crate::classes::rich_text_label::MetaUnderline, CowArg < 'a1, GString >,);
            let args = (data, underline_mode, tooltip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7544usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_meta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_meta_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_meta(&mut self, data: &Variant,) {
            self.push_meta_ex(data,) . done()
        }
        #[inline]
        pub fn push_meta_ex < 'a > (&'a mut self, data: &'a Variant,) -> ExPushMeta < 'a > {
            ExPushMeta::new(self, data,)
        }
        pub fn push_hint(&mut self, description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (description.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7545usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_hint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn push_language(&mut self, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7546usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_language", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn push_underline_full(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7547usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_underline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_underline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_underline(&mut self,) {
            self.push_underline_ex() . done()
        }
        #[inline]
        pub fn push_underline_ex < 'a > (&'a mut self,) -> ExPushUnderline < 'a > {
            ExPushUnderline::new(self,)
        }
        pub(crate) fn push_strikethrough_full(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7548usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_strikethrough", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_strikethrough_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_strikethrough(&mut self,) {
            self.push_strikethrough_ex() . done()
        }
        #[inline]
        pub fn push_strikethrough_ex < 'a > (&'a mut self,) -> ExPushStrikethrough < 'a > {
            ExPushStrikethrough::new(self,)
        }
        pub(crate) fn push_table_full(&mut self, columns: i32, inline_align: crate::global::InlineAlignment, align_to_row: i32, name: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, crate::global::InlineAlignment, i32, CowArg < 'a0, GString >,);
            let args = (columns, inline_align, align_to_row, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7549usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_table", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_table_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_table(&mut self, columns: i32,) {
            self.push_table_ex(columns,) . done()
        }
        #[inline]
        pub fn push_table_ex < 'a > (&'a mut self, columns: i32,) -> ExPushTable < 'a > {
            ExPushTable::new(self, columns,)
        }
        pub(crate) fn push_dropcap_full(&mut self, string: CowArg < GString >, font: CowArg < Option < Gd < crate::classes::Font > > >, size: i32, dropcap_margins: Rect2, color: Color, outline_size: i32, outline_color: Color,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Font > > >, i32, Rect2, Color, i32, Color,);
            let args = (string, font, size, dropcap_margins, color, outline_size, outline_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7550usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_dropcap", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_dropcap_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_dropcap(&mut self, string: impl AsArg < GString >, font: impl AsArg < Option < Gd < crate::classes::Font >> >, size: i32,) {
            self.push_dropcap_ex(string, font, size,) . done()
        }
        #[inline]
        pub fn push_dropcap_ex < 'a > (&'a mut self, string: impl AsArg < GString > + 'a, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a, size: i32,) -> ExPushDropcap < 'a > {
            ExPushDropcap::new(self, string, font, size,)
        }
        pub(crate) fn set_table_column_expand_full(&mut self, column: i32, expand: bool, ratio: i32, shrink: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool, i32, bool,);
            let args = (column, expand, ratio, shrink,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7551usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_table_column_expand", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_table_column_expand_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_table_column_expand(&mut self, column: i32, expand: bool,) {
            self.set_table_column_expand_ex(column, expand,) . done()
        }
        #[inline]
        pub fn set_table_column_expand_ex < 'a > (&'a mut self, column: i32, expand: bool,) -> ExSetTableColumnExpand < 'a > {
            ExSetTableColumnExpand::new(self, column, expand,)
        }
        pub fn set_table_column_name(&mut self, column: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7552usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_table_column_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cell_row_background_color(&mut self, odd_row_bg: Color, even_row_bg: Color,) {
            type CallRet = ();
            type CallParams = (Color, Color,);
            let args = (odd_row_bg, even_row_bg,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7553usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_cell_row_background_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cell_border_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7554usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_cell_border_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cell_size_override(&mut self, min_size: Vector2, max_size: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2,);
            let args = (min_size, max_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7555usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_cell_size_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cell_padding(&mut self, padding: Rect2,) {
            type CallRet = ();
            type CallParams = (Rect2,);
            let args = (padding,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7556usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_cell_padding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn push_cell(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7557usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_cell", Some(self.__validated_obj()), args,)
            }
        }
        pub fn push_fgcolor(&mut self, fgcolor: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (fgcolor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7558usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_fgcolor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn push_bgcolor(&mut self, bgcolor: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (bgcolor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7559usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_bgcolor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn push_customfx(&mut self, effect: impl AsArg < Option < Gd < crate::classes::RichTextEffect >> >, env: &VarDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::RichTextEffect > > >, RefArg < 'a1, VarDictionary >,);
            let args = (effect.into_arg(), RefArg::new(env),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7560usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_customfx", Some(self.__validated_obj()), args,)
            }
        }
        pub fn push_context(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7561usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "push_context", Some(self.__validated_obj()), args,)
            }
        }
        pub fn pop_context(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7562usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "pop_context", Some(self.__validated_obj()), args,)
            }
        }
        pub fn pop(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7563usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "pop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn pop_all(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7564usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "pop_all", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7565usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::classes::text_server::StructuredTextParser,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::StructuredTextParser,);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7566usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::classes::text_server::StructuredTextParser {
            type CallRet = crate::classes::text_server::StructuredTextParser;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7567usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: &VarArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
            let args = (RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7568usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7569usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::TextDirection,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7570usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7571usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7572usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7573usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_horizontal_alignment(&mut self, alignment: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::HorizontalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7574usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_horizontal_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallRet = crate::global::HorizontalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7575usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertical_alignment(&mut self, alignment: crate::global::VerticalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::VerticalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7576usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_vertical_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertical_alignment(&self,) -> crate::global::VerticalAlignment {
            type CallRet = crate::global::VerticalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7577usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_vertical_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_justification_flags(&mut self, justification_flags: crate::classes::text_server::JustificationFlag,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::JustificationFlag,);
            let args = (justification_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7578usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_justification_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_justification_flags(&self,) -> crate::classes::text_server::JustificationFlag {
            type CallRet = crate::classes::text_server::JustificationFlag;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7579usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_justification_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_stops(&mut self, tab_stops: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedFloat32Array >,);
            let args = (RefArg::new(tab_stops),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7580usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_tab_stops", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_stops(&self,) -> PackedFloat32Array {
            type CallRet = PackedFloat32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7581usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_tab_stops", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, autowrap_mode: crate::classes::text_server::AutowrapMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::AutowrapMode,);
            let args = (autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7582usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autowrap_mode(&self,) -> crate::classes::text_server::AutowrapMode {
            type CallRet = crate::classes::text_server::AutowrapMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7583usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autowrap_trim_flags(&mut self, autowrap_trim_flags: crate::classes::text_server::LineBreakFlag,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::LineBreakFlag,);
            let args = (autowrap_trim_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7584usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_autowrap_trim_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autowrap_trim_flags(&self,) -> crate::classes::text_server::LineBreakFlag {
            type CallRet = crate::classes::text_server::LineBreakFlag;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7585usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_autowrap_trim_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_meta_underline(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7586usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_meta_underline", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_meta_underlined(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7587usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_meta_underlined", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hint_underline(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7588usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_hint_underline", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_hint_underlined(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7589usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_hint_underlined", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scroll_active(&mut self, active: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7590usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_scroll_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_scroll_active(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7591usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_scroll_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scroll_follow_visible_characters(&mut self, follow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (follow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7592usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_scroll_follow_visible_characters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_scroll_following_visible_characters(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7593usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_scroll_following_visible_characters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scroll_follow(&mut self, follow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (follow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7594usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_scroll_follow", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_scroll_following(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7595usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_scroll_following", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_v_scroll_bar(&mut self,) -> Option < Gd < crate::classes::VScrollBar > > {
            type CallRet = Option < Gd < crate::classes::VScrollBar > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7596usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_v_scroll_bar", Some(self.__validated_obj()), args,)
            }
        }
        pub fn scroll_to_line(&mut self, line: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7597usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "scroll_to_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn scroll_to_paragraph(&mut self, paragraph: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (paragraph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7598usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "scroll_to_paragraph", Some(self.__validated_obj()), args,)
            }
        }
        pub fn scroll_to_selection(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7599usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "scroll_to_selection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tab_size(&mut self, spaces: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (spaces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7600usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_tab_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tab_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7601usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_tab_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fit_content(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7602usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_fit_content", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_fit_content_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7603usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_fit_content_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_selection_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7604usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_selection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_selection_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7605usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_selection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_context_menu_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7606usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_context_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_context_menu_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7607usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_context_menu_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shortcut_keys_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7608usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_shortcut_keys_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_shortcut_keys_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7609usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_shortcut_keys_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_deselect_on_focus_loss_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7610usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_deselect_on_focus_loss_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_deselect_on_focus_loss_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7611usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_deselect_on_focus_loss_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drag_and_drop_selection_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7612usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_drag_and_drop_selection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_drag_and_drop_selection_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7613usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_drag_and_drop_selection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selection_from(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7614usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_selection_from", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selection_to(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7615usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_selection_to", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selection_line_offset(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7616usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_selection_line_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn select_all(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7617usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "select_all", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selected_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7618usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_selected_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn deselect(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7619usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "deselect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn parse_bbcode(&mut self, bbcode: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (bbcode.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7620usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "parse_bbcode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn append_text(&mut self, bbcode: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (bbcode.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7621usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "append_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7622usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_ready(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7623usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_ready", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_finished(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7624usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_finished", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_threaded(&mut self, threaded: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (threaded,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7625usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_threaded", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_threaded(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7626usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_threaded", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_progress_bar_delay(&mut self, delay_ms: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (delay_ms,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7627usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_progress_bar_delay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_progress_bar_delay(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7628usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_progress_bar_delay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible_characters(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7629usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_visible_characters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_characters(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7630usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_characters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_characters_behavior(&self,) -> crate::classes::text_server::VisibleCharactersBehavior {
            type CallRet = crate::classes::text_server::VisibleCharactersBehavior;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7631usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_characters_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible_characters_behavior(&mut self, behavior: crate::classes::text_server::VisibleCharactersBehavior,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::VisibleCharactersBehavior,);
            let args = (behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7632usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_visible_characters_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible_ratio(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7633usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_visible_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_ratio(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7634usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_character_line(&mut self, character: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (character,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7635usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_character_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_character_paragraph(&mut self, character: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (character,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7636usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_character_paragraph", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_total_character_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7637usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_total_character_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_bbcode(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7638usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_use_bbcode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_bbcode(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7639usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_using_bbcode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7640usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_line_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_range(&mut self, line: i32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7641usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_line_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_line_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7642usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_line_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_paragraph_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7643usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_paragraph_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_paragraph_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7644usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_paragraph_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_content_height(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7645usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_content_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_content_width(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7646usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_content_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_height(&self, line: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7647usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_line_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_width(&self, line: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7648usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_line_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible_content_rect(&self,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7649usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_visible_content_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_offset(&mut self, line: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7650usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_line_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_paragraph_offset(&mut self, paragraph: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (paragraph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7651usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_paragraph_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn parse_expressions_for_values(&mut self, expressions: &PackedStringArray,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(expressions),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7652usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "parse_expressions_for_values", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_effects(&mut self, effects: &VarArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
            let args = (RefArg::new(effects),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7653usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "set_effects", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_effects(&mut self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7654usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_effects", Some(self.__validated_obj()), args,)
            }
        }
        pub fn install_effect(&mut self, effect: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
            let args = (RefArg::new(effect),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7655usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "install_effect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reload_effects(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7656usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "reload_effects", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_menu(&self,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallRet = Option < Gd < crate::classes::PopupMenu > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7657usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "get_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_menu_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7658usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "is_menu_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn menu_option(&mut self, option: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7659usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RichTextLabel", "menu_option", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for RichTextLabel {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("RichTextLabel"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RichTextLabel {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for RichTextLabel {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for RichTextLabel {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for RichTextLabel {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RichTextLabel {
        
    }
    impl crate::obj::cap::GodotDefault for RichTextLabel {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RichTextLabel {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RichTextLabel {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RichTextLabel`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RichTextLabel__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RichTextLabel > for $Class {
                
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
#[doc = "Default-param extender for [`RichTextLabel::add_hr_ex`][super::RichTextLabel::add_hr_ex]."]
#[must_use]
pub struct ExAddHr < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, width: i32, height: i32, color: Color, alignment: crate::global::HorizontalAlignment, width_in_percent: bool, height_in_percent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddHr < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel,) -> Self {
        let width = 90i32;
        let height = 2i32;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let alignment = crate::obj::EngineEnum::from_ord(1);
        let width_in_percent = true;
        let height_in_percent = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, width: width, height: height, color: color, alignment: alignment, width_in_percent: width_in_percent, height_in_percent: height_in_percent,
        }
    }
    #[inline]
    pub fn width(self, width: i32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn height(self, height: i32) -> Self {
        Self {
            height: height, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn alignment(self, alignment: crate::global::HorizontalAlignment) -> Self {
        Self {
            alignment: alignment, .. self
        }
    }
    #[inline]
    pub fn width_in_percent(self, width_in_percent: bool) -> Self {
        Self {
            width_in_percent: width_in_percent, .. self
        }
    }
    #[inline]
    pub fn height_in_percent(self, height_in_percent: bool) -> Self {
        Self {
            height_in_percent: height_in_percent, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, width, height, color, alignment, width_in_percent, height_in_percent,
        }
        = self;
        re_export::RichTextLabel::add_hr_full(surround_object, width, height, color, alignment, width_in_percent, height_in_percent,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::add_image_ex`][super::RichTextLabel::add_image_ex]."]
#[must_use]
pub struct ExAddImage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, image: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, width: i32, height: i32, color: Color, inline_align: crate::global::InlineAlignment, region: Rect2, key: CowArg < 'a, Variant >, pad: bool, tooltip: CowArg < 'a, GString >, width_in_percent: bool, height_in_percent: bool, alt_text: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddImage < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, image: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a,) -> Self {
        let width = 0i32;
        let height = 0i32;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let region = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let key = Variant::nil();
        let pad = false;
        let tooltip = GString::from("");
        let width_in_percent = false;
        let height_in_percent = false;
        let alt_text = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, image: image.into_arg(), width: width, height: height, color: color, inline_align: inline_align, region: region, key: CowArg::Owned(key), pad: pad, tooltip: CowArg::Owned(tooltip), width_in_percent: width_in_percent, height_in_percent: height_in_percent, alt_text: CowArg::Owned(alt_text),
        }
    }
    #[inline]
    pub fn width(self, width: i32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn height(self, height: i32) -> Self {
        Self {
            height: height, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn region(self, region: Rect2) -> Self {
        Self {
            region: region, .. self
        }
    }
    #[inline]
    pub fn key(self, key: &'a Variant) -> Self {
        Self {
            key: CowArg::Borrowed(key), .. self
        }
    }
    #[inline]
    pub fn pad(self, pad: bool) -> Self {
        Self {
            pad: pad, .. self
        }
    }
    #[inline]
    pub fn tooltip(self, tooltip: impl AsArg < GString > + 'a) -> Self {
        Self {
            tooltip: tooltip.into_arg(), .. self
        }
    }
    #[inline]
    pub fn width_in_percent(self, width_in_percent: bool) -> Self {
        Self {
            width_in_percent: width_in_percent, .. self
        }
    }
    #[inline]
    pub fn height_in_percent(self, height_in_percent: bool) -> Self {
        Self {
            height_in_percent: height_in_percent, .. self
        }
    }
    #[inline]
    pub fn alt_text(self, alt_text: impl AsArg < GString > + 'a) -> Self {
        Self {
            alt_text: alt_text.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, image, width, height, color, inline_align, region, key, pad, tooltip, width_in_percent, height_in_percent, alt_text,
        }
        = self;
        re_export::RichTextLabel::add_image_full(surround_object, image, width, height, color, inline_align, region, key.cow_as_arg(), pad, tooltip, width_in_percent, height_in_percent, alt_text,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::update_image_ex`][super::RichTextLabel::update_image_ex]."]
#[must_use]
pub struct ExUpdateImage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, key: CowArg < 'a, Variant >, mask: crate::classes::rich_text_label::ImageUpdateMask, image: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, width: i32, height: i32, color: Color, inline_align: crate::global::InlineAlignment, region: Rect2, pad: bool, tooltip: CowArg < 'a, GString >, width_in_percent: bool, height_in_percent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExUpdateImage < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, key: &'a Variant, mask: crate::classes::rich_text_label::ImageUpdateMask, image: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a,) -> Self {
        let width = 0i32;
        let height = 0i32;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let region = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let pad = false;
        let tooltip = GString::from("");
        let width_in_percent = false;
        let height_in_percent = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, key: CowArg::Borrowed(key), mask: mask, image: image.into_arg(), width: width, height: height, color: color, inline_align: inline_align, region: region, pad: pad, tooltip: CowArg::Owned(tooltip), width_in_percent: width_in_percent, height_in_percent: height_in_percent,
        }
    }
    #[inline]
    pub fn width(self, width: i32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn height(self, height: i32) -> Self {
        Self {
            height: height, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn region(self, region: Rect2) -> Self {
        Self {
            region: region, .. self
        }
    }
    #[inline]
    pub fn pad(self, pad: bool) -> Self {
        Self {
            pad: pad, .. self
        }
    }
    #[inline]
    pub fn tooltip(self, tooltip: impl AsArg < GString > + 'a) -> Self {
        Self {
            tooltip: tooltip.into_arg(), .. self
        }
    }
    #[inline]
    pub fn width_in_percent(self, width_in_percent: bool) -> Self {
        Self {
            width_in_percent: width_in_percent, .. self
        }
    }
    #[inline]
    pub fn height_in_percent(self, height_in_percent: bool) -> Self {
        Self {
            height_in_percent: height_in_percent, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, key, mask, image, width, height, color, inline_align, region, pad, tooltip, width_in_percent, height_in_percent,
        }
        = self;
        re_export::RichTextLabel::update_image_full(surround_object, key.cow_as_arg(), mask, image, width, height, color, inline_align, region, pad, tooltip, width_in_percent, height_in_percent,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::remove_paragraph_ex`][super::RichTextLabel::remove_paragraph_ex]."]
#[must_use]
pub struct ExRemoveParagraph < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, paragraph: i32, no_invalidate: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRemoveParagraph < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, paragraph: i32,) -> Self {
        let no_invalidate = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, paragraph: paragraph, no_invalidate: no_invalidate,
        }
    }
    #[inline]
    pub fn no_invalidate(self, no_invalidate: bool) -> Self {
        Self {
            no_invalidate: no_invalidate, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, paragraph, no_invalidate,
        }
        = self;
        re_export::RichTextLabel::remove_paragraph_full(surround_object, paragraph, no_invalidate,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_font_ex`][super::RichTextLabel::push_font_ex]."]
#[must_use]
pub struct ExPushFont < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, font: CowArg < 'a, Option < Gd < crate::classes::Font > > >, font_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushFont < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a,) -> Self {
        let font_size = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font: font.into_arg(), font_size: font_size,
        }
    }
    #[inline]
    pub fn font_size(self, font_size: i32) -> Self {
        Self {
            font_size: font_size, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, font, font_size,
        }
        = self;
        re_export::RichTextLabel::push_font_full(surround_object, font, font_size,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_paragraph_ex`][super::RichTextLabel::push_paragraph_ex]."]
#[must_use]
pub struct ExPushParagraph < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, alignment: crate::global::HorizontalAlignment, base_direction: crate::classes::control::TextDirection, language: CowArg < 'a, GString >, st_parser: crate::classes::text_server::StructuredTextParser, justification_flags: crate::classes::text_server::JustificationFlag, tab_stops: CowArg < 'a, PackedFloat32Array >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushParagraph < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, alignment: crate::global::HorizontalAlignment,) -> Self {
        let base_direction = crate::obj::EngineEnum::from_ord(0);
        let language = GString::from("");
        let st_parser = crate::obj::EngineEnum::from_ord(0);
        let justification_flags = crate::obj::EngineBitfield::from_ord(163);
        let tab_stops = PackedFloat32Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, alignment: alignment, base_direction: base_direction, language: CowArg::Owned(language), st_parser: st_parser, justification_flags: justification_flags, tab_stops: CowArg::Owned(tab_stops),
        }
    }
    #[inline]
    pub fn base_direction(self, base_direction: crate::classes::control::TextDirection) -> Self {
        Self {
            base_direction: base_direction, .. self
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn st_parser(self, st_parser: crate::classes::text_server::StructuredTextParser) -> Self {
        Self {
            st_parser: st_parser, .. self
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn tab_stops(self, tab_stops: &'a PackedFloat32Array) -> Self {
        Self {
            tab_stops: CowArg::Borrowed(tab_stops), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, alignment, base_direction, language, st_parser, justification_flags, tab_stops,
        }
        = self;
        re_export::RichTextLabel::push_paragraph_full(surround_object, alignment, base_direction, language, st_parser, justification_flags, tab_stops.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_list_ex`][super::RichTextLabel::push_list_ex]."]
#[must_use]
pub struct ExPushList < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, level: i32, type_: crate::classes::rich_text_label::ListType, capitalize: bool, bullet: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushList < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, level: i32, type_: crate::classes::rich_text_label::ListType, capitalize: bool,) -> Self {
        let bullet = GString::from("•");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, level: level, type_: type_, capitalize: capitalize, bullet: CowArg::Owned(bullet),
        }
    }
    #[inline]
    pub fn bullet(self, bullet: impl AsArg < GString > + 'a) -> Self {
        Self {
            bullet: bullet.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, level, type_, capitalize, bullet,
        }
        = self;
        re_export::RichTextLabel::push_list_full(surround_object, level, type_, capitalize, bullet,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_meta_ex`][super::RichTextLabel::push_meta_ex]."]
#[must_use]
pub struct ExPushMeta < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, data: CowArg < 'a, Variant >, underline_mode: crate::classes::rich_text_label::MetaUnderline, tooltip: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushMeta < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, data: &'a Variant,) -> Self {
        let underline_mode = crate::obj::EngineEnum::from_ord(1);
        let tooltip = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, data: CowArg::Borrowed(data), underline_mode: underline_mode, tooltip: CowArg::Owned(tooltip),
        }
    }
    #[inline]
    pub fn underline_mode(self, underline_mode: crate::classes::rich_text_label::MetaUnderline) -> Self {
        Self {
            underline_mode: underline_mode, .. self
        }
    }
    #[inline]
    pub fn tooltip(self, tooltip: impl AsArg < GString > + 'a) -> Self {
        Self {
            tooltip: tooltip.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, data, underline_mode, tooltip,
        }
        = self;
        re_export::RichTextLabel::push_meta_full(surround_object, data.cow_as_arg(), underline_mode, tooltip,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_underline_ex`][super::RichTextLabel::push_underline_ex]."]
#[must_use]
pub struct ExPushUnderline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushUnderline < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel,) -> Self {
        let color = Color::from_rgba(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, color: color,
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, color,
        }
        = self;
        re_export::RichTextLabel::push_underline_full(surround_object, color,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_strikethrough_ex`][super::RichTextLabel::push_strikethrough_ex]."]
#[must_use]
pub struct ExPushStrikethrough < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushStrikethrough < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel,) -> Self {
        let color = Color::from_rgba(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, color: color,
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, color,
        }
        = self;
        re_export::RichTextLabel::push_strikethrough_full(surround_object, color,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_table_ex`][super::RichTextLabel::push_table_ex]."]
#[must_use]
pub struct ExPushTable < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, columns: i32, inline_align: crate::global::InlineAlignment, align_to_row: i32, name: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushTable < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, columns: i32,) -> Self {
        let inline_align = crate::obj::EngineEnum::from_ord(0);
        let align_to_row = - 1i32;
        let name = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, columns: columns, inline_align: inline_align, align_to_row: align_to_row, name: CowArg::Owned(name),
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn align_to_row(self, align_to_row: i32) -> Self {
        Self {
            align_to_row: align_to_row, .. self
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < GString > + 'a) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, columns, inline_align, align_to_row, name,
        }
        = self;
        re_export::RichTextLabel::push_table_full(surround_object, columns, inline_align, align_to_row, name,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::push_dropcap_ex`][super::RichTextLabel::push_dropcap_ex]."]
#[must_use]
pub struct ExPushDropcap < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, string: CowArg < 'a, GString >, font: CowArg < 'a, Option < Gd < crate::classes::Font > > >, size: i32, dropcap_margins: Rect2, color: Color, outline_size: i32, outline_color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushDropcap < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, string: impl AsArg < GString > + 'a, font: impl AsArg < Option < Gd < crate::classes::Font >> > + 'a, size: i32,) -> Self {
        let dropcap_margins = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let outline_size = 0i32;
        let outline_color = Color::from_rgba(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), font: font.into_arg(), size: size, dropcap_margins: dropcap_margins, color: color, outline_size: outline_size, outline_color: outline_color,
        }
    }
    #[inline]
    pub fn dropcap_margins(self, dropcap_margins: Rect2) -> Self {
        Self {
            dropcap_margins: dropcap_margins, .. self
        }
    }
    #[inline]
    pub fn color(self, color: Color) -> Self {
        Self {
            color: color, .. self
        }
    }
    #[inline]
    pub fn outline_size(self, outline_size: i32) -> Self {
        Self {
            outline_size: outline_size, .. self
        }
    }
    #[inline]
    pub fn outline_color(self, outline_color: Color) -> Self {
        Self {
            outline_color: outline_color, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, string, font, size, dropcap_margins, color, outline_size, outline_color,
        }
        = self;
        re_export::RichTextLabel::push_dropcap_full(surround_object, string, font, size, dropcap_margins, color, outline_size, outline_color,)
    }
}
#[doc = "Default-param extender for [`RichTextLabel::set_table_column_expand_ex`][super::RichTextLabel::set_table_column_expand_ex]."]
#[must_use]
pub struct ExSetTableColumnExpand < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RichTextLabel, column: i32, expand: bool, ratio: i32, shrink: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetTableColumnExpand < 'a > {
    fn new(surround_object: &'a mut re_export::RichTextLabel, column: i32, expand: bool,) -> Self {
        let ratio = 1i32;
        let shrink = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, expand: expand, ratio: ratio, shrink: shrink,
        }
    }
    #[inline]
    pub fn ratio(self, ratio: i32) -> Self {
        Self {
            ratio: ratio, .. self
        }
    }
    #[inline]
    pub fn shrink(self, shrink: bool) -> Self {
        Self {
            shrink: shrink, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, expand, ratio, shrink,
        }
        = self;
        re_export::RichTextLabel::set_table_column_expand_full(surround_object, column, expand, ratio, shrink,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ListType {
    ord: i32
}
impl ListType {
    #[doc(alias = "LIST_NUMBERS")]
    #[doc = "Godot enumerator name: `LIST_NUMBERS`"]
    pub const NUMBERS: ListType = ListType {
        ord: 0i32
    };
    #[doc(alias = "LIST_LETTERS")]
    #[doc = "Godot enumerator name: `LIST_LETTERS`"]
    pub const LETTERS: ListType = ListType {
        ord: 1i32
    };
    #[doc(alias = "LIST_ROMAN")]
    #[doc = "Godot enumerator name: `LIST_ROMAN`"]
    pub const ROMAN: ListType = ListType {
        ord: 2i32
    };
    #[doc(alias = "LIST_DOTS")]
    #[doc = "Godot enumerator name: `LIST_DOTS`"]
    pub const DOTS: ListType = ListType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ListType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ListType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ListType {
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
            Self::NUMBERS => "NUMBERS", Self::LETTERS => "LETTERS", Self::ROMAN => "ROMAN", Self::DOTS => "DOTS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ListType::NUMBERS, ListType::LETTERS, ListType::ROMAN, ListType::DOTS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ListType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NUMBERS", "LIST_NUMBERS", ListType::NUMBERS), crate::meta::inspect::EnumConstant::new("LETTERS", "LIST_LETTERS", ListType::LETTERS), crate::meta::inspect::EnumConstant::new("ROMAN", "LIST_ROMAN", ListType::ROMAN), crate::meta::inspect::EnumConstant::new("DOTS", "LIST_DOTS", ListType::DOTS)]
        }
    }
}
impl crate::meta::GodotConvert for ListType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ListType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ListType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MenuItems {
    ord: i32
}
impl MenuItems {
    #[doc(alias = "MENU_COPY")]
    #[doc = "Godot enumerator name: `MENU_COPY`"]
    pub const COPY: MenuItems = MenuItems {
        ord: 0i32
    };
    #[doc(alias = "MENU_SELECT_ALL")]
    #[doc = "Godot enumerator name: `MENU_SELECT_ALL`"]
    pub const SELECT_ALL: MenuItems = MenuItems {
        ord: 1i32
    };
    #[doc(alias = "MENU_MAX")]
    #[doc = "Godot enumerator name: `MENU_MAX`"]
    pub const MAX: MenuItems = MenuItems {
        ord: 2i32
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
            Self::COPY => "COPY", Self::SELECT_ALL => "SELECT_ALL", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MenuItems::COPY, MenuItems::SELECT_ALL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MenuItems >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("COPY", "MENU_COPY", MenuItems::COPY), crate::meta::inspect::EnumConstant::new("SELECT_ALL", "MENU_SELECT_ALL", MenuItems::SELECT_ALL), crate::meta::inspect::EnumConstant::new("MAX", "MENU_MAX", MenuItems::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for MenuItems {
    const ENUMERATOR_COUNT: usize = 2usize;
    
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
pub struct MetaUnderline {
    ord: i32
}
impl MetaUnderline {
    #[doc(alias = "META_UNDERLINE_NEVER")]
    #[doc = "Godot enumerator name: `META_UNDERLINE_NEVER`"]
    pub const NEVER: MetaUnderline = MetaUnderline {
        ord: 0i32
    };
    #[doc(alias = "META_UNDERLINE_ALWAYS")]
    #[doc = "Godot enumerator name: `META_UNDERLINE_ALWAYS`"]
    pub const ALWAYS: MetaUnderline = MetaUnderline {
        ord: 1i32
    };
    #[doc(alias = "META_UNDERLINE_ON_HOVER")]
    #[doc = "Godot enumerator name: `META_UNDERLINE_ON_HOVER`"]
    pub const ON_HOVER: MetaUnderline = MetaUnderline {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for MetaUnderline {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MetaUnderline") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MetaUnderline {
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
            Self::NEVER => "NEVER", Self::ALWAYS => "ALWAYS", Self::ON_HOVER => "ON_HOVER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MetaUnderline::NEVER, MetaUnderline::ALWAYS, MetaUnderline::ON_HOVER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MetaUnderline >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEVER", "META_UNDERLINE_NEVER", MetaUnderline::NEVER), crate::meta::inspect::EnumConstant::new("ALWAYS", "META_UNDERLINE_ALWAYS", MetaUnderline::ALWAYS), crate::meta::inspect::EnumConstant::new("ON_HOVER", "META_UNDERLINE_ON_HOVER", MetaUnderline::ON_HOVER)]
        }
    }
}
impl crate::meta::GodotConvert for MetaUnderline {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MetaUnderline {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MetaUnderline {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ImageUpdateMask {
    ord: u64
}
impl ImageUpdateMask {
    #[doc(alias = "UPDATE_TEXTURE")]
    #[doc = "Godot enumerator name: `UPDATE_TEXTURE`"]
    pub const TEXTURE: ImageUpdateMask = ImageUpdateMask {
        ord: 1u64
    };
    #[doc(alias = "UPDATE_SIZE")]
    #[doc = "Godot enumerator name: `UPDATE_SIZE`"]
    pub const SIZE: ImageUpdateMask = ImageUpdateMask {
        ord: 2u64
    };
    #[doc(alias = "UPDATE_COLOR")]
    #[doc = "Godot enumerator name: `UPDATE_COLOR`"]
    pub const COLOR: ImageUpdateMask = ImageUpdateMask {
        ord: 4u64
    };
    #[doc(alias = "UPDATE_ALIGNMENT")]
    #[doc = "Godot enumerator name: `UPDATE_ALIGNMENT`"]
    pub const ALIGNMENT: ImageUpdateMask = ImageUpdateMask {
        ord: 8u64
    };
    #[doc(alias = "UPDATE_REGION")]
    #[doc = "Godot enumerator name: `UPDATE_REGION`"]
    pub const REGION: ImageUpdateMask = ImageUpdateMask {
        ord: 16u64
    };
    #[doc(alias = "UPDATE_PAD")]
    #[doc = "Godot enumerator name: `UPDATE_PAD`"]
    pub const PAD: ImageUpdateMask = ImageUpdateMask {
        ord: 32u64
    };
    #[doc(alias = "UPDATE_TOOLTIP")]
    #[doc = "Godot enumerator name: `UPDATE_TOOLTIP`"]
    pub const TOOLTIP: ImageUpdateMask = ImageUpdateMask {
        ord: 64u64
    };
    #[doc(alias = "UPDATE_WIDTH_IN_PERCENT")]
    #[doc = "Godot enumerator name: `UPDATE_WIDTH_IN_PERCENT`"]
    pub const WIDTH_IN_PERCENT: ImageUpdateMask = ImageUpdateMask {
        ord: 128u64
    };
    
}
impl std::fmt::Debug for ImageUpdateMask {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::TEXTURE => "TEXTURE", Self::SIZE => "SIZE", Self::COLOR => "COLOR", Self::ALIGNMENT => "ALIGNMENT", Self::REGION => "REGION", Self::PAD => "PAD", Self::TOOLTIP => "TOOLTIP", Self::WIDTH_IN_PERCENT => "WIDTH_IN_PERCENT", _ => {
                f.debug_struct("ImageUpdateMask") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for ImageUpdateMask {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ImageUpdateMask >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TEXTURE", "UPDATE_TEXTURE", ImageUpdateMask::TEXTURE), crate::meta::inspect::EnumConstant::new("SIZE", "UPDATE_SIZE", ImageUpdateMask::SIZE), crate::meta::inspect::EnumConstant::new("COLOR", "UPDATE_COLOR", ImageUpdateMask::COLOR), crate::meta::inspect::EnumConstant::new("ALIGNMENT", "UPDATE_ALIGNMENT", ImageUpdateMask::ALIGNMENT), crate::meta::inspect::EnumConstant::new("REGION", "UPDATE_REGION", ImageUpdateMask::REGION), crate::meta::inspect::EnumConstant::new("PAD", "UPDATE_PAD", ImageUpdateMask::PAD), crate::meta::inspect::EnumConstant::new("TOOLTIP", "UPDATE_TOOLTIP", ImageUpdateMask::TOOLTIP), crate::meta::inspect::EnumConstant::new("WIDTH_IN_PERCENT", "UPDATE_WIDTH_IN_PERCENT", ImageUpdateMask::WIDTH_IN_PERCENT)]
        }
    }
}
impl std::ops::BitOr for ImageUpdateMask {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for ImageUpdateMask {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for ImageUpdateMask {
    type Via = u64;
    
}
impl crate::meta::ToGodot for ImageUpdateMask {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ImageUpdateMask {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::RichTextLabel;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`RichTextLabel`][crate::classes::RichTextLabel] class."]
    pub struct SignalsOfRichTextLabel < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfRichTextLabel < 'c, C > {
        #[doc = "Signature: `(meta: Variant)`"]
        pub fn meta_clicked(&mut self) -> SigMetaClicked < 'c, C > {
            SigMetaClicked {
                typed: TypedSignal::extract(&mut self.__internal_obj, "meta_clicked")
            }
        }
        #[doc = "Signature: `(meta: Variant)`"]
        pub fn meta_hover_started(&mut self) -> SigMetaHoverStarted < 'c, C > {
            SigMetaHoverStarted {
                typed: TypedSignal::extract(&mut self.__internal_obj, "meta_hover_started")
            }
        }
        #[doc = "Signature: `(meta: Variant)`"]
        pub fn meta_hover_ended(&mut self) -> SigMetaHoverEnded < 'c, C > {
            SigMetaHoverEnded {
                typed: TypedSignal::extract(&mut self.__internal_obj, "meta_hover_ended")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn finished(&mut self) -> SigFinished < 'c, C > {
            SigFinished {
                typed: TypedSignal::extract(&mut self.__internal_obj, "finished")
            }
        }
    }
    type TypedSigMetaClicked < 'c, C > = TypedSignal < 'c, C, (Variant,) >;
    pub struct SigMetaClicked < 'c, C: WithSignals > {
        typed: TypedSigMetaClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMetaClicked < 'c, C > {
        pub fn emit(&mut self, meta: Variant,) {
            self.typed.emit_tuple((meta,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMetaClicked < 'c, C > {
        type Target = TypedSigMetaClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMetaClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMetaHoverStarted < 'c, C > = TypedSignal < 'c, C, (Variant,) >;
    pub struct SigMetaHoverStarted < 'c, C: WithSignals > {
        typed: TypedSigMetaHoverStarted < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMetaHoverStarted < 'c, C > {
        pub fn emit(&mut self, meta: Variant,) {
            self.typed.emit_tuple((meta,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMetaHoverStarted < 'c, C > {
        type Target = TypedSigMetaHoverStarted < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMetaHoverStarted < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMetaHoverEnded < 'c, C > = TypedSignal < 'c, C, (Variant,) >;
    pub struct SigMetaHoverEnded < 'c, C: WithSignals > {
        typed: TypedSigMetaHoverEnded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMetaHoverEnded < 'c, C > {
        pub fn emit(&mut self, meta: Variant,) {
            self.typed.emit_tuple((meta,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMetaHoverEnded < 'c, C > {
        type Target = TypedSigMetaHoverEnded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMetaHoverEnded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFinished < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFinished < 'c, C: WithSignals > {
        typed: TypedSigFinished < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFinished < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFinished < 'c, C > {
        type Target = TypedSigFinished < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFinished < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for RichTextLabel {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfRichTextLabel < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfRichTextLabel < 'c, C > {
        type Target = < < RichTextLabel as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = RichTextLabel;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfRichTextLabel < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = RichTextLabel;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}