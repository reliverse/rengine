#![doc = "Sidecar module for class [`EditorExportPreset`][crate::classes::EditorExportPreset].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorExportPreset` enums](https://docs.godotengine.org/en/stable/classes/class_editorexportpreset.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorExportPreset.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`editor_export_preset`][crate::classes::editor_export_preset]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `EditorExportPreset`](https://docs.godotengine.org/en/stable/classes/class_editorexportpreset.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<EditorExportPreset>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorExportPreset {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl EditorExportPreset {
        pub fn has(&self, property: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (property.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(68usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "has", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_files_to_export(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(69usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_files_to_export", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_customized_files(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(70usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_customized_files", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_customized_files_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(71usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_customized_files_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_export_file(&mut self, path: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(72usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "has_export_file", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_file_export_mode_full(&self, path: CowArg < GString >, default: crate::classes::editor_export_preset::FileExportMode,) -> crate::classes::editor_export_preset::FileExportMode {
            type CallRet = crate::classes::editor_export_preset::FileExportMode;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::editor_export_preset::FileExportMode,);
            let args = (path, default,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(73usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_file_export_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_file_export_mode_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_file_export_mode(&self, path: impl AsArg < GString >,) -> crate::classes::editor_export_preset::FileExportMode {
            self.get_file_export_mode_ex(path,) . done()
        }
        #[inline]
        pub fn get_file_export_mode_ex < 'a > (&'a self, path: impl AsArg < GString > + 'a,) -> ExGetFileExportMode < 'a > {
            ExGetFileExportMode::new(self, path,)
        }
        pub fn get_project_setting(&mut self, name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(74usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_project_setting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_preset_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(75usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_preset_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_runnable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(76usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "is_runnable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn are_advanced_options_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(77usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "are_advanced_options_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_dedicated_server(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(78usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "is_dedicated_server", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_export_filter(&self,) -> crate::classes::editor_export_preset::ExportFilter {
            type CallRet = crate::classes::editor_export_preset::ExportFilter;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(79usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_export_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_include_filter(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(80usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_include_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_exclude_filter(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(81usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_exclude_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_features(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(82usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_custom_features", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_patches(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(83usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_patches", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_export_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(84usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_export_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_encryption_in_filter(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(85usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_encryption_in_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_encryption_ex_filter(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(86usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_encryption_ex_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_encrypt_pck(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(87usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_encrypt_pck", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_encrypt_directory(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(88usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_encrypt_directory", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_encryption_key(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(89usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_encryption_key", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_script_export_mode(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(90usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_script_export_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_or_env(&self, name: impl AsArg < StringName >, env_var: impl AsArg < GString >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, GString >,);
            let args = (name.into_arg(), env_var.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(91usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_or_env", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_version(&self, name: impl AsArg < StringName >, windows_version: bool,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (name.into_arg(), windows_version,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(92usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorExportPreset", "get_version", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EditorExportPreset {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorExportPreset"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorExportPreset {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorExportPreset {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorExportPreset {
        
    }
    impl std::ops::Deref for EditorExportPreset {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorExportPreset {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorExportPreset__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `EditorExportPreset` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`EditorExportPreset::get_file_export_mode_ex`][super::EditorExportPreset::get_file_export_mode_ex]."]
#[must_use]
pub struct ExGetFileExportMode < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::EditorExportPreset, path: CowArg < 'a, GString >, default: crate::classes::editor_export_preset::FileExportMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetFileExportMode < 'a > {
    fn new(surround_object: &'a re_export::EditorExportPreset, path: impl AsArg < GString > + 'a,) -> Self {
        let default = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), default: default,
        }
    }
    #[inline]
    pub fn default(self, default: crate::classes::editor_export_preset::FileExportMode) -> Self {
        Self {
            default: default, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::classes::editor_export_preset::FileExportMode {
        let Self {
            _phantom, surround_object, path, default,
        }
        = self;
        re_export::EditorExportPreset::get_file_export_mode_full(surround_object, path, default,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ExportFilter {
    ord: i32
}
impl ExportFilter {
    pub const EXPORT_ALL_RESOURCES: ExportFilter = ExportFilter {
        ord: 0i32
    };
    pub const EXPORT_SELECTED_SCENES: ExportFilter = ExportFilter {
        ord: 1i32
    };
    pub const EXPORT_SELECTED_RESOURCES: ExportFilter = ExportFilter {
        ord: 2i32
    };
    pub const EXCLUDE_SELECTED_RESOURCES: ExportFilter = ExportFilter {
        ord: 3i32
    };
    pub const EXPORT_CUSTOMIZED: ExportFilter = ExportFilter {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ExportFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ExportFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ExportFilter {
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
            Self::EXPORT_ALL_RESOURCES => "EXPORT_ALL_RESOURCES", Self::EXPORT_SELECTED_SCENES => "EXPORT_SELECTED_SCENES", Self::EXPORT_SELECTED_RESOURCES => "EXPORT_SELECTED_RESOURCES", Self::EXCLUDE_SELECTED_RESOURCES => "EXCLUDE_SELECTED_RESOURCES", Self::EXPORT_CUSTOMIZED => "EXPORT_CUSTOMIZED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ExportFilter::EXPORT_ALL_RESOURCES, ExportFilter::EXPORT_SELECTED_SCENES, ExportFilter::EXPORT_SELECTED_RESOURCES, ExportFilter::EXCLUDE_SELECTED_RESOURCES, ExportFilter::EXPORT_CUSTOMIZED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ExportFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("EXPORT_ALL_RESOURCES", "EXPORT_ALL_RESOURCES", ExportFilter::EXPORT_ALL_RESOURCES), crate::meta::inspect::EnumConstant::new("EXPORT_SELECTED_SCENES", "EXPORT_SELECTED_SCENES", ExportFilter::EXPORT_SELECTED_SCENES), crate::meta::inspect::EnumConstant::new("EXPORT_SELECTED_RESOURCES", "EXPORT_SELECTED_RESOURCES", ExportFilter::EXPORT_SELECTED_RESOURCES), crate::meta::inspect::EnumConstant::new("EXCLUDE_SELECTED_RESOURCES", "EXCLUDE_SELECTED_RESOURCES", ExportFilter::EXCLUDE_SELECTED_RESOURCES), crate::meta::inspect::EnumConstant::new("EXPORT_CUSTOMIZED", "EXPORT_CUSTOMIZED", ExportFilter::EXPORT_CUSTOMIZED)]
        }
    }
}
impl crate::meta::GodotConvert for ExportFilter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ExportFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ExportFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FileExportMode {
    ord: i32
}
impl FileExportMode {
    #[doc(alias = "MODE_FILE_NOT_CUSTOMIZED")]
    #[doc = "Godot enumerator name: `MODE_FILE_NOT_CUSTOMIZED`"]
    pub const NOT_CUSTOMIZED: FileExportMode = FileExportMode {
        ord: 0i32
    };
    #[doc(alias = "MODE_FILE_STRIP")]
    #[doc = "Godot enumerator name: `MODE_FILE_STRIP`"]
    pub const STRIP: FileExportMode = FileExportMode {
        ord: 1i32
    };
    #[doc(alias = "MODE_FILE_KEEP")]
    #[doc = "Godot enumerator name: `MODE_FILE_KEEP`"]
    pub const KEEP: FileExportMode = FileExportMode {
        ord: 2i32
    };
    #[doc(alias = "MODE_FILE_REMOVE")]
    #[doc = "Godot enumerator name: `MODE_FILE_REMOVE`"]
    pub const REMOVE: FileExportMode = FileExportMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for FileExportMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FileExportMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FileExportMode {
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
            Self::NOT_CUSTOMIZED => "NOT_CUSTOMIZED", Self::STRIP => "STRIP", Self::KEEP => "KEEP", Self::REMOVE => "REMOVE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FileExportMode::NOT_CUSTOMIZED, FileExportMode::STRIP, FileExportMode::KEEP, FileExportMode::REMOVE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FileExportMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NOT_CUSTOMIZED", "MODE_FILE_NOT_CUSTOMIZED", FileExportMode::NOT_CUSTOMIZED), crate::meta::inspect::EnumConstant::new("STRIP", "MODE_FILE_STRIP", FileExportMode::STRIP), crate::meta::inspect::EnumConstant::new("KEEP", "MODE_FILE_KEEP", FileExportMode::KEEP), crate::meta::inspect::EnumConstant::new("REMOVE", "MODE_FILE_REMOVE", FileExportMode::REMOVE)]
        }
    }
}
impl crate::meta::GodotConvert for FileExportMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FileExportMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FileExportMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ScriptExportMode {
    ord: i32
}
impl ScriptExportMode {
    #[doc(alias = "MODE_SCRIPT_TEXT")]
    #[doc = "Godot enumerator name: `MODE_SCRIPT_TEXT`"]
    pub const TEXT: ScriptExportMode = ScriptExportMode {
        ord: 0i32
    };
    #[doc(alias = "MODE_SCRIPT_BINARY_TOKENS")]
    #[doc = "Godot enumerator name: `MODE_SCRIPT_BINARY_TOKENS`"]
    pub const BINARY_TOKENS: ScriptExportMode = ScriptExportMode {
        ord: 1i32
    };
    #[doc(alias = "MODE_SCRIPT_BINARY_TOKENS_COMPRESSED")]
    #[doc = "Godot enumerator name: `MODE_SCRIPT_BINARY_TOKENS_COMPRESSED`"]
    pub const BINARY_TOKENS_COMPRESSED: ScriptExportMode = ScriptExportMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ScriptExportMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ScriptExportMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ScriptExportMode {
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
            Self::TEXT => "TEXT", Self::BINARY_TOKENS => "BINARY_TOKENS", Self::BINARY_TOKENS_COMPRESSED => "BINARY_TOKENS_COMPRESSED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ScriptExportMode::TEXT, ScriptExportMode::BINARY_TOKENS, ScriptExportMode::BINARY_TOKENS_COMPRESSED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ScriptExportMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TEXT", "MODE_SCRIPT_TEXT", ScriptExportMode::TEXT), crate::meta::inspect::EnumConstant::new("BINARY_TOKENS", "MODE_SCRIPT_BINARY_TOKENS", ScriptExportMode::BINARY_TOKENS), crate::meta::inspect::EnumConstant::new("BINARY_TOKENS_COMPRESSED", "MODE_SCRIPT_BINARY_TOKENS_COMPRESSED", ScriptExportMode::BINARY_TOKENS_COMPRESSED)]
        }
    }
}
impl crate::meta::GodotConvert for ScriptExportMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ScriptExportMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ScriptExportMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorExportPreset;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EditorExportPreset {
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