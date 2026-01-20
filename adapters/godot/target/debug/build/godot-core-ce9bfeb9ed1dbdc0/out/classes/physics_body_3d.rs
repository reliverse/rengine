#![doc = "Sidecar module for class [`PhysicsBody3D`][crate::classes::PhysicsBody3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsBody3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsbody3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsBody3D.`\n\nInherits [`CollisionObject3D`][crate::classes::CollisionObject3D].\n\nRelated symbols:\n\n* [`physics_body_3d`][crate::classes::physics_body_3d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `PhysicsBody3D`](https://docs.godotengine.org/en/stable/classes/class_physicsbody3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<PhysicsBody3D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsBody3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl PhysicsBody3D {
        pub(crate) fn move_and_collide_full(&mut self, motion: Vector3, test_only: bool, safe_margin: f32, recovery_as_collision: bool, max_collisions: i32,) -> Option < Gd < crate::classes::KinematicCollision3D > > {
            type CallRet = Option < Gd < crate::classes::KinematicCollision3D > >;
            type CallParams = (Vector3, bool, f32, bool, i32,);
            let args = (motion, test_only, safe_margin, recovery_as_collision, max_collisions,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6489usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody3D", "move_and_collide", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::move_and_collide_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn move_and_collide(&mut self, motion: Vector3,) -> Option < Gd < crate::classes::KinematicCollision3D > > {
            self.move_and_collide_ex(motion,) . done()
        }
        #[inline]
        pub fn move_and_collide_ex < 'a > (&'a mut self, motion: Vector3,) -> ExMoveAndCollide < 'a > {
            ExMoveAndCollide::new(self, motion,)
        }
        pub(crate) fn test_move_full(&mut self, from: Transform3D, motion: Vector3, collision: CowArg < Option < Gd < crate::classes::KinematicCollision3D > > >, safe_margin: f32, recovery_as_collision: bool, max_collisions: i32,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Transform3D, Vector3, CowArg < 'a0, Option < Gd < crate::classes::KinematicCollision3D > > >, f32, bool, i32,);
            let args = (from, motion, collision, safe_margin, recovery_as_collision, max_collisions,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6490usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody3D", "test_move", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::test_move_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn test_move(&mut self, from: Transform3D, motion: Vector3,) -> bool {
            self.test_move_ex(from, motion,) . done()
        }
        #[inline]
        pub fn test_move_ex < 'a > (&'a mut self, from: Transform3D, motion: Vector3,) -> ExTestMove < 'a > {
            ExTestMove::new(self, from, motion,)
        }
        pub fn get_gravity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6491usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody3D", "get_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_axis_lock(&mut self, axis: crate::classes::physics_server_3d::BodyAxis, lock: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::physics_server_3d::BodyAxis, bool,);
            let args = (axis, lock,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6492usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody3D", "set_axis_lock", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_axis_lock(&self, axis: crate::classes::physics_server_3d::BodyAxis,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::physics_server_3d::BodyAxis,);
            let args = (axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6493usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody3D", "get_axis_lock", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_exceptions(&mut self,) -> Array < Gd < crate::classes::PhysicsBody3D > > {
            type CallRet = Array < Gd < crate::classes::PhysicsBody3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6494usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody3D", "get_collision_exceptions", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_collision_exception_with(&mut self, body: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (body.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6495usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody3D", "add_collision_exception_with", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_collision_exception_with(&mut self, body: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (body.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6496usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody3D", "remove_collision_exception_with", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PhysicsBody3D {
        type Base = crate::classes::CollisionObject3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PhysicsBody3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsBody3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject3D > for PhysicsBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for PhysicsBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for PhysicsBody3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsBody3D {
        
    }
    impl std::ops::Deref for PhysicsBody3D {
        type Target = crate::classes::CollisionObject3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsBody3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PhysicsBody3D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `PhysicsBody3D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`PhysicsBody3D::move_and_collide_ex`][super::PhysicsBody3D::move_and_collide_ex]."]
#[must_use]
pub struct ExMoveAndCollide < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsBody3D, motion: Vector3, test_only: bool, safe_margin: f32, recovery_as_collision: bool, max_collisions: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMoveAndCollide < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsBody3D, motion: Vector3,) -> Self {
        let test_only = false;
        let safe_margin = 0.001f32;
        let recovery_as_collision = false;
        let max_collisions = 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, motion: motion, test_only: test_only, safe_margin: safe_margin, recovery_as_collision: recovery_as_collision, max_collisions: max_collisions,
        }
    }
    #[inline]
    pub fn test_only(self, test_only: bool) -> Self {
        Self {
            test_only: test_only, .. self
        }
    }
    #[inline]
    pub fn safe_margin(self, safe_margin: f32) -> Self {
        Self {
            safe_margin: safe_margin, .. self
        }
    }
    #[inline]
    pub fn recovery_as_collision(self, recovery_as_collision: bool) -> Self {
        Self {
            recovery_as_collision: recovery_as_collision, .. self
        }
    }
    #[inline]
    pub fn max_collisions(self, max_collisions: i32) -> Self {
        Self {
            max_collisions: max_collisions, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::KinematicCollision3D > > {
        let Self {
            _phantom, surround_object, motion, test_only, safe_margin, recovery_as_collision, max_collisions,
        }
        = self;
        re_export::PhysicsBody3D::move_and_collide_full(surround_object, motion, test_only, safe_margin, recovery_as_collision, max_collisions,)
    }
}
#[doc = "Default-param extender for [`PhysicsBody3D::test_move_ex`][super::PhysicsBody3D::test_move_ex]."]
#[must_use]
pub struct ExTestMove < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsBody3D, from: Transform3D, motion: Vector3, collision: CowArg < 'a, Option < Gd < crate::classes::KinematicCollision3D > > >, safe_margin: f32, recovery_as_collision: bool, max_collisions: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTestMove < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsBody3D, from: Transform3D, motion: Vector3,) -> Self {
        let collision = Gd::null_arg();
        let safe_margin = 0.001f32;
        let recovery_as_collision = false;
        let max_collisions = 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from, motion: motion, collision: collision.into_arg(), safe_margin: safe_margin, recovery_as_collision: recovery_as_collision, max_collisions: max_collisions,
        }
    }
    #[inline]
    pub fn collision(self, collision: impl AsArg < Option < Gd < crate::classes::KinematicCollision3D >> > + 'a) -> Self {
        Self {
            collision: collision.into_arg(), .. self
        }
    }
    #[inline]
    pub fn safe_margin(self, safe_margin: f32) -> Self {
        Self {
            safe_margin: safe_margin, .. self
        }
    }
    #[inline]
    pub fn recovery_as_collision(self, recovery_as_collision: bool) -> Self {
        Self {
            recovery_as_collision: recovery_as_collision, .. self
        }
    }
    #[inline]
    pub fn max_collisions(self, max_collisions: i32) -> Self {
        Self {
            max_collisions: max_collisions, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, from, motion, collision, safe_margin, recovery_as_collision, max_collisions,
        }
        = self;
        re_export::PhysicsBody3D::test_move_full(surround_object, from, motion, collision, safe_margin, recovery_as_collision, max_collisions,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PhysicsBody3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::collision_object_3d::SignalsOfCollisionObject3D;
    impl WithSignals for PhysicsBody3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCollisionObject3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}