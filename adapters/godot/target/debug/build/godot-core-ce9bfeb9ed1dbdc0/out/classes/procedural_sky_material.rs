#![doc = "Sidecar module for class [`ProceduralSkyMaterial`][crate::classes::ProceduralSkyMaterial].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ProceduralSkyMaterial` enums](https://docs.godotengine.org/en/stable/classes/class_proceduralskymaterial.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ProceduralSkyMaterial.`\n\nInherits [`Material`][crate::classes::Material].\n\nRelated symbols:\n\n* [`IProceduralSkyMaterial`][crate::classes::IProceduralSkyMaterial]: virtual methods\n\n\nSee also [Godot docs for `ProceduralSkyMaterial`](https://docs.godotengine.org/en/stable/classes/class_proceduralskymaterial.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ProceduralSkyMaterial::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ProceduralSkyMaterial {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ProceduralSkyMaterial`][crate::classes::ProceduralSkyMaterial].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IMaterial`][crate::classes::IMaterial] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `ProceduralSkyMaterial` methods](https://docs.godotengine.org/en/stable/classes/class_proceduralskymaterial.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IProceduralSkyMaterial: crate::obj::GodotClass < Base = ProceduralSkyMaterial > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ProceduralSkyMaterial {
        pub fn set_sky_top_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6858usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "set_sky_top_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sky_top_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "get_sky_top_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sky_horizon_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "set_sky_horizon_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sky_horizon_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6861usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "get_sky_horizon_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sky_curve(&mut self, curve: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (curve,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6862usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "set_sky_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sky_curve(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6863usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "get_sky_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sky_energy_multiplier(&mut self, multiplier: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (multiplier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6864usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "set_sky_energy_multiplier", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sky_energy_multiplier(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6865usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "get_sky_energy_multiplier", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sky_cover(&mut self, sky_cover: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (sky_cover.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6866usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "set_sky_cover", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sky_cover(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6867usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "get_sky_cover", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sky_cover_modulate(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6868usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "set_sky_cover_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sky_cover_modulate(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6869usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "get_sky_cover_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ground_bottom_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6870usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "set_ground_bottom_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ground_bottom_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6871usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "get_ground_bottom_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ground_horizon_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6872usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "set_ground_horizon_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ground_horizon_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6873usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "get_ground_horizon_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ground_curve(&mut self, curve: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (curve,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6874usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "set_ground_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ground_curve(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6875usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "get_ground_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ground_energy_multiplier(&mut self, energy: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6876usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "set_ground_energy_multiplier", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ground_energy_multiplier(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6877usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "get_ground_energy_multiplier", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sun_angle_max(&mut self, degrees: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6878usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "set_sun_angle_max", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sun_angle_max(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6879usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "get_sun_angle_max", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sun_curve(&mut self, curve: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (curve,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6880usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "set_sun_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sun_curve(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6881usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "get_sun_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_debanding(&mut self, use_debanding: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (use_debanding,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6882usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "set_use_debanding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_use_debanding(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6883usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "get_use_debanding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_energy_multiplier(&mut self, multiplier: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (multiplier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6884usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "set_energy_multiplier", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_energy_multiplier(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6885usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProceduralSkyMaterial", "get_energy_multiplier", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ProceduralSkyMaterial {
        type Base = crate::classes::Material;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ProceduralSkyMaterial"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ProceduralSkyMaterial {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Material > for ProceduralSkyMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for ProceduralSkyMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ProceduralSkyMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ProceduralSkyMaterial {
        
    }
    impl crate::obj::cap::GodotDefault for ProceduralSkyMaterial {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ProceduralSkyMaterial {
        type Target = crate::classes::Material;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ProceduralSkyMaterial {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ProceduralSkyMaterial`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ProceduralSkyMaterial__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ProceduralSkyMaterial > for $Class {
                
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ProceduralSkyMaterial;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for ProceduralSkyMaterial {
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