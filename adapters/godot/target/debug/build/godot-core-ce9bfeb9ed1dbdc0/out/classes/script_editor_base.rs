#![doc = "Sidecar module for class [`ScriptEditorBase`][crate::classes::ScriptEditorBase].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ScriptEditorBase` enums](https://docs.godotengine.org/en/stable/classes/class_scripteditorbase.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ScriptEditorBase.`\n\nInherits [`VBoxContainer`][crate::classes::VBoxContainer].\n\nRelated symbols:\n\n* [`script_editor_base`][crate::classes::script_editor_base]: sidecar module with related enum/flag types\n* [`SignalsOfScriptEditorBase`][crate::classes::script_editor_base::SignalsOfScriptEditorBase]: signal collection\n\n\nSee also [Godot docs for `ScriptEditorBase`](https://docs.godotengine.org/en/stable/classes/class_scripteditorbase.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<ScriptEditorBase>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ScriptEditorBase {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl ScriptEditorBase {
        pub fn get_base_editor(&self,) -> Option < Gd < crate::classes::Control > > {
            type CallRet = Option < Gd < crate::classes::Control > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(455usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptEditorBase", "get_base_editor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_syntax_highlighter(&mut self, highlighter: impl AsArg < Option < Gd < crate::classes::EditorSyntaxHighlighter >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorSyntaxHighlighter > > >,);
            let args = (highlighter.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(456usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScriptEditorBase", "add_syntax_highlighter", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ScriptEditorBase {
        type Base = crate::classes::VBoxContainer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ScriptEditorBase"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for ScriptEditorBase {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VBoxContainer > for ScriptEditorBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::BoxContainer > for ScriptEditorBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Container > for ScriptEditorBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for ScriptEditorBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for ScriptEditorBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for ScriptEditorBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ScriptEditorBase {
        
    }
    impl std::ops::Deref for ScriptEditorBase {
        type Target = crate::classes::VBoxContainer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ScriptEditorBase {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ScriptEditorBase__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `ScriptEditorBase` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ScriptEditorBase;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`ScriptEditorBase`][crate::classes::ScriptEditorBase] class."]
    pub struct SignalsOfScriptEditorBase < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfScriptEditorBase < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn name_changed(&mut self) -> SigNameChanged < 'c, C > {
            SigNameChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "name_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn edited_script_changed(&mut self) -> SigEditedScriptChanged < 'c, C > {
            SigEditedScriptChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "edited_script_changed")
            }
        }
        #[doc = "Signature: `(topic: GString)`"]
        pub fn request_help(&mut self) -> SigRequestHelp < 'c, C > {
            SigRequestHelp {
                typed: TypedSignal::extract(&mut self.__internal_obj, "request_help")
            }
        }
        #[doc = "Signature: `(script: Gd<Object>, line: i64)`"]
        pub fn request_open_script_at_line(&mut self) -> SigRequestOpenScriptAtLine < 'c, C > {
            SigRequestOpenScriptAtLine {
                typed: TypedSignal::extract(&mut self.__internal_obj, "request_open_script_at_line")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn request_save_history(&mut self) -> SigRequestSaveHistory < 'c, C > {
            SigRequestSaveHistory {
                typed: TypedSignal::extract(&mut self.__internal_obj, "request_save_history")
            }
        }
        #[doc = "Signature: `(state: VarDictionary)`"]
        pub fn request_save_previous_state(&mut self) -> SigRequestSavePreviousState < 'c, C > {
            SigRequestSavePreviousState {
                typed: TypedSignal::extract(&mut self.__internal_obj, "request_save_previous_state")
            }
        }
        #[doc = "Signature: `(what: GString)`"]
        pub fn go_to_help(&mut self) -> SigGoToHelp < 'c, C > {
            SigGoToHelp {
                typed: TypedSignal::extract(&mut self.__internal_obj, "go_to_help")
            }
        }
        #[doc = "Signature: `(text: GString)`"]
        pub fn search_in_files_requested(&mut self) -> SigSearchInFilesRequested < 'c, C > {
            SigSearchInFilesRequested {
                typed: TypedSignal::extract(&mut self.__internal_obj, "search_in_files_requested")
            }
        }
        #[doc = "Signature: `(text: GString)`"]
        pub fn replace_in_files_requested(&mut self) -> SigReplaceInFilesRequested < 'c, C > {
            SigReplaceInFilesRequested {
                typed: TypedSignal::extract(&mut self.__internal_obj, "replace_in_files_requested")
            }
        }
        #[doc = "Signature: `(script: Gd<Object>, method: GString)`"]
        pub fn go_to_method(&mut self) -> SigGoToMethod < 'c, C > {
            SigGoToMethod {
                typed: TypedSignal::extract(&mut self.__internal_obj, "go_to_method")
            }
        }
    }
    type TypedSigNameChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigNameChanged < 'c, C: WithSignals > {
        typed: TypedSigNameChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigNameChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigNameChanged < 'c, C > {
        type Target = TypedSigNameChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigNameChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigEditedScriptChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigEditedScriptChanged < 'c, C: WithSignals > {
        typed: TypedSigEditedScriptChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigEditedScriptChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigEditedScriptChanged < 'c, C > {
        type Target = TypedSigEditedScriptChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigEditedScriptChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigRequestHelp < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigRequestHelp < 'c, C: WithSignals > {
        typed: TypedSigRequestHelp < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigRequestHelp < 'c, C > {
        pub fn emit(&mut self, topic: GString,) {
            self.typed.emit_tuple((topic,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigRequestHelp < 'c, C > {
        type Target = TypedSigRequestHelp < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigRequestHelp < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigRequestOpenScriptAtLine < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Object >, i64,) >;
    pub struct SigRequestOpenScriptAtLine < 'c, C: WithSignals > {
        typed: TypedSigRequestOpenScriptAtLine < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigRequestOpenScriptAtLine < 'c, C > {
        pub fn emit(&mut self, script: Gd < crate::classes::Object >, line: i64,) {
            self.typed.emit_tuple((script, line,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigRequestOpenScriptAtLine < 'c, C > {
        type Target = TypedSigRequestOpenScriptAtLine < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigRequestOpenScriptAtLine < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigRequestSaveHistory < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigRequestSaveHistory < 'c, C: WithSignals > {
        typed: TypedSigRequestSaveHistory < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigRequestSaveHistory < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigRequestSaveHistory < 'c, C > {
        type Target = TypedSigRequestSaveHistory < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigRequestSaveHistory < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigRequestSavePreviousState < 'c, C > = TypedSignal < 'c, C, (VarDictionary,) >;
    pub struct SigRequestSavePreviousState < 'c, C: WithSignals > {
        typed: TypedSigRequestSavePreviousState < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigRequestSavePreviousState < 'c, C > {
        pub fn emit(&mut self, state: VarDictionary,) {
            self.typed.emit_tuple((state,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigRequestSavePreviousState < 'c, C > {
        type Target = TypedSigRequestSavePreviousState < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigRequestSavePreviousState < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigGoToHelp < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigGoToHelp < 'c, C: WithSignals > {
        typed: TypedSigGoToHelp < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigGoToHelp < 'c, C > {
        pub fn emit(&mut self, what: GString,) {
            self.typed.emit_tuple((what,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigGoToHelp < 'c, C > {
        type Target = TypedSigGoToHelp < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigGoToHelp < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSearchInFilesRequested < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigSearchInFilesRequested < 'c, C: WithSignals > {
        typed: TypedSigSearchInFilesRequested < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSearchInFilesRequested < 'c, C > {
        pub fn emit(&mut self, text: GString,) {
            self.typed.emit_tuple((text,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSearchInFilesRequested < 'c, C > {
        type Target = TypedSigSearchInFilesRequested < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSearchInFilesRequested < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigReplaceInFilesRequested < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigReplaceInFilesRequested < 'c, C: WithSignals > {
        typed: TypedSigReplaceInFilesRequested < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigReplaceInFilesRequested < 'c, C > {
        pub fn emit(&mut self, text: GString,) {
            self.typed.emit_tuple((text,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigReplaceInFilesRequested < 'c, C > {
        type Target = TypedSigReplaceInFilesRequested < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigReplaceInFilesRequested < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigGoToMethod < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Object >, GString,) >;
    pub struct SigGoToMethod < 'c, C: WithSignals > {
        typed: TypedSigGoToMethod < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigGoToMethod < 'c, C > {
        pub fn emit(&mut self, script: Gd < crate::classes::Object >, method: GString,) {
            self.typed.emit_tuple((script, method,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigGoToMethod < 'c, C > {
        type Target = TypedSigGoToMethod < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigGoToMethod < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for ScriptEditorBase {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfScriptEditorBase < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfScriptEditorBase < 'c, C > {
        type Target = < < ScriptEditorBase as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = ScriptEditorBase;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfScriptEditorBase < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = ScriptEditorBase;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}