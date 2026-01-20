#![doc = "Sidecar module for class [`GpuParticles3D`][crate::classes::GpuParticles3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GPUParticles3D` enums](https://docs.godotengine.org/en/stable/classes/class_gpuparticles3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GPUParticles3D.`\n\nInherits [`GeometryInstance3D`][crate::classes::GeometryInstance3D].\n\nRelated symbols:\n\n* [`gpu_particles_3d`][crate::classes::gpu_particles_3d]: sidecar module with related enum/flag types\n* [`IGpuParticles3D`][crate::classes::IGpuParticles3D]: virtual methods\n* [`SignalsOfGpuParticles3D`][crate::classes::gpu_particles_3d::SignalsOfGpuParticles3D]: signal collection\n\n\nSee also [Godot docs for `GPUParticles3D`](https://docs.godotengine.org/en/stable/classes/class_gpuparticles3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`GpuParticles3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GpuParticles3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GpuParticles3D`][crate::classes::GpuParticles3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IGeometryInstance3D`][crate::classes::IGeometryInstance3D] > [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GPUParticles3D` methods](https://docs.godotengine.org/en/stable/classes/class_gpuparticles3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGpuParticles3D: crate::obj::GodotClass < Base = GpuParticles3D > + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `Base<T>` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::classes::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: Node3DNotification) {
            unimplemented !()
        }
        #[doc = r" Called whenever [`get()`](crate::classes::Object::get) is called or Godot gets the value of a property."]
        #[doc = r""]
        #[doc = r" Should return the given `property`'s value as `Some(value)`, or `None` if the property should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-get)."]
        fn get_property(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`set()`](crate::classes::Object::set) is called or Godot sets the value of a property."]
        #[doc = r""]
        #[doc = r" Should set `property` to the given `value` and return `true`, or return `false` to indicate the `property`"]
        #[doc = r" should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_set`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-set)."]
        fn set_property(&mut self, property: StringName, value: Variant) -> bool {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`get_property_list()`](crate::classes::Object::get_property_list) is called, the returned vector here is"]
        #[doc = r" appended to the existing list of properties."]
        #[doc = r""]
        #[doc = r" This should mainly be used for advanced purposes, such as dynamically updating the property list in the editor."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get_property_list`](https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-get-property-list)"]
        #[cfg(since_api = "4.3")]
        #[cfg_attr(published_docs, doc(cfg(since_api = "4.3")))]
        fn get_property_list(&mut self) -> Vec < crate::meta::PropertyInfo > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot retrieves value of property. Allows to customize existing properties."]
        #[doc = r" Every property info goes through this method, except properties **added** with `get_property_list()`."]
        #[doc = r""]
        #[doc = r" Exposed `property` here is a shared mutable reference obtained (and returned to) from Godot."]
        #[doc = r""]
        #[doc = r" See also in the Godot docs:"]
        #[doc = r" * [`Object::_validate_property`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-validate-property)"]
        fn validate_property(&self, property: &mut crate::meta::PropertyInfo) {
            unimplemented !()
        }
        #[doc = r" Called by Godot to tell if a property has a custom revert or not."]
        #[doc = r""]
        #[doc = r" Return `None` for no custom revert, and return `Some(value)` to specify the custom revert."]
        #[doc = r""]
        #[doc = r" This is a combination of Godot's [`Object::_property_get_revert`] and [`Object::_property_can_revert`]. This means that this"]
        #[doc = r" function will usually be called twice by Godot to find the revert."]
        #[doc = r""]
        #[doc = r" Note that this should be a _pure_ function. That is, it should always return the same value for a property as long as `self`"]
        #[doc = r" remains unchanged. Otherwise, this may lead to unexpected (safe) behavior."]
        #[doc = r""]
        #[doc = r" [`Object::_property_get_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-get-revert"]
        #[doc = r" [`Object::_property_can_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-can-revert"]
        #[doc(alias = "property_can_revert")]
        fn property_get_revert(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        fn get_aabb(&self,) -> Aabb {
            unimplemented !()
        }
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_accessibility_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn get_focused_accessibility_element(&self,) -> Rid {
            unimplemented !()
        }
    }
    impl GpuParticles3D {
        pub fn set_emitting(&mut self, emitting: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (emitting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3892usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_emitting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_amount(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3893usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_amount", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lifetime(&mut self, secs: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3894usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_lifetime", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_one_shot(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3895usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_one_shot", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pre_process_time(&mut self, secs: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3896usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_pre_process_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_explosiveness_ratio(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3897usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_explosiveness_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_randomness_ratio(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3898usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_randomness_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_aabb(&mut self, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Aabb,);
            let args = (aabb,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3899usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_visibility_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_local_coordinates(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3900usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_use_local_coordinates", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fixed_fps(&mut self, fps: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (fps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3901usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_fixed_fps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fractional_delta(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3902usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_fractional_delta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_interpolate(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3903usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_interpolate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_material(&mut self, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3904usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_process_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_speed_scale(&mut self, scale: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3905usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_base_size(&mut self, size: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3906usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_collision_base_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_interp_to_end(&mut self, interp: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (interp,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3907usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_interp_to_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_emitting(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3908usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "is_emitting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_amount(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3909usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_amount", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lifetime(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3910usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_lifetime", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_one_shot(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3911usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_one_shot", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pre_process_time(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3912usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_pre_process_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_explosiveness_ratio(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3913usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_explosiveness_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_randomness_ratio(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3914usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_randomness_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_aabb(&self,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3915usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_visibility_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_use_local_coordinates(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3916usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_use_local_coordinates", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fixed_fps(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3917usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_fixed_fps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fractional_delta(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3918usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_fractional_delta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_interpolate(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3919usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_interpolate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_material(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3920usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_process_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_speed_scale(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3921usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_base_size(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3922usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_collision_base_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_interp_to_end(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3923usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_interp_to_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_fixed_seed(&mut self, use_fixed_seed: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (use_fixed_seed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3924usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_use_fixed_seed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_use_fixed_seed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3925usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_use_fixed_seed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_seed(&mut self, seed: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (seed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3926usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_seed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_seed(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3927usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_seed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_order(&mut self, order: crate::classes::gpu_particles_3d::DrawOrder,) {
            type CallRet = ();
            type CallParams = (crate::classes::gpu_particles_3d::DrawOrder,);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3928usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_draw_order", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_draw_order(&self,) -> crate::classes::gpu_particles_3d::DrawOrder {
            type CallRet = crate::classes::gpu_particles_3d::DrawOrder;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3929usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_draw_order", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_passes(&mut self, passes: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (passes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3930usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_draw_passes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_pass_mesh(&mut self, pass: i32, mesh: impl AsArg < Option < Gd < crate::classes::Mesh >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Mesh > > >,);
            let args = (pass, mesh.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3931usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_draw_pass_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_draw_passes(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3932usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_draw_passes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_draw_pass_mesh(&self, pass: i32,) -> Option < Gd < crate::classes::Mesh > > {
            type CallRet = Option < Gd < crate::classes::Mesh > >;
            type CallParams = (i32,);
            let args = (pass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3933usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_draw_pass_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_skin(&mut self, skin: impl AsArg < Option < Gd < crate::classes::Skin >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Skin > > >,);
            let args = (skin.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3934usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_skin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_skin(&self,) -> Option < Gd < crate::classes::Skin > > {
            type CallRet = Option < Gd < crate::classes::Skin > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3935usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_skin", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn restart_full(&mut self, keep_seed: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (keep_seed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3936usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "restart", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::restart_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn restart(&mut self,) {
            self.restart_ex() . done()
        }
        #[inline]
        pub fn restart_ex < 'a > (&'a mut self,) -> ExRestart < 'a > {
            ExRestart::new(self,)
        }
        pub fn capture_aabb(&self,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3937usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "capture_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sub_emitter(&mut self, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3938usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_sub_emitter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sub_emitter(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3939usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_sub_emitter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn emit_particle(&mut self, xform: Transform3D, velocity: Vector3, color: Color, custom: Color, flags: crate::classes::gpu_particles_3d::EmitFlags,) {
            type CallRet = ();
            type CallParams = (Transform3D, Vector3, Color, Color, crate::classes::gpu_particles_3d::EmitFlags,);
            let args = (xform, velocity, color, custom, flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3940usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "emit_particle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_trail_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3941usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_trail_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_trail_lifetime(&mut self, secs: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3942usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_trail_lifetime", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_trail_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3943usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "is_trail_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_trail_lifetime(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3944usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_trail_lifetime", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transform_align(&mut self, align: crate::classes::gpu_particles_3d::TransformAlign,) {
            type CallRet = ();
            type CallParams = (crate::classes::gpu_particles_3d::TransformAlign,);
            let args = (align,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3945usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_transform_align", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transform_align(&self,) -> crate::classes::gpu_particles_3d::TransformAlign {
            type CallRet = crate::classes::gpu_particles_3d::TransformAlign;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3946usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_transform_align", Some(self.__validated_obj()), args,)
            }
        }
        pub fn convert_from_particles(&mut self, particles: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (particles.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3947usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "convert_from_particles", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_amount_ratio(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3948usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "set_amount_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_amount_ratio(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3949usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "get_amount_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn request_particles_process(&mut self, process_time: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (process_time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3950usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticles3D", "request_particles_process", Some(self.__validated_obj()), args,)
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
        pub const MAX_DRAW_PASSES: i32 = 4i32;
        
    }
    impl crate::obj::GodotClass for GpuParticles3D {
        type Base = crate::classes::GeometryInstance3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GPUParticles3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GpuParticles3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for GpuParticles3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for GpuParticles3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for GpuParticles3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for GpuParticles3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GpuParticles3D {
        
    }
    impl crate::obj::cap::GodotDefault for GpuParticles3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GpuParticles3D {
        type Target = crate::classes::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GpuParticles3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GpuParticles3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GpuParticles3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GpuParticles3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::GeometryInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`GpuParticles3D::restart_ex`][super::GpuParticles3D::restart_ex]."]
#[must_use]
pub struct ExRestart < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::GpuParticles3D, keep_seed: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRestart < 'a > {
    fn new(surround_object: &'a mut re_export::GpuParticles3D,) -> Self {
        let keep_seed = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, keep_seed: keep_seed,
        }
    }
    #[inline]
    pub fn keep_seed(self, keep_seed: bool) -> Self {
        Self {
            keep_seed: keep_seed, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, keep_seed,
        }
        = self;
        re_export::GpuParticles3D::restart_full(surround_object, keep_seed,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DrawOrder {
    ord: i32
}
impl DrawOrder {
    #[doc(alias = "DRAW_ORDER_INDEX")]
    #[doc = "Godot enumerator name: `DRAW_ORDER_INDEX`"]
    pub const INDEX: DrawOrder = DrawOrder {
        ord: 0i32
    };
    #[doc(alias = "DRAW_ORDER_LIFETIME")]
    #[doc = "Godot enumerator name: `DRAW_ORDER_LIFETIME`"]
    pub const LIFETIME: DrawOrder = DrawOrder {
        ord: 1i32
    };
    #[doc(alias = "DRAW_ORDER_REVERSE_LIFETIME")]
    #[doc = "Godot enumerator name: `DRAW_ORDER_REVERSE_LIFETIME`"]
    pub const REVERSE_LIFETIME: DrawOrder = DrawOrder {
        ord: 2i32
    };
    #[doc(alias = "DRAW_ORDER_VIEW_DEPTH")]
    #[doc = "Godot enumerator name: `DRAW_ORDER_VIEW_DEPTH`"]
    pub const VIEW_DEPTH: DrawOrder = DrawOrder {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for DrawOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DrawOrder") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DrawOrder {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::INDEX => "INDEX", Self::LIFETIME => "LIFETIME", Self::REVERSE_LIFETIME => "REVERSE_LIFETIME", Self::VIEW_DEPTH => "VIEW_DEPTH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DrawOrder::INDEX, DrawOrder::LIFETIME, DrawOrder::REVERSE_LIFETIME, DrawOrder::VIEW_DEPTH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DrawOrder >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INDEX", "DRAW_ORDER_INDEX", DrawOrder::INDEX), crate::meta::inspect::EnumConstant::new("LIFETIME", "DRAW_ORDER_LIFETIME", DrawOrder::LIFETIME), crate::meta::inspect::EnumConstant::new("REVERSE_LIFETIME", "DRAW_ORDER_REVERSE_LIFETIME", DrawOrder::REVERSE_LIFETIME), crate::meta::inspect::EnumConstant::new("VIEW_DEPTH", "DRAW_ORDER_VIEW_DEPTH", DrawOrder::VIEW_DEPTH)]
        }
    }
}
impl crate::meta::GodotConvert for DrawOrder {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DrawOrder {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DrawOrder {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct EmitFlags {
    ord: u64
}
impl EmitFlags {
    #[doc(alias = "EMIT_FLAG_POSITION")]
    #[doc = "Godot enumerator name: `EMIT_FLAG_POSITION`"]
    pub const POSITION: EmitFlags = EmitFlags {
        ord: 1u64
    };
    #[doc(alias = "EMIT_FLAG_ROTATION_SCALE")]
    #[doc = "Godot enumerator name: `EMIT_FLAG_ROTATION_SCALE`"]
    pub const ROTATION_SCALE: EmitFlags = EmitFlags {
        ord: 2u64
    };
    #[doc(alias = "EMIT_FLAG_VELOCITY")]
    #[doc = "Godot enumerator name: `EMIT_FLAG_VELOCITY`"]
    pub const VELOCITY: EmitFlags = EmitFlags {
        ord: 4u64
    };
    #[doc(alias = "EMIT_FLAG_COLOR")]
    #[doc = "Godot enumerator name: `EMIT_FLAG_COLOR`"]
    pub const COLOR: EmitFlags = EmitFlags {
        ord: 8u64
    };
    #[doc(alias = "EMIT_FLAG_CUSTOM")]
    #[doc = "Godot enumerator name: `EMIT_FLAG_CUSTOM`"]
    pub const CUSTOM: EmitFlags = EmitFlags {
        ord: 16u64
    };
    
}
impl std::fmt::Debug for EmitFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::POSITION => "POSITION", Self::ROTATION_SCALE => "ROTATION_SCALE", Self::VELOCITY => "VELOCITY", Self::COLOR => "COLOR", Self::CUSTOM => "CUSTOM", _ => {
                f.debug_struct("EmitFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for EmitFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EmitFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("POSITION", "EMIT_FLAG_POSITION", EmitFlags::POSITION), crate::meta::inspect::EnumConstant::new("ROTATION_SCALE", "EMIT_FLAG_ROTATION_SCALE", EmitFlags::ROTATION_SCALE), crate::meta::inspect::EnumConstant::new("VELOCITY", "EMIT_FLAG_VELOCITY", EmitFlags::VELOCITY), crate::meta::inspect::EnumConstant::new("COLOR", "EMIT_FLAG_COLOR", EmitFlags::COLOR), crate::meta::inspect::EnumConstant::new("CUSTOM", "EMIT_FLAG_CUSTOM", EmitFlags::CUSTOM)]
        }
    }
}
impl std::ops::BitOr for EmitFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for EmitFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for EmitFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for EmitFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EmitFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TransformAlign {
    ord: i32
}
impl TransformAlign {
    #[doc(alias = "TRANSFORM_ALIGN_DISABLED")]
    #[doc = "Godot enumerator name: `TRANSFORM_ALIGN_DISABLED`"]
    pub const DISABLED: TransformAlign = TransformAlign {
        ord: 0i32
    };
    #[doc(alias = "TRANSFORM_ALIGN_Z_BILLBOARD")]
    #[doc = "Godot enumerator name: `TRANSFORM_ALIGN_Z_BILLBOARD`"]
    pub const Z_BILLBOARD: TransformAlign = TransformAlign {
        ord: 1i32
    };
    #[doc(alias = "TRANSFORM_ALIGN_Y_TO_VELOCITY")]
    #[doc = "Godot enumerator name: `TRANSFORM_ALIGN_Y_TO_VELOCITY`"]
    pub const Y_TO_VELOCITY: TransformAlign = TransformAlign {
        ord: 2i32
    };
    #[doc(alias = "TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY")]
    #[doc = "Godot enumerator name: `TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY`"]
    pub const Z_BILLBOARD_Y_TO_VELOCITY: TransformAlign = TransformAlign {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TransformAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TransformAlign") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TransformAlign {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "DISABLED", Self::Z_BILLBOARD => "Z_BILLBOARD", Self::Y_TO_VELOCITY => "Y_TO_VELOCITY", Self::Z_BILLBOARD_Y_TO_VELOCITY => "Z_BILLBOARD_Y_TO_VELOCITY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TransformAlign::DISABLED, TransformAlign::Z_BILLBOARD, TransformAlign::Y_TO_VELOCITY, TransformAlign::Z_BILLBOARD_Y_TO_VELOCITY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TransformAlign >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "TRANSFORM_ALIGN_DISABLED", TransformAlign::DISABLED), crate::meta::inspect::EnumConstant::new("Z_BILLBOARD", "TRANSFORM_ALIGN_Z_BILLBOARD", TransformAlign::Z_BILLBOARD), crate::meta::inspect::EnumConstant::new("Y_TO_VELOCITY", "TRANSFORM_ALIGN_Y_TO_VELOCITY", TransformAlign::Y_TO_VELOCITY), crate::meta::inspect::EnumConstant::new("Z_BILLBOARD_Y_TO_VELOCITY", "TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY", TransformAlign::Z_BILLBOARD_Y_TO_VELOCITY)]
        }
    }
}
impl crate::meta::GodotConvert for TransformAlign {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TransformAlign {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TransformAlign {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GpuParticles3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`GpuParticles3D`][crate::classes::GpuParticles3D] class."]
    pub struct SignalsOfGpuParticles3D < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfGpuParticles3D < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn finished(&mut self) -> SigFinished < 'c, C > {
            SigFinished {
                typed: TypedSignal::extract(&mut self.__internal_obj, "finished")
            }
        }
    }
    type TypedSigFinished < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFinished < 'c, C: WithSignals > {
        typed: TypedSigFinished < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFinished < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFinished < 'c, C > {
        type Target = TypedSigFinished < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFinished < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for GpuParticles3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfGpuParticles3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfGpuParticles3D < 'c, C > {
        type Target = < < GpuParticles3D as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = GpuParticles3D;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfGpuParticles3D < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = GpuParticles3D;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}