#![doc = "Sidecar module for class [`Script`][crate::classes::Script].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Script` enums](https://docs.godotengine.org/en/stable/classes/class_script.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Script.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`script`][crate::classes::script]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `Script`](https://docs.godotengine.org/en/stable/classes/class_script.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Script>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Script {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Script {
        pub fn can_instantiate(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7893usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "can_instantiate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_has(&self, base_object: impl AsArg < Option < Gd < crate::classes::Object >> >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >,);
            let args = (base_object.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7894usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "instance_has", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_source_code(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7895usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "has_source_code", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_source_code(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7896usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "get_source_code", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_source_code(&mut self, source: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (source.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7897usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "set_source_code", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn reload_full(&mut self, keep_state: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (bool,);
            let args = (keep_state,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7898usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "reload", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::reload_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn reload(&mut self,) -> crate::global::Error {
            self.reload_ex() . done()
        }
        #[inline]
        pub fn reload_ex < 'a > (&'a mut self,) -> ExReload < 'a > {
            ExReload::new(self,)
        }
        pub fn get_base_script(&self,) -> Option < Gd < crate::classes::Script > > {
            type CallRet = Option < Gd < crate::classes::Script > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7899usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "get_base_script", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_instance_base_type(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7900usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "get_instance_base_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_name(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7901usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "get_global_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_script_signal(&self, signal_name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (signal_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7902usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "has_script_signal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_script_property_list(&mut self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7903usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "get_script_property_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_script_method_list(&mut self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7904usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "get_script_method_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_script_signal_list(&mut self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7905usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "get_script_signal_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_script_constant_map(&mut self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7906usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "get_script_constant_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_property_default_value(&mut self, property: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (property.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7907usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "get_property_default_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_tool(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7908usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "is_tool", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_abstract(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7909usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "is_abstract", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rpc_config(&self,) -> Variant {
            type CallRet = Variant;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7910usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Script", "get_rpc_config", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Script {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Script"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Script {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Script {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Script {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Script {
        
    }
    impl std::ops::Deref for Script {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Script {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Script__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Script` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Script::reload_ex`][super::Script::reload_ex]."]
#[must_use]
pub struct ExReload < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Script, keep_state: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExReload < 'a > {
    fn new(surround_object: &'a mut re_export::Script,) -> Self {
        let keep_state = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, keep_state: keep_state,
        }
    }
    #[inline]
    pub fn keep_state(self, keep_state: bool) -> Self {
        Self {
            keep_state: keep_state, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, keep_state,
        }
        = self;
        re_export::Script::reload_full(surround_object, keep_state,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Script;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Script {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfResource < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}