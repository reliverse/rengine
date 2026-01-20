#![doc = "Sidecar module for class [`AudioEffectSpectrumAnalyzerInstance`][crate::classes::AudioEffectSpectrumAnalyzerInstance].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioEffectSpectrumAnalyzerInstance` enums](https://docs.godotengine.org/en/stable/classes/class_audioeffectspectrumanalyzerinstance.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioEffectSpectrumAnalyzerInstance.`\n\nInherits [`AudioEffectInstance`][crate::classes::AudioEffectInstance].\n\nRelated symbols:\n\n* [`audio_effect_spectrum_analyzer_instance`][crate::classes::audio_effect_spectrum_analyzer_instance]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `AudioEffectSpectrumAnalyzerInstance`](https://docs.godotengine.org/en/stable/classes/class_audioeffectspectrumanalyzerinstance.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<AudioEffectSpectrumAnalyzerInstance>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioEffectSpectrumAnalyzerInstance {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl AudioEffectSpectrumAnalyzerInstance {
        pub(crate) fn get_magnitude_for_frequency_range_full(&self, from_hz: f32, to_hz: f32, mode: crate::classes::audio_effect_spectrum_analyzer_instance::MagnitudeMode,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (f32, f32, crate::classes::audio_effect_spectrum_analyzer_instance::MagnitudeMode,);
            let args = (from_hz, to_hz, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(832usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectSpectrumAnalyzerInstance", "get_magnitude_for_frequency_range", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_magnitude_for_frequency_range_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_magnitude_for_frequency_range(&self, from_hz: f32, to_hz: f32,) -> Vector2 {
            self.get_magnitude_for_frequency_range_ex(from_hz, to_hz,) . done()
        }
        #[inline]
        pub fn get_magnitude_for_frequency_range_ex < 'a > (&'a self, from_hz: f32, to_hz: f32,) -> ExGetMagnitudeForFrequencyRange < 'a > {
            ExGetMagnitudeForFrequencyRange::new(self, from_hz, to_hz,)
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
    impl crate::obj::GodotClass for AudioEffectSpectrumAnalyzerInstance {
        type Base = crate::classes::AudioEffectInstance;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AudioEffectSpectrumAnalyzerInstance"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioEffectSpectrumAnalyzerInstance {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioEffectInstance > for AudioEffectSpectrumAnalyzerInstance {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioEffectSpectrumAnalyzerInstance {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioEffectSpectrumAnalyzerInstance {
        
    }
    impl std::ops::Deref for AudioEffectSpectrumAnalyzerInstance {
        type Target = crate::classes::AudioEffectInstance;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioEffectSpectrumAnalyzerInstance {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AudioEffectSpectrumAnalyzerInstance__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `AudioEffectSpectrumAnalyzerInstance` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`AudioEffectSpectrumAnalyzerInstance::get_magnitude_for_frequency_range_ex`][super::AudioEffectSpectrumAnalyzerInstance::get_magnitude_for_frequency_range_ex]."]
#[must_use]
pub struct ExGetMagnitudeForFrequencyRange < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::AudioEffectSpectrumAnalyzerInstance, from_hz: f32, to_hz: f32, mode: crate::classes::audio_effect_spectrum_analyzer_instance::MagnitudeMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetMagnitudeForFrequencyRange < 'a > {
    fn new(surround_object: &'a re_export::AudioEffectSpectrumAnalyzerInstance, from_hz: f32, to_hz: f32,) -> Self {
        let mode = crate::obj::EngineEnum::from_ord(1);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_hz: from_hz, to_hz: to_hz, mode: mode,
        }
    }
    #[inline]
    pub fn mode(self, mode: crate::classes::audio_effect_spectrum_analyzer_instance::MagnitudeMode) -> Self {
        Self {
            mode: mode, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        let Self {
            _phantom, surround_object, from_hz, to_hz, mode,
        }
        = self;
        re_export::AudioEffectSpectrumAnalyzerInstance::get_magnitude_for_frequency_range_full(surround_object, from_hz, to_hz, mode,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MagnitudeMode {
    ord: i32
}
impl MagnitudeMode {
    #[doc(alias = "MAGNITUDE_AVERAGE")]
    #[doc = "Godot enumerator name: `MAGNITUDE_AVERAGE`"]
    pub const AVERAGE: MagnitudeMode = MagnitudeMode {
        ord: 0i32
    };
    #[doc(alias = "MAGNITUDE_MAX")]
    #[doc = "Godot enumerator name: `MAGNITUDE_MAX`"]
    pub const MAX: MagnitudeMode = MagnitudeMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for MagnitudeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MagnitudeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MagnitudeMode {
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
            Self::AVERAGE => "AVERAGE", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MagnitudeMode::AVERAGE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MagnitudeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("AVERAGE", "MAGNITUDE_AVERAGE", MagnitudeMode::AVERAGE), crate::meta::inspect::EnumConstant::new("MAX", "MAGNITUDE_MAX", MagnitudeMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for MagnitudeMode {
    const ENUMERATOR_COUNT: usize = 1usize;
    
}
impl crate::meta::GodotConvert for MagnitudeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MagnitudeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MagnitudeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AudioEffectSpectrumAnalyzerInstance;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for AudioEffectSpectrumAnalyzerInstance {
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