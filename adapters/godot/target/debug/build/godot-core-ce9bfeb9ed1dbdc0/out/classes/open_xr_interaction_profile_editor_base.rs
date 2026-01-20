#![doc = "Sidecar module for class [`OpenXrInteractionProfileEditorBase`][crate::classes::OpenXrInteractionProfileEditorBase].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRInteractionProfileEditorBase` enums](https://docs.godotengine.org/en/stable/classes/class_openxrinteractionprofileeditorbase.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRInteractionProfileEditorBase.`\n\nInherits [`HBoxContainer`][crate::classes::HBoxContainer].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `OpenXRInteractionProfileEditorBase`](https://docs.godotengine.org/en/stable/classes/class_openxrinteractionprofileeditorbase.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<OpenXrInteractionProfileEditorBase>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrInteractionProfileEditorBase {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl OpenXrInteractionProfileEditorBase {
        pub fn setup(&mut self, action_map: impl AsArg < Option < Gd < crate::classes::OpenXrActionMap >> >, interaction_profile: impl AsArg < Option < Gd < crate::classes::OpenXrInteractionProfile >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrActionMap > > >, CowArg < 'a1, Option < Gd < crate::classes::OpenXrInteractionProfile > > >,);
            let args = (action_map.into_arg(), interaction_profile.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(439usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrInteractionProfileEditorBase", "setup", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for OpenXrInteractionProfileEditorBase {
        type Base = crate::classes::HBoxContainer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OpenXRInteractionProfileEditorBase"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrInteractionProfileEditorBase {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::HBoxContainer > for OpenXrInteractionProfileEditorBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::BoxContainer > for OpenXrInteractionProfileEditorBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Container > for OpenXrInteractionProfileEditorBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for OpenXrInteractionProfileEditorBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for OpenXrInteractionProfileEditorBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for OpenXrInteractionProfileEditorBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrInteractionProfileEditorBase {
        
    }
    impl std::ops::Deref for OpenXrInteractionProfileEditorBase {
        type Target = crate::classes::HBoxContainer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrInteractionProfileEditorBase {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrInteractionProfileEditorBase__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `OpenXrInteractionProfileEditorBase` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OpenXrInteractionProfileEditorBase;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::container::SignalsOfContainer;
    impl WithSignals for OpenXrInteractionProfileEditorBase {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfContainer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}