#![doc = "Sidecar module for class [`AnimationNodeAnimation`][crate::classes::AnimationNodeAnimation].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeAnimation` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodeanimation.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNodeAnimation.`\n\nInherits [`AnimationRootNode`][crate::classes::AnimationRootNode].\n\nRelated symbols:\n\n* [`animation_node_animation`][crate::classes::animation_node_animation]: sidecar module with related enum/flag types\n* [`IAnimationNodeAnimation`][crate::classes::IAnimationNodeAnimation]: virtual methods\n\n\nSee also [Godot docs for `AnimationNodeAnimation`](https://docs.godotengine.org/en/stable/classes/class_animationnodeanimation.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AnimationNodeAnimation::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNodeAnimation {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AnimationNodeAnimation`][crate::classes::AnimationNodeAnimation].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IAnimationRootNode`][crate::classes::IAnimationRootNode] > [`IAnimationNode`][crate::classes::IAnimationNode] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AnimationNodeAnimation` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodeanimation.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNodeAnimation: crate::obj::GodotClass < Base = AnimationNodeAnimation > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_child_nodes(&self,) -> VarDictionary {
            unimplemented !()
        }
        fn get_parameter_list(&self,) -> VarArray {
            unimplemented !()
        }
        fn get_child_by_name(&self, name: StringName,) -> Option < Gd < crate::classes::AnimationNode > > {
            unimplemented !()
        }
        fn get_parameter_default_value(&self, parameter: StringName,) -> Variant {
            unimplemented !()
        }
        fn is_parameter_read_only(&self, parameter: StringName,) -> bool {
            unimplemented !()
        }
        fn process(&mut self, time: f64, seek: bool, is_external_seeking: bool, test_only: bool,) -> f64 {
            unimplemented !()
        }
        fn get_caption(&self,) -> GString {
            unimplemented !()
        }
        fn has_filter(&self,) -> bool {
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
    impl AnimationNodeAnimation {
        pub fn set_animation(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeAnimation", "set_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_animation(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeAnimation", "get_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_play_mode(&mut self, mode: crate::classes::animation_node_animation::PlayMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation_node_animation::PlayMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeAnimation", "set_play_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_play_mode(&self,) -> crate::classes::animation_node_animation::PlayMode {
            type CallRet = crate::classes::animation_node_animation::PlayMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeAnimation", "get_play_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_advance_on_start(&mut self, advance_on_start: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (advance_on_start,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeAnimation", "set_advance_on_start", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_advance_on_start(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeAnimation", "is_advance_on_start", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_custom_timeline(&mut self, use_custom_timeline: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (use_custom_timeline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeAnimation", "set_use_custom_timeline", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_custom_timeline(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeAnimation", "is_using_custom_timeline", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_timeline_length(&mut self, timeline_length: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (timeline_length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeAnimation", "set_timeline_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_timeline_length(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeAnimation", "get_timeline_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stretch_time_scale(&mut self, stretch_time_scale: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (stretch_time_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeAnimation", "set_stretch_time_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_stretching_time_scale(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeAnimation", "is_stretching_time_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_start_offset(&mut self, start_offset: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (start_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeAnimation", "set_start_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_start_offset(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeAnimation", "get_start_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_loop_mode(&mut self, loop_mode: crate::classes::animation::LoopMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation::LoopMode,);
            let args = (loop_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeAnimation", "set_loop_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_loop_mode(&self,) -> crate::classes::animation::LoopMode {
            type CallRet = crate::classes::animation::LoopMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeAnimation", "get_loop_mode", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AnimationNodeAnimation {
        type Base = crate::classes::AnimationRootNode;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AnimationNodeAnimation"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNodeAnimation {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationRootNode > for AnimationNodeAnimation {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationNode > for AnimationNodeAnimation {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AnimationNodeAnimation {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AnimationNodeAnimation {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationNodeAnimation {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNodeAnimation {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNodeAnimation {
        type Target = crate::classes::AnimationRootNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNodeAnimation {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimationNodeAnimation`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AnimationNodeAnimation__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationNodeAnimation > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationRootNode > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationNode > for $Class {
                
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
pub struct PlayMode {
    ord: i32
}
impl PlayMode {
    #[doc(alias = "PLAY_MODE_FORWARD")]
    #[doc = "Godot enumerator name: `PLAY_MODE_FORWARD`"]
    pub const FORWARD: PlayMode = PlayMode {
        ord: 0i32
    };
    #[doc(alias = "PLAY_MODE_BACKWARD")]
    #[doc = "Godot enumerator name: `PLAY_MODE_BACKWARD`"]
    pub const BACKWARD: PlayMode = PlayMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for PlayMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PlayMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PlayMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::FORWARD => "FORWARD", Self::BACKWARD => "BACKWARD", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PlayMode::FORWARD, PlayMode::BACKWARD]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PlayMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FORWARD", "PLAY_MODE_FORWARD", PlayMode::FORWARD), crate::meta::inspect::EnumConstant::new("BACKWARD", "PLAY_MODE_BACKWARD", PlayMode::BACKWARD)]
        }
    }
}
impl crate::meta::GodotConvert for PlayMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PlayMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PlayMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AnimationNodeAnimation;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::animation_node::SignalsOfAnimationNode;
    impl WithSignals for AnimationNodeAnimation {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAnimationNode < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}