#![doc = "Sidecar module for class [`AudioStreamPlayer3D`][crate::classes::AudioStreamPlayer3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamPlayer3D` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayer3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamPlayer3D.`\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`audio_stream_player_3d`][crate::classes::audio_stream_player_3d]: sidecar module with related enum/flag types\n* [`IAudioStreamPlayer3D`][crate::classes::IAudioStreamPlayer3D]: virtual methods\n* [`SignalsOfAudioStreamPlayer3D`][crate::classes::audio_stream_player_3d::SignalsOfAudioStreamPlayer3D]: signal collection\n\n\nSee also [Godot docs for `AudioStreamPlayer3D`](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayer3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`AudioStreamPlayer3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamPlayer3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AudioStreamPlayer3D`][crate::classes::AudioStreamPlayer3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AudioStreamPlayer3D` methods](https://docs.godotengine.org/en/stable/classes/class_audiostreamplayer3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStreamPlayer3D: crate::obj::GodotClass < Base = AudioStreamPlayer3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AudioStreamPlayer3D {
        pub fn set_stream(&mut self, stream: impl AsArg < Option < Gd < crate::classes::AudioStream >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::AudioStream > > >,);
            let args = (stream.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(996usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_stream", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stream(&self,) -> Option < Gd < crate::classes::AudioStream > > {
            type CallRet = Option < Gd < crate::classes::AudioStream > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(997usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_stream", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volume_db(&mut self, volume_db: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (volume_db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(998usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_volume_db", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volume_db(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(999usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_volume_db", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volume_linear(&mut self, volume_linear: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (volume_linear,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1000usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_volume_linear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volume_linear(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1001usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_volume_linear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_unit_size(&mut self, unit_size: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (unit_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1002usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_unit_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_unit_size(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1003usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_unit_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_db(&mut self, max_db: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (max_db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1004usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_max_db", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_db(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1005usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_max_db", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pitch_scale(&mut self, pitch_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (pitch_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1006usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_pitch_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pitch_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1007usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_pitch_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn play_full(&mut self, from_position: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (from_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1008usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "play", Some(self.__validated_obj()), args,)
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
                let method_bind = sys::class_scene_api() . fptr_by_index(1009usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "seek", Some(self.__validated_obj()), args,)
            }
        }
        pub fn stop(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1010usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "stop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_playing(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1011usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "is_playing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_playback_position(&mut self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1012usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_playback_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bus(&mut self, bus: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (bus.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1013usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_bus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bus(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1014usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_bus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autoplay(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1015usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_autoplay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_autoplay_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1016usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "is_autoplay_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_playing(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1017usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_playing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_distance(&mut self, meters: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (meters,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1018usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_max_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_distance(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1019usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_max_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_area_mask(&mut self, mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1020usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_area_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_area_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1021usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_area_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_angle(&mut self, degrees: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1022usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_emission_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_angle(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1023usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_emission_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_angle_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1024usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_emission_angle_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_emission_angle_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1025usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "is_emission_angle_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_angle_filter_attenuation_db(&mut self, db: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1026usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_emission_angle_filter_attenuation_db", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_angle_filter_attenuation_db(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1027usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_emission_angle_filter_attenuation_db", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_attenuation_filter_cutoff_hz(&mut self, degrees: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1028usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_attenuation_filter_cutoff_hz", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_attenuation_filter_cutoff_hz(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1029usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_attenuation_filter_cutoff_hz", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_attenuation_filter_db(&mut self, db: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1030usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_attenuation_filter_db", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_attenuation_filter_db(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1031usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_attenuation_filter_db", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_attenuation_model(&mut self, model: crate::classes::audio_stream_player_3d::AttenuationModel,) {
            type CallRet = ();
            type CallParams = (crate::classes::audio_stream_player_3d::AttenuationModel,);
            let args = (model,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1032usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_attenuation_model", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_attenuation_model(&self,) -> crate::classes::audio_stream_player_3d::AttenuationModel {
            type CallRet = crate::classes::audio_stream_player_3d::AttenuationModel;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1033usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_attenuation_model", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_doppler_tracking(&mut self, mode: crate::classes::audio_stream_player_3d::DopplerTracking,) {
            type CallRet = ();
            type CallParams = (crate::classes::audio_stream_player_3d::DopplerTracking,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1034usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_doppler_tracking", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_doppler_tracking(&self,) -> crate::classes::audio_stream_player_3d::DopplerTracking {
            type CallRet = crate::classes::audio_stream_player_3d::DopplerTracking;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1035usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_doppler_tracking", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stream_paused(&mut self, pause: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (pause,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1036usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_stream_paused", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stream_paused(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1037usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_stream_paused", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_polyphony(&mut self, max_polyphony: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_polyphony,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1038usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_max_polyphony", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_polyphony(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1039usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_max_polyphony", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_panning_strength(&mut self, panning_strength: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (panning_strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1040usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_panning_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_panning_strength(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1041usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_panning_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_stream_playback(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1042usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "has_stream_playback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stream_playback(&mut self,) -> Option < Gd < crate::classes::AudioStreamPlayback > > {
            type CallRet = Option < Gd < crate::classes::AudioStreamPlayback > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1043usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_stream_playback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_playback_type(&mut self, playback_type: crate::classes::audio_server::PlaybackType,) {
            type CallRet = ();
            type CallParams = (crate::classes::audio_server::PlaybackType,);
            let args = (playback_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1044usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "set_playback_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_playback_type(&self,) -> crate::classes::audio_server::PlaybackType {
            type CallRet = crate::classes::audio_server::PlaybackType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1045usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlayer3D", "get_playback_type", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AudioStreamPlayer3D {
        type Base = crate::classes::Node3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AudioStreamPlayer3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamPlayer3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for AudioStreamPlayer3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for AudioStreamPlayer3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioStreamPlayer3D {
        
    }
    impl crate::obj::cap::GodotDefault for AudioStreamPlayer3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioStreamPlayer3D {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamPlayer3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioStreamPlayer3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AudioStreamPlayer3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioStreamPlayer3D > for $Class {
                
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
#[doc = "Default-param extender for [`AudioStreamPlayer3D::play_ex`][super::AudioStreamPlayer3D::play_ex]."]
#[must_use]
pub struct ExPlay < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AudioStreamPlayer3D, from_position: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlay < 'a > {
    fn new(surround_object: &'a mut re_export::AudioStreamPlayer3D,) -> Self {
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
        re_export::AudioStreamPlayer3D::play_full(surround_object, from_position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AttenuationModel {
    ord: i32
}
impl AttenuationModel {
    #[doc(alias = "ATTENUATION_INVERSE_DISTANCE")]
    #[doc = "Godot enumerator name: `ATTENUATION_INVERSE_DISTANCE`"]
    pub const INVERSE_DISTANCE: AttenuationModel = AttenuationModel {
        ord: 0i32
    };
    #[doc(alias = "ATTENUATION_INVERSE_SQUARE_DISTANCE")]
    #[doc = "Godot enumerator name: `ATTENUATION_INVERSE_SQUARE_DISTANCE`"]
    pub const INVERSE_SQUARE_DISTANCE: AttenuationModel = AttenuationModel {
        ord: 1i32
    };
    #[doc(alias = "ATTENUATION_LOGARITHMIC")]
    #[doc = "Godot enumerator name: `ATTENUATION_LOGARITHMIC`"]
    pub const LOGARITHMIC: AttenuationModel = AttenuationModel {
        ord: 2i32
    };
    #[doc(alias = "ATTENUATION_DISABLED")]
    #[doc = "Godot enumerator name: `ATTENUATION_DISABLED`"]
    pub const DISABLED: AttenuationModel = AttenuationModel {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for AttenuationModel {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AttenuationModel") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AttenuationModel {
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
            Self::INVERSE_DISTANCE => "INVERSE_DISTANCE", Self::INVERSE_SQUARE_DISTANCE => "INVERSE_SQUARE_DISTANCE", Self::LOGARITHMIC => "LOGARITHMIC", Self::DISABLED => "DISABLED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AttenuationModel::INVERSE_DISTANCE, AttenuationModel::INVERSE_SQUARE_DISTANCE, AttenuationModel::LOGARITHMIC, AttenuationModel::DISABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AttenuationModel >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INVERSE_DISTANCE", "ATTENUATION_INVERSE_DISTANCE", AttenuationModel::INVERSE_DISTANCE), crate::meta::inspect::EnumConstant::new("INVERSE_SQUARE_DISTANCE", "ATTENUATION_INVERSE_SQUARE_DISTANCE", AttenuationModel::INVERSE_SQUARE_DISTANCE), crate::meta::inspect::EnumConstant::new("LOGARITHMIC", "ATTENUATION_LOGARITHMIC", AttenuationModel::LOGARITHMIC), crate::meta::inspect::EnumConstant::new("DISABLED", "ATTENUATION_DISABLED", AttenuationModel::DISABLED)]
        }
    }
}
impl crate::meta::GodotConvert for AttenuationModel {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AttenuationModel {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AttenuationModel {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DopplerTracking {
    ord: i32
}
impl DopplerTracking {
    #[doc(alias = "DOPPLER_TRACKING_DISABLED")]
    #[doc = "Godot enumerator name: `DOPPLER_TRACKING_DISABLED`"]
    pub const DISABLED: DopplerTracking = DopplerTracking {
        ord: 0i32
    };
    #[doc(alias = "DOPPLER_TRACKING_IDLE_STEP")]
    #[doc = "Godot enumerator name: `DOPPLER_TRACKING_IDLE_STEP`"]
    pub const IDLE_STEP: DopplerTracking = DopplerTracking {
        ord: 1i32
    };
    #[doc(alias = "DOPPLER_TRACKING_PHYSICS_STEP")]
    #[doc = "Godot enumerator name: `DOPPLER_TRACKING_PHYSICS_STEP`"]
    pub const PHYSICS_STEP: DopplerTracking = DopplerTracking {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DopplerTracking {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DopplerTracking") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DopplerTracking {
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
            Self::DISABLED => "DISABLED", Self::IDLE_STEP => "IDLE_STEP", Self::PHYSICS_STEP => "PHYSICS_STEP", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DopplerTracking::DISABLED, DopplerTracking::IDLE_STEP, DopplerTracking::PHYSICS_STEP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DopplerTracking >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "DOPPLER_TRACKING_DISABLED", DopplerTracking::DISABLED), crate::meta::inspect::EnumConstant::new("IDLE_STEP", "DOPPLER_TRACKING_IDLE_STEP", DopplerTracking::IDLE_STEP), crate::meta::inspect::EnumConstant::new("PHYSICS_STEP", "DOPPLER_TRACKING_PHYSICS_STEP", DopplerTracking::PHYSICS_STEP)]
        }
    }
}
impl crate::meta::GodotConvert for DopplerTracking {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DopplerTracking {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DopplerTracking {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AudioStreamPlayer3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`AudioStreamPlayer3D`][crate::classes::AudioStreamPlayer3D] class."]
    pub struct SignalsOfAudioStreamPlayer3D < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfAudioStreamPlayer3D < 'c, C > {
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
    impl WithSignals for AudioStreamPlayer3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAudioStreamPlayer3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfAudioStreamPlayer3D < 'c, C > {
        type Target = < < AudioStreamPlayer3D as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = AudioStreamPlayer3D;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfAudioStreamPlayer3D < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = AudioStreamPlayer3D;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}