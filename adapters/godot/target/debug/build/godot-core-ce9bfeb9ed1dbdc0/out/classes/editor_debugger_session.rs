#![doc = "Sidecar module for class [`EditorDebuggerSession`][crate::classes::EditorDebuggerSession].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorDebuggerSession` enums](https://docs.godotengine.org/en/stable/classes/class_editordebuggersession.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorDebuggerSession.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`editor_debugger_session`][crate::classes::editor_debugger_session]: sidecar module with related enum/flag types\n* [`SignalsOfEditorDebuggerSession`][crate::classes::editor_debugger_session::SignalsOfEditorDebuggerSession]: signal collection\n\n\nSee also [Godot docs for `EditorDebuggerSession`](https://docs.godotengine.org/en/stable/classes/class_editordebuggersession.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<EditorDebuggerSession>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorDebuggerSession {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl EditorDebuggerSession {
        pub(crate) fn send_message_full(&mut self, message: CowArg < GString >, data: RefArg < VarArray >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, VarArray >,);
            let args = (message, data,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(8usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorDebuggerSession", "send_message", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::send_message_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn send_message(&mut self, message: impl AsArg < GString >,) {
            self.send_message_ex(message,) . done()
        }
        #[inline]
        pub fn send_message_ex < 'a > (&'a mut self, message: impl AsArg < GString > + 'a,) -> ExSendMessage < 'a > {
            ExSendMessage::new(self, message,)
        }
        pub(crate) fn toggle_profiler_full(&mut self, profiler: CowArg < GString >, enable: bool, data: RefArg < VarArray >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, bool, RefArg < 'a1, VarArray >,);
            let args = (profiler, enable, data,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(9usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorDebuggerSession", "toggle_profiler", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::toggle_profiler_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn toggle_profiler(&mut self, profiler: impl AsArg < GString >, enable: bool,) {
            self.toggle_profiler_ex(profiler, enable,) . done()
        }
        #[inline]
        pub fn toggle_profiler_ex < 'a > (&'a mut self, profiler: impl AsArg < GString > + 'a, enable: bool,) -> ExToggleProfiler < 'a > {
            ExToggleProfiler::new(self, profiler, enable,)
        }
        pub fn is_breaked(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(10usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorDebuggerSession", "is_breaked", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_debuggable(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(11usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorDebuggerSession", "is_debuggable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_active(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(12usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorDebuggerSession", "is_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_session_tab(&mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (control.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(13usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorDebuggerSession", "add_session_tab", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_session_tab(&mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (control.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(14usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorDebuggerSession", "remove_session_tab", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_breakpoint(&mut self, path: impl AsArg < GString >, line: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, bool,);
            let args = (path.into_arg(), line, enabled,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(15usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorDebuggerSession", "set_breakpoint", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EditorDebuggerSession {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorDebuggerSession"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorDebuggerSession {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorDebuggerSession {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorDebuggerSession {
        
    }
    impl std::ops::Deref for EditorDebuggerSession {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorDebuggerSession {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorDebuggerSession__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `EditorDebuggerSession` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`EditorDebuggerSession::send_message_ex`][super::EditorDebuggerSession::send_message_ex]."]
#[must_use]
pub struct ExSendMessage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorDebuggerSession, message: CowArg < 'a, GString >, data: CowArg < 'a, VarArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSendMessage < 'a > {
    fn new(surround_object: &'a mut re_export::EditorDebuggerSession, message: impl AsArg < GString > + 'a,) -> Self {
        let data = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), data: CowArg::Owned(data),
        }
    }
    #[inline]
    pub fn data(self, data: &'a VarArray) -> Self {
        Self {
            data: CowArg::Borrowed(data), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, message, data,
        }
        = self;
        re_export::EditorDebuggerSession::send_message_full(surround_object, message, data.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`EditorDebuggerSession::toggle_profiler_ex`][super::EditorDebuggerSession::toggle_profiler_ex]."]
#[must_use]
pub struct ExToggleProfiler < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorDebuggerSession, profiler: CowArg < 'a, GString >, enable: bool, data: CowArg < 'a, VarArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExToggleProfiler < 'a > {
    fn new(surround_object: &'a mut re_export::EditorDebuggerSession, profiler: impl AsArg < GString > + 'a, enable: bool,) -> Self {
        let data = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, profiler: profiler.into_arg(), enable: enable, data: CowArg::Owned(data),
        }
    }
    #[inline]
    pub fn data(self, data: &'a VarArray) -> Self {
        Self {
            data: CowArg::Borrowed(data), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, profiler, enable, data,
        }
        = self;
        re_export::EditorDebuggerSession::toggle_profiler_full(surround_object, profiler, enable, data.cow_as_arg(),)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorDebuggerSession;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`EditorDebuggerSession`][crate::classes::EditorDebuggerSession] class."]
    pub struct SignalsOfEditorDebuggerSession < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfEditorDebuggerSession < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn started(&mut self) -> SigStarted < 'c, C > {
            SigStarted {
                typed: TypedSignal::extract(&mut self.__internal_obj, "started")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn stopped(&mut self) -> SigStopped < 'c, C > {
            SigStopped {
                typed: TypedSignal::extract(&mut self.__internal_obj, "stopped")
            }
        }
        #[doc = "Signature: `(can_debug: bool)`"]
        pub fn breaked(&mut self) -> SigBreaked < 'c, C > {
            SigBreaked {
                typed: TypedSignal::extract(&mut self.__internal_obj, "breaked")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn continued(&mut self) -> SigContinued < 'c, C > {
            SigContinued {
                typed: TypedSignal::extract(&mut self.__internal_obj, "continued")
            }
        }
    }
    type TypedSigStarted < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigStarted < 'c, C: WithSignals > {
        typed: TypedSigStarted < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigStarted < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigStarted < 'c, C > {
        type Target = TypedSigStarted < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigStarted < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigStopped < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigStopped < 'c, C: WithSignals > {
        typed: TypedSigStopped < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigStopped < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigStopped < 'c, C > {
        type Target = TypedSigStopped < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigStopped < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigBreaked < 'c, C > = TypedSignal < 'c, C, (bool,) >;
    pub struct SigBreaked < 'c, C: WithSignals > {
        typed: TypedSigBreaked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBreaked < 'c, C > {
        pub fn emit(&mut self, can_debug: bool,) {
            self.typed.emit_tuple((can_debug,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigBreaked < 'c, C > {
        type Target = TypedSigBreaked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigBreaked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigContinued < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigContinued < 'c, C: WithSignals > {
        typed: TypedSigContinued < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigContinued < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigContinued < 'c, C > {
        type Target = TypedSigContinued < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigContinued < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for EditorDebuggerSession {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfEditorDebuggerSession < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfEditorDebuggerSession < 'c, C > {
        type Target = < < EditorDebuggerSession as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = EditorDebuggerSession;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfEditorDebuggerSession < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = EditorDebuggerSession;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}