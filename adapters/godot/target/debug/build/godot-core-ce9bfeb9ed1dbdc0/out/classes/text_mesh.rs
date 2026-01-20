#![doc = "Sidecar module for class [`TextMesh`][crate::classes::TextMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextMesh` enums](https://docs.godotengine.org/en/stable/classes/class_textmesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextMesh.`\n\nInherits [`PrimitiveMesh`][crate::classes::PrimitiveMesh].\n\nRelated symbols:\n\n* [`ITextMesh`][crate::classes::ITextMesh]: virtual methods\n\n\nSee also [Godot docs for `TextMesh`](https://docs.godotengine.org/en/stable/classes/class_textmesh.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TextMesh::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextMesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TextMesh`][crate::classes::TextMesh].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IPrimitiveMesh`][crate::classes::IPrimitiveMesh] > [`IMesh`][crate::classes::IMesh] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `TextMesh` methods](https://docs.godotengine.org/en/stable/classes/class_textmesh.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextMesh: crate::obj::GodotClass < Base = TextMesh > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn create_mesh_array(&self,) -> VarArray {
            unimplemented !()
        }
        fn get_surface_count(&self,) -> i32;
        fn surface_get_array_len(&self, index: i32,) -> i32;
        fn surface_get_array_index_len(&self, index: i32,) -> i32;
        fn surface_get_arrays(&self, index: i32,) -> VarArray;
        fn surface_get_blend_shape_arrays(&self, index: i32,) -> Array < VarArray >;
        fn surface_get_lods(&self, index: i32,) -> VarDictionary;
        fn surface_get_format(&self, index: i32,) -> u32;
        fn surface_get_primitive_type(&self, index: i32,) -> u32;
        fn surface_set_material(&mut self, index: i32, material: Option < Gd < crate::classes::Material > >,);
        fn surface_get_material(&self, index: i32,) -> Option < Gd < crate::classes::Material > >;
        fn get_blend_shape_count(&self,) -> i32;
        fn get_blend_shape_name(&self, index: i32,) -> StringName;
        fn set_blend_shape_name(&mut self, index: i32, name: StringName,);
        fn get_aabb(&self,) -> Aabb;
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
        fn get_rid(&self,) -> Rid {
            unimplemented !()
        }
        fn reset_state(&mut self,) {
            unimplemented !()
        }
        fn set_path_cache(&self, path: GString,) {
            unimplemented !()
        }
    }
    impl TextMesh {
        pub fn set_horizontal_alignment(&mut self, alignment: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::HorizontalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_horizontal_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallRet = crate::global::HorizontalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_horizontal_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertical_alignment(&mut self, alignment: crate::global::VerticalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::VerticalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_vertical_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertical_alignment(&self,) -> crate::global::VerticalAlignment {
            type CallRet = crate::global::VerticalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_vertical_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font(&mut self, font: impl AsArg < Option < Gd < crate::classes::Font >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Font > > >,);
            let args = (font.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font(&self,) -> Option < Gd < crate::classes::Font > > {
            type CallRet = Option < Gd < crate::classes::Font > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_size(&mut self, font_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_line_spacing(&mut self, line_spacing: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (line_spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_line_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_spacing(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_line_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, autowrap_mode: crate::classes::text_server::AutowrapMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::AutowrapMode,);
            let args = (autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autowrap_mode(&self,) -> crate::classes::text_server::AutowrapMode {
            type CallRet = crate::classes::text_server::AutowrapMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_justification_flags(&mut self, justification_flags: crate::classes::text_server::JustificationFlag,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::JustificationFlag,);
            let args = (justification_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_justification_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_justification_flags(&self,) -> crate::classes::text_server::JustificationFlag {
            type CallRet = crate::classes::text_server::JustificationFlag;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_justification_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_depth(&mut self, depth: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (depth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_depth", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_depth(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_depth", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_width(&mut self, width: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_width(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pixel_size(&mut self, pixel_size: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (pixel_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_pixel_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pixel_size(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_pixel_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_curve_step(&mut self, curve_step: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (curve_step,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_curve_step", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_curve_step(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_curve_step", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::classes::text_server::Direction,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::Direction,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::classes::text_server::Direction {
            type CallRet = crate::classes::text_server::Direction;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, parser: crate::classes::text_server::StructuredTextParser,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::StructuredTextParser,);
            let args = (parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self,) -> crate::classes::text_server::StructuredTextParser {
            type CallRet = crate::classes::text_server::StructuredTextParser;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: &VarArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
            let args = (RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "get_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_uppercase(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "set_uppercase", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_uppercase(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextMesh", "is_uppercase", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TextMesh {
        type Base = crate::classes::PrimitiveMesh;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TextMesh"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextMesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PrimitiveMesh > for TextMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Mesh > for TextMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for TextMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TextMesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextMesh {
        
    }
    impl crate::obj::cap::GodotDefault for TextMesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TextMesh {
        type Target = crate::classes::PrimitiveMesh;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TextMesh`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TextMesh__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TextMesh > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PrimitiveMesh > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Mesh > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TextMesh;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for TextMesh {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfResource < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}