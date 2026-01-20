#![doc = "Sidecar module for class [`Joint2D`][crate::classes::Joint2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Joint2D` enums](https://docs.godotengine.org/en/stable/classes/class_joint2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Joint2D.`\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `Joint2D`](https://docs.godotengine.org/en/stable/classes/class_joint2d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Joint2D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Joint2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Joint2D {
        pub fn set_node_a(&mut self, node: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint2D", "set_node_a", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_a(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint2D", "get_node_a", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_node_b(&mut self, node: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint2D", "set_node_b", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_b(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint2D", "get_node_b", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bias(&mut self, bias: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint2D", "set_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bias(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint2D", "get_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_exclude_nodes_from_collision(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4702usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint2D", "set_exclude_nodes_from_collision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_exclude_nodes_from_collision(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4703usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint2D", "get_exclude_nodes_from_collision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rid(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint2D", "get_rid", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Joint2D {
        type Base = crate::classes::Node2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Joint2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Joint2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for Joint2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Joint2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Joint2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Joint2D {
        
    }
    impl std::ops::Deref for Joint2D {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Joint2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Joint2D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Joint2D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Joint2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::canvas_item::SignalsOfCanvasItem;
    impl WithSignals for Joint2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCanvasItem < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}