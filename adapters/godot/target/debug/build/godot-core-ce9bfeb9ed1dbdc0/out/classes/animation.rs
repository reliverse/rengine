#![doc = "Sidecar module for class [`Animation`][crate::classes::Animation].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Animation` enums](https://docs.godotengine.org/en/stable/classes/class_animation.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Animation.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`animation`][crate::classes::animation]: sidecar module with related enum/flag types\n* [`IAnimation`][crate::classes::IAnimation]: virtual methods\n\n\nSee also [Godot docs for `Animation`](https://docs.godotengine.org/en/stable/classes/class_animation.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Animation::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Animation {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Animation`][crate::classes::Animation].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Animation` methods](https://docs.godotengine.org/en/stable/classes/class_animation.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimation: crate::obj::GodotClass < Base = Animation > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Animation {
        pub(crate) fn add_track_full(&mut self, type_: crate::classes::animation::TrackType, at_position: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (crate::classes::animation::TrackType, i32,);
            let args = (type_, at_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "add_track", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_track_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_track(&mut self, type_: crate::classes::animation::TrackType,) -> i32 {
            self.add_track_ex(type_,) . done()
        }
        #[inline]
        pub fn add_track_ex < 'a > (&'a mut self, type_: crate::classes::animation::TrackType,) -> ExAddTrack < 'a > {
            ExAddTrack::new(self, type_,)
        }
        pub fn remove_track(&mut self, track_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "remove_track", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_track_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "get_track_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_get_type(&self, track_idx: i32,) -> crate::classes::animation::TrackType {
            type CallRet = crate::classes::animation::TrackType;
            type CallParams = (i32,);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_get_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_get_path(&self, track_idx: i32,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = (i32,);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_get_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_set_path(&mut self, track_idx: i32, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, NodePath >,);
            let args = (track_idx, path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_set_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn find_track(&self, path: impl AsArg < NodePath >, type_: crate::classes::animation::TrackType,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >, crate::classes::animation::TrackType,);
            let args = (path.into_arg(), type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "find_track", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_move_up(&mut self, track_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_move_up", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_move_down(&mut self, track_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_move_down", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_move_to(&mut self, track_idx: i32, to_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (track_idx, to_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_move_to", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_swap(&mut self, track_idx: i32, with_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (track_idx, with_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_swap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_set_imported(&mut self, track_idx: i32, imported: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (track_idx, imported,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_set_imported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_is_imported(&self, track_idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_is_imported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_set_enabled(&mut self, track_idx: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (track_idx, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_set_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_is_enabled(&self, track_idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_is_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn position_track_insert_key(&mut self, track_idx: i32, time: f64, position: Vector3,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, f64, Vector3,);
            let args = (track_idx, time, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "position_track_insert_key", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rotation_track_insert_key(&mut self, track_idx: i32, time: f64, rotation: Quaternion,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, f64, Quaternion,);
            let args = (track_idx, time, rotation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "rotation_track_insert_key", Some(self.__validated_obj()), args,)
            }
        }
        pub fn scale_track_insert_key(&mut self, track_idx: i32, time: f64, scale: Vector3,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, f64, Vector3,);
            let args = (track_idx, time, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "scale_track_insert_key", Some(self.__validated_obj()), args,)
            }
        }
        pub fn blend_shape_track_insert_key(&mut self, track_idx: i32, time: f64, amount: f32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, f64, f32,);
            let args = (track_idx, time, amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "blend_shape_track_insert_key", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn position_track_interpolate_full(&self, track_idx: i32, time_sec: f64, backward: bool,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32, f64, bool,);
            let args = (track_idx, time_sec, backward,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "position_track_interpolate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::position_track_interpolate_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn position_track_interpolate(&self, track_idx: i32, time_sec: f64,) -> Vector3 {
            self.position_track_interpolate_ex(track_idx, time_sec,) . done()
        }
        #[inline]
        pub fn position_track_interpolate_ex < 'a > (&'a self, track_idx: i32, time_sec: f64,) -> ExPositionTrackInterpolate < 'a > {
            ExPositionTrackInterpolate::new(self, track_idx, time_sec,)
        }
        pub(crate) fn rotation_track_interpolate_full(&self, track_idx: i32, time_sec: f64, backward: bool,) -> Quaternion {
            type CallRet = Quaternion;
            type CallParams = (i32, f64, bool,);
            let args = (track_idx, time_sec, backward,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "rotation_track_interpolate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::rotation_track_interpolate_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn rotation_track_interpolate(&self, track_idx: i32, time_sec: f64,) -> Quaternion {
            self.rotation_track_interpolate_ex(track_idx, time_sec,) . done()
        }
        #[inline]
        pub fn rotation_track_interpolate_ex < 'a > (&'a self, track_idx: i32, time_sec: f64,) -> ExRotationTrackInterpolate < 'a > {
            ExRotationTrackInterpolate::new(self, track_idx, time_sec,)
        }
        pub(crate) fn scale_track_interpolate_full(&self, track_idx: i32, time_sec: f64, backward: bool,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (i32, f64, bool,);
            let args = (track_idx, time_sec, backward,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "scale_track_interpolate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::scale_track_interpolate_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn scale_track_interpolate(&self, track_idx: i32, time_sec: f64,) -> Vector3 {
            self.scale_track_interpolate_ex(track_idx, time_sec,) . done()
        }
        #[inline]
        pub fn scale_track_interpolate_ex < 'a > (&'a self, track_idx: i32, time_sec: f64,) -> ExScaleTrackInterpolate < 'a > {
            ExScaleTrackInterpolate::new(self, track_idx, time_sec,)
        }
        pub(crate) fn blend_shape_track_interpolate_full(&self, track_idx: i32, time_sec: f64, backward: bool,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, f64, bool,);
            let args = (track_idx, time_sec, backward,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "blend_shape_track_interpolate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::blend_shape_track_interpolate_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn blend_shape_track_interpolate(&self, track_idx: i32, time_sec: f64,) -> f32 {
            self.blend_shape_track_interpolate_ex(track_idx, time_sec,) . done()
        }
        #[inline]
        pub fn blend_shape_track_interpolate_ex < 'a > (&'a self, track_idx: i32, time_sec: f64,) -> ExBlendShapeTrackInterpolate < 'a > {
            ExBlendShapeTrackInterpolate::new(self, track_idx, time_sec,)
        }
        pub(crate) fn track_insert_key_full(&mut self, track_idx: i32, time: f64, key: RefArg < Variant >, transition: f32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (i32, f64, RefArg < 'a0, Variant >, f32,);
            let args = (track_idx, time, key, transition,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_insert_key", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::track_insert_key_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn track_insert_key(&mut self, track_idx: i32, time: f64, key: &Variant,) -> i32 {
            self.track_insert_key_ex(track_idx, time, key,) . done()
        }
        #[inline]
        pub fn track_insert_key_ex < 'a > (&'a mut self, track_idx: i32, time: f64, key: &'a Variant,) -> ExTrackInsertKey < 'a > {
            ExTrackInsertKey::new(self, track_idx, time, key,)
        }
        pub fn track_remove_key(&mut self, track_idx: i32, key_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_remove_key", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_remove_key_at_time(&mut self, track_idx: i32, time: f64,) {
            type CallRet = ();
            type CallParams = (i32, f64,);
            let args = (track_idx, time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_remove_key_at_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_set_key_value(&mut self, track_idx: i32, key: i32, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, RefArg < 'a0, Variant >,);
            let args = (track_idx, key, RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_set_key_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_set_key_transition(&mut self, track_idx: i32, key_idx: i32, transition: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (track_idx, key_idx, transition,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_set_key_transition", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_set_key_time(&mut self, track_idx: i32, key_idx: i32, time: f64,) {
            type CallRet = ();
            type CallParams = (i32, i32, f64,);
            let args = (track_idx, key_idx, time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_set_key_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_get_key_transition(&self, track_idx: i32, key_idx: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_get_key_transition", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_get_key_count(&self, track_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_get_key_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_get_key_value(&self, track_idx: i32, key_idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32, i32,);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_get_key_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_get_key_time(&self, track_idx: i32, key_idx: i32,) -> f64 {
            type CallRet = f64;
            type CallParams = (i32, i32,);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_get_key_time", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn track_find_key_full(&self, track_idx: i32, time: f64, find_mode: crate::classes::animation::FindMode, limit: bool, backward: bool,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, f64, crate::classes::animation::FindMode, bool, bool,);
            let args = (track_idx, time, find_mode, limit, backward,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_find_key", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::track_find_key_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn track_find_key(&self, track_idx: i32, time: f64,) -> i32 {
            self.track_find_key_ex(track_idx, time,) . done()
        }
        #[inline]
        pub fn track_find_key_ex < 'a > (&'a self, track_idx: i32, time: f64,) -> ExTrackFindKey < 'a > {
            ExTrackFindKey::new(self, track_idx, time,)
        }
        pub fn track_set_interpolation_type(&mut self, track_idx: i32, interpolation: crate::classes::animation::InterpolationType,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::animation::InterpolationType,);
            let args = (track_idx, interpolation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_set_interpolation_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_get_interpolation_type(&self, track_idx: i32,) -> crate::classes::animation::InterpolationType {
            type CallRet = crate::classes::animation::InterpolationType;
            type CallParams = (i32,);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_get_interpolation_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_set_interpolation_loop_wrap(&mut self, track_idx: i32, interpolation: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (track_idx, interpolation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_set_interpolation_loop_wrap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_get_interpolation_loop_wrap(&self, track_idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_get_interpolation_loop_wrap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn track_is_compressed(&self, track_idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "track_is_compressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn value_track_set_update_mode(&mut self, track_idx: i32, mode: crate::classes::animation::UpdateMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::animation::UpdateMode,);
            let args = (track_idx, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "value_track_set_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn value_track_get_update_mode(&self, track_idx: i32,) -> crate::classes::animation::UpdateMode {
            type CallRet = crate::classes::animation::UpdateMode;
            type CallParams = (i32,);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "value_track_get_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn value_track_interpolate_full(&self, track_idx: i32, time_sec: f64, backward: bool,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32, f64, bool,);
            let args = (track_idx, time_sec, backward,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "value_track_interpolate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::value_track_interpolate_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn value_track_interpolate(&self, track_idx: i32, time_sec: f64,) -> Variant {
            self.value_track_interpolate_ex(track_idx, time_sec,) . done()
        }
        #[inline]
        pub fn value_track_interpolate_ex < 'a > (&'a self, track_idx: i32, time_sec: f64,) -> ExValueTrackInterpolate < 'a > {
            ExValueTrackInterpolate::new(self, track_idx, time_sec,)
        }
        pub fn method_track_get_name(&self, track_idx: i32, key_idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32, i32,);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "method_track_get_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn method_track_get_params(&self, track_idx: i32, key_idx: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32, i32,);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "method_track_get_params", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn bezier_track_insert_key_full(&mut self, track_idx: i32, time: f64, value: f32, in_handle: Vector2, out_handle: Vector2,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, f64, f32, Vector2, Vector2,);
            let args = (track_idx, time, value, in_handle, out_handle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "bezier_track_insert_key", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::bezier_track_insert_key_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn bezier_track_insert_key(&mut self, track_idx: i32, time: f64, value: f32,) -> i32 {
            self.bezier_track_insert_key_ex(track_idx, time, value,) . done()
        }
        #[inline]
        pub fn bezier_track_insert_key_ex < 'a > (&'a mut self, track_idx: i32, time: f64, value: f32,) -> ExBezierTrackInsertKey < 'a > {
            ExBezierTrackInsertKey::new(self, track_idx, time, value,)
        }
        pub fn bezier_track_set_key_value(&mut self, track_idx: i32, key_idx: i32, value: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (track_idx, key_idx, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "bezier_track_set_key_value", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn bezier_track_set_key_in_handle_full(&mut self, track_idx: i32, key_idx: i32, in_handle: Vector2, balanced_value_time_ratio: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, Vector2, f32,);
            let args = (track_idx, key_idx, in_handle, balanced_value_time_ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "bezier_track_set_key_in_handle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::bezier_track_set_key_in_handle_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn bezier_track_set_key_in_handle(&mut self, track_idx: i32, key_idx: i32, in_handle: Vector2,) {
            self.bezier_track_set_key_in_handle_ex(track_idx, key_idx, in_handle,) . done()
        }
        #[inline]
        pub fn bezier_track_set_key_in_handle_ex < 'a > (&'a mut self, track_idx: i32, key_idx: i32, in_handle: Vector2,) -> ExBezierTrackSetKeyInHandle < 'a > {
            ExBezierTrackSetKeyInHandle::new(self, track_idx, key_idx, in_handle,)
        }
        pub(crate) fn bezier_track_set_key_out_handle_full(&mut self, track_idx: i32, key_idx: i32, out_handle: Vector2, balanced_value_time_ratio: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, Vector2, f32,);
            let args = (track_idx, key_idx, out_handle, balanced_value_time_ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "bezier_track_set_key_out_handle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::bezier_track_set_key_out_handle_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn bezier_track_set_key_out_handle(&mut self, track_idx: i32, key_idx: i32, out_handle: Vector2,) {
            self.bezier_track_set_key_out_handle_ex(track_idx, key_idx, out_handle,) . done()
        }
        #[inline]
        pub fn bezier_track_set_key_out_handle_ex < 'a > (&'a mut self, track_idx: i32, key_idx: i32, out_handle: Vector2,) -> ExBezierTrackSetKeyOutHandle < 'a > {
            ExBezierTrackSetKeyOutHandle::new(self, track_idx, key_idx, out_handle,)
        }
        pub fn bezier_track_get_key_value(&self, track_idx: i32, key_idx: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "bezier_track_get_key_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn bezier_track_get_key_in_handle(&self, track_idx: i32, key_idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32, i32,);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "bezier_track_get_key_in_handle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn bezier_track_get_key_out_handle(&self, track_idx: i32, key_idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32, i32,);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "bezier_track_get_key_out_handle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn bezier_track_interpolate(&self, track_idx: i32, time: f64,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, f64,);
            let args = (track_idx, time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "bezier_track_interpolate", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn audio_track_insert_key_full(&mut self, track_idx: i32, time: f64, stream: CowArg < Option < Gd < crate::classes::Resource > > >, start_offset: f32, end_offset: f32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (i32, f64, CowArg < 'a0, Option < Gd < crate::classes::Resource > > >, f32, f32,);
            let args = (track_idx, time, stream, start_offset, end_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "audio_track_insert_key", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::audio_track_insert_key_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn audio_track_insert_key(&mut self, track_idx: i32, time: f64, stream: impl AsArg < Option < Gd < crate::classes::Resource >> >,) -> i32 {
            self.audio_track_insert_key_ex(track_idx, time, stream,) . done()
        }
        #[inline]
        pub fn audio_track_insert_key_ex < 'a > (&'a mut self, track_idx: i32, time: f64, stream: impl AsArg < Option < Gd < crate::classes::Resource >> > + 'a,) -> ExAudioTrackInsertKey < 'a > {
            ExAudioTrackInsertKey::new(self, track_idx, time, stream,)
        }
        pub fn audio_track_set_key_stream(&mut self, track_idx: i32, key_idx: i32, stream: impl AsArg < Option < Gd < crate::classes::Resource >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, Option < Gd < crate::classes::Resource > > >,);
            let args = (track_idx, key_idx, stream.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "audio_track_set_key_stream", Some(self.__validated_obj()), args,)
            }
        }
        pub fn audio_track_set_key_start_offset(&mut self, track_idx: i32, key_idx: i32, offset: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (track_idx, key_idx, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "audio_track_set_key_start_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn audio_track_set_key_end_offset(&mut self, track_idx: i32, key_idx: i32, offset: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (track_idx, key_idx, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "audio_track_set_key_end_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn audio_track_get_key_stream(&self, track_idx: i32, key_idx: i32,) -> Option < Gd < crate::classes::Resource > > {
            type CallRet = Option < Gd < crate::classes::Resource > >;
            type CallParams = (i32, i32,);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "audio_track_get_key_stream", Some(self.__validated_obj()), args,)
            }
        }
        pub fn audio_track_get_key_start_offset(&self, track_idx: i32, key_idx: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "audio_track_get_key_start_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn audio_track_get_key_end_offset(&self, track_idx: i32, key_idx: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "audio_track_get_key_end_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn audio_track_set_use_blend(&mut self, track_idx: i32, enable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (track_idx, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "audio_track_set_use_blend", Some(self.__validated_obj()), args,)
            }
        }
        pub fn audio_track_is_use_blend(&self, track_idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (track_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "audio_track_is_use_blend", Some(self.__validated_obj()), args,)
            }
        }
        pub fn animation_track_insert_key(&mut self, track_idx: i32, time: f64, animation: impl AsArg < StringName >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (i32, f64, CowArg < 'a0, StringName >,);
            let args = (track_idx, time, animation.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "animation_track_insert_key", Some(self.__validated_obj()), args,)
            }
        }
        pub fn animation_track_set_key_animation(&mut self, track_idx: i32, key_idx: i32, animation: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, StringName >,);
            let args = (track_idx, key_idx, animation.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "animation_track_set_key_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn animation_track_get_key_animation(&self, track_idx: i32, key_idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32, i32,);
            let args = (track_idx, key_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "animation_track_get_key_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_marker(&mut self, name: impl AsArg < StringName >, time: f64,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f64,);
            let args = (name.into_arg(), time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "add_marker", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_marker(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "remove_marker", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_marker(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "has_marker", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_marker_at_time(&self, time: f64,) -> StringName {
            type CallRet = StringName;
            type CallParams = (f64,);
            let args = (time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "get_marker_at_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_next_marker(&self, time: f64,) -> StringName {
            type CallRet = StringName;
            type CallParams = (f64,);
            let args = (time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "get_next_marker", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_prev_marker(&self, time: f64,) -> StringName {
            type CallRet = StringName;
            type CallParams = (f64,);
            let args = (time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "get_prev_marker", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_marker_time(&self, name: impl AsArg < StringName >,) -> f64 {
            type CallRet = f64;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "get_marker_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_marker_names(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "get_marker_names", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_marker_color(&self, name: impl AsArg < StringName >,) -> Color {
            type CallRet = Color;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "get_marker_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_marker_color(&mut self, name: impl AsArg < StringName >, color: Color,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, Color,);
            let args = (name.into_arg(), color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "set_marker_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_length(&mut self, time_sec: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (time_sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "set_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_length(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "get_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_loop_mode(&mut self, loop_mode: crate::classes::animation::LoopMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation::LoopMode,);
            let args = (loop_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "set_loop_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_loop_mode(&self,) -> crate::classes::animation::LoopMode {
            type CallRet = crate::classes::animation::LoopMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "get_loop_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_step(&mut self, size_sec: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (size_sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "set_step", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_step(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "get_step", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn copy_track(&mut self, track_idx: i32, to_animation: impl AsArg < Option < Gd < crate::classes::Animation >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Animation > > >,);
            let args = (track_idx, to_animation.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "copy_track", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn optimize_full(&mut self, allowed_velocity_err: f32, allowed_angular_err: f32, precision: i32,) {
            type CallRet = ();
            type CallParams = (f32, f32, i32,);
            let args = (allowed_velocity_err, allowed_angular_err, precision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "optimize", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::optimize_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn optimize(&mut self,) {
            self.optimize_ex() . done()
        }
        #[inline]
        pub fn optimize_ex < 'a > (&'a mut self,) -> ExOptimize < 'a > {
            ExOptimize::new(self,)
        }
        pub(crate) fn compress_full(&mut self, page_size: u32, fps: u32, split_tolerance: f32,) {
            type CallRet = ();
            type CallParams = (u32, u32, f32,);
            let args = (page_size, fps, split_tolerance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "compress", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::compress_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn compress(&mut self,) {
            self.compress_ex() . done()
        }
        #[inline]
        pub fn compress_ex < 'a > (&'a mut self,) -> ExCompress < 'a > {
            ExCompress::new(self,)
        }
        pub fn is_capture_included(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Animation", "is_capture_included", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Animation {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Animation"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Animation {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Animation {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Animation {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Animation {
        
    }
    impl crate::obj::cap::GodotDefault for Animation {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Animation {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Animation {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Animation`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Animation__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Animation > for $Class {
                
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
#[doc = "Default-param extender for [`Animation::add_track_ex`][super::Animation::add_track_ex]."]
#[must_use]
pub struct ExAddTrack < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Animation, type_: crate::classes::animation::TrackType, at_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTrack < 'a > {
    fn new(surround_object: &'a mut re_export::Animation, type_: crate::classes::animation::TrackType,) -> Self {
        let at_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, type_: type_, at_position: at_position,
        }
    }
    #[inline]
    pub fn at_position(self, at_position: i32) -> Self {
        Self {
            at_position: at_position, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, type_, at_position,
        }
        = self;
        re_export::Animation::add_track_full(surround_object, type_, at_position,)
    }
}
#[doc = "Default-param extender for [`Animation::position_track_interpolate_ex`][super::Animation::position_track_interpolate_ex]."]
#[must_use]
pub struct ExPositionTrackInterpolate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Animation, track_idx: i32, time_sec: f64, backward: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPositionTrackInterpolate < 'a > {
    fn new(surround_object: &'a re_export::Animation, track_idx: i32, time_sec: f64,) -> Self {
        let backward = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, track_idx: track_idx, time_sec: time_sec, backward: backward,
        }
    }
    #[inline]
    pub fn backward(self, backward: bool) -> Self {
        Self {
            backward: backward, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3 {
        let Self {
            _phantom, surround_object, track_idx, time_sec, backward,
        }
        = self;
        re_export::Animation::position_track_interpolate_full(surround_object, track_idx, time_sec, backward,)
    }
}
#[doc = "Default-param extender for [`Animation::rotation_track_interpolate_ex`][super::Animation::rotation_track_interpolate_ex]."]
#[must_use]
pub struct ExRotationTrackInterpolate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Animation, track_idx: i32, time_sec: f64, backward: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRotationTrackInterpolate < 'a > {
    fn new(surround_object: &'a re_export::Animation, track_idx: i32, time_sec: f64,) -> Self {
        let backward = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, track_idx: track_idx, time_sec: time_sec, backward: backward,
        }
    }
    #[inline]
    pub fn backward(self, backward: bool) -> Self {
        Self {
            backward: backward, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Quaternion {
        let Self {
            _phantom, surround_object, track_idx, time_sec, backward,
        }
        = self;
        re_export::Animation::rotation_track_interpolate_full(surround_object, track_idx, time_sec, backward,)
    }
}
#[doc = "Default-param extender for [`Animation::scale_track_interpolate_ex`][super::Animation::scale_track_interpolate_ex]."]
#[must_use]
pub struct ExScaleTrackInterpolate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Animation, track_idx: i32, time_sec: f64, backward: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScaleTrackInterpolate < 'a > {
    fn new(surround_object: &'a re_export::Animation, track_idx: i32, time_sec: f64,) -> Self {
        let backward = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, track_idx: track_idx, time_sec: time_sec, backward: backward,
        }
    }
    #[inline]
    pub fn backward(self, backward: bool) -> Self {
        Self {
            backward: backward, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector3 {
        let Self {
            _phantom, surround_object, track_idx, time_sec, backward,
        }
        = self;
        re_export::Animation::scale_track_interpolate_full(surround_object, track_idx, time_sec, backward,)
    }
}
#[doc = "Default-param extender for [`Animation::blend_shape_track_interpolate_ex`][super::Animation::blend_shape_track_interpolate_ex]."]
#[must_use]
pub struct ExBlendShapeTrackInterpolate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Animation, track_idx: i32, time_sec: f64, backward: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBlendShapeTrackInterpolate < 'a > {
    fn new(surround_object: &'a re_export::Animation, track_idx: i32, time_sec: f64,) -> Self {
        let backward = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, track_idx: track_idx, time_sec: time_sec, backward: backward,
        }
    }
    #[inline]
    pub fn backward(self, backward: bool) -> Self {
        Self {
            backward: backward, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, track_idx, time_sec, backward,
        }
        = self;
        re_export::Animation::blend_shape_track_interpolate_full(surround_object, track_idx, time_sec, backward,)
    }
}
#[doc = "Default-param extender for [`Animation::track_insert_key_ex`][super::Animation::track_insert_key_ex]."]
#[must_use]
pub struct ExTrackInsertKey < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Animation, track_idx: i32, time: f64, key: CowArg < 'a, Variant >, transition: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTrackInsertKey < 'a > {
    fn new(surround_object: &'a mut re_export::Animation, track_idx: i32, time: f64, key: &'a Variant,) -> Self {
        let transition = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, track_idx: track_idx, time: time, key: CowArg::Borrowed(key), transition: transition,
        }
    }
    #[inline]
    pub fn transition(self, transition: f32) -> Self {
        Self {
            transition: transition, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, track_idx, time, key, transition,
        }
        = self;
        re_export::Animation::track_insert_key_full(surround_object, track_idx, time, key.cow_as_arg(), transition,)
    }
}
#[doc = "Default-param extender for [`Animation::track_find_key_ex`][super::Animation::track_find_key_ex]."]
#[must_use]
pub struct ExTrackFindKey < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Animation, track_idx: i32, time: f64, find_mode: crate::classes::animation::FindMode, limit: bool, backward: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTrackFindKey < 'a > {
    fn new(surround_object: &'a re_export::Animation, track_idx: i32, time: f64,) -> Self {
        let find_mode = crate::obj::EngineEnum::from_ord(0);
        let limit = false;
        let backward = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, track_idx: track_idx, time: time, find_mode: find_mode, limit: limit, backward: backward,
        }
    }
    #[inline]
    pub fn find_mode(self, find_mode: crate::classes::animation::FindMode) -> Self {
        Self {
            find_mode: find_mode, .. self
        }
    }
    #[inline]
    pub fn limit(self, limit: bool) -> Self {
        Self {
            limit: limit, .. self
        }
    }
    #[inline]
    pub fn backward(self, backward: bool) -> Self {
        Self {
            backward: backward, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, track_idx, time, find_mode, limit, backward,
        }
        = self;
        re_export::Animation::track_find_key_full(surround_object, track_idx, time, find_mode, limit, backward,)
    }
}
#[doc = "Default-param extender for [`Animation::value_track_interpolate_ex`][super::Animation::value_track_interpolate_ex]."]
#[must_use]
pub struct ExValueTrackInterpolate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Animation, track_idx: i32, time_sec: f64, backward: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExValueTrackInterpolate < 'a > {
    fn new(surround_object: &'a re_export::Animation, track_idx: i32, time_sec: f64,) -> Self {
        let backward = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, track_idx: track_idx, time_sec: time_sec, backward: backward,
        }
    }
    #[inline]
    pub fn backward(self, backward: bool) -> Self {
        Self {
            backward: backward, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, track_idx, time_sec, backward,
        }
        = self;
        re_export::Animation::value_track_interpolate_full(surround_object, track_idx, time_sec, backward,)
    }
}
#[doc = "Default-param extender for [`Animation::bezier_track_insert_key_ex`][super::Animation::bezier_track_insert_key_ex]."]
#[must_use]
pub struct ExBezierTrackInsertKey < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Animation, track_idx: i32, time: f64, value: f32, in_handle: Vector2, out_handle: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBezierTrackInsertKey < 'a > {
    fn new(surround_object: &'a mut re_export::Animation, track_idx: i32, time: f64, value: f32,) -> Self {
        let in_handle = Vector2::new(0 as _, 0 as _);
        let out_handle = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, track_idx: track_idx, time: time, value: value, in_handle: in_handle, out_handle: out_handle,
        }
    }
    #[inline]
    pub fn in_handle(self, in_handle: Vector2) -> Self {
        Self {
            in_handle: in_handle, .. self
        }
    }
    #[inline]
    pub fn out_handle(self, out_handle: Vector2) -> Self {
        Self {
            out_handle: out_handle, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, track_idx, time, value, in_handle, out_handle,
        }
        = self;
        re_export::Animation::bezier_track_insert_key_full(surround_object, track_idx, time, value, in_handle, out_handle,)
    }
}
#[doc = "Default-param extender for [`Animation::bezier_track_set_key_in_handle_ex`][super::Animation::bezier_track_set_key_in_handle_ex]."]
#[must_use]
pub struct ExBezierTrackSetKeyInHandle < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Animation, track_idx: i32, key_idx: i32, in_handle: Vector2, balanced_value_time_ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBezierTrackSetKeyInHandle < 'a > {
    fn new(surround_object: &'a mut re_export::Animation, track_idx: i32, key_idx: i32, in_handle: Vector2,) -> Self {
        let balanced_value_time_ratio = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, track_idx: track_idx, key_idx: key_idx, in_handle: in_handle, balanced_value_time_ratio: balanced_value_time_ratio,
        }
    }
    #[inline]
    pub fn balanced_value_time_ratio(self, balanced_value_time_ratio: f32) -> Self {
        Self {
            balanced_value_time_ratio: balanced_value_time_ratio, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, track_idx, key_idx, in_handle, balanced_value_time_ratio,
        }
        = self;
        re_export::Animation::bezier_track_set_key_in_handle_full(surround_object, track_idx, key_idx, in_handle, balanced_value_time_ratio,)
    }
}
#[doc = "Default-param extender for [`Animation::bezier_track_set_key_out_handle_ex`][super::Animation::bezier_track_set_key_out_handle_ex]."]
#[must_use]
pub struct ExBezierTrackSetKeyOutHandle < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Animation, track_idx: i32, key_idx: i32, out_handle: Vector2, balanced_value_time_ratio: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBezierTrackSetKeyOutHandle < 'a > {
    fn new(surround_object: &'a mut re_export::Animation, track_idx: i32, key_idx: i32, out_handle: Vector2,) -> Self {
        let balanced_value_time_ratio = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, track_idx: track_idx, key_idx: key_idx, out_handle: out_handle, balanced_value_time_ratio: balanced_value_time_ratio,
        }
    }
    #[inline]
    pub fn balanced_value_time_ratio(self, balanced_value_time_ratio: f32) -> Self {
        Self {
            balanced_value_time_ratio: balanced_value_time_ratio, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, track_idx, key_idx, out_handle, balanced_value_time_ratio,
        }
        = self;
        re_export::Animation::bezier_track_set_key_out_handle_full(surround_object, track_idx, key_idx, out_handle, balanced_value_time_ratio,)
    }
}
#[doc = "Default-param extender for [`Animation::audio_track_insert_key_ex`][super::Animation::audio_track_insert_key_ex]."]
#[must_use]
pub struct ExAudioTrackInsertKey < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Animation, track_idx: i32, time: f64, stream: CowArg < 'a, Option < Gd < crate::classes::Resource > > >, start_offset: f32, end_offset: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAudioTrackInsertKey < 'a > {
    fn new(surround_object: &'a mut re_export::Animation, track_idx: i32, time: f64, stream: impl AsArg < Option < Gd < crate::classes::Resource >> > + 'a,) -> Self {
        let start_offset = 0f32;
        let end_offset = 0f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, track_idx: track_idx, time: time, stream: stream.into_arg(), start_offset: start_offset, end_offset: end_offset,
        }
    }
    #[inline]
    pub fn start_offset(self, start_offset: f32) -> Self {
        Self {
            start_offset: start_offset, .. self
        }
    }
    #[inline]
    pub fn end_offset(self, end_offset: f32) -> Self {
        Self {
            end_offset: end_offset, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, track_idx, time, stream, start_offset, end_offset,
        }
        = self;
        re_export::Animation::audio_track_insert_key_full(surround_object, track_idx, time, stream, start_offset, end_offset,)
    }
}
#[doc = "Default-param extender for [`Animation::optimize_ex`][super::Animation::optimize_ex]."]
#[must_use]
pub struct ExOptimize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Animation, allowed_velocity_err: f32, allowed_angular_err: f32, precision: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExOptimize < 'a > {
    fn new(surround_object: &'a mut re_export::Animation,) -> Self {
        let allowed_velocity_err = 0.01f32;
        let allowed_angular_err = 0.01f32;
        let precision = 3i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, allowed_velocity_err: allowed_velocity_err, allowed_angular_err: allowed_angular_err, precision: precision,
        }
    }
    #[inline]
    pub fn allowed_velocity_err(self, allowed_velocity_err: f32) -> Self {
        Self {
            allowed_velocity_err: allowed_velocity_err, .. self
        }
    }
    #[inline]
    pub fn allowed_angular_err(self, allowed_angular_err: f32) -> Self {
        Self {
            allowed_angular_err: allowed_angular_err, .. self
        }
    }
    #[inline]
    pub fn precision(self, precision: i32) -> Self {
        Self {
            precision: precision, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, allowed_velocity_err, allowed_angular_err, precision,
        }
        = self;
        re_export::Animation::optimize_full(surround_object, allowed_velocity_err, allowed_angular_err, precision,)
    }
}
#[doc = "Default-param extender for [`Animation::compress_ex`][super::Animation::compress_ex]."]
#[must_use]
pub struct ExCompress < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Animation, page_size: u32, fps: u32, split_tolerance: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCompress < 'a > {
    fn new(surround_object: &'a mut re_export::Animation,) -> Self {
        let page_size = 8192u32;
        let fps = 120u32;
        let split_tolerance = 4f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, page_size: page_size, fps: fps, split_tolerance: split_tolerance,
        }
    }
    #[inline]
    pub fn page_size(self, page_size: u32) -> Self {
        Self {
            page_size: page_size, .. self
        }
    }
    #[inline]
    pub fn fps(self, fps: u32) -> Self {
        Self {
            fps: fps, .. self
        }
    }
    #[inline]
    pub fn split_tolerance(self, split_tolerance: f32) -> Self {
        Self {
            split_tolerance: split_tolerance, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, page_size, fps, split_tolerance,
        }
        = self;
        re_export::Animation::compress_full(surround_object, page_size, fps, split_tolerance,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TrackType {
    ord: i32
}
impl TrackType {
    #[doc(alias = "TYPE_VALUE")]
    #[doc = "Godot enumerator name: `TYPE_VALUE`"]
    pub const VALUE: TrackType = TrackType {
        ord: 0i32
    };
    #[doc(alias = "TYPE_POSITION_3D")]
    #[doc = "Godot enumerator name: `TYPE_POSITION_3D`"]
    pub const POSITION_3D: TrackType = TrackType {
        ord: 1i32
    };
    #[doc(alias = "TYPE_ROTATION_3D")]
    #[doc = "Godot enumerator name: `TYPE_ROTATION_3D`"]
    pub const ROTATION_3D: TrackType = TrackType {
        ord: 2i32
    };
    #[doc(alias = "TYPE_SCALE_3D")]
    #[doc = "Godot enumerator name: `TYPE_SCALE_3D`"]
    pub const SCALE_3D: TrackType = TrackType {
        ord: 3i32
    };
    #[doc(alias = "TYPE_BLEND_SHAPE")]
    #[doc = "Godot enumerator name: `TYPE_BLEND_SHAPE`"]
    pub const BLEND_SHAPE: TrackType = TrackType {
        ord: 4i32
    };
    #[doc(alias = "TYPE_METHOD")]
    #[doc = "Godot enumerator name: `TYPE_METHOD`"]
    pub const METHOD: TrackType = TrackType {
        ord: 5i32
    };
    #[doc(alias = "TYPE_BEZIER")]
    #[doc = "Godot enumerator name: `TYPE_BEZIER`"]
    pub const BEZIER: TrackType = TrackType {
        ord: 6i32
    };
    #[doc(alias = "TYPE_AUDIO")]
    #[doc = "Godot enumerator name: `TYPE_AUDIO`"]
    pub const AUDIO: TrackType = TrackType {
        ord: 7i32
    };
    #[doc(alias = "TYPE_ANIMATION")]
    #[doc = "Godot enumerator name: `TYPE_ANIMATION`"]
    pub const ANIMATION: TrackType = TrackType {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for TrackType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TrackType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TrackType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::VALUE => "VALUE", Self::POSITION_3D => "POSITION_3D", Self::ROTATION_3D => "ROTATION_3D", Self::SCALE_3D => "SCALE_3D", Self::BLEND_SHAPE => "BLEND_SHAPE", Self::METHOD => "METHOD", Self::BEZIER => "BEZIER", Self::AUDIO => "AUDIO", Self::ANIMATION => "ANIMATION", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TrackType::VALUE, TrackType::POSITION_3D, TrackType::ROTATION_3D, TrackType::SCALE_3D, TrackType::BLEND_SHAPE, TrackType::METHOD, TrackType::BEZIER, TrackType::AUDIO, TrackType::ANIMATION]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TrackType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VALUE", "TYPE_VALUE", TrackType::VALUE), crate::meta::inspect::EnumConstant::new("POSITION_3D", "TYPE_POSITION_3D", TrackType::POSITION_3D), crate::meta::inspect::EnumConstant::new("ROTATION_3D", "TYPE_ROTATION_3D", TrackType::ROTATION_3D), crate::meta::inspect::EnumConstant::new("SCALE_3D", "TYPE_SCALE_3D", TrackType::SCALE_3D), crate::meta::inspect::EnumConstant::new("BLEND_SHAPE", "TYPE_BLEND_SHAPE", TrackType::BLEND_SHAPE), crate::meta::inspect::EnumConstant::new("METHOD", "TYPE_METHOD", TrackType::METHOD), crate::meta::inspect::EnumConstant::new("BEZIER", "TYPE_BEZIER", TrackType::BEZIER), crate::meta::inspect::EnumConstant::new("AUDIO", "TYPE_AUDIO", TrackType::AUDIO), crate::meta::inspect::EnumConstant::new("ANIMATION", "TYPE_ANIMATION", TrackType::ANIMATION)]
        }
    }
}
impl crate::meta::GodotConvert for TrackType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TrackType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TrackType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct InterpolationType {
    ord: i32
}
impl InterpolationType {
    #[doc(alias = "INTERPOLATION_NEAREST")]
    #[doc = "Godot enumerator name: `INTERPOLATION_NEAREST`"]
    pub const NEAREST: InterpolationType = InterpolationType {
        ord: 0i32
    };
    #[doc(alias = "INTERPOLATION_LINEAR")]
    #[doc = "Godot enumerator name: `INTERPOLATION_LINEAR`"]
    pub const LINEAR: InterpolationType = InterpolationType {
        ord: 1i32
    };
    #[doc(alias = "INTERPOLATION_CUBIC")]
    #[doc = "Godot enumerator name: `INTERPOLATION_CUBIC`"]
    pub const CUBIC: InterpolationType = InterpolationType {
        ord: 2i32
    };
    #[doc(alias = "INTERPOLATION_LINEAR_ANGLE")]
    #[doc = "Godot enumerator name: `INTERPOLATION_LINEAR_ANGLE`"]
    pub const LINEAR_ANGLE: InterpolationType = InterpolationType {
        ord: 3i32
    };
    #[doc(alias = "INTERPOLATION_CUBIC_ANGLE")]
    #[doc = "Godot enumerator name: `INTERPOLATION_CUBIC_ANGLE`"]
    pub const CUBIC_ANGLE: InterpolationType = InterpolationType {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for InterpolationType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("InterpolationType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for InterpolationType {
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
            Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", Self::CUBIC => "CUBIC", Self::LINEAR_ANGLE => "LINEAR_ANGLE", Self::CUBIC_ANGLE => "CUBIC_ANGLE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[InterpolationType::NEAREST, InterpolationType::LINEAR, InterpolationType::CUBIC, InterpolationType::LINEAR_ANGLE, InterpolationType::CUBIC_ANGLE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < InterpolationType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEAREST", "INTERPOLATION_NEAREST", InterpolationType::NEAREST), crate::meta::inspect::EnumConstant::new("LINEAR", "INTERPOLATION_LINEAR", InterpolationType::LINEAR), crate::meta::inspect::EnumConstant::new("CUBIC", "INTERPOLATION_CUBIC", InterpolationType::CUBIC), crate::meta::inspect::EnumConstant::new("LINEAR_ANGLE", "INTERPOLATION_LINEAR_ANGLE", InterpolationType::LINEAR_ANGLE), crate::meta::inspect::EnumConstant::new("CUBIC_ANGLE", "INTERPOLATION_CUBIC_ANGLE", InterpolationType::CUBIC_ANGLE)]
        }
    }
}
impl crate::meta::GodotConvert for InterpolationType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for InterpolationType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for InterpolationType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct UpdateMode {
    ord: i32
}
impl UpdateMode {
    #[doc(alias = "UPDATE_CONTINUOUS")]
    #[doc = "Godot enumerator name: `UPDATE_CONTINUOUS`"]
    pub const CONTINUOUS: UpdateMode = UpdateMode {
        ord: 0i32
    };
    #[doc(alias = "UPDATE_DISCRETE")]
    #[doc = "Godot enumerator name: `UPDATE_DISCRETE`"]
    pub const DISCRETE: UpdateMode = UpdateMode {
        ord: 1i32
    };
    #[doc(alias = "UPDATE_CAPTURE")]
    #[doc = "Godot enumerator name: `UPDATE_CAPTURE`"]
    pub const CAPTURE: UpdateMode = UpdateMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for UpdateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("UpdateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for UpdateMode {
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
            Self::CONTINUOUS => "CONTINUOUS", Self::DISCRETE => "DISCRETE", Self::CAPTURE => "CAPTURE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[UpdateMode::CONTINUOUS, UpdateMode::DISCRETE, UpdateMode::CAPTURE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < UpdateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CONTINUOUS", "UPDATE_CONTINUOUS", UpdateMode::CONTINUOUS), crate::meta::inspect::EnumConstant::new("DISCRETE", "UPDATE_DISCRETE", UpdateMode::DISCRETE), crate::meta::inspect::EnumConstant::new("CAPTURE", "UPDATE_CAPTURE", UpdateMode::CAPTURE)]
        }
    }
}
impl crate::meta::GodotConvert for UpdateMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for UpdateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for UpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LoopMode {
    ord: i32
}
impl LoopMode {
    #[doc(alias = "LOOP_NONE")]
    #[doc = "Godot enumerator name: `LOOP_NONE`"]
    pub const NONE: LoopMode = LoopMode {
        ord: 0i32
    };
    #[doc(alias = "LOOP_LINEAR")]
    #[doc = "Godot enumerator name: `LOOP_LINEAR`"]
    pub const LINEAR: LoopMode = LoopMode {
        ord: 1i32
    };
    #[doc(alias = "LOOP_PINGPONG")]
    #[doc = "Godot enumerator name: `LOOP_PINGPONG`"]
    pub const PINGPONG: LoopMode = LoopMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for LoopMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LoopMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LoopMode {
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
            Self::NONE => "NONE", Self::LINEAR => "LINEAR", Self::PINGPONG => "PINGPONG", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LoopMode::NONE, LoopMode::LINEAR, LoopMode::PINGPONG]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LoopMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "LOOP_NONE", LoopMode::NONE), crate::meta::inspect::EnumConstant::new("LINEAR", "LOOP_LINEAR", LoopMode::LINEAR), crate::meta::inspect::EnumConstant::new("PINGPONG", "LOOP_PINGPONG", LoopMode::PINGPONG)]
        }
    }
}
impl crate::meta::GodotConvert for LoopMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LoopMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LoopMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LoopedFlag {
    ord: i32
}
impl LoopedFlag {
    #[doc(alias = "LOOPED_FLAG_NONE")]
    #[doc = "Godot enumerator name: `LOOPED_FLAG_NONE`"]
    pub const NONE: LoopedFlag = LoopedFlag {
        ord: 0i32
    };
    #[doc(alias = "LOOPED_FLAG_END")]
    #[doc = "Godot enumerator name: `LOOPED_FLAG_END`"]
    pub const END: LoopedFlag = LoopedFlag {
        ord: 1i32
    };
    #[doc(alias = "LOOPED_FLAG_START")]
    #[doc = "Godot enumerator name: `LOOPED_FLAG_START`"]
    pub const START: LoopedFlag = LoopedFlag {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for LoopedFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LoopedFlag") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LoopedFlag {
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
            Self::NONE => "NONE", Self::END => "END", Self::START => "START", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LoopedFlag::NONE, LoopedFlag::END, LoopedFlag::START]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LoopedFlag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "LOOPED_FLAG_NONE", LoopedFlag::NONE), crate::meta::inspect::EnumConstant::new("END", "LOOPED_FLAG_END", LoopedFlag::END), crate::meta::inspect::EnumConstant::new("START", "LOOPED_FLAG_START", LoopedFlag::START)]
        }
    }
}
impl crate::meta::GodotConvert for LoopedFlag {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LoopedFlag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LoopedFlag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FindMode {
    ord: i32
}
impl FindMode {
    #[doc(alias = "FIND_MODE_NEAREST")]
    #[doc = "Godot enumerator name: `FIND_MODE_NEAREST`"]
    pub const NEAREST: FindMode = FindMode {
        ord: 0i32
    };
    #[doc(alias = "FIND_MODE_APPROX")]
    #[doc = "Godot enumerator name: `FIND_MODE_APPROX`"]
    pub const APPROX: FindMode = FindMode {
        ord: 1i32
    };
    #[doc(alias = "FIND_MODE_EXACT")]
    #[doc = "Godot enumerator name: `FIND_MODE_EXACT`"]
    pub const EXACT: FindMode = FindMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for FindMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FindMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FindMode {
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
            Self::NEAREST => "NEAREST", Self::APPROX => "APPROX", Self::EXACT => "EXACT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FindMode::NEAREST, FindMode::APPROX, FindMode::EXACT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FindMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEAREST", "FIND_MODE_NEAREST", FindMode::NEAREST), crate::meta::inspect::EnumConstant::new("APPROX", "FIND_MODE_APPROX", FindMode::APPROX), crate::meta::inspect::EnumConstant::new("EXACT", "FIND_MODE_EXACT", FindMode::EXACT)]
        }
    }
}
impl crate::meta::GodotConvert for FindMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FindMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FindMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Animation;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Animation {
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