#![doc = "Sidecar module for class [`TextServer`][crate::classes::TextServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextServer` enums](https://docs.godotengine.org/en/stable/classes/class_textserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextServer.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`text_server`][crate::classes::text_server]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `TextServer`](https://docs.godotengine.org/en/stable/classes/class_textserver.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<TextServer>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl TextServer {
        pub fn has_feature(&self, feature: crate::classes::text_server::Feature,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::text_server::Feature,);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "has_feature", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "get_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_features(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "get_features", Some(self.__validated_obj()), args,)
            }
        }
        pub fn load_support_data(&mut self, filename: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (filename.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "load_support_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_support_data_filename(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "get_support_data_filename", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_support_data_info(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "get_support_data_info", Some(self.__validated_obj()), args,)
            }
        }
        pub fn save_support_data(&self, filename: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (filename.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "save_support_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_support_data(&self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "get_support_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_locale_right_to_left(&self, locale: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "is_locale_right_to_left", Some(self.__validated_obj()), args,)
            }
        }
        pub fn name_to_tag(&self, name: impl AsArg < GString >,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "name_to_tag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tag_to_name(&self, tag: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (i64,);
            let args = (tag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "tag_to_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has(&mut self, rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "has", Some(self.__validated_obj()), args,)
            }
        }
        pub fn free_rid(&mut self, rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "free_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_font(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "create_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_font_linked_variation(&mut self, font_rid: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "create_font_linked_variation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_data(&mut self, font_rid: Rid, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, PackedByteArray >,);
            let args = (font_rid, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_face_index(&mut self, font_rid: Rid, face_index: i64,) {
            type CallRet = ();
            type CallParams = (Rid, i64,);
            let args = (font_rid, face_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_face_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_face_index(&self, font_rid: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_face_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_face_count(&self, font_rid: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_face_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_style(&mut self, font_rid: Rid, style: crate::classes::text_server::FontStyle,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::FontStyle,);
            let args = (font_rid, style,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_style", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_style(&self, font_rid: Rid,) -> crate::classes::text_server::FontStyle {
            type CallRet = crate::classes::text_server::FontStyle;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_style", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_name(&mut self, font_rid: Rid, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (font_rid, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_name(&self, font_rid: Rid,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_ot_name_strings(&self, font_rid: Rid,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_ot_name_strings", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_style_name(&mut self, font_rid: Rid, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (font_rid, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_style_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_style_name(&self, font_rid: Rid,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_style_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_weight(&mut self, font_rid: Rid, weight: i64,) {
            type CallRet = ();
            type CallParams = (Rid, i64,);
            let args = (font_rid, weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9280usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_weight", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_weight(&self, font_rid: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9281usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_weight", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_stretch(&mut self, font_rid: Rid, weight: i64,) {
            type CallRet = ();
            type CallParams = (Rid, i64,);
            let args = (font_rid, weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9282usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_stretch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_stretch(&self, font_rid: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9283usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_stretch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_antialiasing(&mut self, font_rid: Rid, antialiasing: crate::classes::text_server::FontAntialiasing,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::FontAntialiasing,);
            let args = (font_rid, antialiasing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9284usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_antialiasing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_antialiasing(&self, font_rid: Rid,) -> crate::classes::text_server::FontAntialiasing {
            type CallRet = crate::classes::text_server::FontAntialiasing;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9285usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_antialiasing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_disable_embedded_bitmaps(&mut self, font_rid: Rid, disable_embedded_bitmaps: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (font_rid, disable_embedded_bitmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9286usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_disable_embedded_bitmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_disable_embedded_bitmaps(&self, font_rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9287usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_disable_embedded_bitmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_generate_mipmaps(&mut self, font_rid: Rid, generate_mipmaps: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (font_rid, generate_mipmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9288usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_generate_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_generate_mipmaps(&self, font_rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9289usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_generate_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_multichannel_signed_distance_field(&mut self, font_rid: Rid, msdf: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (font_rid, msdf,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9290usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_multichannel_signed_distance_field", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_is_multichannel_signed_distance_field(&self, font_rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9291usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_is_multichannel_signed_distance_field", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_msdf_pixel_range(&mut self, font_rid: Rid, msdf_pixel_range: i64,) {
            type CallRet = ();
            type CallParams = (Rid, i64,);
            let args = (font_rid, msdf_pixel_range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9292usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_msdf_pixel_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_msdf_pixel_range(&self, font_rid: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9293usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_msdf_pixel_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_msdf_size(&mut self, font_rid: Rid, msdf_size: i64,) {
            type CallRet = ();
            type CallParams = (Rid, i64,);
            let args = (font_rid, msdf_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9294usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_msdf_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_msdf_size(&self, font_rid: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9295usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_msdf_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_fixed_size(&mut self, font_rid: Rid, fixed_size: i64,) {
            type CallRet = ();
            type CallParams = (Rid, i64,);
            let args = (font_rid, fixed_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9296usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_fixed_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_fixed_size(&self, font_rid: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9297usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_fixed_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_fixed_size_scale_mode(&mut self, font_rid: Rid, fixed_size_scale_mode: crate::classes::text_server::FixedSizeScaleMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::FixedSizeScaleMode,);
            let args = (font_rid, fixed_size_scale_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9298usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_fixed_size_scale_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_fixed_size_scale_mode(&self, font_rid: Rid,) -> crate::classes::text_server::FixedSizeScaleMode {
            type CallRet = crate::classes::text_server::FixedSizeScaleMode;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9299usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_fixed_size_scale_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_allow_system_fallback(&mut self, font_rid: Rid, allow_system_fallback: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (font_rid, allow_system_fallback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9300usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_allow_system_fallback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_is_allow_system_fallback(&self, font_rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9301usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_is_allow_system_fallback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_clear_system_fallback_cache(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9302usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_clear_system_fallback_cache", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_force_autohinter(&mut self, font_rid: Rid, force_autohinter: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (font_rid, force_autohinter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9303usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_force_autohinter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_is_force_autohinter(&self, font_rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9304usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_is_force_autohinter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_modulate_color_glyphs(&mut self, font_rid: Rid, force_autohinter: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (font_rid, force_autohinter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9305usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_modulate_color_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_is_modulate_color_glyphs(&self, font_rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_is_modulate_color_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_hinting(&mut self, font_rid: Rid, hinting: crate::classes::text_server::Hinting,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::Hinting,);
            let args = (font_rid, hinting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_hinting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_hinting(&self, font_rid: Rid,) -> crate::classes::text_server::Hinting {
            type CallRet = crate::classes::text_server::Hinting;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9308usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_hinting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_subpixel_positioning(&mut self, font_rid: Rid, subpixel_positioning: crate::classes::text_server::SubpixelPositioning,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::SubpixelPositioning,);
            let args = (font_rid, subpixel_positioning,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9309usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_subpixel_positioning", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_subpixel_positioning(&self, font_rid: Rid,) -> crate::classes::text_server::SubpixelPositioning {
            type CallRet = crate::classes::text_server::SubpixelPositioning;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9310usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_subpixel_positioning", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_keep_rounding_remainders(&mut self, font_rid: Rid, keep_rounding_remainders: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (font_rid, keep_rounding_remainders,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9311usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_keep_rounding_remainders", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_keep_rounding_remainders(&self, font_rid: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9312usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_keep_rounding_remainders", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_embolden(&mut self, font_rid: Rid, strength: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (font_rid, strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9313usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_embolden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_embolden(&self, font_rid: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9314usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_embolden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_spacing(&mut self, font_rid: Rid, spacing: crate::classes::text_server::SpacingType, value: i64,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::SpacingType, i64,);
            let args = (font_rid, spacing, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9315usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_spacing(&self, font_rid: Rid, spacing: crate::classes::text_server::SpacingType,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, crate::classes::text_server::SpacingType,);
            let args = (font_rid, spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9316usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_baseline_offset(&mut self, font_rid: Rid, baseline_offset: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (font_rid, baseline_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9317usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_baseline_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_baseline_offset(&self, font_rid: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9318usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_baseline_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_transform(&mut self, font_rid: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (font_rid, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9319usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_transform(&self, font_rid: Rid,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9320usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_variation_coordinates(&mut self, font_rid: Rid, variation_coordinates: &VarDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, VarDictionary >,);
            let args = (font_rid, RefArg::new(variation_coordinates),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9321usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_variation_coordinates", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_variation_coordinates(&self, font_rid: Rid,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_variation_coordinates", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_oversampling(&mut self, font_rid: Rid, oversampling: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (font_rid, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9323usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_oversampling(&self, font_rid: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_size_cache_list(&self, font_rid: Rid,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_size_cache_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_clear_size_cache(&mut self, font_rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_clear_size_cache", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_remove_size_cache(&mut self, font_rid: Rid, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_size_cache", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_size_cache_info(&self, font_rid: Rid,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_size_cache_info", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_ascent(&mut self, font_rid: Rid, size: i64, ascent: f64,) {
            type CallRet = ();
            type CallParams = (Rid, i64, f64,);
            let args = (font_rid, size, ascent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_ascent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_ascent(&self, font_rid: Rid, size: i64,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid, i64,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_ascent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_descent(&mut self, font_rid: Rid, size: i64, descent: f64,) {
            type CallRet = ();
            type CallParams = (Rid, i64, f64,);
            let args = (font_rid, size, descent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_descent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_descent(&self, font_rid: Rid, size: i64,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid, i64,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_descent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_underline_position(&mut self, font_rid: Rid, size: i64, underline_position: f64,) {
            type CallRet = ();
            type CallParams = (Rid, i64, f64,);
            let args = (font_rid, size, underline_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_underline_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_underline_position(&self, font_rid: Rid, size: i64,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid, i64,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_underline_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_underline_thickness(&mut self, font_rid: Rid, size: i64, underline_thickness: f64,) {
            type CallRet = ();
            type CallParams = (Rid, i64, f64,);
            let args = (font_rid, size, underline_thickness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_underline_thickness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_underline_thickness(&self, font_rid: Rid, size: i64,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid, i64,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_underline_thickness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_scale(&mut self, font_rid: Rid, size: i64, scale: f64,) {
            type CallRet = ();
            type CallParams = (Rid, i64, f64,);
            let args = (font_rid, size, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_scale(&self, font_rid: Rid, size: i64,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid, i64,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_texture_count(&self, font_rid: Rid, size: Vector2i,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, Vector2i,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_texture_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_clear_textures(&mut self, font_rid: Rid, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_clear_textures", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_remove_texture(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_texture_image(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64, image: impl AsArg < Option < Gd < crate::classes::Image >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, Vector2i, i64, CowArg < 'a0, Option < Gd < crate::classes::Image > > >,);
            let args = (font_rid, size, texture_index, image.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9342usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_texture_image", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_texture_image(&self, font_rid: Rid, size: Vector2i, texture_index: i64,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9343usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_texture_image", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_texture_offsets(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64, offset: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, Vector2i, i64, RefArg < 'a0, PackedInt32Array >,);
            let args = (font_rid, size, texture_index, RefArg::new(offset),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9344usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_texture_offsets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_texture_offsets(&self, font_rid: Rid, size: Vector2i, texture_index: i64,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, texture_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9345usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_texture_offsets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_glyph_list(&self, font_rid: Rid, size: Vector2i,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid, Vector2i,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9346usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_clear_glyphs(&mut self, font_rid: Rid, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9347usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_clear_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_remove_glyph(&mut self, font_rid: Rid, size: Vector2i, glyph: i64,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9348usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_glyph", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_glyph_advance(&self, font_rid: Rid, size: i64, glyph: i64,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid, i64, i64,);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9349usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_advance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_glyph_advance(&mut self, font_rid: Rid, size: i64, glyph: i64, advance: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, i64, i64, Vector2,);
            let args = (font_rid, size, glyph, advance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9350usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_glyph_advance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_glyph_offset(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9351usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_glyph_offset(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i, i64, Vector2,);
            let args = (font_rid, size, glyph, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9352usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_glyph_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_glyph_size(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9353usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_glyph_size(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, gl_size: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i, i64, Vector2,);
            let args = (font_rid, size, glyph, gl_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9354usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_glyph_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_glyph_uv_rect(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9355usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_uv_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_glyph_uv_rect(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, uv_rect: Rect2,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i, i64, Rect2,);
            let args = (font_rid, size, glyph, uv_rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9356usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_glyph_uv_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_glyph_texture_idx(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9357usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_texture_idx", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_glyph_texture_idx(&mut self, font_rid: Rid, size: Vector2i, glyph: i64, texture_idx: i64,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i, i64, i64,);
            let args = (font_rid, size, glyph, texture_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9358usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_glyph_texture_idx", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_glyph_texture_rid(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_texture_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_glyph_texture_size(&self, font_rid: Rid, size: Vector2i, glyph: i64,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, glyph,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_texture_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_glyph_contours(&self, font: Rid, size: i64, index: i64,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (Rid, i64, i64,);
            let args = (font, size, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9361usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_contours", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_kerning_list(&self, font_rid: Rid, size: i64,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (Rid, i64,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9362usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_kerning_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_clear_kerning_map(&mut self, font_rid: Rid, size: i64,) {
            type CallRet = ();
            type CallParams = (Rid, i64,);
            let args = (font_rid, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9363usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_clear_kerning_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_remove_kerning(&mut self, font_rid: Rid, size: i64, glyph_pair: Vector2i,) {
            type CallRet = ();
            type CallParams = (Rid, i64, Vector2i,);
            let args = (font_rid, size, glyph_pair,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9364usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_kerning", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_kerning(&mut self, font_rid: Rid, size: i64, glyph_pair: Vector2i, kerning: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, i64, Vector2i, Vector2,);
            let args = (font_rid, size, glyph_pair, kerning,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9365usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_kerning", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_kerning(&self, font_rid: Rid, size: i64, glyph_pair: Vector2i,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid, i64, Vector2i,);
            let args = (font_rid, size, glyph_pair,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9366usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_kerning", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_glyph_index(&self, font_rid: Rid, size: i64, char: i64, variation_selector: i64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, i64, i64, i64,);
            let args = (font_rid, size, char, variation_selector,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9367usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_glyph_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_char_from_glyph_index(&self, font_rid: Rid, size: i64, glyph_index: i64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, i64, i64,);
            let args = (font_rid, size, glyph_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9368usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_char_from_glyph_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_has_char(&self, font_rid: Rid, char: i64,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid, i64,);
            let args = (font_rid, char,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9369usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_has_char", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_supported_chars(&self, font_rid: Rid,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9370usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_supported_chars", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_supported_glyphs(&self, font_rid: Rid,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9371usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_supported_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_render_range(&mut self, font_rid: Rid, size: Vector2i, start: i64, end: i64,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i, i64, i64,);
            let args = (font_rid, size, start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9372usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_render_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_render_glyph(&mut self, font_rid: Rid, size: Vector2i, index: i64,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2i, i64,);
            let args = (font_rid, size, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9373usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_render_glyph", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn font_draw_glyph_full(&self, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64, color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, i64, Vector2, i64, Color, f32,);
            let args = (font_rid, canvas, size, pos, index, color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9374usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_draw_glyph", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::font_draw_glyph_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn font_draw_glyph(&self, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64,) {
            self.font_draw_glyph_ex(font_rid, canvas, size, pos, index,) . done()
        }
        #[inline]
        pub fn font_draw_glyph_ex < 'a > (&'a self, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64,) -> ExFontDrawGlyph < 'a > {
            ExFontDrawGlyph::new(self, font_rid, canvas, size, pos, index,)
        }
        pub(crate) fn font_draw_glyph_outline_full(&self, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64, color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, i64, i64, Vector2, i64, Color, f32,);
            let args = (font_rid, canvas, size, outline_size, pos, index, color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9375usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_draw_glyph_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::font_draw_glyph_outline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn font_draw_glyph_outline(&self, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64,) {
            self.font_draw_glyph_outline_ex(font_rid, canvas, size, outline_size, pos, index,) . done()
        }
        #[inline]
        pub fn font_draw_glyph_outline_ex < 'a > (&'a self, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64,) -> ExFontDrawGlyphOutline < 'a > {
            ExFontDrawGlyphOutline::new(self, font_rid, canvas, size, outline_size, pos, index,)
        }
        pub fn font_is_language_supported(&self, font_rid: Rid, language: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (font_rid, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9376usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_is_language_supported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_language_support_override(&mut self, font_rid: Rid, language: impl AsArg < GString >, supported: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >, bool,);
            let args = (font_rid, language.into_arg(), supported,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9377usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_language_support_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_language_support_override(&mut self, font_rid: Rid, language: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (font_rid, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9378usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_language_support_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_remove_language_support_override(&mut self, font_rid: Rid, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (font_rid, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9379usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_language_support_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_language_support_overrides(&mut self, font_rid: Rid,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9380usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_language_support_overrides", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_is_script_supported(&self, font_rid: Rid, script: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (font_rid, script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9381usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_is_script_supported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_script_support_override(&mut self, font_rid: Rid, script: impl AsArg < GString >, supported: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >, bool,);
            let args = (font_rid, script.into_arg(), supported,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9382usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_script_support_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_script_support_override(&mut self, font_rid: Rid, script: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (font_rid, script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9383usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_script_support_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_remove_script_support_override(&mut self, font_rid: Rid, script: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (font_rid, script.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9384usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_remove_script_support_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_script_support_overrides(&mut self, font_rid: Rid,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9385usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_script_support_overrides", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_opentype_feature_overrides(&mut self, font_rid: Rid, overrides: &VarDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, VarDictionary >,);
            let args = (font_rid, RefArg::new(overrides),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9386usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_opentype_feature_overrides", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_opentype_feature_overrides(&self, font_rid: Rid,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9387usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_opentype_feature_overrides", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_supported_feature_list(&self, font_rid: Rid,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9388usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_supported_feature_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_supported_variation_list(&self, font_rid: Rid,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (Rid,);
            let args = (font_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9389usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_supported_variation_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_get_global_oversampling(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9390usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_get_global_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn font_set_global_oversampling(&mut self, oversampling: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9391usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "font_set_global_oversampling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hex_code_box_size(&self, size: i64, index: i64,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i64, i64,);
            let args = (size, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9392usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "get_hex_code_box_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn draw_hex_code_box(&self, canvas: Rid, size: i64, pos: Vector2, index: i64, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, i64, Vector2, i64, Color,);
            let args = (canvas, size, pos, index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9393usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "draw_hex_code_box", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn create_shaped_text_full(&mut self, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,) -> Rid {
            type CallRet = Rid;
            type CallParams = (crate::classes::text_server::Direction, crate::classes::text_server::Orientation,);
            let args = (direction, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9394usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "create_shaped_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_shaped_text_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_shaped_text(&mut self,) -> Rid {
            self.create_shaped_text_ex() . done()
        }
        #[inline]
        pub fn create_shaped_text_ex < 'a > (&'a mut self,) -> ExCreateShapedText < 'a > {
            ExCreateShapedText::new(self,)
        }
        pub fn shaped_text_clear(&mut self, rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9395usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_clear", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn shaped_text_set_direction_full(&mut self, shaped: Rid, direction: crate::classes::text_server::Direction,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::Direction,);
            let args = (shaped, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9396usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_direction", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_set_direction_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_set_direction(&mut self, shaped: Rid,) {
            self.shaped_text_set_direction_ex(shaped,) . done()
        }
        #[inline]
        pub fn shaped_text_set_direction_ex < 'a > (&'a mut self, shaped: Rid,) -> ExShapedTextSetDirection < 'a > {
            ExShapedTextSetDirection::new(self, shaped,)
        }
        pub fn shaped_text_get_direction(&self, shaped: Rid,) -> crate::classes::text_server::Direction {
            type CallRet = crate::classes::text_server::Direction;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9397usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_inferred_direction(&self, shaped: Rid,) -> crate::classes::text_server::Direction {
            type CallRet = crate::classes::text_server::Direction;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9398usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_inferred_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_set_bidi_override(&mut self, shaped: Rid, override_: &VarArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, VarArray >,);
            let args = (shaped, RefArg::new(override_),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9399usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_set_custom_punctuation(&mut self, shaped: Rid, punct: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (shaped, punct.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9400usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_custom_punctuation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_custom_punctuation(&self, shaped: Rid,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9401usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_custom_punctuation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_set_custom_ellipsis(&mut self, shaped: Rid, char: i64,) {
            type CallRet = ();
            type CallParams = (Rid, i64,);
            let args = (shaped, char,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9402usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_custom_ellipsis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_custom_ellipsis(&self, shaped: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9403usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_custom_ellipsis", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn shaped_text_set_orientation_full(&mut self, shaped: Rid, orientation: crate::classes::text_server::Orientation,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::Orientation,);
            let args = (shaped, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_orientation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_set_orientation_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_set_orientation(&mut self, shaped: Rid,) {
            self.shaped_text_set_orientation_ex(shaped,) . done()
        }
        #[inline]
        pub fn shaped_text_set_orientation_ex < 'a > (&'a mut self, shaped: Rid,) -> ExShapedTextSetOrientation < 'a > {
            ExShapedTextSetOrientation::new(self, shaped,)
        }
        pub fn shaped_text_get_orientation(&self, shaped: Rid,) -> crate::classes::text_server::Orientation {
            type CallRet = crate::classes::text_server::Orientation;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_orientation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_set_preserve_invalid(&mut self, shaped: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (shaped, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_preserve_invalid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_preserve_invalid(&self, shaped: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_preserve_invalid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_set_preserve_control(&mut self, shaped: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (shaped, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_preserve_control", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_preserve_control(&self, shaped: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_preserve_control", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_set_spacing(&mut self, shaped: Rid, spacing: crate::classes::text_server::SpacingType, value: i64,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::text_server::SpacingType, i64,);
            let args = (shaped, spacing, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_set_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_spacing(&self, shaped: Rid, spacing: crate::classes::text_server::SpacingType,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, crate::classes::text_server::SpacingType,);
            let args = (shaped, spacing,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_spacing", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn shaped_text_add_string_full(&mut self, shaped: Rid, text: CowArg < GString >, fonts: RefArg < Array < Rid > >, size: i64, opentype_features: RefArg < VarDictionary >, language: CowArg < GString >, meta: RefArg < Variant >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (Rid, CowArg < 'a0, GString >, RefArg < 'a1, Array < Rid > >, i64, RefArg < 'a2, VarDictionary >, CowArg < 'a3, GString >, RefArg < 'a4, Variant >,);
            let args = (shaped, text, fonts, size, opentype_features, language, meta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_add_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_add_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_add_string(&mut self, shaped: Rid, text: impl AsArg < GString >, fonts: &Array < Rid >, size: i64,) -> bool {
            self.shaped_text_add_string_ex(shaped, text, fonts, size,) . done()
        }
        #[inline]
        pub fn shaped_text_add_string_ex < 'a > (&'a mut self, shaped: Rid, text: impl AsArg < GString > + 'a, fonts: &'a Array < Rid >, size: i64,) -> ExShapedTextAddString < 'a > {
            ExShapedTextAddString::new(self, shaped, text, fonts, size,)
        }
        pub(crate) fn shaped_text_add_object_full(&mut self, shaped: Rid, key: RefArg < Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, length: i64, baseline: f64,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >, Vector2, crate::global::InlineAlignment, i64, f64,);
            let args = (shaped, key, size, inline_align, length, baseline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_add_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_add_object_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_add_object(&mut self, shaped: Rid, key: &Variant, size: Vector2,) -> bool {
            self.shaped_text_add_object_ex(shaped, key, size,) . done()
        }
        #[inline]
        pub fn shaped_text_add_object_ex < 'a > (&'a mut self, shaped: Rid, key: &'a Variant, size: Vector2,) -> ExShapedTextAddObject < 'a > {
            ExShapedTextAddObject::new(self, shaped, key, size,)
        }
        pub(crate) fn shaped_text_resize_object_full(&mut self, shaped: Rid, key: RefArg < Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, baseline: f64,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >, Vector2, crate::global::InlineAlignment, f64,);
            let args = (shaped, key, size, inline_align, baseline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_resize_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_resize_object_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_resize_object(&mut self, shaped: Rid, key: &Variant, size: Vector2,) -> bool {
            self.shaped_text_resize_object_ex(shaped, key, size,) . done()
        }
        #[inline]
        pub fn shaped_text_resize_object_ex < 'a > (&'a mut self, shaped: Rid, key: &'a Variant, size: Vector2,) -> ExShapedTextResizeObject < 'a > {
            ExShapedTextResizeObject::new(self, shaped, key, size,)
        }
        pub fn shaped_get_text(&self, shaped: Rid,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_get_span_count(&self, shaped: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_span_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_get_span_meta(&self, shaped: Rid, index: i64,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_span_meta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_get_span_embedded_object(&self, shaped: Rid, index: i64,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_span_embedded_object", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_get_span_text(&self, shaped: Rid, index: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_span_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_get_span_object(&self, shaped: Rid, index: i64,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9420usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_span_object", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn shaped_set_span_update_font_full(&mut self, shaped: Rid, index: i64, fonts: RefArg < Array < Rid > >, size: i64, opentype_features: RefArg < VarDictionary >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, i64, RefArg < 'a0, Array < Rid > >, i64, RefArg < 'a1, VarDictionary >,);
            let args = (shaped, index, fonts, size, opentype_features,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9421usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_set_span_update_font", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_set_span_update_font_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_set_span_update_font(&mut self, shaped: Rid, index: i64, fonts: &Array < Rid >, size: i64,) {
            self.shaped_set_span_update_font_ex(shaped, index, fonts, size,) . done()
        }
        #[inline]
        pub fn shaped_set_span_update_font_ex < 'a > (&'a mut self, shaped: Rid, index: i64, fonts: &'a Array < Rid >, size: i64,) -> ExShapedSetSpanUpdateFont < 'a > {
            ExShapedSetSpanUpdateFont::new(self, shaped, index, fonts, size,)
        }
        pub fn shaped_get_run_count(&self, shaped: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9422usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_run_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_get_run_text(&self, shaped: Rid, index: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9423usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_run_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_get_run_range(&self, shaped: Rid, index: i64,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9424usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_run_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_get_run_font_rid(&self, shaped: Rid, index: i64,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9425usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_run_font_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_get_run_font_size(&self, shaped: Rid, index: i64,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9426usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_run_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_get_run_language(&self, shaped: Rid, index: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9427usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_run_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_get_run_direction(&self, shaped: Rid, index: i64,) -> crate::classes::text_server::Direction {
            type CallRet = crate::classes::text_server::Direction;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9428usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_run_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_get_run_object(&self, shaped: Rid, index: i64,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Rid, i64,);
            let args = (shaped, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9429usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_get_run_object", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_substr(&self, shaped: Rid, start: i64, length: i64,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, i64, i64,);
            let args = (shaped, start, length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9430usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_substr", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_parent(&self, shaped: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9431usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn shaped_text_fit_to_width_full(&mut self, shaped: Rid, width: f64, justification_flags: crate::classes::text_server::JustificationFlag,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid, f64, crate::classes::text_server::JustificationFlag,);
            let args = (shaped, width, justification_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9432usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_fit_to_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_fit_to_width_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_fit_to_width(&mut self, shaped: Rid, width: f64,) -> f64 {
            self.shaped_text_fit_to_width_ex(shaped, width,) . done()
        }
        #[inline]
        pub fn shaped_text_fit_to_width_ex < 'a > (&'a mut self, shaped: Rid, width: f64,) -> ExShapedTextFitToWidth < 'a > {
            ExShapedTextFitToWidth::new(self, shaped, width,)
        }
        pub fn shaped_text_tab_align(&mut self, shaped: Rid, tab_stops: &PackedFloat32Array,) -> f64 {
            type CallRet = f64;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, PackedFloat32Array >,);
            let args = (shaped, RefArg::new(tab_stops),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9433usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_tab_align", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_shape(&mut self, shaped: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9434usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_is_ready(&self, shaped: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9435usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_is_ready", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_has_visible_chars(&self, shaped: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9436usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_has_visible_chars", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_glyphs(&self, shaped: Rid,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9437usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_sort_logical(&mut self, shaped: Rid,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9438usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_sort_logical", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_glyph_count(&self, shaped: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9439usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_glyph_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_range(&self, shaped: Rid,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9440usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_range", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn shaped_text_get_line_breaks_adv_full(&self, shaped: Rid, width: RefArg < PackedFloat32Array >, start: i64, once: bool, break_flags: crate::classes::text_server::LineBreakFlag,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, PackedFloat32Array >, i64, bool, crate::classes::text_server::LineBreakFlag,);
            let args = (shaped, width, start, once, break_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9441usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_line_breaks_adv", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_get_line_breaks_adv_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_get_line_breaks_adv(&self, shaped: Rid, width: &PackedFloat32Array,) -> PackedInt32Array {
            self.shaped_text_get_line_breaks_adv_ex(shaped, width,) . done()
        }
        #[inline]
        pub fn shaped_text_get_line_breaks_adv_ex < 'a > (&'a self, shaped: Rid, width: &'a PackedFloat32Array,) -> ExShapedTextGetLineBreaksAdv < 'a > {
            ExShapedTextGetLineBreaksAdv::new(self, shaped, width,)
        }
        pub(crate) fn shaped_text_get_line_breaks_full(&self, shaped: Rid, width: f64, start: i64, break_flags: crate::classes::text_server::LineBreakFlag,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid, f64, i64, crate::classes::text_server::LineBreakFlag,);
            let args = (shaped, width, start, break_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9442usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_line_breaks", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_get_line_breaks_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_get_line_breaks(&self, shaped: Rid, width: f64,) -> PackedInt32Array {
            self.shaped_text_get_line_breaks_ex(shaped, width,) . done()
        }
        #[inline]
        pub fn shaped_text_get_line_breaks_ex < 'a > (&'a self, shaped: Rid, width: f64,) -> ExShapedTextGetLineBreaks < 'a > {
            ExShapedTextGetLineBreaks::new(self, shaped, width,)
        }
        pub(crate) fn shaped_text_get_word_breaks_full(&self, shaped: Rid, grapheme_flags: crate::classes::text_server::GraphemeFlag, skip_grapheme_flags: crate::classes::text_server::GraphemeFlag,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid, crate::classes::text_server::GraphemeFlag, crate::classes::text_server::GraphemeFlag,);
            let args = (shaped, grapheme_flags, skip_grapheme_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9443usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_word_breaks", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_get_word_breaks_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_get_word_breaks(&self, shaped: Rid,) -> PackedInt32Array {
            self.shaped_text_get_word_breaks_ex(shaped,) . done()
        }
        #[inline]
        pub fn shaped_text_get_word_breaks_ex < 'a > (&'a self, shaped: Rid,) -> ExShapedTextGetWordBreaks < 'a > {
            ExShapedTextGetWordBreaks::new(self, shaped,)
        }
        pub fn shaped_text_get_trim_pos(&self, shaped: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9444usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_trim_pos", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_ellipsis_pos(&self, shaped: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9445usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_ellipsis_pos", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_ellipsis_glyphs(&self, shaped: Rid,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9446usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_ellipsis_glyphs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_ellipsis_glyph_count(&self, shaped: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9447usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_ellipsis_glyph_count", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn shaped_text_overrun_trim_to_width_full(&mut self, shaped: Rid, width: f64, overrun_trim_flags: crate::classes::text_server::TextOverrunFlag,) {
            type CallRet = ();
            type CallParams = (Rid, f64, crate::classes::text_server::TextOverrunFlag,);
            let args = (shaped, width, overrun_trim_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9448usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_overrun_trim_to_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_overrun_trim_to_width_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_overrun_trim_to_width(&mut self, shaped: Rid,) {
            self.shaped_text_overrun_trim_to_width_ex(shaped,) . done()
        }
        #[inline]
        pub fn shaped_text_overrun_trim_to_width_ex < 'a > (&'a mut self, shaped: Rid,) -> ExShapedTextOverrunTrimToWidth < 'a > {
            ExShapedTextOverrunTrimToWidth::new(self, shaped,)
        }
        pub fn shaped_text_get_objects(&self, shaped: Rid,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9449usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_objects", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_object_rect(&self, shaped: Rid, key: &Variant,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >,);
            let args = (shaped, RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9450usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_object_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_object_range(&self, shaped: Rid, key: &Variant,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >,);
            let args = (shaped, RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9451usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_object_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_object_glyph(&self, shaped: Rid, key: &Variant,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Variant >,);
            let args = (shaped, RefArg::new(key),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9452usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_object_glyph", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_size(&self, shaped: Rid,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9453usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_ascent(&self, shaped: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9454usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_ascent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_descent(&self, shaped: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9455usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_descent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_width(&self, shaped: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9456usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_underline_position(&self, shaped: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9457usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_underline_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_underline_thickness(&self, shaped: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9458usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_underline_thickness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_carets(&self, shaped: Rid, position: i64,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (Rid, i64,);
            let args = (shaped, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9459usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_carets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_selection(&self, shaped: Rid, start: i64, end: i64,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = (Rid, i64, i64,);
            let args = (shaped, start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9460usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_selection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_hit_test_grapheme(&self, shaped: Rid, coords: f64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, f64,);
            let args = (shaped, coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9461usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_hit_test_grapheme", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_hit_test_position(&self, shaped: Rid, coords: f64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, f64,);
            let args = (shaped, coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9462usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_hit_test_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_grapheme_bounds(&self, shaped: Rid, pos: i64,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Rid, i64,);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9463usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_grapheme_bounds", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_next_grapheme_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, i64,);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9464usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_next_grapheme_pos", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_prev_grapheme_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, i64,);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9465usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_prev_grapheme_pos", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_get_character_breaks(&self, shaped: Rid,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid,);
            let args = (shaped,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9466usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_character_breaks", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_next_character_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, i64,);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9467usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_next_character_pos", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_prev_character_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, i64,);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9468usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_prev_character_pos", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shaped_text_closest_character_pos(&self, shaped: Rid, pos: i64,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid, i64,);
            let args = (shaped, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9469usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_closest_character_pos", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn shaped_text_draw_full(&self, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Vector2, f64, f64, Color, f32,);
            let args = (shaped, canvas, pos, clip_l, clip_r, color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9470usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_draw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_draw_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_draw(&self, shaped: Rid, canvas: Rid, pos: Vector2,) {
            self.shaped_text_draw_ex(shaped, canvas, pos,) . done()
        }
        #[inline]
        pub fn shaped_text_draw_ex < 'a > (&'a self, shaped: Rid, canvas: Rid, pos: Vector2,) -> ExShapedTextDraw < 'a > {
            ExShapedTextDraw::new(self, shaped, canvas, pos,)
        }
        pub(crate) fn shaped_text_draw_outline_full(&self, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, outline_size: i64, color: Color, oversampling: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Vector2, f64, f64, i64, Color, f32,);
            let args = (shaped, canvas, pos, clip_l, clip_r, outline_size, color, oversampling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9471usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_draw_outline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shaped_text_draw_outline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shaped_text_draw_outline(&self, shaped: Rid, canvas: Rid, pos: Vector2,) {
            self.shaped_text_draw_outline_ex(shaped, canvas, pos,) . done()
        }
        #[inline]
        pub fn shaped_text_draw_outline_ex < 'a > (&'a self, shaped: Rid, canvas: Rid, pos: Vector2,) -> ExShapedTextDrawOutline < 'a > {
            ExShapedTextDrawOutline::new(self, shaped, canvas, pos,)
        }
        pub fn shaped_text_get_dominant_direction_in_range(&self, shaped: Rid, start: i64, end: i64,) -> crate::classes::text_server::Direction {
            type CallRet = crate::classes::text_server::Direction;
            type CallParams = (Rid, i64, i64,);
            let args = (shaped, start, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9472usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "shaped_text_get_dominant_direction_in_range", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn format_number_full(&self, number: CowArg < GString >, language: CowArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (number, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9473usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "format_number", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::format_number_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn format_number(&self, number: impl AsArg < GString >,) -> GString {
            self.format_number_ex(number,) . done()
        }
        #[inline]
        pub fn format_number_ex < 'a > (&'a self, number: impl AsArg < GString > + 'a,) -> ExFormatNumber < 'a > {
            ExFormatNumber::new(self, number,)
        }
        pub(crate) fn parse_number_full(&self, number: CowArg < GString >, language: CowArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (number, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9474usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "parse_number", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::parse_number_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn parse_number(&self, number: impl AsArg < GString >,) -> GString {
            self.parse_number_ex(number,) . done()
        }
        #[inline]
        pub fn parse_number_ex < 'a > (&'a self, number: impl AsArg < GString > + 'a,) -> ExParseNumber < 'a > {
            ExParseNumber::new(self, number,)
        }
        pub(crate) fn percent_sign_full(&self, language: CowArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9475usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "percent_sign", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::percent_sign_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn percent_sign(&self,) -> GString {
            self.percent_sign_ex() . done()
        }
        #[inline]
        pub fn percent_sign_ex < 'a > (&'a self,) -> ExPercentSign < 'a > {
            ExPercentSign::new(self,)
        }
        pub(crate) fn string_get_word_breaks_full(&self, string: CowArg < GString >, language: CowArg < GString >, chars_per_line: i64,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, i64,);
            let args = (string, language, chars_per_line,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9476usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "string_get_word_breaks", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::string_get_word_breaks_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn string_get_word_breaks(&self, string: impl AsArg < GString >,) -> PackedInt32Array {
            self.string_get_word_breaks_ex(string,) . done()
        }
        #[inline]
        pub fn string_get_word_breaks_ex < 'a > (&'a self, string: impl AsArg < GString > + 'a,) -> ExStringGetWordBreaks < 'a > {
            ExStringGetWordBreaks::new(self, string,)
        }
        pub(crate) fn string_get_character_breaks_full(&self, string: CowArg < GString >, language: CowArg < GString >,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (string, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9477usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "string_get_character_breaks", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::string_get_character_breaks_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn string_get_character_breaks(&self, string: impl AsArg < GString >,) -> PackedInt32Array {
            self.string_get_character_breaks_ex(string,) . done()
        }
        #[inline]
        pub fn string_get_character_breaks_ex < 'a > (&'a self, string: impl AsArg < GString > + 'a,) -> ExStringGetCharacterBreaks < 'a > {
            ExStringGetCharacterBreaks::new(self, string,)
        }
        pub fn is_confusable(&self, string: impl AsArg < GString >, dict: &PackedStringArray,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >,);
            let args = (string.into_arg(), RefArg::new(dict),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9478usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "is_confusable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn spoof_check(&self, string: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9479usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "spoof_check", Some(self.__validated_obj()), args,)
            }
        }
        pub fn strip_diacritics(&self, string: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9480usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "strip_diacritics", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_valid_identifier(&self, string: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (string.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9481usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "is_valid_identifier", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_valid_letter(&self, unicode: u64,) -> bool {
            type CallRet = bool;
            type CallParams = (u64,);
            let args = (unicode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9482usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "is_valid_letter", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn string_to_upper_full(&self, string: CowArg < GString >, language: CowArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (string, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9483usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "string_to_upper", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::string_to_upper_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn string_to_upper(&self, string: impl AsArg < GString >,) -> GString {
            self.string_to_upper_ex(string,) . done()
        }
        #[inline]
        pub fn string_to_upper_ex < 'a > (&'a self, string: impl AsArg < GString > + 'a,) -> ExStringToUpper < 'a > {
            ExStringToUpper::new(self, string,)
        }
        pub(crate) fn string_to_lower_full(&self, string: CowArg < GString >, language: CowArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (string, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9484usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "string_to_lower", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::string_to_lower_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn string_to_lower(&self, string: impl AsArg < GString >,) -> GString {
            self.string_to_lower_ex(string,) . done()
        }
        #[inline]
        pub fn string_to_lower_ex < 'a > (&'a self, string: impl AsArg < GString > + 'a,) -> ExStringToLower < 'a > {
            ExStringToLower::new(self, string,)
        }
        pub(crate) fn string_to_title_full(&self, string: CowArg < GString >, language: CowArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (string, language,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9485usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "string_to_title", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::string_to_title_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn string_to_title(&self, string: impl AsArg < GString >,) -> GString {
            self.string_to_title_ex(string,) . done()
        }
        #[inline]
        pub fn string_to_title_ex < 'a > (&'a self, string: impl AsArg < GString > + 'a,) -> ExStringToTitle < 'a > {
            ExStringToTitle::new(self, string,)
        }
        pub fn parse_structured_text(&self, parser_type: crate::classes::text_server::StructuredTextParser, args: &VarArray, text: impl AsArg < GString >,) -> Array < Vector3i > {
            type CallRet = Array < Vector3i >;
            type CallParams < 'a0, 'a1, > = (crate::classes::text_server::StructuredTextParser, RefArg < 'a0, VarArray >, CowArg < 'a1, GString >,);
            let args = (parser_type, RefArg::new(args), text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9486usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServer", "parse_structured_text", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TextServer {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TextServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextServer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TextServer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextServer {
        
    }
    impl std::ops::Deref for TextServer {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TextServer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `TextServer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`TextServer::font_draw_glyph_ex`][super::TextServer::font_draw_glyph_ex]."]
#[must_use]
pub struct ExFontDrawGlyph < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64, color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFontDrawGlyph < 'a > {
    fn new(surround_object: &'a re_export::TextServer, font_rid: Rid, canvas: Rid, size: i64, pos: Vector2, index: i64,) -> Self {
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_rid: font_rid, canvas: canvas, size: size, pos: pos, index: index, color: color, oversampling: oversampling,
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
            _phantom, surround_object, font_rid, canvas, size, pos, index, color, oversampling,
        }
        = self;
        re_export::TextServer::font_draw_glyph_full(surround_object, font_rid, canvas, size, pos, index, color, oversampling,)
    }
}
#[doc = "Default-param extender for [`TextServer::font_draw_glyph_outline_ex`][super::TextServer::font_draw_glyph_outline_ex]."]
#[must_use]
pub struct ExFontDrawGlyphOutline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64, color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFontDrawGlyphOutline < 'a > {
    fn new(surround_object: &'a re_export::TextServer, font_rid: Rid, canvas: Rid, size: i64, outline_size: i64, pos: Vector2, index: i64,) -> Self {
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, font_rid: font_rid, canvas: canvas, size: size, outline_size: outline_size, pos: pos, index: index, color: color, oversampling: oversampling,
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
            _phantom, surround_object, font_rid, canvas, size, outline_size, pos, index, color, oversampling,
        }
        = self;
        re_export::TextServer::font_draw_glyph_outline_full(surround_object, font_rid, canvas, size, outline_size, pos, index, color, oversampling,)
    }
}
#[doc = "Default-param extender for [`TextServer::create_shaped_text_ex`][super::TextServer::create_shaped_text_ex]."]
#[must_use]
pub struct ExCreateShapedText < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, direction: crate::classes::text_server::Direction, orientation: crate::classes::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateShapedText < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer,) -> Self {
        let direction = crate::obj::EngineEnum::from_ord(0);
        let orientation = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, direction: direction, orientation: orientation,
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, direction, orientation,
        }
        = self;
        re_export::TextServer::create_shaped_text_full(surround_object, direction, orientation,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_set_direction_ex`][super::TextServer::shaped_text_set_direction_ex]."]
#[must_use]
pub struct ExShapedTextSetDirection < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, shaped: Rid, direction: crate::classes::text_server::Direction,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextSetDirection < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid,) -> Self {
        let direction = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, direction: direction,
        }
    }
    #[inline]
    pub fn direction(self, direction: crate::classes::text_server::Direction) -> Self {
        Self {
            direction: direction, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shaped, direction,
        }
        = self;
        re_export::TextServer::shaped_text_set_direction_full(surround_object, shaped, direction,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_set_orientation_ex`][super::TextServer::shaped_text_set_orientation_ex]."]
#[must_use]
pub struct ExShapedTextSetOrientation < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, shaped: Rid, orientation: crate::classes::text_server::Orientation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextSetOrientation < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid,) -> Self {
        let orientation = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, orientation: orientation,
        }
    }
    #[inline]
    pub fn orientation(self, orientation: crate::classes::text_server::Orientation) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shaped, orientation,
        }
        = self;
        re_export::TextServer::shaped_text_set_orientation_full(surround_object, shaped, orientation,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_add_string_ex`][super::TextServer::shaped_text_add_string_ex]."]
#[must_use]
pub struct ExShapedTextAddString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, shaped: Rid, text: CowArg < 'a, GString >, fonts: CowArg < 'a, Array < Rid > >, size: i64, opentype_features: CowArg < 'a, VarDictionary >, language: CowArg < 'a, GString >, meta: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextAddString < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid, text: impl AsArg < GString > + 'a, fonts: &'a Array < Rid >, size: i64,) -> Self {
        let opentype_features = VarDictionary::new();
        let language = GString::from("");
        let meta = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, text: text.into_arg(), fonts: CowArg::Borrowed(fonts), size: size, opentype_features: CowArg::Owned(opentype_features), language: CowArg::Owned(language), meta: CowArg::Owned(meta),
        }
    }
    #[inline]
    pub fn opentype_features(self, opentype_features: &'a VarDictionary) -> Self {
        Self {
            opentype_features: CowArg::Borrowed(opentype_features), .. self
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
            _phantom, surround_object, shaped, text, fonts, size, opentype_features, language, meta,
        }
        = self;
        re_export::TextServer::shaped_text_add_string_full(surround_object, shaped, text, fonts.cow_as_arg(), size, opentype_features.cow_as_arg(), language, meta.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_add_object_ex`][super::TextServer::shaped_text_add_object_ex]."]
#[must_use]
pub struct ExShapedTextAddObject < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, shaped: Rid, key: CowArg < 'a, Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, length: i64, baseline: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextAddObject < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid, key: &'a Variant, size: Vector2,) -> Self {
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let length = 1i64;
        let baseline = 0f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, key: CowArg::Borrowed(key), size: size, inline_align: inline_align, length: length, baseline: baseline,
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn length(self, length: i64) -> Self {
        Self {
            length: length, .. self
        }
    }
    #[inline]
    pub fn baseline(self, baseline: f64) -> Self {
        Self {
            baseline: baseline, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, shaped, key, size, inline_align, length, baseline,
        }
        = self;
        re_export::TextServer::shaped_text_add_object_full(surround_object, shaped, key.cow_as_arg(), size, inline_align, length, baseline,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_resize_object_ex`][super::TextServer::shaped_text_resize_object_ex]."]
#[must_use]
pub struct ExShapedTextResizeObject < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, shaped: Rid, key: CowArg < 'a, Variant >, size: Vector2, inline_align: crate::global::InlineAlignment, baseline: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextResizeObject < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid, key: &'a Variant, size: Vector2,) -> Self {
        let inline_align = crate::obj::EngineEnum::from_ord(5);
        let baseline = 0f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, key: CowArg::Borrowed(key), size: size, inline_align: inline_align, baseline: baseline,
        }
    }
    #[inline]
    pub fn inline_align(self, inline_align: crate::global::InlineAlignment) -> Self {
        Self {
            inline_align: inline_align, .. self
        }
    }
    #[inline]
    pub fn baseline(self, baseline: f64) -> Self {
        Self {
            baseline: baseline, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, shaped, key, size, inline_align, baseline,
        }
        = self;
        re_export::TextServer::shaped_text_resize_object_full(surround_object, shaped, key.cow_as_arg(), size, inline_align, baseline,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_set_span_update_font_ex`][super::TextServer::shaped_set_span_update_font_ex]."]
#[must_use]
pub struct ExShapedSetSpanUpdateFont < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, shaped: Rid, index: i64, fonts: CowArg < 'a, Array < Rid > >, size: i64, opentype_features: CowArg < 'a, VarDictionary >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedSetSpanUpdateFont < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid, index: i64, fonts: &'a Array < Rid >, size: i64,) -> Self {
        let opentype_features = VarDictionary::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, index: index, fonts: CowArg::Borrowed(fonts), size: size, opentype_features: CowArg::Owned(opentype_features),
        }
    }
    #[inline]
    pub fn opentype_features(self, opentype_features: &'a VarDictionary) -> Self {
        Self {
            opentype_features: CowArg::Borrowed(opentype_features), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shaped, index, fonts, size, opentype_features,
        }
        = self;
        re_export::TextServer::shaped_set_span_update_font_full(surround_object, shaped, index, fonts.cow_as_arg(), size, opentype_features.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_fit_to_width_ex`][super::TextServer::shaped_text_fit_to_width_ex]."]
#[must_use]
pub struct ExShapedTextFitToWidth < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, shaped: Rid, width: f64, justification_flags: crate::classes::text_server::JustificationFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextFitToWidth < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid, width: f64,) -> Self {
        let justification_flags = crate::obj::EngineBitfield::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, width: width, justification_flags: justification_flags,
        }
    }
    #[inline]
    pub fn justification_flags(self, justification_flags: crate::classes::text_server::JustificationFlag) -> Self {
        Self {
            justification_flags: justification_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f64 {
        let Self {
            _phantom, surround_object, shaped, width, justification_flags,
        }
        = self;
        re_export::TextServer::shaped_text_fit_to_width_full(surround_object, shaped, width, justification_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_get_line_breaks_adv_ex`][super::TextServer::shaped_text_get_line_breaks_adv_ex]."]
#[must_use]
pub struct ExShapedTextGetLineBreaksAdv < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, shaped: Rid, width: CowArg < 'a, PackedFloat32Array >, start: i64, once: bool, break_flags: crate::classes::text_server::LineBreakFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextGetLineBreaksAdv < 'a > {
    fn new(surround_object: &'a re_export::TextServer, shaped: Rid, width: &'a PackedFloat32Array,) -> Self {
        let start = 0i64;
        let once = true;
        let break_flags = crate::obj::EngineBitfield::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, width: CowArg::Borrowed(width), start: start, once: once, break_flags: break_flags,
        }
    }
    #[inline]
    pub fn start(self, start: i64) -> Self {
        Self {
            start: start, .. self
        }
    }
    #[inline]
    pub fn once(self, once: bool) -> Self {
        Self {
            once: once, .. self
        }
    }
    #[inline]
    pub fn break_flags(self, break_flags: crate::classes::text_server::LineBreakFlag) -> Self {
        Self {
            break_flags: break_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, shaped, width, start, once, break_flags,
        }
        = self;
        re_export::TextServer::shaped_text_get_line_breaks_adv_full(surround_object, shaped, width.cow_as_arg(), start, once, break_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_get_line_breaks_ex`][super::TextServer::shaped_text_get_line_breaks_ex]."]
#[must_use]
pub struct ExShapedTextGetLineBreaks < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, shaped: Rid, width: f64, start: i64, break_flags: crate::classes::text_server::LineBreakFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextGetLineBreaks < 'a > {
    fn new(surround_object: &'a re_export::TextServer, shaped: Rid, width: f64,) -> Self {
        let start = 0i64;
        let break_flags = crate::obj::EngineBitfield::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, width: width, start: start, break_flags: break_flags,
        }
    }
    #[inline]
    pub fn start(self, start: i64) -> Self {
        Self {
            start: start, .. self
        }
    }
    #[inline]
    pub fn break_flags(self, break_flags: crate::classes::text_server::LineBreakFlag) -> Self {
        Self {
            break_flags: break_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, shaped, width, start, break_flags,
        }
        = self;
        re_export::TextServer::shaped_text_get_line_breaks_full(surround_object, shaped, width, start, break_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_get_word_breaks_ex`][super::TextServer::shaped_text_get_word_breaks_ex]."]
#[must_use]
pub struct ExShapedTextGetWordBreaks < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, shaped: Rid, grapheme_flags: crate::classes::text_server::GraphemeFlag, skip_grapheme_flags: crate::classes::text_server::GraphemeFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextGetWordBreaks < 'a > {
    fn new(surround_object: &'a re_export::TextServer, shaped: Rid,) -> Self {
        let grapheme_flags = crate::obj::EngineBitfield::from_ord(264);
        let skip_grapheme_flags = crate::obj::EngineBitfield::from_ord(4);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, grapheme_flags: grapheme_flags, skip_grapheme_flags: skip_grapheme_flags,
        }
    }
    #[inline]
    pub fn grapheme_flags(self, grapheme_flags: crate::classes::text_server::GraphemeFlag) -> Self {
        Self {
            grapheme_flags: grapheme_flags, .. self
        }
    }
    #[inline]
    pub fn skip_grapheme_flags(self, skip_grapheme_flags: crate::classes::text_server::GraphemeFlag) -> Self {
        Self {
            skip_grapheme_flags: skip_grapheme_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, shaped, grapheme_flags, skip_grapheme_flags,
        }
        = self;
        re_export::TextServer::shaped_text_get_word_breaks_full(surround_object, shaped, grapheme_flags, skip_grapheme_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_overrun_trim_to_width_ex`][super::TextServer::shaped_text_overrun_trim_to_width_ex]."]
#[must_use]
pub struct ExShapedTextOverrunTrimToWidth < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TextServer, shaped: Rid, width: f64, overrun_trim_flags: crate::classes::text_server::TextOverrunFlag,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextOverrunTrimToWidth < 'a > {
    fn new(surround_object: &'a mut re_export::TextServer, shaped: Rid,) -> Self {
        let width = 0f64;
        let overrun_trim_flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, width: width, overrun_trim_flags: overrun_trim_flags,
        }
    }
    #[inline]
    pub fn width(self, width: f64) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn overrun_trim_flags(self, overrun_trim_flags: crate::classes::text_server::TextOverrunFlag) -> Self {
        Self {
            overrun_trim_flags: overrun_trim_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shaped, width, overrun_trim_flags,
        }
        = self;
        re_export::TextServer::shaped_text_overrun_trim_to_width_full(surround_object, shaped, width, overrun_trim_flags,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_draw_ex`][super::TextServer::shaped_text_draw_ex]."]
#[must_use]
pub struct ExShapedTextDraw < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextDraw < 'a > {
    fn new(surround_object: &'a re_export::TextServer, shaped: Rid, canvas: Rid, pos: Vector2,) -> Self {
        let clip_l = - 1f64;
        let clip_r = - 1f64;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, canvas: canvas, pos: pos, clip_l: clip_l, clip_r: clip_r, color: color, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn clip_l(self, clip_l: f64) -> Self {
        Self {
            clip_l: clip_l, .. self
        }
    }
    #[inline]
    pub fn clip_r(self, clip_r: f64) -> Self {
        Self {
            clip_r: clip_r, .. self
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
            _phantom, surround_object, shaped, canvas, pos, clip_l, clip_r, color, oversampling,
        }
        = self;
        re_export::TextServer::shaped_text_draw_full(surround_object, shaped, canvas, pos, clip_l, clip_r, color, oversampling,)
    }
}
#[doc = "Default-param extender for [`TextServer::shaped_text_draw_outline_ex`][super::TextServer::shaped_text_draw_outline_ex]."]
#[must_use]
pub struct ExShapedTextDrawOutline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, shaped: Rid, canvas: Rid, pos: Vector2, clip_l: f64, clip_r: f64, outline_size: i64, color: Color, oversampling: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShapedTextDrawOutline < 'a > {
    fn new(surround_object: &'a re_export::TextServer, shaped: Rid, canvas: Rid, pos: Vector2,) -> Self {
        let clip_l = - 1f64;
        let clip_r = - 1f64;
        let outline_size = 1i64;
        let color = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let oversampling = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shaped: shaped, canvas: canvas, pos: pos, clip_l: clip_l, clip_r: clip_r, outline_size: outline_size, color: color, oversampling: oversampling,
        }
    }
    #[inline]
    pub fn clip_l(self, clip_l: f64) -> Self {
        Self {
            clip_l: clip_l, .. self
        }
    }
    #[inline]
    pub fn clip_r(self, clip_r: f64) -> Self {
        Self {
            clip_r: clip_r, .. self
        }
    }
    #[inline]
    pub fn outline_size(self, outline_size: i64) -> Self {
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
            _phantom, surround_object, shaped, canvas, pos, clip_l, clip_r, outline_size, color, oversampling,
        }
        = self;
        re_export::TextServer::shaped_text_draw_outline_full(surround_object, shaped, canvas, pos, clip_l, clip_r, outline_size, color, oversampling,)
    }
}
#[doc = "Default-param extender for [`TextServer::format_number_ex`][super::TextServer::format_number_ex]."]
#[must_use]
pub struct ExFormatNumber < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, number: CowArg < 'a, GString >, language: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFormatNumber < 'a > {
    fn new(surround_object: &'a re_export::TextServer, number: impl AsArg < GString > + 'a,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, number: number.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, number, language,
        }
        = self;
        re_export::TextServer::format_number_full(surround_object, number, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::parse_number_ex`][super::TextServer::parse_number_ex]."]
#[must_use]
pub struct ExParseNumber < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, number: CowArg < 'a, GString >, language: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExParseNumber < 'a > {
    fn new(surround_object: &'a re_export::TextServer, number: impl AsArg < GString > + 'a,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, number: number.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, number, language,
        }
        = self;
        re_export::TextServer::parse_number_full(surround_object, number, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::percent_sign_ex`][super::TextServer::percent_sign_ex]."]
#[must_use]
pub struct ExPercentSign < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, language: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPercentSign < 'a > {
    fn new(surround_object: &'a re_export::TextServer,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, language,
        }
        = self;
        re_export::TextServer::percent_sign_full(surround_object, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_get_word_breaks_ex`][super::TextServer::string_get_word_breaks_ex]."]
#[must_use]
pub struct ExStringGetWordBreaks < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, string: CowArg < 'a, GString >, language: CowArg < 'a, GString >, chars_per_line: i64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStringGetWordBreaks < 'a > {
    fn new(surround_object: &'a re_export::TextServer, string: impl AsArg < GString > + 'a,) -> Self {
        let language = GString::from("");
        let chars_per_line = 0i64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), language: CowArg::Owned(language), chars_per_line: chars_per_line,
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn chars_per_line(self, chars_per_line: i64) -> Self {
        Self {
            chars_per_line: chars_per_line, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, string, language, chars_per_line,
        }
        = self;
        re_export::TextServer::string_get_word_breaks_full(surround_object, string, language, chars_per_line,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_get_character_breaks_ex`][super::TextServer::string_get_character_breaks_ex]."]
#[must_use]
pub struct ExStringGetCharacterBreaks < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, string: CowArg < 'a, GString >, language: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStringGetCharacterBreaks < 'a > {
    fn new(surround_object: &'a re_export::TextServer, string: impl AsArg < GString > + 'a,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt32Array {
        let Self {
            _phantom, surround_object, string, language,
        }
        = self;
        re_export::TextServer::string_get_character_breaks_full(surround_object, string, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_to_upper_ex`][super::TextServer::string_to_upper_ex]."]
#[must_use]
pub struct ExStringToUpper < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, string: CowArg < 'a, GString >, language: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStringToUpper < 'a > {
    fn new(surround_object: &'a re_export::TextServer, string: impl AsArg < GString > + 'a,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, string, language,
        }
        = self;
        re_export::TextServer::string_to_upper_full(surround_object, string, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_to_lower_ex`][super::TextServer::string_to_lower_ex]."]
#[must_use]
pub struct ExStringToLower < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, string: CowArg < 'a, GString >, language: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStringToLower < 'a > {
    fn new(surround_object: &'a re_export::TextServer, string: impl AsArg < GString > + 'a,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, string, language,
        }
        = self;
        re_export::TextServer::string_to_lower_full(surround_object, string, language,)
    }
}
#[doc = "Default-param extender for [`TextServer::string_to_title_ex`][super::TextServer::string_to_title_ex]."]
#[must_use]
pub struct ExStringToTitle < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TextServer, string: CowArg < 'a, GString >, language: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStringToTitle < 'a > {
    fn new(surround_object: &'a re_export::TextServer, string: impl AsArg < GString > + 'a,) -> Self {
        let language = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, string: string.into_arg(), language: CowArg::Owned(language),
        }
    }
    #[inline]
    pub fn language(self, language: impl AsArg < GString > + 'a) -> Self {
        Self {
            language: language.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, string, language,
        }
        = self;
        re_export::TextServer::string_to_title_full(surround_object, string, language,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FontAntialiasing {
    ord: i32
}
impl FontAntialiasing {
    #[doc(alias = "FONT_ANTIALIASING_NONE")]
    #[doc = "Godot enumerator name: `FONT_ANTIALIASING_NONE`"]
    pub const NONE: FontAntialiasing = FontAntialiasing {
        ord: 0i32
    };
    #[doc(alias = "FONT_ANTIALIASING_GRAY")]
    #[doc = "Godot enumerator name: `FONT_ANTIALIASING_GRAY`"]
    pub const GRAY: FontAntialiasing = FontAntialiasing {
        ord: 1i32
    };
    #[doc(alias = "FONT_ANTIALIASING_LCD")]
    #[doc = "Godot enumerator name: `FONT_ANTIALIASING_LCD`"]
    pub const LCD: FontAntialiasing = FontAntialiasing {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for FontAntialiasing {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FontAntialiasing") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FontAntialiasing {
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
            Self::NONE => "NONE", Self::GRAY => "GRAY", Self::LCD => "LCD", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FontAntialiasing::NONE, FontAntialiasing::GRAY, FontAntialiasing::LCD]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FontAntialiasing >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "FONT_ANTIALIASING_NONE", FontAntialiasing::NONE), crate::meta::inspect::EnumConstant::new("GRAY", "FONT_ANTIALIASING_GRAY", FontAntialiasing::GRAY), crate::meta::inspect::EnumConstant::new("LCD", "FONT_ANTIALIASING_LCD", FontAntialiasing::LCD)]
        }
    }
}
impl crate::meta::GodotConvert for FontAntialiasing {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FontAntialiasing {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FontAntialiasing {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `FontLCDSubpixelLayout`."]
pub struct FontLcdSubpixelLayout {
    ord: i32
}
impl FontLcdSubpixelLayout {
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_NONE")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_NONE`"]
    pub const NONE: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 0i32
    };
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_HRGB")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_HRGB`"]
    pub const HRGB: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 1i32
    };
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_HBGR")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_HBGR`"]
    pub const HBGR: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 2i32
    };
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_VRGB")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_VRGB`"]
    pub const VRGB: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 3i32
    };
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_VBGR")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_VBGR`"]
    pub const VBGR: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 4i32
    };
    #[doc(alias = "FONT_LCD_SUBPIXEL_LAYOUT_MAX")]
    #[doc = "Godot enumerator name: `FONT_LCD_SUBPIXEL_LAYOUT_MAX`"]
    pub const MAX: FontLcdSubpixelLayout = FontLcdSubpixelLayout {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for FontLcdSubpixelLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FontLcdSubpixelLayout") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FontLcdSubpixelLayout {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::NONE => "NONE", Self::HRGB => "HRGB", Self::HBGR => "HBGR", Self::VRGB => "VRGB", Self::VBGR => "VBGR", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FontLcdSubpixelLayout::NONE, FontLcdSubpixelLayout::HRGB, FontLcdSubpixelLayout::HBGR, FontLcdSubpixelLayout::VRGB, FontLcdSubpixelLayout::VBGR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FontLcdSubpixelLayout >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "FONT_LCD_SUBPIXEL_LAYOUT_NONE", FontLcdSubpixelLayout::NONE), crate::meta::inspect::EnumConstant::new("HRGB", "FONT_LCD_SUBPIXEL_LAYOUT_HRGB", FontLcdSubpixelLayout::HRGB), crate::meta::inspect::EnumConstant::new("HBGR", "FONT_LCD_SUBPIXEL_LAYOUT_HBGR", FontLcdSubpixelLayout::HBGR), crate::meta::inspect::EnumConstant::new("VRGB", "FONT_LCD_SUBPIXEL_LAYOUT_VRGB", FontLcdSubpixelLayout::VRGB), crate::meta::inspect::EnumConstant::new("VBGR", "FONT_LCD_SUBPIXEL_LAYOUT_VBGR", FontLcdSubpixelLayout::VBGR), crate::meta::inspect::EnumConstant::new("MAX", "FONT_LCD_SUBPIXEL_LAYOUT_MAX", FontLcdSubpixelLayout::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for FontLcdSubpixelLayout {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for FontLcdSubpixelLayout {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FontLcdSubpixelLayout {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FontLcdSubpixelLayout {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Direction {
    ord: i32
}
impl Direction {
    #[doc(alias = "DIRECTION_AUTO")]
    #[doc = "Godot enumerator name: `DIRECTION_AUTO`"]
    pub const AUTO: Direction = Direction {
        ord: 0i32
    };
    #[doc(alias = "DIRECTION_LTR")]
    #[doc = "Godot enumerator name: `DIRECTION_LTR`"]
    pub const LTR: Direction = Direction {
        ord: 1i32
    };
    #[doc(alias = "DIRECTION_RTL")]
    #[doc = "Godot enumerator name: `DIRECTION_RTL`"]
    pub const RTL: Direction = Direction {
        ord: 2i32
    };
    #[doc(alias = "DIRECTION_INHERITED")]
    #[doc = "Godot enumerator name: `DIRECTION_INHERITED`"]
    pub const INHERITED: Direction = Direction {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Direction") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Direction {
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
            Self::AUTO => "AUTO", Self::LTR => "LTR", Self::RTL => "RTL", Self::INHERITED => "INHERITED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Direction::AUTO, Direction::LTR, Direction::RTL, Direction::INHERITED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Direction >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("AUTO", "DIRECTION_AUTO", Direction::AUTO), crate::meta::inspect::EnumConstant::new("LTR", "DIRECTION_LTR", Direction::LTR), crate::meta::inspect::EnumConstant::new("RTL", "DIRECTION_RTL", Direction::RTL), crate::meta::inspect::EnumConstant::new("INHERITED", "DIRECTION_INHERITED", Direction::INHERITED)]
        }
    }
}
impl crate::meta::GodotConvert for Direction {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Direction {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Direction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Orientation {
    ord: i32
}
impl Orientation {
    #[doc(alias = "ORIENTATION_HORIZONTAL")]
    #[doc = "Godot enumerator name: `ORIENTATION_HORIZONTAL`"]
    pub const HORIZONTAL: Orientation = Orientation {
        ord: 0i32
    };
    #[doc(alias = "ORIENTATION_VERTICAL")]
    #[doc = "Godot enumerator name: `ORIENTATION_VERTICAL`"]
    pub const VERTICAL: Orientation = Orientation {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Orientation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Orientation {
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
            Self::HORIZONTAL => "HORIZONTAL", Self::VERTICAL => "VERTICAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Orientation::HORIZONTAL, Orientation::VERTICAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Orientation >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("HORIZONTAL", "ORIENTATION_HORIZONTAL", Orientation::HORIZONTAL), crate::meta::inspect::EnumConstant::new("VERTICAL", "ORIENTATION_VERTICAL", Orientation::VERTICAL)]
        }
    }
}
impl crate::meta::GodotConvert for Orientation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Orientation {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Orientation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct JustificationFlag {
    ord: u64
}
impl JustificationFlag {
    #[doc(alias = "JUSTIFICATION_NONE")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_NONE`"]
    pub const NONE: JustificationFlag = JustificationFlag {
        ord: 0u64
    };
    #[doc(alias = "JUSTIFICATION_KASHIDA")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_KASHIDA`"]
    pub const KASHIDA: JustificationFlag = JustificationFlag {
        ord: 1u64
    };
    #[doc(alias = "JUSTIFICATION_WORD_BOUND")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_WORD_BOUND`"]
    pub const WORD_BOUND: JustificationFlag = JustificationFlag {
        ord: 2u64
    };
    #[doc(alias = "JUSTIFICATION_TRIM_EDGE_SPACES")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_TRIM_EDGE_SPACES`"]
    pub const TRIM_EDGE_SPACES: JustificationFlag = JustificationFlag {
        ord: 4u64
    };
    #[doc(alias = "JUSTIFICATION_AFTER_LAST_TAB")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_AFTER_LAST_TAB`"]
    pub const AFTER_LAST_TAB: JustificationFlag = JustificationFlag {
        ord: 8u64
    };
    #[doc(alias = "JUSTIFICATION_CONSTRAIN_ELLIPSIS")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_CONSTRAIN_ELLIPSIS`"]
    pub const CONSTRAIN_ELLIPSIS: JustificationFlag = JustificationFlag {
        ord: 16u64
    };
    #[doc(alias = "JUSTIFICATION_SKIP_LAST_LINE")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_SKIP_LAST_LINE`"]
    pub const SKIP_LAST_LINE: JustificationFlag = JustificationFlag {
        ord: 32u64
    };
    #[doc(alias = "JUSTIFICATION_SKIP_LAST_LINE_WITH_VISIBLE_CHARS")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_SKIP_LAST_LINE_WITH_VISIBLE_CHARS`"]
    pub const SKIP_LAST_LINE_WITH_VISIBLE_CHARS: JustificationFlag = JustificationFlag {
        ord: 64u64
    };
    #[doc(alias = "JUSTIFICATION_DO_NOT_SKIP_SINGLE_LINE")]
    #[doc = "Godot enumerator name: `JUSTIFICATION_DO_NOT_SKIP_SINGLE_LINE`"]
    pub const DO_NOT_SKIP_SINGLE_LINE: JustificationFlag = JustificationFlag {
        ord: 128u64
    };
    
}
impl std::fmt::Debug for JustificationFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::NONE => "NONE", Self::KASHIDA => "KASHIDA", Self::WORD_BOUND => "WORD_BOUND", Self::TRIM_EDGE_SPACES => "TRIM_EDGE_SPACES", Self::AFTER_LAST_TAB => "AFTER_LAST_TAB", Self::CONSTRAIN_ELLIPSIS => "CONSTRAIN_ELLIPSIS", Self::SKIP_LAST_LINE => "SKIP_LAST_LINE", Self::SKIP_LAST_LINE_WITH_VISIBLE_CHARS => "SKIP_LAST_LINE_WITH_VISIBLE_CHARS", Self::DO_NOT_SKIP_SINGLE_LINE => "DO_NOT_SKIP_SINGLE_LINE", _ => {
                f.debug_struct("JustificationFlag") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for JustificationFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < JustificationFlag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "JUSTIFICATION_NONE", JustificationFlag::NONE), crate::meta::inspect::EnumConstant::new("KASHIDA", "JUSTIFICATION_KASHIDA", JustificationFlag::KASHIDA), crate::meta::inspect::EnumConstant::new("WORD_BOUND", "JUSTIFICATION_WORD_BOUND", JustificationFlag::WORD_BOUND), crate::meta::inspect::EnumConstant::new("TRIM_EDGE_SPACES", "JUSTIFICATION_TRIM_EDGE_SPACES", JustificationFlag::TRIM_EDGE_SPACES), crate::meta::inspect::EnumConstant::new("AFTER_LAST_TAB", "JUSTIFICATION_AFTER_LAST_TAB", JustificationFlag::AFTER_LAST_TAB), crate::meta::inspect::EnumConstant::new("CONSTRAIN_ELLIPSIS", "JUSTIFICATION_CONSTRAIN_ELLIPSIS", JustificationFlag::CONSTRAIN_ELLIPSIS), crate::meta::inspect::EnumConstant::new("SKIP_LAST_LINE", "JUSTIFICATION_SKIP_LAST_LINE", JustificationFlag::SKIP_LAST_LINE), crate::meta::inspect::EnumConstant::new("SKIP_LAST_LINE_WITH_VISIBLE_CHARS", "JUSTIFICATION_SKIP_LAST_LINE_WITH_VISIBLE_CHARS", JustificationFlag::SKIP_LAST_LINE_WITH_VISIBLE_CHARS), crate::meta::inspect::EnumConstant::new("DO_NOT_SKIP_SINGLE_LINE", "JUSTIFICATION_DO_NOT_SKIP_SINGLE_LINE", JustificationFlag::DO_NOT_SKIP_SINGLE_LINE)]
        }
    }
}
impl std::ops::BitOr for JustificationFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for JustificationFlag {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for JustificationFlag {
    type Via = u64;
    
}
impl crate::meta::ToGodot for JustificationFlag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for JustificationFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AutowrapMode {
    ord: i32
}
impl AutowrapMode {
    #[doc(alias = "AUTOWRAP_OFF")]
    #[doc = "Godot enumerator name: `AUTOWRAP_OFF`"]
    pub const OFF: AutowrapMode = AutowrapMode {
        ord: 0i32
    };
    #[doc(alias = "AUTOWRAP_ARBITRARY")]
    #[doc = "Godot enumerator name: `AUTOWRAP_ARBITRARY`"]
    pub const ARBITRARY: AutowrapMode = AutowrapMode {
        ord: 1i32
    };
    #[doc(alias = "AUTOWRAP_WORD")]
    #[doc = "Godot enumerator name: `AUTOWRAP_WORD`"]
    pub const WORD: AutowrapMode = AutowrapMode {
        ord: 2i32
    };
    #[doc(alias = "AUTOWRAP_WORD_SMART")]
    #[doc = "Godot enumerator name: `AUTOWRAP_WORD_SMART`"]
    pub const WORD_SMART: AutowrapMode = AutowrapMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for AutowrapMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AutowrapMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AutowrapMode {
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
            Self::OFF => "OFF", Self::ARBITRARY => "ARBITRARY", Self::WORD => "WORD", Self::WORD_SMART => "WORD_SMART", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AutowrapMode::OFF, AutowrapMode::ARBITRARY, AutowrapMode::WORD, AutowrapMode::WORD_SMART]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AutowrapMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OFF", "AUTOWRAP_OFF", AutowrapMode::OFF), crate::meta::inspect::EnumConstant::new("ARBITRARY", "AUTOWRAP_ARBITRARY", AutowrapMode::ARBITRARY), crate::meta::inspect::EnumConstant::new("WORD", "AUTOWRAP_WORD", AutowrapMode::WORD), crate::meta::inspect::EnumConstant::new("WORD_SMART", "AUTOWRAP_WORD_SMART", AutowrapMode::WORD_SMART)]
        }
    }
}
impl crate::meta::GodotConvert for AutowrapMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AutowrapMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AutowrapMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct LineBreakFlag {
    ord: u64
}
impl LineBreakFlag {
    #[doc(alias = "BREAK_NONE")]
    #[doc = "Godot enumerator name: `BREAK_NONE`"]
    pub const NONE: LineBreakFlag = LineBreakFlag {
        ord: 0u64
    };
    #[doc(alias = "BREAK_MANDATORY")]
    #[doc = "Godot enumerator name: `BREAK_MANDATORY`"]
    pub const MANDATORY: LineBreakFlag = LineBreakFlag {
        ord: 1u64
    };
    #[doc(alias = "BREAK_WORD_BOUND")]
    #[doc = "Godot enumerator name: `BREAK_WORD_BOUND`"]
    pub const WORD_BOUND: LineBreakFlag = LineBreakFlag {
        ord: 2u64
    };
    #[doc(alias = "BREAK_GRAPHEME_BOUND")]
    #[doc = "Godot enumerator name: `BREAK_GRAPHEME_BOUND`"]
    pub const GRAPHEME_BOUND: LineBreakFlag = LineBreakFlag {
        ord: 4u64
    };
    #[doc(alias = "BREAK_ADAPTIVE")]
    #[doc = "Godot enumerator name: `BREAK_ADAPTIVE`"]
    pub const ADAPTIVE: LineBreakFlag = LineBreakFlag {
        ord: 8u64
    };
    #[doc(alias = "BREAK_TRIM_EDGE_SPACES")]
    #[doc = "Godot enumerator name: `BREAK_TRIM_EDGE_SPACES`"]
    pub const TRIM_EDGE_SPACES: LineBreakFlag = LineBreakFlag {
        ord: 16u64
    };
    #[doc(alias = "BREAK_TRIM_INDENT")]
    #[doc = "Godot enumerator name: `BREAK_TRIM_INDENT`"]
    pub const TRIM_INDENT: LineBreakFlag = LineBreakFlag {
        ord: 32u64
    };
    #[doc(alias = "BREAK_TRIM_START_EDGE_SPACES")]
    #[doc = "Godot enumerator name: `BREAK_TRIM_START_EDGE_SPACES`"]
    pub const TRIM_START_EDGE_SPACES: LineBreakFlag = LineBreakFlag {
        ord: 64u64
    };
    #[doc(alias = "BREAK_TRIM_END_EDGE_SPACES")]
    #[doc = "Godot enumerator name: `BREAK_TRIM_END_EDGE_SPACES`"]
    pub const TRIM_END_EDGE_SPACES: LineBreakFlag = LineBreakFlag {
        ord: 128u64
    };
    
}
impl std::fmt::Debug for LineBreakFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::NONE => "NONE", Self::MANDATORY => "MANDATORY", Self::WORD_BOUND => "WORD_BOUND", Self::GRAPHEME_BOUND => "GRAPHEME_BOUND", Self::ADAPTIVE => "ADAPTIVE", Self::TRIM_EDGE_SPACES => "TRIM_EDGE_SPACES", Self::TRIM_INDENT => "TRIM_INDENT", Self::TRIM_START_EDGE_SPACES => "TRIM_START_EDGE_SPACES", Self::TRIM_END_EDGE_SPACES => "TRIM_END_EDGE_SPACES", _ => {
                f.debug_struct("LineBreakFlag") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for LineBreakFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LineBreakFlag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "BREAK_NONE", LineBreakFlag::NONE), crate::meta::inspect::EnumConstant::new("MANDATORY", "BREAK_MANDATORY", LineBreakFlag::MANDATORY), crate::meta::inspect::EnumConstant::new("WORD_BOUND", "BREAK_WORD_BOUND", LineBreakFlag::WORD_BOUND), crate::meta::inspect::EnumConstant::new("GRAPHEME_BOUND", "BREAK_GRAPHEME_BOUND", LineBreakFlag::GRAPHEME_BOUND), crate::meta::inspect::EnumConstant::new("ADAPTIVE", "BREAK_ADAPTIVE", LineBreakFlag::ADAPTIVE), crate::meta::inspect::EnumConstant::new("TRIM_EDGE_SPACES", "BREAK_TRIM_EDGE_SPACES", LineBreakFlag::TRIM_EDGE_SPACES), crate::meta::inspect::EnumConstant::new("TRIM_INDENT", "BREAK_TRIM_INDENT", LineBreakFlag::TRIM_INDENT), crate::meta::inspect::EnumConstant::new("TRIM_START_EDGE_SPACES", "BREAK_TRIM_START_EDGE_SPACES", LineBreakFlag::TRIM_START_EDGE_SPACES), crate::meta::inspect::EnumConstant::new("TRIM_END_EDGE_SPACES", "BREAK_TRIM_END_EDGE_SPACES", LineBreakFlag::TRIM_END_EDGE_SPACES)]
        }
    }
}
impl std::ops::BitOr for LineBreakFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for LineBreakFlag {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for LineBreakFlag {
    type Via = u64;
    
}
impl crate::meta::ToGodot for LineBreakFlag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LineBreakFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VisibleCharactersBehavior {
    ord: i32
}
impl VisibleCharactersBehavior {
    #[doc(alias = "VC_CHARS_BEFORE_SHAPING")]
    #[doc = "Godot enumerator name: `VC_CHARS_BEFORE_SHAPING`"]
    pub const CHARS_BEFORE_SHAPING: VisibleCharactersBehavior = VisibleCharactersBehavior {
        ord: 0i32
    };
    #[doc(alias = "VC_CHARS_AFTER_SHAPING")]
    #[doc = "Godot enumerator name: `VC_CHARS_AFTER_SHAPING`"]
    pub const CHARS_AFTER_SHAPING: VisibleCharactersBehavior = VisibleCharactersBehavior {
        ord: 1i32
    };
    #[doc(alias = "VC_GLYPHS_AUTO")]
    #[doc = "Godot enumerator name: `VC_GLYPHS_AUTO`"]
    pub const GLYPHS_AUTO: VisibleCharactersBehavior = VisibleCharactersBehavior {
        ord: 2i32
    };
    #[doc(alias = "VC_GLYPHS_LTR")]
    #[doc = "Godot enumerator name: `VC_GLYPHS_LTR`"]
    pub const GLYPHS_LTR: VisibleCharactersBehavior = VisibleCharactersBehavior {
        ord: 3i32
    };
    #[doc(alias = "VC_GLYPHS_RTL")]
    #[doc = "Godot enumerator name: `VC_GLYPHS_RTL`"]
    pub const GLYPHS_RTL: VisibleCharactersBehavior = VisibleCharactersBehavior {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for VisibleCharactersBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VisibleCharactersBehavior") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VisibleCharactersBehavior {
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
            Self::CHARS_BEFORE_SHAPING => "CHARS_BEFORE_SHAPING", Self::CHARS_AFTER_SHAPING => "CHARS_AFTER_SHAPING", Self::GLYPHS_AUTO => "GLYPHS_AUTO", Self::GLYPHS_LTR => "GLYPHS_LTR", Self::GLYPHS_RTL => "GLYPHS_RTL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VisibleCharactersBehavior::CHARS_BEFORE_SHAPING, VisibleCharactersBehavior::CHARS_AFTER_SHAPING, VisibleCharactersBehavior::GLYPHS_AUTO, VisibleCharactersBehavior::GLYPHS_LTR, VisibleCharactersBehavior::GLYPHS_RTL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VisibleCharactersBehavior >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CHARS_BEFORE_SHAPING", "VC_CHARS_BEFORE_SHAPING", VisibleCharactersBehavior::CHARS_BEFORE_SHAPING), crate::meta::inspect::EnumConstant::new("CHARS_AFTER_SHAPING", "VC_CHARS_AFTER_SHAPING", VisibleCharactersBehavior::CHARS_AFTER_SHAPING), crate::meta::inspect::EnumConstant::new("GLYPHS_AUTO", "VC_GLYPHS_AUTO", VisibleCharactersBehavior::GLYPHS_AUTO), crate::meta::inspect::EnumConstant::new("GLYPHS_LTR", "VC_GLYPHS_LTR", VisibleCharactersBehavior::GLYPHS_LTR), crate::meta::inspect::EnumConstant::new("GLYPHS_RTL", "VC_GLYPHS_RTL", VisibleCharactersBehavior::GLYPHS_RTL)]
        }
    }
}
impl crate::meta::GodotConvert for VisibleCharactersBehavior {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VisibleCharactersBehavior {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VisibleCharactersBehavior {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct OverrunBehavior {
    ord: i32
}
impl OverrunBehavior {
    #[doc(alias = "OVERRUN_NO_TRIMMING")]
    #[doc = "Godot enumerator name: `OVERRUN_NO_TRIMMING`"]
    pub const NO_TRIMMING: OverrunBehavior = OverrunBehavior {
        ord: 0i32
    };
    #[doc(alias = "OVERRUN_TRIM_CHAR")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_CHAR`"]
    pub const TRIM_CHAR: OverrunBehavior = OverrunBehavior {
        ord: 1i32
    };
    #[doc(alias = "OVERRUN_TRIM_WORD")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_WORD`"]
    pub const TRIM_WORD: OverrunBehavior = OverrunBehavior {
        ord: 2i32
    };
    #[doc(alias = "OVERRUN_TRIM_ELLIPSIS")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_ELLIPSIS`"]
    pub const TRIM_ELLIPSIS: OverrunBehavior = OverrunBehavior {
        ord: 3i32
    };
    #[doc(alias = "OVERRUN_TRIM_WORD_ELLIPSIS")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_WORD_ELLIPSIS`"]
    pub const TRIM_WORD_ELLIPSIS: OverrunBehavior = OverrunBehavior {
        ord: 4i32
    };
    #[doc(alias = "OVERRUN_TRIM_ELLIPSIS_FORCE")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_ELLIPSIS_FORCE`"]
    pub const TRIM_ELLIPSIS_FORCE: OverrunBehavior = OverrunBehavior {
        ord: 5i32
    };
    #[doc(alias = "OVERRUN_TRIM_WORD_ELLIPSIS_FORCE")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_WORD_ELLIPSIS_FORCE`"]
    pub const TRIM_WORD_ELLIPSIS_FORCE: OverrunBehavior = OverrunBehavior {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for OverrunBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("OverrunBehavior") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for OverrunBehavior {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::NO_TRIMMING => "NO_TRIMMING", Self::TRIM_CHAR => "TRIM_CHAR", Self::TRIM_WORD => "TRIM_WORD", Self::TRIM_ELLIPSIS => "TRIM_ELLIPSIS", Self::TRIM_WORD_ELLIPSIS => "TRIM_WORD_ELLIPSIS", Self::TRIM_ELLIPSIS_FORCE => "TRIM_ELLIPSIS_FORCE", Self::TRIM_WORD_ELLIPSIS_FORCE => "TRIM_WORD_ELLIPSIS_FORCE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[OverrunBehavior::NO_TRIMMING, OverrunBehavior::TRIM_CHAR, OverrunBehavior::TRIM_WORD, OverrunBehavior::TRIM_ELLIPSIS, OverrunBehavior::TRIM_WORD_ELLIPSIS, OverrunBehavior::TRIM_ELLIPSIS_FORCE, OverrunBehavior::TRIM_WORD_ELLIPSIS_FORCE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < OverrunBehavior >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NO_TRIMMING", "OVERRUN_NO_TRIMMING", OverrunBehavior::NO_TRIMMING), crate::meta::inspect::EnumConstant::new("TRIM_CHAR", "OVERRUN_TRIM_CHAR", OverrunBehavior::TRIM_CHAR), crate::meta::inspect::EnumConstant::new("TRIM_WORD", "OVERRUN_TRIM_WORD", OverrunBehavior::TRIM_WORD), crate::meta::inspect::EnumConstant::new("TRIM_ELLIPSIS", "OVERRUN_TRIM_ELLIPSIS", OverrunBehavior::TRIM_ELLIPSIS), crate::meta::inspect::EnumConstant::new("TRIM_WORD_ELLIPSIS", "OVERRUN_TRIM_WORD_ELLIPSIS", OverrunBehavior::TRIM_WORD_ELLIPSIS), crate::meta::inspect::EnumConstant::new("TRIM_ELLIPSIS_FORCE", "OVERRUN_TRIM_ELLIPSIS_FORCE", OverrunBehavior::TRIM_ELLIPSIS_FORCE), crate::meta::inspect::EnumConstant::new("TRIM_WORD_ELLIPSIS_FORCE", "OVERRUN_TRIM_WORD_ELLIPSIS_FORCE", OverrunBehavior::TRIM_WORD_ELLIPSIS_FORCE)]
        }
    }
}
impl crate::meta::GodotConvert for OverrunBehavior {
    type Via = i32;
    
}
impl crate::meta::ToGodot for OverrunBehavior {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for OverrunBehavior {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct TextOverrunFlag {
    ord: u64
}
impl TextOverrunFlag {
    #[doc(alias = "OVERRUN_NO_TRIM")]
    #[doc = "Godot enumerator name: `OVERRUN_NO_TRIM`"]
    pub const NO_TRIM: TextOverrunFlag = TextOverrunFlag {
        ord: 0u64
    };
    #[doc(alias = "OVERRUN_TRIM")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM`"]
    pub const TRIM: TextOverrunFlag = TextOverrunFlag {
        ord: 1u64
    };
    #[doc(alias = "OVERRUN_TRIM_WORD_ONLY")]
    #[doc = "Godot enumerator name: `OVERRUN_TRIM_WORD_ONLY`"]
    pub const TRIM_WORD_ONLY: TextOverrunFlag = TextOverrunFlag {
        ord: 2u64
    };
    #[doc(alias = "OVERRUN_ADD_ELLIPSIS")]
    #[doc = "Godot enumerator name: `OVERRUN_ADD_ELLIPSIS`"]
    pub const ADD_ELLIPSIS: TextOverrunFlag = TextOverrunFlag {
        ord: 4u64
    };
    #[doc(alias = "OVERRUN_ENFORCE_ELLIPSIS")]
    #[doc = "Godot enumerator name: `OVERRUN_ENFORCE_ELLIPSIS`"]
    pub const ENFORCE_ELLIPSIS: TextOverrunFlag = TextOverrunFlag {
        ord: 8u64
    };
    #[doc(alias = "OVERRUN_JUSTIFICATION_AWARE")]
    #[doc = "Godot enumerator name: `OVERRUN_JUSTIFICATION_AWARE`"]
    pub const JUSTIFICATION_AWARE: TextOverrunFlag = TextOverrunFlag {
        ord: 16u64
    };
    
}
impl std::fmt::Debug for TextOverrunFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::NO_TRIM => "NO_TRIM", Self::TRIM => "TRIM", Self::TRIM_WORD_ONLY => "TRIM_WORD_ONLY", Self::ADD_ELLIPSIS => "ADD_ELLIPSIS", Self::ENFORCE_ELLIPSIS => "ENFORCE_ELLIPSIS", Self::JUSTIFICATION_AWARE => "JUSTIFICATION_AWARE", _ => {
                f.debug_struct("TextOverrunFlag") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for TextOverrunFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextOverrunFlag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NO_TRIM", "OVERRUN_NO_TRIM", TextOverrunFlag::NO_TRIM), crate::meta::inspect::EnumConstant::new("TRIM", "OVERRUN_TRIM", TextOverrunFlag::TRIM), crate::meta::inspect::EnumConstant::new("TRIM_WORD_ONLY", "OVERRUN_TRIM_WORD_ONLY", TextOverrunFlag::TRIM_WORD_ONLY), crate::meta::inspect::EnumConstant::new("ADD_ELLIPSIS", "OVERRUN_ADD_ELLIPSIS", TextOverrunFlag::ADD_ELLIPSIS), crate::meta::inspect::EnumConstant::new("ENFORCE_ELLIPSIS", "OVERRUN_ENFORCE_ELLIPSIS", TextOverrunFlag::ENFORCE_ELLIPSIS), crate::meta::inspect::EnumConstant::new("JUSTIFICATION_AWARE", "OVERRUN_JUSTIFICATION_AWARE", TextOverrunFlag::JUSTIFICATION_AWARE)]
        }
    }
}
impl std::ops::BitOr for TextOverrunFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for TextOverrunFlag {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for TextOverrunFlag {
    type Via = u64;
    
}
impl crate::meta::ToGodot for TextOverrunFlag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextOverrunFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct GraphemeFlag {
    ord: u64
}
impl GraphemeFlag {
    #[doc(alias = "GRAPHEME_IS_VALID")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_VALID`"]
    pub const VALID: GraphemeFlag = GraphemeFlag {
        ord: 1u64
    };
    #[doc(alias = "GRAPHEME_IS_RTL")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_RTL`"]
    pub const RTL: GraphemeFlag = GraphemeFlag {
        ord: 2u64
    };
    #[doc(alias = "GRAPHEME_IS_VIRTUAL")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_VIRTUAL`"]
    pub const VIRTUAL: GraphemeFlag = GraphemeFlag {
        ord: 4u64
    };
    #[doc(alias = "GRAPHEME_IS_SPACE")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_SPACE`"]
    pub const SPACE: GraphemeFlag = GraphemeFlag {
        ord: 8u64
    };
    #[doc(alias = "GRAPHEME_IS_BREAK_HARD")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_BREAK_HARD`"]
    pub const BREAK_HARD: GraphemeFlag = GraphemeFlag {
        ord: 16u64
    };
    #[doc(alias = "GRAPHEME_IS_BREAK_SOFT")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_BREAK_SOFT`"]
    pub const BREAK_SOFT: GraphemeFlag = GraphemeFlag {
        ord: 32u64
    };
    #[doc(alias = "GRAPHEME_IS_TAB")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_TAB`"]
    pub const TAB: GraphemeFlag = GraphemeFlag {
        ord: 64u64
    };
    #[doc(alias = "GRAPHEME_IS_ELONGATION")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_ELONGATION`"]
    pub const ELONGATION: GraphemeFlag = GraphemeFlag {
        ord: 128u64
    };
    #[doc(alias = "GRAPHEME_IS_PUNCTUATION")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_PUNCTUATION`"]
    pub const PUNCTUATION: GraphemeFlag = GraphemeFlag {
        ord: 256u64
    };
    #[doc(alias = "GRAPHEME_IS_UNDERSCORE")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_UNDERSCORE`"]
    pub const UNDERSCORE: GraphemeFlag = GraphemeFlag {
        ord: 512u64
    };
    #[doc(alias = "GRAPHEME_IS_CONNECTED")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_CONNECTED`"]
    pub const CONNECTED: GraphemeFlag = GraphemeFlag {
        ord: 1024u64
    };
    #[doc(alias = "GRAPHEME_IS_SAFE_TO_INSERT_TATWEEL")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_SAFE_TO_INSERT_TATWEEL`"]
    pub const SAFE_TO_INSERT_TATWEEL: GraphemeFlag = GraphemeFlag {
        ord: 2048u64
    };
    #[doc(alias = "GRAPHEME_IS_EMBEDDED_OBJECT")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_EMBEDDED_OBJECT`"]
    pub const EMBEDDED_OBJECT: GraphemeFlag = GraphemeFlag {
        ord: 4096u64
    };
    #[doc(alias = "GRAPHEME_IS_SOFT_HYPHEN")]
    #[doc = "Godot enumerator name: `GRAPHEME_IS_SOFT_HYPHEN`"]
    pub const SOFT_HYPHEN: GraphemeFlag = GraphemeFlag {
        ord: 8192u64
    };
    
}
impl std::fmt::Debug for GraphemeFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::VALID => "VALID", Self::RTL => "RTL", Self::VIRTUAL => "VIRTUAL", Self::SPACE => "SPACE", Self::BREAK_HARD => "BREAK_HARD", Self::BREAK_SOFT => "BREAK_SOFT", Self::TAB => "TAB", Self::ELONGATION => "ELONGATION", Self::PUNCTUATION => "PUNCTUATION", Self::UNDERSCORE => "UNDERSCORE", Self::CONNECTED => "CONNECTED", Self::SAFE_TO_INSERT_TATWEEL => "SAFE_TO_INSERT_TATWEEL", Self::EMBEDDED_OBJECT => "EMBEDDED_OBJECT", Self::SOFT_HYPHEN => "SOFT_HYPHEN", _ => {
                f.debug_struct("GraphemeFlag") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for GraphemeFlag {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GraphemeFlag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VALID", "GRAPHEME_IS_VALID", GraphemeFlag::VALID), crate::meta::inspect::EnumConstant::new("RTL", "GRAPHEME_IS_RTL", GraphemeFlag::RTL), crate::meta::inspect::EnumConstant::new("VIRTUAL", "GRAPHEME_IS_VIRTUAL", GraphemeFlag::VIRTUAL), crate::meta::inspect::EnumConstant::new("SPACE", "GRAPHEME_IS_SPACE", GraphemeFlag::SPACE), crate::meta::inspect::EnumConstant::new("BREAK_HARD", "GRAPHEME_IS_BREAK_HARD", GraphemeFlag::BREAK_HARD), crate::meta::inspect::EnumConstant::new("BREAK_SOFT", "GRAPHEME_IS_BREAK_SOFT", GraphemeFlag::BREAK_SOFT), crate::meta::inspect::EnumConstant::new("TAB", "GRAPHEME_IS_TAB", GraphemeFlag::TAB), crate::meta::inspect::EnumConstant::new("ELONGATION", "GRAPHEME_IS_ELONGATION", GraphemeFlag::ELONGATION), crate::meta::inspect::EnumConstant::new("PUNCTUATION", "GRAPHEME_IS_PUNCTUATION", GraphemeFlag::PUNCTUATION), crate::meta::inspect::EnumConstant::new("UNDERSCORE", "GRAPHEME_IS_UNDERSCORE", GraphemeFlag::UNDERSCORE), crate::meta::inspect::EnumConstant::new("CONNECTED", "GRAPHEME_IS_CONNECTED", GraphemeFlag::CONNECTED), crate::meta::inspect::EnumConstant::new("SAFE_TO_INSERT_TATWEEL", "GRAPHEME_IS_SAFE_TO_INSERT_TATWEEL", GraphemeFlag::SAFE_TO_INSERT_TATWEEL), crate::meta::inspect::EnumConstant::new("EMBEDDED_OBJECT", "GRAPHEME_IS_EMBEDDED_OBJECT", GraphemeFlag::EMBEDDED_OBJECT), crate::meta::inspect::EnumConstant::new("SOFT_HYPHEN", "GRAPHEME_IS_SOFT_HYPHEN", GraphemeFlag::SOFT_HYPHEN)]
        }
    }
}
impl std::ops::BitOr for GraphemeFlag {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for GraphemeFlag {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for GraphemeFlag {
    type Via = u64;
    
}
impl crate::meta::ToGodot for GraphemeFlag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GraphemeFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Hinting {
    ord: i32
}
impl Hinting {
    #[doc(alias = "HINTING_NONE")]
    #[doc = "Godot enumerator name: `HINTING_NONE`"]
    pub const NONE: Hinting = Hinting {
        ord: 0i32
    };
    #[doc(alias = "HINTING_LIGHT")]
    #[doc = "Godot enumerator name: `HINTING_LIGHT`"]
    pub const LIGHT: Hinting = Hinting {
        ord: 1i32
    };
    #[doc(alias = "HINTING_NORMAL")]
    #[doc = "Godot enumerator name: `HINTING_NORMAL`"]
    pub const NORMAL: Hinting = Hinting {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Hinting {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Hinting") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Hinting {
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
            Self::NONE => "NONE", Self::LIGHT => "LIGHT", Self::NORMAL => "NORMAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Hinting::NONE, Hinting::LIGHT, Hinting::NORMAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Hinting >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "HINTING_NONE", Hinting::NONE), crate::meta::inspect::EnumConstant::new("LIGHT", "HINTING_LIGHT", Hinting::LIGHT), crate::meta::inspect::EnumConstant::new("NORMAL", "HINTING_NORMAL", Hinting::NORMAL)]
        }
    }
}
impl crate::meta::GodotConvert for Hinting {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Hinting {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Hinting {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SubpixelPositioning {
    ord: i32
}
impl SubpixelPositioning {
    #[doc(alias = "SUBPIXEL_POSITIONING_DISABLED")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_DISABLED`"]
    pub const DISABLED: SubpixelPositioning = SubpixelPositioning {
        ord: 0i32
    };
    #[doc(alias = "SUBPIXEL_POSITIONING_AUTO")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_AUTO`"]
    pub const AUTO: SubpixelPositioning = SubpixelPositioning {
        ord: 1i32
    };
    #[doc(alias = "SUBPIXEL_POSITIONING_ONE_HALF")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_ONE_HALF`"]
    pub const ONE_HALF: SubpixelPositioning = SubpixelPositioning {
        ord: 2i32
    };
    #[doc(alias = "SUBPIXEL_POSITIONING_ONE_QUARTER")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_ONE_QUARTER`"]
    pub const ONE_QUARTER: SubpixelPositioning = SubpixelPositioning {
        ord: 3i32
    };
    #[doc(alias = "SUBPIXEL_POSITIONING_ONE_HALF_MAX_SIZE")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_ONE_HALF_MAX_SIZE`"]
    pub const ONE_HALF_MAX_SIZE: SubpixelPositioning = SubpixelPositioning {
        ord: 20i32
    };
    #[doc(alias = "SUBPIXEL_POSITIONING_ONE_QUARTER_MAX_SIZE")]
    #[doc = "Godot enumerator name: `SUBPIXEL_POSITIONING_ONE_QUARTER_MAX_SIZE`"]
    pub const ONE_QUARTER_MAX_SIZE: SubpixelPositioning = SubpixelPositioning {
        ord: 16i32
    };
    
}
impl std::fmt::Debug for SubpixelPositioning {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SubpixelPositioning") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SubpixelPositioning {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 16i32 | ord @ 20i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::AUTO => "AUTO", Self::ONE_HALF => "ONE_HALF", Self::ONE_QUARTER => "ONE_QUARTER", Self::ONE_HALF_MAX_SIZE => "ONE_HALF_MAX_SIZE", Self::ONE_QUARTER_MAX_SIZE => "ONE_QUARTER_MAX_SIZE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SubpixelPositioning::DISABLED, SubpixelPositioning::AUTO, SubpixelPositioning::ONE_HALF, SubpixelPositioning::ONE_QUARTER, SubpixelPositioning::ONE_HALF_MAX_SIZE, SubpixelPositioning::ONE_QUARTER_MAX_SIZE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SubpixelPositioning >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "SUBPIXEL_POSITIONING_DISABLED", SubpixelPositioning::DISABLED), crate::meta::inspect::EnumConstant::new("AUTO", "SUBPIXEL_POSITIONING_AUTO", SubpixelPositioning::AUTO), crate::meta::inspect::EnumConstant::new("ONE_HALF", "SUBPIXEL_POSITIONING_ONE_HALF", SubpixelPositioning::ONE_HALF), crate::meta::inspect::EnumConstant::new("ONE_QUARTER", "SUBPIXEL_POSITIONING_ONE_QUARTER", SubpixelPositioning::ONE_QUARTER), crate::meta::inspect::EnumConstant::new("ONE_HALF_MAX_SIZE", "SUBPIXEL_POSITIONING_ONE_HALF_MAX_SIZE", SubpixelPositioning::ONE_HALF_MAX_SIZE), crate::meta::inspect::EnumConstant::new("ONE_QUARTER_MAX_SIZE", "SUBPIXEL_POSITIONING_ONE_QUARTER_MAX_SIZE", SubpixelPositioning::ONE_QUARTER_MAX_SIZE)]
        }
    }
}
impl crate::meta::GodotConvert for SubpixelPositioning {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SubpixelPositioning {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SubpixelPositioning {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Feature {
    ord: i32
}
impl Feature {
    #[doc(alias = "FEATURE_SIMPLE_LAYOUT")]
    #[doc = "Godot enumerator name: `FEATURE_SIMPLE_LAYOUT`"]
    pub const SIMPLE_LAYOUT: Feature = Feature {
        ord: 1i32
    };
    #[doc(alias = "FEATURE_BIDI_LAYOUT")]
    #[doc = "Godot enumerator name: `FEATURE_BIDI_LAYOUT`"]
    pub const BIDI_LAYOUT: Feature = Feature {
        ord: 2i32
    };
    #[doc(alias = "FEATURE_VERTICAL_LAYOUT")]
    #[doc = "Godot enumerator name: `FEATURE_VERTICAL_LAYOUT`"]
    pub const VERTICAL_LAYOUT: Feature = Feature {
        ord: 4i32
    };
    #[doc(alias = "FEATURE_SHAPING")]
    #[doc = "Godot enumerator name: `FEATURE_SHAPING`"]
    pub const SHAPING: Feature = Feature {
        ord: 8i32
    };
    #[doc(alias = "FEATURE_KASHIDA_JUSTIFICATION")]
    #[doc = "Godot enumerator name: `FEATURE_KASHIDA_JUSTIFICATION`"]
    pub const KASHIDA_JUSTIFICATION: Feature = Feature {
        ord: 16i32
    };
    #[doc(alias = "FEATURE_BREAK_ITERATORS")]
    #[doc = "Godot enumerator name: `FEATURE_BREAK_ITERATORS`"]
    pub const BREAK_ITERATORS: Feature = Feature {
        ord: 32i32
    };
    #[doc(alias = "FEATURE_FONT_BITMAP")]
    #[doc = "Godot enumerator name: `FEATURE_FONT_BITMAP`"]
    pub const FONT_BITMAP: Feature = Feature {
        ord: 64i32
    };
    #[doc(alias = "FEATURE_FONT_DYNAMIC")]
    #[doc = "Godot enumerator name: `FEATURE_FONT_DYNAMIC`"]
    pub const FONT_DYNAMIC: Feature = Feature {
        ord: 128i32
    };
    #[doc(alias = "FEATURE_FONT_MSDF")]
    #[doc = "Godot enumerator name: `FEATURE_FONT_MSDF`"]
    pub const FONT_MSDF: Feature = Feature {
        ord: 256i32
    };
    #[doc(alias = "FEATURE_FONT_SYSTEM")]
    #[doc = "Godot enumerator name: `FEATURE_FONT_SYSTEM`"]
    pub const FONT_SYSTEM: Feature = Feature {
        ord: 512i32
    };
    #[doc(alias = "FEATURE_FONT_VARIABLE")]
    #[doc = "Godot enumerator name: `FEATURE_FONT_VARIABLE`"]
    pub const FONT_VARIABLE: Feature = Feature {
        ord: 1024i32
    };
    #[doc(alias = "FEATURE_CONTEXT_SENSITIVE_CASE_CONVERSION")]
    #[doc = "Godot enumerator name: `FEATURE_CONTEXT_SENSITIVE_CASE_CONVERSION`"]
    pub const CONTEXT_SENSITIVE_CASE_CONVERSION: Feature = Feature {
        ord: 2048i32
    };
    #[doc(alias = "FEATURE_USE_SUPPORT_DATA")]
    #[doc = "Godot enumerator name: `FEATURE_USE_SUPPORT_DATA`"]
    pub const USE_SUPPORT_DATA: Feature = Feature {
        ord: 4096i32
    };
    #[doc(alias = "FEATURE_UNICODE_IDENTIFIERS")]
    #[doc = "Godot enumerator name: `FEATURE_UNICODE_IDENTIFIERS`"]
    pub const UNICODE_IDENTIFIERS: Feature = Feature {
        ord: 8192i32
    };
    #[doc(alias = "FEATURE_UNICODE_SECURITY")]
    #[doc = "Godot enumerator name: `FEATURE_UNICODE_SECURITY`"]
    pub const UNICODE_SECURITY: Feature = Feature {
        ord: 16384i32
    };
    
}
impl std::fmt::Debug for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Feature") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Feature {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 | ord @ 32i32 | ord @ 64i32 | ord @ 128i32 | ord @ 256i32 | ord @ 512i32 | ord @ 1024i32 | ord @ 2048i32 | ord @ 4096i32 | ord @ 8192i32 | ord @ 16384i32 => Some(Self {
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
            Self::SIMPLE_LAYOUT => "SIMPLE_LAYOUT", Self::BIDI_LAYOUT => "BIDI_LAYOUT", Self::VERTICAL_LAYOUT => "VERTICAL_LAYOUT", Self::SHAPING => "SHAPING", Self::KASHIDA_JUSTIFICATION => "KASHIDA_JUSTIFICATION", Self::BREAK_ITERATORS => "BREAK_ITERATORS", Self::FONT_BITMAP => "FONT_BITMAP", Self::FONT_DYNAMIC => "FONT_DYNAMIC", Self::FONT_MSDF => "FONT_MSDF", Self::FONT_SYSTEM => "FONT_SYSTEM", Self::FONT_VARIABLE => "FONT_VARIABLE", Self::CONTEXT_SENSITIVE_CASE_CONVERSION => "CONTEXT_SENSITIVE_CASE_CONVERSION", Self::USE_SUPPORT_DATA => "USE_SUPPORT_DATA", Self::UNICODE_IDENTIFIERS => "UNICODE_IDENTIFIERS", Self::UNICODE_SECURITY => "UNICODE_SECURITY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Feature::SIMPLE_LAYOUT, Feature::BIDI_LAYOUT, Feature::VERTICAL_LAYOUT, Feature::SHAPING, Feature::KASHIDA_JUSTIFICATION, Feature::BREAK_ITERATORS, Feature::FONT_BITMAP, Feature::FONT_DYNAMIC, Feature::FONT_MSDF, Feature::FONT_SYSTEM, Feature::FONT_VARIABLE, Feature::CONTEXT_SENSITIVE_CASE_CONVERSION, Feature::USE_SUPPORT_DATA, Feature::UNICODE_IDENTIFIERS, Feature::UNICODE_SECURITY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Feature >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SIMPLE_LAYOUT", "FEATURE_SIMPLE_LAYOUT", Feature::SIMPLE_LAYOUT), crate::meta::inspect::EnumConstant::new("BIDI_LAYOUT", "FEATURE_BIDI_LAYOUT", Feature::BIDI_LAYOUT), crate::meta::inspect::EnumConstant::new("VERTICAL_LAYOUT", "FEATURE_VERTICAL_LAYOUT", Feature::VERTICAL_LAYOUT), crate::meta::inspect::EnumConstant::new("SHAPING", "FEATURE_SHAPING", Feature::SHAPING), crate::meta::inspect::EnumConstant::new("KASHIDA_JUSTIFICATION", "FEATURE_KASHIDA_JUSTIFICATION", Feature::KASHIDA_JUSTIFICATION), crate::meta::inspect::EnumConstant::new("BREAK_ITERATORS", "FEATURE_BREAK_ITERATORS", Feature::BREAK_ITERATORS), crate::meta::inspect::EnumConstant::new("FONT_BITMAP", "FEATURE_FONT_BITMAP", Feature::FONT_BITMAP), crate::meta::inspect::EnumConstant::new("FONT_DYNAMIC", "FEATURE_FONT_DYNAMIC", Feature::FONT_DYNAMIC), crate::meta::inspect::EnumConstant::new("FONT_MSDF", "FEATURE_FONT_MSDF", Feature::FONT_MSDF), crate::meta::inspect::EnumConstant::new("FONT_SYSTEM", "FEATURE_FONT_SYSTEM", Feature::FONT_SYSTEM), crate::meta::inspect::EnumConstant::new("FONT_VARIABLE", "FEATURE_FONT_VARIABLE", Feature::FONT_VARIABLE), crate::meta::inspect::EnumConstant::new("CONTEXT_SENSITIVE_CASE_CONVERSION", "FEATURE_CONTEXT_SENSITIVE_CASE_CONVERSION", Feature::CONTEXT_SENSITIVE_CASE_CONVERSION), crate::meta::inspect::EnumConstant::new("USE_SUPPORT_DATA", "FEATURE_USE_SUPPORT_DATA", Feature::USE_SUPPORT_DATA), crate::meta::inspect::EnumConstant::new("UNICODE_IDENTIFIERS", "FEATURE_UNICODE_IDENTIFIERS", Feature::UNICODE_IDENTIFIERS), crate::meta::inspect::EnumConstant::new("UNICODE_SECURITY", "FEATURE_UNICODE_SECURITY", Feature::UNICODE_SECURITY)]
        }
    }
}
impl crate::meta::GodotConvert for Feature {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Feature {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Feature {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ContourPointTag {
    ord: i32
}
impl ContourPointTag {
    #[doc(alias = "CONTOUR_CURVE_TAG_ON")]
    #[doc = "Godot enumerator name: `CONTOUR_CURVE_TAG_ON`"]
    pub const ON: ContourPointTag = ContourPointTag {
        ord: 1i32
    };
    #[doc(alias = "CONTOUR_CURVE_TAG_OFF_CONIC")]
    #[doc = "Godot enumerator name: `CONTOUR_CURVE_TAG_OFF_CONIC`"]
    pub const OFF_CONIC: ContourPointTag = ContourPointTag {
        ord: 0i32
    };
    #[doc(alias = "CONTOUR_CURVE_TAG_OFF_CUBIC")]
    #[doc = "Godot enumerator name: `CONTOUR_CURVE_TAG_OFF_CUBIC`"]
    pub const OFF_CUBIC: ContourPointTag = ContourPointTag {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ContourPointTag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ContourPointTag") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ContourPointTag {
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
            Self::ON => "ON", Self::OFF_CONIC => "OFF_CONIC", Self::OFF_CUBIC => "OFF_CUBIC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ContourPointTag::ON, ContourPointTag::OFF_CONIC, ContourPointTag::OFF_CUBIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ContourPointTag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ON", "CONTOUR_CURVE_TAG_ON", ContourPointTag::ON), crate::meta::inspect::EnumConstant::new("OFF_CONIC", "CONTOUR_CURVE_TAG_OFF_CONIC", ContourPointTag::OFF_CONIC), crate::meta::inspect::EnumConstant::new("OFF_CUBIC", "CONTOUR_CURVE_TAG_OFF_CUBIC", ContourPointTag::OFF_CUBIC)]
        }
    }
}
impl crate::meta::GodotConvert for ContourPointTag {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ContourPointTag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ContourPointTag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpacingType {
    ord: i32
}
impl SpacingType {
    #[doc(alias = "SPACING_GLYPH")]
    #[doc = "Godot enumerator name: `SPACING_GLYPH`"]
    pub const GLYPH: SpacingType = SpacingType {
        ord: 0i32
    };
    #[doc(alias = "SPACING_SPACE")]
    #[doc = "Godot enumerator name: `SPACING_SPACE`"]
    pub const SPACE: SpacingType = SpacingType {
        ord: 1i32
    };
    #[doc(alias = "SPACING_TOP")]
    #[doc = "Godot enumerator name: `SPACING_TOP`"]
    pub const TOP: SpacingType = SpacingType {
        ord: 2i32
    };
    #[doc(alias = "SPACING_BOTTOM")]
    #[doc = "Godot enumerator name: `SPACING_BOTTOM`"]
    pub const BOTTOM: SpacingType = SpacingType {
        ord: 3i32
    };
    #[doc(alias = "SPACING_MAX")]
    #[doc = "Godot enumerator name: `SPACING_MAX`"]
    pub const MAX: SpacingType = SpacingType {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for SpacingType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SpacingType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SpacingType {
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
            Self::GLYPH => "GLYPH", Self::SPACE => "SPACE", Self::TOP => "TOP", Self::BOTTOM => "BOTTOM", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SpacingType::GLYPH, SpacingType::SPACE, SpacingType::TOP, SpacingType::BOTTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SpacingType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("GLYPH", "SPACING_GLYPH", SpacingType::GLYPH), crate::meta::inspect::EnumConstant::new("SPACE", "SPACING_SPACE", SpacingType::SPACE), crate::meta::inspect::EnumConstant::new("TOP", "SPACING_TOP", SpacingType::TOP), crate::meta::inspect::EnumConstant::new("BOTTOM", "SPACING_BOTTOM", SpacingType::BOTTOM), crate::meta::inspect::EnumConstant::new("MAX", "SPACING_MAX", SpacingType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for SpacingType {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for SpacingType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SpacingType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SpacingType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct FontStyle {
    ord: u64
}
impl FontStyle {
    #[doc(alias = "FONT_BOLD")]
    #[doc = "Godot enumerator name: `FONT_BOLD`"]
    pub const BOLD: FontStyle = FontStyle {
        ord: 1u64
    };
    #[doc(alias = "FONT_ITALIC")]
    #[doc = "Godot enumerator name: `FONT_ITALIC`"]
    pub const ITALIC: FontStyle = FontStyle {
        ord: 2u64
    };
    #[doc(alias = "FONT_FIXED_WIDTH")]
    #[doc = "Godot enumerator name: `FONT_FIXED_WIDTH`"]
    pub const FIXED_WIDTH: FontStyle = FontStyle {
        ord: 4u64
    };
    
}
impl std::fmt::Debug for FontStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::BOLD => "BOLD", Self::ITALIC => "ITALIC", Self::FIXED_WIDTH => "FIXED_WIDTH", _ => {
                f.debug_struct("FontStyle") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for FontStyle {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FontStyle >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BOLD", "FONT_BOLD", FontStyle::BOLD), crate::meta::inspect::EnumConstant::new("ITALIC", "FONT_ITALIC", FontStyle::ITALIC), crate::meta::inspect::EnumConstant::new("FIXED_WIDTH", "FONT_FIXED_WIDTH", FontStyle::FIXED_WIDTH)]
        }
    }
}
impl std::ops::BitOr for FontStyle {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for FontStyle {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for FontStyle {
    type Via = u64;
    
}
impl crate::meta::ToGodot for FontStyle {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FontStyle {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct StructuredTextParser {
    ord: i32
}
impl StructuredTextParser {
    #[doc(alias = "STRUCTURED_TEXT_DEFAULT")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_DEFAULT`"]
    pub const DEFAULT: StructuredTextParser = StructuredTextParser {
        ord: 0i32
    };
    #[doc(alias = "STRUCTURED_TEXT_URI")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_URI`"]
    pub const URI: StructuredTextParser = StructuredTextParser {
        ord: 1i32
    };
    #[doc(alias = "STRUCTURED_TEXT_FILE")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_FILE`"]
    pub const FILE: StructuredTextParser = StructuredTextParser {
        ord: 2i32
    };
    #[doc(alias = "STRUCTURED_TEXT_EMAIL")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_EMAIL`"]
    pub const EMAIL: StructuredTextParser = StructuredTextParser {
        ord: 3i32
    };
    #[doc(alias = "STRUCTURED_TEXT_LIST")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_LIST`"]
    pub const LIST: StructuredTextParser = StructuredTextParser {
        ord: 4i32
    };
    #[doc(alias = "STRUCTURED_TEXT_GDSCRIPT")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_GDSCRIPT`"]
    pub const GDSCRIPT: StructuredTextParser = StructuredTextParser {
        ord: 5i32
    };
    #[doc(alias = "STRUCTURED_TEXT_CUSTOM")]
    #[doc = "Godot enumerator name: `STRUCTURED_TEXT_CUSTOM`"]
    pub const CUSTOM: StructuredTextParser = StructuredTextParser {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for StructuredTextParser {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("StructuredTextParser") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for StructuredTextParser {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::DEFAULT => "DEFAULT", Self::URI => "URI", Self::FILE => "FILE", Self::EMAIL => "EMAIL", Self::LIST => "LIST", Self::GDSCRIPT => "GDSCRIPT", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[StructuredTextParser::DEFAULT, StructuredTextParser::URI, StructuredTextParser::FILE, StructuredTextParser::EMAIL, StructuredTextParser::LIST, StructuredTextParser::GDSCRIPT, StructuredTextParser::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < StructuredTextParser >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "STRUCTURED_TEXT_DEFAULT", StructuredTextParser::DEFAULT), crate::meta::inspect::EnumConstant::new("URI", "STRUCTURED_TEXT_URI", StructuredTextParser::URI), crate::meta::inspect::EnumConstant::new("FILE", "STRUCTURED_TEXT_FILE", StructuredTextParser::FILE), crate::meta::inspect::EnumConstant::new("EMAIL", "STRUCTURED_TEXT_EMAIL", StructuredTextParser::EMAIL), crate::meta::inspect::EnumConstant::new("LIST", "STRUCTURED_TEXT_LIST", StructuredTextParser::LIST), crate::meta::inspect::EnumConstant::new("GDSCRIPT", "STRUCTURED_TEXT_GDSCRIPT", StructuredTextParser::GDSCRIPT), crate::meta::inspect::EnumConstant::new("CUSTOM", "STRUCTURED_TEXT_CUSTOM", StructuredTextParser::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for StructuredTextParser {
    type Via = i32;
    
}
impl crate::meta::ToGodot for StructuredTextParser {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for StructuredTextParser {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FixedSizeScaleMode {
    ord: i32
}
impl FixedSizeScaleMode {
    #[doc(alias = "FIXED_SIZE_SCALE_DISABLE")]
    #[doc = "Godot enumerator name: `FIXED_SIZE_SCALE_DISABLE`"]
    pub const DISABLE: FixedSizeScaleMode = FixedSizeScaleMode {
        ord: 0i32
    };
    #[doc(alias = "FIXED_SIZE_SCALE_INTEGER_ONLY")]
    #[doc = "Godot enumerator name: `FIXED_SIZE_SCALE_INTEGER_ONLY`"]
    pub const INTEGER_ONLY: FixedSizeScaleMode = FixedSizeScaleMode {
        ord: 1i32
    };
    #[doc(alias = "FIXED_SIZE_SCALE_ENABLED")]
    #[doc = "Godot enumerator name: `FIXED_SIZE_SCALE_ENABLED`"]
    pub const ENABLED: FixedSizeScaleMode = FixedSizeScaleMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for FixedSizeScaleMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FixedSizeScaleMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FixedSizeScaleMode {
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
            Self::DISABLE => "DISABLE", Self::INTEGER_ONLY => "INTEGER_ONLY", Self::ENABLED => "ENABLED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FixedSizeScaleMode::DISABLE, FixedSizeScaleMode::INTEGER_ONLY, FixedSizeScaleMode::ENABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FixedSizeScaleMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLE", "FIXED_SIZE_SCALE_DISABLE", FixedSizeScaleMode::DISABLE), crate::meta::inspect::EnumConstant::new("INTEGER_ONLY", "FIXED_SIZE_SCALE_INTEGER_ONLY", FixedSizeScaleMode::INTEGER_ONLY), crate::meta::inspect::EnumConstant::new("ENABLED", "FIXED_SIZE_SCALE_ENABLED", FixedSizeScaleMode::ENABLED)]
        }
    }
}
impl crate::meta::GodotConvert for FixedSizeScaleMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FixedSizeScaleMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FixedSizeScaleMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TextServer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for TextServer {
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