#![doc = "Sidecar module for class [`EditorExportPlatform`][crate::classes::EditorExportPlatform].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorExportPlatform` enums](https://docs.godotengine.org/en/stable/classes/class_editorexportplatform.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorExportPlatform.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`editor_export_platform`][crate::classes::editor_export_platform]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `EditorExportPlatform`](https://docs.godotengine.org/en/stable/classes/class_editorexportplatform.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<EditorExportPlatform>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorExportPlatform {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl EditorExportPlatform {
        pub fn get_os_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(16usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_os_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_preset(&mut self,) -> Option < Gd < crate::classes::EditorExportPreset > > {
            type CallRet = Option < Gd < crate::classes::EditorExportPreset > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(17usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "create_preset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn find_export_template(&self, template_file_name: impl AsArg < GString >,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (template_file_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(18usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "find_export_template", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_presets(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(19usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_current_presets", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn save_pack_full(&mut self, preset: CowArg < Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < GString >, embed: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >, bool,);
            let args = (preset, debug, path, embed,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(20usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "save_pack", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::save_pack_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn save_pack(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> VarDictionary {
            self.save_pack_ex(preset, debug, path,) . done()
        }
        #[inline]
        pub fn save_pack_ex < 'a > (&'a mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'a, debug: bool, path: impl AsArg < GString > + 'a,) -> ExSavePack < 'a > {
            ExSavePack::new(self, preset, debug, path,)
        }
        pub fn save_zip(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >,);
            let args = (preset.into_arg(), debug, path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(21usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "save_zip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn save_pack_patch(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >,);
            let args = (preset.into_arg(), debug, path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(22usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "save_pack_patch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn save_zip_patch(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >,);
            let args = (preset.into_arg(), debug, path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(23usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "save_zip_patch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn gen_export_flags(&mut self, flags: crate::classes::editor_export_platform::DebugFlags,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = (crate::classes::editor_export_platform::DebugFlags,);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(24usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "gen_export_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn export_project_files_full(&mut self, preset: CowArg < Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, save_cb: RefArg < Callable >, shared_cb: RefArg < Callable >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, RefArg < 'a1, Callable >, RefArg < 'a2, Callable >,);
            let args = (preset, debug, save_cb, shared_cb,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(25usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "export_project_files", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::export_project_files_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn export_project_files(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, save_cb: &Callable,) -> crate::global::Error {
            self.export_project_files_ex(preset, debug, save_cb,) . done()
        }
        #[inline]
        pub fn export_project_files_ex < 'a > (&'a mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'a, debug: bool, save_cb: &'a Callable,) -> ExExportProjectFiles < 'a > {
            ExExportProjectFiles::new(self, preset, debug, save_cb,)
        }
        pub(crate) fn export_project_full(&mut self, preset: CowArg < Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < GString >, flags: crate::classes::editor_export_platform::DebugFlags,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >, crate::classes::editor_export_platform::DebugFlags,);
            let args = (preset, debug, path, flags,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(26usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "export_project", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::export_project_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn export_project(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> crate::global::Error {
            self.export_project_ex(preset, debug, path,) . done()
        }
        #[inline]
        pub fn export_project_ex < 'a > (&'a mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'a, debug: bool, path: impl AsArg < GString > + 'a,) -> ExExportProject < 'a > {
            ExExportProject::new(self, preset, debug, path,)
        }
        pub(crate) fn export_pack_full(&mut self, preset: CowArg < Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < GString >, flags: crate::classes::editor_export_platform::DebugFlags,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >, crate::classes::editor_export_platform::DebugFlags,);
            let args = (preset, debug, path, flags,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(27usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "export_pack", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::export_pack_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn export_pack(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> crate::global::Error {
            self.export_pack_ex(preset, debug, path,) . done()
        }
        #[inline]
        pub fn export_pack_ex < 'a > (&'a mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'a, debug: bool, path: impl AsArg < GString > + 'a,) -> ExExportPack < 'a > {
            ExExportPack::new(self, preset, debug, path,)
        }
        pub(crate) fn export_zip_full(&mut self, preset: CowArg < Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < GString >, flags: crate::classes::editor_export_platform::DebugFlags,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >, crate::classes::editor_export_platform::DebugFlags,);
            let args = (preset, debug, path, flags,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(28usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "export_zip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::export_zip_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn export_zip(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> crate::global::Error {
            self.export_zip_ex(preset, debug, path,) . done()
        }
        #[inline]
        pub fn export_zip_ex < 'a > (&'a mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'a, debug: bool, path: impl AsArg < GString > + 'a,) -> ExExportZip < 'a > {
            ExExportZip::new(self, preset, debug, path,)
        }
        pub(crate) fn export_pack_patch_full(&mut self, preset: CowArg < Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < GString >, patches: RefArg < PackedStringArray >, flags: crate::classes::editor_export_platform::DebugFlags,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >, RefArg < 'a2, PackedStringArray >, crate::classes::editor_export_platform::DebugFlags,);
            let args = (preset, debug, path, patches, flags,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(29usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "export_pack_patch", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::export_pack_patch_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn export_pack_patch(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> crate::global::Error {
            self.export_pack_patch_ex(preset, debug, path,) . done()
        }
        #[inline]
        pub fn export_pack_patch_ex < 'a > (&'a mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'a, debug: bool, path: impl AsArg < GString > + 'a,) -> ExExportPackPatch < 'a > {
            ExExportPackPatch::new(self, preset, debug, path,)
        }
        pub(crate) fn export_zip_patch_full(&mut self, preset: CowArg < Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < GString >, patches: RefArg < PackedStringArray >, flags: crate::classes::editor_export_platform::DebugFlags,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool, CowArg < 'a1, GString >, RefArg < 'a2, PackedStringArray >, crate::classes::editor_export_platform::DebugFlags,);
            let args = (preset, debug, path, patches, flags,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(30usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "export_zip_patch", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::export_zip_patch_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn export_zip_patch(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool, path: impl AsArg < GString >,) -> crate::global::Error {
            self.export_zip_patch_ex(preset, debug, path,) . done()
        }
        #[inline]
        pub fn export_zip_patch_ex < 'a > (&'a mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'a, debug: bool, path: impl AsArg < GString > + 'a,) -> ExExportZipPatch < 'a > {
            ExExportZipPatch::new(self, preset, debug, path,)
        }
        pub fn clear_messages(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(31usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "clear_messages", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_message(&mut self, type_: crate::classes::editor_export_platform::ExportMessageType, category: impl AsArg < GString >, message: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (crate::classes::editor_export_platform::ExportMessageType, CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (type_, category.into_arg(), message.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(32usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "add_message", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_message_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(33usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_message_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_message_type(&self, index: i32,) -> crate::classes::editor_export_platform::ExportMessageType {
            type CallRet = crate::classes::editor_export_platform::ExportMessageType;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(34usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_message_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_message_category(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(35usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_message_category", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_message_text(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(36usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_message_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_worst_message_type(&self,) -> crate::classes::editor_export_platform::ExportMessageType {
            type CallRet = crate::classes::editor_export_platform::ExportMessageType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(37usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_worst_message_type", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn ssh_run_on_remote_full(&self, host: CowArg < GString >, port: CowArg < GString >, ssh_arg: RefArg < PackedStringArray >, cmd_args: CowArg < GString >, output: RefArg < VarArray >, port_fwd: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, PackedStringArray >, CowArg < 'a3, GString >, RefArg < 'a4, VarArray >, i32,);
            let args = (host, port, ssh_arg, cmd_args, output, port_fwd,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(38usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "ssh_run_on_remote", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::ssh_run_on_remote_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn ssh_run_on_remote(&self, host: impl AsArg < GString >, port: impl AsArg < GString >, ssh_arg: &PackedStringArray, cmd_args: impl AsArg < GString >,) -> crate::global::Error {
            self.ssh_run_on_remote_ex(host, port, ssh_arg, cmd_args,) . done()
        }
        #[inline]
        pub fn ssh_run_on_remote_ex < 'a > (&'a self, host: impl AsArg < GString > + 'a, port: impl AsArg < GString > + 'a, ssh_arg: &'a PackedStringArray, cmd_args: impl AsArg < GString > + 'a,) -> ExSshRunOnRemote < 'a > {
            ExSshRunOnRemote::new(self, host, port, ssh_arg, cmd_args,)
        }
        pub(crate) fn ssh_run_on_remote_no_wait_full(&self, host: CowArg < GString >, port: CowArg < GString >, ssh_args: RefArg < PackedStringArray >, cmd_args: CowArg < GString >, port_fwd: i32,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, PackedStringArray >, CowArg < 'a3, GString >, i32,);
            let args = (host, port, ssh_args, cmd_args, port_fwd,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(39usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "ssh_run_on_remote_no_wait", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::ssh_run_on_remote_no_wait_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn ssh_run_on_remote_no_wait(&self, host: impl AsArg < GString >, port: impl AsArg < GString >, ssh_args: &PackedStringArray, cmd_args: impl AsArg < GString >,) -> i64 {
            self.ssh_run_on_remote_no_wait_ex(host, port, ssh_args, cmd_args,) . done()
        }
        #[inline]
        pub fn ssh_run_on_remote_no_wait_ex < 'a > (&'a self, host: impl AsArg < GString > + 'a, port: impl AsArg < GString > + 'a, ssh_args: &'a PackedStringArray, cmd_args: impl AsArg < GString > + 'a,) -> ExSshRunOnRemoteNoWait < 'a > {
            ExSshRunOnRemoteNoWait::new(self, host, port, ssh_args, cmd_args,)
        }
        pub fn ssh_push_to_remote(&self, host: impl AsArg < GString >, port: impl AsArg < GString >, scp_args: &PackedStringArray, src_file: impl AsArg < GString >, dst_file: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, PackedStringArray >, CowArg < 'a3, GString >, CowArg < 'a4, GString >,);
            let args = (host.into_arg(), port.into_arg(), RefArg::new(scp_args), src_file.into_arg(), dst_file.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(40usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "ssh_push_to_remote", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_internal_export_files(&mut self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> >, debug: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >, bool,);
            let args = (preset.into_arg(), debug,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(41usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_internal_export_files", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_forced_export_files_full(preset: CowArg < Option < Gd < crate::classes::EditorExportPreset > > >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorExportPreset > > >,);
            let args = (preset,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(42usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPlatform", "get_forced_export_files", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_forced_export_files_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_forced_export_files() -> PackedStringArray {
            Self::get_forced_export_files_ex() . done()
        }
        #[inline]
        pub fn get_forced_export_files_ex < 'a > () -> ExGetForcedExportFiles < 'a > {
            ExGetForcedExportFiles::new()
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
    impl crate::obj::GodotClass for EditorExportPlatform {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorExportPlatform"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorExportPlatform {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorExportPlatform {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorExportPlatform {
        
    }
    impl std::ops::Deref for EditorExportPlatform {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorExportPlatform {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorExportPlatform__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `EditorExportPlatform` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::save_pack_ex`][super::EditorExportPlatform::save_pack_ex]."]
#[must_use]
pub struct ExSavePack < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorExportPlatform, preset: CowArg < 'a, Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < 'a, GString >, embed: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSavePack < 'a > {
    fn new(surround_object: &'a mut re_export::EditorExportPlatform, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'a, debug: bool, path: impl AsArg < GString > + 'a,) -> Self {
        let embed = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset.into_arg(), debug: debug, path: path.into_arg(), embed: embed,
        }
    }
    #[inline]
    pub fn embed(self, embed: bool) -> Self {
        Self {
            embed: embed, .. self
        }
    }
    #[inline]
    pub fn done(self) -> VarDictionary {
        let Self {
            _phantom, surround_object, preset, debug, path, embed,
        }
        = self;
        re_export::EditorExportPlatform::save_pack_full(surround_object, preset, debug, path, embed,)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::export_project_files_ex`][super::EditorExportPlatform::export_project_files_ex]."]
#[must_use]
pub struct ExExportProjectFiles < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorExportPlatform, preset: CowArg < 'a, Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, save_cb: CowArg < 'a, Callable >, shared_cb: CowArg < 'a, Callable >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExExportProjectFiles < 'a > {
    fn new(surround_object: &'a mut re_export::EditorExportPlatform, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'a, debug: bool, save_cb: &'a Callable,) -> Self {
        let shared_cb = Callable::invalid();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset.into_arg(), debug: debug, save_cb: CowArg::Borrowed(save_cb), shared_cb: CowArg::Owned(shared_cb),
        }
    }
    #[inline]
    pub fn shared_cb(self, shared_cb: &'a Callable) -> Self {
        Self {
            shared_cb: CowArg::Borrowed(shared_cb), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, preset, debug, save_cb, shared_cb,
        }
        = self;
        re_export::EditorExportPlatform::export_project_files_full(surround_object, preset, debug, save_cb.cow_as_arg(), shared_cb.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::export_project_ex`][super::EditorExportPlatform::export_project_ex]."]
#[must_use]
pub struct ExExportProject < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorExportPlatform, preset: CowArg < 'a, Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < 'a, GString >, flags: crate::classes::editor_export_platform::DebugFlags,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExExportProject < 'a > {
    fn new(surround_object: &'a mut re_export::EditorExportPlatform, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'a, debug: bool, path: impl AsArg < GString > + 'a,) -> Self {
        let flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset.into_arg(), debug: debug, path: path.into_arg(), flags: flags,
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::editor_export_platform::DebugFlags) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, preset, debug, path, flags,
        }
        = self;
        re_export::EditorExportPlatform::export_project_full(surround_object, preset, debug, path, flags,)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::export_pack_ex`][super::EditorExportPlatform::export_pack_ex]."]
#[must_use]
pub struct ExExportPack < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorExportPlatform, preset: CowArg < 'a, Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < 'a, GString >, flags: crate::classes::editor_export_platform::DebugFlags,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExExportPack < 'a > {
    fn new(surround_object: &'a mut re_export::EditorExportPlatform, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'a, debug: bool, path: impl AsArg < GString > + 'a,) -> Self {
        let flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset.into_arg(), debug: debug, path: path.into_arg(), flags: flags,
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::editor_export_platform::DebugFlags) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, preset, debug, path, flags,
        }
        = self;
        re_export::EditorExportPlatform::export_pack_full(surround_object, preset, debug, path, flags,)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::export_zip_ex`][super::EditorExportPlatform::export_zip_ex]."]
#[must_use]
pub struct ExExportZip < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorExportPlatform, preset: CowArg < 'a, Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < 'a, GString >, flags: crate::classes::editor_export_platform::DebugFlags,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExExportZip < 'a > {
    fn new(surround_object: &'a mut re_export::EditorExportPlatform, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'a, debug: bool, path: impl AsArg < GString > + 'a,) -> Self {
        let flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset.into_arg(), debug: debug, path: path.into_arg(), flags: flags,
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::editor_export_platform::DebugFlags) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, preset, debug, path, flags,
        }
        = self;
        re_export::EditorExportPlatform::export_zip_full(surround_object, preset, debug, path, flags,)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::export_pack_patch_ex`][super::EditorExportPlatform::export_pack_patch_ex]."]
#[must_use]
pub struct ExExportPackPatch < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorExportPlatform, preset: CowArg < 'a, Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < 'a, GString >, patches: CowArg < 'a, PackedStringArray >, flags: crate::classes::editor_export_platform::DebugFlags,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExExportPackPatch < 'a > {
    fn new(surround_object: &'a mut re_export::EditorExportPlatform, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'a, debug: bool, path: impl AsArg < GString > + 'a,) -> Self {
        let patches = PackedStringArray::new();
        let flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset.into_arg(), debug: debug, path: path.into_arg(), patches: CowArg::Owned(patches), flags: flags,
        }
    }
    #[inline]
    pub fn patches(self, patches: &'a PackedStringArray) -> Self {
        Self {
            patches: CowArg::Borrowed(patches), .. self
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::editor_export_platform::DebugFlags) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, preset, debug, path, patches, flags,
        }
        = self;
        re_export::EditorExportPlatform::export_pack_patch_full(surround_object, preset, debug, path, patches.cow_as_arg(), flags,)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::export_zip_patch_ex`][super::EditorExportPlatform::export_zip_patch_ex]."]
#[must_use]
pub struct ExExportZipPatch < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorExportPlatform, preset: CowArg < 'a, Option < Gd < crate::classes::EditorExportPreset > > >, debug: bool, path: CowArg < 'a, GString >, patches: CowArg < 'a, PackedStringArray >, flags: crate::classes::editor_export_platform::DebugFlags,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExExportZipPatch < 'a > {
    fn new(surround_object: &'a mut re_export::EditorExportPlatform, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'a, debug: bool, path: impl AsArg < GString > + 'a,) -> Self {
        let patches = PackedStringArray::new();
        let flags = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, preset: preset.into_arg(), debug: debug, path: path.into_arg(), patches: CowArg::Owned(patches), flags: flags,
        }
    }
    #[inline]
    pub fn patches(self, patches: &'a PackedStringArray) -> Self {
        Self {
            patches: CowArg::Borrowed(patches), .. self
        }
    }
    #[inline]
    pub fn flags(self, flags: crate::classes::editor_export_platform::DebugFlags) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, preset, debug, path, patches, flags,
        }
        = self;
        re_export::EditorExportPlatform::export_zip_patch_full(surround_object, preset, debug, path, patches.cow_as_arg(), flags,)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::ssh_run_on_remote_ex`][super::EditorExportPlatform::ssh_run_on_remote_ex]."]
#[must_use]
pub struct ExSshRunOnRemote < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::EditorExportPlatform, host: CowArg < 'a, GString >, port: CowArg < 'a, GString >, ssh_arg: CowArg < 'a, PackedStringArray >, cmd_args: CowArg < 'a, GString >, output: CowArg < 'a, VarArray >, port_fwd: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSshRunOnRemote < 'a > {
    fn new(surround_object: &'a re_export::EditorExportPlatform, host: impl AsArg < GString > + 'a, port: impl AsArg < GString > + 'a, ssh_arg: &'a PackedStringArray, cmd_args: impl AsArg < GString > + 'a,) -> Self {
        let output = Array::new();
        let port_fwd = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, host: host.into_arg(), port: port.into_arg(), ssh_arg: CowArg::Borrowed(ssh_arg), cmd_args: cmd_args.into_arg(), output: CowArg::Owned(output), port_fwd: port_fwd,
        }
    }
    #[inline]
    pub fn output(self, output: &'a VarArray) -> Self {
        Self {
            output: CowArg::Borrowed(output), .. self
        }
    }
    #[inline]
    pub fn port_fwd(self, port_fwd: i32) -> Self {
        Self {
            port_fwd: port_fwd, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, host, port, ssh_arg, cmd_args, output, port_fwd,
        }
        = self;
        re_export::EditorExportPlatform::ssh_run_on_remote_full(surround_object, host, port, ssh_arg.cow_as_arg(), cmd_args, output.cow_as_arg(), port_fwd,)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::ssh_run_on_remote_no_wait_ex`][super::EditorExportPlatform::ssh_run_on_remote_no_wait_ex]."]
#[must_use]
pub struct ExSshRunOnRemoteNoWait < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::EditorExportPlatform, host: CowArg < 'a, GString >, port: CowArg < 'a, GString >, ssh_args: CowArg < 'a, PackedStringArray >, cmd_args: CowArg < 'a, GString >, port_fwd: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSshRunOnRemoteNoWait < 'a > {
    fn new(surround_object: &'a re_export::EditorExportPlatform, host: impl AsArg < GString > + 'a, port: impl AsArg < GString > + 'a, ssh_args: &'a PackedStringArray, cmd_args: impl AsArg < GString > + 'a,) -> Self {
        let port_fwd = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, host: host.into_arg(), port: port.into_arg(), ssh_args: CowArg::Borrowed(ssh_args), cmd_args: cmd_args.into_arg(), port_fwd: port_fwd,
        }
    }
    #[inline]
    pub fn port_fwd(self, port_fwd: i32) -> Self {
        Self {
            port_fwd: port_fwd, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, host, port, ssh_args, cmd_args, port_fwd,
        }
        = self;
        re_export::EditorExportPlatform::ssh_run_on_remote_no_wait_full(surround_object, host, port, ssh_args.cow_as_arg(), cmd_args, port_fwd,)
    }
}
#[doc = "Default-param extender for [`EditorExportPlatform::get_forced_export_files_ex`][super::EditorExportPlatform::get_forced_export_files_ex]."]
#[must_use]
pub struct ExGetForcedExportFiles < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, preset: CowArg < 'a, Option < Gd < crate::classes::EditorExportPreset > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetForcedExportFiles < 'a > {
    fn new() -> Self {
        let preset = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, preset: preset.into_arg(),
        }
    }
    #[inline]
    pub fn preset(self, preset: impl AsArg < Option < Gd < crate::classes::EditorExportPreset >> > + 'a) -> Self {
        Self {
            preset: preset.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, preset,
        }
        = self;
        re_export::EditorExportPlatform::get_forced_export_files_full(preset,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ExportMessageType {
    ord: i32
}
impl ExportMessageType {
    #[doc(alias = "EXPORT_MESSAGE_NONE")]
    #[doc = "Godot enumerator name: `EXPORT_MESSAGE_NONE`"]
    pub const NONE: ExportMessageType = ExportMessageType {
        ord: 0i32
    };
    #[doc(alias = "EXPORT_MESSAGE_INFO")]
    #[doc = "Godot enumerator name: `EXPORT_MESSAGE_INFO`"]
    pub const INFO: ExportMessageType = ExportMessageType {
        ord: 1i32
    };
    #[doc(alias = "EXPORT_MESSAGE_WARNING")]
    #[doc = "Godot enumerator name: `EXPORT_MESSAGE_WARNING`"]
    pub const WARNING: ExportMessageType = ExportMessageType {
        ord: 2i32
    };
    #[doc(alias = "EXPORT_MESSAGE_ERROR")]
    #[doc = "Godot enumerator name: `EXPORT_MESSAGE_ERROR`"]
    pub const ERROR: ExportMessageType = ExportMessageType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ExportMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ExportMessageType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ExportMessageType {
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
            Self::NONE => "NONE", Self::INFO => "INFO", Self::WARNING => "WARNING", Self::ERROR => "ERROR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ExportMessageType::NONE, ExportMessageType::INFO, ExportMessageType::WARNING, ExportMessageType::ERROR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ExportMessageType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "EXPORT_MESSAGE_NONE", ExportMessageType::NONE), crate::meta::inspect::EnumConstant::new("INFO", "EXPORT_MESSAGE_INFO", ExportMessageType::INFO), crate::meta::inspect::EnumConstant::new("WARNING", "EXPORT_MESSAGE_WARNING", ExportMessageType::WARNING), crate::meta::inspect::EnumConstant::new("ERROR", "EXPORT_MESSAGE_ERROR", ExportMessageType::ERROR)]
        }
    }
}
impl crate::meta::GodotConvert for ExportMessageType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ExportMessageType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ExportMessageType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct DebugFlags {
    ord: u64
}
impl DebugFlags {
    #[doc(alias = "DEBUG_FLAG_DUMB_CLIENT")]
    #[doc = "Godot enumerator name: `DEBUG_FLAG_DUMB_CLIENT`"]
    pub const DUMB_CLIENT: DebugFlags = DebugFlags {
        ord: 1u64
    };
    #[doc(alias = "DEBUG_FLAG_REMOTE_DEBUG")]
    #[doc = "Godot enumerator name: `DEBUG_FLAG_REMOTE_DEBUG`"]
    pub const REMOTE_DEBUG: DebugFlags = DebugFlags {
        ord: 2u64
    };
    #[doc(alias = "DEBUG_FLAG_REMOTE_DEBUG_LOCALHOST")]
    #[doc = "Godot enumerator name: `DEBUG_FLAG_REMOTE_DEBUG_LOCALHOST`"]
    pub const REMOTE_DEBUG_LOCALHOST: DebugFlags = DebugFlags {
        ord: 4u64
    };
    #[doc(alias = "DEBUG_FLAG_VIEW_COLLISIONS")]
    #[doc = "Godot enumerator name: `DEBUG_FLAG_VIEW_COLLISIONS`"]
    pub const VIEW_COLLISIONS: DebugFlags = DebugFlags {
        ord: 8u64
    };
    #[doc(alias = "DEBUG_FLAG_VIEW_NAVIGATION")]
    #[doc = "Godot enumerator name: `DEBUG_FLAG_VIEW_NAVIGATION`"]
    pub const VIEW_NAVIGATION: DebugFlags = DebugFlags {
        ord: 16u64
    };
    
}
impl std::fmt::Debug for DebugFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::DUMB_CLIENT => "DUMB_CLIENT", Self::REMOTE_DEBUG => "REMOTE_DEBUG", Self::REMOTE_DEBUG_LOCALHOST => "REMOTE_DEBUG_LOCALHOST", Self::VIEW_COLLISIONS => "VIEW_COLLISIONS", Self::VIEW_NAVIGATION => "VIEW_NAVIGATION", _ => {
                f.debug_struct("DebugFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for DebugFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DebugFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DUMB_CLIENT", "DEBUG_FLAG_DUMB_CLIENT", DebugFlags::DUMB_CLIENT), crate::meta::inspect::EnumConstant::new("REMOTE_DEBUG", "DEBUG_FLAG_REMOTE_DEBUG", DebugFlags::REMOTE_DEBUG), crate::meta::inspect::EnumConstant::new("REMOTE_DEBUG_LOCALHOST", "DEBUG_FLAG_REMOTE_DEBUG_LOCALHOST", DebugFlags::REMOTE_DEBUG_LOCALHOST), crate::meta::inspect::EnumConstant::new("VIEW_COLLISIONS", "DEBUG_FLAG_VIEW_COLLISIONS", DebugFlags::VIEW_COLLISIONS), crate::meta::inspect::EnumConstant::new("VIEW_NAVIGATION", "DEBUG_FLAG_VIEW_NAVIGATION", DebugFlags::VIEW_NAVIGATION)]
        }
    }
}
impl std::ops::BitOr for DebugFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for DebugFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for DebugFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for DebugFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DebugFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorExportPlatform;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EditorExportPlatform {
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