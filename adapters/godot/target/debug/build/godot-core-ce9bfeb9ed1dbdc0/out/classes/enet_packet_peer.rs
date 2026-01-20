#![doc = "Sidecar module for class [`ENetPacketPeer`][crate::classes::ENetPacketPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ENetPacketPeer` enums](https://docs.godotengine.org/en/stable/classes/class_enetpacketpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ENetPacketPeer.`\n\nInherits [`PacketPeer`][crate::classes::PacketPeer].\n\nRelated symbols:\n\n* [`enet_packet_peer`][crate::classes::enet_packet_peer]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `ENetPacketPeer`](https://docs.godotengine.org/en/stable/classes/class_enetpacketpeer.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<ENetPacketPeer>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ENetPacketPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl ENetPacketPeer {
        pub(crate) fn peer_disconnect_full(&mut self, data: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2915usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "peer_disconnect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::peer_disconnect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn peer_disconnect(&mut self,) {
            self.peer_disconnect_ex() . done()
        }
        #[inline]
        pub fn peer_disconnect_ex < 'a > (&'a mut self,) -> ExPeerDisconnect < 'a > {
            ExPeerDisconnect::new(self,)
        }
        pub(crate) fn peer_disconnect_later_full(&mut self, data: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2916usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "peer_disconnect_later", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::peer_disconnect_later_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn peer_disconnect_later(&mut self,) {
            self.peer_disconnect_later_ex() . done()
        }
        #[inline]
        pub fn peer_disconnect_later_ex < 'a > (&'a mut self,) -> ExPeerDisconnectLater < 'a > {
            ExPeerDisconnectLater::new(self,)
        }
        pub(crate) fn peer_disconnect_now_full(&mut self, data: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2917usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "peer_disconnect_now", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::peer_disconnect_now_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn peer_disconnect_now(&mut self,) {
            self.peer_disconnect_now_ex() . done()
        }
        #[inline]
        pub fn peer_disconnect_now_ex < 'a > (&'a mut self,) -> ExPeerDisconnectNow < 'a > {
            ExPeerDisconnectNow::new(self,)
        }
        pub fn ping(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2918usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "ping", Some(self.__validated_obj()), args,)
            }
        }
        pub fn ping_interval(&mut self, ping_interval: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (ping_interval,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2919usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "ping_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reset(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2920usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "reset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn send(&mut self, channel: i32, packet: &PackedByteArray, flags: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (i32, RefArg < 'a0, PackedByteArray >, i32,);
            let args = (channel, RefArg::new(packet), flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2921usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "send", Some(self.__validated_obj()), args,)
            }
        }
        pub fn throttle_configure(&mut self, interval: i32, acceleration: i32, deceleration: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32, i32,);
            let args = (interval, acceleration, deceleration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2922usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "throttle_configure", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_timeout(&mut self, timeout: i32, timeout_min: i32, timeout_max: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32, i32,);
            let args = (timeout, timeout_min, timeout_max,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2923usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "set_timeout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_packet_flags(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2924usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "get_packet_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_remote_address(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2925usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "get_remote_address", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_remote_port(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2926usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "get_remote_port", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_statistic(&mut self, statistic: crate::classes::enet_packet_peer::PeerStatistic,) -> f64 {
            type CallRet = f64;
            type CallParams = (crate::classes::enet_packet_peer::PeerStatistic,);
            let args = (statistic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2927usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "get_statistic", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_state(&self,) -> crate::classes::enet_packet_peer::PeerState {
            type CallRet = crate::classes::enet_packet_peer::PeerState;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2928usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "get_state", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_channels(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2929usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "get_channels", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_active(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2930usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "is_active", Some(self.__validated_obj()), args,)
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
        pub const PACKET_LOSS_SCALE: i32 = 65536i32;
        pub const PACKET_THROTTLE_SCALE: i32 = 32i32;
        pub const FLAG_RELIABLE: i32 = 1i32;
        pub const FLAG_UNSEQUENCED: i32 = 2i32;
        pub const FLAG_UNRELIABLE_FRAGMENT: i32 = 8i32;
        
    }
    impl crate::obj::GodotClass for ENetPacketPeer {
        type Base = crate::classes::PacketPeer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ENetPacketPeer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ENetPacketPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PacketPeer > for ENetPacketPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ENetPacketPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ENetPacketPeer {
        
    }
    impl std::ops::Deref for ENetPacketPeer {
        type Target = crate::classes::PacketPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ENetPacketPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ENetPacketPeer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `ENetPacketPeer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`ENetPacketPeer::peer_disconnect_ex`][super::ENetPacketPeer::peer_disconnect_ex]."]
#[must_use]
pub struct ExPeerDisconnect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ENetPacketPeer, data: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPeerDisconnect < 'a > {
    fn new(surround_object: &'a mut re_export::ENetPacketPeer,) -> Self {
        let data = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, data: data,
        }
    }
    #[inline]
    pub fn data(self, data: i32) -> Self {
        Self {
            data: data, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, data,
        }
        = self;
        re_export::ENetPacketPeer::peer_disconnect_full(surround_object, data,)
    }
}
#[doc = "Default-param extender for [`ENetPacketPeer::peer_disconnect_later_ex`][super::ENetPacketPeer::peer_disconnect_later_ex]."]
#[must_use]
pub struct ExPeerDisconnectLater < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ENetPacketPeer, data: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPeerDisconnectLater < 'a > {
    fn new(surround_object: &'a mut re_export::ENetPacketPeer,) -> Self {
        let data = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, data: data,
        }
    }
    #[inline]
    pub fn data(self, data: i32) -> Self {
        Self {
            data: data, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, data,
        }
        = self;
        re_export::ENetPacketPeer::peer_disconnect_later_full(surround_object, data,)
    }
}
#[doc = "Default-param extender for [`ENetPacketPeer::peer_disconnect_now_ex`][super::ENetPacketPeer::peer_disconnect_now_ex]."]
#[must_use]
pub struct ExPeerDisconnectNow < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ENetPacketPeer, data: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPeerDisconnectNow < 'a > {
    fn new(surround_object: &'a mut re_export::ENetPacketPeer,) -> Self {
        let data = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, data: data,
        }
    }
    #[inline]
    pub fn data(self, data: i32) -> Self {
        Self {
            data: data, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, data,
        }
        = self;
        re_export::ENetPacketPeer::peer_disconnect_now_full(surround_object, data,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PeerState {
    ord: i32
}
impl PeerState {
    #[doc(alias = "STATE_DISCONNECTED")]
    #[doc = "Godot enumerator name: `STATE_DISCONNECTED`"]
    pub const DISCONNECTED: PeerState = PeerState {
        ord: 0i32
    };
    #[doc(alias = "STATE_CONNECTING")]
    #[doc = "Godot enumerator name: `STATE_CONNECTING`"]
    pub const CONNECTING: PeerState = PeerState {
        ord: 1i32
    };
    #[doc(alias = "STATE_ACKNOWLEDGING_CONNECT")]
    #[doc = "Godot enumerator name: `STATE_ACKNOWLEDGING_CONNECT`"]
    pub const ACKNOWLEDGING_CONNECT: PeerState = PeerState {
        ord: 2i32
    };
    #[doc(alias = "STATE_CONNECTION_PENDING")]
    #[doc = "Godot enumerator name: `STATE_CONNECTION_PENDING`"]
    pub const CONNECTION_PENDING: PeerState = PeerState {
        ord: 3i32
    };
    #[doc(alias = "STATE_CONNECTION_SUCCEEDED")]
    #[doc = "Godot enumerator name: `STATE_CONNECTION_SUCCEEDED`"]
    pub const CONNECTION_SUCCEEDED: PeerState = PeerState {
        ord: 4i32
    };
    #[doc(alias = "STATE_CONNECTED")]
    #[doc = "Godot enumerator name: `STATE_CONNECTED`"]
    pub const CONNECTED: PeerState = PeerState {
        ord: 5i32
    };
    #[doc(alias = "STATE_DISCONNECT_LATER")]
    #[doc = "Godot enumerator name: `STATE_DISCONNECT_LATER`"]
    pub const DISCONNECT_LATER: PeerState = PeerState {
        ord: 6i32
    };
    #[doc(alias = "STATE_DISCONNECTING")]
    #[doc = "Godot enumerator name: `STATE_DISCONNECTING`"]
    pub const DISCONNECTING: PeerState = PeerState {
        ord: 7i32
    };
    #[doc(alias = "STATE_ACKNOWLEDGING_DISCONNECT")]
    #[doc = "Godot enumerator name: `STATE_ACKNOWLEDGING_DISCONNECT`"]
    pub const ACKNOWLEDGING_DISCONNECT: PeerState = PeerState {
        ord: 8i32
    };
    #[doc(alias = "STATE_ZOMBIE")]
    #[doc = "Godot enumerator name: `STATE_ZOMBIE`"]
    pub const ZOMBIE: PeerState = PeerState {
        ord: 9i32
    };
    
}
impl std::fmt::Debug for PeerState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PeerState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PeerState {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
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
            Self::DISCONNECTED => "DISCONNECTED", Self::CONNECTING => "CONNECTING", Self::ACKNOWLEDGING_CONNECT => "ACKNOWLEDGING_CONNECT", Self::CONNECTION_PENDING => "CONNECTION_PENDING", Self::CONNECTION_SUCCEEDED => "CONNECTION_SUCCEEDED", Self::CONNECTED => "CONNECTED", Self::DISCONNECT_LATER => "DISCONNECT_LATER", Self::DISCONNECTING => "DISCONNECTING", Self::ACKNOWLEDGING_DISCONNECT => "ACKNOWLEDGING_DISCONNECT", Self::ZOMBIE => "ZOMBIE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PeerState::DISCONNECTED, PeerState::CONNECTING, PeerState::ACKNOWLEDGING_CONNECT, PeerState::CONNECTION_PENDING, PeerState::CONNECTION_SUCCEEDED, PeerState::CONNECTED, PeerState::DISCONNECT_LATER, PeerState::DISCONNECTING, PeerState::ACKNOWLEDGING_DISCONNECT, PeerState::ZOMBIE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PeerState >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISCONNECTED", "STATE_DISCONNECTED", PeerState::DISCONNECTED), crate::meta::inspect::EnumConstant::new("CONNECTING", "STATE_CONNECTING", PeerState::CONNECTING), crate::meta::inspect::EnumConstant::new("ACKNOWLEDGING_CONNECT", "STATE_ACKNOWLEDGING_CONNECT", PeerState::ACKNOWLEDGING_CONNECT), crate::meta::inspect::EnumConstant::new("CONNECTION_PENDING", "STATE_CONNECTION_PENDING", PeerState::CONNECTION_PENDING), crate::meta::inspect::EnumConstant::new("CONNECTION_SUCCEEDED", "STATE_CONNECTION_SUCCEEDED", PeerState::CONNECTION_SUCCEEDED), crate::meta::inspect::EnumConstant::new("CONNECTED", "STATE_CONNECTED", PeerState::CONNECTED), crate::meta::inspect::EnumConstant::new("DISCONNECT_LATER", "STATE_DISCONNECT_LATER", PeerState::DISCONNECT_LATER), crate::meta::inspect::EnumConstant::new("DISCONNECTING", "STATE_DISCONNECTING", PeerState::DISCONNECTING), crate::meta::inspect::EnumConstant::new("ACKNOWLEDGING_DISCONNECT", "STATE_ACKNOWLEDGING_DISCONNECT", PeerState::ACKNOWLEDGING_DISCONNECT), crate::meta::inspect::EnumConstant::new("ZOMBIE", "STATE_ZOMBIE", PeerState::ZOMBIE)]
        }
    }
}
impl crate::meta::GodotConvert for PeerState {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PeerState {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PeerState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PeerStatistic {
    ord: i32
}
impl PeerStatistic {
    #[doc(alias = "PEER_PACKET_LOSS")]
    #[doc = "Godot enumerator name: `PEER_PACKET_LOSS`"]
    pub const PACKET_LOSS: PeerStatistic = PeerStatistic {
        ord: 0i32
    };
    #[doc(alias = "PEER_PACKET_LOSS_VARIANCE")]
    #[doc = "Godot enumerator name: `PEER_PACKET_LOSS_VARIANCE`"]
    pub const PACKET_LOSS_VARIANCE: PeerStatistic = PeerStatistic {
        ord: 1i32
    };
    #[doc(alias = "PEER_PACKET_LOSS_EPOCH")]
    #[doc = "Godot enumerator name: `PEER_PACKET_LOSS_EPOCH`"]
    pub const PACKET_LOSS_EPOCH: PeerStatistic = PeerStatistic {
        ord: 2i32
    };
    #[doc(alias = "PEER_ROUND_TRIP_TIME")]
    #[doc = "Godot enumerator name: `PEER_ROUND_TRIP_TIME`"]
    pub const ROUND_TRIP_TIME: PeerStatistic = PeerStatistic {
        ord: 3i32
    };
    #[doc(alias = "PEER_ROUND_TRIP_TIME_VARIANCE")]
    #[doc = "Godot enumerator name: `PEER_ROUND_TRIP_TIME_VARIANCE`"]
    pub const ROUND_TRIP_TIME_VARIANCE: PeerStatistic = PeerStatistic {
        ord: 4i32
    };
    #[doc(alias = "PEER_LAST_ROUND_TRIP_TIME")]
    #[doc = "Godot enumerator name: `PEER_LAST_ROUND_TRIP_TIME`"]
    pub const LAST_ROUND_TRIP_TIME: PeerStatistic = PeerStatistic {
        ord: 5i32
    };
    #[doc(alias = "PEER_LAST_ROUND_TRIP_TIME_VARIANCE")]
    #[doc = "Godot enumerator name: `PEER_LAST_ROUND_TRIP_TIME_VARIANCE`"]
    pub const LAST_ROUND_TRIP_TIME_VARIANCE: PeerStatistic = PeerStatistic {
        ord: 6i32
    };
    #[doc(alias = "PEER_PACKET_THROTTLE")]
    #[doc = "Godot enumerator name: `PEER_PACKET_THROTTLE`"]
    pub const PACKET_THROTTLE: PeerStatistic = PeerStatistic {
        ord: 7i32
    };
    #[doc(alias = "PEER_PACKET_THROTTLE_LIMIT")]
    #[doc = "Godot enumerator name: `PEER_PACKET_THROTTLE_LIMIT`"]
    pub const PACKET_THROTTLE_LIMIT: PeerStatistic = PeerStatistic {
        ord: 8i32
    };
    #[doc(alias = "PEER_PACKET_THROTTLE_COUNTER")]
    #[doc = "Godot enumerator name: `PEER_PACKET_THROTTLE_COUNTER`"]
    pub const PACKET_THROTTLE_COUNTER: PeerStatistic = PeerStatistic {
        ord: 9i32
    };
    #[doc(alias = "PEER_PACKET_THROTTLE_EPOCH")]
    #[doc = "Godot enumerator name: `PEER_PACKET_THROTTLE_EPOCH`"]
    pub const PACKET_THROTTLE_EPOCH: PeerStatistic = PeerStatistic {
        ord: 10i32
    };
    #[doc(alias = "PEER_PACKET_THROTTLE_ACCELERATION")]
    #[doc = "Godot enumerator name: `PEER_PACKET_THROTTLE_ACCELERATION`"]
    pub const PACKET_THROTTLE_ACCELERATION: PeerStatistic = PeerStatistic {
        ord: 11i32
    };
    #[doc(alias = "PEER_PACKET_THROTTLE_DECELERATION")]
    #[doc = "Godot enumerator name: `PEER_PACKET_THROTTLE_DECELERATION`"]
    pub const PACKET_THROTTLE_DECELERATION: PeerStatistic = PeerStatistic {
        ord: 12i32
    };
    #[doc(alias = "PEER_PACKET_THROTTLE_INTERVAL")]
    #[doc = "Godot enumerator name: `PEER_PACKET_THROTTLE_INTERVAL`"]
    pub const PACKET_THROTTLE_INTERVAL: PeerStatistic = PeerStatistic {
        ord: 13i32
    };
    
}
impl std::fmt::Debug for PeerStatistic {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PeerStatistic") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PeerStatistic {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
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
            Self::PACKET_LOSS => "PACKET_LOSS", Self::PACKET_LOSS_VARIANCE => "PACKET_LOSS_VARIANCE", Self::PACKET_LOSS_EPOCH => "PACKET_LOSS_EPOCH", Self::ROUND_TRIP_TIME => "ROUND_TRIP_TIME", Self::ROUND_TRIP_TIME_VARIANCE => "ROUND_TRIP_TIME_VARIANCE", Self::LAST_ROUND_TRIP_TIME => "LAST_ROUND_TRIP_TIME", Self::LAST_ROUND_TRIP_TIME_VARIANCE => "LAST_ROUND_TRIP_TIME_VARIANCE", Self::PACKET_THROTTLE => "PACKET_THROTTLE", Self::PACKET_THROTTLE_LIMIT => "PACKET_THROTTLE_LIMIT", Self::PACKET_THROTTLE_COUNTER => "PACKET_THROTTLE_COUNTER", Self::PACKET_THROTTLE_EPOCH => "PACKET_THROTTLE_EPOCH", Self::PACKET_THROTTLE_ACCELERATION => "PACKET_THROTTLE_ACCELERATION", Self::PACKET_THROTTLE_DECELERATION => "PACKET_THROTTLE_DECELERATION", Self::PACKET_THROTTLE_INTERVAL => "PACKET_THROTTLE_INTERVAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PeerStatistic::PACKET_LOSS, PeerStatistic::PACKET_LOSS_VARIANCE, PeerStatistic::PACKET_LOSS_EPOCH, PeerStatistic::ROUND_TRIP_TIME, PeerStatistic::ROUND_TRIP_TIME_VARIANCE, PeerStatistic::LAST_ROUND_TRIP_TIME, PeerStatistic::LAST_ROUND_TRIP_TIME_VARIANCE, PeerStatistic::PACKET_THROTTLE, PeerStatistic::PACKET_THROTTLE_LIMIT, PeerStatistic::PACKET_THROTTLE_COUNTER, PeerStatistic::PACKET_THROTTLE_EPOCH, PeerStatistic::PACKET_THROTTLE_ACCELERATION, PeerStatistic::PACKET_THROTTLE_DECELERATION, PeerStatistic::PACKET_THROTTLE_INTERVAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PeerStatistic >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PACKET_LOSS", "PEER_PACKET_LOSS", PeerStatistic::PACKET_LOSS), crate::meta::inspect::EnumConstant::new("PACKET_LOSS_VARIANCE", "PEER_PACKET_LOSS_VARIANCE", PeerStatistic::PACKET_LOSS_VARIANCE), crate::meta::inspect::EnumConstant::new("PACKET_LOSS_EPOCH", "PEER_PACKET_LOSS_EPOCH", PeerStatistic::PACKET_LOSS_EPOCH), crate::meta::inspect::EnumConstant::new("ROUND_TRIP_TIME", "PEER_ROUND_TRIP_TIME", PeerStatistic::ROUND_TRIP_TIME), crate::meta::inspect::EnumConstant::new("ROUND_TRIP_TIME_VARIANCE", "PEER_ROUND_TRIP_TIME_VARIANCE", PeerStatistic::ROUND_TRIP_TIME_VARIANCE), crate::meta::inspect::EnumConstant::new("LAST_ROUND_TRIP_TIME", "PEER_LAST_ROUND_TRIP_TIME", PeerStatistic::LAST_ROUND_TRIP_TIME), crate::meta::inspect::EnumConstant::new("LAST_ROUND_TRIP_TIME_VARIANCE", "PEER_LAST_ROUND_TRIP_TIME_VARIANCE", PeerStatistic::LAST_ROUND_TRIP_TIME_VARIANCE), crate::meta::inspect::EnumConstant::new("PACKET_THROTTLE", "PEER_PACKET_THROTTLE", PeerStatistic::PACKET_THROTTLE), crate::meta::inspect::EnumConstant::new("PACKET_THROTTLE_LIMIT", "PEER_PACKET_THROTTLE_LIMIT", PeerStatistic::PACKET_THROTTLE_LIMIT), crate::meta::inspect::EnumConstant::new("PACKET_THROTTLE_COUNTER", "PEER_PACKET_THROTTLE_COUNTER", PeerStatistic::PACKET_THROTTLE_COUNTER), crate::meta::inspect::EnumConstant::new("PACKET_THROTTLE_EPOCH", "PEER_PACKET_THROTTLE_EPOCH", PeerStatistic::PACKET_THROTTLE_EPOCH), crate::meta::inspect::EnumConstant::new("PACKET_THROTTLE_ACCELERATION", "PEER_PACKET_THROTTLE_ACCELERATION", PeerStatistic::PACKET_THROTTLE_ACCELERATION), crate::meta::inspect::EnumConstant::new("PACKET_THROTTLE_DECELERATION", "PEER_PACKET_THROTTLE_DECELERATION", PeerStatistic::PACKET_THROTTLE_DECELERATION), crate::meta::inspect::EnumConstant::new("PACKET_THROTTLE_INTERVAL", "PEER_PACKET_THROTTLE_INTERVAL", PeerStatistic::PACKET_THROTTLE_INTERVAL)]
        }
    }
}
impl crate::meta::GodotConvert for PeerStatistic {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PeerStatistic {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PeerStatistic {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ENetPacketPeer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for ENetPacketPeer {
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