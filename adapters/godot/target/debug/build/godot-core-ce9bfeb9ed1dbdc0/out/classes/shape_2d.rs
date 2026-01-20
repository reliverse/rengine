#![doc = "Sidecar module for class [`Shape2D`][crate::classes::Shape2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Shape2D` enums](https://docs.godotengine.org/en/stable/classes/class_shape2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Shape2D.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n\n\nSee also [Godot docs for `Shape2D`](https://docs.godotengine.org/en/stable/classes/class_shape2d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Shape2D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Shape2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Shape2D {
        pub fn set_custom_solver_bias(&mut self, bias: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7978usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Shape2D", "set_custom_solver_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_solver_bias(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7979usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Shape2D", "get_custom_solver_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn collide(&mut self, local_xform: Transform2D, with_shape: impl AsArg < Option < Gd < crate::classes::Shape2D >> >, shape_xform: Transform2D,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Transform2D, CowArg < 'a0, Option < Gd < crate::classes::Shape2D > > >, Transform2D,);
            let args = (local_xform, with_shape.into_arg(), shape_xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7980usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Shape2D", "collide", Some(self.__validated_obj()), args,)
            }
        }
        pub fn collide_with_motion(&mut self, local_xform: Transform2D, local_motion: Vector2, with_shape: impl AsArg < Option < Gd < crate::classes::Shape2D >> >, shape_xform: Transform2D, shape_motion: Vector2,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Transform2D, Vector2, CowArg < 'a0, Option < Gd < crate::classes::Shape2D > > >, Transform2D, Vector2,);
            let args = (local_xform, local_motion, with_shape.into_arg(), shape_xform, shape_motion,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7981usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Shape2D", "collide_with_motion", Some(self.__validated_obj()), args,)
            }
        }
        pub fn collide_and_get_contacts(&mut self, local_xform: Transform2D, with_shape: impl AsArg < Option < Gd < crate::classes::Shape2D >> >, shape_xform: Transform2D,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams < 'a0, > = (Transform2D, CowArg < 'a0, Option < Gd < crate::classes::Shape2D > > >, Transform2D,);
            let args = (local_xform, with_shape.into_arg(), shape_xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7982usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Shape2D", "collide_and_get_contacts", Some(self.__validated_obj()), args,)
            }
        }
        pub fn collide_with_motion_and_get_contacts(&mut self, local_xform: Transform2D, local_motion: Vector2, with_shape: impl AsArg < Option < Gd < crate::classes::Shape2D >> >, shape_xform: Transform2D, shape_motion: Vector2,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams < 'a0, > = (Transform2D, Vector2, CowArg < 'a0, Option < Gd < crate::classes::Shape2D > > >, Transform2D, Vector2,);
            let args = (local_xform, local_motion, with_shape.into_arg(), shape_xform, shape_motion,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7983usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Shape2D", "collide_with_motion_and_get_contacts", Some(self.__validated_obj()), args,)
            }
        }
        pub fn draw(&mut self, canvas_item: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (canvas_item, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7984usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Shape2D", "draw", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rect(&self,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7985usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Shape2D", "get_rect", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Shape2D {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Shape2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Shape2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Shape2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Shape2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Shape2D {
        
    }
    impl std::ops::Deref for Shape2D {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Shape2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Shape2D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Shape2D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Shape2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Shape2D {
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