#![doc = "Sidecar module for class [`Ip`][crate::classes::Ip].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `IP` enums](https://docs.godotengine.org/en/stable/classes/class_ip.html#enumerations).\n\n"]
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
    #[doc = "Godot class `IP.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`ip`][crate::classes::ip]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `IP`](https://docs.godotengine.org/en/stable/classes/class_ip.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Ip {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Ip {
        pub(crate) fn resolve_hostname_full(&mut self, host: CowArg < GString >, ip_type: crate::classes::ip::Type,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::ip::Type,);
            let args = (host, ip_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "resolve_hostname", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::resolve_hostname_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn resolve_hostname(&mut self, host: impl AsArg < GString >,) -> GString {
            self.resolve_hostname_ex(host,) . done()
        }
        #[inline]
        pub fn resolve_hostname_ex < 'a > (&'a mut self, host: impl AsArg < GString > + 'a,) -> ExResolveHostname < 'a > {
            ExResolveHostname::new(self, host,)
        }
        pub(crate) fn resolve_hostname_addresses_full(&mut self, host: CowArg < GString >, ip_type: crate::classes::ip::Type,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::ip::Type,);
            let args = (host, ip_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "resolve_hostname_addresses", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::resolve_hostname_addresses_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn resolve_hostname_addresses(&mut self, host: impl AsArg < GString >,) -> PackedStringArray {
            self.resolve_hostname_addresses_ex(host,) . done()
        }
        #[inline]
        pub fn resolve_hostname_addresses_ex < 'a > (&'a mut self, host: impl AsArg < GString > + 'a,) -> ExResolveHostnameAddresses < 'a > {
            ExResolveHostnameAddresses::new(self, host,)
        }
        pub(crate) fn resolve_hostname_queue_item_full(&mut self, host: CowArg < GString >, ip_type: crate::classes::ip::Type,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::ip::Type,);
            let args = (host, ip_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "resolve_hostname_queue_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::resolve_hostname_queue_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn resolve_hostname_queue_item(&mut self, host: impl AsArg < GString >,) -> i32 {
            self.resolve_hostname_queue_item_ex(host,) . done()
        }
        #[inline]
        pub fn resolve_hostname_queue_item_ex < 'a > (&'a mut self, host: impl AsArg < GString > + 'a,) -> ExResolveHostnameQueueItem < 'a > {
            ExResolveHostnameQueueItem::new(self, host,)
        }
        pub fn get_resolve_item_status(&self, id: i32,) -> crate::classes::ip::ResolverStatus {
            type CallRet = crate::classes::ip::ResolverStatus;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "get_resolve_item_status", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_resolve_item_address(&self, id: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "get_resolve_item_address", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_resolve_item_addresses(&self, id: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "get_resolve_item_addresses", Some(self.__validated_obj()), args,)
            }
        }
        pub fn erase_resolve_item(&mut self, id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "erase_resolve_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_local_addresses(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "get_local_addresses", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_local_interfaces(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "get_local_interfaces", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn clear_cache_full(&mut self, hostname: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (hostname,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "clear_cache", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::clear_cache_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn clear_cache(&mut self,) {
            self.clear_cache_ex() . done()
        }
        #[inline]
        pub fn clear_cache_ex < 'a > (&'a mut self,) -> ExClearCache < 'a > {
            ExClearCache::new(self,)
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
        pub const RESOLVER_MAX_QUERIES: i32 = 256i32;
        pub const RESOLVER_INVALID_ID: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for Ip {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("IP"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Ip {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Ip {
        
    }
    impl crate::obj::Singleton for Ip {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"IP"))
            }
        }
    }
    impl std::ops::Deref for Ip {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Ip {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Ip__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Ip` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Ip::resolve_hostname_ex`][super::Ip::resolve_hostname_ex]."]
#[must_use]
pub struct ExResolveHostname < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Ip, host: CowArg < 'a, GString >, ip_type: crate::classes::ip::Type,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExResolveHostname < 'a > {
    fn new(surround_object: &'a mut re_export::Ip, host: impl AsArg < GString > + 'a,) -> Self {
        let ip_type = crate::obj::EngineEnum::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, host: host.into_arg(), ip_type: ip_type,
        }
    }
    #[inline]
    pub fn ip_type(self, ip_type: crate::classes::ip::Type) -> Self {
        Self {
            ip_type: ip_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, host, ip_type,
        }
        = self;
        re_export::Ip::resolve_hostname_full(surround_object, host, ip_type,)
    }
}
#[doc = "Default-param extender for [`Ip::resolve_hostname_addresses_ex`][super::Ip::resolve_hostname_addresses_ex]."]
#[must_use]
pub struct ExResolveHostnameAddresses < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Ip, host: CowArg < 'a, GString >, ip_type: crate::classes::ip::Type,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExResolveHostnameAddresses < 'a > {
    fn new(surround_object: &'a mut re_export::Ip, host: impl AsArg < GString > + 'a,) -> Self {
        let ip_type = crate::obj::EngineEnum::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, host: host.into_arg(), ip_type: ip_type,
        }
    }
    #[inline]
    pub fn ip_type(self, ip_type: crate::classes::ip::Type) -> Self {
        Self {
            ip_type: ip_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, surround_object, host, ip_type,
        }
        = self;
        re_export::Ip::resolve_hostname_addresses_full(surround_object, host, ip_type,)
    }
}
#[doc = "Default-param extender for [`Ip::resolve_hostname_queue_item_ex`][super::Ip::resolve_hostname_queue_item_ex]."]
#[must_use]
pub struct ExResolveHostnameQueueItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Ip, host: CowArg < 'a, GString >, ip_type: crate::classes::ip::Type,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExResolveHostnameQueueItem < 'a > {
    fn new(surround_object: &'a mut re_export::Ip, host: impl AsArg < GString > + 'a,) -> Self {
        let ip_type = crate::obj::EngineEnum::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, host: host.into_arg(), ip_type: ip_type,
        }
    }
    #[inline]
    pub fn ip_type(self, ip_type: crate::classes::ip::Type) -> Self {
        Self {
            ip_type: ip_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, host, ip_type,
        }
        = self;
        re_export::Ip::resolve_hostname_queue_item_full(surround_object, host, ip_type,)
    }
}
#[doc = "Default-param extender for [`Ip::clear_cache_ex`][super::Ip::clear_cache_ex]."]
#[must_use]
pub struct ExClearCache < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Ip, hostname: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClearCache < 'a > {
    fn new(surround_object: &'a mut re_export::Ip,) -> Self {
        let hostname = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, hostname: CowArg::Owned(hostname),
        }
    }
    #[inline]
    pub fn hostname(self, hostname: impl AsArg < GString > + 'a) -> Self {
        Self {
            hostname: hostname.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, hostname,
        }
        = self;
        re_export::Ip::clear_cache_full(surround_object, hostname,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ResolverStatus {
    ord: i32
}
impl ResolverStatus {
    #[doc(alias = "RESOLVER_STATUS_NONE")]
    #[doc = "Godot enumerator name: `RESOLVER_STATUS_NONE`"]
    pub const NONE: ResolverStatus = ResolverStatus {
        ord: 0i32
    };
    #[doc(alias = "RESOLVER_STATUS_WAITING")]
    #[doc = "Godot enumerator name: `RESOLVER_STATUS_WAITING`"]
    pub const WAITING: ResolverStatus = ResolverStatus {
        ord: 1i32
    };
    #[doc(alias = "RESOLVER_STATUS_DONE")]
    #[doc = "Godot enumerator name: `RESOLVER_STATUS_DONE`"]
    pub const DONE: ResolverStatus = ResolverStatus {
        ord: 2i32
    };
    #[doc(alias = "RESOLVER_STATUS_ERROR")]
    #[doc = "Godot enumerator name: `RESOLVER_STATUS_ERROR`"]
    pub const ERROR: ResolverStatus = ResolverStatus {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ResolverStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ResolverStatus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ResolverStatus {
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
            Self::NONE => "NONE", Self::WAITING => "WAITING", Self::DONE => "DONE", Self::ERROR => "ERROR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ResolverStatus::NONE, ResolverStatus::WAITING, ResolverStatus::DONE, ResolverStatus::ERROR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ResolverStatus >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "RESOLVER_STATUS_NONE", ResolverStatus::NONE), crate::meta::inspect::EnumConstant::new("WAITING", "RESOLVER_STATUS_WAITING", ResolverStatus::WAITING), crate::meta::inspect::EnumConstant::new("DONE", "RESOLVER_STATUS_DONE", ResolverStatus::DONE), crate::meta::inspect::EnumConstant::new("ERROR", "RESOLVER_STATUS_ERROR", ResolverStatus::ERROR)]
        }
    }
}
impl crate::meta::GodotConvert for ResolverStatus {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ResolverStatus {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ResolverStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Type {
    ord: i32
}
impl Type {
    #[doc(alias = "TYPE_NONE")]
    #[doc = "Godot enumerator name: `TYPE_NONE`"]
    pub const NONE: Type = Type {
        ord: 0i32
    };
    #[doc(alias = "TYPE_IPV4")]
    #[doc = "Godot enumerator name: `TYPE_IPV4`"]
    pub const IPV4: Type = Type {
        ord: 1i32
    };
    #[doc(alias = "TYPE_IPV6")]
    #[doc = "Godot enumerator name: `TYPE_IPV6`"]
    pub const IPV6: Type = Type {
        ord: 2i32
    };
    #[doc(alias = "TYPE_ANY")]
    #[doc = "Godot enumerator name: `TYPE_ANY`"]
    pub const ANY: Type = Type {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Type") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Type {
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
            Self::NONE => "NONE", Self::IPV4 => "IPV4", Self::IPV6 => "IPV6", Self::ANY => "ANY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Type::NONE, Type::IPV4, Type::IPV6, Type::ANY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Type >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "TYPE_NONE", Type::NONE), crate::meta::inspect::EnumConstant::new("IPV4", "TYPE_IPV4", Type::IPV4), crate::meta::inspect::EnumConstant::new("IPV6", "TYPE_IPV6", Type::IPV6), crate::meta::inspect::EnumConstant::new("ANY", "TYPE_ANY", Type::ANY)]
        }
    }
}
impl crate::meta::GodotConvert for Type {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Type {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Type {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Ip;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Ip {
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