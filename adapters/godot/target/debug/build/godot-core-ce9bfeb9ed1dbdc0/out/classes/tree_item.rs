#![doc = "Sidecar module for class [`TreeItem`][crate::classes::TreeItem].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TreeItem` enums](https://docs.godotengine.org/en/stable/classes/class_treeitem.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TreeItem.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`tree_item`][crate::classes::tree_item]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `TreeItem`](https://docs.godotengine.org/en/stable/classes/class_treeitem.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<TreeItem>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TreeItem {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl TreeItem {
        pub fn set_cell_mode(&mut self, column: i32, mode: crate::classes::tree_item::TreeCellMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::tree_item::TreeCellMode,);
            let args = (column, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_cell_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cell_mode(&self, column: i32,) -> crate::classes::tree_item::TreeCellMode {
            type CallRet = crate::classes::tree_item::TreeCellMode;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_cell_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_translate_mode(&mut self, column: i32, mode: crate::classes::node::AutoTranslateMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::node::AutoTranslateMode,);
            let args = (column, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_auto_translate_mode(&self, column: i32,) -> crate::classes::node::AutoTranslateMode {
            type CallRet = crate::classes::node::AutoTranslateMode;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_edit_multiline(&mut self, column: i32, multiline: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, multiline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_edit_multiline", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_edit_multiline(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_edit_multiline", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_checked(&mut self, column: i32, checked: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, checked,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_checked", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_indeterminate(&mut self, column: i32, indeterminate: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, indeterminate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_indeterminate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_checked(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_checked", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_indeterminate(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_indeterminate", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn propagate_check_full(&mut self, column: i32, emit_signal: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, emit_signal,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "propagate_check", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::propagate_check_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn propagate_check(&mut self, column: i32,) {
            self.propagate_check_ex(column,) . done()
        }
        #[inline]
        pub fn propagate_check_ex < 'a > (&'a mut self, column: i32,) -> ExPropagateCheck < 'a > {
            ExPropagateCheck::new(self, column,)
        }
        pub fn set_text(&mut self, column: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text(&self, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_description(&mut self, column: i32, description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, description.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_description(&self, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_direction(&mut self, column: i32, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::control::TextDirection,);
            let args = (column, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_direction(&self, column: i32,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10148usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autowrap_mode(&mut self, column: i32, autowrap_mode: crate::classes::text_server::AutowrapMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::text_server::AutowrapMode,);
            let args = (column, autowrap_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10149usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autowrap_mode(&self, column: i32,) -> crate::classes::text_server::AutowrapMode {
            type CallRet = crate::classes::text_server::AutowrapMode;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_autowrap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_overrun_behavior(&mut self, column: i32, overrun_behavior: crate::classes::text_server::OverrunBehavior,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::text_server::OverrunBehavior,);
            let args = (column, overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_overrun_behavior(&self, column: i32,) -> crate::classes::text_server::OverrunBehavior {
            type CallRet = crate::classes::text_server::OverrunBehavior;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override(&mut self, column: i32, parser: crate::classes::text_server::StructuredTextParser,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::text_server::StructuredTextParser,);
            let args = (column, parser,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override(&self, column: i32,) -> crate::classes::text_server::StructuredTextParser {
            type CallRet = crate::classes::text_server::StructuredTextParser;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_structured_text_bidi_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, column: i32, args: &VarArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, VarArray >,);
            let args = (column, RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_structured_text_bidi_override_options(&self, column: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_structured_text_bidi_override_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_language(&mut self, column: i32, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_language(&self, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_suffix(&mut self, column: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_suffix", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_suffix(&self, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_suffix", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_icon(&mut self, column: i32, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (column, texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_icon(&self, column: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_icon_overlay(&mut self, column: i32, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (column, texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_icon_overlay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_icon_overlay(&self, column: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_icon_overlay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_icon_region(&mut self, column: i32, region: Rect2,) {
            type CallRet = ();
            type CallParams = (i32, Rect2,);
            let args = (column, region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_icon_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_icon_region(&self, column: i32,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_icon_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_icon_max_width(&mut self, column: i32, width: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (column, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_icon_max_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_icon_max_width(&self, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_icon_max_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_icon_modulate(&mut self, column: i32, modulate: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (column, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_icon_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_icon_modulate(&self, column: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_icon_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_range(&mut self, column: i32, value: f64,) {
            type CallRet = ();
            type CallParams = (i32, f64,);
            let args = (column, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_range(&self, column: i32,) -> f64 {
            type CallRet = f64;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_range", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn set_range_config_full(&mut self, column: i32, min: f64, max: f64, step: f64, expr: bool,) {
            type CallRet = ();
            type CallParams = (i32, f64, f64, f64, bool,);
            let args = (column, min, max, step, expr,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_range_config", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_range_config_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_range_config(&mut self, column: i32, min: f64, max: f64, step: f64,) {
            self.set_range_config_ex(column, min, max, step,) . done()
        }
        #[inline]
        pub fn set_range_config_ex < 'a > (&'a mut self, column: i32, min: f64, max: f64, step: f64,) -> ExSetRangeConfig < 'a > {
            ExSetRangeConfig::new(self, column, min, max, step,)
        }
        pub fn get_range_config(&mut self, column: i32,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_range_config", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_metadata(&mut self, column: i32, meta: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (column, RefArg::new(meta),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_metadata", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_metadata(&self, column: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_metadata", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_draw(&mut self, column: i32, object: impl AsArg < Option < Gd < crate::classes::Object >> >, callback: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Object > > >, CowArg < 'a1, StringName >,);
            let args = (column, object.into_arg(), callback.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_draw", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_draw_callback(&mut self, column: i32, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Callable >,);
            let args = (column, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_draw_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_draw_callback(&self, column: i32,) -> Callable {
            type CallRet = Callable;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_draw_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collapsed(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_collapsed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_collapsed(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_collapsed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collapsed_recursive(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_collapsed_recursive", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn is_any_collapsed_full(&mut self, only_visible: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (bool,);
            let args = (only_visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_any_collapsed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_any_collapsed_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_any_collapsed(&mut self,) -> bool {
            self.is_any_collapsed_ex() . done()
        }
        #[inline]
        pub fn is_any_collapsed_ex < 'a > (&'a mut self,) -> ExIsAnyCollapsed < 'a > {
            ExIsAnyCollapsed::new(self,)
        }
        pub fn set_visible(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_visible(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_visible_in_tree(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_visible_in_tree", Some(self.__validated_obj()), args,)
            }
        }
        pub fn uncollapse_tree(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "uncollapse_tree", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_minimum_height(&mut self, height: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_minimum_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_minimum_height(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_minimum_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_selectable(&mut self, column: i32, selectable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, selectable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_selectable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_selectable(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_selectable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_selected(&mut self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_selected", Some(self.__validated_obj()), args,)
            }
        }
        pub fn select(&mut self, column: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "select", Some(self.__validated_obj()), args,)
            }
        }
        pub fn deselect(&mut self, column: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "deselect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_editable(&mut self, column: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_editable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_editable(&mut self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_editable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_color(&mut self, column: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (column, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_color(&self, column: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_custom_color(&mut self, column: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "clear_custom_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_font(&mut self, column: i32, font: impl AsArg < Option < Gd < crate::classes::Font >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Font > > >,);
            let args = (column, font.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_font(&self, column: i32,) -> Option < Gd < crate::classes::Font > > {
            type CallRet = Option < Gd < crate::classes::Font > >;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_font_size(&mut self, column: i32, font_size: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (column, font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_font_size(&self, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn set_custom_bg_color_full(&mut self, column: i32, color: Color, just_outline: bool,) {
            type CallRet = ();
            type CallParams = (i32, Color, bool,);
            let args = (column, color, just_outline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_bg_color", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_custom_bg_color_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_custom_bg_color(&mut self, column: i32, color: Color,) {
            self.set_custom_bg_color_ex(column, color,) . done()
        }
        #[inline]
        pub fn set_custom_bg_color_ex < 'a > (&'a mut self, column: i32, color: Color,) -> ExSetCustomBgColor < 'a > {
            ExSetCustomBgColor::new(self, column, color,)
        }
        pub fn clear_custom_bg_color(&mut self, column: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "clear_custom_bg_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_bg_color(&self, column: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_custom_bg_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_as_button(&mut self, column: i32, enable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_custom_as_button", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_custom_set_as_button(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_custom_set_as_button", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_buttons(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "clear_buttons", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_button_full(&mut self, column: i32, button: CowArg < Option < Gd < crate::classes::Texture2D > > >, id: i32, disabled: bool, tooltip_text: CowArg < GString >, description: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, i32, bool, CowArg < 'a1, GString >, CowArg < 'a2, GString >,);
            let args = (column, button, id, disabled, tooltip_text, description,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "add_button", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_button_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_button(&mut self, column: i32, button: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            self.add_button_ex(column, button,) . done()
        }
        #[inline]
        pub fn add_button_ex < 'a > (&'a mut self, column: i32, button: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a,) -> ExAddButton < 'a > {
            ExAddButton::new(self, column, button,)
        }
        pub fn get_button_count(&self, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_button_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_button_tooltip_text(&self, column: i32, button_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32, i32,);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_button_tooltip_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_button_id(&self, column: i32, button_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_button_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_button_by_id(&self, column: i32, id: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (column, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_button_by_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_button_color(&self, column: i32, id: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32, i32,);
            let args = (column, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_button_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_button(&self, column: i32, button_index: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32, i32,);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_button", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_button_tooltip_text(&mut self, column: i32, button_index: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, GString >,);
            let args = (column, button_index, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_button_tooltip_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_button(&mut self, column: i32, button_index: i32, button: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (column, button_index, button.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_button", Some(self.__validated_obj()), args,)
            }
        }
        pub fn erase_button(&mut self, column: i32, button_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "erase_button", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_button_description(&mut self, column: i32, button_index: i32, description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, GString >,);
            let args = (column, button_index, description.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_button_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_button_disabled(&mut self, column: i32, button_index: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, i32, bool,);
            let args = (column, button_index, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_button_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_button_color(&mut self, column: i32, button_index: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, i32, Color,);
            let args = (column, button_index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_button_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_button_disabled(&self, column: i32, button_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, i32,);
            let args = (column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_button_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tooltip_text(&mut self, column: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_tooltip_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tooltip_text(&self, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_tooltip_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_alignment(&mut self, column: i32, text_alignment: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (i32, crate::global::HorizontalAlignment,);
            let args = (column, text_alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_text_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_alignment(&self, column: i32,) -> crate::global::HorizontalAlignment {
            type CallRet = crate::global::HorizontalAlignment;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_text_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_expand_right(&mut self, column: i32, enable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_expand_right", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_expand_right(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_expand_right", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_disable_folding(&mut self, disable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "set_disable_folding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_folding_disabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "is_folding_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn create_child_full(&mut self, index: i32,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "create_child", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_child_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_child(&mut self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.create_child_ex() . done()
        }
        #[inline]
        pub fn create_child_ex < 'a > (&'a mut self,) -> ExCreateChild < 'a > {
            ExCreateChild::new(self,)
        }
        pub fn add_child(&mut self, child: impl AsArg < Option < Gd < crate::classes::TreeItem >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >,);
            let args = (child.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "add_child", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_child(&mut self, child: impl AsArg < Option < Gd < crate::classes::TreeItem >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >,);
            let args = (child.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "remove_child", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tree(&self,) -> Option < Gd < crate::classes::Tree > > {
            type CallRet = Option < Gd < crate::classes::Tree > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_tree", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_next(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_next", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_prev(&mut self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_prev", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_parent(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_first_child(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_first_child", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_next_in_tree_full(&mut self, wrap: bool,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = (bool,);
            let args = (wrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_next_in_tree", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_next_in_tree_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_next_in_tree(&mut self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.get_next_in_tree_ex() . done()
        }
        #[inline]
        pub fn get_next_in_tree_ex < 'a > (&'a mut self,) -> ExGetNextInTree < 'a > {
            ExGetNextInTree::new(self,)
        }
        pub(crate) fn get_prev_in_tree_full(&mut self, wrap: bool,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = (bool,);
            let args = (wrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_prev_in_tree", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_prev_in_tree_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_prev_in_tree(&mut self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.get_prev_in_tree_ex() . done()
        }
        #[inline]
        pub fn get_prev_in_tree_ex < 'a > (&'a mut self,) -> ExGetPrevInTree < 'a > {
            ExGetPrevInTree::new(self,)
        }
        pub(crate) fn get_next_visible_full(&mut self, wrap: bool,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = (bool,);
            let args = (wrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_next_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_next_visible_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_next_visible(&mut self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.get_next_visible_ex() . done()
        }
        #[inline]
        pub fn get_next_visible_ex < 'a > (&'a mut self,) -> ExGetNextVisible < 'a > {
            ExGetNextVisible::new(self,)
        }
        pub(crate) fn get_prev_visible_full(&mut self, wrap: bool,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = (bool,);
            let args = (wrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_prev_visible", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_prev_visible_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_prev_visible(&mut self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.get_prev_visible_ex() . done()
        }
        #[inline]
        pub fn get_prev_visible_ex < 'a > (&'a mut self,) -> ExGetPrevVisible < 'a > {
            ExGetPrevVisible::new(self,)
        }
        pub fn get_child(&mut self, index: i32,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_child", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_child_count(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_child_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_children(&mut self,) -> Array < Gd < crate::classes::TreeItem > > {
            type CallRet = Array < Gd < crate::classes::TreeItem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_children", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_index(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "get_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn move_before(&mut self, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >,);
            let args = (item.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "move_before", Some(self.__validated_obj()), args,)
            }
        }
        pub fn move_after(&mut self, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >,);
            let args = (item.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TreeItem", "move_after", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn call_recursive(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) {
            Self::try_call_recursive(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_call_recursive(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < (), crate::meta::error::CallError > {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10250usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "TreeItem", "call_recursive", Some(self.__validated_obj()), args, varargs)
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
    impl crate::obj::GodotClass for TreeItem {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TreeItem"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TreeItem {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TreeItem {
        
    }
    impl std::ops::Deref for TreeItem {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TreeItem {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TreeItem__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `TreeItem` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`TreeItem::propagate_check_ex`][super::TreeItem::propagate_check_ex]."]
#[must_use]
pub struct ExPropagateCheck < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, column: i32, emit_signal: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPropagateCheck < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem, column: i32,) -> Self {
        let emit_signal = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, emit_signal: emit_signal,
        }
    }
    #[inline]
    pub fn emit_signal(self, emit_signal: bool) -> Self {
        Self {
            emit_signal: emit_signal, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, emit_signal,
        }
        = self;
        re_export::TreeItem::propagate_check_full(surround_object, column, emit_signal,)
    }
}
#[doc = "Default-param extender for [`TreeItem::set_range_config_ex`][super::TreeItem::set_range_config_ex]."]
#[must_use]
pub struct ExSetRangeConfig < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, column: i32, min: f64, max: f64, step: f64, expr: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetRangeConfig < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem, column: i32, min: f64, max: f64, step: f64,) -> Self {
        let expr = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, min: min, max: max, step: step, expr: expr,
        }
    }
    #[inline]
    pub fn expr(self, expr: bool) -> Self {
        Self {
            expr: expr, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, min, max, step, expr,
        }
        = self;
        re_export::TreeItem::set_range_config_full(surround_object, column, min, max, step, expr,)
    }
}
#[doc = "Default-param extender for [`TreeItem::is_any_collapsed_ex`][super::TreeItem::is_any_collapsed_ex]."]
#[must_use]
pub struct ExIsAnyCollapsed < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, only_visible: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsAnyCollapsed < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        let only_visible = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, only_visible: only_visible,
        }
    }
    #[inline]
    pub fn only_visible(self, only_visible: bool) -> Self {
        Self {
            only_visible: only_visible, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, only_visible,
        }
        = self;
        re_export::TreeItem::is_any_collapsed_full(surround_object, only_visible,)
    }
}
#[doc = "Default-param extender for [`TreeItem::set_custom_bg_color_ex`][super::TreeItem::set_custom_bg_color_ex]."]
#[must_use]
pub struct ExSetCustomBgColor < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, column: i32, color: Color, just_outline: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCustomBgColor < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem, column: i32, color: Color,) -> Self {
        let just_outline = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, color: color, just_outline: just_outline,
        }
    }
    #[inline]
    pub fn just_outline(self, just_outline: bool) -> Self {
        Self {
            just_outline: just_outline, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, color, just_outline,
        }
        = self;
        re_export::TreeItem::set_custom_bg_color_full(surround_object, column, color, just_outline,)
    }
}
#[doc = "Default-param extender for [`TreeItem::add_button_ex`][super::TreeItem::add_button_ex]."]
#[must_use]
pub struct ExAddButton < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, column: i32, button: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, id: i32, disabled: bool, tooltip_text: CowArg < 'a, GString >, description: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddButton < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem, column: i32, button: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a,) -> Self {
        let id = - 1i32;
        let disabled = false;
        let tooltip_text = GString::from("");
        let description = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, column: column, button: button.into_arg(), id: id, disabled: disabled, tooltip_text: CowArg::Owned(tooltip_text), description: CowArg::Owned(description),
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn disabled(self, disabled: bool) -> Self {
        Self {
            disabled: disabled, .. self
        }
    }
    #[inline]
    pub fn tooltip_text(self, tooltip_text: impl AsArg < GString > + 'a) -> Self {
        Self {
            tooltip_text: tooltip_text.into_arg(), .. self
        }
    }
    #[inline]
    pub fn description(self, description: impl AsArg < GString > + 'a) -> Self {
        Self {
            description: description.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, column, button, id, disabled, tooltip_text, description,
        }
        = self;
        re_export::TreeItem::add_button_full(surround_object, column, button, id, disabled, tooltip_text, description,)
    }
}
#[doc = "Default-param extender for [`TreeItem::create_child_ex`][super::TreeItem::create_child_ex]."]
#[must_use]
pub struct ExCreateChild < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateChild < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
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
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, index,
        }
        = self;
        re_export::TreeItem::create_child_full(surround_object, index,)
    }
}
#[doc = "Default-param extender for [`TreeItem::get_next_in_tree_ex`][super::TreeItem::get_next_in_tree_ex]."]
#[must_use]
pub struct ExGetNextInTree < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, wrap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetNextInTree < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        let wrap = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, wrap: wrap,
        }
    }
    #[inline]
    pub fn wrap(self, wrap: bool) -> Self {
        Self {
            wrap: wrap, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, wrap,
        }
        = self;
        re_export::TreeItem::get_next_in_tree_full(surround_object, wrap,)
    }
}
#[doc = "Default-param extender for [`TreeItem::get_prev_in_tree_ex`][super::TreeItem::get_prev_in_tree_ex]."]
#[must_use]
pub struct ExGetPrevInTree < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, wrap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetPrevInTree < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        let wrap = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, wrap: wrap,
        }
    }
    #[inline]
    pub fn wrap(self, wrap: bool) -> Self {
        Self {
            wrap: wrap, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, wrap,
        }
        = self;
        re_export::TreeItem::get_prev_in_tree_full(surround_object, wrap,)
    }
}
#[doc = "Default-param extender for [`TreeItem::get_next_visible_ex`][super::TreeItem::get_next_visible_ex]."]
#[must_use]
pub struct ExGetNextVisible < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, wrap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetNextVisible < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        let wrap = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, wrap: wrap,
        }
    }
    #[inline]
    pub fn wrap(self, wrap: bool) -> Self {
        Self {
            wrap: wrap, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, wrap,
        }
        = self;
        re_export::TreeItem::get_next_visible_full(surround_object, wrap,)
    }
}
#[doc = "Default-param extender for [`TreeItem::get_prev_visible_ex`][super::TreeItem::get_prev_visible_ex]."]
#[must_use]
pub struct ExGetPrevVisible < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TreeItem, wrap: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetPrevVisible < 'a > {
    fn new(surround_object: &'a mut re_export::TreeItem,) -> Self {
        let wrap = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, wrap: wrap,
        }
    }
    #[inline]
    pub fn wrap(self, wrap: bool) -> Self {
        Self {
            wrap: wrap, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, wrap,
        }
        = self;
        re_export::TreeItem::get_prev_visible_full(surround_object, wrap,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TreeCellMode {
    ord: i32
}
impl TreeCellMode {
    #[doc(alias = "CELL_MODE_STRING")]
    #[doc = "Godot enumerator name: `CELL_MODE_STRING`"]
    pub const STRING: TreeCellMode = TreeCellMode {
        ord: 0i32
    };
    #[doc(alias = "CELL_MODE_CHECK")]
    #[doc = "Godot enumerator name: `CELL_MODE_CHECK`"]
    pub const CHECK: TreeCellMode = TreeCellMode {
        ord: 1i32
    };
    #[doc(alias = "CELL_MODE_RANGE")]
    #[doc = "Godot enumerator name: `CELL_MODE_RANGE`"]
    pub const RANGE: TreeCellMode = TreeCellMode {
        ord: 2i32
    };
    #[doc(alias = "CELL_MODE_ICON")]
    #[doc = "Godot enumerator name: `CELL_MODE_ICON`"]
    pub const ICON: TreeCellMode = TreeCellMode {
        ord: 3i32
    };
    #[doc(alias = "CELL_MODE_CUSTOM")]
    #[doc = "Godot enumerator name: `CELL_MODE_CUSTOM`"]
    pub const CUSTOM: TreeCellMode = TreeCellMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for TreeCellMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TreeCellMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TreeCellMode {
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
            Self::STRING => "STRING", Self::CHECK => "CHECK", Self::RANGE => "RANGE", Self::ICON => "ICON", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TreeCellMode::STRING, TreeCellMode::CHECK, TreeCellMode::RANGE, TreeCellMode::ICON, TreeCellMode::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TreeCellMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STRING", "CELL_MODE_STRING", TreeCellMode::STRING), crate::meta::inspect::EnumConstant::new("CHECK", "CELL_MODE_CHECK", TreeCellMode::CHECK), crate::meta::inspect::EnumConstant::new("RANGE", "CELL_MODE_RANGE", TreeCellMode::RANGE), crate::meta::inspect::EnumConstant::new("ICON", "CELL_MODE_ICON", TreeCellMode::ICON), crate::meta::inspect::EnumConstant::new("CUSTOM", "CELL_MODE_CUSTOM", TreeCellMode::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for TreeCellMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TreeCellMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TreeCellMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TreeItem;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for TreeItem {
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