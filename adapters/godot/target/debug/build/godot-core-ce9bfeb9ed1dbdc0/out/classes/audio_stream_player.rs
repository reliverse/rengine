#![doc = "Sidecar module for class [`AudioStreamPlayer`][crate::classes::AudioStreamPlayer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamPlayer` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamPlayer.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`audio_stream_player`][crate::classes::audio_stream_player]: sidecar module with related enum/flag types\n* [`IAudioStreamPlayer`][crate::classes::IAudioStreamPlayer]: virtual methods\n* [`SignalsOfAudioStreamPlayer`][crate::classes::audio_stream_player::SignalsOfAudioStreamPlayer]: signal collection\n\n\nSee also [Godot docs for `AudioStreamPlayer`](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`AudioStreamPlayer::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamPlayer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AudioStreamPlayer`][crate::classes::AudioStreamPlayer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AudioStreamPlayer` methods](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStreamPlayer: crate::obj::GodotClass < Base = AudioStreamPlayer > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: NodeNotification) {
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
    impl AudioStreamPlayer {
        pub fn set_stream(&mut self, stream: impl AsArg < Option < Gd < crate::classes::AudioStream >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::AudioStream > > >,);
            let args = (stream.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(934usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "set_stream", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stream(&self,) -> Option < Gd < crate::classes::AudioStream > > {
            type CallRet = Option < Gd < crate::classes::AudioStream > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(935usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "get_stream", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volume_db(&mut self, volume_db: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (volume_db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(936usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "set_volume_db", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volume_db(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(937usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "get_volume_db", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volume_linear(&mut self, volume_linear: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (volume_linear,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(938usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "set_volume_linear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volume_linear(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(939usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "get_volume_linear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pitch_scale(&mut self, pitch_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (pitch_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(940usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "set_pitch_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pitch_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(941usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "get_pitch_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn play_full(&mut self, from_position: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (from_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(942usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "play", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::play_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn play(&mut self,) {
            self.play_ex() . done()
        }
        #[inline]
        pub fn play_ex < 'a > (&'a mut self,) -> ExPlay < 'a > {
            ExPlay::new(self,)
        }
        pub fn seek(&mut self, to_position: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(943usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "seek", Some(self.__validated_obj()), args,)
            }
        }
        pub fn stop(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(944usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "stop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_playing(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(945usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "is_playing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_playback_position(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(946usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "get_playback_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bus(&mut self, bus: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (bus.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(947usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "set_bus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bus(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(948usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "get_bus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autoplay(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(949usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "set_autoplay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_autoplay_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(950usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "is_autoplay_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mix_target(&mut self, mix_target: crate::classes::audio_stream_player::MixTarget,) {
            type CallRet = ();
            type CallParams = (crate::classes::audio_stream_player::MixTarget,);
            let args = (mix_target,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(951usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "set_mix_target", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mix_target(&self,) -> crate::classes::audio_stream_player::MixTarget {
            type CallRet = crate::classes::audio_stream_player::MixTarget;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(952usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "get_mix_target", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_playing(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(953usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "set_playing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stream_paused(&mut self, pause: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (pause,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(954usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "set_stream_paused", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stream_paused(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(955usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "get_stream_paused", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_polyphony(&mut self, max_polyphony: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_polyphony,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(956usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "set_max_polyphony", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_polyphony(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(957usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "get_max_polyphony", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_stream_playback(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(958usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "has_stream_playback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stream_playback(&mut self,) -> Option < Gd < crate::classes::AudioStreamPlayback > > {
            type CallRet = Option < Gd < crate::classes::AudioStreamPlayback > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(959usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "get_stream_playback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_playback_type(&mut self, playback_type: crate::classes::audio_server::PlaybackType,) {
            type CallRet = ();
            type CallParams = (crate::classes::audio_server::PlaybackType,);
            let args = (playback_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(960usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "set_playback_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_playback_type(&self,) -> crate::classes::audio_server::PlaybackType {
            type CallRet = crate::classes::audio_server::PlaybackType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(961usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer", "get_playback_type", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AudioStreamPlayer {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AudioStreamPlayer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamPlayer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for AudioStreamPlayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioStreamPlayer {
        
    }
    impl crate::obj::cap::GodotDefault for AudioStreamPlayer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioStreamPlayer {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamPlayer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioStreamPlayer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AudioStreamPlayer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioStreamPlayer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`AudioStreamPlayer::play_ex`][super::AudioStreamPlayer::play_ex]."]
#[must_use]
pub struct ExPlay < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AudioStreamPlayer, from_position: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlay < 'a > {
    fn new(surround_object: &'a mut re_export::AudioStreamPlayer,) -> Self {
        let from_position = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_position: from_position,
        }
    }
    #[inline]
    pub fn from_position(self, from_position: f32) -> Self {
        Self {
            from_position: from_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from_position,
        }
        = self;
        re_export::AudioStreamPlayer::play_full(surround_object, from_position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MixTarget {
    ord: i32
}
impl MixTarget {
    #[doc(alias = "MIX_TARGET_STEREO")]
    #[doc = "Godot enumerator name: `MIX_TARGET_STEREO`"]
    pub const STEREO: MixTarget = MixTarget {
        ord: 0i32
    };
    #[doc(alias = "MIX_TARGET_SURROUND")]
    #[doc = "Godot enumerator name: `MIX_TARGET_SURROUND`"]
    pub const SURROUND: MixTarget = MixTarget {
        ord: 1i32
    };
    #[doc(alias = "MIX_TARGET_CENTER")]
    #[doc = "Godot enumerator name: `MIX_TARGET_CENTER`"]
    pub const CENTER: MixTarget = MixTarget {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for MixTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MixTarget") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MixTarget {
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
            Self::STEREO => "STEREO", Self::SURROUND => "SURROUND", Self::CENTER => "CENTER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MixTarget::STEREO, MixTarget::SURROUND, MixTarget::CENTER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MixTarget >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STEREO", "MIX_TARGET_STEREO", MixTarget::STEREO), crate::meta::inspect::EnumConstant::new("SURROUND", "MIX_TARGET_SURROUND", MixTarget::SURROUND), crate::meta::inspect::EnumConstant::new("CENTER", "MIX_TARGET_CENTER", MixTarget::CENTER)]
        }
    }
}
impl crate::meta::GodotConvert for MixTarget {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MixTarget {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MixTarget {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AudioStreamPlayer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`AudioStreamPlayer`][crate::classes::AudioStreamPlayer] class."]
    pub struct SignalsOfAudioStreamPlayer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfAudioStreamPlayer < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn finished(&mut self) -> SigFinished < 'c, C > {
            SigFinished {
                typed: TypedSignal::extract(&mut self.__internal_obj, "finished")
            }
        }
    }
    type TypedSigFinished < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFinished < 'c, C: WithSignals > {
        typed: TypedSigFinished < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFinished < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFinished < 'c, C > {
        type Target = TypedSigFinished < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFinished < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for AudioStreamPlayer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAudioStreamPlayer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfAudioStreamPlayer < 'c, C > {
        type Target = < < AudioStreamPlayer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = AudioStreamPlayer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfAudioStreamPlayer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = AudioStreamPlayer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}