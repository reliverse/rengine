#![doc = "Sidecar module for class [`TlsOptions`][crate::classes::TlsOptions].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TLSOptions` enums](https://docs.godotengine.org/en/stable/classes/class_tlsoptions.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TLSOptions.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`tls_options`][crate::classes::tls_options]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `TLSOptions`](https://docs.godotengine.org/en/stable/classes/class_tlsoptions.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<TlsOptions>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TlsOptions {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl TlsOptions {
        pub(crate) fn client_full(trusted_chain: CowArg < Option < Gd < crate::classes::X509Certificate > > >, common_name_override: CowArg < GString >,) -> Option < Gd < crate::classes::TlsOptions > > {
            type CallRet = Option < Gd < crate::classes::TlsOptions > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::X509Certificate > > >, CowArg < 'a1, GString >,);
            let args = (trusted_chain, common_name_override,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8757usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "client", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::client_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn client() -> Option < Gd < crate::classes::TlsOptions > > {
            Self::client_ex() . done()
        }
        #[inline]
        pub fn client_ex < 'a > () -> ExClient < 'a > {
            ExClient::new()
        }
        pub(crate) fn client_unsafe_full(trusted_chain: CowArg < Option < Gd < crate::classes::X509Certificate > > >,) -> Option < Gd < crate::classes::TlsOptions > > {
            type CallRet = Option < Gd < crate::classes::TlsOptions > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::X509Certificate > > >,);
            let args = (trusted_chain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8758usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "client_unsafe", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::client_unsafe_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn client_unsafe() -> Option < Gd < crate::classes::TlsOptions > > {
            Self::client_unsafe_ex() . done()
        }
        #[inline]
        pub fn client_unsafe_ex < 'a > () -> ExClientUnsafe < 'a > {
            ExClientUnsafe::new()
        }
        pub fn server(key: impl AsArg < Option < Gd < crate::classes::CryptoKey >> >, certificate: impl AsArg < Option < Gd < crate::classes::X509Certificate >> >,) -> Option < Gd < crate::classes::TlsOptions > > {
            type CallRet = Option < Gd < crate::classes::TlsOptions > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::CryptoKey > > >, CowArg < 'a1, Option < Gd < crate::classes::X509Certificate > > >,);
            let args = (key.into_arg(), certificate.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8759usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "server", None, args,)
            }
        }
        pub fn is_server(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8760usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "is_server", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_unsafe_client(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8761usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "is_unsafe_client", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_common_name_override(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8762usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "get_common_name_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_trusted_ca_chain(&self,) -> Option < Gd < crate::classes::X509Certificate > > {
            type CallRet = Option < Gd < crate::classes::X509Certificate > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8763usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "get_trusted_ca_chain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_private_key(&self,) -> Option < Gd < crate::classes::CryptoKey > > {
            type CallRet = Option < Gd < crate::classes::CryptoKey > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8764usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "get_private_key", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_own_certificate(&self,) -> Option < Gd < crate::classes::X509Certificate > > {
            type CallRet = Option < Gd < crate::classes::X509Certificate > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8765usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "get_own_certificate", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TlsOptions {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TLSOptions"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TlsOptions {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TlsOptions {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TlsOptions {
        
    }
    impl std::ops::Deref for TlsOptions {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TlsOptions {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TlsOptions__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `TlsOptions` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`TlsOptions::client_ex`][super::TlsOptions::client_ex]."]
#[must_use]
pub struct ExClient < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, trusted_chain: CowArg < 'a, Option < Gd < crate::classes::X509Certificate > > >, common_name_override: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClient < 'a > {
    fn new() -> Self {
        let trusted_chain = Gd::null_arg();
        let common_name_override = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, trusted_chain: trusted_chain.into_arg(), common_name_override: CowArg::Owned(common_name_override),
        }
    }
    #[inline]
    pub fn trusted_chain(self, trusted_chain: impl AsArg < Option < Gd < crate::classes::X509Certificate >> > + 'a) -> Self {
        Self {
            trusted_chain: trusted_chain.into_arg(), .. self
        }
    }
    #[inline]
    pub fn common_name_override(self, common_name_override: impl AsArg < GString > + 'a) -> Self {
        Self {
            common_name_override: common_name_override.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TlsOptions > > {
        let Self {
            _phantom, trusted_chain, common_name_override,
        }
        = self;
        re_export::TlsOptions::client_full(trusted_chain, common_name_override,)
    }
}
#[doc = "Default-param extender for [`TlsOptions::client_unsafe_ex`][super::TlsOptions::client_unsafe_ex]."]
#[must_use]
pub struct ExClientUnsafe < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, trusted_chain: CowArg < 'a, Option < Gd < crate::classes::X509Certificate > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClientUnsafe < 'a > {
    fn new() -> Self {
        let trusted_chain = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, trusted_chain: trusted_chain.into_arg(),
        }
    }
    #[inline]
    pub fn trusted_chain(self, trusted_chain: impl AsArg < Option < Gd < crate::classes::X509Certificate >> > + 'a) -> Self {
        Self {
            trusted_chain: trusted_chain.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TlsOptions > > {
        let Self {
            _phantom, trusted_chain,
        }
        = self;
        re_export::TlsOptions::client_unsafe_full(trusted_chain,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TlsOptions;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for TlsOptions {
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