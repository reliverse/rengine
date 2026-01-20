#![doc = "Sidecar module for class [`XrTracker`][crate::classes::XrTracker].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `XRTracker` enums](https://docs.godotengine.org/en/stable/classes/class_xrtracker.html#enumerations).\n\n"]
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
    #[doc = "Godot class `XRTracker.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `XRTracker`](https://docs.godotengine.org/en/stable/classes/class_xrtracker.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<XrTracker>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct XrTracker {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl XrTracker {
        pub fn get_tracker_type(&self,) -> crate::classes::xr_server::TrackerType {
            type CallRet = crate::classes::xr_server::TrackerType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrTracker", "get_tracker_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tracker_type(&mut self, type_: crate::classes::xr_server::TrackerType,) {
            type CallRet = ();
            type CallParams = (crate::classes::xr_server::TrackerType,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrTracker", "set_tracker_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tracker_name(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrTracker", "get_tracker_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tracker_name(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrTracker", "set_tracker_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tracker_desc(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrTracker", "get_tracker_desc", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tracker_desc(&mut self, description: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (description.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrTracker", "set_tracker_desc", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for XrTracker {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("XRTracker"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for XrTracker {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for XrTracker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for XrTracker {
        
    }
    impl std::ops::Deref for XrTracker {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for XrTracker {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_XrTracker__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `XrTracker` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::XrTracker;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for XrTracker {
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