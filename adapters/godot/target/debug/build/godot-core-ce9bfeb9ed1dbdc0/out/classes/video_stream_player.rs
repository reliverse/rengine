#![doc = "Sidecar module for class [`VideoStreamPlayer`][crate::classes::VideoStreamPlayer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VideoStreamPlayer` enums](https://docs.godotengine.org/en/stable/classes/class_videostreamplayer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VideoStreamPlayer.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`video_stream_player`][crate::classes::video_stream_player]: sidecar module with related enum/flag types\n* [`IVideoStreamPlayer`][crate::classes::IVideoStreamPlayer]: virtual methods\n* [`SignalsOfVideoStreamPlayer`][crate::classes::video_stream_player::SignalsOfVideoStreamPlayer]: signal collection\n\n\nSee also [Godot docs for `VideoStreamPlayer`](https://docs.godotengine.org/en/stable/classes/class_videostreamplayer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`VideoStreamPlayer::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VideoStreamPlayer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`VideoStreamPlayer`][crate::classes::VideoStreamPlayer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `VideoStreamPlayer` methods](https://docs.godotengine.org/en/stable/classes/class_videostreamplayer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVideoStreamPlayer: crate::obj::GodotClass < Base = VideoStreamPlayer > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ControlNotification) {
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
        fn has_point(&self, point: Vector2,) -> bool {
            unimplemented !()
        }
        fn structured_text_parser(&self, args: VarArray, text: GString,) -> Array < Vector3i > {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_tooltip(&self, at_position: Vector2,) -> GString {
            unimplemented !()
        }
        fn get_drag_data(&mut self, at_position: Vector2,) -> Variant {
            unimplemented !()
        }
        fn can_drop_data(&self, at_position: Vector2, data: Variant,) -> bool {
            unimplemented !()
        }
        fn drop_data(&mut self, at_position: Vector2, data: Variant,) {
            unimplemented !()
        }
        fn make_custom_tooltip(&self, for_text: GString,) -> Option < Gd < crate::classes::Object > > {
            unimplemented !()
        }
        fn accessibility_get_contextual_info(&self,) -> GString {
            unimplemented !()
        }
        fn get_accessibility_container_name(&self, node: Option < Gd < crate::classes::Node > >,) -> GString {
            unimplemented !()
        }
        fn gui_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
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
    impl VideoStreamPlayer {
        pub fn set_stream(&mut self, stream: impl AsArg < Option < Gd < crate::classes::VideoStream >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::VideoStream > > >,);
            let args = (stream.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "set_stream", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stream(&self,) -> Option < Gd < crate::classes::VideoStream > > {
            type CallRet = Option < Gd < crate::classes::VideoStream > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "get_stream", Some(self.__validated_obj()), args,)
            }
        }
        pub fn play(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "play", Some(self.__validated_obj()), args,)
            }
        }
        pub fn stop(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "stop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_playing(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "is_playing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_paused(&mut self, paused: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (paused,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "set_paused", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_paused(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "is_paused", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_loop(&mut self, loop_: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (loop_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "set_loop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_loop(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "has_loop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volume(&mut self, volume: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (volume,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "set_volume", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volume(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "get_volume", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_volume_db(&mut self, db: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "set_volume_db", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_volume_db(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "get_volume_db", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_speed_scale(&mut self, speed_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (speed_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "set_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_speed_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10420usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "get_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_audio_track(&mut self, track: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (track,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10421usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "set_audio_track", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_audio_track(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10422usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "get_audio_track", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stream_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10423usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "get_stream_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stream_length(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10424usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "get_stream_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stream_position(&mut self, position: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10425usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "set_stream_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stream_position(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10426usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "get_stream_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autoplay(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10427usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "set_autoplay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_autoplay(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10428usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "has_autoplay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_expand(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10429usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "set_expand", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_expand(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10430usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "has_expand", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_buffering_msec(&mut self, msec: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (msec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10431usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "set_buffering_msec", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_buffering_msec(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10432usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "get_buffering_msec", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bus(&mut self, bus: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (bus.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10433usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "set_bus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bus(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10434usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "get_bus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_video_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10435usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayer", "get_video_texture", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for VideoStreamPlayer {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VideoStreamPlayer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VideoStreamPlayer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for VideoStreamPlayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for VideoStreamPlayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for VideoStreamPlayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VideoStreamPlayer {
        
    }
    impl crate::obj::cap::GodotDefault for VideoStreamPlayer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VideoStreamPlayer {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VideoStreamPlayer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VideoStreamPlayer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VideoStreamPlayer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VideoStreamPlayer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Control > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
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
    use super::re_export::VideoStreamPlayer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`VideoStreamPlayer`][crate::classes::VideoStreamPlayer] class."]
    pub struct SignalsOfVideoStreamPlayer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfVideoStreamPlayer < 'c, C > {
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
    impl WithSignals for VideoStreamPlayer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfVideoStreamPlayer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfVideoStreamPlayer < 'c, C > {
        type Target = < < VideoStreamPlayer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = VideoStreamPlayer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfVideoStreamPlayer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = VideoStreamPlayer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}