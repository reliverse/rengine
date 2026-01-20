#![doc = "Sidecar module for class [`LabelSettings`][crate::classes::LabelSettings].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `LabelSettings` enums](https://docs.godotengine.org/en/stable/classes/class_labelsettings.html#enumerations).\n\n"]
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
    #[doc = "Godot class `LabelSettings.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`label_settings`][crate::classes::label_settings]: sidecar module with related enum/flag types\n* [`ILabelSettings`][crate::classes::ILabelSettings]: virtual methods\n\n\nSee also [Godot docs for `LabelSettings`](https://docs.godotengine.org/en/stable/classes/class_labelsettings.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`LabelSettings::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct LabelSettings {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`LabelSettings`][crate::classes::LabelSettings].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `LabelSettings` methods](https://docs.godotengine.org/en/stable/classes/class_labelsettings.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILabelSettings: crate::obj::GodotClass < Base = LabelSettings > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl LabelSettings {
        pub fn set_line_spacing(&mut self, spacing: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4851usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_line_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_spacing(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4852usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_line_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_paragraph_spacing(&mut self, spacing: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4853usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_paragraph_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_paragraph_spacing(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4854usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_paragraph_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font(&mut self, font: impl AsArg < Option < Gd < crate::classes::Font >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Font > > >,);
            let args = (font.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4855usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font(&self,) -> Option < Gd < crate::classes::Font > > {
            type CallRet = Option < Gd < crate::classes::Font > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4856usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4857usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4858usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_font_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_font_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_outline_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4861usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_outline_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_outline_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4862usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_outline_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_outline_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4863usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_outline_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_outline_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4864usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_outline_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4865usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_shadow_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadow_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4866usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_shadow_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4867usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_shadow_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadow_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4868usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_shadow_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4869usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_shadow_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadow_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4870usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_shadow_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stacked_outline_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4871usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_stacked_outline_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stacked_outline_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4872usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_stacked_outline_count", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_stacked_outline_full(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4873usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "add_stacked_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_stacked_outline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_stacked_outline(&mut self,) {
            self.add_stacked_outline_ex() . done()
        }
        #[inline]
        pub fn add_stacked_outline_ex < 'a > (&'a mut self,) -> ExAddStackedOutline < 'a > {
            ExAddStackedOutline::new(self,)
        }
        pub fn move_stacked_outline(&mut self, from_index: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4874usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "move_stacked_outline", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_stacked_outline(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4875usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "remove_stacked_outline", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stacked_outline_size(&mut self, index: i32, size: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4876usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_stacked_outline_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stacked_outline_size(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4877usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_stacked_outline_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stacked_outline_color(&mut self, index: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4878usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_stacked_outline_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stacked_outline_color(&self, index: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4879usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_stacked_outline_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stacked_shadow_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4880usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_stacked_shadow_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stacked_shadow_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4881usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_stacked_shadow_count", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_stacked_shadow_full(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4882usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "add_stacked_shadow", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_stacked_shadow_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_stacked_shadow(&mut self,) {
            self.add_stacked_shadow_ex() . done()
        }
        #[inline]
        pub fn add_stacked_shadow_ex < 'a > (&'a mut self,) -> ExAddStackedShadow < 'a > {
            ExAddStackedShadow::new(self,)
        }
        pub fn move_stacked_shadow(&mut self, from_index: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4883usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "move_stacked_shadow", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_stacked_shadow(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4884usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "remove_stacked_shadow", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stacked_shadow_offset(&mut self, index: i32, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2,);
            let args = (index, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4885usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_stacked_shadow_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stacked_shadow_offset(&self, index: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4886usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_stacked_shadow_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stacked_shadow_color(&mut self, index: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4887usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_stacked_shadow_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stacked_shadow_color(&self, index: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4888usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_stacked_shadow_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stacked_shadow_outline_size(&mut self, index: i32, size: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4889usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "set_stacked_shadow_outline_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stacked_shadow_outline_size(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4890usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "LabelSettings", "get_stacked_shadow_outline_size", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for LabelSettings {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("LabelSettings"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for LabelSettings {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for LabelSettings {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for LabelSettings {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for LabelSettings {
        
    }
    impl crate::obj::cap::GodotDefault for LabelSettings {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for LabelSettings {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for LabelSettings {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`LabelSettings`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_LabelSettings__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::LabelSettings > for $Class {
                
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
#[doc = "Default-param extender for [`LabelSettings::add_stacked_outline_ex`][super::LabelSettings::add_stacked_outline_ex]."]
#[must_use]
pub struct ExAddStackedOutline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::LabelSettings, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddStackedOutline < 'a > {
    fn new(surround_object: &'a mut re_export::LabelSettings,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, index,
        }
        = self;
        re_export::LabelSettings::add_stacked_outline_full(surround_object, index,)
    }
}
#[doc = "Default-param extender for [`LabelSettings::add_stacked_shadow_ex`][super::LabelSettings::add_stacked_shadow_ex]."]
#[must_use]
pub struct ExAddStackedShadow < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::LabelSettings, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddStackedShadow < 'a > {
    fn new(surround_object: &'a mut re_export::LabelSettings,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, index,
        }
        = self;
        re_export::LabelSettings::add_stacked_shadow_full(surround_object, index,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::LabelSettings;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for LabelSettings {
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