#![doc = "Sidecar module for class [`RenderData`][crate::classes::RenderData].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RenderData` enums](https://docs.godotengine.org/en/stable/classes/class_renderdata.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RenderData.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `RenderData`](https://docs.godotengine.org/en/stable/classes/class_renderdata.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<RenderData>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RenderData {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl RenderData {
        pub fn get_render_scene_buffers(&self,) -> Option < Gd < crate::classes::RenderSceneBuffers > > {
            type CallRet = Option < Gd < crate::classes::RenderSceneBuffers > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(745usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderData", "get_render_scene_buffers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_render_scene_data(&self,) -> Option < Gd < crate::classes::RenderSceneData > > {
            type CallRet = Option < Gd < crate::classes::RenderSceneData > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(746usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderData", "get_render_scene_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_environment(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(747usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderData", "get_environment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_camera_attributes(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(748usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderData", "get_camera_attributes", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for RenderData {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("RenderData"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for RenderData {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RenderData {
        
    }
    impl std::ops::Deref for RenderData {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RenderData {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RenderData__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `RenderData` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::RenderData;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for RenderData {
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