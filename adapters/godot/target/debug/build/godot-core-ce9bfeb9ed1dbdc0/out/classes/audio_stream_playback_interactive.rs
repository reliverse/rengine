#![doc = "Sidecar module for class [`AudioStreamPlaybackInteractive`][crate::classes::AudioStreamPlaybackInteractive].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamPlaybackInteractive` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamplaybackinteractive.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamPlaybackInteractive.`\n\nInherits [`AudioStreamPlayback`][crate::classes::AudioStreamPlayback].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `AudioStreamPlaybackInteractive`](https://docs.godotengine.org/en/stable/classes/class_audiostreamplaybackinteractive.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<AudioStreamPlaybackInteractive>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamPlaybackInteractive {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl AudioStreamPlaybackInteractive {
        pub fn switch_to_clip_by_name(&mut self, clip_name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (clip_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(925usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlaybackInteractive", "switch_to_clip_by_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn switch_to_clip(&mut self, clip_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (clip_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(926usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlaybackInteractive", "switch_to_clip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_clip_index(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(927usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlaybackInteractive", "get_current_clip_index", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AudioStreamPlaybackInteractive {
        type Base = crate::classes::AudioStreamPlayback;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AudioStreamPlaybackInteractive"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamPlaybackInteractive {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioStreamPlayback > for AudioStreamPlaybackInteractive {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioStreamPlaybackInteractive {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioStreamPlaybackInteractive {
        
    }
    impl std::ops::Deref for AudioStreamPlaybackInteractive {
        type Target = crate::classes::AudioStreamPlayback;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamPlaybackInteractive {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AudioStreamPlaybackInteractive__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `AudioStreamPlaybackInteractive` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AudioStreamPlaybackInteractive;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for AudioStreamPlaybackInteractive {
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