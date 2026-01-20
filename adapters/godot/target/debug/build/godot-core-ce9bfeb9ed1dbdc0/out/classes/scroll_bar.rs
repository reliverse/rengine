#![doc = "Sidecar module for class [`ScrollBar`][crate::classes::ScrollBar].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ScrollBar` enums](https://docs.godotengine.org/en/stable/classes/class_scrollbar.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ScrollBar.`\n\nInherits [`Range`][crate::classes::Range].\n\nRelated symbols:\n\n* [`scroll_bar`][crate::classes::scroll_bar]: sidecar module with related enum/flag types\n* [`SignalsOfScrollBar`][crate::classes::scroll_bar::SignalsOfScrollBar]: signal collection\n\n\nSee also [Godot docs for `ScrollBar`](https://docs.godotengine.org/en/stable/classes/class_scrollbar.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<ScrollBar>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ScrollBar {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl ScrollBar {
        pub fn set_custom_step(&mut self, step: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (step,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7927usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollBar", "set_custom_step", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_step(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7928usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollBar", "get_custom_step", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ScrollBar {
        type Base = crate::classes::Range;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ScrollBar"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ScrollBar {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Range > for ScrollBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for ScrollBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for ScrollBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for ScrollBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ScrollBar {
        
    }
    impl std::ops::Deref for ScrollBar {
        type Target = crate::classes::Range;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ScrollBar {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ScrollBar__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `ScrollBar` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ScrollBar;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`ScrollBar`][crate::classes::ScrollBar] class."]
    pub struct SignalsOfScrollBar < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfScrollBar < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn scrolling(&mut self) -> SigScrolling < 'c, C > {
            SigScrolling {
                typed: TypedSignal::extract(&mut self.__internal_obj, "scrolling")
            }
        }
    }
    type TypedSigScrolling < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigScrolling < 'c, C: WithSignals > {
        typed: TypedSigScrolling < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigScrolling < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigScrolling < 'c, C > {
        type Target = TypedSigScrolling < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigScrolling < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for ScrollBar {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfScrollBar < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfScrollBar < 'c, C > {
        type Target = < < ScrollBar as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = ScrollBar;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfScrollBar < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = ScrollBar;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}