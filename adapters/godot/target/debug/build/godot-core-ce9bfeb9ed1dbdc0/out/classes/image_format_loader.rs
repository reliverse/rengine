#![doc = "Sidecar module for class [`ImageFormatLoader`][crate::classes::ImageFormatLoader].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ImageFormatLoader` enums](https://docs.godotengine.org/en/stable/classes/class_imageformatloader.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ImageFormatLoader.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`image_format_loader`][crate::classes::image_format_loader]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `ImageFormatLoader`](https://docs.godotengine.org/en/stable/classes/class_imageformatloader.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<ImageFormatLoader>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ImageFormatLoader {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl ImageFormatLoader {
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
    impl crate::obj::GodotClass for ImageFormatLoader {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ImageFormatLoader"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ImageFormatLoader {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ImageFormatLoader {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ImageFormatLoader {
        
    }
    impl std::ops::Deref for ImageFormatLoader {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ImageFormatLoader {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ImageFormatLoader__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `ImageFormatLoader` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct LoaderFlags {
    ord: u64
}
impl LoaderFlags {
    #[doc(alias = "FLAG_NONE")]
    #[doc = "Godot enumerator name: `FLAG_NONE`"]
    pub const NONE: LoaderFlags = LoaderFlags {
        ord: 0u64
    };
    #[doc(alias = "FLAG_FORCE_LINEAR")]
    #[doc = "Godot enumerator name: `FLAG_FORCE_LINEAR`"]
    pub const FORCE_LINEAR: LoaderFlags = LoaderFlags {
        ord: 1u64
    };
    #[doc(alias = "FLAG_CONVERT_COLORS")]
    #[doc = "Godot enumerator name: `FLAG_CONVERT_COLORS`"]
    pub const CONVERT_COLORS: LoaderFlags = LoaderFlags {
        ord: 2u64
    };
    
}
impl std::fmt::Debug for LoaderFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::NONE => "NONE", Self::FORCE_LINEAR => "FORCE_LINEAR", Self::CONVERT_COLORS => "CONVERT_COLORS", _ => {
                f.debug_struct("LoaderFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for LoaderFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LoaderFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "FLAG_NONE", LoaderFlags::NONE), crate::meta::inspect::EnumConstant::new("FORCE_LINEAR", "FLAG_FORCE_LINEAR", LoaderFlags::FORCE_LINEAR), crate::meta::inspect::EnumConstant::new("CONVERT_COLORS", "FLAG_CONVERT_COLORS", LoaderFlags::CONVERT_COLORS)]
        }
    }
}
impl std::ops::BitOr for LoaderFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for LoaderFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for LoaderFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for LoaderFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LoaderFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ImageFormatLoader;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for ImageFormatLoader {
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