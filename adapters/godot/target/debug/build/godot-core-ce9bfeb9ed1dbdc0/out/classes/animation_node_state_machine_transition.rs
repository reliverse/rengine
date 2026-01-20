#![doc = "Sidecar module for class [`AnimationNodeStateMachineTransition`][crate::classes::AnimationNodeStateMachineTransition].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeStateMachineTransition` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachinetransition.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNodeStateMachineTransition.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`animation_node_state_machine_transition`][crate::classes::animation_node_state_machine_transition]: sidecar module with related enum/flag types\n* [`IAnimationNodeStateMachineTransition`][crate::classes::IAnimationNodeStateMachineTransition]: virtual methods\n* [`SignalsOfAnimationNodeStateMachineTransition`][crate::classes::animation_node_state_machine_transition::SignalsOfAnimationNodeStateMachineTransition]: signal collection\n\n\nSee also [Godot docs for `AnimationNodeStateMachineTransition`](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachinetransition.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AnimationNodeStateMachineTransition::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNodeStateMachineTransition {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AnimationNodeStateMachineTransition`][crate::classes::AnimationNodeStateMachineTransition].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AnimationNodeStateMachineTransition` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachinetransition.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNodeStateMachineTransition: crate::obj::GodotClass < Base = AnimationNodeStateMachineTransition > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AnimationNodeStateMachineTransition {
        pub fn set_switch_mode(&mut self, mode: crate::classes::animation_node_state_machine_transition::SwitchMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation_node_state_machine_transition::SwitchMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(454usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_switch_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_switch_mode(&self,) -> crate::classes::animation_node_state_machine_transition::SwitchMode {
            type CallRet = crate::classes::animation_node_state_machine_transition::SwitchMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(455usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "get_switch_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_advance_mode(&mut self, mode: crate::classes::animation_node_state_machine_transition::AdvanceMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation_node_state_machine_transition::AdvanceMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(456usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_advance_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_advance_mode(&self,) -> crate::classes::animation_node_state_machine_transition::AdvanceMode {
            type CallRet = crate::classes::animation_node_state_machine_transition::AdvanceMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(457usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "get_advance_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_advance_condition(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(458usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_advance_condition", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_advance_condition(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(459usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "get_advance_condition", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_xfade_time(&mut self, secs: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (secs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(460usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_xfade_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_xfade_time(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(461usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "get_xfade_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_xfade_curve(&mut self, curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Curve > > >,);
            let args = (curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(462usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_xfade_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_xfade_curve(&self,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(463usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "get_xfade_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_break_loop_at_end(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(464usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_break_loop_at_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_loop_broken_at_end(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(465usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "is_loop_broken_at_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_reset(&mut self, reset: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (reset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(466usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_reset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_reset(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(467usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "is_reset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_priority(&mut self, priority: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(468usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_priority(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(469usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "get_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_advance_expression(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(470usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "set_advance_expression", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_advance_expression(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(471usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachineTransition", "get_advance_expression", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AnimationNodeStateMachineTransition {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AnimationNodeStateMachineTransition"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNodeStateMachineTransition {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AnimationNodeStateMachineTransition {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AnimationNodeStateMachineTransition {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationNodeStateMachineTransition {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNodeStateMachineTransition {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNodeStateMachineTransition {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNodeStateMachineTransition {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimationNodeStateMachineTransition`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AnimationNodeStateMachineTransition__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationNodeStateMachineTransition > for $Class {
                
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
pub struct SwitchMode {
    ord: i32
}
impl SwitchMode {
    #[doc(alias = "SWITCH_MODE_IMMEDIATE")]
    #[doc = "Godot enumerator name: `SWITCH_MODE_IMMEDIATE`"]
    pub const IMMEDIATE: SwitchMode = SwitchMode {
        ord: 0i32
    };
    #[doc(alias = "SWITCH_MODE_SYNC")]
    #[doc = "Godot enumerator name: `SWITCH_MODE_SYNC`"]
    pub const SYNC: SwitchMode = SwitchMode {
        ord: 1i32
    };
    #[doc(alias = "SWITCH_MODE_AT_END")]
    #[doc = "Godot enumerator name: `SWITCH_MODE_AT_END`"]
    pub const AT_END: SwitchMode = SwitchMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for SwitchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SwitchMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SwitchMode {
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
            Self::IMMEDIATE => "IMMEDIATE", Self::SYNC => "SYNC", Self::AT_END => "AT_END", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SwitchMode::IMMEDIATE, SwitchMode::SYNC, SwitchMode::AT_END]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SwitchMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("IMMEDIATE", "SWITCH_MODE_IMMEDIATE", SwitchMode::IMMEDIATE), crate::meta::inspect::EnumConstant::new("SYNC", "SWITCH_MODE_SYNC", SwitchMode::SYNC), crate::meta::inspect::EnumConstant::new("AT_END", "SWITCH_MODE_AT_END", SwitchMode::AT_END)]
        }
    }
}
impl crate::meta::GodotConvert for SwitchMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SwitchMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SwitchMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AdvanceMode {
    ord: i32
}
impl AdvanceMode {
    #[doc(alias = "ADVANCE_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `ADVANCE_MODE_DISABLED`"]
    pub const DISABLED: AdvanceMode = AdvanceMode {
        ord: 0i32
    };
    #[doc(alias = "ADVANCE_MODE_ENABLED")]
    #[doc = "Godot enumerator name: `ADVANCE_MODE_ENABLED`"]
    pub const ENABLED: AdvanceMode = AdvanceMode {
        ord: 1i32
    };
    #[doc(alias = "ADVANCE_MODE_AUTO")]
    #[doc = "Godot enumerator name: `ADVANCE_MODE_AUTO`"]
    pub const AUTO: AdvanceMode = AdvanceMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AdvanceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AdvanceMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AdvanceMode {
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
            Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::AUTO => "AUTO", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AdvanceMode::DISABLED, AdvanceMode::ENABLED, AdvanceMode::AUTO]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AdvanceMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "ADVANCE_MODE_DISABLED", AdvanceMode::DISABLED), crate::meta::inspect::EnumConstant::new("ENABLED", "ADVANCE_MODE_ENABLED", AdvanceMode::ENABLED), crate::meta::inspect::EnumConstant::new("AUTO", "ADVANCE_MODE_AUTO", AdvanceMode::AUTO)]
        }
    }
}
impl crate::meta::GodotConvert for AdvanceMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AdvanceMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AdvanceMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AnimationNodeStateMachineTransition;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`AnimationNodeStateMachineTransition`][crate::classes::AnimationNodeStateMachineTransition] class."]
    pub struct SignalsOfAnimationNodeStateMachineTransition < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfAnimationNodeStateMachineTransition < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn advance_condition_changed(&mut self) -> SigAdvanceConditionChanged < 'c, C > {
            SigAdvanceConditionChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "advance_condition_changed")
            }
        }
    }
    type TypedSigAdvanceConditionChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigAdvanceConditionChanged < 'c, C: WithSignals > {
        typed: TypedSigAdvanceConditionChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAdvanceConditionChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAdvanceConditionChanged < 'c, C > {
        type Target = TypedSigAdvanceConditionChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAdvanceConditionChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for AnimationNodeStateMachineTransition {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAnimationNodeStateMachineTransition < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfAnimationNodeStateMachineTransition < 'c, C > {
        type Target = < < AnimationNodeStateMachineTransition as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = AnimationNodeStateMachineTransition;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfAnimationNodeStateMachineTransition < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = AnimationNodeStateMachineTransition;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}