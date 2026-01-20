#![doc = "Sidecar module for class [`NoiseTexture2D`][crate::classes::NoiseTexture2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `NoiseTexture2D` enums](https://docs.godotengine.org/en/stable/classes/class_noisetexture2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `NoiseTexture2D.`\n\nInherits [`Texture2D`][crate::classes::Texture2D].\n\nRelated symbols:\n\n* [`INoiseTexture2D`][crate::classes::INoiseTexture2D]: virtual methods\n\n\nSee also [Godot docs for `NoiseTexture2D`](https://docs.godotengine.org/en/stable/classes/class_noisetexture2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`NoiseTexture2D::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct NoiseTexture2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`NoiseTexture2D`][crate::classes::NoiseTexture2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`ITexture2D`][crate::classes::ITexture2D] > [`ITexture`][crate::classes::ITexture] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `NoiseTexture2D` methods](https://docs.godotengine.org/en/stable/classes/class_noisetexture2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait INoiseTexture2D: crate::obj::GodotClass < Base = NoiseTexture2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_width(&self,) -> i32;
        fn get_height(&self,) -> i32;
        fn is_pixel_opaque(&self, x: i32, y: i32,) -> bool {
            unimplemented !()
        }
        fn has_alpha(&self,) -> bool {
            unimplemented !()
        }
        fn draw(&self, to_canvas_item: Rid, pos: Vector2, modulate: Color, transpose: bool,) {
            unimplemented !()
        }
        fn draw_rect(&self, to_canvas_item: Rid, rect: Rect2, tile: bool, modulate: Color, transpose: bool,) {
            unimplemented !()
        }
        fn draw_rect_region(&self, to_canvas_item: Rid, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,) {
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
    impl NoiseTexture2D {
        pub fn set_width(&mut self, width: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5839usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_height(&mut self, height: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5840usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_generate_mipmaps(&mut self, invert: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (invert,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5841usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_generate_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_generating_mipmaps(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5842usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "is_generating_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_noise(&mut self, noise: impl AsArg < Option < Gd < crate::classes::Noise >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Noise > > >,);
            let args = (noise.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5843usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_noise", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_noise(&mut self,) -> Option < Gd < crate::classes::Noise > > {
            type CallRet = Option < Gd < crate::classes::Noise > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5844usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "get_noise", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color_ramp(&mut self, gradient: impl AsArg < Option < Gd < crate::classes::Gradient >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Gradient > > >,);
            let args = (gradient.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5845usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_color_ramp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color_ramp(&self,) -> Option < Gd < crate::classes::Gradient > > {
            type CallRet = Option < Gd < crate::classes::Gradient > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5846usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "get_color_ramp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_seamless(&mut self, seamless: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (seamless,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5847usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_seamless", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_seamless(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5848usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "get_seamless", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_invert(&mut self, invert: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (invert,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5849usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_invert", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_invert(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5850usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "get_invert", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_in_3d_space(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5851usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_in_3d_space", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_in_3d_space(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5852usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "is_in_3d_space", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_as_normal_map(&mut self, as_normal_map: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (as_normal_map,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5853usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_as_normal_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_normal_map(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5854usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "is_normal_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_normalize(&mut self, normalize: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (normalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5855usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_normalize", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_normalized(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5856usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "is_normalized", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_seamless_blend_skirt(&mut self, seamless_blend_skirt: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (seamless_blend_skirt,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5857usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_seamless_blend_skirt", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_seamless_blend_skirt(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5858usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "get_seamless_blend_skirt", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bump_strength(&mut self, bump_strength: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (bump_strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "set_bump_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bump_strength(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "NoiseTexture2D", "get_bump_strength", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for NoiseTexture2D {
        type Base = crate::classes::Texture2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("NoiseTexture2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for NoiseTexture2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture2D > for NoiseTexture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture > for NoiseTexture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for NoiseTexture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for NoiseTexture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for NoiseTexture2D {
        
    }
    impl crate::obj::cap::GodotDefault for NoiseTexture2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for NoiseTexture2D {
        type Target = crate::classes::Texture2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for NoiseTexture2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`NoiseTexture2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_NoiseTexture2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::NoiseTexture2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Texture2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Texture > for $Class {
                
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
    use super::re_export::NoiseTexture2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for NoiseTexture2D {
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