#![doc = "Sidecar module for class [`AnimationNodeStateMachinePlayback`][crate::classes::AnimationNodeStateMachinePlayback].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeStateMachinePlayback` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachineplayback.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNodeStateMachinePlayback.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`animation_node_state_machine_playback`][crate::classes::animation_node_state_machine_playback]: sidecar module with related enum/flag types\n* [`IAnimationNodeStateMachinePlayback`][crate::classes::IAnimationNodeStateMachinePlayback]: virtual methods\n* [`SignalsOfAnimationNodeStateMachinePlayback`][crate::classes::animation_node_state_machine_playback::SignalsOfAnimationNodeStateMachinePlayback]: signal collection\n\n\nSee also [Godot docs for `AnimationNodeStateMachinePlayback`](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachineplayback.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AnimationNodeStateMachinePlayback::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNodeStateMachinePlayback {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AnimationNodeStateMachinePlayback`][crate::classes::AnimationNodeStateMachinePlayback].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AnimationNodeStateMachinePlayback` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachineplayback.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNodeStateMachinePlayback: crate::obj::GodotClass < Base = AnimationNodeStateMachinePlayback > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AnimationNodeStateMachinePlayback {
        pub(crate) fn travel_full(&mut self, to_node: CowArg < StringName >, reset_on_teleport: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (to_node, reset_on_teleport,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(444usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachinePlayback", "travel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::travel_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn travel(&mut self, to_node: impl AsArg < StringName >,) {
            self.travel_ex(to_node,) . done()
        }
        #[inline]
        pub fn travel_ex < 'a > (&'a mut self, to_node: impl AsArg < StringName > + 'a,) -> ExTravel < 'a > {
            ExTravel::new(self, to_node,)
        }
        pub(crate) fn start_full(&mut self, node: CowArg < StringName >, reset: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (node, reset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(445usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachinePlayback", "start", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::start_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn start(&mut self, node: impl AsArg < StringName >,) {
            self.start_ex(node,) . done()
        }
        #[inline]
        pub fn start_ex < 'a > (&'a mut self, node: impl AsArg < StringName > + 'a,) -> ExStart < 'a > {
            ExStart::new(self, node,)
        }
        pub fn next(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(446usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachinePlayback", "next", Some(self.__validated_obj()), args,)
            }
        }
        pub fn stop(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(447usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachinePlayback", "stop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_playing(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(448usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachinePlayback", "is_playing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_node(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(449usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachinePlayback", "get_current_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_play_position(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(450usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachinePlayback", "get_current_play_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_length(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(451usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachinePlayback", "get_current_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fading_from_node(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(452usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachinePlayback", "get_fading_from_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_travel_path(&self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(453usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachinePlayback", "get_travel_path", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AnimationNodeStateMachinePlayback {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AnimationNodeStateMachinePlayback"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNodeStateMachinePlayback {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AnimationNodeStateMachinePlayback {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AnimationNodeStateMachinePlayback {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationNodeStateMachinePlayback {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNodeStateMachinePlayback {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNodeStateMachinePlayback {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNodeStateMachinePlayback {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimationNodeStateMachinePlayback`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AnimationNodeStateMachinePlayback__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationNodeStateMachinePlayback > for $Class {
                
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
#[doc = "Default-param extender for [`AnimationNodeStateMachinePlayback::travel_ex`][super::AnimationNodeStateMachinePlayback::travel_ex]."]
#[must_use]
pub struct ExTravel < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimationNodeStateMachinePlayback, to_node: CowArg < 'a, StringName >, reset_on_teleport: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTravel < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationNodeStateMachinePlayback, to_node: impl AsArg < StringName > + 'a,) -> Self {
        let reset_on_teleport = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_node: to_node.into_arg(), reset_on_teleport: reset_on_teleport,
        }
    }
    #[inline]
    pub fn reset_on_teleport(self, reset_on_teleport: bool) -> Self {
        Self {
            reset_on_teleport: reset_on_teleport, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, to_node, reset_on_teleport,
        }
        = self;
        re_export::AnimationNodeStateMachinePlayback::travel_full(surround_object, to_node, reset_on_teleport,)
    }
}
#[doc = "Default-param extender for [`AnimationNodeStateMachinePlayback::start_ex`][super::AnimationNodeStateMachinePlayback::start_ex]."]
#[must_use]
pub struct ExStart < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimationNodeStateMachinePlayback, node: CowArg < 'a, StringName >, reset: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStart < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationNodeStateMachinePlayback, node: impl AsArg < StringName > + 'a,) -> Self {
        let reset = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, node: node.into_arg(), reset: reset,
        }
    }
    #[inline]
    pub fn reset(self, reset: bool) -> Self {
        Self {
            reset: reset, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, node, reset,
        }
        = self;
        re_export::AnimationNodeStateMachinePlayback::start_full(surround_object, node, reset,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AnimationNodeStateMachinePlayback;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`AnimationNodeStateMachinePlayback`][crate::classes::AnimationNodeStateMachinePlayback] class."]
    pub struct SignalsOfAnimationNodeStateMachinePlayback < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfAnimationNodeStateMachinePlayback < 'c, C > {
        #[doc = "Signature: `(state: StringName)`"]
        pub fn state_started(&mut self) -> SigStateStarted < 'c, C > {
            SigStateStarted {
                typed: TypedSignal::extract(&mut self.__internal_obj, "state_started")
            }
        }
        #[doc = "Signature: `(state: StringName)`"]
        pub fn state_finished(&mut self) -> SigStateFinished < 'c, C > {
            SigStateFinished {
                typed: TypedSignal::extract(&mut self.__internal_obj, "state_finished")
            }
        }
    }
    type TypedSigStateStarted < 'c, C > = TypedSignal < 'c, C, (StringName,) >;
    pub struct SigStateStarted < 'c, C: WithSignals > {
        typed: TypedSigStateStarted < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigStateStarted < 'c, C > {
        pub fn emit(&mut self, state: StringName,) {
            self.typed.emit_tuple((state,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigStateStarted < 'c, C > {
        type Target = TypedSigStateStarted < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigStateStarted < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigStateFinished < 'c, C > = TypedSignal < 'c, C, (StringName,) >;
    pub struct SigStateFinished < 'c, C: WithSignals > {
        typed: TypedSigStateFinished < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigStateFinished < 'c, C > {
        pub fn emit(&mut self, state: StringName,) {
            self.typed.emit_tuple((state,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigStateFinished < 'c, C > {
        type Target = TypedSigStateFinished < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigStateFinished < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for AnimationNodeStateMachinePlayback {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAnimationNodeStateMachinePlayback < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfAnimationNodeStateMachinePlayback < 'c, C > {
        type Target = < < AnimationNodeStateMachinePlayback as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = AnimationNodeStateMachinePlayback;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfAnimationNodeStateMachinePlayback < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = AnimationNodeStateMachinePlayback;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}