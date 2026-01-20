#![doc = "Sidecar module for class [`InputEventMouse`][crate::classes::InputEventMouse].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputEventMouse` enums](https://docs.godotengine.org/en/stable/classes/class_inputeventmouse.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputEventMouse.`\n\nInherits [`InputEventWithModifiers`][crate::classes::InputEventWithModifiers].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `InputEventMouse`](https://docs.godotengine.org/en/stable/classes/class_inputeventmouse.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<InputEventMouse>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputEventMouse {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl InputEventMouse {
        pub fn set_button_mask(&mut self, button_mask: crate::global::MouseButtonMask,) {
            type CallRet = ();
            type CallParams = (crate::global::MouseButtonMask,);
            let args = (button_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4512usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMouse", "set_button_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_button_mask(&self,) -> crate::global::MouseButtonMask {
            type CallRet = crate::global::MouseButtonMask;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4513usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMouse", "get_button_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_position(&mut self, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4514usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMouse", "set_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4515usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMouse", "get_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_global_position(&mut self, global_position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (global_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4516usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMouse", "set_global_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_global_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4517usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMouse", "get_global_position", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for InputEventMouse {
        type Base = crate::classes::InputEventWithModifiers;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("InputEventMouse"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputEventMouse {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::InputEventWithModifiers > for InputEventMouse {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::InputEventFromWindow > for InputEventMouse {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::InputEvent > for InputEventMouse {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for InputEventMouse {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for InputEventMouse {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for InputEventMouse {
        
    }
    impl std::ops::Deref for InputEventMouse {
        type Target = crate::classes::InputEventWithModifiers;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputEventMouse {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_InputEventMouse__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `InputEventMouse` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::InputEventMouse;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for InputEventMouse {
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