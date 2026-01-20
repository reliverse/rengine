#![doc = "Sidecar module for class [`InputEventFromWindow`][crate::classes::InputEventFromWindow].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputEventFromWindow` enums](https://docs.godotengine.org/en/stable/classes/class_inputeventfromwindow.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputEventFromWindow.`\n\nInherits [`InputEvent`][crate::classes::InputEvent].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `InputEventFromWindow`](https://docs.godotengine.org/en/stable/classes/class_inputeventfromwindow.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<InputEventFromWindow>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputEventFromWindow {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl InputEventFromWindow {
        pub fn set_window_id(&mut self, id: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4462usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventFromWindow", "set_window_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_window_id(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4463usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventFromWindow", "get_window_id", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for InputEventFromWindow {
        type Base = crate::classes::InputEvent;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("InputEventFromWindow"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputEventFromWindow {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::InputEvent > for InputEventFromWindow {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for InputEventFromWindow {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for InputEventFromWindow {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for InputEventFromWindow {
        
    }
    impl std::ops::Deref for InputEventFromWindow {
        type Target = crate::classes::InputEvent;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputEventFromWindow {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_InputEventFromWindow__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `InputEventFromWindow` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::InputEventFromWindow;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for InputEventFromWindow {
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