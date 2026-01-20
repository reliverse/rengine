#![doc = "Sidecar module for class [`Tween`][crate::classes::Tween].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Tween` enums](https://docs.godotengine.org/en/stable/classes/class_tween.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Tween.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`tween`][crate::classes::tween]: sidecar module with related enum/flag types\n* [`ITween`][crate::classes::ITween]: virtual methods\n* [`SignalsOfTween`][crate::classes::tween::SignalsOfTween]: signal collection\n\n\nSee also [Godot docs for `Tween`](https://docs.godotengine.org/en/stable/classes/class_tween.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Tween>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Tween {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Tween`][crate::classes::Tween].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Tween` methods](https://docs.godotengine.org/en/stable/classes/class_tween.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITween: crate::obj::GodotClass < Base = Tween > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl Tween {
        pub fn tween_property(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, property: impl AsArg < NodePath >, final_val: &Variant, duration: f64,) -> Option < Gd < crate::classes::PropertyTweener > > {
            type CallRet = Option < Gd < crate::classes::PropertyTweener > >;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, CowArg < 'a1, NodePath >, RefArg < 'a2, Variant >, f64,);
            let args = (object.into_arg(), property.into_arg(), RefArg::new(final_val), duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "tween_property", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tween_interval(&mut self, time: f64,) -> Option < Gd < crate::classes::IntervalTweener > > {
            type CallRet = Option < Gd < crate::classes::IntervalTweener > >;
            type CallParams = (f64,);
            let args = (time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "tween_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tween_callback(&mut self, callback: &Callable,) -> Option < Gd < crate::classes::CallbackTweener > > {
            type CallRet = Option < Gd < crate::classes::CallbackTweener > >;
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "tween_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tween_method(&mut self, method: &Callable, from: &Variant, to: &Variant, duration: f64,) -> Option < Gd < crate::classes::MethodTweener > > {
            type CallRet = Option < Gd < crate::classes::MethodTweener > >;
            type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, Callable >, RefArg < 'a1, Variant >, RefArg < 'a2, Variant >, f64,);
            let args = (RefArg::new(method), RefArg::new(from), RefArg::new(to), duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "tween_method", Some(self.__validated_obj()), args,)
            }
        }
        pub fn tween_subtween(&mut self, subtween: impl AsArg < Option < Gd < crate::classes::Tween >> >,) -> Option < Gd < crate::classes::SubtweenTweener > > {
            type CallRet = Option < Gd < crate::classes::SubtweenTweener > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Tween > > >,);
            let args = (subtween.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "tween_subtween", Some(self.__validated_obj()), args,)
            }
        }
        pub fn custom_step(&mut self, delta: f64,) -> bool {
            type CallRet = bool;
            type CallParams = (f64,);
            let args = (delta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "custom_step", Some(self.__validated_obj()), args,)
            }
        }
        pub fn stop(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "stop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn pause(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "pause", Some(self.__validated_obj()), args,)
            }
        }
        pub fn play(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "play", Some(self.__validated_obj()), args,)
            }
        }
        pub fn kill(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10280usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "kill", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_total_elapsed_time(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10281usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "get_total_elapsed_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_running(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10282usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "is_running", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_valid(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10283usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "is_valid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn bind_node(&mut self, node: impl AsArg < Option < Gd < crate::classes::Node >> >,) -> Option < Gd < crate::classes::Tween > > {
            type CallRet = Option < Gd < crate::classes::Tween > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10284usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "bind_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_mode(&mut self, mode: crate::classes::tween::TweenProcessMode,) -> Option < Gd < crate::classes::Tween > > {
            type CallRet = Option < Gd < crate::classes::Tween > >;
            type CallParams = (crate::classes::tween::TweenProcessMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10285usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "set_process_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pause_mode(&mut self, mode: crate::classes::tween::TweenPauseMode,) -> Option < Gd < crate::classes::Tween > > {
            type CallRet = Option < Gd < crate::classes::Tween > >;
            type CallParams = (crate::classes::tween::TweenPauseMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10286usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "set_pause_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn set_ignore_time_scale_full(&mut self, ignore: bool,) -> Option < Gd < crate::classes::Tween > > {
            type CallRet = Option < Gd < crate::classes::Tween > >;
            type CallParams = (bool,);
            let args = (ignore,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10287usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "set_ignore_time_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_ignore_time_scale_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_ignore_time_scale(&mut self,) -> Option < Gd < crate::classes::Tween > > {
            self.set_ignore_time_scale_ex() . done()
        }
        #[inline]
        pub fn set_ignore_time_scale_ex < 'a > (&'a mut self,) -> ExSetIgnoreTimeScale < 'a > {
            ExSetIgnoreTimeScale::new(self,)
        }
        pub(crate) fn set_parallel_full(&mut self, parallel: bool,) -> Option < Gd < crate::classes::Tween > > {
            type CallRet = Option < Gd < crate::classes::Tween > >;
            type CallParams = (bool,);
            let args = (parallel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10288usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "set_parallel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_parallel_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_parallel(&mut self,) -> Option < Gd < crate::classes::Tween > > {
            self.set_parallel_ex() . done()
        }
        #[inline]
        pub fn set_parallel_ex < 'a > (&'a mut self,) -> ExSetParallel < 'a > {
            ExSetParallel::new(self,)
        }
        pub(crate) fn set_loops_full(&mut self, loops: i32,) -> Option < Gd < crate::classes::Tween > > {
            type CallRet = Option < Gd < crate::classes::Tween > >;
            type CallParams = (i32,);
            let args = (loops,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10289usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "set_loops", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_loops_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_loops(&mut self,) -> Option < Gd < crate::classes::Tween > > {
            self.set_loops_ex() . done()
        }
        #[inline]
        pub fn set_loops_ex < 'a > (&'a mut self,) -> ExSetLoops < 'a > {
            ExSetLoops::new(self,)
        }
        pub fn get_loops_left(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10290usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "get_loops_left", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_speed_scale(&mut self, speed: f32,) -> Option < Gd < crate::classes::Tween > > {
            type CallRet = Option < Gd < crate::classes::Tween > >;
            type CallParams = (f32,);
            let args = (speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10291usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "set_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_trans(&mut self, trans: crate::classes::tween::TransitionType,) -> Option < Gd < crate::classes::Tween > > {
            type CallRet = Option < Gd < crate::classes::Tween > >;
            type CallParams = (crate::classes::tween::TransitionType,);
            let args = (trans,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10292usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "set_trans", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ease(&mut self, ease: crate::classes::tween::EaseType,) -> Option < Gd < crate::classes::Tween > > {
            type CallRet = Option < Gd < crate::classes::Tween > >;
            type CallParams = (crate::classes::tween::EaseType,);
            let args = (ease,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10293usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "set_ease", Some(self.__validated_obj()), args,)
            }
        }
        pub fn parallel(&mut self,) -> Option < Gd < crate::classes::Tween > > {
            type CallRet = Option < Gd < crate::classes::Tween > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10294usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "parallel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn chain(&mut self,) -> Option < Gd < crate::classes::Tween > > {
            type CallRet = Option < Gd < crate::classes::Tween > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10295usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "chain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn interpolate_value(initial_value: &Variant, delta_value: &Variant, elapsed_time: f64, duration: f64, trans_type: crate::classes::tween::TransitionType, ease_type: crate::classes::tween::EaseType,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >, f64, f64, crate::classes::tween::TransitionType, crate::classes::tween::EaseType,);
            let args = (RefArg::new(initial_value), RefArg::new(delta_value), elapsed_time, duration, trans_type, ease_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10296usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "interpolate_value", None, args,)
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
    impl crate::obj::GodotClass for Tween {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Tween"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Tween {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Tween {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Tween {
        
    }
    impl std::ops::Deref for Tween {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Tween {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Tween`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Tween__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Tween > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Tween::set_ignore_time_scale_ex`][super::Tween::set_ignore_time_scale_ex]."]
#[must_use]
pub struct ExSetIgnoreTimeScale < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Tween, ignore: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetIgnoreTimeScale < 'a > {
    fn new(surround_object: &'a mut re_export::Tween,) -> Self {
        let ignore = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, ignore: ignore,
        }
    }
    #[inline]
    pub fn ignore(self, ignore: bool) -> Self {
        Self {
            ignore: ignore, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Tween > > {
        let Self {
            _phantom, surround_object, ignore,
        }
        = self;
        re_export::Tween::set_ignore_time_scale_full(surround_object, ignore,)
    }
}
#[doc = "Default-param extender for [`Tween::set_parallel_ex`][super::Tween::set_parallel_ex]."]
#[must_use]
pub struct ExSetParallel < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Tween, parallel: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetParallel < 'a > {
    fn new(surround_object: &'a mut re_export::Tween,) -> Self {
        let parallel = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parallel: parallel,
        }
    }
    #[inline]
    pub fn parallel(self, parallel: bool) -> Self {
        Self {
            parallel: parallel, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Tween > > {
        let Self {
            _phantom, surround_object, parallel,
        }
        = self;
        re_export::Tween::set_parallel_full(surround_object, parallel,)
    }
}
#[doc = "Default-param extender for [`Tween::set_loops_ex`][super::Tween::set_loops_ex]."]
#[must_use]
pub struct ExSetLoops < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Tween, loops: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetLoops < 'a > {
    fn new(surround_object: &'a mut re_export::Tween,) -> Self {
        let loops = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, loops: loops,
        }
    }
    #[inline]
    pub fn loops(self, loops: i32) -> Self {
        Self {
            loops: loops, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Tween > > {
        let Self {
            _phantom, surround_object, loops,
        }
        = self;
        re_export::Tween::set_loops_full(surround_object, loops,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TweenProcessMode {
    ord: i32
}
impl TweenProcessMode {
    #[doc(alias = "TWEEN_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `TWEEN_PROCESS_PHYSICS`"]
    pub const PHYSICS: TweenProcessMode = TweenProcessMode {
        ord: 0i32
    };
    #[doc(alias = "TWEEN_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `TWEEN_PROCESS_IDLE`"]
    pub const IDLE: TweenProcessMode = TweenProcessMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for TweenProcessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TweenProcessMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TweenProcessMode {
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
            Self::PHYSICS => "PHYSICS", Self::IDLE => "IDLE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TweenProcessMode::PHYSICS, TweenProcessMode::IDLE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TweenProcessMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PHYSICS", "TWEEN_PROCESS_PHYSICS", TweenProcessMode::PHYSICS), crate::meta::inspect::EnumConstant::new("IDLE", "TWEEN_PROCESS_IDLE", TweenProcessMode::IDLE)]
        }
    }
}
impl crate::meta::GodotConvert for TweenProcessMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TweenProcessMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TweenProcessMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TweenPauseMode {
    ord: i32
}
impl TweenPauseMode {
    #[doc(alias = "TWEEN_PAUSE_BOUND")]
    #[doc = "Godot enumerator name: `TWEEN_PAUSE_BOUND`"]
    pub const BOUND: TweenPauseMode = TweenPauseMode {
        ord: 0i32
    };
    #[doc(alias = "TWEEN_PAUSE_STOP")]
    #[doc = "Godot enumerator name: `TWEEN_PAUSE_STOP`"]
    pub const STOP: TweenPauseMode = TweenPauseMode {
        ord: 1i32
    };
    #[doc(alias = "TWEEN_PAUSE_PROCESS")]
    #[doc = "Godot enumerator name: `TWEEN_PAUSE_PROCESS`"]
    pub const PROCESS: TweenPauseMode = TweenPauseMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TweenPauseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TweenPauseMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TweenPauseMode {
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
            Self::BOUND => "BOUND", Self::STOP => "STOP", Self::PROCESS => "PROCESS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TweenPauseMode::BOUND, TweenPauseMode::STOP, TweenPauseMode::PROCESS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TweenPauseMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BOUND", "TWEEN_PAUSE_BOUND", TweenPauseMode::BOUND), crate::meta::inspect::EnumConstant::new("STOP", "TWEEN_PAUSE_STOP", TweenPauseMode::STOP), crate::meta::inspect::EnumConstant::new("PROCESS", "TWEEN_PAUSE_PROCESS", TweenPauseMode::PROCESS)]
        }
    }
}
impl crate::meta::GodotConvert for TweenPauseMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TweenPauseMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TweenPauseMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TransitionType {
    ord: i32
}
impl TransitionType {
    #[doc(alias = "TRANS_LINEAR")]
    #[doc = "Godot enumerator name: `TRANS_LINEAR`"]
    pub const LINEAR: TransitionType = TransitionType {
        ord: 0i32
    };
    #[doc(alias = "TRANS_SINE")]
    #[doc = "Godot enumerator name: `TRANS_SINE`"]
    pub const SINE: TransitionType = TransitionType {
        ord: 1i32
    };
    #[doc(alias = "TRANS_QUINT")]
    #[doc = "Godot enumerator name: `TRANS_QUINT`"]
    pub const QUINT: TransitionType = TransitionType {
        ord: 2i32
    };
    #[doc(alias = "TRANS_QUART")]
    #[doc = "Godot enumerator name: `TRANS_QUART`"]
    pub const QUART: TransitionType = TransitionType {
        ord: 3i32
    };
    #[doc(alias = "TRANS_QUAD")]
    #[doc = "Godot enumerator name: `TRANS_QUAD`"]
    pub const QUAD: TransitionType = TransitionType {
        ord: 4i32
    };
    #[doc(alias = "TRANS_EXPO")]
    #[doc = "Godot enumerator name: `TRANS_EXPO`"]
    pub const EXPO: TransitionType = TransitionType {
        ord: 5i32
    };
    #[doc(alias = "TRANS_ELASTIC")]
    #[doc = "Godot enumerator name: `TRANS_ELASTIC`"]
    pub const ELASTIC: TransitionType = TransitionType {
        ord: 6i32
    };
    #[doc(alias = "TRANS_CUBIC")]
    #[doc = "Godot enumerator name: `TRANS_CUBIC`"]
    pub const CUBIC: TransitionType = TransitionType {
        ord: 7i32
    };
    #[doc(alias = "TRANS_CIRC")]
    #[doc = "Godot enumerator name: `TRANS_CIRC`"]
    pub const CIRC: TransitionType = TransitionType {
        ord: 8i32
    };
    #[doc(alias = "TRANS_BOUNCE")]
    #[doc = "Godot enumerator name: `TRANS_BOUNCE`"]
    pub const BOUNCE: TransitionType = TransitionType {
        ord: 9i32
    };
    #[doc(alias = "TRANS_BACK")]
    #[doc = "Godot enumerator name: `TRANS_BACK`"]
    pub const BACK: TransitionType = TransitionType {
        ord: 10i32
    };
    #[doc(alias = "TRANS_SPRING")]
    #[doc = "Godot enumerator name: `TRANS_SPRING`"]
    pub const SPRING: TransitionType = TransitionType {
        ord: 11i32
    };
    
}
impl std::fmt::Debug for TransitionType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TransitionType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TransitionType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 => Some(Self {
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
            Self::LINEAR => "LINEAR", Self::SINE => "SINE", Self::QUINT => "QUINT", Self::QUART => "QUART", Self::QUAD => "QUAD", Self::EXPO => "EXPO", Self::ELASTIC => "ELASTIC", Self::CUBIC => "CUBIC", Self::CIRC => "CIRC", Self::BOUNCE => "BOUNCE", Self::BACK => "BACK", Self::SPRING => "SPRING", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TransitionType::LINEAR, TransitionType::SINE, TransitionType::QUINT, TransitionType::QUART, TransitionType::QUAD, TransitionType::EXPO, TransitionType::ELASTIC, TransitionType::CUBIC, TransitionType::CIRC, TransitionType::BOUNCE, TransitionType::BACK, TransitionType::SPRING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TransitionType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LINEAR", "TRANS_LINEAR", TransitionType::LINEAR), crate::meta::inspect::EnumConstant::new("SINE", "TRANS_SINE", TransitionType::SINE), crate::meta::inspect::EnumConstant::new("QUINT", "TRANS_QUINT", TransitionType::QUINT), crate::meta::inspect::EnumConstant::new("QUART", "TRANS_QUART", TransitionType::QUART), crate::meta::inspect::EnumConstant::new("QUAD", "TRANS_QUAD", TransitionType::QUAD), crate::meta::inspect::EnumConstant::new("EXPO", "TRANS_EXPO", TransitionType::EXPO), crate::meta::inspect::EnumConstant::new("ELASTIC", "TRANS_ELASTIC", TransitionType::ELASTIC), crate::meta::inspect::EnumConstant::new("CUBIC", "TRANS_CUBIC", TransitionType::CUBIC), crate::meta::inspect::EnumConstant::new("CIRC", "TRANS_CIRC", TransitionType::CIRC), crate::meta::inspect::EnumConstant::new("BOUNCE", "TRANS_BOUNCE", TransitionType::BOUNCE), crate::meta::inspect::EnumConstant::new("BACK", "TRANS_BACK", TransitionType::BACK), crate::meta::inspect::EnumConstant::new("SPRING", "TRANS_SPRING", TransitionType::SPRING)]
        }
    }
}
impl crate::meta::GodotConvert for TransitionType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TransitionType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TransitionType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EaseType {
    ord: i32
}
impl EaseType {
    #[doc(alias = "EASE_IN")]
    #[doc = "Godot enumerator name: `EASE_IN`"]
    pub const IN: EaseType = EaseType {
        ord: 0i32
    };
    #[doc(alias = "EASE_OUT")]
    #[doc = "Godot enumerator name: `EASE_OUT`"]
    pub const OUT: EaseType = EaseType {
        ord: 1i32
    };
    #[doc(alias = "EASE_IN_OUT")]
    #[doc = "Godot enumerator name: `EASE_IN_OUT`"]
    pub const IN_OUT: EaseType = EaseType {
        ord: 2i32
    };
    #[doc(alias = "EASE_OUT_IN")]
    #[doc = "Godot enumerator name: `EASE_OUT_IN`"]
    pub const OUT_IN: EaseType = EaseType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for EaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EaseType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EaseType {
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
            Self::IN => "IN", Self::OUT => "OUT", Self::IN_OUT => "IN_OUT", Self::OUT_IN => "OUT_IN", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EaseType::IN, EaseType::OUT, EaseType::IN_OUT, EaseType::OUT_IN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EaseType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("IN", "EASE_IN", EaseType::IN), crate::meta::inspect::EnumConstant::new("OUT", "EASE_OUT", EaseType::OUT), crate::meta::inspect::EnumConstant::new("IN_OUT", "EASE_IN_OUT", EaseType::IN_OUT), crate::meta::inspect::EnumConstant::new("OUT_IN", "EASE_OUT_IN", EaseType::OUT_IN)]
        }
    }
}
impl crate::meta::GodotConvert for EaseType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EaseType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EaseType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Tween;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Tween`][crate::classes::Tween] class."]
    pub struct SignalsOfTween < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfTween < 'c, C > {
        #[doc = "Signature: `(idx: i64)`"]
        pub fn step_finished(&mut self) -> SigStepFinished < 'c, C > {
            SigStepFinished {
                typed: TypedSignal::extract(&mut self.__internal_obj, "step_finished")
            }
        }
        #[doc = "Signature: `(loop_count: i64)`"]
        pub fn loop_finished(&mut self) -> SigLoopFinished < 'c, C > {
            SigLoopFinished {
                typed: TypedSignal::extract(&mut self.__internal_obj, "loop_finished")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn finished(&mut self) -> SigFinished < 'c, C > {
            SigFinished {
                typed: TypedSignal::extract(&mut self.__internal_obj, "finished")
            }
        }
    }
    type TypedSigStepFinished < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigStepFinished < 'c, C: WithSignals > {
        typed: TypedSigStepFinished < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigStepFinished < 'c, C > {
        pub fn emit(&mut self, idx: i64,) {
            self.typed.emit_tuple((idx,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigStepFinished < 'c, C > {
        type Target = TypedSigStepFinished < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigStepFinished < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigLoopFinished < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigLoopFinished < 'c, C: WithSignals > {
        typed: TypedSigLoopFinished < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigLoopFinished < 'c, C > {
        pub fn emit(&mut self, loop_count: i64,) {
            self.typed.emit_tuple((loop_count,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigLoopFinished < 'c, C > {
        type Target = TypedSigLoopFinished < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigLoopFinished < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
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
    impl WithSignals for Tween {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfTween < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfTween < 'c, C > {
        type Target = < < Tween as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Tween;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfTween < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Tween;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}