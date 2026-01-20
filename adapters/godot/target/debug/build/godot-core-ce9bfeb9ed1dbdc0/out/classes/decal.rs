#![doc = "Sidecar module for class [`Decal`][crate::classes::Decal].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Decal` enums](https://docs.godotengine.org/en/stable/classes/class_decal.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Decal.`\n\nInherits [`VisualInstance3D`][crate::classes::VisualInstance3D].\n\nRelated symbols:\n\n* [`decal`][crate::classes::decal]: sidecar module with related enum/flag types\n* [`IDecal`][crate::classes::IDecal]: virtual methods\n\n\nSee also [Godot docs for `Decal`](https://docs.godotengine.org/en/stable/classes/class_decal.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Decal::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Decal {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Decal`][crate::classes::Decal].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Decal` methods](https://docs.godotengine.org/en/stable/classes/class_decal.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IDecal: crate::obj::GodotClass < Base = Decal > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Decal {
        pub fn set_size(&mut self, size: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2817usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2818usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture(&mut self, type_: crate::classes::decal::DecalTexture, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::decal::DecalTexture, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (type_, texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2819usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "set_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture(&self, type_: crate::classes::decal::DecalTexture,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (crate::classes::decal::DecalTexture,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2820usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "get_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_energy(&mut self, energy: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2821usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "set_emission_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_energy(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2822usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "get_emission_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_albedo_mix(&mut self, energy: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2823usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "set_albedo_mix", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_albedo_mix(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2824usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "get_albedo_mix", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_modulate(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2825usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "set_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_modulate(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2826usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "get_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_upper_fade(&mut self, fade: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (fade,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2827usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "set_upper_fade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_upper_fade(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2828usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "get_upper_fade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lower_fade(&mut self, fade: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (fade,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2829usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "set_lower_fade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lower_fade(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2830usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "get_lower_fade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_normal_fade(&mut self, fade: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (fade,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2831usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "set_normal_fade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_normal_fade(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2832usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "get_normal_fade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enable_distance_fade(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2833usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "set_enable_distance_fade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_distance_fade_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2834usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "is_distance_fade_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_distance_fade_begin(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2835usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "set_distance_fade_begin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_distance_fade_begin(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2836usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "get_distance_fade_begin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_distance_fade_length(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2837usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "set_distance_fade_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_distance_fade_length(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2838usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "get_distance_fade_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cull_mask(&mut self, mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2839usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "set_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cull_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2840usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Decal", "get_cull_mask", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Decal {
        type Base = crate::classes::VisualInstance3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Decal"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Decal {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for Decal {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for Decal {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Decal {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Decal {
        
    }
    impl crate::obj::cap::GodotDefault for Decal {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Decal {
        type Target = crate::classes::VisualInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Decal {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Decal`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Decal__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Decal > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DecalTexture {
    ord: i32
}
impl DecalTexture {
    #[doc(alias = "TEXTURE_ALBEDO")]
    #[doc = "Godot enumerator name: `TEXTURE_ALBEDO`"]
    pub const ALBEDO: DecalTexture = DecalTexture {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_NORMAL")]
    #[doc = "Godot enumerator name: `TEXTURE_NORMAL`"]
    pub const NORMAL: DecalTexture = DecalTexture {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_ORM")]
    #[doc = "Godot enumerator name: `TEXTURE_ORM`"]
    pub const ORM: DecalTexture = DecalTexture {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_EMISSION")]
    #[doc = "Godot enumerator name: `TEXTURE_EMISSION`"]
    pub const EMISSION: DecalTexture = DecalTexture {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_MAX")]
    #[doc = "Godot enumerator name: `TEXTURE_MAX`"]
    pub const MAX: DecalTexture = DecalTexture {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for DecalTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DecalTexture") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DecalTexture {
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
            Self::ALBEDO => "ALBEDO", Self::NORMAL => "NORMAL", Self::ORM => "ORM", Self::EMISSION => "EMISSION", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DecalTexture::ALBEDO, DecalTexture::NORMAL, DecalTexture::ORM, DecalTexture::EMISSION]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DecalTexture >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ALBEDO", "TEXTURE_ALBEDO", DecalTexture::ALBEDO), crate::meta::inspect::EnumConstant::new("NORMAL", "TEXTURE_NORMAL", DecalTexture::NORMAL), crate::meta::inspect::EnumConstant::new("ORM", "TEXTURE_ORM", DecalTexture::ORM), crate::meta::inspect::EnumConstant::new("EMISSION", "TEXTURE_EMISSION", DecalTexture::EMISSION), crate::meta::inspect::EnumConstant::new("MAX", "TEXTURE_MAX", DecalTexture::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for DecalTexture {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for DecalTexture {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DecalTexture {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DecalTexture {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Decal;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for Decal {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}