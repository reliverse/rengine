#![doc = "Sidecar module for class [`RenderSceneData`][crate::classes::RenderSceneData].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RenderSceneData` enums](https://docs.godotengine.org/en/stable/classes/class_renderscenedata.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RenderSceneData.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `RenderSceneData`](https://docs.godotengine.org/en/stable/classes/class_renderscenedata.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<RenderSceneData>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RenderSceneData {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl RenderSceneData {
        pub fn get_cam_transform(&self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(749usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneData", "get_cam_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cam_projection(&self,) -> Projection {
            type CallRet = Projection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(750usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneData", "get_cam_projection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_view_count(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(751usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneData", "get_view_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_view_eye_offset(&self, view: u32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (u32,);
            let args = (view,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(752usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneData", "get_view_eye_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_view_projection(&self, view: u32,) -> Projection {
            type CallRet = Projection;
            type CallParams = (u32,);
            let args = (view,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(753usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneData", "get_view_projection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_uniform_buffer(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(754usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneData", "get_uniform_buffer", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for RenderSceneData {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("RenderSceneData"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for RenderSceneData {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RenderSceneData {
        
    }
    impl std::ops::Deref for RenderSceneData {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RenderSceneData {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RenderSceneData__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `RenderSceneData` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::RenderSceneData;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for RenderSceneData {
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