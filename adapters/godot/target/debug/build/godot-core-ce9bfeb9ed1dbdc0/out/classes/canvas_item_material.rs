#![doc = "Sidecar module for class [`CanvasItemMaterial`][crate::classes::CanvasItemMaterial].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CanvasItemMaterial` enums](https://docs.godotengine.org/en/stable/classes/class_canvasitemmaterial.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CanvasItemMaterial.`\n\nInherits [`Material`][crate::classes::Material].\n\nRelated symbols:\n\n* [`canvas_item_material`][crate::classes::canvas_item_material]: sidecar module with related enum/flag types\n* [`ICanvasItemMaterial`][crate::classes::ICanvasItemMaterial]: virtual methods\n\n\nSee also [Godot docs for `CanvasItemMaterial`](https://docs.godotengine.org/en/stable/classes/class_canvasitemmaterial.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`CanvasItemMaterial::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CanvasItemMaterial {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`CanvasItemMaterial`][crate::classes::CanvasItemMaterial].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IMaterial`][crate::classes::IMaterial] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `CanvasItemMaterial` methods](https://docs.godotengine.org/en/stable/classes/class_canvasitemmaterial.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICanvasItemMaterial: crate::obj::GodotClass < Base = CanvasItemMaterial > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl CanvasItemMaterial {
        pub fn set_blend_mode(&mut self, blend_mode: crate::classes::canvas_item_material::BlendMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::canvas_item_material::BlendMode,);
            let args = (blend_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1906usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItemMaterial", "set_blend_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_blend_mode(&self,) -> crate::classes::canvas_item_material::BlendMode {
            type CallRet = crate::classes::canvas_item_material::BlendMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1907usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItemMaterial", "get_blend_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_light_mode(&mut self, light_mode: crate::classes::canvas_item_material::LightMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::canvas_item_material::LightMode,);
            let args = (light_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1908usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItemMaterial", "set_light_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_light_mode(&self,) -> crate::classes::canvas_item_material::LightMode {
            type CallRet = crate::classes::canvas_item_material::LightMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1909usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItemMaterial", "get_light_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_particles_animation(&mut self, particles_anim: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (particles_anim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1910usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItemMaterial", "set_particles_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_particles_animation(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1911usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItemMaterial", "get_particles_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_particles_anim_h_frames(&mut self, frames: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1912usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItemMaterial", "set_particles_anim_h_frames", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_particles_anim_h_frames(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1913usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItemMaterial", "get_particles_anim_h_frames", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_particles_anim_v_frames(&mut self, frames: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1914usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItemMaterial", "set_particles_anim_v_frames", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_particles_anim_v_frames(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1915usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItemMaterial", "get_particles_anim_v_frames", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_particles_anim_loop(&mut self, loop_: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (loop_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1916usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItemMaterial", "set_particles_anim_loop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_particles_anim_loop(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1917usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CanvasItemMaterial", "get_particles_anim_loop", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CanvasItemMaterial {
        type Base = crate::classes::Material;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CanvasItemMaterial"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CanvasItemMaterial {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Material > for CanvasItemMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for CanvasItemMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for CanvasItemMaterial {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CanvasItemMaterial {
        
    }
    impl crate::obj::cap::GodotDefault for CanvasItemMaterial {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CanvasItemMaterial {
        type Target = crate::classes::Material;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CanvasItemMaterial {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CanvasItemMaterial`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CanvasItemMaterial__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItemMaterial > for $Class {
                
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
pub struct BlendMode {
    ord: i32
}
impl BlendMode {
    #[doc(alias = "BLEND_MODE_MIX")]
    #[doc = "Godot enumerator name: `BLEND_MODE_MIX`"]
    pub const MIX: BlendMode = BlendMode {
        ord: 0i32
    };
    #[doc(alias = "BLEND_MODE_ADD")]
    #[doc = "Godot enumerator name: `BLEND_MODE_ADD`"]
    pub const ADD: BlendMode = BlendMode {
        ord: 1i32
    };
    #[doc(alias = "BLEND_MODE_SUB")]
    #[doc = "Godot enumerator name: `BLEND_MODE_SUB`"]
    pub const SUB: BlendMode = BlendMode {
        ord: 2i32
    };
    #[doc(alias = "BLEND_MODE_MUL")]
    #[doc = "Godot enumerator name: `BLEND_MODE_MUL`"]
    pub const MUL: BlendMode = BlendMode {
        ord: 3i32
    };
    #[doc(alias = "BLEND_MODE_PREMULT_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_MODE_PREMULT_ALPHA`"]
    pub const PREMULT_ALPHA: BlendMode = BlendMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for BlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BlendMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BlendMode {
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
            Self::MIX => "MIX", Self::ADD => "ADD", Self::SUB => "SUB", Self::MUL => "MUL", Self::PREMULT_ALPHA => "PREMULT_ALPHA", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BlendMode::MIX, BlendMode::ADD, BlendMode::SUB, BlendMode::MUL, BlendMode::PREMULT_ALPHA]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BlendMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("MIX", "BLEND_MODE_MIX", BlendMode::MIX), crate::meta::inspect::EnumConstant::new("ADD", "BLEND_MODE_ADD", BlendMode::ADD), crate::meta::inspect::EnumConstant::new("SUB", "BLEND_MODE_SUB", BlendMode::SUB), crate::meta::inspect::EnumConstant::new("MUL", "BLEND_MODE_MUL", BlendMode::MUL), crate::meta::inspect::EnumConstant::new("PREMULT_ALPHA", "BLEND_MODE_PREMULT_ALPHA", BlendMode::PREMULT_ALPHA)]
        }
    }
}
impl crate::meta::GodotConvert for BlendMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BlendMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightMode {
    ord: i32
}
impl LightMode {
    #[doc(alias = "LIGHT_MODE_NORMAL")]
    #[doc = "Godot enumerator name: `LIGHT_MODE_NORMAL`"]
    pub const NORMAL: LightMode = LightMode {
        ord: 0i32
    };
    #[doc(alias = "LIGHT_MODE_UNSHADED")]
    #[doc = "Godot enumerator name: `LIGHT_MODE_UNSHADED`"]
    pub const UNSHADED: LightMode = LightMode {
        ord: 1i32
    };
    #[doc(alias = "LIGHT_MODE_LIGHT_ONLY")]
    #[doc = "Godot enumerator name: `LIGHT_MODE_LIGHT_ONLY`"]
    pub const LIGHT_ONLY: LightMode = LightMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for LightMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
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
            Self::NORMAL => "NORMAL", Self::UNSHADED => "UNSHADED", Self::LIGHT_ONLY => "LIGHT_ONLY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightMode::NORMAL, LightMode::UNSHADED, LightMode::LIGHT_ONLY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NORMAL", "LIGHT_MODE_NORMAL", LightMode::NORMAL), crate::meta::inspect::EnumConstant::new("UNSHADED", "LIGHT_MODE_UNSHADED", LightMode::UNSHADED), crate::meta::inspect::EnumConstant::new("LIGHT_ONLY", "LIGHT_MODE_LIGHT_ONLY", LightMode::LIGHT_ONLY)]
        }
    }
}
impl crate::meta::GodotConvert for LightMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LightMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CanvasItemMaterial;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for CanvasItemMaterial {
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