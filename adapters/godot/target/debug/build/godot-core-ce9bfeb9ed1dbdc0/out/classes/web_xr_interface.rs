#![doc = "Sidecar module for class [`WebXrInterface`][crate::classes::WebXrInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebXRInterface` enums](https://docs.godotengine.org/en/stable/classes/class_webxrinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WebXRInterface.`\n\nInherits [`XrInterface`][crate::classes::XrInterface].\n\nRelated symbols:\n\n* [`web_xr_interface`][crate::classes::web_xr_interface]: sidecar module with related enum/flag types\n* [`SignalsOfWebXrInterface`][crate::classes::web_xr_interface::SignalsOfWebXrInterface]: signal collection\n\n\nSee also [Godot docs for `WebXRInterface`](https://docs.godotengine.org/en/stable/classes/class_webxrinterface.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<WebXrInterface>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WebXrInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl WebXrInterface {
        pub fn is_session_supported(&mut self, session_mode: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (session_mode.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10969usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "is_session_supported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_session_mode(&mut self, session_mode: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (session_mode.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10970usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "set_session_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_session_mode(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10971usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_session_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_required_features(&mut self, required_features: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (required_features.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10972usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "set_required_features", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_required_features(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10973usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_required_features", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_optional_features(&mut self, optional_features: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (optional_features.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10974usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "set_optional_features", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_optional_features(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10975usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_optional_features", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_reference_space_type(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10976usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_reference_space_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_enabled_features(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10977usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_enabled_features", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_requested_reference_space_types(&mut self, requested_reference_space_types: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (requested_reference_space_types.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10978usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "set_requested_reference_space_types", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_requested_reference_space_types(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10979usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_requested_reference_space_types", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_input_source_active(&self, input_source_id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (input_source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10980usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "is_input_source_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_input_source_tracker(&self, input_source_id: i32,) -> Option < Gd < crate::classes::XrControllerTracker > > {
            type CallRet = Option < Gd < crate::classes::XrControllerTracker > >;
            type CallParams = (i32,);
            let args = (input_source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10981usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_input_source_tracker", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_input_source_target_ray_mode(&self, input_source_id: i32,) -> crate::classes::web_xr_interface::TargetRayMode {
            type CallRet = crate::classes::web_xr_interface::TargetRayMode;
            type CallParams = (i32,);
            let args = (input_source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10982usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_input_source_target_ray_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_state(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10983usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_visibility_state", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_display_refresh_rate(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10984usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_display_refresh_rate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_display_refresh_rate(&mut self, refresh_rate: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (refresh_rate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10985usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "set_display_refresh_rate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_available_display_refresh_rates(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10986usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_available_display_refresh_rates", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for WebXrInterface {
        type Base = crate::classes::XrInterface;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("WebXRInterface"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WebXrInterface {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrInterface > for WebXrInterface {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for WebXrInterface {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for WebXrInterface {
        
    }
    impl std::ops::Deref for WebXrInterface {
        type Target = crate::classes::XrInterface;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WebXrInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_WebXrInterface__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `WebXrInterface` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TargetRayMode {
    ord: i32
}
impl TargetRayMode {
    #[doc(alias = "TARGET_RAY_MODE_UNKNOWN")]
    #[doc = "Godot enumerator name: `TARGET_RAY_MODE_UNKNOWN`"]
    pub const UNKNOWN: TargetRayMode = TargetRayMode {
        ord: 0i32
    };
    #[doc(alias = "TARGET_RAY_MODE_GAZE")]
    #[doc = "Godot enumerator name: `TARGET_RAY_MODE_GAZE`"]
    pub const GAZE: TargetRayMode = TargetRayMode {
        ord: 1i32
    };
    #[doc(alias = "TARGET_RAY_MODE_TRACKED_POINTER")]
    #[doc = "Godot enumerator name: `TARGET_RAY_MODE_TRACKED_POINTER`"]
    pub const TRACKED_POINTER: TargetRayMode = TargetRayMode {
        ord: 2i32
    };
    #[doc(alias = "TARGET_RAY_MODE_SCREEN")]
    #[doc = "Godot enumerator name: `TARGET_RAY_MODE_SCREEN`"]
    pub const SCREEN: TargetRayMode = TargetRayMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TargetRayMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TargetRayMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TargetRayMode {
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
            Self::UNKNOWN => "UNKNOWN", Self::GAZE => "GAZE", Self::TRACKED_POINTER => "TRACKED_POINTER", Self::SCREEN => "SCREEN", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TargetRayMode::UNKNOWN, TargetRayMode::GAZE, TargetRayMode::TRACKED_POINTER, TargetRayMode::SCREEN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TargetRayMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNKNOWN", "TARGET_RAY_MODE_UNKNOWN", TargetRayMode::UNKNOWN), crate::meta::inspect::EnumConstant::new("GAZE", "TARGET_RAY_MODE_GAZE", TargetRayMode::GAZE), crate::meta::inspect::EnumConstant::new("TRACKED_POINTER", "TARGET_RAY_MODE_TRACKED_POINTER", TargetRayMode::TRACKED_POINTER), crate::meta::inspect::EnumConstant::new("SCREEN", "TARGET_RAY_MODE_SCREEN", TargetRayMode::SCREEN)]
        }
    }
}
impl crate::meta::GodotConvert for TargetRayMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TargetRayMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TargetRayMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::WebXrInterface;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`WebXrInterface`][crate::classes::WebXrInterface] class."]
    pub struct SignalsOfWebXrInterface < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfWebXrInterface < 'c, C > {
        #[doc = "Signature: `(session_mode: GString, supported: bool)`"]
        pub fn session_supported(&mut self) -> SigSessionSupported < 'c, C > {
            SigSessionSupported {
                typed: TypedSignal::extract(&mut self.__internal_obj, "session_supported")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn session_started(&mut self) -> SigSessionStarted < 'c, C > {
            SigSessionStarted {
                typed: TypedSignal::extract(&mut self.__internal_obj, "session_started")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn session_ended(&mut self) -> SigSessionEnded < 'c, C > {
            SigSessionEnded {
                typed: TypedSignal::extract(&mut self.__internal_obj, "session_ended")
            }
        }
        #[doc = "Signature: `(message: GString)`"]
        pub fn session_failed(&mut self) -> SigSessionFailed < 'c, C > {
            SigSessionFailed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "session_failed")
            }
        }
        #[doc = "Signature: `(input_source_id: i64)`"]
        pub fn selectstart(&mut self) -> SigSelectstart < 'c, C > {
            SigSelectstart {
                typed: TypedSignal::extract(&mut self.__internal_obj, "selectstart")
            }
        }
        #[doc = "Signature: `(input_source_id: i64)`"]
        pub fn select(&mut self) -> SigSelect < 'c, C > {
            SigSelect {
                typed: TypedSignal::extract(&mut self.__internal_obj, "select")
            }
        }
        #[doc = "Signature: `(input_source_id: i64)`"]
        pub fn selectend(&mut self) -> SigSelectend < 'c, C > {
            SigSelectend {
                typed: TypedSignal::extract(&mut self.__internal_obj, "selectend")
            }
        }
        #[doc = "Signature: `(input_source_id: i64)`"]
        pub fn squeezestart(&mut self) -> SigSqueezestart < 'c, C > {
            SigSqueezestart {
                typed: TypedSignal::extract(&mut self.__internal_obj, "squeezestart")
            }
        }
        #[doc = "Signature: `(input_source_id: i64)`"]
        pub fn squeeze(&mut self) -> SigSqueeze < 'c, C > {
            SigSqueeze {
                typed: TypedSignal::extract(&mut self.__internal_obj, "squeeze")
            }
        }
        #[doc = "Signature: `(input_source_id: i64)`"]
        pub fn squeezeend(&mut self) -> SigSqueezeend < 'c, C > {
            SigSqueezeend {
                typed: TypedSignal::extract(&mut self.__internal_obj, "squeezeend")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn visibility_state_changed(&mut self) -> SigVisibilityStateChanged < 'c, C > {
            SigVisibilityStateChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "visibility_state_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn reference_space_reset(&mut self) -> SigReferenceSpaceReset < 'c, C > {
            SigReferenceSpaceReset {
                typed: TypedSignal::extract(&mut self.__internal_obj, "reference_space_reset")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn display_refresh_rate_changed(&mut self) -> SigDisplayRefreshRateChanged < 'c, C > {
            SigDisplayRefreshRateChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "display_refresh_rate_changed")
            }
        }
    }
    type TypedSigSessionSupported < 'c, C > = TypedSignal < 'c, C, (GString, bool,) >;
    pub struct SigSessionSupported < 'c, C: WithSignals > {
        typed: TypedSigSessionSupported < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSessionSupported < 'c, C > {
        pub fn emit(&mut self, session_mode: GString, supported: bool,) {
            self.typed.emit_tuple((session_mode, supported,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSessionSupported < 'c, C > {
        type Target = TypedSigSessionSupported < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSessionSupported < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSessionStarted < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSessionStarted < 'c, C: WithSignals > {
        typed: TypedSigSessionStarted < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSessionStarted < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSessionStarted < 'c, C > {
        type Target = TypedSigSessionStarted < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSessionStarted < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSessionEnded < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSessionEnded < 'c, C: WithSignals > {
        typed: TypedSigSessionEnded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSessionEnded < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSessionEnded < 'c, C > {
        type Target = TypedSigSessionEnded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSessionEnded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSessionFailed < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigSessionFailed < 'c, C: WithSignals > {
        typed: TypedSigSessionFailed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSessionFailed < 'c, C > {
        pub fn emit(&mut self, message: GString,) {
            self.typed.emit_tuple((message,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSessionFailed < 'c, C > {
        type Target = TypedSigSessionFailed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSessionFailed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSelectstart < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigSelectstart < 'c, C: WithSignals > {
        typed: TypedSigSelectstart < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSelectstart < 'c, C > {
        pub fn emit(&mut self, input_source_id: i64,) {
            self.typed.emit_tuple((input_source_id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSelectstart < 'c, C > {
        type Target = TypedSigSelectstart < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSelectstart < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSelect < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigSelect < 'c, C: WithSignals > {
        typed: TypedSigSelect < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSelect < 'c, C > {
        pub fn emit(&mut self, input_source_id: i64,) {
            self.typed.emit_tuple((input_source_id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSelect < 'c, C > {
        type Target = TypedSigSelect < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSelect < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSelectend < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigSelectend < 'c, C: WithSignals > {
        typed: TypedSigSelectend < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSelectend < 'c, C > {
        pub fn emit(&mut self, input_source_id: i64,) {
            self.typed.emit_tuple((input_source_id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSelectend < 'c, C > {
        type Target = TypedSigSelectend < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSelectend < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSqueezestart < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigSqueezestart < 'c, C: WithSignals > {
        typed: TypedSigSqueezestart < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSqueezestart < 'c, C > {
        pub fn emit(&mut self, input_source_id: i64,) {
            self.typed.emit_tuple((input_source_id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSqueezestart < 'c, C > {
        type Target = TypedSigSqueezestart < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSqueezestart < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSqueeze < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigSqueeze < 'c, C: WithSignals > {
        typed: TypedSigSqueeze < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSqueeze < 'c, C > {
        pub fn emit(&mut self, input_source_id: i64,) {
            self.typed.emit_tuple((input_source_id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSqueeze < 'c, C > {
        type Target = TypedSigSqueeze < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSqueeze < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSqueezeend < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigSqueezeend < 'c, C: WithSignals > {
        typed: TypedSigSqueezeend < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSqueezeend < 'c, C > {
        pub fn emit(&mut self, input_source_id: i64,) {
            self.typed.emit_tuple((input_source_id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSqueezeend < 'c, C > {
        type Target = TypedSigSqueezeend < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSqueezeend < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigVisibilityStateChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigVisibilityStateChanged < 'c, C: WithSignals > {
        typed: TypedSigVisibilityStateChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigVisibilityStateChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigVisibilityStateChanged < 'c, C > {
        type Target = TypedSigVisibilityStateChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigVisibilityStateChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigReferenceSpaceReset < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigReferenceSpaceReset < 'c, C: WithSignals > {
        typed: TypedSigReferenceSpaceReset < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigReferenceSpaceReset < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigReferenceSpaceReset < 'c, C > {
        type Target = TypedSigReferenceSpaceReset < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigReferenceSpaceReset < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigDisplayRefreshRateChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigDisplayRefreshRateChanged < 'c, C: WithSignals > {
        typed: TypedSigDisplayRefreshRateChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDisplayRefreshRateChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDisplayRefreshRateChanged < 'c, C > {
        type Target = TypedSigDisplayRefreshRateChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDisplayRefreshRateChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for WebXrInterface {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfWebXrInterface < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfWebXrInterface < 'c, C > {
        type Target = < < WebXrInterface as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = WebXrInterface;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfWebXrInterface < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = WebXrInterface;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}