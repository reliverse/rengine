#![doc = "Sidecar module for class [`InputEventWithModifiers`][crate::classes::InputEventWithModifiers].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputEventWithModifiers` enums](https://docs.godotengine.org/en/stable/classes/class_inputeventwithmodifiers.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputEventWithModifiers.`\n\nInherits [`InputEventFromWindow`][crate::classes::InputEventFromWindow].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `InputEventWithModifiers`](https://docs.godotengine.org/en/stable/classes/class_inputeventwithmodifiers.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<InputEventWithModifiers>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputEventWithModifiers {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl InputEventWithModifiers {
        pub fn set_command_or_control_autoremap(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4570usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "set_command_or_control_autoremap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_command_or_control_autoremap(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4571usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "is_command_or_control_autoremap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_command_or_control_pressed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4572usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "is_command_or_control_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alt_pressed(&mut self, pressed: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4573usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "set_alt_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_alt_pressed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4574usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "is_alt_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shift_pressed(&mut self, pressed: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4575usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "set_shift_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_shift_pressed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4576usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "is_shift_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ctrl_pressed(&mut self, pressed: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4577usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "set_ctrl_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_ctrl_pressed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4578usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "is_ctrl_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_meta_pressed(&mut self, pressed: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4579usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "set_meta_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_meta_pressed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4580usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "is_meta_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_modifiers_mask(&self,) -> crate::global::KeyModifierMask {
            type CallRet = crate::global::KeyModifierMask;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4581usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventWithModifiers", "get_modifiers_mask", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for InputEventWithModifiers {
        type Base = crate::classes::InputEventFromWindow;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("InputEventWithModifiers"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputEventWithModifiers {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::InputEventFromWindow > for InputEventWithModifiers {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::InputEvent > for InputEventWithModifiers {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for InputEventWithModifiers {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for InputEventWithModifiers {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for InputEventWithModifiers {
        
    }
    impl std::ops::Deref for InputEventWithModifiers {
        type Target = crate::classes::InputEventFromWindow;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputEventWithModifiers {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_InputEventWithModifiers__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `InputEventWithModifiers` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::InputEventWithModifiers;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for InputEventWithModifiers {
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