#![doc = "Sidecar module for class [`EngineDebugger`][crate::classes::EngineDebugger].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EngineDebugger` enums](https://docs.godotengine.org/en/stable/classes/class_enginedebugger.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EngineDebugger.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`engine_debugger`][crate::classes::engine_debugger]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `EngineDebugger`](https://docs.godotengine.org/en/stable/classes/class_enginedebugger.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EngineDebugger {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl EngineDebugger {
        pub fn is_active(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2933usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "is_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn register_profiler(&mut self, name: impl AsArg < StringName >, profiler: impl AsArg < Option < Gd < crate::classes::EngineProfiler >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::EngineProfiler > > >,);
            let args = (name.into_arg(), profiler.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2934usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "register_profiler", Some(self.__validated_obj()), args,)
            }
        }
        pub fn unregister_profiler(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2935usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "unregister_profiler", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_profiling(&mut self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2936usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "is_profiling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_profiler(&mut self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2937usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "has_profiler", Some(self.__validated_obj()), args,)
            }
        }
        pub fn profiler_add_frame_data(&mut self, name: impl AsArg < StringName >, data: &VarArray,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, VarArray >,);
            let args = (name.into_arg(), RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2938usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "profiler_add_frame_data", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn profiler_enable_full(&mut self, name: CowArg < StringName >, enable: bool, arguments: RefArg < VarArray >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, bool, RefArg < 'a1, VarArray >,);
            let args = (name, enable, arguments,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2939usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "profiler_enable", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::profiler_enable_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn profiler_enable(&mut self, name: impl AsArg < StringName >, enable: bool,) {
            self.profiler_enable_ex(name, enable,) . done()
        }
        #[inline]
        pub fn profiler_enable_ex < 'a > (&'a mut self, name: impl AsArg < StringName > + 'a, enable: bool,) -> ExProfilerEnable < 'a > {
            ExProfilerEnable::new(self, name, enable,)
        }
        pub fn register_message_capture(&mut self, name: impl AsArg < StringName >, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Callable >,);
            let args = (name.into_arg(), RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2940usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "register_message_capture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn unregister_message_capture(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2941usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "unregister_message_capture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_capture(&mut self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2942usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "has_capture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn line_poll(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2943usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "line_poll", Some(self.__validated_obj()), args,)
            }
        }
        pub fn send_message(&mut self, message: impl AsArg < GString >, data: &VarArray,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, VarArray >,);
            let args = (message.into_arg(), RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2944usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "send_message", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn debug_full(&mut self, can_continue: bool, is_error_breakpoint: bool,) {
            type CallRet = ();
            type CallParams = (bool, bool,);
            let args = (can_continue, is_error_breakpoint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2945usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "debug", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::debug_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn debug(&mut self,) {
            self.debug_ex() . done()
        }
        #[inline]
        pub fn debug_ex < 'a > (&'a mut self,) -> ExDebug < 'a > {
            ExDebug::new(self,)
        }
        pub(crate) fn script_debug_full(&mut self, language: CowArg < Option < Gd < crate::classes::ScriptLanguage > > >, can_continue: bool, is_error_breakpoint: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ScriptLanguage > > >, bool, bool,);
            let args = (language, can_continue, is_error_breakpoint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2946usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "script_debug", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::script_debug_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn script_debug(&mut self, language: impl AsArg < Option < Gd < crate::classes::ScriptLanguage >> >,) {
            self.script_debug_ex(language,) . done()
        }
        #[inline]
        pub fn script_debug_ex < 'a > (&'a mut self, language: impl AsArg < Option < Gd < crate::classes::ScriptLanguage >> > + 'a,) -> ExScriptDebug < 'a > {
            ExScriptDebug::new(self, language,)
        }
        pub fn set_lines_left(&mut self, lines: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (lines,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2947usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "set_lines_left", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lines_left(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2948usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "get_lines_left", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_depth(&mut self, depth: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (depth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2949usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "set_depth", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_depth(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2950usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "get_depth", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_breakpoint(&self, line: i32, source: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (i32, CowArg < 'a0, StringName >,);
            let args = (line, source.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2951usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "is_breakpoint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_skipping_breakpoints(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2952usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "is_skipping_breakpoints", Some(self.__validated_obj()), args,)
            }
        }
        pub fn insert_breakpoint(&mut self, line: i32, source: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, StringName >,);
            let args = (line, source.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2953usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "insert_breakpoint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_breakpoint(&mut self, line: i32, source: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, StringName >,);
            let args = (line, source.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2954usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "remove_breakpoint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_breakpoints(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2955usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EngineDebugger", "clear_breakpoints", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EngineDebugger {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EngineDebugger"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for EngineDebugger {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EngineDebugger {
        
    }
    impl crate::obj::Singleton for EngineDebugger {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"EngineDebugger"))
            }
        }
    }
    impl std::ops::Deref for EngineDebugger {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EngineDebugger {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EngineDebugger__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `EngineDebugger` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`EngineDebugger::profiler_enable_ex`][super::EngineDebugger::profiler_enable_ex]."]
#[must_use]
pub struct ExProfilerEnable < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EngineDebugger, name: CowArg < 'a, StringName >, enable: bool, arguments: CowArg < 'a, VarArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExProfilerEnable < 'a > {
    fn new(surround_object: &'a mut re_export::EngineDebugger, name: impl AsArg < StringName > + 'a, enable: bool,) -> Self {
        let arguments = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), enable: enable, arguments: CowArg::Owned(arguments),
        }
    }
    #[inline]
    pub fn arguments(self, arguments: &'a VarArray) -> Self {
        Self {
            arguments: CowArg::Borrowed(arguments), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, enable, arguments,
        }
        = self;
        re_export::EngineDebugger::profiler_enable_full(surround_object, name, enable, arguments.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`EngineDebugger::debug_ex`][super::EngineDebugger::debug_ex]."]
#[must_use]
pub struct ExDebug < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EngineDebugger, can_continue: bool, is_error_breakpoint: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDebug < 'a > {
    fn new(surround_object: &'a mut re_export::EngineDebugger,) -> Self {
        let can_continue = true;
        let is_error_breakpoint = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, can_continue: can_continue, is_error_breakpoint: is_error_breakpoint,
        }
    }
    #[inline]
    pub fn can_continue(self, can_continue: bool) -> Self {
        Self {
            can_continue: can_continue, .. self
        }
    }
    #[inline]
    pub fn is_error_breakpoint(self, is_error_breakpoint: bool) -> Self {
        Self {
            is_error_breakpoint: is_error_breakpoint, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, can_continue, is_error_breakpoint,
        }
        = self;
        re_export::EngineDebugger::debug_full(surround_object, can_continue, is_error_breakpoint,)
    }
}
#[doc = "Default-param extender for [`EngineDebugger::script_debug_ex`][super::EngineDebugger::script_debug_ex]."]
#[must_use]
pub struct ExScriptDebug < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EngineDebugger, language: CowArg < 'a, Option < Gd < crate::classes::ScriptLanguage > > >, can_continue: bool, is_error_breakpoint: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScriptDebug < 'a > {
    fn new(surround_object: &'a mut re_export::EngineDebugger, language: impl AsArg < Option < Gd < crate::classes::ScriptLanguage >> > + 'a,) -> Self {
        let can_continue = true;
        let is_error_breakpoint = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, language: language.into_arg(), can_continue: can_continue, is_error_breakpoint: is_error_breakpoint,
        }
    }
    #[inline]
    pub fn can_continue(self, can_continue: bool) -> Self {
        Self {
            can_continue: can_continue, .. self
        }
    }
    #[inline]
    pub fn is_error_breakpoint(self, is_error_breakpoint: bool) -> Self {
        Self {
            is_error_breakpoint: is_error_breakpoint, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, language, can_continue, is_error_breakpoint,
        }
        = self;
        re_export::EngineDebugger::script_debug_full(surround_object, language, can_continue, is_error_breakpoint,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EngineDebugger;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EngineDebugger {
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