#![doc = "Sidecar module for class [`VisualShaderNodeParameter`][crate::classes::VisualShaderNodeParameter].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeParameter` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeparameter.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeParameter.`\n\nInherits [`VisualShaderNode`][crate::classes::VisualShaderNode].\n\nRelated symbols:\n\n* [`visual_shader_node_parameter`][crate::classes::visual_shader_node_parameter]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `VisualShaderNodeParameter`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeparameter.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<VisualShaderNodeParameter>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeParameter {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl VisualShaderNodeParameter {
        pub fn set_parameter_name(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10751usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeParameter", "set_parameter_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_parameter_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10752usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeParameter", "get_parameter_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_qualifier(&mut self, qualifier: crate::classes::visual_shader_node_parameter::Qualifier,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_parameter::Qualifier,);
            let args = (qualifier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10753usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeParameter", "set_qualifier", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_qualifier(&self,) -> crate::classes::visual_shader_node_parameter::Qualifier {
            type CallRet = crate::classes::visual_shader_node_parameter::Qualifier;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10754usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeParameter", "get_qualifier", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeParameter {
        type Base = crate::classes::VisualShaderNode;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VisualShaderNodeParameter"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeParameter {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeParameter {
        
    }
    impl std::ops::Deref for VisualShaderNodeParameter {
        type Target = crate::classes::VisualShaderNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeParameter {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisualShaderNodeParameter__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `VisualShaderNodeParameter` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Qualifier {
    ord: i32
}
impl Qualifier {
    #[doc(alias = "QUAL_NONE")]
    #[doc = "Godot enumerator name: `QUAL_NONE`"]
    pub const NONE: Qualifier = Qualifier {
        ord: 0i32
    };
    #[doc(alias = "QUAL_GLOBAL")]
    #[doc = "Godot enumerator name: `QUAL_GLOBAL`"]
    pub const GLOBAL: Qualifier = Qualifier {
        ord: 1i32
    };
    #[doc(alias = "QUAL_INSTANCE")]
    #[doc = "Godot enumerator name: `QUAL_INSTANCE`"]
    pub const INSTANCE: Qualifier = Qualifier {
        ord: 2i32
    };
    #[doc(alias = "QUAL_MAX")]
    #[doc = "Godot enumerator name: `QUAL_MAX`"]
    pub const MAX: Qualifier = Qualifier {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for Qualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Qualifier") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Qualifier {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::NONE => "NONE", Self::GLOBAL => "GLOBAL", Self::INSTANCE => "INSTANCE", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Qualifier::NONE, Qualifier::GLOBAL, Qualifier::INSTANCE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Qualifier >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "QUAL_NONE", Qualifier::NONE), crate::meta::inspect::EnumConstant::new("GLOBAL", "QUAL_GLOBAL", Qualifier::GLOBAL), crate::meta::inspect::EnumConstant::new("INSTANCE", "QUAL_INSTANCE", Qualifier::INSTANCE), crate::meta::inspect::EnumConstant::new("MAX", "QUAL_MAX", Qualifier::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Qualifier {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for Qualifier {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Qualifier {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Qualifier {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VisualShaderNodeParameter;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VisualShaderNodeParameter {
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