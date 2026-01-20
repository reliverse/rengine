#![doc = "Sidecar module for class [`Timer`][crate::classes::Timer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Timer` enums](https://docs.godotengine.org/en/stable/classes/class_timer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Timer.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`timer`][crate::classes::timer]: sidecar module with related enum/flag types\n* [`ITimer`][crate::classes::ITimer]: virtual methods\n* [`SignalsOfTimer`][crate::classes::timer::SignalsOfTimer]: signal collection\n\n\nSee also [Godot docs for `Timer`](https://docs.godotengine.org/en/stable/classes/class_timer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Timer::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Timer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Timer`][crate::classes::Timer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Timer` methods](https://docs.godotengine.org/en/stable/classes/class_timer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITimer: crate::obj::GodotClass < Base = Timer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Timer {
        pub fn set_wait_time(&mut self, time_sec: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (time_sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9987usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Timer", "set_wait_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_wait_time(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9988usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Timer", "get_wait_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_one_shot(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9989usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Timer", "set_one_shot", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_one_shot(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9990usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Timer", "is_one_shot", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autostart(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9991usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Timer", "set_autostart", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_autostart(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9992usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Timer", "has_autostart", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn start_full(&mut self, time_sec: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (time_sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9993usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Timer", "start", Some(self.__validated_obj()), args,)
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
        pub fn stop(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9994usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Timer", "stop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_paused(&mut self, paused: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (paused,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9995usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Timer", "set_paused", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_paused(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9996usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Timer", "is_paused", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ignore_time_scale(&mut self, ignore: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (ignore,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9997usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Timer", "set_ignore_time_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_ignoring_time_scale(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9998usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Timer", "is_ignoring_time_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_stopped(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9999usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Timer", "is_stopped", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_time_left(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10000usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Timer", "get_time_left", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_timer_process_callback(&mut self, callback: crate::classes::timer::TimerProcessCallback,) {
            type CallRet = ();
            type CallParams = (crate::classes::timer::TimerProcessCallback,);
            let args = (callback,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10001usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Timer", "set_timer_process_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_timer_process_callback(&self,) -> crate::classes::timer::TimerProcessCallback {
            type CallRet = crate::classes::timer::TimerProcessCallback;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10002usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Timer", "get_timer_process_callback", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Timer {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Timer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Timer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Timer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Timer {
        
    }
    impl crate::obj::cap::GodotDefault for Timer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Timer {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Timer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Timer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Timer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Timer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Timer::start_ex`][super::Timer::start_ex]."]
#[must_use]
pub struct ExStart < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Timer, time_sec: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStart < 'a > {
    fn new(surround_object: &'a mut re_export::Timer,) -> Self {
        let time_sec = - 1f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, time_sec: time_sec,
        }
    }
    #[inline]
    pub fn time_sec(self, time_sec: f64) -> Self {
        Self {
            time_sec: time_sec, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, time_sec,
        }
        = self;
        re_export::Timer::start_full(surround_object, time_sec,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TimerProcessCallback {
    ord: i32
}
impl TimerProcessCallback {
    #[doc(alias = "TIMER_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `TIMER_PROCESS_PHYSICS`"]
    pub const PHYSICS: TimerProcessCallback = TimerProcessCallback {
        ord: 0i32
    };
    #[doc(alias = "TIMER_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `TIMER_PROCESS_IDLE`"]
    pub const IDLE: TimerProcessCallback = TimerProcessCallback {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for TimerProcessCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TimerProcessCallback") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TimerProcessCallback {
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
        &[TimerProcessCallback::PHYSICS, TimerProcessCallback::IDLE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TimerProcessCallback >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PHYSICS", "TIMER_PROCESS_PHYSICS", TimerProcessCallback::PHYSICS), crate::meta::inspect::EnumConstant::new("IDLE", "TIMER_PROCESS_IDLE", TimerProcessCallback::IDLE)]
        }
    }
}
impl crate::meta::GodotConvert for TimerProcessCallback {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TimerProcessCallback {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TimerProcessCallback {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Timer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Timer`][crate::classes::Timer] class."]
    pub struct SignalsOfTimer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfTimer < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn timeout(&mut self) -> SigTimeout < 'c, C > {
            SigTimeout {
                typed: TypedSignal::extract(&mut self.__internal_obj, "timeout")
            }
        }
    }
    type TypedSigTimeout < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTimeout < 'c, C: WithSignals > {
        typed: TypedSigTimeout < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTimeout < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTimeout < 'c, C > {
        type Target = TypedSigTimeout < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTimeout < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Timer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfTimer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfTimer < 'c, C > {
        type Target = < < Timer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Timer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfTimer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Timer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}