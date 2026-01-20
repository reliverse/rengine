#![doc = "Sidecar module for class [`ParticleProcessMaterial`][crate::classes::ParticleProcessMaterial].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ParticleProcessMaterial` enums](https://docs.godotengine.org/en/stable/classes/class_particleprocessmaterial.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ParticleProcessMaterial.`\n\nInherits [`Material`][crate::classes::Material].\n\nRelated symbols:\n\n* [`particle_process_material`][crate::classes::particle_process_material]: sidecar module with related enum/flag types\n* [`IParticleProcessMaterial`][crate::classes::IParticleProcessMaterial]: virtual methods\n* [`SignalsOfParticleProcessMaterial`][crate::classes::particle_process_material::SignalsOfParticleProcessMaterial]: signal collection\n\n\nSee also [Godot docs for `ParticleProcessMaterial`](https://docs.godotengine.org/en/stable/classes/class_particleprocessmaterial.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ParticleProcessMaterial::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ParticleProcessMaterial {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ParticleProcessMaterial`][crate::classes::ParticleProcessMaterial].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IMaterial`][crate::classes::IMaterial] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `ParticleProcessMaterial` methods](https://docs.godotengine.org/en/stable/classes/class_particleprocessmaterial.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IParticleProcessMaterial: crate::obj::GodotClass < Base = ParticleProcessMaterial > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ObjectNotification) {
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
        fn get_shader_rid(&self,) -> Rid;
        fn get_shader_mode(&self,) -> crate::classes::shader::Mode;
        fn can_do_next_pass(&self,) -> bool {
            unimplemented !()
        }
        fn can_use_render_priority(&self,) -> bool {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
        fn get_rid(&self,) -> Rid {
            unimplemented !()
        }
        fn reset_state(&mut self,) {
            unimplemented !()
        }
        fn set_path_cache(&self, path: GString,) {
            unimplemented !()
        }
    }
    impl ParticleProcessMaterial {
        pub fn set_direction(&mut self, degrees: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_direction(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_inherit_velocity_ratio(&mut self, ratio: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_inherit_velocity_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inherit_velocity_ratio(&mut self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_inherit_velocity_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_spread(&mut self, degrees: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_spread", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_spread(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_spread", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flatness(&mut self, amount: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_flatness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_flatness(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_flatness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_param(&mut self, param: crate::classes::particle_process_material::Parameter, value: Vector2,) {
            type CallRet = ();
            type CallParams = (crate::classes::particle_process_material::Parameter, Vector2,);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_param", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_param(&self, param: crate::classes::particle_process_material::Parameter,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (crate::classes::particle_process_material::Parameter,);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_param", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_param_min(&mut self, param: crate::classes::particle_process_material::Parameter, value: f32,) {
            type CallRet = ();
            type CallParams = (crate::classes::particle_process_material::Parameter, f32,);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_param_min", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_param_min(&self, param: crate::classes::particle_process_material::Parameter,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::classes::particle_process_material::Parameter,);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_param_min", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_param_max(&mut self, param: crate::classes::particle_process_material::Parameter, value: f32,) {
            type CallRet = ();
            type CallParams = (crate::classes::particle_process_material::Parameter, f32,);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_param_max", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_param_max(&self, param: crate::classes::particle_process_material::Parameter,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::classes::particle_process_material::Parameter,);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_param_max", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_param_texture(&mut self, param: crate::classes::particle_process_material::Parameter, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::particle_process_material::Parameter, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (param, texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_param_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_param_texture(&self, param: crate::classes::particle_process_material::Parameter,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (crate::classes::particle_process_material::Parameter,);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6280usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_param_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6281usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6282usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color_ramp(&mut self, ramp: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (ramp.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6283usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_color_ramp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color_ramp(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6284usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_color_ramp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_curve(&mut self, curve: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6285usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_alpha_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_curve(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6286usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_alpha_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_curve(&mut self, curve: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6287usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_curve(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6288usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color_initial_ramp(&mut self, ramp: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (ramp.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6289usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_color_initial_ramp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color_initial_ramp(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6290usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_color_initial_ramp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_velocity_limit_curve(&mut self, curve: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6291usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_velocity_limit_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_velocity_limit_curve(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6292usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_velocity_limit_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_particle_flag(&mut self, particle_flag: crate::classes::particle_process_material::ParticleFlags, enable: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::particle_process_material::ParticleFlags, bool,);
            let args = (particle_flag, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6293usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_particle_flag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_particle_flag(&self, particle_flag: crate::classes::particle_process_material::ParticleFlags,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::particle_process_material::ParticleFlags,);
            let args = (particle_flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6294usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_particle_flag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_velocity_pivot(&mut self, pivot: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (pivot,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6295usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_velocity_pivot", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_velocity_pivot(&mut self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6296usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_velocity_pivot", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_shape(&mut self, shape: crate::classes::particle_process_material::EmissionShape,) {
            type CallRet = ();
            type CallParams = (crate::classes::particle_process_material::EmissionShape,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6297usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_shape(&self,) -> crate::classes::particle_process_material::EmissionShape {
            type CallRet = crate::classes::particle_process_material::EmissionShape;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6298usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_sphere_radius(&mut self, radius: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6299usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_sphere_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_sphere_radius(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6300usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_sphere_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_box_extents(&mut self, extents: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (extents,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6301usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_box_extents", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_box_extents(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6302usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_box_extents", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_point_texture(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6303usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_point_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_point_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6304usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_point_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_normal_texture(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6305usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_normal_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_normal_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_normal_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_color_texture(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_color_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_color_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6308usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_color_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_point_count(&mut self, point_count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (point_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6309usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_point_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_point_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6310usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_point_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_ring_axis(&mut self, axis: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6311usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_ring_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_ring_axis(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6312usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_ring_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_ring_height(&mut self, height: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6313usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_ring_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_ring_height(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6314usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_ring_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_ring_radius(&mut self, radius: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6315usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_ring_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_ring_radius(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6316usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_ring_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_ring_inner_radius(&mut self, inner_radius: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (inner_radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6317usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_ring_inner_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_ring_inner_radius(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6318usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_ring_inner_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_ring_cone_angle(&mut self, cone_angle: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (cone_angle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6319usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_ring_cone_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_ring_cone_angle(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6320usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_ring_cone_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_shape_offset(&mut self, emission_shape_offset: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (emission_shape_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6321usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_shape_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_shape_offset(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_shape_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_shape_scale(&mut self, emission_shape_scale: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (emission_shape_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6323usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_emission_shape_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_shape_scale(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_emission_shape_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_turbulence_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_turbulence_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_turbulence_enabled(&mut self, turbulence_enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (turbulence_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_turbulence_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_turbulence_noise_strength(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_turbulence_noise_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_turbulence_noise_strength(&mut self, turbulence_noise_strength: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (turbulence_noise_strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_turbulence_noise_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_turbulence_noise_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_turbulence_noise_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_turbulence_noise_scale(&mut self, turbulence_noise_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (turbulence_noise_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_turbulence_noise_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_turbulence_noise_speed_random(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_turbulence_noise_speed_random", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_turbulence_noise_speed_random(&mut self, turbulence_noise_speed_random: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (turbulence_noise_speed_random,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_turbulence_noise_speed_random", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_turbulence_noise_speed(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_turbulence_noise_speed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_turbulence_noise_speed(&mut self, turbulence_noise_speed: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (turbulence_noise_speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_turbulence_noise_speed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gravity(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gravity(&mut self, accel_vec: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (accel_vec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_gravity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lifetime_randomness(&mut self, randomness: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (randomness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_lifetime_randomness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lifetime_randomness(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_lifetime_randomness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sub_emitter_mode(&self,) -> crate::classes::particle_process_material::SubEmitterMode {
            type CallRet = crate::classes::particle_process_material::SubEmitterMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_sub_emitter_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sub_emitter_mode(&mut self, mode: crate::classes::particle_process_material::SubEmitterMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::particle_process_material::SubEmitterMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_sub_emitter_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sub_emitter_frequency(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_sub_emitter_frequency", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sub_emitter_frequency(&mut self, hz: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (hz,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6342usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_sub_emitter_frequency", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sub_emitter_amount_at_end(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6343usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_sub_emitter_amount_at_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sub_emitter_amount_at_end(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6344usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_sub_emitter_amount_at_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sub_emitter_amount_at_collision(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6345usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_sub_emitter_amount_at_collision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sub_emitter_amount_at_collision(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6346usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_sub_emitter_amount_at_collision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sub_emitter_amount_at_start(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6347usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_sub_emitter_amount_at_start", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sub_emitter_amount_at_start(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6348usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_sub_emitter_amount_at_start", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sub_emitter_keep_velocity(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6349usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_sub_emitter_keep_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sub_emitter_keep_velocity(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6350usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_sub_emitter_keep_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_attractor_interaction_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6351usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_attractor_interaction_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_attractor_interaction_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6352usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "is_attractor_interaction_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_mode(&mut self, mode: crate::classes::particle_process_material::CollisionMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::particle_process_material::CollisionMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6353usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_collision_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_mode(&self,) -> crate::classes::particle_process_material::CollisionMode {
            type CallRet = crate::classes::particle_process_material::CollisionMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6354usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_collision_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_use_scale(&mut self, radius: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6355usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_collision_use_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_collision_using_scale(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6356usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "is_collision_using_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_friction(&mut self, friction: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (friction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6357usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_collision_friction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_friction(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6358usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_collision_friction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_bounce(&mut self, bounce: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (bounce,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "set_collision_bounce", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_bounce(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ParticleProcessMaterial", "get_collision_bounce", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ParticleProcessMaterial {
        type Base = crate::classes::Material;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ParticleProcessMaterial"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ParticleProcessMaterial {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Material > for ParticleProcessMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for ParticleProcessMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ParticleProcessMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ParticleProcessMaterial {
        
    }
    impl crate::obj::cap::GodotDefault for ParticleProcessMaterial {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ParticleProcessMaterial {
        type Target = crate::classes::Material;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ParticleProcessMaterial {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ParticleProcessMaterial`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ParticleProcessMaterial__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ParticleProcessMaterial > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Material > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Parameter {
    ord: i32
}
impl Parameter {
    #[doc(alias = "PARAM_INITIAL_LINEAR_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_INITIAL_LINEAR_VELOCITY`"]
    pub const INITIAL_LINEAR_VELOCITY: Parameter = Parameter {
        ord: 0i32
    };
    #[doc(alias = "PARAM_ANGULAR_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_ANGULAR_VELOCITY`"]
    pub const ANGULAR_VELOCITY: Parameter = Parameter {
        ord: 1i32
    };
    #[doc(alias = "PARAM_ORBIT_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_ORBIT_VELOCITY`"]
    pub const ORBIT_VELOCITY: Parameter = Parameter {
        ord: 2i32
    };
    #[doc(alias = "PARAM_LINEAR_ACCEL")]
    #[doc = "Godot enumerator name: `PARAM_LINEAR_ACCEL`"]
    pub const LINEAR_ACCEL: Parameter = Parameter {
        ord: 3i32
    };
    #[doc(alias = "PARAM_RADIAL_ACCEL")]
    #[doc = "Godot enumerator name: `PARAM_RADIAL_ACCEL`"]
    pub const RADIAL_ACCEL: Parameter = Parameter {
        ord: 4i32
    };
    #[doc(alias = "PARAM_TANGENTIAL_ACCEL")]
    #[doc = "Godot enumerator name: `PARAM_TANGENTIAL_ACCEL`"]
    pub const TANGENTIAL_ACCEL: Parameter = Parameter {
        ord: 5i32
    };
    #[doc(alias = "PARAM_DAMPING")]
    #[doc = "Godot enumerator name: `PARAM_DAMPING`"]
    pub const DAMPING: Parameter = Parameter {
        ord: 6i32
    };
    #[doc(alias = "PARAM_ANGLE")]
    #[doc = "Godot enumerator name: `PARAM_ANGLE`"]
    pub const ANGLE: Parameter = Parameter {
        ord: 7i32
    };
    #[doc(alias = "PARAM_SCALE")]
    #[doc = "Godot enumerator name: `PARAM_SCALE`"]
    pub const SCALE: Parameter = Parameter {
        ord: 8i32
    };
    #[doc(alias = "PARAM_HUE_VARIATION")]
    #[doc = "Godot enumerator name: `PARAM_HUE_VARIATION`"]
    pub const HUE_VARIATION: Parameter = Parameter {
        ord: 9i32
    };
    #[doc(alias = "PARAM_ANIM_SPEED")]
    #[doc = "Godot enumerator name: `PARAM_ANIM_SPEED`"]
    pub const ANIM_SPEED: Parameter = Parameter {
        ord: 10i32
    };
    #[doc(alias = "PARAM_ANIM_OFFSET")]
    #[doc = "Godot enumerator name: `PARAM_ANIM_OFFSET`"]
    pub const ANIM_OFFSET: Parameter = Parameter {
        ord: 11i32
    };
    #[doc(alias = "PARAM_RADIAL_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_RADIAL_VELOCITY`"]
    pub const RADIAL_VELOCITY: Parameter = Parameter {
        ord: 15i32
    };
    #[doc(alias = "PARAM_DIRECTIONAL_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_DIRECTIONAL_VELOCITY`"]
    pub const DIRECTIONAL_VELOCITY: Parameter = Parameter {
        ord: 16i32
    };
    #[doc(alias = "PARAM_SCALE_OVER_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_SCALE_OVER_VELOCITY`"]
    pub const SCALE_OVER_VELOCITY: Parameter = Parameter {
        ord: 17i32
    };
    #[doc(alias = "PARAM_MAX")]
    #[doc = "Godot enumerator name: `PARAM_MAX`"]
    pub const MAX: Parameter = Parameter {
        ord: 18i32
    };
    #[doc(alias = "PARAM_TURB_VEL_INFLUENCE")]
    #[doc = "Godot enumerator name: `PARAM_TURB_VEL_INFLUENCE`"]
    pub const TURB_VEL_INFLUENCE: Parameter = Parameter {
        ord: 13i32
    };
    #[doc(alias = "PARAM_TURB_INIT_DISPLACEMENT")]
    #[doc = "Godot enumerator name: `PARAM_TURB_INIT_DISPLACEMENT`"]
    pub const TURB_INIT_DISPLACEMENT: Parameter = Parameter {
        ord: 14i32
    };
    #[doc(alias = "PARAM_TURB_INFLUENCE_OVER_LIFE")]
    #[doc = "Godot enumerator name: `PARAM_TURB_INFLUENCE_OVER_LIFE`"]
    pub const TURB_INFLUENCE_OVER_LIFE: Parameter = Parameter {
        ord: 12i32
    };
    
}
impl std::fmt::Debug for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Parameter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Parameter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 => Some(Self {
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
            Self::INITIAL_LINEAR_VELOCITY => "INITIAL_LINEAR_VELOCITY", Self::ANGULAR_VELOCITY => "ANGULAR_VELOCITY", Self::ORBIT_VELOCITY => "ORBIT_VELOCITY", Self::LINEAR_ACCEL => "LINEAR_ACCEL", Self::RADIAL_ACCEL => "RADIAL_ACCEL", Self::TANGENTIAL_ACCEL => "TANGENTIAL_ACCEL", Self::DAMPING => "DAMPING", Self::ANGLE => "ANGLE", Self::SCALE => "SCALE", Self::HUE_VARIATION => "HUE_VARIATION", Self::ANIM_SPEED => "ANIM_SPEED", Self::ANIM_OFFSET => "ANIM_OFFSET", Self::RADIAL_VELOCITY => "RADIAL_VELOCITY", Self::DIRECTIONAL_VELOCITY => "DIRECTIONAL_VELOCITY", Self::SCALE_OVER_VELOCITY => "SCALE_OVER_VELOCITY", Self::MAX => "MAX", Self::TURB_VEL_INFLUENCE => "TURB_VEL_INFLUENCE", Self::TURB_INIT_DISPLACEMENT => "TURB_INIT_DISPLACEMENT", Self::TURB_INFLUENCE_OVER_LIFE => "TURB_INFLUENCE_OVER_LIFE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Parameter::INITIAL_LINEAR_VELOCITY, Parameter::ANGULAR_VELOCITY, Parameter::ORBIT_VELOCITY, Parameter::LINEAR_ACCEL, Parameter::RADIAL_ACCEL, Parameter::TANGENTIAL_ACCEL, Parameter::DAMPING, Parameter::ANGLE, Parameter::SCALE, Parameter::HUE_VARIATION, Parameter::ANIM_SPEED, Parameter::ANIM_OFFSET, Parameter::RADIAL_VELOCITY, Parameter::DIRECTIONAL_VELOCITY, Parameter::SCALE_OVER_VELOCITY, Parameter::MAX, Parameter::TURB_VEL_INFLUENCE, Parameter::TURB_INIT_DISPLACEMENT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Parameter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INITIAL_LINEAR_VELOCITY", "PARAM_INITIAL_LINEAR_VELOCITY", Parameter::INITIAL_LINEAR_VELOCITY), crate::meta::inspect::EnumConstant::new("ANGULAR_VELOCITY", "PARAM_ANGULAR_VELOCITY", Parameter::ANGULAR_VELOCITY), crate::meta::inspect::EnumConstant::new("ORBIT_VELOCITY", "PARAM_ORBIT_VELOCITY", Parameter::ORBIT_VELOCITY), crate::meta::inspect::EnumConstant::new("LINEAR_ACCEL", "PARAM_LINEAR_ACCEL", Parameter::LINEAR_ACCEL), crate::meta::inspect::EnumConstant::new("RADIAL_ACCEL", "PARAM_RADIAL_ACCEL", Parameter::RADIAL_ACCEL), crate::meta::inspect::EnumConstant::new("TANGENTIAL_ACCEL", "PARAM_TANGENTIAL_ACCEL", Parameter::TANGENTIAL_ACCEL), crate::meta::inspect::EnumConstant::new("DAMPING", "PARAM_DAMPING", Parameter::DAMPING), crate::meta::inspect::EnumConstant::new("ANGLE", "PARAM_ANGLE", Parameter::ANGLE), crate::meta::inspect::EnumConstant::new("SCALE", "PARAM_SCALE", Parameter::SCALE), crate::meta::inspect::EnumConstant::new("HUE_VARIATION", "PARAM_HUE_VARIATION", Parameter::HUE_VARIATION), crate::meta::inspect::EnumConstant::new("ANIM_SPEED", "PARAM_ANIM_SPEED", Parameter::ANIM_SPEED), crate::meta::inspect::EnumConstant::new("ANIM_OFFSET", "PARAM_ANIM_OFFSET", Parameter::ANIM_OFFSET), crate::meta::inspect::EnumConstant::new("RADIAL_VELOCITY", "PARAM_RADIAL_VELOCITY", Parameter::RADIAL_VELOCITY), crate::meta::inspect::EnumConstant::new("DIRECTIONAL_VELOCITY", "PARAM_DIRECTIONAL_VELOCITY", Parameter::DIRECTIONAL_VELOCITY), crate::meta::inspect::EnumConstant::new("SCALE_OVER_VELOCITY", "PARAM_SCALE_OVER_VELOCITY", Parameter::SCALE_OVER_VELOCITY), crate::meta::inspect::EnumConstant::new("MAX", "PARAM_MAX", Parameter::MAX), crate::meta::inspect::EnumConstant::new("TURB_VEL_INFLUENCE", "PARAM_TURB_VEL_INFLUENCE", Parameter::TURB_VEL_INFLUENCE), crate::meta::inspect::EnumConstant::new("TURB_INIT_DISPLACEMENT", "PARAM_TURB_INIT_DISPLACEMENT", Parameter::TURB_INIT_DISPLACEMENT), crate::meta::inspect::EnumConstant::new("TURB_INFLUENCE_OVER_LIFE", "PARAM_TURB_INFLUENCE_OVER_LIFE", Parameter::TURB_INFLUENCE_OVER_LIFE)]
        }
    }
}
impl crate::obj::IndexEnum for Parameter {
    const ENUMERATOR_COUNT: usize = 18usize;
    
}
impl crate::meta::GodotConvert for Parameter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Parameter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Parameter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ParticleFlags {
    ord: i32
}
impl ParticleFlags {
    #[doc(alias = "PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY`"]
    pub const ALIGN_Y_TO_VELOCITY: ParticleFlags = ParticleFlags {
        ord: 0i32
    };
    #[doc(alias = "PARTICLE_FLAG_ROTATE_Y")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_ROTATE_Y`"]
    pub const ROTATE_Y: ParticleFlags = ParticleFlags {
        ord: 1i32
    };
    #[doc(alias = "PARTICLE_FLAG_DISABLE_Z")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_DISABLE_Z`"]
    pub const DISABLE_Z: ParticleFlags = ParticleFlags {
        ord: 2i32
    };
    #[doc(alias = "PARTICLE_FLAG_DAMPING_AS_FRICTION")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_DAMPING_AS_FRICTION`"]
    pub const DAMPING_AS_FRICTION: ParticleFlags = ParticleFlags {
        ord: 3i32
    };
    #[doc(alias = "PARTICLE_FLAG_MAX")]
    #[doc = "Godot enumerator name: `PARTICLE_FLAG_MAX`"]
    pub const MAX: ParticleFlags = ParticleFlags {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ParticleFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ParticleFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ParticleFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::ALIGN_Y_TO_VELOCITY => "ALIGN_Y_TO_VELOCITY", Self::ROTATE_Y => "ROTATE_Y", Self::DISABLE_Z => "DISABLE_Z", Self::DAMPING_AS_FRICTION => "DAMPING_AS_FRICTION", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ParticleFlags::ALIGN_Y_TO_VELOCITY, ParticleFlags::ROTATE_Y, ParticleFlags::DISABLE_Z, ParticleFlags::DAMPING_AS_FRICTION]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ParticleFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ALIGN_Y_TO_VELOCITY", "PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY", ParticleFlags::ALIGN_Y_TO_VELOCITY), crate::meta::inspect::EnumConstant::new("ROTATE_Y", "PARTICLE_FLAG_ROTATE_Y", ParticleFlags::ROTATE_Y), crate::meta::inspect::EnumConstant::new("DISABLE_Z", "PARTICLE_FLAG_DISABLE_Z", ParticleFlags::DISABLE_Z), crate::meta::inspect::EnumConstant::new("DAMPING_AS_FRICTION", "PARTICLE_FLAG_DAMPING_AS_FRICTION", ParticleFlags::DAMPING_AS_FRICTION), crate::meta::inspect::EnumConstant::new("MAX", "PARTICLE_FLAG_MAX", ParticleFlags::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ParticleFlags {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for ParticleFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ParticleFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ParticleFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EmissionShape {
    ord: i32
}
impl EmissionShape {
    #[doc(alias = "EMISSION_SHAPE_POINT")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_POINT`"]
    pub const POINT: EmissionShape = EmissionShape {
        ord: 0i32
    };
    #[doc(alias = "EMISSION_SHAPE_SPHERE")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_SPHERE`"]
    pub const SPHERE: EmissionShape = EmissionShape {
        ord: 1i32
    };
    #[doc(alias = "EMISSION_SHAPE_SPHERE_SURFACE")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_SPHERE_SURFACE`"]
    pub const SPHERE_SURFACE: EmissionShape = EmissionShape {
        ord: 2i32
    };
    #[doc(alias = "EMISSION_SHAPE_BOX")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_BOX`"]
    pub const BOX: EmissionShape = EmissionShape {
        ord: 3i32
    };
    #[doc(alias = "EMISSION_SHAPE_POINTS")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_POINTS`"]
    pub const POINTS: EmissionShape = EmissionShape {
        ord: 4i32
    };
    #[doc(alias = "EMISSION_SHAPE_DIRECTED_POINTS")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_DIRECTED_POINTS`"]
    pub const DIRECTED_POINTS: EmissionShape = EmissionShape {
        ord: 5i32
    };
    #[doc(alias = "EMISSION_SHAPE_RING")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_RING`"]
    pub const RING: EmissionShape = EmissionShape {
        ord: 6i32
    };
    #[doc(alias = "EMISSION_SHAPE_MAX")]
    #[doc = "Godot enumerator name: `EMISSION_SHAPE_MAX`"]
    pub const MAX: EmissionShape = EmissionShape {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for EmissionShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EmissionShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EmissionShape {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
            Self::POINT => "POINT", Self::SPHERE => "SPHERE", Self::SPHERE_SURFACE => "SPHERE_SURFACE", Self::BOX => "BOX", Self::POINTS => "POINTS", Self::DIRECTED_POINTS => "DIRECTED_POINTS", Self::RING => "RING", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EmissionShape::POINT, EmissionShape::SPHERE, EmissionShape::SPHERE_SURFACE, EmissionShape::BOX, EmissionShape::POINTS, EmissionShape::DIRECTED_POINTS, EmissionShape::RING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EmissionShape >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("POINT", "EMISSION_SHAPE_POINT", EmissionShape::POINT), crate::meta::inspect::EnumConstant::new("SPHERE", "EMISSION_SHAPE_SPHERE", EmissionShape::SPHERE), crate::meta::inspect::EnumConstant::new("SPHERE_SURFACE", "EMISSION_SHAPE_SPHERE_SURFACE", EmissionShape::SPHERE_SURFACE), crate::meta::inspect::EnumConstant::new("BOX", "EMISSION_SHAPE_BOX", EmissionShape::BOX), crate::meta::inspect::EnumConstant::new("POINTS", "EMISSION_SHAPE_POINTS", EmissionShape::POINTS), crate::meta::inspect::EnumConstant::new("DIRECTED_POINTS", "EMISSION_SHAPE_DIRECTED_POINTS", EmissionShape::DIRECTED_POINTS), crate::meta::inspect::EnumConstant::new("RING", "EMISSION_SHAPE_RING", EmissionShape::RING), crate::meta::inspect::EnumConstant::new("MAX", "EMISSION_SHAPE_MAX", EmissionShape::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for EmissionShape {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for EmissionShape {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EmissionShape {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EmissionShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SubEmitterMode {
    ord: i32
}
impl SubEmitterMode {
    #[doc(alias = "SUB_EMITTER_DISABLED")]
    #[doc = "Godot enumerator name: `SUB_EMITTER_DISABLED`"]
    pub const DISABLED: SubEmitterMode = SubEmitterMode {
        ord: 0i32
    };
    #[doc(alias = "SUB_EMITTER_CONSTANT")]
    #[doc = "Godot enumerator name: `SUB_EMITTER_CONSTANT`"]
    pub const CONSTANT: SubEmitterMode = SubEmitterMode {
        ord: 1i32
    };
    #[doc(alias = "SUB_EMITTER_AT_END")]
    #[doc = "Godot enumerator name: `SUB_EMITTER_AT_END`"]
    pub const AT_END: SubEmitterMode = SubEmitterMode {
        ord: 2i32
    };
    #[doc(alias = "SUB_EMITTER_AT_COLLISION")]
    #[doc = "Godot enumerator name: `SUB_EMITTER_AT_COLLISION`"]
    pub const AT_COLLISION: SubEmitterMode = SubEmitterMode {
        ord: 3i32
    };
    #[doc(alias = "SUB_EMITTER_AT_START")]
    #[doc = "Godot enumerator name: `SUB_EMITTER_AT_START`"]
    pub const AT_START: SubEmitterMode = SubEmitterMode {
        ord: 4i32
    };
    #[doc(alias = "SUB_EMITTER_MAX")]
    #[doc = "Godot enumerator name: `SUB_EMITTER_MAX`"]
    pub const MAX: SubEmitterMode = SubEmitterMode {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for SubEmitterMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SubEmitterMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SubEmitterMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::CONSTANT => "CONSTANT", Self::AT_END => "AT_END", Self::AT_COLLISION => "AT_COLLISION", Self::AT_START => "AT_START", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SubEmitterMode::DISABLED, SubEmitterMode::CONSTANT, SubEmitterMode::AT_END, SubEmitterMode::AT_COLLISION, SubEmitterMode::AT_START]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SubEmitterMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "SUB_EMITTER_DISABLED", SubEmitterMode::DISABLED), crate::meta::inspect::EnumConstant::new("CONSTANT", "SUB_EMITTER_CONSTANT", SubEmitterMode::CONSTANT), crate::meta::inspect::EnumConstant::new("AT_END", "SUB_EMITTER_AT_END", SubEmitterMode::AT_END), crate::meta::inspect::EnumConstant::new("AT_COLLISION", "SUB_EMITTER_AT_COLLISION", SubEmitterMode::AT_COLLISION), crate::meta::inspect::EnumConstant::new("AT_START", "SUB_EMITTER_AT_START", SubEmitterMode::AT_START), crate::meta::inspect::EnumConstant::new("MAX", "SUB_EMITTER_MAX", SubEmitterMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for SubEmitterMode {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for SubEmitterMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SubEmitterMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SubEmitterMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CollisionMode {
    ord: i32
}
impl CollisionMode {
    #[doc(alias = "COLLISION_DISABLED")]
    #[doc = "Godot enumerator name: `COLLISION_DISABLED`"]
    pub const DISABLED: CollisionMode = CollisionMode {
        ord: 0i32
    };
    #[doc(alias = "COLLISION_RIGID")]
    #[doc = "Godot enumerator name: `COLLISION_RIGID`"]
    pub const RIGID: CollisionMode = CollisionMode {
        ord: 1i32
    };
    #[doc(alias = "COLLISION_HIDE_ON_CONTACT")]
    #[doc = "Godot enumerator name: `COLLISION_HIDE_ON_CONTACT`"]
    pub const HIDE_ON_CONTACT: CollisionMode = CollisionMode {
        ord: 2i32
    };
    #[doc(alias = "COLLISION_MAX")]
    #[doc = "Godot enumerator name: `COLLISION_MAX`"]
    pub const MAX: CollisionMode = CollisionMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for CollisionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CollisionMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CollisionMode {
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
            Self::DISABLED => "DISABLED", Self::RIGID => "RIGID", Self::HIDE_ON_CONTACT => "HIDE_ON_CONTACT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CollisionMode::DISABLED, CollisionMode::RIGID, CollisionMode::HIDE_ON_CONTACT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CollisionMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "COLLISION_DISABLED", CollisionMode::DISABLED), crate::meta::inspect::EnumConstant::new("RIGID", "COLLISION_RIGID", CollisionMode::RIGID), crate::meta::inspect::EnumConstant::new("HIDE_ON_CONTACT", "COLLISION_HIDE_ON_CONTACT", CollisionMode::HIDE_ON_CONTACT), crate::meta::inspect::EnumConstant::new("MAX", "COLLISION_MAX", CollisionMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CollisionMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for CollisionMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CollisionMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CollisionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ParticleProcessMaterial;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`ParticleProcessMaterial`][crate::classes::ParticleProcessMaterial] class."]
    pub struct SignalsOfParticleProcessMaterial < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfParticleProcessMaterial < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn emission_shape_changed(&mut self) -> SigEmissionShapeChanged < 'c, C > {
            SigEmissionShapeChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "emission_shape_changed")
            }
        }
    }
    type TypedSigEmissionShapeChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigEmissionShapeChanged < 'c, C: WithSignals > {
        typed: TypedSigEmissionShapeChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigEmissionShapeChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigEmissionShapeChanged < 'c, C > {
        type Target = TypedSigEmissionShapeChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigEmissionShapeChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for ParticleProcessMaterial {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfParticleProcessMaterial < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfParticleProcessMaterial < 'c, C > {
        type Target = < < ParticleProcessMaterial as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = ParticleProcessMaterial;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfParticleProcessMaterial < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = ParticleProcessMaterial;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}