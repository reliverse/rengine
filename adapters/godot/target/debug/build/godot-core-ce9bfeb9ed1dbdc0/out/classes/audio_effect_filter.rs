#![doc = "Sidecar module for class [`AudioEffectFilter`][crate::classes::AudioEffectFilter].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioEffectFilter` enums](https://docs.godotengine.org/en/stable/classes/class_audioeffectfilter.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioEffectFilter.`\n\nInherits [`AudioEffect`][crate::classes::AudioEffect].\n\nRelated symbols:\n\n* [`audio_effect_filter`][crate::classes::audio_effect_filter]: sidecar module with related enum/flag types\n* [`IAudioEffectFilter`][crate::classes::IAudioEffectFilter]: virtual methods\n\n\nSee also [Godot docs for `AudioEffectFilter`](https://docs.godotengine.org/en/stable/classes/class_audioeffectfilter.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AudioEffectFilter::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioEffectFilter {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AudioEffectFilter`][crate::classes::AudioEffectFilter].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IAudioEffect`][crate::classes::IAudioEffect] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AudioEffectFilter` methods](https://docs.godotengine.org/en/stable/classes/class_audioeffectfilter.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioEffectFilter: crate::obj::GodotClass < Base = AudioEffectFilter > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn instantiate(&mut self,) -> Option < Gd < crate::classes::AudioEffectInstance > >;
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
    impl AudioEffectFilter {
        pub fn set_cutoff(&mut self, freq: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (freq,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(765usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectFilter", "set_cutoff", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cutoff(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(766usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectFilter", "get_cutoff", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_resonance(&mut self, amount: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(767usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectFilter", "set_resonance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_resonance(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(768usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectFilter", "get_resonance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gain(&mut self, amount: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(769usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectFilter", "set_gain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gain(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(770usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectFilter", "get_gain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_db(&mut self, amount: crate::classes::audio_effect_filter::FilterDb,) {
            type CallRet = ();
            type CallParams = (crate::classes::audio_effect_filter::FilterDb,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(771usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectFilter", "set_db", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_db(&self,) -> crate::classes::audio_effect_filter::FilterDb {
            type CallRet = crate::classes::audio_effect_filter::FilterDb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(772usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectFilter", "get_db", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AudioEffectFilter {
        type Base = crate::classes::AudioEffect;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AudioEffectFilter"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioEffectFilter {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioEffect > for AudioEffectFilter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AudioEffectFilter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioEffectFilter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioEffectFilter {
        
    }
    impl crate::obj::cap::GodotDefault for AudioEffectFilter {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioEffectFilter {
        type Target = crate::classes::AudioEffect;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioEffectFilter {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioEffectFilter`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AudioEffectFilter__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioEffectFilter > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioEffect > for $Class {
                
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
#[doc = "Godot enum name: `FilterDB`."]
pub struct FilterDb {
    ord: i32
}
impl FilterDb {
    pub const FILTER_6DB: FilterDb = FilterDb {
        ord: 0i32
    };
    pub const FILTER_12DB: FilterDb = FilterDb {
        ord: 1i32
    };
    pub const FILTER_18DB: FilterDb = FilterDb {
        ord: 2i32
    };
    pub const FILTER_24DB: FilterDb = FilterDb {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for FilterDb {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FilterDb") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FilterDb {
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
            Self::FILTER_6DB => "FILTER_6DB", Self::FILTER_12DB => "FILTER_12DB", Self::FILTER_18DB => "FILTER_18DB", Self::FILTER_24DB => "FILTER_24DB", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FilterDb::FILTER_6DB, FilterDb::FILTER_12DB, FilterDb::FILTER_18DB, FilterDb::FILTER_24DB]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FilterDb >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FILTER_6DB", "FILTER_6DB", FilterDb::FILTER_6DB), crate::meta::inspect::EnumConstant::new("FILTER_12DB", "FILTER_12DB", FilterDb::FILTER_12DB), crate::meta::inspect::EnumConstant::new("FILTER_18DB", "FILTER_18DB", FilterDb::FILTER_18DB), crate::meta::inspect::EnumConstant::new("FILTER_24DB", "FILTER_24DB", FilterDb::FILTER_24DB)]
        }
    }
}
impl crate::meta::GodotConvert for FilterDb {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FilterDb {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FilterDb {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AudioEffectFilter;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for AudioEffectFilter {
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