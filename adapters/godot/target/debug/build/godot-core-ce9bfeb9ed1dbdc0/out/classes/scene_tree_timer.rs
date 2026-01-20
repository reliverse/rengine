#![doc = "Sidecar module for class [`SceneTreeTimer`][crate::classes::SceneTreeTimer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SceneTreeTimer` enums](https://docs.godotengine.org/en/stable/classes/class_scenetreetimer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SceneTreeTimer.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`scene_tree_timer`][crate::classes::scene_tree_timer]: sidecar module with related enum/flag types\n* [`SignalsOfSceneTreeTimer`][crate::classes::scene_tree_timer::SignalsOfSceneTreeTimer]: signal collection\n\n\nSee also [Godot docs for `SceneTreeTimer`](https://docs.godotengine.org/en/stable/classes/class_scenetreetimer.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<SceneTreeTimer>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SceneTreeTimer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl SceneTreeTimer {
        pub fn set_time_left(&mut self, time: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7891usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTreeTimer", "set_time_left", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_time_left(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7892usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTreeTimer", "get_time_left", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for SceneTreeTimer {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("SceneTreeTimer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SceneTreeTimer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for SceneTreeTimer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SceneTreeTimer {
        
    }
    impl std::ops::Deref for SceneTreeTimer {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SceneTreeTimer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_SceneTreeTimer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `SceneTreeTimer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::SceneTreeTimer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`SceneTreeTimer`][crate::classes::SceneTreeTimer] class."]
    pub struct SignalsOfSceneTreeTimer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfSceneTreeTimer < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn timeout(&mut self) -> SigTimeout < 'c, C > {
            SigTimeout {
                typed: TypedSignal::extract(&mut self.__internal_obj, "timeout")
            }
        }
    }
    type TypedSigTimeout < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTimeout < 'c, C: WithSignals > {
        typed: TypedSigTimeout < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTimeout < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTimeout < 'c, C > {
        type Target = TypedSigTimeout < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTimeout < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for SceneTreeTimer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfSceneTreeTimer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfSceneTreeTimer < 'c, C > {
        type Target = < < SceneTreeTimer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = SceneTreeTimer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfSceneTreeTimer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = SceneTreeTimer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}