#![doc = "Sidecar module for class [`ScriptLanguage`][crate::classes::ScriptLanguage].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ScriptLanguage` enums](https://docs.godotengine.org/en/stable/classes/class_scriptlanguage.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ScriptLanguage.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`script_language`][crate::classes::script_language]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `ScriptLanguage`](https://docs.godotengine.org/en/stable/classes/class_scriptlanguage.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<ScriptLanguage>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ScriptLanguage {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl ScriptLanguage {
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
    impl crate::obj::GodotClass for ScriptLanguage {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ScriptLanguage"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ScriptLanguage {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ScriptLanguage {
        
    }
    impl std::ops::Deref for ScriptLanguage {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ScriptLanguage {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ScriptLanguage__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `ScriptLanguage` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ScriptNameCasing {
    ord: i32
}
impl ScriptNameCasing {
    #[doc(alias = "SCRIPT_NAME_CASING_AUTO")]
    #[doc = "Godot enumerator name: `SCRIPT_NAME_CASING_AUTO`"]
    pub const AUTO: ScriptNameCasing = ScriptNameCasing {
        ord: 0i32
    };
    #[doc(alias = "SCRIPT_NAME_CASING_PASCAL_CASE")]
    #[doc = "Godot enumerator name: `SCRIPT_NAME_CASING_PASCAL_CASE`"]
    pub const PASCAL_CASE: ScriptNameCasing = ScriptNameCasing {
        ord: 1i32
    };
    #[doc(alias = "SCRIPT_NAME_CASING_SNAKE_CASE")]
    #[doc = "Godot enumerator name: `SCRIPT_NAME_CASING_SNAKE_CASE`"]
    pub const SNAKE_CASE: ScriptNameCasing = ScriptNameCasing {
        ord: 2i32
    };
    #[doc(alias = "SCRIPT_NAME_CASING_KEBAB_CASE")]
    #[doc = "Godot enumerator name: `SCRIPT_NAME_CASING_KEBAB_CASE`"]
    pub const KEBAB_CASE: ScriptNameCasing = ScriptNameCasing {
        ord: 3i32
    };
    #[doc(alias = "SCRIPT_NAME_CASING_CAMEL_CASE")]
    #[doc = "Godot enumerator name: `SCRIPT_NAME_CASING_CAMEL_CASE`"]
    pub const CAMEL_CASE: ScriptNameCasing = ScriptNameCasing {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ScriptNameCasing {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ScriptNameCasing") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ScriptNameCasing {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::AUTO => "AUTO", Self::PASCAL_CASE => "PASCAL_CASE", Self::SNAKE_CASE => "SNAKE_CASE", Self::KEBAB_CASE => "KEBAB_CASE", Self::CAMEL_CASE => "CAMEL_CASE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ScriptNameCasing::AUTO, ScriptNameCasing::PASCAL_CASE, ScriptNameCasing::SNAKE_CASE, ScriptNameCasing::KEBAB_CASE, ScriptNameCasing::CAMEL_CASE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ScriptNameCasing >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("AUTO", "SCRIPT_NAME_CASING_AUTO", ScriptNameCasing::AUTO), crate::meta::inspect::EnumConstant::new("PASCAL_CASE", "SCRIPT_NAME_CASING_PASCAL_CASE", ScriptNameCasing::PASCAL_CASE), crate::meta::inspect::EnumConstant::new("SNAKE_CASE", "SCRIPT_NAME_CASING_SNAKE_CASE", ScriptNameCasing::SNAKE_CASE), crate::meta::inspect::EnumConstant::new("KEBAB_CASE", "SCRIPT_NAME_CASING_KEBAB_CASE", ScriptNameCasing::KEBAB_CASE), crate::meta::inspect::EnumConstant::new("CAMEL_CASE", "SCRIPT_NAME_CASING_CAMEL_CASE", ScriptNameCasing::CAMEL_CASE)]
        }
    }
}
impl crate::meta::GodotConvert for ScriptNameCasing {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ScriptNameCasing {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ScriptNameCasing {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ScriptLanguage;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for ScriptLanguage {
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