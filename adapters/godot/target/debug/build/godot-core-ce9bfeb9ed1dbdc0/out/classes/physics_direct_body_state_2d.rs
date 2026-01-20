#![doc = "Sidecar module for class [`PhysicsDirectBodyState2D`][crate::classes::PhysicsDirectBodyState2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsDirectBodyState2D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsDirectBodyState2D.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`physics_direct_body_state_2d`][crate::classes::physics_direct_body_state_2d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `PhysicsDirectBodyState2D`](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate2d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<PhysicsDirectBodyState2D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsDirectBodyState2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl PhysicsDirectBodyState2D {
        pub fn get_total_gravity(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_total_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_total_linear_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_total_linear_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_total_angular_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_total_angular_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_of_mass(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_center_of_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_of_mass_local(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_center_of_mass_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inverse_mass(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_inverse_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inverse_inertia(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_inverse_inertia", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_velocity(&mut self, velocity: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "set_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_velocity(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_velocity(&mut self, velocity: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "set_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_velocity(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transform(&mut self, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Transform2D,);
            let args = (transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_velocity_at_local_position(&self, local_position: Vector2,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector2,);
            let args = (local_position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_velocity_at_local_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn apply_central_impulse(&mut self, impulse: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(342usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "apply_central_impulse", Some(self.__validated_obj()), args,)
            }
        }
        pub fn apply_torque_impulse(&mut self, impulse: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(343usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "apply_torque_impulse", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn apply_impulse_full(&mut self, impulse: Vector2, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2,);
            let args = (impulse, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(344usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "apply_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::apply_impulse_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn apply_impulse(&mut self, impulse: Vector2,) {
            self.apply_impulse_ex(impulse,) . done()
        }
        #[inline]
        pub fn apply_impulse_ex < 'a > (&'a mut self, impulse: Vector2,) -> ExApplyImpulse < 'a > {
            ExApplyImpulse::new(self, impulse,)
        }
        pub(crate) fn apply_central_force_full(&mut self, force: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(345usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "apply_central_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::apply_central_force_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn apply_central_force(&mut self,) {
            self.apply_central_force_ex() . done()
        }
        #[inline]
        pub fn apply_central_force_ex < 'a > (&'a mut self,) -> ExApplyCentralForce < 'a > {
            ExApplyCentralForce::new(self,)
        }
        pub(crate) fn apply_force_full(&mut self, force: Vector2, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2,);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(346usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "apply_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::apply_force_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn apply_force(&mut self, force: Vector2,) {
            self.apply_force_ex(force,) . done()
        }
        #[inline]
        pub fn apply_force_ex < 'a > (&'a mut self, force: Vector2,) -> ExApplyForce < 'a > {
            ExApplyForce::new(self, force,)
        }
        pub fn apply_torque(&mut self, torque: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(347usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "apply_torque", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_constant_central_force_full(&mut self, force: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(348usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "add_constant_central_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_constant_central_force_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_constant_central_force(&mut self,) {
            self.add_constant_central_force_ex() . done()
        }
        #[inline]
        pub fn add_constant_central_force_ex < 'a > (&'a mut self,) -> ExAddConstantCentralForce < 'a > {
            ExAddConstantCentralForce::new(self,)
        }
        pub(crate) fn add_constant_force_full(&mut self, force: Vector2, position: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2,);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(349usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "add_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_constant_force_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_constant_force(&mut self, force: Vector2,) {
            self.add_constant_force_ex(force,) . done()
        }
        #[inline]
        pub fn add_constant_force_ex < 'a > (&'a mut self, force: Vector2,) -> ExAddConstantForce < 'a > {
            ExAddConstantForce::new(self, force,)
        }
        pub fn add_constant_torque(&mut self, torque: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(350usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "add_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_constant_force(&mut self, force: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(351usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "set_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_constant_force(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(352usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_constant_torque(&mut self, torque: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(353usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "set_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_constant_torque(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(354usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sleep_state(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(355usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "set_sleep_state", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_sleeping(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(356usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "is_sleeping", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_layer(&mut self, layer: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(357usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "set_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_layer(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(358usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_mask(&mut self, mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "set_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(361usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_local_position(&self, contact_idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(362usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_local_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_local_normal(&self, contact_idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(363usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_local_normal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_local_shape(&self, contact_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(364usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_local_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_local_velocity_at_position(&self, contact_idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(365usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_local_velocity_at_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_collider(&self, contact_idx: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(366usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_collider", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_collider_position(&self, contact_idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(367usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_collider_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_collider_id(&self, contact_idx: i32,) -> u64 {
            type CallRet = u64;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(368usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_collider_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_collider_object(&self, contact_idx: i32,) -> Option < Gd < crate::classes::Object > > {
            type CallRet = Option < Gd < crate::classes::Object > >;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(369usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_collider_object", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_collider_shape(&self, contact_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(370usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_collider_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_collider_velocity_at_position(&self, contact_idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(371usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_collider_velocity_at_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_impulse(&self, contact_idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(372usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_contact_impulse", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_step(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(373usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_step", Some(self.__validated_obj()), args,)
            }
        }
        pub fn integrate_forces(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(374usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "integrate_forces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_space_state(&mut self,) -> Option < Gd < crate::classes::PhysicsDirectSpaceState2D > > {
            type CallRet = Option < Gd < crate::classes::PhysicsDirectSpaceState2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(375usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState2D", "get_space_state", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PhysicsDirectBodyState2D {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PhysicsDirectBodyState2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsDirectBodyState2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsDirectBodyState2D {
        
    }
    impl std::ops::Deref for PhysicsDirectBodyState2D {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsDirectBodyState2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PhysicsDirectBodyState2D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `PhysicsDirectBodyState2D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState2D::apply_impulse_ex`][super::PhysicsDirectBodyState2D::apply_impulse_ex]."]
#[must_use]
pub struct ExApplyImpulse < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState2D, impulse: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState2D, impulse: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, impulse: impulse, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, impulse, position,
        }
        = self;
        re_export::PhysicsDirectBodyState2D::apply_impulse_full(surround_object, impulse, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState2D::apply_central_force_ex`][super::PhysicsDirectBodyState2D::apply_central_force_ex]."]
#[must_use]
pub struct ExApplyCentralForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState2D, force: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyCentralForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState2D,) -> Self {
        let force = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force,
        }
    }
    #[inline]
    pub fn force(self, force: Vector2) -> Self {
        Self {
            force: force, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force,
        }
        = self;
        re_export::PhysicsDirectBodyState2D::apply_central_force_full(surround_object, force,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState2D::apply_force_ex`][super::PhysicsDirectBodyState2D::apply_force_ex]."]
#[must_use]
pub struct ExApplyForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState2D, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState2D, force: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force, position,
        }
        = self;
        re_export::PhysicsDirectBodyState2D::apply_force_full(surround_object, force, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState2D::add_constant_central_force_ex`][super::PhysicsDirectBodyState2D::add_constant_central_force_ex]."]
#[must_use]
pub struct ExAddConstantCentralForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState2D, force: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddConstantCentralForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState2D,) -> Self {
        let force = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force,
        }
    }
    #[inline]
    pub fn force(self, force: Vector2) -> Self {
        Self {
            force: force, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force,
        }
        = self;
        re_export::PhysicsDirectBodyState2D::add_constant_central_force_full(surround_object, force,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState2D::add_constant_force_ex`][super::PhysicsDirectBodyState2D::add_constant_force_ex]."]
#[must_use]
pub struct ExAddConstantForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState2D, force: Vector2, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddConstantForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState2D, force: Vector2,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, force, position,
        }
        = self;
        re_export::PhysicsDirectBodyState2D::add_constant_force_full(surround_object, force, position,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PhysicsDirectBodyState2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for PhysicsDirectBodyState2D {
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