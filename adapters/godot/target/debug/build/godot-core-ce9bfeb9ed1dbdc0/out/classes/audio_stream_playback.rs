#![doc = "Sidecar module for class [`AudioStreamPlayback`][crate::classes::AudioStreamPlayback].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamPlayback` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayback.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamPlayback.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`audio_stream_playback`][crate::classes::audio_stream_playback]: sidecar module with related enum/flag types\n* [`IAudioStreamPlayback`][crate::classes::IAudioStreamPlayback]: virtual methods\n\n\nSee also [Godot docs for `AudioStreamPlayback`](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayback.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AudioStreamPlayback::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamPlayback {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AudioStreamPlayback`][crate::classes::AudioStreamPlayback].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AudioStreamPlayback` methods](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayback.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStreamPlayback: crate::obj::GodotClass < Base = AudioStreamPlayback > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn start(&mut self, from_pos: f64,) {
            unimplemented !()
        }
        fn stop(&mut self,) {
            unimplemented !()
        }
        fn is_playing(&self,) -> bool {
            unimplemented !()
        }
        fn get_loop_count(&self,) -> i32 {
            unimplemented !()
        }
        fn get_playback_position(&self,) -> f64 {
            unimplemented !()
        }
        fn seek(&mut self, position: f64,) {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn mix_rawptr(&mut self, buffer: * mut AudioFrame, rate_scale: f32, frames: i32,) -> i32;
        fn tag_used_streams(&mut self,) {
            unimplemented !()
        }
        fn set_parameter(&mut self, name: StringName, value: Variant,) {
            unimplemented !()
        }
        fn get_parameter(&self, name: StringName,) -> Variant {
            unimplemented !()
        }
    }
    impl AudioStreamPlayback {
        pub fn mix_audio(&mut self, rate_scale: f32, frames: i32,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = (f32, i32,);
            let args = (rate_scale, frames,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(918usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayback", "mix_audio", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn start_full(&mut self, from_pos: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (from_pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(919usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayback", "start", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::start_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn start(&mut self,) {
            self.start_ex() . done()
        }
        #[inline]
        pub fn start_ex < 'a > (&'a mut self,) -> ExStart < 'a > {
            ExStart::new(self,)
        }
        pub(crate) fn seek_full(&mut self, time: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(920usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayback", "seek", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::seek_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn seek(&mut self,) {
            self.seek_ex() . done()
        }
        #[inline]
        pub fn seek_ex < 'a > (&'a mut self,) -> ExSeek < 'a > {
            ExSeek::new(self,)
        }
        pub fn stop(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(921usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayback", "stop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_loop_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(922usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayback", "get_loop_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_playback_position(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(923usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayback", "get_playback_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_playing(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(924usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayback", "is_playing", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AudioStreamPlayback {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AudioStreamPlayback"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamPlayback {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioStreamPlayback {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioStreamPlayback {
        
    }
    impl crate::obj::cap::GodotDefault for AudioStreamPlayback {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioStreamPlayback {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamPlayback {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioStreamPlayback`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AudioStreamPlayback__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioStreamPlayback > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AudioStreamPlayback::start_ex`][super::AudioStreamPlayback::start_ex]."]
#[must_use]
pub struct ExStart < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AudioStreamPlayback, from_pos: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStart < 'a > {
    fn new(surround_object: &'a mut re_export::AudioStreamPlayback,) -> Self {
        let from_pos = 0f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_pos: from_pos,
        }
    }
    #[inline]
    pub fn from_pos(self, from_pos: f64) -> Self {
        Self {
            from_pos: from_pos, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from_pos,
        }
        = self;
        re_export::AudioStreamPlayback::start_full(surround_object, from_pos,)
    }
}
#[doc = "Default-param extender for [`AudioStreamPlayback::seek_ex`][super::AudioStreamPlayback::seek_ex]."]
#[must_use]
pub struct ExSeek < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AudioStreamPlayback, time: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSeek < 'a > {
    fn new(surround_object: &'a mut re_export::AudioStreamPlayback,) -> Self {
        let time = 0f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, time: time,
        }
    }
    #[inline]
    pub fn time(self, time: f64) -> Self {
        Self {
            time: time, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, time,
        }
        = self;
        re_export::AudioStreamPlayback::seek_full(surround_object, time,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AudioStreamPlayback;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for AudioStreamPlayback {
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