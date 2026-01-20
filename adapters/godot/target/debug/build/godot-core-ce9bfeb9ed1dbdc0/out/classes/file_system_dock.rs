#![doc = "Sidecar module for class [`FileSystemDock`][crate::classes::FileSystemDock].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FileSystemDock` enums](https://docs.godotengine.org/en/stable/classes/class_filesystemdock.html#enumerations).\n\n"]
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
    #[doc = "Godot class `FileSystemDock.`\n\nInherits [`VBoxContainer`][crate::classes::VBoxContainer].\n\nRelated symbols:\n\n* [`file_system_dock`][crate::classes::file_system_dock]: sidecar module with related enum/flag types\n* [`SignalsOfFileSystemDock`][crate::classes::file_system_dock::SignalsOfFileSystemDock]: signal collection\n\n\nSee also [Godot docs for `FileSystemDock`](https://docs.godotengine.org/en/stable/classes/class_filesystemdock.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<FileSystemDock>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FileSystemDock {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl FileSystemDock {
        pub fn navigate_to_path(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(426usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileSystemDock", "navigate_to_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_resource_tooltip_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorResourceTooltipPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorResourceTooltipPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(427usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileSystemDock", "add_resource_tooltip_plugin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_resource_tooltip_plugin(&mut self, plugin: impl AsArg < Option < Gd < crate::classes::EditorResourceTooltipPlugin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorResourceTooltipPlugin > > >,);
            let args = (plugin.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(428usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FileSystemDock", "remove_resource_tooltip_plugin", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for FileSystemDock {
        type Base = crate::classes::VBoxContainer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("FileSystemDock"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for FileSystemDock {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VBoxContainer > for FileSystemDock {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::BoxContainer > for FileSystemDock {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Container > for FileSystemDock {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for FileSystemDock {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for FileSystemDock {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for FileSystemDock {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for FileSystemDock {
        
    }
    impl std::ops::Deref for FileSystemDock {
        type Target = crate::classes::VBoxContainer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FileSystemDock {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_FileSystemDock__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `FileSystemDock` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::FileSystemDock;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`FileSystemDock`][crate::classes::FileSystemDock] class."]
    pub struct SignalsOfFileSystemDock < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfFileSystemDock < 'c, C > {
        #[doc = "Signature: `(file: GString)`"]
        pub fn inherit(&mut self) -> SigInherit < 'c, C > {
            SigInherit {
                typed: TypedSignal::extract(&mut self.__internal_obj, "inherit")
            }
        }
        #[doc = "Signature: `(files: PackedStringArray)`"]
        pub fn instantiate(&mut self) -> SigInstantiate < 'c, C > {
            SigInstantiate {
                typed: TypedSignal::extract(&mut self.__internal_obj, "instantiate")
            }
        }
        #[doc = "Signature: `(resource: Gd<Resource>)`"]
        pub fn resource_removed(&mut self) -> SigResourceRemoved < 'c, C > {
            SigResourceRemoved {
                typed: TypedSignal::extract(&mut self.__internal_obj, "resource_removed")
            }
        }
        #[doc = "Signature: `(file: GString)`"]
        pub fn file_removed(&mut self) -> SigFileRemoved < 'c, C > {
            SigFileRemoved {
                typed: TypedSignal::extract(&mut self.__internal_obj, "file_removed")
            }
        }
        #[doc = "Signature: `(folder: GString)`"]
        pub fn folder_removed(&mut self) -> SigFolderRemoved < 'c, C > {
            SigFolderRemoved {
                typed: TypedSignal::extract(&mut self.__internal_obj, "folder_removed")
            }
        }
        #[doc = "Signature: `(old_file: GString, new_file: GString)`"]
        pub fn files_moved(&mut self) -> SigFilesMoved < 'c, C > {
            SigFilesMoved {
                typed: TypedSignal::extract(&mut self.__internal_obj, "files_moved")
            }
        }
        #[doc = "Signature: `(old_folder: GString, new_folder: GString)`"]
        pub fn folder_moved(&mut self) -> SigFolderMoved < 'c, C > {
            SigFolderMoved {
                typed: TypedSignal::extract(&mut self.__internal_obj, "folder_moved")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn folder_color_changed(&mut self) -> SigFolderColorChanged < 'c, C > {
            SigFolderColorChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "folder_color_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn display_mode_changed(&mut self) -> SigDisplayModeChanged < 'c, C > {
            SigDisplayModeChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "display_mode_changed")
            }
        }
    }
    type TypedSigInherit < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigInherit < 'c, C: WithSignals > {
        typed: TypedSigInherit < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigInherit < 'c, C > {
        pub fn emit(&mut self, file: GString,) {
            self.typed.emit_tuple((file,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigInherit < 'c, C > {
        type Target = TypedSigInherit < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigInherit < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigInstantiate < 'c, C > = TypedSignal < 'c, C, (PackedStringArray,) >;
    pub struct SigInstantiate < 'c, C: WithSignals > {
        typed: TypedSigInstantiate < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigInstantiate < 'c, C > {
        pub fn emit(&mut self, files: PackedStringArray,) {
            self.typed.emit_tuple((files,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigInstantiate < 'c, C > {
        type Target = TypedSigInstantiate < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigInstantiate < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigResourceRemoved < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Resource >,) >;
    pub struct SigResourceRemoved < 'c, C: WithSignals > {
        typed: TypedSigResourceRemoved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigResourceRemoved < 'c, C > {
        pub fn emit(&mut self, resource: Gd < crate::classes::Resource >,) {
            self.typed.emit_tuple((resource,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigResourceRemoved < 'c, C > {
        type Target = TypedSigResourceRemoved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigResourceRemoved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFileRemoved < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigFileRemoved < 'c, C: WithSignals > {
        typed: TypedSigFileRemoved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFileRemoved < 'c, C > {
        pub fn emit(&mut self, file: GString,) {
            self.typed.emit_tuple((file,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFileRemoved < 'c, C > {
        type Target = TypedSigFileRemoved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFileRemoved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFolderRemoved < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigFolderRemoved < 'c, C: WithSignals > {
        typed: TypedSigFolderRemoved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFolderRemoved < 'c, C > {
        pub fn emit(&mut self, folder: GString,) {
            self.typed.emit_tuple((folder,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFolderRemoved < 'c, C > {
        type Target = TypedSigFolderRemoved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFolderRemoved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFilesMoved < 'c, C > = TypedSignal < 'c, C, (GString, GString,) >;
    pub struct SigFilesMoved < 'c, C: WithSignals > {
        typed: TypedSigFilesMoved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFilesMoved < 'c, C > {
        pub fn emit(&mut self, old_file: GString, new_file: GString,) {
            self.typed.emit_tuple((old_file, new_file,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFilesMoved < 'c, C > {
        type Target = TypedSigFilesMoved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFilesMoved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFolderMoved < 'c, C > = TypedSignal < 'c, C, (GString, GString,) >;
    pub struct SigFolderMoved < 'c, C: WithSignals > {
        typed: TypedSigFolderMoved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFolderMoved < 'c, C > {
        pub fn emit(&mut self, old_folder: GString, new_folder: GString,) {
            self.typed.emit_tuple((old_folder, new_folder,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFolderMoved < 'c, C > {
        type Target = TypedSigFolderMoved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFolderMoved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFolderColorChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFolderColorChanged < 'c, C: WithSignals > {
        typed: TypedSigFolderColorChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFolderColorChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFolderColorChanged < 'c, C > {
        type Target = TypedSigFolderColorChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFolderColorChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigDisplayModeChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigDisplayModeChanged < 'c, C: WithSignals > {
        typed: TypedSigDisplayModeChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDisplayModeChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDisplayModeChanged < 'c, C > {
        type Target = TypedSigDisplayModeChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDisplayModeChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for FileSystemDock {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfFileSystemDock < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfFileSystemDock < 'c, C > {
        type Target = < < FileSystemDock as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = FileSystemDock;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfFileSystemDock < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = FileSystemDock;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}