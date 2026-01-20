#![doc = "Sidecar module for class [`VisualShaderNodeSample3D`][crate::classes::VisualShaderNodeSample3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeSample3D` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodesample3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeSample3D.`\n\nInherits [`VisualShaderNode`][crate::classes::VisualShaderNode].\n\nRelated symbols:\n\n* [`visual_shader_node_sample_3d`][crate::classes::visual_shader_node_sample_3d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `VisualShaderNodeSample3D`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodesample3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<VisualShaderNodeSample3D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeSample3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl VisualShaderNodeSample3D {
        pub fn set_source(&mut self, value: crate::classes::visual_shader_node_sample_3d::Source,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_sample_3d::Source,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10778usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeSample3D", "set_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_source(&self,) -> crate::classes::visual_shader_node_sample_3d::Source {
            type CallRet = crate::classes::visual_shader_node_sample_3d::Source;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10779usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeSample3D", "get_source", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeSample3D {
        type Base = crate::classes::VisualShaderNode;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VisualShaderNodeSample3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeSample3D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeSample3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeSample3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeSample3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeSample3D {
        
    }
    impl std::ops::Deref for VisualShaderNodeSample3D {
        type Target = crate::classes::VisualShaderNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeSample3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisualShaderNodeSample3D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `VisualShaderNodeSample3D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Source {
    ord: i32
}
impl Source {
    #[doc(alias = "SOURCE_TEXTURE")]
    #[doc = "Godot enumerator name: `SOURCE_TEXTURE`"]
    pub const TEXTURE: Source = Source {
        ord: 0i32
    };
    #[doc(alias = "SOURCE_PORT")]
    #[doc = "Godot enumerator name: `SOURCE_PORT`"]
    pub const PORT: Source = Source {
        ord: 1i32
    };
    #[doc(alias = "SOURCE_MAX")]
    #[doc = "Godot enumerator name: `SOURCE_MAX`"]
    pub const MAX: Source = Source {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Source") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Source {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::TEXTURE => "TEXTURE", Self::PORT => "PORT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Source::TEXTURE, Source::PORT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Source >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TEXTURE", "SOURCE_TEXTURE", Source::TEXTURE), crate::meta::inspect::EnumConstant::new("PORT", "SOURCE_PORT", Source::PORT), crate::meta::inspect::EnumConstant::new("MAX", "SOURCE_MAX", Source::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Source {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for Source {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Source {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Source {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VisualShaderNodeSample3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VisualShaderNodeSample3D {
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