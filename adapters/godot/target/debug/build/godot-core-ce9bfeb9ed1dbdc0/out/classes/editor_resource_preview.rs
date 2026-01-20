#![doc = "Sidecar module for class [`EditorResourcePreview`][crate::classes::EditorResourcePreview].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorResourcePreview` enums](https://docs.godotengine.org/en/stable/classes/class_editorresourcepreview.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorResourcePreview.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`editor_resource_preview`][crate::classes::editor_resource_preview]: sidecar module with related enum/flag types\n* [`SignalsOfEditorResourcePreview`][crate::classes::editor_resource_preview::SignalsOfEditorResourcePreview]: signal collection\n\n\nSee also [Godot docs for `EditorResourcePreview`](https://docs.godotengine.org/en/stable/classes/class_editorresourcepreview.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<EditorResourcePreview>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorResourcePreview {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl EditorResourcePreview {
        pub fn queue_resource_preview(&mut self, path: impl AsArg < GString >, receiver: impl AsArg < Option < Gd < crate::classes::Object >> >, receiver_func: impl AsArg < StringName >, userdata: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Object > > >, CowArg < 'a2, StringName >, RefArg < 'a3, Variant >,);
            let args = (path.into_arg(), receiver.into_arg(), receiver_func.into_arg(), RefArg::new(userdata),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(353usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorResourcePreview", "queue_resource_preview", Some(self.__validated_obj()), args,)
            }
        }
        pub fn queue_edited_resource_preview(&mut self, resource: impl AsArg < Option < Gd < crate::classes::Resource >> >, receiver: impl AsArg < Option < Gd < crate::classes::Object >> >, receiver_func: impl AsArg < StringName >, userdata: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, Option < Gd < crate::classes::Resource > > >, CowArg < 'a1, Option < Gd < crate::classes::Object > > >, CowArg < 'a2, StringName >, RefArg < 'a3, Variant >,);
            let args = (resource.into_arg(), receiver.into_arg(), receiver_func.into_arg(), RefArg::new(userdata),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(354usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorResourcePreview", "queue_edited_resource_preview", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_preview_generator(&mut self, generator: impl AsArg < Option < Gd < crate::classes::EditorResourcePreviewGenerator >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorResourcePreviewGenerator > > >,);
            let args = (generator.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(355usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorResourcePreview", "add_preview_generator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_preview_generator(&mut self, generator: impl AsArg < Option < Gd < crate::classes::EditorResourcePreviewGenerator >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::EditorResourcePreviewGenerator > > >,);
            let args = (generator.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(356usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorResourcePreview", "remove_preview_generator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn check_for_invalidation(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(357usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorResourcePreview", "check_for_invalidation", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EditorResourcePreview {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorResourcePreview"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorResourcePreview {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for EditorResourcePreview {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorResourcePreview {
        
    }
    impl std::ops::Deref for EditorResourcePreview {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorResourcePreview {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorResourcePreview__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `EditorResourcePreview` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorResourcePreview;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`EditorResourcePreview`][crate::classes::EditorResourcePreview] class."]
    pub struct SignalsOfEditorResourcePreview < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfEditorResourcePreview < 'c, C > {
        #[doc = "Signature: `(path: GString)`"]
        pub fn preview_invalidated(&mut self) -> SigPreviewInvalidated < 'c, C > {
            SigPreviewInvalidated {
                typed: TypedSignal::extract(&mut self.__internal_obj, "preview_invalidated")
            }
        }
    }
    type TypedSigPreviewInvalidated < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigPreviewInvalidated < 'c, C: WithSignals > {
        typed: TypedSigPreviewInvalidated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPreviewInvalidated < 'c, C > {
        pub fn emit(&mut self, path: GString,) {
            self.typed.emit_tuple((path,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPreviewInvalidated < 'c, C > {
        type Target = TypedSigPreviewInvalidated < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPreviewInvalidated < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for EditorResourcePreview {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfEditorResourcePreview < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfEditorResourcePreview < 'c, C > {
        type Target = < < EditorResourcePreview as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = EditorResourcePreview;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfEditorResourcePreview < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = EditorResourcePreview;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}