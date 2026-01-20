#![doc = "Sidecar module for class [`PhysicsBody2D`][crate::classes::PhysicsBody2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsBody2D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsbody2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsBody2D.`\n\nInherits [`CollisionObject2D`][crate::classes::CollisionObject2D].\n\nRelated symbols:\n\n* [`physics_body_2d`][crate::classes::physics_body_2d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `PhysicsBody2D`](https://docs.godotengine.org/en/stable/classes/class_physicsbody2d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<PhysicsBody2D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsBody2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl PhysicsBody2D {
        pub(crate) fn move_and_collide_full(&mut self, motion: Vector2, test_only: bool, safe_margin: f32, recovery_as_collision: bool,) -> Option < Gd < crate::classes::KinematicCollision2D > > {
            type CallRet = Option < Gd < crate::classes::KinematicCollision2D > >;
            type CallParams = (Vector2, bool, f32, bool,);
            let args = (motion, test_only, safe_margin, recovery_as_collision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6483usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "move_and_collide", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::move_and_collide_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn move_and_collide(&mut self, motion: Vector2,) -> Option < Gd < crate::classes::KinematicCollision2D > > {
            self.move_and_collide_ex(motion,) . done()
        }
        #[inline]
        pub fn move_and_collide_ex < 'a > (&'a mut self, motion: Vector2,) -> ExMoveAndCollide < 'a > {
            ExMoveAndCollide::new(self, motion,)
        }
        pub(crate) fn test_move_full(&mut self, from: Transform2D, motion: Vector2, collision: CowArg < Option < Gd < crate::classes::KinematicCollision2D > > >, safe_margin: f32, recovery_as_collision: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (Transform2D, Vector2, CowArg < 'a0, Option < Gd < crate::classes::KinematicCollision2D > > >, f32, bool,);
            let args = (from, motion, collision, safe_margin, recovery_as_collision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6484usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "test_move", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::test_move_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn test_move(&mut self, from: Transform2D, motion: Vector2,) -> bool {
            self.test_move_ex(from, motion,) . done()
        }
        #[inline]
        pub fn test_move_ex < 'a > (&'a mut self, from: Transform2D, motion: Vector2,) -> ExTestMove < 'a > {
            ExTestMove::new(self, from, motion,)
        }
        pub fn get_gravity(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6485usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "get_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_exceptions(&mut self,) -> Array < Gd < crate::classes::PhysicsBody2D > > {
            type CallRet = Array < Gd < crate::classes::PhysicsBody2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6486usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "get_collision_exceptions", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_collision_exception_with(&mut self, body: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (body.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6487usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "add_collision_exception_with", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_collision_exception_with(&mut self, body: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (body.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6488usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsBody2D", "remove_collision_exception_with", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PhysicsBody2D {
        type Base = crate::classes::CollisionObject2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PhysicsBody2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsBody2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CollisionObject2D > for PhysicsBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for PhysicsBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for PhysicsBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for PhysicsBody2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsBody2D {
        
    }
    impl std::ops::Deref for PhysicsBody2D {
        type Target = crate::classes::CollisionObject2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsBody2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PhysicsBody2D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `PhysicsBody2D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`PhysicsBody2D::move_and_collide_ex`][super::PhysicsBody2D::move_and_collide_ex]."]
#[must_use]
pub struct ExMoveAndCollide < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsBody2D, motion: Vector2, test_only: bool, safe_margin: f32, recovery_as_collision: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMoveAndCollide < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsBody2D, motion: Vector2,) -> Self {
        let test_only = false;
        let safe_margin = 0.08f32;
        let recovery_as_collision = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, motion: motion, test_only: test_only, safe_margin: safe_margin, recovery_as_collision: recovery_as_collision,
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
    pub fn done(self) -> Option < Gd < crate::classes::KinematicCollision2D > > {
        let Self {
            _phantom, surround_object, motion, test_only, safe_margin, recovery_as_collision,
        }
        = self;
        re_export::PhysicsBody2D::move_and_collide_full(surround_object, motion, test_only, safe_margin, recovery_as_collision,)
    }
}
#[doc = "Default-param extender for [`PhysicsBody2D::test_move_ex`][super::PhysicsBody2D::test_move_ex]."]
#[must_use]
pub struct ExTestMove < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsBody2D, from: Transform2D, motion: Vector2, collision: CowArg < 'a, Option < Gd < crate::classes::KinematicCollision2D > > >, safe_margin: f32, recovery_as_collision: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTestMove < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsBody2D, from: Transform2D, motion: Vector2,) -> Self {
        let collision = Gd::null_arg();
        let safe_margin = 0.08f32;
        let recovery_as_collision = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from, motion: motion, collision: collision.into_arg(), safe_margin: safe_margin, recovery_as_collision: recovery_as_collision,
        }
    }
    #[inline]
    pub fn collision(self, collision: impl AsArg < Option < Gd < crate::classes::KinematicCollision2D >> > + 'a) -> Self {
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
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, from, motion, collision, safe_margin, recovery_as_collision,
        }
        = self;
        re_export::PhysicsBody2D::test_move_full(surround_object, from, motion, collision, safe_margin, recovery_as_collision,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PhysicsBody2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::collision_object_2d::SignalsOfCollisionObject2D;
    impl WithSignals for PhysicsBody2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCollisionObject2D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}