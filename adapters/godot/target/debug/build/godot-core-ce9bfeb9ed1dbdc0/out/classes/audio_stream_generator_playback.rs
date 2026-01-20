#![doc = "Sidecar module for class [`AudioStreamGeneratorPlayback`][crate::classes::AudioStreamGeneratorPlayback].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamGeneratorPlayback` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamgeneratorplayback.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamGeneratorPlayback.`\n\nInherits [`AudioStreamPlaybackResampled`][crate::classes::AudioStreamPlaybackResampled].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `AudioStreamGeneratorPlayback`](https://docs.godotengine.org/en/stable/classes/class_audiostreamgeneratorplayback.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<AudioStreamGeneratorPlayback>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamGeneratorPlayback {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl AudioStreamGeneratorPlayback {
        pub fn push_frame(&mut self, frame: Vector2,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector2,);
            let args = (frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamGeneratorPlayback", "push_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn can_push_buffer(&self, amount: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamGeneratorPlayback", "can_push_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn push_buffer(&mut self, frames: &PackedVector2Array,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >,);
            let args = (RefArg::new(frames),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(861usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamGeneratorPlayback", "push_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frames_available(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(862usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamGeneratorPlayback", "get_frames_available", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_skips(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(863usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamGeneratorPlayback", "get_skips", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_buffer(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(864usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamGeneratorPlayback", "clear_buffer", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AudioStreamGeneratorPlayback {
        type Base = crate::classes::AudioStreamPlaybackResampled;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AudioStreamGeneratorPlayback"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamGeneratorPlayback {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioStreamPlaybackResampled > for AudioStreamGeneratorPlayback {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioStreamPlayback > for AudioStreamGeneratorPlayback {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioStreamGeneratorPlayback {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioStreamGeneratorPlayback {
        
    }
    impl std::ops::Deref for AudioStreamGeneratorPlayback {
        type Target = crate::classes::AudioStreamPlaybackResampled;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamGeneratorPlayback {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AudioStreamGeneratorPlayback__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `AudioStreamGeneratorPlayback` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AudioStreamGeneratorPlayback;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for AudioStreamGeneratorPlayback {
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