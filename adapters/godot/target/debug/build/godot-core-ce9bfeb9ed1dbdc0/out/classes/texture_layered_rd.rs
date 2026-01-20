#![doc = "Sidecar module for class [`TextureLayeredRd`][crate::classes::TextureLayeredRd].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextureLayeredRD` enums](https://docs.godotengine.org/en/stable/classes/class_texturelayeredrd.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextureLayeredRD.`\n\nInherits [`TextureLayered`][crate::classes::TextureLayered].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `TextureLayeredRD`](https://docs.godotengine.org/en/stable/classes/class_texturelayeredrd.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<TextureLayeredRd>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextureLayeredRd {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl TextureLayeredRd {
        pub fn set_texture_rd_rid(&mut self, texture_rd_rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (texture_rd_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9543usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureLayeredRd", "set_texture_rd_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_rd_rid(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9544usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureLayeredRd", "get_texture_rd_rid", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TextureLayeredRd {
        type Base = crate::classes::TextureLayered;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TextureLayeredRD"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextureLayeredRd {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::TextureLayered > for TextureLayeredRd {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture > for TextureLayeredRd {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for TextureLayeredRd {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TextureLayeredRd {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextureLayeredRd {
        
    }
    impl std::ops::Deref for TextureLayeredRd {
        type Target = crate::classes::TextureLayered;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextureLayeredRd {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TextureLayeredRd__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `TextureLayeredRd` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TextureLayeredRd;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for TextureLayeredRd {
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