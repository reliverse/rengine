#![doc = "Sidecar module for class [`InputEventGesture`][crate::classes::InputEventGesture].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputEventGesture` enums](https://docs.godotengine.org/en/stable/classes/class_inputeventgesture.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputEventGesture.`\n\nInherits [`InputEventWithModifiers`][crate::classes::InputEventWithModifiers].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `InputEventGesture`](https://docs.godotengine.org/en/stable/classes/class_inputeventgesture.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<InputEventGesture>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputEventGesture {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl InputEventGesture {
        pub fn set_position(&mut self, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4464usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventGesture", "set_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_position(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4465usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventGesture", "get_position", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for InputEventGesture {
        type Base = crate::classes::InputEventWithModifiers;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("InputEventGesture"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputEventGesture {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::InputEventWithModifiers > for InputEventGesture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::InputEventFromWindow > for InputEventGesture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::InputEvent > for InputEventGesture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for InputEventGesture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for InputEventGesture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for InputEventGesture {
        
    }
    impl std::ops::Deref for InputEventGesture {
        type Target = crate::classes::InputEventWithModifiers;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputEventGesture {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_InputEventGesture__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `InputEventGesture` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::InputEventGesture;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for InputEventGesture {
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