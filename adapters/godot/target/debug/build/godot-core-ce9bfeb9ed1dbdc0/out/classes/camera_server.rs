#![doc = "Sidecar module for class [`CameraServer`][crate::classes::CameraServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CameraServer` enums](https://docs.godotengine.org/en/stable/classes/class_cameraserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CameraServer.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`camera_server`][crate::classes::camera_server]: sidecar module with related enum/flag types\n* [`SignalsOfCameraServer`][crate::classes::camera_server::SignalsOfCameraServer]: signal collection\n\n\nSee also [Godot docs for `CameraServer`](https://docs.godotengine.org/en/stable/classes/class_cameraserver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CameraServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl CameraServer {
        pub fn set_monitoring_feeds(&mut self, is_monitoring_feeds: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (is_monitoring_feeds,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(53usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraServer", "set_monitoring_feeds", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_monitoring_feeds(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(54usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraServer", "is_monitoring_feeds", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_feed(&mut self, index: i32,) -> Option < Gd < crate::classes::CameraFeed > > {
            type CallRet = Option < Gd < crate::classes::CameraFeed > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(55usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraServer", "get_feed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_feed_count(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(56usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraServer", "get_feed_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn feeds(&mut self,) -> Array < Gd < crate::classes::CameraFeed > > {
            type CallRet = Array < Gd < crate::classes::CameraFeed > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(57usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraServer", "feeds", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_feed(&mut self, feed: impl AsArg < Option < Gd < crate::classes::CameraFeed >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::CameraFeed > > >,);
            let args = (feed.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(58usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraServer", "add_feed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_feed(&mut self, feed: impl AsArg < Option < Gd < crate::classes::CameraFeed >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::CameraFeed > > >,);
            let args = (feed.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(59usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraServer", "remove_feed", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CameraServer {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CameraServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for CameraServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CameraServer {
        
    }
    impl crate::obj::Singleton for CameraServer {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"CameraServer"))
            }
        }
    }
    impl std::ops::Deref for CameraServer {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CameraServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CameraServer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `CameraServer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FeedImage {
    ord: i32
}
impl FeedImage {
    #[doc(alias = "FEED_RGBA_IMAGE")]
    #[doc = "Godot enumerator name: `FEED_RGBA_IMAGE`"]
    pub const RGBA_IMAGE: FeedImage = FeedImage {
        ord: 0i32
    };
    #[doc(alias = "FEED_YCBCR_IMAGE")]
    #[doc = "Godot enumerator name: `FEED_YCBCR_IMAGE`"]
    pub const YCBCR_IMAGE: FeedImage = FeedImage {
        ord: 0i32
    };
    #[doc(alias = "FEED_Y_IMAGE")]
    #[doc = "Godot enumerator name: `FEED_Y_IMAGE`"]
    pub const Y_IMAGE: FeedImage = FeedImage {
        ord: 0i32
    };
    #[doc(alias = "FEED_CBCR_IMAGE")]
    #[doc = "Godot enumerator name: `FEED_CBCR_IMAGE`"]
    pub const CBCR_IMAGE: FeedImage = FeedImage {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for FeedImage {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FeedImage") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FeedImage {
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
            Self::RGBA_IMAGE => "RGBA_IMAGE", Self::YCBCR_IMAGE => "YCBCR_IMAGE", Self::Y_IMAGE => "Y_IMAGE", Self::CBCR_IMAGE => "CBCR_IMAGE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FeedImage::RGBA_IMAGE, FeedImage::CBCR_IMAGE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FeedImage >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RGBA_IMAGE", "FEED_RGBA_IMAGE", FeedImage::RGBA_IMAGE), crate::meta::inspect::EnumConstant::new("YCBCR_IMAGE", "FEED_YCBCR_IMAGE", FeedImage::YCBCR_IMAGE), crate::meta::inspect::EnumConstant::new("Y_IMAGE", "FEED_Y_IMAGE", FeedImage::Y_IMAGE), crate::meta::inspect::EnumConstant::new("CBCR_IMAGE", "FEED_CBCR_IMAGE", FeedImage::CBCR_IMAGE)]
        }
    }
}
impl crate::meta::GodotConvert for FeedImage {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FeedImage {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FeedImage {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CameraServer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`CameraServer`][crate::classes::CameraServer] class."]
    pub struct SignalsOfCameraServer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfCameraServer < 'c, C > {
        #[doc = "Signature: `(id: i64)`"]
        pub fn camera_feed_added(&mut self) -> SigCameraFeedAdded < 'c, C > {
            SigCameraFeedAdded {
                typed: TypedSignal::extract(&mut self.__internal_obj, "camera_feed_added")
            }
        }
        #[doc = "Signature: `(id: i64)`"]
        pub fn camera_feed_removed(&mut self) -> SigCameraFeedRemoved < 'c, C > {
            SigCameraFeedRemoved {
                typed: TypedSignal::extract(&mut self.__internal_obj, "camera_feed_removed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn camera_feeds_updated(&mut self) -> SigCameraFeedsUpdated < 'c, C > {
            SigCameraFeedsUpdated {
                typed: TypedSignal::extract(&mut self.__internal_obj, "camera_feeds_updated")
            }
        }
    }
    type TypedSigCameraFeedAdded < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigCameraFeedAdded < 'c, C: WithSignals > {
        typed: TypedSigCameraFeedAdded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCameraFeedAdded < 'c, C > {
        pub fn emit(&mut self, id: i64,) {
            self.typed.emit_tuple((id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCameraFeedAdded < 'c, C > {
        type Target = TypedSigCameraFeedAdded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCameraFeedAdded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCameraFeedRemoved < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigCameraFeedRemoved < 'c, C: WithSignals > {
        typed: TypedSigCameraFeedRemoved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCameraFeedRemoved < 'c, C > {
        pub fn emit(&mut self, id: i64,) {
            self.typed.emit_tuple((id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCameraFeedRemoved < 'c, C > {
        type Target = TypedSigCameraFeedRemoved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCameraFeedRemoved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCameraFeedsUpdated < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigCameraFeedsUpdated < 'c, C: WithSignals > {
        typed: TypedSigCameraFeedsUpdated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCameraFeedsUpdated < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCameraFeedsUpdated < 'c, C > {
        type Target = TypedSigCameraFeedsUpdated < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCameraFeedsUpdated < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for CameraServer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCameraServer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfCameraServer < 'c, C > {
        type Target = < < CameraServer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = CameraServer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfCameraServer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = CameraServer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}