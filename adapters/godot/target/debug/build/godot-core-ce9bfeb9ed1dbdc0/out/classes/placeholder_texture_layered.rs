#![doc = "Sidecar module for class [`PlaceholderTextureLayered`][crate::classes::PlaceholderTextureLayered].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PlaceholderTextureLayered` enums](https://docs.godotengine.org/en/stable/classes/class_placeholdertexturelayered.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PlaceholderTextureLayered.`\n\nInherits [`TextureLayered`][crate::classes::TextureLayered].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `PlaceholderTextureLayered`](https://docs.godotengine.org/en/stable/classes/class_placeholdertexturelayered.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<PlaceholderTextureLayered>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PlaceholderTextureLayered {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl PlaceholderTextureLayered {
        pub fn set_size(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PlaceholderTextureLayered", "set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PlaceholderTextureLayered", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_layers(&mut self, layers: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layers,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PlaceholderTextureLayered", "set_layers", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PlaceholderTextureLayered {
        type Base = crate::classes::TextureLayered;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PlaceholderTextureLayered"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PlaceholderTextureLayered {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::TextureLayered > for PlaceholderTextureLayered {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture > for PlaceholderTextureLayered {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for PlaceholderTextureLayered {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for PlaceholderTextureLayered {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PlaceholderTextureLayered {
        
    }
    impl std::ops::Deref for PlaceholderTextureLayered {
        type Target = crate::classes::TextureLayered;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PlaceholderTextureLayered {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PlaceholderTextureLayered__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `PlaceholderTextureLayered` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PlaceholderTextureLayered;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for PlaceholderTextureLayered {
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