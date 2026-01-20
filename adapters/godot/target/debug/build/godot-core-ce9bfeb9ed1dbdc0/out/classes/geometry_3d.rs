#![doc = "Sidecar module for class [`Geometry3D`][crate::classes::Geometry3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Geometry3D` enums](https://docs.godotengine.org/en/stable/classes/class_geometry3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Geometry3D.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`geometry_3d`][crate::classes::geometry_3d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `Geometry3D`](https://docs.godotengine.org/en/stable/classes/class_geometry3d.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Geometry3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Geometry3D {
        pub fn compute_convex_mesh_points(&mut self, planes: &Array < Plane >,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Plane > >,);
            let args = (RefArg::new(planes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4033usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "compute_convex_mesh_points", Some(self.__validated_obj()), args,)
            }
        }
        pub fn build_box_planes(&mut self, extents: Vector3,) -> Array < Plane > {
            type CallRet = Array < Plane >;
            type CallParams = (Vector3,);
            let args = (extents,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4034usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "build_box_planes", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn build_cylinder_planes_full(&mut self, radius: f32, height: f32, sides: i32, axis: Vector3Axis,) -> Array < Plane > {
            type CallRet = Array < Plane >;
            type CallParams = (f32, f32, i32, Vector3Axis,);
            let args = (radius, height, sides, axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4035usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "build_cylinder_planes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::build_cylinder_planes_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn build_cylinder_planes(&mut self, radius: f32, height: f32, sides: i32,) -> Array < Plane > {
            self.build_cylinder_planes_ex(radius, height, sides,) . done()
        }
        #[inline]
        pub fn build_cylinder_planes_ex < 'a > (&'a mut self, radius: f32, height: f32, sides: i32,) -> ExBuildCylinderPlanes < 'a > {
            ExBuildCylinderPlanes::new(self, radius, height, sides,)
        }
        pub(crate) fn build_capsule_planes_full(&mut self, radius: f32, height: f32, sides: i32, lats: i32, axis: Vector3Axis,) -> Array < Plane > {
            type CallRet = Array < Plane >;
            type CallParams = (f32, f32, i32, i32, Vector3Axis,);
            let args = (radius, height, sides, lats, axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4036usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "build_capsule_planes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::build_capsule_planes_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn build_capsule_planes(&mut self, radius: f32, height: f32, sides: i32, lats: i32,) -> Array < Plane > {
            self.build_capsule_planes_ex(radius, height, sides, lats,) . done()
        }
        #[inline]
        pub fn build_capsule_planes_ex < 'a > (&'a mut self, radius: f32, height: f32, sides: i32, lats: i32,) -> ExBuildCapsulePlanes < 'a > {
            ExBuildCapsulePlanes::new(self, radius, height, sides, lats,)
        }
        pub fn get_closest_points_between_segments(&mut self, p1: Vector3, p2: Vector3, q1: Vector3, q2: Vector3,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams = (Vector3, Vector3, Vector3, Vector3,);
            let args = (p1, p2, q1, q2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4037usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "get_closest_points_between_segments", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_closest_point_to_segment(&mut self, point: Vector3, s1: Vector3, s2: Vector3,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector3, Vector3, Vector3,);
            let args = (point, s1, s2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4038usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "get_closest_point_to_segment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_closest_point_to_segment_uncapped(&mut self, point: Vector3, s1: Vector3, s2: Vector3,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector3, Vector3, Vector3,);
            let args = (point, s1, s2,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4039usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "get_closest_point_to_segment_uncapped", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_triangle_barycentric_coords(&mut self, point: Vector3, a: Vector3, b: Vector3, c: Vector3,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector3, Vector3, Vector3, Vector3,);
            let args = (point, a, b, c,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4040usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "get_triangle_barycentric_coords", Some(self.__validated_obj()), args,)
            }
        }
        pub fn ray_intersects_triangle(&mut self, from: Vector3, dir: Vector3, a: Vector3, b: Vector3, c: Vector3,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Vector3, Vector3, Vector3, Vector3, Vector3,);
            let args = (from, dir, a, b, c,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4041usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "ray_intersects_triangle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn segment_intersects_triangle(&mut self, from: Vector3, to: Vector3, a: Vector3, b: Vector3, c: Vector3,) -> Variant {
            type CallRet = Variant;
            type CallParams = (Vector3, Vector3, Vector3, Vector3, Vector3,);
            let args = (from, to, a, b, c,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4042usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "segment_intersects_triangle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn segment_intersects_sphere(&mut self, from: Vector3, to: Vector3, sphere_position: Vector3, sphere_radius: f32,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams = (Vector3, Vector3, Vector3, f32,);
            let args = (from, to, sphere_position, sphere_radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4043usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "segment_intersects_sphere", Some(self.__validated_obj()), args,)
            }
        }
        pub fn segment_intersects_cylinder(&mut self, from: Vector3, to: Vector3, height: f32, radius: f32,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams = (Vector3, Vector3, f32, f32,);
            let args = (from, to, height, radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4044usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "segment_intersects_cylinder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn segment_intersects_convex(&mut self, from: Vector3, to: Vector3, planes: &Array < Plane >,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams < 'a0, > = (Vector3, Vector3, RefArg < 'a0, Array < Plane > >,);
            let args = (from, to, RefArg::new(planes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4045usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "segment_intersects_convex", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clip_polygon(&mut self, points: &PackedVector3Array, plane: Plane,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector3Array >, Plane,);
            let args = (RefArg::new(points), plane,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4046usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "clip_polygon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tetrahedralize_delaunay(&mut self, points: &PackedVector3Array,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector3Array >,);
            let args = (RefArg::new(points),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4047usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Geometry3D", "tetrahedralize_delaunay", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Geometry3D {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Geometry3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Geometry3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Geometry3D {
        
    }
    impl crate::obj::Singleton for Geometry3D {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"Geometry3D"))
            }
        }
    }
    impl std::ops::Deref for Geometry3D {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Geometry3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Geometry3D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Geometry3D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Geometry3D::build_cylinder_planes_ex`][super::Geometry3D::build_cylinder_planes_ex]."]
#[must_use]
pub struct ExBuildCylinderPlanes < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Geometry3D, radius: f32, height: f32, sides: i32, axis: Vector3Axis,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBuildCylinderPlanes < 'a > {
    fn new(surround_object: &'a mut re_export::Geometry3D, radius: f32, height: f32, sides: i32,) -> Self {
        let axis = crate::obj::EngineEnum::from_ord(2);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, radius: radius, height: height, sides: sides, axis: axis,
        }
    }
    #[inline]
    pub fn axis(self, axis: Vector3Axis) -> Self {
        Self {
            axis: axis, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Plane > {
        let Self {
            _phantom, surround_object, radius, height, sides, axis,
        }
        = self;
        re_export::Geometry3D::build_cylinder_planes_full(surround_object, radius, height, sides, axis,)
    }
}
#[doc = "Default-param extender for [`Geometry3D::build_capsule_planes_ex`][super::Geometry3D::build_capsule_planes_ex]."]
#[must_use]
pub struct ExBuildCapsulePlanes < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Geometry3D, radius: f32, height: f32, sides: i32, lats: i32, axis: Vector3Axis,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBuildCapsulePlanes < 'a > {
    fn new(surround_object: &'a mut re_export::Geometry3D, radius: f32, height: f32, sides: i32, lats: i32,) -> Self {
        let axis = crate::obj::EngineEnum::from_ord(2);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, radius: radius, height: height, sides: sides, lats: lats, axis: axis,
        }
    }
    #[inline]
    pub fn axis(self, axis: Vector3Axis) -> Self {
        Self {
            axis: axis, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Plane > {
        let Self {
            _phantom, surround_object, radius, height, sides, lats, axis,
        }
        = self;
        re_export::Geometry3D::build_capsule_planes_full(surround_object, radius, height, sides, lats, axis,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Geometry3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Geometry3D {
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