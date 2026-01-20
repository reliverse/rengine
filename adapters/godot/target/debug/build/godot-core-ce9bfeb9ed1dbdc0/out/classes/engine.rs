#![doc = "Sidecar module for class [`Engine`][crate::classes::Engine].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Engine` enums](https://docs.godotengine.org/en/stable/classes/class_engine.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Engine.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`engine`][crate::classes::engine]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `Engine`](https://docs.godotengine.org/en/stable/classes/class_engine.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Engine {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Engine {
        pub fn set_physics_ticks_per_second(&mut self, physics_ticks_per_second: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (physics_ticks_per_second,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(0usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_physics_ticks_per_second", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_ticks_per_second(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(1usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_physics_ticks_per_second", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_physics_steps_per_frame(&mut self, max_physics_steps: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_physics_steps,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(2usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_max_physics_steps_per_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_physics_steps_per_frame(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(3usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_max_physics_steps_per_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_jitter_fix(&mut self, physics_jitter_fix: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (physics_jitter_fix,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(4usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_physics_jitter_fix", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_jitter_fix(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(5usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_physics_jitter_fix", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_interpolation_fraction(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(6usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_physics_interpolation_fraction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_fps(&mut self, max_fps: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_fps,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(7usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_max_fps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_fps(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(8usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_max_fps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_time_scale(&mut self, time_scale: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (time_scale,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(9usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_time_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_time_scale(&mut self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(10usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_time_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frames_drawn(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(11usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_frames_drawn", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frames_per_second(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(12usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_frames_per_second", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_frames(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(13usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_physics_frames", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_frames(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(14usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_process_frames", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_main_loop(&self,) -> Option < Gd < crate::classes::MainLoop > > {
            type CallRet = Option < Gd < crate::classes::MainLoop > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(15usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_main_loop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_version_info(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(16usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_version_info", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_author_info(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(17usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_author_info", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_copyright_info(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(18usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_copyright_info", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_donor_info(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(19usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_donor_info", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_license_info(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(20usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_license_info", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_license_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(21usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_license_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_architecture_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(22usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_architecture_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_in_physics_frame(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(23usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "is_in_physics_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_singleton(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(24usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "has_singleton", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_singleton(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Object > > {
            type CallRet = Option < Gd < crate::classes::Object > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(25usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_singleton", Some(self.__validated_obj()), args,)
            }
        }
        pub fn register_singleton(&mut self, name: impl AsArg < StringName >, instance: impl AsArg < Option < Gd < crate::classes::Object >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::Object > > >,);
            let args = (name.into_arg(), instance.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(26usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "register_singleton", Some(self.__validated_obj()), args,)
            }
        }
        pub fn unregister_singleton(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(27usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "unregister_singleton", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_singleton_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(28usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_singleton_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn register_script_language(&mut self, language: impl AsArg < Option < Gd < crate::classes::ScriptLanguage >> >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ScriptLanguage > > >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(29usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "register_script_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn unregister_script_language(&mut self, language: impl AsArg < Option < Gd < crate::classes::ScriptLanguage >> >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ScriptLanguage > > >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(30usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "unregister_script_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_script_language_count(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(31usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_script_language_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_script_language(&self, index: i32,) -> Option < Gd < crate::classes::ScriptLanguage > > {
            type CallRet = Option < Gd < crate::classes::ScriptLanguage > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(32usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_script_language", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn capture_script_backtraces_full(&self, include_variables: bool,) -> Array < Gd < crate::classes::ScriptBacktrace > > {
            type CallRet = Array < Gd < crate::classes::ScriptBacktrace > >;
            type CallParams = (bool,);
            let args = (include_variables,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(33usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "capture_script_backtraces", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::capture_script_backtraces_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn capture_script_backtraces(&self,) -> Array < Gd < crate::classes::ScriptBacktrace > > {
            self.capture_script_backtraces_ex() . done()
        }
        #[inline]
        pub fn capture_script_backtraces_ex < 'a > (&'a self,) -> ExCaptureScriptBacktraces < 'a > {
            ExCaptureScriptBacktraces::new(self,)
        }
        pub fn is_editor_hint(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(34usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "is_editor_hint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_embedded_in_editor(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(35usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "is_embedded_in_editor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_write_movie_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(36usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "get_write_movie_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_print_to_stdout(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(37usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_print_to_stdout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_printing_to_stdout(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(38usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "is_printing_to_stdout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_print_error_messages(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(39usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "set_print_error_messages", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_printing_error_messages(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(40usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Engine", "is_printing_error_messages", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Engine {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Engine"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Core;
        
    }
    unsafe impl crate::obj::Bounds for Engine {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Engine {
        
    }
    impl crate::obj::Singleton for Engine {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"Engine"))
            }
        }
    }
    impl std::ops::Deref for Engine {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Engine {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Engine__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Engine` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Engine::capture_script_backtraces_ex`][super::Engine::capture_script_backtraces_ex]."]
#[must_use]
pub struct ExCaptureScriptBacktraces < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Engine, include_variables: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCaptureScriptBacktraces < 'a > {
    fn new(surround_object: &'a re_export::Engine,) -> Self {
        let include_variables = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, include_variables: include_variables,
        }
    }
    #[inline]
    pub fn include_variables(self, include_variables: bool) -> Self {
        Self {
            include_variables: include_variables, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::classes::ScriptBacktrace > > {
        let Self {
            _phantom, surround_object, include_variables,
        }
        = self;
        re_export::Engine::capture_script_backtraces_full(surround_object, include_variables,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Engine;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Engine {
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