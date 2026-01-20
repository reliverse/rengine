#![doc = "Sidecar module for class [`Joint3D`][crate::classes::Joint3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Joint3D` enums](https://docs.godotengine.org/en/stable/classes/class_joint3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Joint3D.`\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `Joint3D`](https://docs.godotengine.org/en/stable/classes/class_joint3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Joint3D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Joint3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Joint3D {
        pub fn set_node_a(&mut self, node: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint3D", "set_node_a", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_a(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint3D", "get_node_a", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_node_b(&mut self, node: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4707usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint3D", "set_node_b", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_b(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4708usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint3D", "get_node_b", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_solver_priority(&mut self, priority: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4709usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint3D", "set_solver_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_solver_priority(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4710usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint3D", "get_solver_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_exclude_nodes_from_collision(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4711usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint3D", "set_exclude_nodes_from_collision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_exclude_nodes_from_collision(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4712usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint3D", "get_exclude_nodes_from_collision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rid(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4713usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Joint3D", "get_rid", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Joint3D {
        type Base = crate::classes::Node3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Joint3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Joint3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for Joint3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Joint3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Joint3D {
        
    }
    impl std::ops::Deref for Joint3D {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Joint3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Joint3D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Joint3D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Joint3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for Joint3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}