#![doc = "Sidecar module for class [`EditorToaster`][crate::classes::EditorToaster].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorToaster` enums](https://docs.godotengine.org/en/stable/classes/class_editortoaster.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorToaster.`\n\nInherits [`HBoxContainer`][crate::classes::HBoxContainer].\n\nRelated symbols:\n\n* [`editor_toaster`][crate::classes::editor_toaster]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `EditorToaster`](https://docs.godotengine.org/en/stable/classes/class_editortoaster.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<EditorToaster>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorToaster {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl EditorToaster {
        pub(crate) fn push_toast_full(&mut self, message: CowArg < GString >, severity: crate::classes::editor_toaster::Severity, tooltip: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, crate::classes::editor_toaster::Severity, CowArg < 'a1, GString >,);
            let args = (message, severity, tooltip,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorToaster", "push_toast", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::push_toast_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn push_toast(&mut self, message: impl AsArg < GString >,) {
            self.push_toast_ex(message,) . done()
        }
        #[inline]
        pub fn push_toast_ex < 'a > (&'a mut self, message: impl AsArg < GString > + 'a,) -> ExPushToast < 'a > {
            ExPushToast::new(self, message,)
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
    impl crate::obj::GodotClass for EditorToaster {
        type Base = crate::classes::HBoxContainer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorToaster"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorToaster {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::HBoxContainer > for EditorToaster {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::BoxContainer > for EditorToaster {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Container > for EditorToaster {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for EditorToaster {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for EditorToaster {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for EditorToaster {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorToaster {
        
    }
    impl std::ops::Deref for EditorToaster {
        type Target = crate::classes::HBoxContainer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorToaster {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorToaster__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `EditorToaster` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`EditorToaster::push_toast_ex`][super::EditorToaster::push_toast_ex]."]
#[must_use]
pub struct ExPushToast < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorToaster, message: CowArg < 'a, GString >, severity: crate::classes::editor_toaster::Severity, tooltip: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPushToast < 'a > {
    fn new(surround_object: &'a mut re_export::EditorToaster, message: impl AsArg < GString > + 'a,) -> Self {
        let severity = crate::obj::EngineEnum::from_ord(0);
        let tooltip = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), severity: severity, tooltip: CowArg::Owned(tooltip),
        }
    }
    #[inline]
    pub fn severity(self, severity: crate::classes::editor_toaster::Severity) -> Self {
        Self {
            severity: severity, .. self
        }
    }
    #[inline]
    pub fn tooltip(self, tooltip: impl AsArg < GString > + 'a) -> Self {
        Self {
            tooltip: tooltip.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, message, severity, tooltip,
        }
        = self;
        re_export::EditorToaster::push_toast_full(surround_object, message, severity, tooltip,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Severity {
    ord: i32
}
impl Severity {
    #[doc(alias = "SEVERITY_INFO")]
    #[doc = "Godot enumerator name: `SEVERITY_INFO`"]
    pub const INFO: Severity = Severity {
        ord: 0i32
    };
    #[doc(alias = "SEVERITY_WARNING")]
    #[doc = "Godot enumerator name: `SEVERITY_WARNING`"]
    pub const WARNING: Severity = Severity {
        ord: 1i32
    };
    #[doc(alias = "SEVERITY_ERROR")]
    #[doc = "Godot enumerator name: `SEVERITY_ERROR`"]
    pub const ERROR: Severity = Severity {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Severity") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Severity {
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
            Self::INFO => "INFO", Self::WARNING => "WARNING", Self::ERROR => "ERROR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Severity::INFO, Severity::WARNING, Severity::ERROR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Severity >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INFO", "SEVERITY_INFO", Severity::INFO), crate::meta::inspect::EnumConstant::new("WARNING", "SEVERITY_WARNING", Severity::WARNING), crate::meta::inspect::EnumConstant::new("ERROR", "SEVERITY_ERROR", Severity::ERROR)]
        }
    }
}
impl crate::meta::GodotConvert for Severity {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Severity {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Severity {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorToaster;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::container::SignalsOfContainer;
    impl WithSignals for EditorToaster {
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