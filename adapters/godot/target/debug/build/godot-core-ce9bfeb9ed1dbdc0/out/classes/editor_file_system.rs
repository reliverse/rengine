#![doc = "Sidecar module for class [`EditorFileSystem`][crate::classes::EditorFileSystem].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorFileSystem` enums](https://docs.godotengine.org/en/stable/classes/class_editorfilesystem.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorFileSystem.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`editor_file_system`][crate::classes::editor_file_system]: sidecar module with related enum/flag types\n* [`SignalsOfEditorFileSystem`][crate::classes::editor_file_system::SignalsOfEditorFileSystem]: signal collection\n\n\nSee also [Godot docs for `EditorFileSystem`](https://docs.godotengine.org/en/stable/classes/class_editorfilesystem.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<EditorFileSystem>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorFileSystem {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl EditorFileSystem {
        pub fn get_filesystem(&mut self,) -> Option < Gd < crate::classes::EditorFileSystemDirectory > > {
            type CallRet = Option < Gd < crate::classes::EditorFileSystemDirectory > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileSystem", "get_filesystem", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_scanning(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileSystem", "is_scanning", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scanning_progress(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileSystem", "get_scanning_progress", Some(self.__validated_obj()), args,)
            }
        }
        pub fn scan(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileSystem", "scan", Some(self.__validated_obj()), args,)
            }
        }
        pub fn scan_sources(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileSystem", "scan_sources", Some(self.__validated_obj()), args,)
            }
        }
        pub fn update_file(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileSystem", "update_file", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_filesystem_path(&mut self, path: impl AsArg < GString >,) -> Option < Gd < crate::classes::EditorFileSystemDirectory > > {
            type CallRet = Option < Gd < crate::classes::EditorFileSystemDirectory > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(148usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileSystem", "get_filesystem_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_file_type(&self, path: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(149usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileSystem", "get_file_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reimport_files(&mut self, files: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(files),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileSystem", "reimport_files", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EditorFileSystem {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorFileSystem"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorFileSystem {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for EditorFileSystem {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorFileSystem {
        
    }
    impl std::ops::Deref for EditorFileSystem {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorFileSystem {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorFileSystem__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `EditorFileSystem` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorFileSystem;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`EditorFileSystem`][crate::classes::EditorFileSystem] class."]
    pub struct SignalsOfEditorFileSystem < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfEditorFileSystem < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn filesystem_changed(&mut self) -> SigFilesystemChanged < 'c, C > {
            SigFilesystemChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "filesystem_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn script_classes_updated(&mut self) -> SigScriptClassesUpdated < 'c, C > {
            SigScriptClassesUpdated {
                typed: TypedSignal::extract(&mut self.__internal_obj, "script_classes_updated")
            }
        }
        #[doc = "Signature: `(exist: bool)`"]
        pub fn sources_changed(&mut self) -> SigSourcesChanged < 'c, C > {
            SigSourcesChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "sources_changed")
            }
        }
        #[doc = "Signature: `(resources: PackedStringArray)`"]
        pub fn resources_reimporting(&mut self) -> SigResourcesReimporting < 'c, C > {
            SigResourcesReimporting {
                typed: TypedSignal::extract(&mut self.__internal_obj, "resources_reimporting")
            }
        }
        #[doc = "Signature: `(resources: PackedStringArray)`"]
        pub fn resources_reimported(&mut self) -> SigResourcesReimported < 'c, C > {
            SigResourcesReimported {
                typed: TypedSignal::extract(&mut self.__internal_obj, "resources_reimported")
            }
        }
        #[doc = "Signature: `(resources: PackedStringArray)`"]
        pub fn resources_reload(&mut self) -> SigResourcesReload < 'c, C > {
            SigResourcesReload {
                typed: TypedSignal::extract(&mut self.__internal_obj, "resources_reload")
            }
        }
    }
    type TypedSigFilesystemChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFilesystemChanged < 'c, C: WithSignals > {
        typed: TypedSigFilesystemChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFilesystemChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFilesystemChanged < 'c, C > {
        type Target = TypedSigFilesystemChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFilesystemChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigScriptClassesUpdated < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigScriptClassesUpdated < 'c, C: WithSignals > {
        typed: TypedSigScriptClassesUpdated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigScriptClassesUpdated < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigScriptClassesUpdated < 'c, C > {
        type Target = TypedSigScriptClassesUpdated < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigScriptClassesUpdated < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSourcesChanged < 'c, C > = TypedSignal < 'c, C, (bool,) >;
    pub struct SigSourcesChanged < 'c, C: WithSignals > {
        typed: TypedSigSourcesChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSourcesChanged < 'c, C > {
        pub fn emit(&mut self, exist: bool,) {
            self.typed.emit_tuple((exist,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSourcesChanged < 'c, C > {
        type Target = TypedSigSourcesChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSourcesChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigResourcesReimporting < 'c, C > = TypedSignal < 'c, C, (PackedStringArray,) >;
    pub struct SigResourcesReimporting < 'c, C: WithSignals > {
        typed: TypedSigResourcesReimporting < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigResourcesReimporting < 'c, C > {
        pub fn emit(&mut self, resources: PackedStringArray,) {
            self.typed.emit_tuple((resources,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigResourcesReimporting < 'c, C > {
        type Target = TypedSigResourcesReimporting < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigResourcesReimporting < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigResourcesReimported < 'c, C > = TypedSignal < 'c, C, (PackedStringArray,) >;
    pub struct SigResourcesReimported < 'c, C: WithSignals > {
        typed: TypedSigResourcesReimported < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigResourcesReimported < 'c, C > {
        pub fn emit(&mut self, resources: PackedStringArray,) {
            self.typed.emit_tuple((resources,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigResourcesReimported < 'c, C > {
        type Target = TypedSigResourcesReimported < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigResourcesReimported < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigResourcesReload < 'c, C > = TypedSignal < 'c, C, (PackedStringArray,) >;
    pub struct SigResourcesReload < 'c, C: WithSignals > {
        typed: TypedSigResourcesReload < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigResourcesReload < 'c, C > {
        pub fn emit(&mut self, resources: PackedStringArray,) {
            self.typed.emit_tuple((resources,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigResourcesReload < 'c, C > {
        type Target = TypedSigResourcesReload < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigResourcesReload < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for EditorFileSystem {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfEditorFileSystem < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfEditorFileSystem < 'c, C > {
        type Target = < < EditorFileSystem as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = EditorFileSystem;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfEditorFileSystem < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = EditorFileSystem;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}