#![doc = "Sidecar module for class [`CsgPrimitive3D`][crate::classes::CsgPrimitive3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CSGPrimitive3D` enums](https://docs.godotengine.org/en/stable/classes/class_csgprimitive3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CSGPrimitive3D.`\n\nInherits [`CsgShape3D`][crate::classes::CsgShape3D].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `CSGPrimitive3D`](https://docs.godotengine.org/en/stable/classes/class_csgprimitive3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<CsgPrimitive3D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CsgPrimitive3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl CsgPrimitive3D {
        pub fn set_flip_faces(&mut self, flip_faces: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (flip_faces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1598usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPrimitive3D", "set_flip_faces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_flip_faces(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1599usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPrimitive3D", "get_flip_faces", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CsgPrimitive3D {
        type Base = crate::classes::CsgShape3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CSGPrimitive3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CsgPrimitive3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CsgShape3D > for CsgPrimitive3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for CsgPrimitive3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for CsgPrimitive3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for CsgPrimitive3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CsgPrimitive3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CsgPrimitive3D {
        
    }
    impl std::ops::Deref for CsgPrimitive3D {
        type Target = crate::classes::CsgShape3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CsgPrimitive3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CsgPrimitive3D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `CsgPrimitive3D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CsgPrimitive3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for CsgPrimitive3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}