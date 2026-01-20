#![doc = "Sidecar module for class [`EditorUndoRedoManager`][crate::classes::EditorUndoRedoManager].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorUndoRedoManager` enums](https://docs.godotengine.org/en/stable/classes/class_editorundoredomanager.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorUndoRedoManager.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`editor_undo_redo_manager`][crate::classes::editor_undo_redo_manager]: sidecar module with related enum/flag types\n* [`SignalsOfEditorUndoRedoManager`][crate::classes::editor_undo_redo_manager::SignalsOfEditorUndoRedoManager]: signal collection\n\n\nSee also [Godot docs for `EditorUndoRedoManager`](https://docs.godotengine.org/en/stable/classes/class_editorundoredomanager.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<EditorUndoRedoManager>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorUndoRedoManager {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl EditorUndoRedoManager {
        pub(crate) fn create_action_full(&mut self, name: CowArg < GString >, merge_mode: crate::classes::undo_redo::MergeMode, custom_context: CowArg < Option < Gd < crate::classes::Object > > >, backward_undo_ops: bool, mark_unsaved: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, crate::classes::undo_redo::MergeMode, CowArg < 'a1, Option < Gd < crate::classes::Object > > >, bool, bool,);
            let args = (name, merge_mode, custom_context, backward_undo_ops, mark_unsaved,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "create_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_action_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_action(&mut self, name: impl AsArg < GString >,) {
            self.create_action_ex(name,) . done()
        }
        #[inline]
        pub fn create_action_ex < 'a > (&'a mut self, name: impl AsArg < GString > + 'a,) -> ExCreateAction < 'a > {
            ExCreateAction::new(self, name,)
        }
        pub(crate) fn commit_action_full(&mut self, execute: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (execute,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "commit_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::commit_action_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn commit_action(&mut self,) {
            self.commit_action_ex() . done()
        }
        #[inline]
        pub fn commit_action_ex < 'a > (&'a mut self,) -> ExCommitAction < 'a > {
            ExCommitAction::new(self,)
        }
        pub fn is_committing_action(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "is_committing_action", Some(self.__validated_obj()), args,)
            }
        }
        pub fn force_fixed_history(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "force_fixed_history", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn add_do_method(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, method: impl AsArg < StringName >, varargs: &[Variant]) {
            Self::try_add_do_method(self, object, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_add_do_method(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < (), crate::meta::error::CallError > {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, CowArg < 'a1, StringName >,);
            let args = (object.into_arg(), method.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(409usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "EditorUndoRedoManager", "add_do_method", Some(self.__validated_obj()), args, varargs)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn add_undo_method(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, method: impl AsArg < StringName >, varargs: &[Variant]) {
            Self::try_add_undo_method(self, object, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_add_undo_method(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < (), crate::meta::error::CallError > {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, CowArg < 'a1, StringName >,);
            let args = (object.into_arg(), method.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(410usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "EditorUndoRedoManager", "add_undo_method", Some(self.__validated_obj()), args, varargs)
            }
        }
        pub fn add_do_property(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, property: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, CowArg < 'a1, StringName >, RefArg < 'a2, Variant >,);
            let args = (object.into_arg(), property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "add_do_property", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_undo_property(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, property: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, CowArg < 'a1, StringName >, RefArg < 'a2, Variant >,);
            let args = (object.into_arg(), property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "add_undo_property", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_do_reference(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >,);
            let args = (object.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "add_do_reference", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_undo_reference(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >,);
            let args = (object.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "add_undo_reference", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_object_history_id(&self, object: impl AsArg < Option < Gd < crate::classes::Object >> >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >,);
            let args = (object.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "get_object_history_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_history_undo_redo(&self, id: i32,) -> Option < Gd < crate::classes::UndoRedo > > {
            type CallRet = Option < Gd < crate::classes::UndoRedo > >;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "get_history_undo_redo", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn clear_history_full(&mut self, id: i32, increase_version: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (id, increase_version,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorUndoRedoManager", "clear_history", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::clear_history_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn clear_history(&mut self,) {
            self.clear_history_ex() . done()
        }
        #[inline]
        pub fn clear_history_ex < 'a > (&'a mut self,) -> ExClearHistory < 'a > {
            ExClearHistory::new(self,)
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
    impl crate::obj::GodotClass for EditorUndoRedoManager {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorUndoRedoManager"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorUndoRedoManager {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorUndoRedoManager {
        
    }
    impl std::ops::Deref for EditorUndoRedoManager {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorUndoRedoManager {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorUndoRedoManager__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `EditorUndoRedoManager` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`EditorUndoRedoManager::create_action_ex`][super::EditorUndoRedoManager::create_action_ex]."]
#[must_use]
pub struct ExCreateAction < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorUndoRedoManager, name: CowArg < 'a, GString >, merge_mode: crate::classes::undo_redo::MergeMode, custom_context: CowArg < 'a, Option < Gd < crate::classes::Object > > >, backward_undo_ops: bool, mark_unsaved: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateAction < 'a > {
    fn new(surround_object: &'a mut re_export::EditorUndoRedoManager, name: impl AsArg < GString > + 'a,) -> Self {
        let merge_mode = crate::obj::EngineEnum::from_ord(0);
        let custom_context = Gd::null_arg();
        let backward_undo_ops = false;
        let mark_unsaved = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), merge_mode: merge_mode, custom_context: custom_context.into_arg(), backward_undo_ops: backward_undo_ops, mark_unsaved: mark_unsaved,
        }
    }
    #[inline]
    pub fn merge_mode(self, merge_mode: crate::classes::undo_redo::MergeMode) -> Self {
        Self {
            merge_mode: merge_mode, .. self
        }
    }
    #[inline]
    pub fn custom_context(self, custom_context: impl AsArg < Option < Gd < crate::classes::Object >> > + 'a) -> Self {
        Self {
            custom_context: custom_context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn backward_undo_ops(self, backward_undo_ops: bool) -> Self {
        Self {
            backward_undo_ops: backward_undo_ops, .. self
        }
    }
    #[inline]
    pub fn mark_unsaved(self, mark_unsaved: bool) -> Self {
        Self {
            mark_unsaved: mark_unsaved, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, merge_mode, custom_context, backward_undo_ops, mark_unsaved,
        }
        = self;
        re_export::EditorUndoRedoManager::create_action_full(surround_object, name, merge_mode, custom_context, backward_undo_ops, mark_unsaved,)
    }
}
#[doc = "Default-param extender for [`EditorUndoRedoManager::commit_action_ex`][super::EditorUndoRedoManager::commit_action_ex]."]
#[must_use]
pub struct ExCommitAction < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorUndoRedoManager, execute: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCommitAction < 'a > {
    fn new(surround_object: &'a mut re_export::EditorUndoRedoManager,) -> Self {
        let execute = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, execute: execute,
        }
    }
    #[inline]
    pub fn execute(self, execute: bool) -> Self {
        Self {
            execute: execute, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, execute,
        }
        = self;
        re_export::EditorUndoRedoManager::commit_action_full(surround_object, execute,)
    }
}
#[doc = "Default-param extender for [`EditorUndoRedoManager::clear_history_ex`][super::EditorUndoRedoManager::clear_history_ex]."]
#[must_use]
pub struct ExClearHistory < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorUndoRedoManager, id: i32, increase_version: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClearHistory < 'a > {
    fn new(surround_object: &'a mut re_export::EditorUndoRedoManager,) -> Self {
        let id = - 99i32;
        let increase_version = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id, increase_version: increase_version,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn increase_version(self, increase_version: bool) -> Self {
        Self {
            increase_version: increase_version, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, id, increase_version,
        }
        = self;
        re_export::EditorUndoRedoManager::clear_history_full(surround_object, id, increase_version,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpecialHistory {
    ord: i32
}
impl SpecialHistory {
    pub const GLOBAL_HISTORY: SpecialHistory = SpecialHistory {
        ord: 0i32
    };
    pub const REMOTE_HISTORY: SpecialHistory = SpecialHistory {
        ord: - 9i32
    };
    pub const INVALID_HISTORY: SpecialHistory = SpecialHistory {
        ord: - 99i32
    };
    
}
impl std::fmt::Debug for SpecialHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SpecialHistory") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SpecialHistory {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ - 99i32 | ord @ - 9i32 | ord @ 0i32 => Some(Self {
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
            Self::GLOBAL_HISTORY => "GLOBAL_HISTORY", Self::REMOTE_HISTORY => "REMOTE_HISTORY", Self::INVALID_HISTORY => "INVALID_HISTORY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SpecialHistory::GLOBAL_HISTORY, SpecialHistory::REMOTE_HISTORY, SpecialHistory::INVALID_HISTORY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SpecialHistory >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("GLOBAL_HISTORY", "GLOBAL_HISTORY", SpecialHistory::GLOBAL_HISTORY), crate::meta::inspect::EnumConstant::new("REMOTE_HISTORY", "REMOTE_HISTORY", SpecialHistory::REMOTE_HISTORY), crate::meta::inspect::EnumConstant::new("INVALID_HISTORY", "INVALID_HISTORY", SpecialHistory::INVALID_HISTORY)]
        }
    }
}
impl crate::meta::GodotConvert for SpecialHistory {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SpecialHistory {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SpecialHistory {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorUndoRedoManager;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`EditorUndoRedoManager`][crate::classes::EditorUndoRedoManager] class."]
    pub struct SignalsOfEditorUndoRedoManager < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfEditorUndoRedoManager < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn history_changed(&mut self) -> SigHistoryChanged < 'c, C > {
            SigHistoryChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "history_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn version_changed(&mut self) -> SigVersionChanged < 'c, C > {
            SigVersionChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "version_changed")
            }
        }
    }
    type TypedSigHistoryChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigHistoryChanged < 'c, C: WithSignals > {
        typed: TypedSigHistoryChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigHistoryChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigHistoryChanged < 'c, C > {
        type Target = TypedSigHistoryChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigHistoryChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigVersionChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigVersionChanged < 'c, C: WithSignals > {
        typed: TypedSigVersionChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigVersionChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigVersionChanged < 'c, C > {
        type Target = TypedSigVersionChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigVersionChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for EditorUndoRedoManager {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfEditorUndoRedoManager < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfEditorUndoRedoManager < 'c, C > {
        type Target = < < EditorUndoRedoManager as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = EditorUndoRedoManager;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfEditorUndoRedoManager < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = EditorUndoRedoManager;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}