#![doc = "Sidecar module for class [`PhysicsDirectSpaceState2D`][crate::classes::PhysicsDirectSpaceState2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsDirectSpaceState2D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsdirectspacestate2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsDirectSpaceState2D.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`physics_direct_space_state_2d`][crate::classes::physics_direct_space_state_2d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `PhysicsDirectSpaceState2D`](https://docs.godotengine.org/en/stable/classes/class_physicsdirectspacestate2d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<PhysicsDirectSpaceState2D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsDirectSpaceState2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl PhysicsDirectSpaceState2D {
        pub(crate) fn intersect_point_full(&mut self, parameters: CowArg < Option < Gd < crate::classes::PhysicsPointQueryParameters2D > > >, max_results: i32,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::PhysicsPointQueryParameters2D > > >, i32,);
            let args = (parameters, max_results,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(426usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState2D", "intersect_point", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::intersect_point_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn intersect_point(&mut self, parameters: impl AsArg < Option < Gd < crate::classes::PhysicsPointQueryParameters2D >> >,) -> Array < VarDictionary > {
            self.intersect_point_ex(parameters,) . done()
        }
        #[inline]
        pub fn intersect_point_ex < 'a > (&'a mut self, parameters: impl AsArg < Option < Gd < crate::classes::PhysicsPointQueryParameters2D >> > + 'a,) -> ExIntersectPoint < 'a > {
            ExIntersectPoint::new(self, parameters,)
        }
        pub fn intersect_ray(&mut self, parameters: impl AsArg < Option < Gd < crate::classes::PhysicsRayQueryParameters2D >> >,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::PhysicsRayQueryParameters2D > > >,);
            let args = (parameters.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(427usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState2D", "intersect_ray", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn intersect_shape_full(&mut self, parameters: CowArg < Option < Gd < crate::classes::PhysicsShapeQueryParameters2D > > >, max_results: i32,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::PhysicsShapeQueryParameters2D > > >, i32,);
            let args = (parameters, max_results,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(428usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState2D", "intersect_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::intersect_shape_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn intersect_shape(&mut self, parameters: impl AsArg < Option < Gd < crate::classes::PhysicsShapeQueryParameters2D >> >,) -> Array < VarDictionary > {
            self.intersect_shape_ex(parameters,) . done()
        }
        #[inline]
        pub fn intersect_shape_ex < 'a > (&'a mut self, parameters: impl AsArg < Option < Gd < crate::classes::PhysicsShapeQueryParameters2D >> > + 'a,) -> ExIntersectShape < 'a > {
            ExIntersectShape::new(self, parameters,)
        }
        pub fn cast_motion(&mut self, parameters: impl AsArg < Option < Gd < crate::classes::PhysicsShapeQueryParameters2D >> >,) -> PackedFloat32Array {
            type CallRet = PackedFloat32Array;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::PhysicsShapeQueryParameters2D > > >,);
            let args = (parameters.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(429usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState2D", "cast_motion", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn collide_shape_full(&mut self, parameters: CowArg < Option < Gd < crate::classes::PhysicsShapeQueryParameters2D > > >, max_results: i32,) -> Array < Vector2 > {
            type CallRet = Array < Vector2 >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::PhysicsShapeQueryParameters2D > > >, i32,);
            let args = (parameters, max_results,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(430usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState2D", "collide_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::collide_shape_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn collide_shape(&mut self, parameters: impl AsArg < Option < Gd < crate::classes::PhysicsShapeQueryParameters2D >> >,) -> Array < Vector2 > {
            self.collide_shape_ex(parameters,) . done()
        }
        #[inline]
        pub fn collide_shape_ex < 'a > (&'a mut self, parameters: impl AsArg < Option < Gd < crate::classes::PhysicsShapeQueryParameters2D >> > + 'a,) -> ExCollideShape < 'a > {
            ExCollideShape::new(self, parameters,)
        }
        pub fn get_rest_info(&mut self, parameters: impl AsArg < Option < Gd < crate::classes::PhysicsShapeQueryParameters2D >> >,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::PhysicsShapeQueryParameters2D > > >,);
            let args = (parameters.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(431usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectSpaceState2D", "get_rest_info", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PhysicsDirectSpaceState2D {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PhysicsDirectSpaceState2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsDirectSpaceState2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsDirectSpaceState2D {
        
    }
    impl std::ops::Deref for PhysicsDirectSpaceState2D {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsDirectSpaceState2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PhysicsDirectSpaceState2D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `PhysicsDirectSpaceState2D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`PhysicsDirectSpaceState2D::intersect_point_ex`][super::PhysicsDirectSpaceState2D::intersect_point_ex]."]
#[must_use]
pub struct ExIntersectPoint < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: CowArg < 'a, Option < Gd < crate::classes::PhysicsPointQueryParameters2D > > >, max_results: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIntersectPoint < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: impl AsArg < Option < Gd < crate::classes::PhysicsPointQueryParameters2D >> > + 'a,) -> Self {
        let max_results = 32i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parameters: parameters.into_arg(), max_results: max_results,
        }
    }
    #[inline]
    pub fn max_results(self, max_results: i32) -> Self {
        Self {
            max_results: max_results, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < VarDictionary > {
        let Self {
            _phantom, surround_object, parameters, max_results,
        }
        = self;
        re_export::PhysicsDirectSpaceState2D::intersect_point_full(surround_object, parameters, max_results,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectSpaceState2D::intersect_shape_ex`][super::PhysicsDirectSpaceState2D::intersect_shape_ex]."]
#[must_use]
pub struct ExIntersectShape < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: CowArg < 'a, Option < Gd < crate::classes::PhysicsShapeQueryParameters2D > > >, max_results: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIntersectShape < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: impl AsArg < Option < Gd < crate::classes::PhysicsShapeQueryParameters2D >> > + 'a,) -> Self {
        let max_results = 32i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parameters: parameters.into_arg(), max_results: max_results,
        }
    }
    #[inline]
    pub fn max_results(self, max_results: i32) -> Self {
        Self {
            max_results: max_results, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < VarDictionary > {
        let Self {
            _phantom, surround_object, parameters, max_results,
        }
        = self;
        re_export::PhysicsDirectSpaceState2D::intersect_shape_full(surround_object, parameters, max_results,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectSpaceState2D::collide_shape_ex`][super::PhysicsDirectSpaceState2D::collide_shape_ex]."]
#[must_use]
pub struct ExCollideShape < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: CowArg < 'a, Option < Gd < crate::classes::PhysicsShapeQueryParameters2D > > >, max_results: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCollideShape < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectSpaceState2D, parameters: impl AsArg < Option < Gd < crate::classes::PhysicsShapeQueryParameters2D >> > + 'a,) -> Self {
        let max_results = 32i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parameters: parameters.into_arg(), max_results: max_results,
        }
    }
    #[inline]
    pub fn max_results(self, max_results: i32) -> Self {
        Self {
            max_results: max_results, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Vector2 > {
        let Self {
            _phantom, surround_object, parameters, max_results,
        }
        = self;
        re_export::PhysicsDirectSpaceState2D::collide_shape_full(surround_object, parameters, max_results,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PhysicsDirectSpaceState2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for PhysicsDirectSpaceState2D {
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