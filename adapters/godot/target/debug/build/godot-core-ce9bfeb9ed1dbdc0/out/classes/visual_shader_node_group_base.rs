#![doc = "Sidecar module for class [`VisualShaderNodeGroupBase`][crate::classes::VisualShaderNodeGroupBase].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeGroupBase` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodegroupbase.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeGroupBase.`\n\nInherits [`VisualShaderNodeResizableBase`][crate::classes::VisualShaderNodeResizableBase].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `VisualShaderNodeGroupBase`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodegroupbase.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<VisualShaderNodeGroupBase>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeGroupBase {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl VisualShaderNodeGroupBase {
        pub fn set_inputs(&mut self, inputs: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (inputs.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "set_inputs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inputs(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10702usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "get_inputs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_outputs(&mut self, outputs: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (outputs.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10703usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "set_outputs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_outputs(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "get_outputs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_valid_port_name(&self, name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "is_valid_port_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_input_port(&mut self, id: i32, type_: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, GString >,);
            let args = (id, type_, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "add_input_port", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_input_port(&mut self, id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10707usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "remove_input_port", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_input_port_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10708usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "get_input_port_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_input_port(&self, id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10709usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "has_input_port", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_input_ports(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10710usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "clear_input_ports", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_output_port(&mut self, id: i32, type_: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, GString >,);
            let args = (id, type_, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10711usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "add_output_port", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_output_port(&mut self, id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10712usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "remove_output_port", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_output_port_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10713usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "get_output_port_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_output_port(&self, id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10714usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "has_output_port", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_output_ports(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10715usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "clear_output_ports", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_input_port_name(&mut self, id: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (id, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10716usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "set_input_port_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_input_port_type(&mut self, id: i32, type_: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (id, type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10717usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "set_input_port_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_output_port_name(&mut self, id: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (id, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10718usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "set_output_port_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_output_port_type(&mut self, id: i32, type_: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (id, type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10719usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "set_output_port_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_free_input_port_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10720usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "get_free_input_port_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_free_output_port_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10721usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeGroupBase", "get_free_output_port_id", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeGroupBase {
        type Base = crate::classes::VisualShaderNodeResizableBase;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VisualShaderNodeGroupBase"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeGroupBase {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNodeResizableBase > for VisualShaderNodeGroupBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeGroupBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeGroupBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeGroupBase {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeGroupBase {
        
    }
    impl std::ops::Deref for VisualShaderNodeGroupBase {
        type Target = crate::classes::VisualShaderNodeResizableBase;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeGroupBase {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisualShaderNodeGroupBase__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `VisualShaderNodeGroupBase` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VisualShaderNodeGroupBase;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VisualShaderNodeGroupBase {
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