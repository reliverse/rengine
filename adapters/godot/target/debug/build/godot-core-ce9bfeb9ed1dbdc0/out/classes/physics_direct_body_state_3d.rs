#![doc = "Sidecar module for class [`PhysicsDirectBodyState3D`][crate::classes::PhysicsDirectBodyState3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsDirectBodyState3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PhysicsDirectBodyState3D.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`physics_direct_body_state_3d`][crate::classes::physics_direct_body_state_3d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `PhysicsDirectBodyState3D`](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<PhysicsDirectBodyState3D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PhysicsDirectBodyState3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl PhysicsDirectBodyState3D {
        pub fn get_total_gravity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(376usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_total_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_total_linear_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(377usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_total_linear_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_total_angular_damp(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(378usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_total_angular_damp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_of_mass(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(379usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_center_of_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_of_mass_local(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(380usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_center_of_mass_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_principal_inertia_axes(&self,) -> Basis {
            type CallRet = Basis;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(381usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_principal_inertia_axes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inverse_mass(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(382usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_inverse_mass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inverse_inertia(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(383usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_inverse_inertia", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inverse_inertia_tensor(&self,) -> Basis {
            type CallRet = Basis;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(384usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_inverse_inertia_tensor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_linear_velocity(&mut self, velocity: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(385usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "set_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_linear_velocity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(386usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_angular_velocity(&mut self, velocity: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(387usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "set_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_angular_velocity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(388usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transform(&mut self, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Transform3D,);
            let args = (transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(389usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transform(&self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(390usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_velocity_at_local_position(&self, local_position: Vector3,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector3,);
            let args = (local_position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(391usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_velocity_at_local_position", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn apply_central_impulse_full(&mut self, impulse: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(392usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "apply_central_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::apply_central_impulse_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn apply_central_impulse(&mut self,) {
            self.apply_central_impulse_ex() . done()
        }
        #[inline]
        pub fn apply_central_impulse_ex < 'a > (&'a mut self,) -> ExApplyCentralImpulse < 'a > {
            ExApplyCentralImpulse::new(self,)
        }
        pub(crate) fn apply_impulse_full(&mut self, impulse: Vector3, position: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3, Vector3,);
            let args = (impulse, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(393usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "apply_impulse", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::apply_impulse_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn apply_impulse(&mut self, impulse: Vector3,) {
            self.apply_impulse_ex(impulse,) . done()
        }
        #[inline]
        pub fn apply_impulse_ex < 'a > (&'a mut self, impulse: Vector3,) -> ExApplyImpulse < 'a > {
            ExApplyImpulse::new(self, impulse,)
        }
        pub fn apply_torque_impulse(&mut self, impulse: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (impulse,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(394usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "apply_torque_impulse", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn apply_central_force_full(&mut self, force: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(395usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "apply_central_force", Some(self.__validated_obj()), args,)
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
        pub(crate) fn apply_force_full(&mut self, force: Vector3, position: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3, Vector3,);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(396usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "apply_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::apply_force_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn apply_force(&mut self, force: Vector3,) {
            self.apply_force_ex(force,) . done()
        }
        #[inline]
        pub fn apply_force_ex < 'a > (&'a mut self, force: Vector3,) -> ExApplyForce < 'a > {
            ExApplyForce::new(self, force,)
        }
        pub fn apply_torque(&mut self, torque: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(397usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "apply_torque", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_constant_central_force_full(&mut self, force: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(398usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "add_constant_central_force", Some(self.__validated_obj()), args,)
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
        pub(crate) fn add_constant_force_full(&mut self, force: Vector3, position: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3, Vector3,);
            let args = (force, position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(399usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "add_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_constant_force_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_constant_force(&mut self, force: Vector3,) {
            self.add_constant_force_ex(force,) . done()
        }
        #[inline]
        pub fn add_constant_force_ex < 'a > (&'a mut self, force: Vector3,) -> ExAddConstantForce < 'a > {
            ExAddConstantForce::new(self, force,)
        }
        pub fn add_constant_torque(&mut self, torque: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(400usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "add_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_constant_force(&mut self, force: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (force,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(401usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "set_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_constant_force(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(402usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_constant_force", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_constant_torque(&mut self, torque: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (torque,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(403usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "set_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_constant_torque(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_constant_torque", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sleep_state(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "set_sleep_state", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_sleeping(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "is_sleeping", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_layer(&mut self, layer: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "set_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_layer(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_mask(&mut self, mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "set_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_local_position(&self, contact_idx: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_local_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_local_normal(&self, contact_idx: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_local_normal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_impulse(&self, contact_idx: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_impulse", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_local_shape(&self, contact_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_local_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_local_velocity_at_position(&self, contact_idx: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_local_velocity_at_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_collider(&self, contact_idx: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_collider", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_collider_position(&self, contact_idx: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_collider_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_collider_id(&self, contact_idx: i32,) -> u64 {
            type CallRet = u64;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_collider_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_collider_object(&self, contact_idx: i32,) -> Option < Gd < crate::classes::Object > > {
            type CallRet = Option < Gd < crate::classes::Object > >;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(420usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_collider_object", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_collider_shape(&self, contact_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(421usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_collider_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_contact_collider_velocity_at_position(&self, contact_idx: i32,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32,);
            let args = (contact_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(422usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_contact_collider_velocity_at_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_step(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(423usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_step", Some(self.__validated_obj()), args,)
            }
        }
        pub fn integrate_forces(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(424usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "integrate_forces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_space_state(&mut self,) -> Option < Gd < crate::classes::PhysicsDirectSpaceState3D > > {
            type CallRet = Option < Gd < crate::classes::PhysicsDirectSpaceState3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(425usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PhysicsDirectBodyState3D", "get_space_state", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PhysicsDirectBodyState3D {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PhysicsDirectBodyState3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for PhysicsDirectBodyState3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PhysicsDirectBodyState3D {
        
    }
    impl std::ops::Deref for PhysicsDirectBodyState3D {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PhysicsDirectBodyState3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PhysicsDirectBodyState3D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `PhysicsDirectBodyState3D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState3D::apply_central_impulse_ex`][super::PhysicsDirectBodyState3D::apply_central_impulse_ex]."]
#[must_use]
pub struct ExApplyCentralImpulse < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState3D, impulse: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyCentralImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState3D,) -> Self {
        let impulse = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, impulse: impulse,
        }
    }
    #[inline]
    pub fn impulse(self, impulse: Vector3) -> Self {
        Self {
            impulse: impulse, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, impulse,
        }
        = self;
        re_export::PhysicsDirectBodyState3D::apply_central_impulse_full(surround_object, impulse,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState3D::apply_impulse_ex`][super::PhysicsDirectBodyState3D::apply_impulse_ex]."]
#[must_use]
pub struct ExApplyImpulse < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState3D, impulse: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyImpulse < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState3D, impulse: Vector3,) -> Self {
        let position = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, impulse: impulse, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector3) -> Self {
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
        re_export::PhysicsDirectBodyState3D::apply_impulse_full(surround_object, impulse, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState3D::apply_central_force_ex`][super::PhysicsDirectBodyState3D::apply_central_force_ex]."]
#[must_use]
pub struct ExApplyCentralForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState3D, force: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyCentralForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState3D,) -> Self {
        let force = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force,
        }
    }
    #[inline]
    pub fn force(self, force: Vector3) -> Self {
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
        re_export::PhysicsDirectBodyState3D::apply_central_force_full(surround_object, force,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState3D::apply_force_ex`][super::PhysicsDirectBodyState3D::apply_force_ex]."]
#[must_use]
pub struct ExApplyForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState3D, force: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExApplyForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState3D, force: Vector3,) -> Self {
        let position = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector3) -> Self {
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
        re_export::PhysicsDirectBodyState3D::apply_force_full(surround_object, force, position,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState3D::add_constant_central_force_ex`][super::PhysicsDirectBodyState3D::add_constant_central_force_ex]."]
#[must_use]
pub struct ExAddConstantCentralForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState3D, force: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddConstantCentralForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState3D,) -> Self {
        let force = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force,
        }
    }
    #[inline]
    pub fn force(self, force: Vector3) -> Self {
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
        re_export::PhysicsDirectBodyState3D::add_constant_central_force_full(surround_object, force,)
    }
}
#[doc = "Default-param extender for [`PhysicsDirectBodyState3D::add_constant_force_ex`][super::PhysicsDirectBodyState3D::add_constant_force_ex]."]
#[must_use]
pub struct ExAddConstantForce < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PhysicsDirectBodyState3D, force: Vector3, position: Vector3,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddConstantForce < 'a > {
    fn new(surround_object: &'a mut re_export::PhysicsDirectBodyState3D, force: Vector3,) -> Self {
        let position = Vector3::new(0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force: force, position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector3) -> Self {
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
        re_export::PhysicsDirectBodyState3D::add_constant_force_full(surround_object, force, position,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PhysicsDirectBodyState3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for PhysicsDirectBodyState3D {
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