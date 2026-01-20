#![doc = "Sidecar module for class [`FontFile`][crate::classes::FontFile].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FontFile` enums](https://docs.godotengine.org/en/stable/classes/class_fontfile.html#enumerations).\n\n"]
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
    #[doc = "Godot class `FontFile.`\n\nInherits [`Font`][crate::classes::Font].\n\nRelated symbols:\n\n* [`IFontFile`][crate::classes::IFontFile]: virtual methods\n\n\nSee also [Godot docs for `FontFile`](https://docs.godotengine.org/en/stable/classes/class_fontfile.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`FontFile::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FontFile {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`FontFile`][crate::classes::FontFile].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IFont`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `FontFile` methods](https://docs.godotengine.org/en/stable/classes/class_fontfile.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IFontFile: crate::obj::GodotClass < Base = FontFile > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl FontFile {
        pub fn load_bitmap_font(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3381usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "load_bitmap_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn load_dynamic_font(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3382usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "load_dynamic_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_data(&mut self, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3383usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_data(&self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3384usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_name(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3385usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_font_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_style_name(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3386usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_font_style_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_style(&mut self, style: crate::classes::text_server::FontStyle,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::FontStyle,);
            let args = (style,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3387usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_font_style", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_weight(&mut self, weight: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3388usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_font_weight", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_stretch(&mut self, stretch: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (stretch,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3389usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_font_stretch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_antialiasing(&mut self, antialiasing: crate::classes::text_server::FontAntialiasing,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::FontAntialiasing,);
            let args = (antialiasing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3390usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_antialiasing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_antialiasing(&self,) -> crate::classes::text_server::FontAntialiasing {
            type CallRet = crate::classes::text_server::FontAntialiasing;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3391usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_antialiasing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_disable_embedded_bitmaps(&mut self, disable_embedded_bitmaps: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (disable_embedded_bitmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3392usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_disable_embedded_bitmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_disable_embedded_bitmaps(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3393usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_disable_embedded_bitmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_generate_mipmaps(&mut self, generate_mipmaps: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (generate_mipmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3394usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_generate_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_generate_mipmaps(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3395usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_generate_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_multichannel_signed_distance_field(&mut self, msdf: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (msdf,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3396usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_multichannel_signed_distance_field", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_multichannel_signed_distance_field(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3397usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "is_multichannel_signed_distance_field", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_msdf_pixel_range(&mut self, msdf_pixel_range: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (msdf_pixel_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3398usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_msdf_pixel_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_msdf_pixel_range(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3399usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_msdf_pixel_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_msdf_size(&mut self, msdf_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (msdf_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3400usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_msdf_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_msdf_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3401usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_msdf_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fixed_size(&mut self, fixed_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (fixed_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3402usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_fixed_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fixed_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3403usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_fixed_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fixed_size_scale_mode(&mut self, fixed_size_scale_mode: crate::classes::text_server::FixedSizeScaleMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::FixedSizeScaleMode,);
            let args = (fixed_size_scale_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_fixed_size_scale_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fixed_size_scale_mode(&self,) -> crate::classes::text_server::FixedSizeScaleMode {
            type CallRet = crate::classes::text_server::FixedSizeScaleMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_fixed_size_scale_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_system_fallback(&mut self, allow_system_fallback: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow_system_fallback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_allow_system_fallback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_allow_system_fallback(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "is_allow_system_fallback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_force_autohinter(&mut self, force_autohinter: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (force_autohinter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_force_autohinter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_force_autohinter(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "is_force_autohinter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_modulate_color_glyphs(&mut self, modulate: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_modulate_color_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_modulate_color_glyphs(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "is_modulate_color_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hinting(&mut self, hinting: crate::classes::text_server::Hinting,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::Hinting,);
            let args = (hinting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_hinting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hinting(&self,) -> crate::classes::text_server::Hinting {
            type CallRet = crate::classes::text_server::Hinting;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_hinting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_subpixel_positioning(&mut self, subpixel_positioning: crate::classes::text_server::SubpixelPositioning,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::SubpixelPositioning,);
            let args = (subpixel_positioning,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_subpixel_positioning", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_subpixel_positioning(&self,) -> crate::classes::text_server::SubpixelPositioning {
            type CallRet = crate::classes::text_server::SubpixelPositioning;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_subpixel_positioning", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_keep_rounding_remainders(&mut self, keep_rounding_remainders: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (keep_rounding_remainders,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_keep_rounding_remainders", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_keep_rounding_remainders(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_keep_rounding_remainders", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_oversampling(&mut self, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_oversampling(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cache_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3420usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_cache(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3421usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "clear_cache", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_cache(&mut self, cache_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3422usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "remove_cache", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size_cache_list(&self, cache_index: i32,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (i32,);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3423usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_size_cache_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_size_cache(&mut self, cache_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3424usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "clear_size_cache", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_size_cache(&mut self, cache_index: i32, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3425usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "remove_size_cache", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_variation_coordinates(&mut self, cache_index: i32, variation_coordinates: &VarDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, VarDictionary >,);
            let args = (cache_index, RefArg::new(variation_coordinates),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3426usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_variation_coordinates", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_variation_coordinates(&self, cache_index: i32,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i32,);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3427usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_variation_coordinates", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_embolden(&mut self, cache_index: i32, strength: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (cache_index, strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3428usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_embolden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_embolden(&self, cache_index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3429usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_embolden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transform(&mut self, cache_index: i32, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (i32, Transform2D,);
            let args = (cache_index, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3430usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transform(&self, cache_index: i32,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = (i32,);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3431usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_extra_spacing(&mut self, cache_index: i32, spacing: crate::classes::text_server::SpacingType, value: i64,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::text_server::SpacingType, i64,);
            let args = (cache_index, spacing, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3432usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_extra_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_extra_spacing(&self, cache_index: i32, spacing: crate::classes::text_server::SpacingType,) -> i64 {
            type CallRet = i64;
            type CallParams = (i32, crate::classes::text_server::SpacingType,);
            let args = (cache_index, spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3433usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_extra_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_extra_baseline_offset(&mut self, cache_index: i32, baseline_offset: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (cache_index, baseline_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3434usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_extra_baseline_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_extra_baseline_offset(&self, cache_index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3435usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_extra_baseline_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_face_index(&mut self, cache_index: i32, face_index: i64,) {
            type CallRet = ();
            type CallParams = (i32, i64,);
            let args = (cache_index, face_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3436usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_face_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_face_index(&self, cache_index: i32,) -> i64 {
            type CallRet = i64;
            type CallParams = (i32,);
            let args = (cache_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3437usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_face_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cache_ascent(&mut self, cache_index: i32, size: i32, ascent: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (cache_index, size, ascent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3438usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_cache_ascent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cache_ascent(&self, cache_index: i32, size: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3439usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_ascent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cache_descent(&mut self, cache_index: i32, size: i32, descent: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (cache_index, size, descent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3440usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_cache_descent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cache_descent(&self, cache_index: i32, size: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3441usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_descent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cache_underline_position(&mut self, cache_index: i32, size: i32, underline_position: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (cache_index, size, underline_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3442usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_cache_underline_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cache_underline_position(&self, cache_index: i32, size: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3443usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_underline_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cache_underline_thickness(&mut self, cache_index: i32, size: i32, underline_thickness: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (cache_index, size, underline_thickness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3444usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_cache_underline_thickness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cache_underline_thickness(&self, cache_index: i32, size: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3445usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_underline_thickness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cache_scale(&mut self, cache_index: i32, size: i32, scale: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (cache_index, size, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3446usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_cache_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cache_scale(&self, cache_index: i32, size: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3447usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_cache_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_count(&self, cache_index: i32, size: Vector2i,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, Vector2i,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3448usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_texture_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_textures(&mut self, cache_index: i32, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3449usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "clear_textures", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_texture(&mut self, cache_index: i32, size: Vector2i, texture_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3450usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "remove_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_image(&mut self, cache_index: i32, size: Vector2i, texture_index: i32, image: impl AsArg < Option < Gd < crate::classes::Image >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, Vector2i, i32, CowArg < 'a0, Option < Gd < crate::classes::Image > > >,);
            let args = (cache_index, size, texture_index, image.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3451usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_texture_image", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_image(&self, cache_index: i32, size: Vector2i, texture_index: i32,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3452usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_texture_image", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_offsets(&mut self, cache_index: i32, size: Vector2i, texture_index: i32, offset: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, Vector2i, i32, RefArg < 'a0, PackedInt32Array >,);
            let args = (cache_index, size, texture_index, RefArg::new(offset),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3453usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_texture_offsets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_offsets(&self, cache_index: i32, size: Vector2i, texture_index: i32,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3454usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_texture_offsets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glyph_list(&self, cache_index: i32, size: Vector2i,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (i32, Vector2i,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3455usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_glyphs(&mut self, cache_index: i32, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3456usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "clear_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_glyph(&mut self, cache_index: i32, size: Vector2i, glyph: i32,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3457usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "remove_glyph", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glyph_advance(&mut self, cache_index: i32, size: i32, glyph: i32, advance: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, i32, i32, Vector2,);
            let args = (cache_index, size, glyph, advance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3458usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_glyph_advance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glyph_advance(&self, cache_index: i32, size: i32, glyph: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32, i32, i32,);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3459usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_advance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glyph_offset(&mut self, cache_index: i32, size: Vector2i, glyph: i32, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32, Vector2,);
            let args = (cache_index, size, glyph, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3460usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_glyph_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glyph_offset(&self, cache_index: i32, size: Vector2i, glyph: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3461usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glyph_size(&mut self, cache_index: i32, size: Vector2i, glyph: i32, gl_size: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32, Vector2,);
            let args = (cache_index, size, glyph, gl_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3462usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_glyph_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glyph_size(&self, cache_index: i32, size: Vector2i, glyph: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3463usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glyph_uv_rect(&mut self, cache_index: i32, size: Vector2i, glyph: i32, uv_rect: Rect2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32, Rect2,);
            let args = (cache_index, size, glyph, uv_rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3464usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_glyph_uv_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glyph_uv_rect(&self, cache_index: i32, size: Vector2i, glyph: i32,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3465usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_uv_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glyph_texture_idx(&mut self, cache_index: i32, size: Vector2i, glyph: i32, texture_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32, i32,);
            let args = (cache_index, size, glyph, texture_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3466usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_glyph_texture_idx", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glyph_texture_idx(&self, cache_index: i32, size: Vector2i, glyph: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3467usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_texture_idx", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_kerning_list(&self, cache_index: i32, size: i32,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (i32, i32,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3468usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_kerning_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_kerning_map(&mut self, cache_index: i32, size: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (cache_index, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3469usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "clear_kerning_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_kerning(&mut self, cache_index: i32, size: i32, glyph_pair: Vector2i,) {
            type CallRet = ();
            type CallParams = (i32, i32, Vector2i,);
            let args = (cache_index, size, glyph_pair,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3470usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "remove_kerning", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_kerning(&mut self, cache_index: i32, size: i32, glyph_pair: Vector2i, kerning: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, i32, Vector2i, Vector2,);
            let args = (cache_index, size, glyph_pair, kerning,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3471usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_kerning", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_kerning(&self, cache_index: i32, size: i32, glyph_pair: Vector2i,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32, i32, Vector2i,);
            let args = (cache_index, size, glyph_pair,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3472usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_kerning", Some(self.__validated_obj()), args,)
            }
        }
        pub fn render_range(&mut self, cache_index: i32, size: Vector2i, start: u32, end: u32,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, u32, u32,);
            let args = (cache_index, size, start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3473usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "render_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn render_glyph(&mut self, cache_index: i32, size: Vector2i, index: i32,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32,);
            let args = (cache_index, size, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3474usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "render_glyph", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_language_support_override(&mut self, language: impl AsArg < GString >, supported: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (language.into_arg(), supported,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3475usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_language_support_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_language_support_override(&self, language: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3476usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_language_support_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_language_support_override(&mut self, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3477usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "remove_language_support_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_language_support_overrides(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3478usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_language_support_overrides", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_script_support_override(&mut self, script: impl AsArg < GString >, supported: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (script.into_arg(), supported,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3479usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_script_support_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_script_support_override(&self, script: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3480usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_script_support_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_script_support_override(&mut self, script: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3481usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "remove_script_support_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_script_support_overrides(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3482usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_script_support_overrides", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_opentype_feature_overrides(&mut self, overrides: &VarDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >,);
            let args = (RefArg::new(overrides),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3483usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "set_opentype_feature_overrides", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_opentype_feature_overrides(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3484usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_opentype_feature_overrides", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glyph_index(&self, size: i32, char: u32, variation_selector: u32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, u32, u32,);
            let args = (size, char, variation_selector,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3485usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_glyph_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_char_from_glyph_index(&self, size: i32, glyph_index: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (i32, i32,);
            let args = (size, glyph_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3486usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FontFile", "get_char_from_glyph_index", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for FontFile {
        type Base = crate::classes::Font;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("FontFile"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for FontFile {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Font > for FontFile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for FontFile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for FontFile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for FontFile {
        
    }
    impl crate::obj::cap::GodotDefault for FontFile {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for FontFile {
        type Target = crate::classes::Font;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FontFile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`FontFile`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_FontFile__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::FontFile > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Font > for $Class {
                
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
    use super::re_export::FontFile;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for FontFile {
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