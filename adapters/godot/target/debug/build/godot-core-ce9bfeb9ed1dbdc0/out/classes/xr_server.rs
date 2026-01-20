#![doc = "Sidecar module for class [`XrServer`][crate::classes::XrServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `XRServer` enums](https://docs.godotengine.org/en/stable/classes/class_xrserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `XRServer.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`xr_server`][crate::classes::xr_server]: sidecar module with related enum/flag types\n* [`SignalsOfXrServer`][crate::classes::xr_server::SignalsOfXrServer]: signal collection\n\n\nSee also [Godot docs for `XRServer`](https://docs.godotengine.org/en/stable/classes/class_xrserver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct XrServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl XrServer {
        pub fn get_world_scale(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1301usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "get_world_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_world_scale(&mut self, scale: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1302usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "set_world_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_world_origin(&self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1303usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "get_world_origin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_world_origin(&mut self, world_origin: Transform3D,) {
            type CallRet = ();
            type CallParams = (Transform3D,);
            let args = (world_origin,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1304usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "set_world_origin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_reference_frame(&self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1305usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "get_reference_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_reference_frame(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "clear_reference_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn center_on_hmd(&mut self, rotation_mode: crate::classes::xr_server::RotationMode, keep_height: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::xr_server::RotationMode, bool,);
            let args = (rotation_mode, keep_height,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "center_on_hmd", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hmd_transform(&mut self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1308usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "get_hmd_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_camera_locked_to_origin(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1309usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "set_camera_locked_to_origin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_camera_locked_to_origin(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1310usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "is_camera_locked_to_origin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_interface(&mut self, interface: impl AsArg < Option < Gd < crate::classes::XrInterface >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::XrInterface > > >,);
            let args = (interface.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1311usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "add_interface", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_interface_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1312usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "get_interface_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_interface(&mut self, interface: impl AsArg < Option < Gd < crate::classes::XrInterface >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::XrInterface > > >,);
            let args = (interface.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1313usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "remove_interface", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_interface(&self, idx: i32,) -> Option < Gd < crate::classes::XrInterface > > {
            type CallRet = Option < Gd < crate::classes::XrInterface > >;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1314usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "get_interface", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_interfaces(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1315usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "get_interfaces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn find_interface(&self, name: impl AsArg < GString >,) -> Option < Gd < crate::classes::XrInterface > > {
            type CallRet = Option < Gd < crate::classes::XrInterface > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1316usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "find_interface", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_tracker(&mut self, tracker: impl AsArg < Option < Gd < crate::classes::XrTracker >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::XrTracker > > >,);
            let args = (tracker.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1317usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "add_tracker", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_tracker(&mut self, tracker: impl AsArg < Option < Gd < crate::classes::XrTracker >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::XrTracker > > >,);
            let args = (tracker.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1318usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "remove_tracker", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_trackers(&mut self, tracker_types: i32,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i32,);
            let args = (tracker_types,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1319usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "get_trackers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tracker(&self, tracker_name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::XrTracker > > {
            type CallRet = Option < Gd < crate::classes::XrTracker > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (tracker_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1320usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "get_tracker", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_primary_interface(&self,) -> Option < Gd < crate::classes::XrInterface > > {
            type CallRet = Option < Gd < crate::classes::XrInterface > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1321usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "get_primary_interface", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_primary_interface(&mut self, interface: impl AsArg < Option < Gd < crate::classes::XrInterface >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::XrInterface > > >,);
            let args = (interface.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XrServer", "set_primary_interface", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for XrServer {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("XRServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for XrServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for XrServer {
        
    }
    impl crate::obj::Singleton for XrServer {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"XRServer"))
            }
        }
    }
    impl std::ops::Deref for XrServer {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for XrServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_XrServer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `XrServer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TrackerType {
    ord: i32
}
impl TrackerType {
    #[doc(alias = "TRACKER_HEAD")]
    #[doc = "Godot enumerator name: `TRACKER_HEAD`"]
    pub const HEAD: TrackerType = TrackerType {
        ord: 1i32
    };
    #[doc(alias = "TRACKER_CONTROLLER")]
    #[doc = "Godot enumerator name: `TRACKER_CONTROLLER`"]
    pub const CONTROLLER: TrackerType = TrackerType {
        ord: 2i32
    };
    #[doc(alias = "TRACKER_BASESTATION")]
    #[doc = "Godot enumerator name: `TRACKER_BASESTATION`"]
    pub const BASESTATION: TrackerType = TrackerType {
        ord: 4i32
    };
    #[doc(alias = "TRACKER_ANCHOR")]
    #[doc = "Godot enumerator name: `TRACKER_ANCHOR`"]
    pub const ANCHOR: TrackerType = TrackerType {
        ord: 8i32
    };
    #[doc(alias = "TRACKER_HAND")]
    #[doc = "Godot enumerator name: `TRACKER_HAND`"]
    pub const HAND: TrackerType = TrackerType {
        ord: 16i32
    };
    #[doc(alias = "TRACKER_BODY")]
    #[doc = "Godot enumerator name: `TRACKER_BODY`"]
    pub const BODY: TrackerType = TrackerType {
        ord: 32i32
    };
    #[doc(alias = "TRACKER_FACE")]
    #[doc = "Godot enumerator name: `TRACKER_FACE`"]
    pub const FACE: TrackerType = TrackerType {
        ord: 64i32
    };
    #[doc(alias = "TRACKER_ANY_KNOWN")]
    #[doc = "Godot enumerator name: `TRACKER_ANY_KNOWN`"]
    pub const ANY_KNOWN: TrackerType = TrackerType {
        ord: 127i32
    };
    #[doc(alias = "TRACKER_UNKNOWN")]
    #[doc = "Godot enumerator name: `TRACKER_UNKNOWN`"]
    pub const UNKNOWN: TrackerType = TrackerType {
        ord: 128i32
    };
    #[doc(alias = "TRACKER_ANY")]
    #[doc = "Godot enumerator name: `TRACKER_ANY`"]
    pub const ANY: TrackerType = TrackerType {
        ord: 255i32
    };
    
}
impl std::fmt::Debug for TrackerType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TrackerType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TrackerType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 | ord @ 32i32 | ord @ 64i32 | ord @ 127i32 | ord @ 128i32 | ord @ 255i32 => Some(Self {
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
            Self::HEAD => "HEAD", Self::CONTROLLER => "CONTROLLER", Self::BASESTATION => "BASESTATION", Self::ANCHOR => "ANCHOR", Self::HAND => "HAND", Self::BODY => "BODY", Self::FACE => "FACE", Self::ANY_KNOWN => "ANY_KNOWN", Self::UNKNOWN => "UNKNOWN", Self::ANY => "ANY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TrackerType::HEAD, TrackerType::CONTROLLER, TrackerType::BASESTATION, TrackerType::ANCHOR, TrackerType::HAND, TrackerType::BODY, TrackerType::FACE, TrackerType::ANY_KNOWN, TrackerType::UNKNOWN, TrackerType::ANY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TrackerType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("HEAD", "TRACKER_HEAD", TrackerType::HEAD), crate::meta::inspect::EnumConstant::new("CONTROLLER", "TRACKER_CONTROLLER", TrackerType::CONTROLLER), crate::meta::inspect::EnumConstant::new("BASESTATION", "TRACKER_BASESTATION", TrackerType::BASESTATION), crate::meta::inspect::EnumConstant::new("ANCHOR", "TRACKER_ANCHOR", TrackerType::ANCHOR), crate::meta::inspect::EnumConstant::new("HAND", "TRACKER_HAND", TrackerType::HAND), crate::meta::inspect::EnumConstant::new("BODY", "TRACKER_BODY", TrackerType::BODY), crate::meta::inspect::EnumConstant::new("FACE", "TRACKER_FACE", TrackerType::FACE), crate::meta::inspect::EnumConstant::new("ANY_KNOWN", "TRACKER_ANY_KNOWN", TrackerType::ANY_KNOWN), crate::meta::inspect::EnumConstant::new("UNKNOWN", "TRACKER_UNKNOWN", TrackerType::UNKNOWN), crate::meta::inspect::EnumConstant::new("ANY", "TRACKER_ANY", TrackerType::ANY)]
        }
    }
}
impl crate::meta::GodotConvert for TrackerType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TrackerType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TrackerType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RotationMode {
    ord: i32
}
impl RotationMode {
    pub const RESET_FULL_ROTATION: RotationMode = RotationMode {
        ord: 0i32
    };
    pub const RESET_BUT_KEEP_TILT: RotationMode = RotationMode {
        ord: 1i32
    };
    pub const DONT_RESET_ROTATION: RotationMode = RotationMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for RotationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RotationMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RotationMode {
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
            Self::RESET_FULL_ROTATION => "RESET_FULL_ROTATION", Self::RESET_BUT_KEEP_TILT => "RESET_BUT_KEEP_TILT", Self::DONT_RESET_ROTATION => "DONT_RESET_ROTATION", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[RotationMode::RESET_FULL_ROTATION, RotationMode::RESET_BUT_KEEP_TILT, RotationMode::DONT_RESET_ROTATION]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < RotationMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RESET_FULL_ROTATION", "RESET_FULL_ROTATION", RotationMode::RESET_FULL_ROTATION), crate::meta::inspect::EnumConstant::new("RESET_BUT_KEEP_TILT", "RESET_BUT_KEEP_TILT", RotationMode::RESET_BUT_KEEP_TILT), crate::meta::inspect::EnumConstant::new("DONT_RESET_ROTATION", "DONT_RESET_ROTATION", RotationMode::DONT_RESET_ROTATION)]
        }
    }
}
impl crate::meta::GodotConvert for RotationMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RotationMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RotationMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::XrServer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`XrServer`][crate::classes::XrServer] class."]
    pub struct SignalsOfXrServer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfXrServer < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn reference_frame_changed(&mut self) -> SigReferenceFrameChanged < 'c, C > {
            SigReferenceFrameChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "reference_frame_changed")
            }
        }
        #[doc = "Signature: `(interface_name: StringName)`"]
        pub fn interface_added(&mut self) -> SigInterfaceAdded < 'c, C > {
            SigInterfaceAdded {
                typed: TypedSignal::extract(&mut self.__internal_obj, "interface_added")
            }
        }
        #[doc = "Signature: `(interface_name: StringName)`"]
        pub fn interface_removed(&mut self) -> SigInterfaceRemoved < 'c, C > {
            SigInterfaceRemoved {
                typed: TypedSignal::extract(&mut self.__internal_obj, "interface_removed")
            }
        }
        #[doc = "Signature: `(tracker_name: StringName, type_: i64)`"]
        pub fn tracker_added(&mut self) -> SigTrackerAdded < 'c, C > {
            SigTrackerAdded {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tracker_added")
            }
        }
        #[doc = "Signature: `(tracker_name: StringName, type_: i64)`"]
        pub fn tracker_updated(&mut self) -> SigTrackerUpdated < 'c, C > {
            SigTrackerUpdated {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tracker_updated")
            }
        }
        #[doc = "Signature: `(tracker_name: StringName, type_: i64)`"]
        pub fn tracker_removed(&mut self) -> SigTrackerRemoved < 'c, C > {
            SigTrackerRemoved {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tracker_removed")
            }
        }
    }
    type TypedSigReferenceFrameChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigReferenceFrameChanged < 'c, C: WithSignals > {
        typed: TypedSigReferenceFrameChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigReferenceFrameChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigReferenceFrameChanged < 'c, C > {
        type Target = TypedSigReferenceFrameChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigReferenceFrameChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigInterfaceAdded < 'c, C > = TypedSignal < 'c, C, (StringName,) >;
    pub struct SigInterfaceAdded < 'c, C: WithSignals > {
        typed: TypedSigInterfaceAdded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigInterfaceAdded < 'c, C > {
        pub fn emit(&mut self, interface_name: StringName,) {
            self.typed.emit_tuple((interface_name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigInterfaceAdded < 'c, C > {
        type Target = TypedSigInterfaceAdded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigInterfaceAdded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigInterfaceRemoved < 'c, C > = TypedSignal < 'c, C, (StringName,) >;
    pub struct SigInterfaceRemoved < 'c, C: WithSignals > {
        typed: TypedSigInterfaceRemoved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigInterfaceRemoved < 'c, C > {
        pub fn emit(&mut self, interface_name: StringName,) {
            self.typed.emit_tuple((interface_name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigInterfaceRemoved < 'c, C > {
        type Target = TypedSigInterfaceRemoved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigInterfaceRemoved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTrackerAdded < 'c, C > = TypedSignal < 'c, C, (StringName, i64,) >;
    pub struct SigTrackerAdded < 'c, C: WithSignals > {
        typed: TypedSigTrackerAdded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTrackerAdded < 'c, C > {
        pub fn emit(&mut self, tracker_name: StringName, type_: i64,) {
            self.typed.emit_tuple((tracker_name, type_,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTrackerAdded < 'c, C > {
        type Target = TypedSigTrackerAdded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTrackerAdded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTrackerUpdated < 'c, C > = TypedSignal < 'c, C, (StringName, i64,) >;
    pub struct SigTrackerUpdated < 'c, C: WithSignals > {
        typed: TypedSigTrackerUpdated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTrackerUpdated < 'c, C > {
        pub fn emit(&mut self, tracker_name: StringName, type_: i64,) {
            self.typed.emit_tuple((tracker_name, type_,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTrackerUpdated < 'c, C > {
        type Target = TypedSigTrackerUpdated < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTrackerUpdated < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTrackerRemoved < 'c, C > = TypedSignal < 'c, C, (StringName, i64,) >;
    pub struct SigTrackerRemoved < 'c, C: WithSignals > {
        typed: TypedSigTrackerRemoved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTrackerRemoved < 'c, C > {
        pub fn emit(&mut self, tracker_name: StringName, type_: i64,) {
            self.typed.emit_tuple((tracker_name, type_,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTrackerRemoved < 'c, C > {
        type Target = TypedSigTrackerRemoved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTrackerRemoved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for XrServer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfXrServer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfXrServer < 'c, C > {
        type Target = < < XrServer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = XrServer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfXrServer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = XrServer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}