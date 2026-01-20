#![doc = "Sidecar module for class [`AudioStreamInteractive`][crate::classes::AudioStreamInteractive].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamInteractive` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreaminteractive.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamInteractive.`\n\nInherits [`AudioStream`][crate::classes::AudioStream].\n\nRelated symbols:\n\n* [`audio_stream_interactive`][crate::classes::audio_stream_interactive]: sidecar module with related enum/flag types\n* [`IAudioStreamInteractive`][crate::classes::IAudioStreamInteractive]: virtual methods\n\n\nSee also [Godot docs for `AudioStreamInteractive`](https://docs.godotengine.org/en/stable/classes/class_audiostreaminteractive.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AudioStreamInteractive::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamInteractive {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AudioStreamInteractive`][crate::classes::AudioStreamInteractive].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IAudioStream`][crate::classes::IAudioStream] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AudioStreamInteractive` methods](https://docs.godotengine.org/en/stable/classes/class_audiostreaminteractive.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStreamInteractive: crate::obj::GodotClass < Base = AudioStreamInteractive > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn instantiate_playback(&self,) -> Option < Gd < crate::classes::AudioStreamPlayback > > {
            unimplemented !()
        }
        fn get_stream_name(&self,) -> GString {
            unimplemented !()
        }
        fn get_length(&self,) -> f64 {
            unimplemented !()
        }
        fn is_monophonic(&self,) -> bool {
            unimplemented !()
        }
        fn get_bpm(&self,) -> f64 {
            unimplemented !()
        }
        fn get_beat_count(&self,) -> i32 {
            unimplemented !()
        }
        fn get_tags(&self,) -> VarDictionary {
            unimplemented !()
        }
        fn get_parameter_list(&self,) -> Array < VarDictionary > {
            unimplemented !()
        }
        fn has_loop(&self,) -> bool {
            unimplemented !()
        }
        fn get_bar_beats(&self,) -> i32 {
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
    impl AudioStreamInteractive {
        pub fn set_clip_count(&mut self, clip_count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (clip_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(865usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "set_clip_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_clip_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(866usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_clip_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_initial_clip(&mut self, clip_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (clip_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(867usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "set_initial_clip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_initial_clip(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(868usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_initial_clip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_clip_name(&mut self, clip_index: i32, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, StringName >,);
            let args = (clip_index, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(869usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "set_clip_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_clip_name(&self, clip_index: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (clip_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(870usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_clip_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_clip_stream(&mut self, clip_index: i32, stream: impl AsArg < Option < Gd < crate::classes::AudioStream >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::AudioStream > > >,);
            let args = (clip_index, stream.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(871usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "set_clip_stream", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_clip_stream(&self, clip_index: i32,) -> Option < Gd < crate::classes::AudioStream > > {
            type CallRet = Option < Gd < crate::classes::AudioStream > >;
            type CallParams = (i32,);
            let args = (clip_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(872usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_clip_stream", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_clip_auto_advance(&mut self, clip_index: i32, mode: crate::classes::audio_stream_interactive::AutoAdvanceMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::audio_stream_interactive::AutoAdvanceMode,);
            let args = (clip_index, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(873usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "set_clip_auto_advance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_clip_auto_advance(&self, clip_index: i32,) -> crate::classes::audio_stream_interactive::AutoAdvanceMode {
            type CallRet = crate::classes::audio_stream_interactive::AutoAdvanceMode;
            type CallParams = (i32,);
            let args = (clip_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(874usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_clip_auto_advance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_clip_auto_advance_next_clip(&mut self, clip_index: i32, auto_advance_next_clip: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (clip_index, auto_advance_next_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(875usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "set_clip_auto_advance_next_clip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_clip_auto_advance_next_clip(&self, clip_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (clip_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(876usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_clip_auto_advance_next_clip", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_transition_full(&mut self, from_clip: i32, to_clip: i32, from_time: crate::classes::audio_stream_interactive::TransitionFromTime, to_time: crate::classes::audio_stream_interactive::TransitionToTime, fade_mode: crate::classes::audio_stream_interactive::FadeMode, fade_beats: f32, use_filler_clip: bool, filler_clip: i32, hold_previous: bool,) {
            type CallRet = ();
            type CallParams = (i32, i32, crate::classes::audio_stream_interactive::TransitionFromTime, crate::classes::audio_stream_interactive::TransitionToTime, crate::classes::audio_stream_interactive::FadeMode, f32, bool, i32, bool,);
            let args = (from_clip, to_clip, from_time, to_time, fade_mode, fade_beats, use_filler_clip, filler_clip, hold_previous,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(877usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "add_transition", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_transition_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_transition(&mut self, from_clip: i32, to_clip: i32, from_time: crate::classes::audio_stream_interactive::TransitionFromTime, to_time: crate::classes::audio_stream_interactive::TransitionToTime, fade_mode: crate::classes::audio_stream_interactive::FadeMode, fade_beats: f32,) {
            self.add_transition_ex(from_clip, to_clip, from_time, to_time, fade_mode, fade_beats,) . done()
        }
        #[inline]
        pub fn add_transition_ex < 'a > (&'a mut self, from_clip: i32, to_clip: i32, from_time: crate::classes::audio_stream_interactive::TransitionFromTime, to_time: crate::classes::audio_stream_interactive::TransitionToTime, fade_mode: crate::classes::audio_stream_interactive::FadeMode, fade_beats: f32,) -> ExAddTransition < 'a > {
            ExAddTransition::new(self, from_clip, to_clip, from_time, to_time, fade_mode, fade_beats,)
        }
        pub fn has_transition(&self, from_clip: i32, to_clip: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, i32,);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(878usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "has_transition", Some(self.__validated_obj()), args,)
            }
        }
        pub fn erase_transition(&mut self, from_clip: i32, to_clip: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(879usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "erase_transition", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transition_list(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(880usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_transition_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transition_from_time(&self, from_clip: i32, to_clip: i32,) -> crate::classes::audio_stream_interactive::TransitionFromTime {
            type CallRet = crate::classes::audio_stream_interactive::TransitionFromTime;
            type CallParams = (i32, i32,);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(881usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_transition_from_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transition_to_time(&self, from_clip: i32, to_clip: i32,) -> crate::classes::audio_stream_interactive::TransitionToTime {
            type CallRet = crate::classes::audio_stream_interactive::TransitionToTime;
            type CallParams = (i32, i32,);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(882usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_transition_to_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transition_fade_mode(&self, from_clip: i32, to_clip: i32,) -> crate::classes::audio_stream_interactive::FadeMode {
            type CallRet = crate::classes::audio_stream_interactive::FadeMode;
            type CallParams = (i32, i32,);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(883usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_transition_fade_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transition_fade_beats(&self, from_clip: i32, to_clip: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(884usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_transition_fade_beats", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_transition_using_filler_clip(&self, from_clip: i32, to_clip: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, i32,);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(885usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "is_transition_using_filler_clip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transition_filler_clip(&self, from_clip: i32, to_clip: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(886usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "get_transition_filler_clip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_transition_holding_previous(&self, from_clip: i32, to_clip: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, i32,);
            let args = (from_clip, to_clip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(887usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamInteractive", "is_transition_holding_previous", Some(self.__validated_obj()), args,)
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
        pub const CLIP_ANY: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for AudioStreamInteractive {
        type Base = crate::classes::AudioStream;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AudioStreamInteractive"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamInteractive {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioStream > for AudioStreamInteractive {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AudioStreamInteractive {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioStreamInteractive {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioStreamInteractive {
        
    }
    impl crate::obj::cap::GodotDefault for AudioStreamInteractive {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioStreamInteractive {
        type Target = crate::classes::AudioStream;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamInteractive {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioStreamInteractive`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AudioStreamInteractive__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioStreamInteractive > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioStream > for $Class {
                
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
#[doc = "Default-param extender for [`AudioStreamInteractive::add_transition_ex`][super::AudioStreamInteractive::add_transition_ex]."]
#[must_use]
pub struct ExAddTransition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AudioStreamInteractive, from_clip: i32, to_clip: i32, from_time: crate::classes::audio_stream_interactive::TransitionFromTime, to_time: crate::classes::audio_stream_interactive::TransitionToTime, fade_mode: crate::classes::audio_stream_interactive::FadeMode, fade_beats: f32, use_filler_clip: bool, filler_clip: i32, hold_previous: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTransition < 'a > {
    fn new(surround_object: &'a mut re_export::AudioStreamInteractive, from_clip: i32, to_clip: i32, from_time: crate::classes::audio_stream_interactive::TransitionFromTime, to_time: crate::classes::audio_stream_interactive::TransitionToTime, fade_mode: crate::classes::audio_stream_interactive::FadeMode, fade_beats: f32,) -> Self {
        let use_filler_clip = false;
        let filler_clip = - 1i32;
        let hold_previous = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_clip: from_clip, to_clip: to_clip, from_time: from_time, to_time: to_time, fade_mode: fade_mode, fade_beats: fade_beats, use_filler_clip: use_filler_clip, filler_clip: filler_clip, hold_previous: hold_previous,
        }
    }
    #[inline]
    pub fn use_filler_clip(self, use_filler_clip: bool) -> Self {
        Self {
            use_filler_clip: use_filler_clip, .. self
        }
    }
    #[inline]
    pub fn filler_clip(self, filler_clip: i32) -> Self {
        Self {
            filler_clip: filler_clip, .. self
        }
    }
    #[inline]
    pub fn hold_previous(self, hold_previous: bool) -> Self {
        Self {
            hold_previous: hold_previous, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from_clip, to_clip, from_time, to_time, fade_mode, fade_beats, use_filler_clip, filler_clip, hold_previous,
        }
        = self;
        re_export::AudioStreamInteractive::add_transition_full(surround_object, from_clip, to_clip, from_time, to_time, fade_mode, fade_beats, use_filler_clip, filler_clip, hold_previous,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TransitionFromTime {
    ord: i32
}
impl TransitionFromTime {
    #[doc(alias = "TRANSITION_FROM_TIME_IMMEDIATE")]
    #[doc = "Godot enumerator name: `TRANSITION_FROM_TIME_IMMEDIATE`"]
    pub const IMMEDIATE: TransitionFromTime = TransitionFromTime {
        ord: 0i32
    };
    #[doc(alias = "TRANSITION_FROM_TIME_NEXT_BEAT")]
    #[doc = "Godot enumerator name: `TRANSITION_FROM_TIME_NEXT_BEAT`"]
    pub const NEXT_BEAT: TransitionFromTime = TransitionFromTime {
        ord: 1i32
    };
    #[doc(alias = "TRANSITION_FROM_TIME_NEXT_BAR")]
    #[doc = "Godot enumerator name: `TRANSITION_FROM_TIME_NEXT_BAR`"]
    pub const NEXT_BAR: TransitionFromTime = TransitionFromTime {
        ord: 2i32
    };
    #[doc(alias = "TRANSITION_FROM_TIME_END")]
    #[doc = "Godot enumerator name: `TRANSITION_FROM_TIME_END`"]
    pub const END: TransitionFromTime = TransitionFromTime {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TransitionFromTime {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TransitionFromTime") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TransitionFromTime {
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
            Self::IMMEDIATE => "IMMEDIATE", Self::NEXT_BEAT => "NEXT_BEAT", Self::NEXT_BAR => "NEXT_BAR", Self::END => "END", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TransitionFromTime::IMMEDIATE, TransitionFromTime::NEXT_BEAT, TransitionFromTime::NEXT_BAR, TransitionFromTime::END]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TransitionFromTime >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("IMMEDIATE", "TRANSITION_FROM_TIME_IMMEDIATE", TransitionFromTime::IMMEDIATE), crate::meta::inspect::EnumConstant::new("NEXT_BEAT", "TRANSITION_FROM_TIME_NEXT_BEAT", TransitionFromTime::NEXT_BEAT), crate::meta::inspect::EnumConstant::new("NEXT_BAR", "TRANSITION_FROM_TIME_NEXT_BAR", TransitionFromTime::NEXT_BAR), crate::meta::inspect::EnumConstant::new("END", "TRANSITION_FROM_TIME_END", TransitionFromTime::END)]
        }
    }
}
impl crate::meta::GodotConvert for TransitionFromTime {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TransitionFromTime {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TransitionFromTime {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TransitionToTime {
    ord: i32
}
impl TransitionToTime {
    #[doc(alias = "TRANSITION_TO_TIME_SAME_POSITION")]
    #[doc = "Godot enumerator name: `TRANSITION_TO_TIME_SAME_POSITION`"]
    pub const SAME_POSITION: TransitionToTime = TransitionToTime {
        ord: 0i32
    };
    #[doc(alias = "TRANSITION_TO_TIME_START")]
    #[doc = "Godot enumerator name: `TRANSITION_TO_TIME_START`"]
    pub const START: TransitionToTime = TransitionToTime {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for TransitionToTime {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TransitionToTime") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TransitionToTime {
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
            Self::SAME_POSITION => "SAME_POSITION", Self::START => "START", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TransitionToTime::SAME_POSITION, TransitionToTime::START]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TransitionToTime >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SAME_POSITION", "TRANSITION_TO_TIME_SAME_POSITION", TransitionToTime::SAME_POSITION), crate::meta::inspect::EnumConstant::new("START", "TRANSITION_TO_TIME_START", TransitionToTime::START)]
        }
    }
}
impl crate::meta::GodotConvert for TransitionToTime {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TransitionToTime {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TransitionToTime {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FadeMode {
    ord: i32
}
impl FadeMode {
    #[doc(alias = "FADE_DISABLED")]
    #[doc = "Godot enumerator name: `FADE_DISABLED`"]
    pub const DISABLED: FadeMode = FadeMode {
        ord: 0i32
    };
    #[doc(alias = "FADE_IN")]
    #[doc = "Godot enumerator name: `FADE_IN`"]
    pub const IN: FadeMode = FadeMode {
        ord: 1i32
    };
    #[doc(alias = "FADE_OUT")]
    #[doc = "Godot enumerator name: `FADE_OUT`"]
    pub const OUT: FadeMode = FadeMode {
        ord: 2i32
    };
    #[doc(alias = "FADE_CROSS")]
    #[doc = "Godot enumerator name: `FADE_CROSS`"]
    pub const CROSS: FadeMode = FadeMode {
        ord: 3i32
    };
    #[doc(alias = "FADE_AUTOMATIC")]
    #[doc = "Godot enumerator name: `FADE_AUTOMATIC`"]
    pub const AUTOMATIC: FadeMode = FadeMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for FadeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FadeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FadeMode {
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
            Self::DISABLED => "DISABLED", Self::IN => "IN", Self::OUT => "OUT", Self::CROSS => "CROSS", Self::AUTOMATIC => "AUTOMATIC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FadeMode::DISABLED, FadeMode::IN, FadeMode::OUT, FadeMode::CROSS, FadeMode::AUTOMATIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FadeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "FADE_DISABLED", FadeMode::DISABLED), crate::meta::inspect::EnumConstant::new("IN", "FADE_IN", FadeMode::IN), crate::meta::inspect::EnumConstant::new("OUT", "FADE_OUT", FadeMode::OUT), crate::meta::inspect::EnumConstant::new("CROSS", "FADE_CROSS", FadeMode::CROSS), crate::meta::inspect::EnumConstant::new("AUTOMATIC", "FADE_AUTOMATIC", FadeMode::AUTOMATIC)]
        }
    }
}
impl crate::meta::GodotConvert for FadeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FadeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FadeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AutoAdvanceMode {
    ord: i32
}
impl AutoAdvanceMode {
    #[doc(alias = "AUTO_ADVANCE_DISABLED")]
    #[doc = "Godot enumerator name: `AUTO_ADVANCE_DISABLED`"]
    pub const DISABLED: AutoAdvanceMode = AutoAdvanceMode {
        ord: 0i32
    };
    #[doc(alias = "AUTO_ADVANCE_ENABLED")]
    #[doc = "Godot enumerator name: `AUTO_ADVANCE_ENABLED`"]
    pub const ENABLED: AutoAdvanceMode = AutoAdvanceMode {
        ord: 1i32
    };
    #[doc(alias = "AUTO_ADVANCE_RETURN_TO_HOLD")]
    #[doc = "Godot enumerator name: `AUTO_ADVANCE_RETURN_TO_HOLD`"]
    pub const RETURN_TO_HOLD: AutoAdvanceMode = AutoAdvanceMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AutoAdvanceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AutoAdvanceMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AutoAdvanceMode {
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
            Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::RETURN_TO_HOLD => "RETURN_TO_HOLD", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AutoAdvanceMode::DISABLED, AutoAdvanceMode::ENABLED, AutoAdvanceMode::RETURN_TO_HOLD]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AutoAdvanceMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "AUTO_ADVANCE_DISABLED", AutoAdvanceMode::DISABLED), crate::meta::inspect::EnumConstant::new("ENABLED", "AUTO_ADVANCE_ENABLED", AutoAdvanceMode::ENABLED), crate::meta::inspect::EnumConstant::new("RETURN_TO_HOLD", "AUTO_ADVANCE_RETURN_TO_HOLD", AutoAdvanceMode::RETURN_TO_HOLD)]
        }
    }
}
impl crate::meta::GodotConvert for AutoAdvanceMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AutoAdvanceMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AutoAdvanceMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AudioStreamInteractive;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::audio_stream::SignalsOfAudioStream;
    impl WithSignals for AudioStreamInteractive {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAudioStream < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}