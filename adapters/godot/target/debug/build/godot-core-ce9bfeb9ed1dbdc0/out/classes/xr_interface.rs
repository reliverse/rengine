#![doc = "Sidecar module for class [`XrInterface`][crate::classes::XrInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `XRInterface` enums](https://docs.godotengine.org/en/stable/classes/class_xrinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `XRInterface.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`xr_interface`][crate::classes::xr_interface]: sidecar module with related enum/flag types\n* [`SignalsOfXrInterface`][crate::classes::xr_interface::SignalsOfXrInterface]: signal collection\n\n\nSee also [Godot docs for `XRInterface`](https://docs.godotengine.org/en/stable/classes/class_xrinterface.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<XrInterface>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct XrInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl XrInterface {
        pub fn get_name(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "get_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_capabilities(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "get_capabilities", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_primary(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "is_primary", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_primary(&mut self, primary: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (primary,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "set_primary", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_initialized(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "is_initialized", Some(self.__validated_obj()), args,)
            }
        }
        pub fn initialize(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "initialize", Some(self.__validated_obj()), args,)
            }
        }
        pub fn uninitialize(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "uninitialize", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_system_info(&mut self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "get_system_info", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tracking_status(&self,) -> crate::classes::xr_interface::TrackingStatus {
            type CallRet = crate::classes::xr_interface::TrackingStatus;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "get_tracking_status", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_render_target_size(&mut self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "get_render_target_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_view_count(&mut self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "get_view_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn trigger_haptic_pulse(&mut self, action_name: impl AsArg < GString >, tracker_name: impl AsArg < StringName >, frequency: f64, amplitude: f64, duration_sec: f64, delay_sec: f64,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, StringName >, f64, f64, f64, f64,);
            let args = (action_name.into_arg(), tracker_name.into_arg(), frequency, amplitude, duration_sec, delay_sec,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "trigger_haptic_pulse", Some(self.__validated_obj()), args,)
            }
        }
        pub fn supports_play_area_mode(&mut self, mode: crate::classes::xr_interface::PlayAreaMode,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::xr_interface::PlayAreaMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "supports_play_area_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_play_area_mode(&self,) -> crate::classes::xr_interface::PlayAreaMode {
            type CallRet = crate::classes::xr_interface::PlayAreaMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "get_play_area_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_play_area_mode(&mut self, mode: crate::classes::xr_interface::PlayAreaMode,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::xr_interface::PlayAreaMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "set_play_area_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_play_area(&self,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "get_play_area", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_anchor_detection_is_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "get_anchor_detection_is_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_anchor_detection_is_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "set_anchor_detection_is_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_camera_feed_id(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "get_camera_feed_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_passthrough_supported(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "is_passthrough_supported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_passthrough_enabled(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "is_passthrough_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn start_passthrough(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "start_passthrough", Some(self.__validated_obj()), args,)
            }
        }
        pub fn stop_passthrough(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "stop_passthrough", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transform_for_view(&mut self, view: u32, cam_transform: Transform3D,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (u32, Transform3D,);
            let args = (view, cam_transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "get_transform_for_view", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_projection_for_view(&mut self, view: u32, aspect: f64, near: f64, far: f64,) -> Projection {
            type CallRet = Projection;
            type CallParams = (u32, f64, f64, f64,);
            let args = (view, aspect, near, far,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "get_projection_for_view", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_supported_environment_blend_modes(&mut self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "get_supported_environment_blend_modes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_environment_blend_mode(&mut self, mode: crate::classes::xr_interface::EnvironmentBlendMode,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::xr_interface::EnvironmentBlendMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "set_environment_blend_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_environment_blend_mode(&self,) -> crate::classes::xr_interface::EnvironmentBlendMode {
            type CallRet = crate::classes::xr_interface::EnvironmentBlendMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrInterface", "get_environment_blend_mode", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for XrInterface {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("XRInterface"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for XrInterface {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for XrInterface {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for XrInterface {
        
    }
    impl std::ops::Deref for XrInterface {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for XrInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_XrInterface__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `XrInterface` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Capabilities {
    ord: i32
}
impl Capabilities {
    #[doc(alias = "XR_NONE")]
    #[doc = "Godot enumerator name: `XR_NONE`"]
    pub const NONE: Capabilities = Capabilities {
        ord: 0i32
    };
    #[doc(alias = "XR_MONO")]
    #[doc = "Godot enumerator name: `XR_MONO`"]
    pub const MONO: Capabilities = Capabilities {
        ord: 1i32
    };
    #[doc(alias = "XR_STEREO")]
    #[doc = "Godot enumerator name: `XR_STEREO`"]
    pub const STEREO: Capabilities = Capabilities {
        ord: 2i32
    };
    #[doc(alias = "XR_QUAD")]
    #[doc = "Godot enumerator name: `XR_QUAD`"]
    pub const QUAD: Capabilities = Capabilities {
        ord: 4i32
    };
    #[doc(alias = "XR_VR")]
    #[doc = "Godot enumerator name: `XR_VR`"]
    pub const VR: Capabilities = Capabilities {
        ord: 8i32
    };
    #[doc(alias = "XR_AR")]
    #[doc = "Godot enumerator name: `XR_AR`"]
    pub const AR: Capabilities = Capabilities {
        ord: 16i32
    };
    #[doc(alias = "XR_EXTERNAL")]
    #[doc = "Godot enumerator name: `XR_EXTERNAL`"]
    pub const EXTERNAL: Capabilities = Capabilities {
        ord: 32i32
    };
    
}
impl std::fmt::Debug for Capabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Capabilities") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Capabilities {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 | ord @ 32i32 => Some(Self {
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
            Self::NONE => "NONE", Self::MONO => "MONO", Self::STEREO => "STEREO", Self::QUAD => "QUAD", Self::VR => "VR", Self::AR => "AR", Self::EXTERNAL => "EXTERNAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Capabilities::NONE, Capabilities::MONO, Capabilities::STEREO, Capabilities::QUAD, Capabilities::VR, Capabilities::AR, Capabilities::EXTERNAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Capabilities >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "XR_NONE", Capabilities::NONE), crate::meta::inspect::EnumConstant::new("MONO", "XR_MONO", Capabilities::MONO), crate::meta::inspect::EnumConstant::new("STEREO", "XR_STEREO", Capabilities::STEREO), crate::meta::inspect::EnumConstant::new("QUAD", "XR_QUAD", Capabilities::QUAD), crate::meta::inspect::EnumConstant::new("VR", "XR_VR", Capabilities::VR), crate::meta::inspect::EnumConstant::new("AR", "XR_AR", Capabilities::AR), crate::meta::inspect::EnumConstant::new("EXTERNAL", "XR_EXTERNAL", Capabilities::EXTERNAL)]
        }
    }
}
impl crate::meta::GodotConvert for Capabilities {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Capabilities {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Capabilities {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TrackingStatus {
    ord: i32
}
impl TrackingStatus {
    #[doc(alias = "XR_NORMAL_TRACKING")]
    #[doc = "Godot enumerator name: `XR_NORMAL_TRACKING`"]
    pub const NORMAL_TRACKING: TrackingStatus = TrackingStatus {
        ord: 0i32
    };
    #[doc(alias = "XR_EXCESSIVE_MOTION")]
    #[doc = "Godot enumerator name: `XR_EXCESSIVE_MOTION`"]
    pub const EXCESSIVE_MOTION: TrackingStatus = TrackingStatus {
        ord: 1i32
    };
    #[doc(alias = "XR_INSUFFICIENT_FEATURES")]
    #[doc = "Godot enumerator name: `XR_INSUFFICIENT_FEATURES`"]
    pub const INSUFFICIENT_FEATURES: TrackingStatus = TrackingStatus {
        ord: 2i32
    };
    #[doc(alias = "XR_UNKNOWN_TRACKING")]
    #[doc = "Godot enumerator name: `XR_UNKNOWN_TRACKING`"]
    pub const UNKNOWN_TRACKING: TrackingStatus = TrackingStatus {
        ord: 3i32
    };
    #[doc(alias = "XR_NOT_TRACKING")]
    #[doc = "Godot enumerator name: `XR_NOT_TRACKING`"]
    pub const NOT_TRACKING: TrackingStatus = TrackingStatus {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for TrackingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TrackingStatus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TrackingStatus {
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
            Self::NORMAL_TRACKING => "NORMAL_TRACKING", Self::EXCESSIVE_MOTION => "EXCESSIVE_MOTION", Self::INSUFFICIENT_FEATURES => "INSUFFICIENT_FEATURES", Self::UNKNOWN_TRACKING => "UNKNOWN_TRACKING", Self::NOT_TRACKING => "NOT_TRACKING", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TrackingStatus::NORMAL_TRACKING, TrackingStatus::EXCESSIVE_MOTION, TrackingStatus::INSUFFICIENT_FEATURES, TrackingStatus::UNKNOWN_TRACKING, TrackingStatus::NOT_TRACKING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TrackingStatus >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NORMAL_TRACKING", "XR_NORMAL_TRACKING", TrackingStatus::NORMAL_TRACKING), crate::meta::inspect::EnumConstant::new("EXCESSIVE_MOTION", "XR_EXCESSIVE_MOTION", TrackingStatus::EXCESSIVE_MOTION), crate::meta::inspect::EnumConstant::new("INSUFFICIENT_FEATURES", "XR_INSUFFICIENT_FEATURES", TrackingStatus::INSUFFICIENT_FEATURES), crate::meta::inspect::EnumConstant::new("UNKNOWN_TRACKING", "XR_UNKNOWN_TRACKING", TrackingStatus::UNKNOWN_TRACKING), crate::meta::inspect::EnumConstant::new("NOT_TRACKING", "XR_NOT_TRACKING", TrackingStatus::NOT_TRACKING)]
        }
    }
}
impl crate::meta::GodotConvert for TrackingStatus {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TrackingStatus {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TrackingStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PlayAreaMode {
    ord: i32
}
impl PlayAreaMode {
    #[doc(alias = "XR_PLAY_AREA_UNKNOWN")]
    #[doc = "Godot enumerator name: `XR_PLAY_AREA_UNKNOWN`"]
    pub const UNKNOWN: PlayAreaMode = PlayAreaMode {
        ord: 0i32
    };
    #[doc(alias = "XR_PLAY_AREA_3DOF")]
    #[doc = "Godot enumerator name: `XR_PLAY_AREA_3DOF`"]
    pub const AREA_3DOF: PlayAreaMode = PlayAreaMode {
        ord: 1i32
    };
    #[doc(alias = "XR_PLAY_AREA_SITTING")]
    #[doc = "Godot enumerator name: `XR_PLAY_AREA_SITTING`"]
    pub const SITTING: PlayAreaMode = PlayAreaMode {
        ord: 2i32
    };
    #[doc(alias = "XR_PLAY_AREA_ROOMSCALE")]
    #[doc = "Godot enumerator name: `XR_PLAY_AREA_ROOMSCALE`"]
    pub const ROOMSCALE: PlayAreaMode = PlayAreaMode {
        ord: 3i32
    };
    #[doc(alias = "XR_PLAY_AREA_STAGE")]
    #[doc = "Godot enumerator name: `XR_PLAY_AREA_STAGE`"]
    pub const STAGE: PlayAreaMode = PlayAreaMode {
        ord: 4i32
    };
    #[doc(alias = "XR_PLAY_AREA_CUSTOM")]
    #[doc = "Godot enumerator name: `XR_PLAY_AREA_CUSTOM`"]
    pub const CUSTOM: PlayAreaMode = PlayAreaMode {
        ord: 2147483647i32
    };
    
}
impl std::fmt::Debug for PlayAreaMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PlayAreaMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PlayAreaMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 2147483647i32 => Some(Self {
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
            Self::UNKNOWN => "UNKNOWN", Self::AREA_3DOF => "AREA_3DOF", Self::SITTING => "SITTING", Self::ROOMSCALE => "ROOMSCALE", Self::STAGE => "STAGE", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PlayAreaMode::UNKNOWN, PlayAreaMode::AREA_3DOF, PlayAreaMode::SITTING, PlayAreaMode::ROOMSCALE, PlayAreaMode::STAGE, PlayAreaMode::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PlayAreaMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNKNOWN", "XR_PLAY_AREA_UNKNOWN", PlayAreaMode::UNKNOWN), crate::meta::inspect::EnumConstant::new("AREA_3DOF", "XR_PLAY_AREA_3DOF", PlayAreaMode::AREA_3DOF), crate::meta::inspect::EnumConstant::new("SITTING", "XR_PLAY_AREA_SITTING", PlayAreaMode::SITTING), crate::meta::inspect::EnumConstant::new("ROOMSCALE", "XR_PLAY_AREA_ROOMSCALE", PlayAreaMode::ROOMSCALE), crate::meta::inspect::EnumConstant::new("STAGE", "XR_PLAY_AREA_STAGE", PlayAreaMode::STAGE), crate::meta::inspect::EnumConstant::new("CUSTOM", "XR_PLAY_AREA_CUSTOM", PlayAreaMode::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for PlayAreaMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PlayAreaMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PlayAreaMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EnvironmentBlendMode {
    ord: i32
}
impl EnvironmentBlendMode {
    #[doc(alias = "XR_ENV_BLEND_MODE_OPAQUE")]
    #[doc = "Godot enumerator name: `XR_ENV_BLEND_MODE_OPAQUE`"]
    pub const OPAQUE: EnvironmentBlendMode = EnvironmentBlendMode {
        ord: 0i32
    };
    #[doc(alias = "XR_ENV_BLEND_MODE_ADDITIVE")]
    #[doc = "Godot enumerator name: `XR_ENV_BLEND_MODE_ADDITIVE`"]
    pub const ADDITIVE: EnvironmentBlendMode = EnvironmentBlendMode {
        ord: 1i32
    };
    #[doc(alias = "XR_ENV_BLEND_MODE_ALPHA_BLEND")]
    #[doc = "Godot enumerator name: `XR_ENV_BLEND_MODE_ALPHA_BLEND`"]
    pub const ALPHA_BLEND: EnvironmentBlendMode = EnvironmentBlendMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for EnvironmentBlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentBlendMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentBlendMode {
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
            Self::OPAQUE => "OPAQUE", Self::ADDITIVE => "ADDITIVE", Self::ALPHA_BLEND => "ALPHA_BLEND", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentBlendMode::OPAQUE, EnvironmentBlendMode::ADDITIVE, EnvironmentBlendMode::ALPHA_BLEND]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentBlendMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OPAQUE", "XR_ENV_BLEND_MODE_OPAQUE", EnvironmentBlendMode::OPAQUE), crate::meta::inspect::EnumConstant::new("ADDITIVE", "XR_ENV_BLEND_MODE_ADDITIVE", EnvironmentBlendMode::ADDITIVE), crate::meta::inspect::EnumConstant::new("ALPHA_BLEND", "XR_ENV_BLEND_MODE_ALPHA_BLEND", EnvironmentBlendMode::ALPHA_BLEND)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentBlendMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnvironmentBlendMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentBlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `VRSTextureFormat`."]
pub struct VrsTextureFormat {
    ord: i32
}
impl VrsTextureFormat {
    #[doc(alias = "XR_VRS_TEXTURE_FORMAT_UNIFIED")]
    #[doc = "Godot enumerator name: `XR_VRS_TEXTURE_FORMAT_UNIFIED`"]
    pub const UNIFIED: VrsTextureFormat = VrsTextureFormat {
        ord: 0i32
    };
    #[doc(alias = "XR_VRS_TEXTURE_FORMAT_FRAGMENT_SHADING_RATE")]
    #[doc = "Godot enumerator name: `XR_VRS_TEXTURE_FORMAT_FRAGMENT_SHADING_RATE`"]
    pub const FRAGMENT_SHADING_RATE: VrsTextureFormat = VrsTextureFormat {
        ord: 1i32
    };
    #[doc(alias = "XR_VRS_TEXTURE_FORMAT_FRAGMENT_DENSITY_MAP")]
    #[doc = "Godot enumerator name: `XR_VRS_TEXTURE_FORMAT_FRAGMENT_DENSITY_MAP`"]
    pub const FRAGMENT_DENSITY_MAP: VrsTextureFormat = VrsTextureFormat {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for VrsTextureFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VrsTextureFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VrsTextureFormat {
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
            Self::UNIFIED => "UNIFIED", Self::FRAGMENT_SHADING_RATE => "FRAGMENT_SHADING_RATE", Self::FRAGMENT_DENSITY_MAP => "FRAGMENT_DENSITY_MAP", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VrsTextureFormat::UNIFIED, VrsTextureFormat::FRAGMENT_SHADING_RATE, VrsTextureFormat::FRAGMENT_DENSITY_MAP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VrsTextureFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNIFIED", "XR_VRS_TEXTURE_FORMAT_UNIFIED", VrsTextureFormat::UNIFIED), crate::meta::inspect::EnumConstant::new("FRAGMENT_SHADING_RATE", "XR_VRS_TEXTURE_FORMAT_FRAGMENT_SHADING_RATE", VrsTextureFormat::FRAGMENT_SHADING_RATE), crate::meta::inspect::EnumConstant::new("FRAGMENT_DENSITY_MAP", "XR_VRS_TEXTURE_FORMAT_FRAGMENT_DENSITY_MAP", VrsTextureFormat::FRAGMENT_DENSITY_MAP)]
        }
    }
}
impl crate::meta::GodotConvert for VrsTextureFormat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VrsTextureFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VrsTextureFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::XrInterface;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`XrInterface`][crate::classes::XrInterface] class."]
    pub struct SignalsOfXrInterface < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfXrInterface < 'c, C > {
        #[doc = "Signature: `(mode: i64)`"]
        pub fn play_area_changed(&mut self) -> SigPlayAreaChanged < 'c, C > {
            SigPlayAreaChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "play_area_changed")
            }
        }
    }
    type TypedSigPlayAreaChanged < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigPlayAreaChanged < 'c, C: WithSignals > {
        typed: TypedSigPlayAreaChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPlayAreaChanged < 'c, C > {
        pub fn emit(&mut self, mode: i64,) {
            self.typed.emit_tuple((mode,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPlayAreaChanged < 'c, C > {
        type Target = TypedSigPlayAreaChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPlayAreaChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for XrInterface {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfXrInterface < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfXrInterface < 'c, C > {
        type Target = < < XrInterface as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = XrInterface;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfXrInterface < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = XrInterface;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}