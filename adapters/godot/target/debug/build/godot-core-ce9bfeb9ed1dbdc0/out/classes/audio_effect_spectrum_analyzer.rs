#![doc = "Sidecar module for class [`AudioEffectSpectrumAnalyzer`][crate::classes::AudioEffectSpectrumAnalyzer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioEffectSpectrumAnalyzer` enums](https://docs.godotengine.org/en/stable/classes/class_audioeffectspectrumanalyzer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioEffectSpectrumAnalyzer.`\n\nInherits [`AudioEffect`][crate::classes::AudioEffect].\n\nRelated symbols:\n\n* [`audio_effect_spectrum_analyzer`][crate::classes::audio_effect_spectrum_analyzer]: sidecar module with related enum/flag types\n* [`IAudioEffectSpectrumAnalyzer`][crate::classes::IAudioEffectSpectrumAnalyzer]: virtual methods\n\n\nSee also [Godot docs for `AudioEffectSpectrumAnalyzer`](https://docs.godotengine.org/en/stable/classes/class_audioeffectspectrumanalyzer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AudioEffectSpectrumAnalyzer::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioEffectSpectrumAnalyzer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AudioEffectSpectrumAnalyzer`][crate::classes::AudioEffectSpectrumAnalyzer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IAudioEffect`][crate::classes::IAudioEffect] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AudioEffectSpectrumAnalyzer` methods](https://docs.godotengine.org/en/stable/classes/class_audioeffectspectrumanalyzer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioEffectSpectrumAnalyzer: crate::obj::GodotClass < Base = AudioEffectSpectrumAnalyzer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AudioEffectSpectrumAnalyzer {
        pub fn set_buffer_length(&mut self, seconds: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (seconds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(826usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectSpectrumAnalyzer", "set_buffer_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_buffer_length(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(827usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectSpectrumAnalyzer", "get_buffer_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tap_back_pos(&mut self, seconds: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (seconds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(828usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectSpectrumAnalyzer", "set_tap_back_pos", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tap_back_pos(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(829usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectSpectrumAnalyzer", "get_tap_back_pos", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fft_size(&mut self, size: crate::classes::audio_effect_spectrum_analyzer::FftSize,) {
            type CallRet = ();
            type CallParams = (crate::classes::audio_effect_spectrum_analyzer::FftSize,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(830usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectSpectrumAnalyzer", "set_fft_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fft_size(&self,) -> crate::classes::audio_effect_spectrum_analyzer::FftSize {
            type CallRet = crate::classes::audio_effect_spectrum_analyzer::FftSize;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(831usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectSpectrumAnalyzer", "get_fft_size", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AudioEffectSpectrumAnalyzer {
        type Base = crate::classes::AudioEffect;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AudioEffectSpectrumAnalyzer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioEffectSpectrumAnalyzer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioEffect > for AudioEffectSpectrumAnalyzer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AudioEffectSpectrumAnalyzer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioEffectSpectrumAnalyzer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioEffectSpectrumAnalyzer {
        
    }
    impl crate::obj::cap::GodotDefault for AudioEffectSpectrumAnalyzer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioEffectSpectrumAnalyzer {
        type Target = crate::classes::AudioEffect;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioEffectSpectrumAnalyzer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioEffectSpectrumAnalyzer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AudioEffectSpectrumAnalyzer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioEffectSpectrumAnalyzer > for $Class {
                
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
#[doc = "Godot enum name: `FFTSize`."]
pub struct FftSize {
    ord: i32
}
impl FftSize {
    #[doc(alias = "FFT_SIZE_256")]
    #[doc = "Godot enumerator name: `FFT_SIZE_256`"]
    pub const SIZE_256: FftSize = FftSize {
        ord: 0i32
    };
    #[doc(alias = "FFT_SIZE_512")]
    #[doc = "Godot enumerator name: `FFT_SIZE_512`"]
    pub const SIZE_512: FftSize = FftSize {
        ord: 1i32
    };
    #[doc(alias = "FFT_SIZE_1024")]
    #[doc = "Godot enumerator name: `FFT_SIZE_1024`"]
    pub const SIZE_1024: FftSize = FftSize {
        ord: 2i32
    };
    #[doc(alias = "FFT_SIZE_2048")]
    #[doc = "Godot enumerator name: `FFT_SIZE_2048`"]
    pub const SIZE_2048: FftSize = FftSize {
        ord: 3i32
    };
    #[doc(alias = "FFT_SIZE_4096")]
    #[doc = "Godot enumerator name: `FFT_SIZE_4096`"]
    pub const SIZE_4096: FftSize = FftSize {
        ord: 4i32
    };
    #[doc(alias = "FFT_SIZE_MAX")]
    #[doc = "Godot enumerator name: `FFT_SIZE_MAX`"]
    pub const MAX: FftSize = FftSize {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for FftSize {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FftSize") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FftSize {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::SIZE_256 => "SIZE_256", Self::SIZE_512 => "SIZE_512", Self::SIZE_1024 => "SIZE_1024", Self::SIZE_2048 => "SIZE_2048", Self::SIZE_4096 => "SIZE_4096", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FftSize::SIZE_256, FftSize::SIZE_512, FftSize::SIZE_1024, FftSize::SIZE_2048, FftSize::SIZE_4096]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FftSize >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SIZE_256", "FFT_SIZE_256", FftSize::SIZE_256), crate::meta::inspect::EnumConstant::new("SIZE_512", "FFT_SIZE_512", FftSize::SIZE_512), crate::meta::inspect::EnumConstant::new("SIZE_1024", "FFT_SIZE_1024", FftSize::SIZE_1024), crate::meta::inspect::EnumConstant::new("SIZE_2048", "FFT_SIZE_2048", FftSize::SIZE_2048), crate::meta::inspect::EnumConstant::new("SIZE_4096", "FFT_SIZE_4096", FftSize::SIZE_4096), crate::meta::inspect::EnumConstant::new("MAX", "FFT_SIZE_MAX", FftSize::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for FftSize {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for FftSize {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FftSize {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FftSize {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AudioEffectSpectrumAnalyzer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for AudioEffectSpectrumAnalyzer {
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