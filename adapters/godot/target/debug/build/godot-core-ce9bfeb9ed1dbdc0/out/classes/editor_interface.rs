#![doc = "Sidecar module for class [`EditorInterface`][crate::classes::EditorInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorInterface` enums](https://docs.godotengine.org/en/stable/classes/class_editorinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorInterface.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`editor_interface`][crate::classes::editor_interface]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `EditorInterface`](https://docs.godotengine.org/en/stable/classes/class_editorinterface.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl EditorInterface {
        pub(crate) fn restart_editor_full(&mut self, save: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (save,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "restart_editor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::restart_editor_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn restart_editor(&mut self,) {
            self.restart_editor_ex() . done()
        }
        #[inline]
        pub fn restart_editor_ex < 'a > (&'a mut self,) -> ExRestartEditor < 'a > {
            ExRestartEditor::new(self,)
        }
        pub fn get_command_palette(&self,) -> Option < Gd < crate::classes::EditorCommandPalette > > {
            type CallRet = Option < Gd < crate::classes::EditorCommandPalette > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_command_palette", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_resource_filesystem(&self,) -> Option < Gd < crate::classes::EditorFileSystem > > {
            type CallRet = Option < Gd < crate::classes::EditorFileSystem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_resource_filesystem", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_editor_paths(&self,) -> Option < Gd < crate::classes::EditorPaths > > {
            type CallRet = Option < Gd < crate::classes::EditorPaths > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_paths", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_resource_previewer(&self,) -> Option < Gd < crate::classes::EditorResourcePreview > > {
            type CallRet = Option < Gd < crate::classes::EditorResourcePreview > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_resource_previewer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selection(&self,) -> Option < Gd < crate::classes::EditorSelection > > {
            type CallRet = Option < Gd < crate::classes::EditorSelection > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_selection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_editor_settings(&self,) -> Option < Gd < crate::classes::EditorSettings > > {
            type CallRet = Option < Gd < crate::classes::EditorSettings > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_settings", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_editor_toaster(&self,) -> Option < Gd < crate::classes::EditorToaster > > {
            type CallRet = Option < Gd < crate::classes::EditorToaster > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_toaster", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_editor_undo_redo(&self,) -> Option < Gd < crate::classes::EditorUndoRedoManager > > {
            type CallRet = Option < Gd < crate::classes::EditorUndoRedoManager > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_undo_redo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn make_mesh_previews(&mut self, meshes: &Array < Gd < crate::classes::Mesh > >, preview_size: i32,) -> Array < Gd < crate::classes::Texture2D > > {
            type CallRet = Array < Gd < crate::classes::Texture2D > >;
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::Mesh > > >, i32,);
            let args = (RefArg::new(meshes), preview_size,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "make_mesh_previews", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_plugin_enabled(&mut self, plugin: impl AsArg < GString >, enabled: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (plugin.into_arg(), enabled,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "set_plugin_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_plugin_enabled(&self, plugin: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "is_plugin_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_editor_theme(&self,) -> Option < Gd < crate::classes::Theme > > {
            type CallRet = Option < Gd < crate::classes::Theme > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_theme", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_base_control(&self,) -> Option < Gd < crate::classes::Control > > {
            type CallRet = Option < Gd < crate::classes::Control > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_base_control", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_editor_main_screen(&self,) -> Option < Gd < crate::classes::VBoxContainer > > {
            type CallRet = Option < Gd < crate::classes::VBoxContainer > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_main_screen", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_script_editor(&self,) -> Option < Gd < crate::classes::ScriptEditor > > {
            type CallRet = Option < Gd < crate::classes::ScriptEditor > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_script_editor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_editor_viewport_2d(&self,) -> Option < Gd < crate::classes::SubViewport > > {
            type CallRet = Option < Gd < crate::classes::SubViewport > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_viewport_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_editor_viewport_3d_full(&self, idx: i32,) -> Option < Gd < crate::classes::SubViewport > > {
            type CallRet = Option < Gd < crate::classes::SubViewport > >;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_viewport_3d", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_editor_viewport_3d_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_editor_viewport_3d(&self,) -> Option < Gd < crate::classes::SubViewport > > {
            self.get_editor_viewport_3d_ex() . done()
        }
        #[inline]
        pub fn get_editor_viewport_3d_ex < 'a > (&'a self,) -> ExGetEditorViewport3d < 'a > {
            ExGetEditorViewport3d::new(self,)
        }
        pub fn set_main_screen_editor(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "set_main_screen_editor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_distraction_free_mode(&mut self, enter: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enter,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "set_distraction_free_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_distraction_free_mode_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "is_distraction_free_mode_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_multi_window_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "is_multi_window_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_editor_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_editor_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn popup_dialog_full(&mut self, dialog: CowArg < Option < Gd < crate::classes::Window > > >, rect: Rect2i,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Window > > >, Rect2i,);
            let args = (dialog, rect,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_dialog", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_dialog_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_dialog(&mut self, dialog: impl AsArg < Option < Gd < crate::classes::Window >> >,) {
            self.popup_dialog_ex(dialog,) . done()
        }
        #[inline]
        pub fn popup_dialog_ex < 'a > (&'a mut self, dialog: impl AsArg < Option < Gd < crate::classes::Window >> > + 'a,) -> ExPopupDialog < 'a > {
            ExPopupDialog::new(self, dialog,)
        }
        pub(crate) fn popup_dialog_centered_full(&mut self, dialog: CowArg < Option < Gd < crate::classes::Window > > >, minsize: Vector2i,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Window > > >, Vector2i,);
            let args = (dialog, minsize,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_dialog_centered", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_dialog_centered_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_dialog_centered(&mut self, dialog: impl AsArg < Option < Gd < crate::classes::Window >> >,) {
            self.popup_dialog_centered_ex(dialog,) . done()
        }
        #[inline]
        pub fn popup_dialog_centered_ex < 'a > (&'a mut self, dialog: impl AsArg < Option < Gd < crate::classes::Window >> > + 'a,) -> ExPopupDialogCentered < 'a > {
            ExPopupDialogCentered::new(self, dialog,)
        }
        pub(crate) fn popup_dialog_centered_ratio_full(&mut self, dialog: CowArg < Option < Gd < crate::classes::Window > > >, ratio: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Window > > >, f32,);
            let args = (dialog, ratio,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_dialog_centered_ratio", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_dialog_centered_ratio_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_dialog_centered_ratio(&mut self, dialog: impl AsArg < Option < Gd < crate::classes::Window >> >,) {
            self.popup_dialog_centered_ratio_ex(dialog,) . done()
        }
        #[inline]
        pub fn popup_dialog_centered_ratio_ex < 'a > (&'a mut self, dialog: impl AsArg < Option < Gd < crate::classes::Window >> > + 'a,) -> ExPopupDialogCenteredRatio < 'a > {
            ExPopupDialogCenteredRatio::new(self, dialog,)
        }
        pub(crate) fn popup_dialog_centered_clamped_full(&mut self, dialog: CowArg < Option < Gd < crate::classes::Window > > >, minsize: Vector2i, fallback_ratio: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Window > > >, Vector2i, f32,);
            let args = (dialog, minsize, fallback_ratio,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_dialog_centered_clamped", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_dialog_centered_clamped_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_dialog_centered_clamped(&mut self, dialog: impl AsArg < Option < Gd < crate::classes::Window >> >,) {
            self.popup_dialog_centered_clamped_ex(dialog,) . done()
        }
        #[inline]
        pub fn popup_dialog_centered_clamped_ex < 'a > (&'a mut self, dialog: impl AsArg < Option < Gd < crate::classes::Window >> > + 'a,) -> ExPopupDialogCenteredClamped < 'a > {
            ExPopupDialogCenteredClamped::new(self, dialog,)
        }
        pub fn get_current_feature_profile(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_current_feature_profile", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_current_feature_profile(&mut self, profile_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (profile_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "set_current_feature_profile", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn popup_node_selector_full(&mut self, callback: RefArg < Callable >, valid_types: RefArg < Array < StringName > >, current_value: CowArg < Option < Gd < crate::classes::Node > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, Callable >, RefArg < 'a1, Array < StringName > >, CowArg < 'a2, Option < Gd < crate::classes::Node > > >,);
            let args = (callback, valid_types, current_value,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_node_selector", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_node_selector_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_node_selector(&mut self, callback: &Callable,) {
            self.popup_node_selector_ex(callback,) . done()
        }
        #[inline]
        pub fn popup_node_selector_ex < 'a > (&'a mut self, callback: &'a Callable,) -> ExPopupNodeSelector < 'a > {
            ExPopupNodeSelector::new(self, callback,)
        }
        pub(crate) fn popup_property_selector_full(&mut self, object: CowArg < Option < Gd < crate::classes::Object > > >, callback: RefArg < Callable >, type_filter: RefArg < PackedInt32Array >, current_value: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, RefArg < 'a1, Callable >, RefArg < 'a2, PackedInt32Array >, CowArg < 'a3, GString >,);
            let args = (object, callback, type_filter, current_value,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_property_selector", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_property_selector_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_property_selector(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, callback: &Callable,) {
            self.popup_property_selector_ex(object, callback,) . done()
        }
        #[inline]
        pub fn popup_property_selector_ex < 'a > (&'a mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> > + 'a, callback: &'a Callable,) -> ExPopupPropertySelector < 'a > {
            ExPopupPropertySelector::new(self, object, callback,)
        }
        pub(crate) fn popup_method_selector_full(&mut self, object: CowArg < Option < Gd < crate::classes::Object > > >, callback: RefArg < Callable >, current_value: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, RefArg < 'a1, Callable >, CowArg < 'a2, GString >,);
            let args = (object, callback, current_value,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_method_selector", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_method_selector_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_method_selector(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, callback: &Callable,) {
            self.popup_method_selector_ex(object, callback,) . done()
        }
        #[inline]
        pub fn popup_method_selector_ex < 'a > (&'a mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> > + 'a, callback: &'a Callable,) -> ExPopupMethodSelector < 'a > {
            ExPopupMethodSelector::new(self, object, callback,)
        }
        pub(crate) fn popup_quick_open_full(&mut self, callback: RefArg < Callable >, base_types: RefArg < Array < StringName > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Callable >, RefArg < 'a1, Array < StringName > >,);
            let args = (callback, base_types,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_quick_open", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_quick_open_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_quick_open(&mut self, callback: &Callable,) {
            self.popup_quick_open_ex(callback,) . done()
        }
        #[inline]
        pub fn popup_quick_open_ex < 'a > (&'a mut self, callback: &'a Callable,) -> ExPopupQuickOpen < 'a > {
            ExPopupQuickOpen::new(self, callback,)
        }
        pub(crate) fn popup_create_dialog_full(&mut self, callback: RefArg < Callable >, base_type: CowArg < StringName >, current_type: CowArg < GString >, dialog_title: CowArg < GString >, type_blocklist: RefArg < Array < StringName > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (RefArg < 'a0, Callable >, CowArg < 'a1, StringName >, CowArg < 'a2, GString >, CowArg < 'a3, GString >, RefArg < 'a4, Array < StringName > >,);
            let args = (callback, base_type, current_type, dialog_title, type_blocklist,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "popup_create_dialog", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::popup_create_dialog_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn popup_create_dialog(&mut self, callback: &Callable,) {
            self.popup_create_dialog_ex(callback,) . done()
        }
        #[inline]
        pub fn popup_create_dialog_ex < 'a > (&'a mut self, callback: &'a Callable,) -> ExPopupCreateDialog < 'a > {
            ExPopupCreateDialog::new(self, callback,)
        }
        pub fn get_file_system_dock(&self,) -> Option < Gd < crate::classes::FileSystemDock > > {
            type CallRet = Option < Gd < crate::classes::FileSystemDock > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_file_system_dock", Some(self.__validated_obj()), args,)
            }
        }
        pub fn select_file(&mut self, file: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "select_file", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selected_paths(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_selected_paths", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_current_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_directory(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_current_directory", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inspector(&self,) -> Option < Gd < crate::classes::EditorInspector > > {
            type CallRet = Option < Gd < crate::classes::EditorInspector > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_inspector", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn inspect_object_full(&mut self, object: CowArg < Option < Gd < crate::classes::Object > > >, for_property: CowArg < GString >, inspector_only: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, CowArg < 'a1, GString >, bool,);
            let args = (object, for_property, inspector_only,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "inspect_object", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::inspect_object_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn inspect_object(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >,) {
            self.inspect_object_ex(object,) . done()
        }
        #[inline]
        pub fn inspect_object_ex < 'a > (&'a mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> > + 'a,) -> ExInspectObject < 'a > {
            ExInspectObject::new(self, object,)
        }
        pub fn edit_resource(&mut self, resource: impl AsArg < Option < Gd < crate::classes::Resource >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Resource > > >,);
            let args = (resource.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "edit_resource", Some(self.__validated_obj()), args,)
            }
        }
        pub fn edit_node(&mut self, node: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "edit_node", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn edit_script_full(&mut self, script: CowArg < Option < Gd < crate::classes::Script > > >, line: i32, column: i32, grab_focus: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Script > > >, i32, i32, bool,);
            let args = (script, line, column, grab_focus,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "edit_script", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::edit_script_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn edit_script(&mut self, script: impl AsArg < Option < Gd < crate::classes::Script >> >,) {
            self.edit_script_ex(script,) . done()
        }
        #[inline]
        pub fn edit_script_ex < 'a > (&'a mut self, script: impl AsArg < Option < Gd < crate::classes::Script >> > + 'a,) -> ExEditScript < 'a > {
            ExEditScript::new(self, script,)
        }
        pub(crate) fn open_scene_from_path_full(&mut self, scene_filepath: CowArg < GString >, set_inherited: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (scene_filepath, set_inherited,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "open_scene_from_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::open_scene_from_path_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn open_scene_from_path(&mut self, scene_filepath: impl AsArg < GString >,) {
            self.open_scene_from_path_ex(scene_filepath,) . done()
        }
        #[inline]
        pub fn open_scene_from_path_ex < 'a > (&'a mut self, scene_filepath: impl AsArg < GString > + 'a,) -> ExOpenSceneFromPath < 'a > {
            ExOpenSceneFromPath::new(self, scene_filepath,)
        }
        pub fn reload_scene_from_path(&mut self, scene_filepath: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (scene_filepath.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "reload_scene_from_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_open_scenes(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_open_scenes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_open_scene_roots(&self,) -> Array < Gd < crate::classes::Node > > {
            type CallRet = Array < Gd < crate::classes::Node > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_open_scene_roots", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_edited_scene_root(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_edited_scene_root", Some(self.__validated_obj()), args,)
            }
        }
        pub fn save_scene(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "save_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn save_scene_as_full(&mut self, path: CowArg < GString >, with_preview: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (path, with_preview,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "save_scene_as", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::save_scene_as_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn save_scene_as(&mut self, path: impl AsArg < GString >,) {
            self.save_scene_as_ex(path,) . done()
        }
        #[inline]
        pub fn save_scene_as_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a,) -> ExSaveSceneAs < 'a > {
            ExSaveSceneAs::new(self, path,)
        }
        pub fn save_all_scenes(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "save_all_scenes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn close_scene(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "close_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mark_scene_as_unsaved(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "mark_scene_as_unsaved", Some(self.__validated_obj()), args,)
            }
        }
        pub fn play_main_scene(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "play_main_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn play_current_scene(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "play_current_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn play_custom_scene(&mut self, scene_filepath: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (scene_filepath.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "play_custom_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn stop_playing_scene(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "stop_playing_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_playing_scene(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "is_playing_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_playing_scene(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "get_playing_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_movie_maker_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "set_movie_maker_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_movie_maker_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInterface", "is_movie_maker_enabled", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EditorInterface {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorInterface"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorInterface {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorInterface {
        
    }
    impl crate::obj::Singleton for EditorInterface {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"EditorInterface"))
            }
        }
    }
    impl std::ops::Deref for EditorInterface {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorInterface__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `EditorInterface` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`EditorInterface::restart_editor_ex`][super::EditorInterface::restart_editor_ex]."]
#[must_use]
pub struct ExRestartEditor < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, save: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRestartEditor < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface,) -> Self {
        let save = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, save: save,
        }
    }
    #[inline]
    pub fn save(self, save: bool) -> Self {
        Self {
            save: save, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, save,
        }
        = self;
        re_export::EditorInterface::restart_editor_full(surround_object, save,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::get_editor_viewport_3d_ex`][super::EditorInterface::get_editor_viewport_3d_ex]."]
#[must_use]
pub struct ExGetEditorViewport3d < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::EditorInterface, idx: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetEditorViewport3d < 'a > {
    fn new(surround_object: &'a re_export::EditorInterface,) -> Self {
        let idx = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, idx: idx,
        }
    }
    #[inline]
    pub fn idx(self, idx: i32) -> Self {
        Self {
            idx: idx, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::SubViewport > > {
        let Self {
            _phantom, surround_object, idx,
        }
        = self;
        re_export::EditorInterface::get_editor_viewport_3d_full(surround_object, idx,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_dialog_ex`][super::EditorInterface::popup_dialog_ex]."]
#[must_use]
pub struct ExPopupDialog < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, dialog: CowArg < 'a, Option < Gd < crate::classes::Window > > >, rect: Rect2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupDialog < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, dialog: impl AsArg < Option < Gd < crate::classes::Window >> > + 'a,) -> Self {
        let rect = Rect2i::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, dialog: dialog.into_arg(), rect: rect,
        }
    }
    #[inline]
    pub fn rect(self, rect: Rect2i) -> Self {
        Self {
            rect: rect, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, dialog, rect,
        }
        = self;
        re_export::EditorInterface::popup_dialog_full(surround_object, dialog, rect,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_dialog_centered_ex`][super::EditorInterface::popup_dialog_centered_ex]."]
#[must_use]
pub struct ExPopupDialogCentered < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, dialog: CowArg < 'a, Option < Gd < crate::classes::Window > > >, minsize: Vector2i,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupDialogCentered < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, dialog: impl AsArg < Option < Gd < crate::classes::Window >> > + 'a,) -> Self {
        let minsize = Vector2i::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, dialog: dialog.into_arg(), minsize: minsize,
        }
    }
    #[inline]
    pub fn minsize(self, minsize: Vector2i) -> Self {
        Self {
            minsize: minsize, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, dialog, minsize,
        }
        = self;
        re_export::EditorInterface::popup_dialog_centered_full(surround_object, dialog, minsize,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_dialog_centered_ratio_ex`][super::EditorInterface::popup_dialog_centered_ratio_ex]."]
#[must_use]
pub struct ExPopupDialogCenteredRatio < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, dialog: CowArg < 'a, Option < Gd < crate::classes::Window > > >, ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupDialogCenteredRatio < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, dialog: impl AsArg < Option < Gd < crate::classes::Window >> > + 'a,) -> Self {
        let ratio = 0.8f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, dialog: dialog.into_arg(), ratio: ratio,
        }
    }
    #[inline]
    pub fn ratio(self, ratio: f32) -> Self {
        Self {
            ratio: ratio, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, dialog, ratio,
        }
        = self;
        re_export::EditorInterface::popup_dialog_centered_ratio_full(surround_object, dialog, ratio,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_dialog_centered_clamped_ex`][super::EditorInterface::popup_dialog_centered_clamped_ex]."]
#[must_use]
pub struct ExPopupDialogCenteredClamped < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, dialog: CowArg < 'a, Option < Gd < crate::classes::Window > > >, minsize: Vector2i, fallback_ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupDialogCenteredClamped < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, dialog: impl AsArg < Option < Gd < crate::classes::Window >> > + 'a,) -> Self {
        let minsize = Vector2i::new(0 as _, 0 as _);
        let fallback_ratio = 0.75f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, dialog: dialog.into_arg(), minsize: minsize, fallback_ratio: fallback_ratio,
        }
    }
    #[inline]
    pub fn minsize(self, minsize: Vector2i) -> Self {
        Self {
            minsize: minsize, .. self
        }
    }
    #[inline]
    pub fn fallback_ratio(self, fallback_ratio: f32) -> Self {
        Self {
            fallback_ratio: fallback_ratio, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, dialog, minsize, fallback_ratio,
        }
        = self;
        re_export::EditorInterface::popup_dialog_centered_clamped_full(surround_object, dialog, minsize, fallback_ratio,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_node_selector_ex`][super::EditorInterface::popup_node_selector_ex]."]
#[must_use]
pub struct ExPopupNodeSelector < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, callback: CowArg < 'a, Callable >, valid_types: CowArg < 'a, Array < StringName > >, current_value: CowArg < 'a, Option < Gd < crate::classes::Node > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupNodeSelector < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, callback: &'a Callable,) -> Self {
        let valid_types = Array::new();
        let current_value = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), valid_types: CowArg::Owned(valid_types), current_value: current_value.into_arg(),
        }
    }
    #[inline]
    pub fn valid_types(self, valid_types: &'a Array < StringName >) -> Self {
        Self {
            valid_types: CowArg::Borrowed(valid_types), .. self
        }
    }
    #[inline]
    pub fn current_value(self, current_value: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a) -> Self {
        Self {
            current_value: current_value.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, valid_types, current_value,
        }
        = self;
        re_export::EditorInterface::popup_node_selector_full(surround_object, callback.cow_as_arg(), valid_types.cow_as_arg(), current_value,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_property_selector_ex`][super::EditorInterface::popup_property_selector_ex]."]
#[must_use]
pub struct ExPopupPropertySelector < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, object: CowArg < 'a, Option < Gd < crate::classes::Object > > >, callback: CowArg < 'a, Callable >, type_filter: CowArg < 'a, PackedInt32Array >, current_value: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupPropertySelector < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, object: impl AsArg < Option < Gd < crate::classes::Object >> > + 'a, callback: &'a Callable,) -> Self {
        let type_filter = PackedInt32Array::new();
        let current_value = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, object: object.into_arg(), callback: CowArg::Borrowed(callback), type_filter: CowArg::Owned(type_filter), current_value: CowArg::Owned(current_value),
        }
    }
    #[inline]
    pub fn type_filter(self, type_filter: &'a PackedInt32Array) -> Self {
        Self {
            type_filter: CowArg::Borrowed(type_filter), .. self
        }
    }
    #[inline]
    pub fn current_value(self, current_value: impl AsArg < GString > + 'a) -> Self {
        Self {
            current_value: current_value.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, object, callback, type_filter, current_value,
        }
        = self;
        re_export::EditorInterface::popup_property_selector_full(surround_object, object, callback.cow_as_arg(), type_filter.cow_as_arg(), current_value,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_method_selector_ex`][super::EditorInterface::popup_method_selector_ex]."]
#[must_use]
pub struct ExPopupMethodSelector < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, object: CowArg < 'a, Option < Gd < crate::classes::Object > > >, callback: CowArg < 'a, Callable >, current_value: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupMethodSelector < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, object: impl AsArg < Option < Gd < crate::classes::Object >> > + 'a, callback: &'a Callable,) -> Self {
        let current_value = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, object: object.into_arg(), callback: CowArg::Borrowed(callback), current_value: CowArg::Owned(current_value),
        }
    }
    #[inline]
    pub fn current_value(self, current_value: impl AsArg < GString > + 'a) -> Self {
        Self {
            current_value: current_value.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, object, callback, current_value,
        }
        = self;
        re_export::EditorInterface::popup_method_selector_full(surround_object, object, callback.cow_as_arg(), current_value,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_quick_open_ex`][super::EditorInterface::popup_quick_open_ex]."]
#[must_use]
pub struct ExPopupQuickOpen < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, callback: CowArg < 'a, Callable >, base_types: CowArg < 'a, Array < StringName > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupQuickOpen < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, callback: &'a Callable,) -> Self {
        let base_types = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), base_types: CowArg::Owned(base_types),
        }
    }
    #[inline]
    pub fn base_types(self, base_types: &'a Array < StringName >) -> Self {
        Self {
            base_types: CowArg::Borrowed(base_types), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, base_types,
        }
        = self;
        re_export::EditorInterface::popup_quick_open_full(surround_object, callback.cow_as_arg(), base_types.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`EditorInterface::popup_create_dialog_ex`][super::EditorInterface::popup_create_dialog_ex]."]
#[must_use]
pub struct ExPopupCreateDialog < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, callback: CowArg < 'a, Callable >, base_type: CowArg < 'a, StringName >, current_type: CowArg < 'a, GString >, dialog_title: CowArg < 'a, GString >, type_blocklist: CowArg < 'a, Array < StringName > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPopupCreateDialog < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, callback: &'a Callable,) -> Self {
        let base_type = StringName::from("");
        let current_type = GString::from("");
        let dialog_title = GString::from("");
        let type_blocklist = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, callback: CowArg::Borrowed(callback), base_type: CowArg::Owned(base_type), current_type: CowArg::Owned(current_type), dialog_title: CowArg::Owned(dialog_title), type_blocklist: CowArg::Owned(type_blocklist),
        }
    }
    #[inline]
    pub fn base_type(self, base_type: impl AsArg < StringName > + 'a) -> Self {
        Self {
            base_type: base_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn current_type(self, current_type: impl AsArg < GString > + 'a) -> Self {
        Self {
            current_type: current_type.into_arg(), .. self
        }
    }
    #[inline]
    pub fn dialog_title(self, dialog_title: impl AsArg < GString > + 'a) -> Self {
        Self {
            dialog_title: dialog_title.into_arg(), .. self
        }
    }
    #[inline]
    pub fn type_blocklist(self, type_blocklist: &'a Array < StringName >) -> Self {
        Self {
            type_blocklist: CowArg::Borrowed(type_blocklist), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, callback, base_type, current_type, dialog_title, type_blocklist,
        }
        = self;
        re_export::EditorInterface::popup_create_dialog_full(surround_object, callback.cow_as_arg(), base_type, current_type, dialog_title, type_blocklist.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`EditorInterface::inspect_object_ex`][super::EditorInterface::inspect_object_ex]."]
#[must_use]
pub struct ExInspectObject < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, object: CowArg < 'a, Option < Gd < crate::classes::Object > > >, for_property: CowArg < 'a, GString >, inspector_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExInspectObject < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, object: impl AsArg < Option < Gd < crate::classes::Object >> > + 'a,) -> Self {
        let for_property = GString::from("");
        let inspector_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, object: object.into_arg(), for_property: CowArg::Owned(for_property), inspector_only: inspector_only,
        }
    }
    #[inline]
    pub fn for_property(self, for_property: impl AsArg < GString > + 'a) -> Self {
        Self {
            for_property: for_property.into_arg(), .. self
        }
    }
    #[inline]
    pub fn inspector_only(self, inspector_only: bool) -> Self {
        Self {
            inspector_only: inspector_only, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, object, for_property, inspector_only,
        }
        = self;
        re_export::EditorInterface::inspect_object_full(surround_object, object, for_property, inspector_only,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::edit_script_ex`][super::EditorInterface::edit_script_ex]."]
#[must_use]
pub struct ExEditScript < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, script: CowArg < 'a, Option < Gd < crate::classes::Script > > >, line: i32, column: i32, grab_focus: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExEditScript < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, script: impl AsArg < Option < Gd < crate::classes::Script >> > + 'a,) -> Self {
        let line = - 1i32;
        let column = 0i32;
        let grab_focus = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, script: script.into_arg(), line: line, column: column, grab_focus: grab_focus,
        }
    }
    #[inline]
    pub fn line(self, line: i32) -> Self {
        Self {
            line: line, .. self
        }
    }
    #[inline]
    pub fn column(self, column: i32) -> Self {
        Self {
            column: column, .. self
        }
    }
    #[inline]
    pub fn grab_focus(self, grab_focus: bool) -> Self {
        Self {
            grab_focus: grab_focus, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, script, line, column, grab_focus,
        }
        = self;
        re_export::EditorInterface::edit_script_full(surround_object, script, line, column, grab_focus,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::open_scene_from_path_ex`][super::EditorInterface::open_scene_from_path_ex]."]
#[must_use]
pub struct ExOpenSceneFromPath < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, scene_filepath: CowArg < 'a, GString >, set_inherited: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExOpenSceneFromPath < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, scene_filepath: impl AsArg < GString > + 'a,) -> Self {
        let set_inherited = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, scene_filepath: scene_filepath.into_arg(), set_inherited: set_inherited,
        }
    }
    #[inline]
    pub fn set_inherited(self, set_inherited: bool) -> Self {
        Self {
            set_inherited: set_inherited, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, scene_filepath, set_inherited,
        }
        = self;
        re_export::EditorInterface::open_scene_from_path_full(surround_object, scene_filepath, set_inherited,)
    }
}
#[doc = "Default-param extender for [`EditorInterface::save_scene_as_ex`][super::EditorInterface::save_scene_as_ex]."]
#[must_use]
pub struct ExSaveSceneAs < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorInterface, path: CowArg < 'a, GString >, with_preview: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveSceneAs < 'a > {
    fn new(surround_object: &'a mut re_export::EditorInterface, path: impl AsArg < GString > + 'a,) -> Self {
        let with_preview = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), with_preview: with_preview,
        }
    }
    #[inline]
    pub fn with_preview(self, with_preview: bool) -> Self {
        Self {
            with_preview: with_preview, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, path, with_preview,
        }
        = self;
        re_export::EditorInterface::save_scene_as_full(surround_object, path, with_preview,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorInterface;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EditorInterface {
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