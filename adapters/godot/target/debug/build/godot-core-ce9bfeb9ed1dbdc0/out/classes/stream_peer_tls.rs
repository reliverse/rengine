#![doc = "Sidecar module for class [`StreamPeerTls`][crate::classes::StreamPeerTls].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `StreamPeerTLS` enums](https://docs.godotengine.org/en/stable/classes/class_streampeertls.html#enumerations).\n\n"]
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
    #[doc = "Godot class `StreamPeerTLS.`\n\nInherits [`StreamPeer`][crate::classes::StreamPeer].\n\nRelated symbols:\n\n* [`stream_peer_tls`][crate::classes::stream_peer_tls]: sidecar module with related enum/flag types\n* [`IStreamPeerTls`][crate::classes::IStreamPeerTls]: virtual methods\n\n\nSee also [Godot docs for `StreamPeerTLS`](https://docs.godotengine.org/en/stable/classes/class_streampeertls.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`StreamPeerTls::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct StreamPeerTls {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`StreamPeerTls`][crate::classes::StreamPeerTls].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IStreamPeer`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `StreamPeerTLS` methods](https://docs.godotengine.org/en/stable/classes/class_streampeertls.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IStreamPeerTls: crate::obj::GodotClass < Base = StreamPeerTls > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl StreamPeerTls {
        pub fn poll(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8590usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeerTls", "poll", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accept_stream(&mut self, stream: impl AsArg < Option < Gd < crate::classes::StreamPeer >> >, server_options: impl AsArg < Option < Gd < crate::classes::TlsOptions >> >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::StreamPeer > > >, CowArg < 'a1, Option < Gd < crate::classes::TlsOptions > > >,);
            let args = (stream.into_arg(), server_options.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8591usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeerTls", "accept_stream", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn connect_to_stream_full(&mut self, stream: CowArg < Option < Gd < crate::classes::StreamPeer > > >, common_name: CowArg < GString >, client_options: CowArg < Option < Gd < crate::classes::TlsOptions > > >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::StreamPeer > > >, CowArg < 'a1, GString >, CowArg < 'a2, Option < Gd < crate::classes::TlsOptions > > >,);
            let args = (stream, common_name, client_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8592usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeerTls", "connect_to_stream", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::connect_to_stream_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn connect_to_stream(&mut self, stream: impl AsArg < Option < Gd < crate::classes::StreamPeer >> >, common_name: impl AsArg < GString >,) -> crate::global::Error {
            self.connect_to_stream_ex(stream, common_name,) . done()
        }
        #[inline]
        pub fn connect_to_stream_ex < 'a > (&'a mut self, stream: impl AsArg < Option < Gd < crate::classes::StreamPeer >> > + 'a, common_name: impl AsArg < GString > + 'a,) -> ExConnectToStream < 'a > {
            ExConnectToStream::new(self, stream, common_name,)
        }
        pub fn get_status(&self,) -> crate::classes::stream_peer_tls::Status {
            type CallRet = crate::classes::stream_peer_tls::Status;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8593usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeerTls", "get_status", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stream(&self,) -> Option < Gd < crate::classes::StreamPeer > > {
            type CallRet = Option < Gd < crate::classes::StreamPeer > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8594usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeerTls", "get_stream", Some(self.__validated_obj()), args,)
            }
        }
        pub fn disconnect_from_stream(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8595usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeerTls", "disconnect_from_stream", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for StreamPeerTls {
        type Base = crate::classes::StreamPeer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("StreamPeerTLS"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for StreamPeerTls {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::StreamPeer > for StreamPeerTls {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for StreamPeerTls {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for StreamPeerTls {
        
    }
    impl crate::obj::cap::GodotDefault for StreamPeerTls {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for StreamPeerTls {
        type Target = crate::classes::StreamPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for StreamPeerTls {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`StreamPeerTls`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_StreamPeerTls__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::StreamPeerTls > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::StreamPeer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`StreamPeerTls::connect_to_stream_ex`][super::StreamPeerTls::connect_to_stream_ex]."]
#[must_use]
pub struct ExConnectToStream < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::StreamPeerTls, stream: CowArg < 'a, Option < Gd < crate::classes::StreamPeer > > >, common_name: CowArg < 'a, GString >, client_options: CowArg < 'a, Option < Gd < crate::classes::TlsOptions > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConnectToStream < 'a > {
    fn new(surround_object: &'a mut re_export::StreamPeerTls, stream: impl AsArg < Option < Gd < crate::classes::StreamPeer >> > + 'a, common_name: impl AsArg < GString > + 'a,) -> Self {
        let client_options = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, stream: stream.into_arg(), common_name: common_name.into_arg(), client_options: client_options.into_arg(),
        }
    }
    #[inline]
    pub fn client_options(self, client_options: impl AsArg < Option < Gd < crate::classes::TlsOptions >> > + 'a) -> Self {
        Self {
            client_options: client_options.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, stream, common_name, client_options,
        }
        = self;
        re_export::StreamPeerTls::connect_to_stream_full(surround_object, stream, common_name, client_options,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Status {
    ord: i32
}
impl Status {
    #[doc(alias = "STATUS_DISCONNECTED")]
    #[doc = "Godot enumerator name: `STATUS_DISCONNECTED`"]
    pub const DISCONNECTED: Status = Status {
        ord: 0i32
    };
    #[doc(alias = "STATUS_HANDSHAKING")]
    #[doc = "Godot enumerator name: `STATUS_HANDSHAKING`"]
    pub const HANDSHAKING: Status = Status {
        ord: 1i32
    };
    #[doc(alias = "STATUS_CONNECTED")]
    #[doc = "Godot enumerator name: `STATUS_CONNECTED`"]
    pub const CONNECTED: Status = Status {
        ord: 2i32
    };
    #[doc(alias = "STATUS_ERROR")]
    #[doc = "Godot enumerator name: `STATUS_ERROR`"]
    pub const ERROR: Status = Status {
        ord: 3i32
    };
    #[doc(alias = "STATUS_ERROR_HOSTNAME_MISMATCH")]
    #[doc = "Godot enumerator name: `STATUS_ERROR_HOSTNAME_MISMATCH`"]
    pub const ERROR_HOSTNAME_MISMATCH: Status = Status {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Status") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Status {
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
            Self::DISCONNECTED => "DISCONNECTED", Self::HANDSHAKING => "HANDSHAKING", Self::CONNECTED => "CONNECTED", Self::ERROR => "ERROR", Self::ERROR_HOSTNAME_MISMATCH => "ERROR_HOSTNAME_MISMATCH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Status::DISCONNECTED, Status::HANDSHAKING, Status::CONNECTED, Status::ERROR, Status::ERROR_HOSTNAME_MISMATCH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Status >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISCONNECTED", "STATUS_DISCONNECTED", Status::DISCONNECTED), crate::meta::inspect::EnumConstant::new("HANDSHAKING", "STATUS_HANDSHAKING", Status::HANDSHAKING), crate::meta::inspect::EnumConstant::new("CONNECTED", "STATUS_CONNECTED", Status::CONNECTED), crate::meta::inspect::EnumConstant::new("ERROR", "STATUS_ERROR", Status::ERROR), crate::meta::inspect::EnumConstant::new("ERROR_HOSTNAME_MISMATCH", "STATUS_ERROR_HOSTNAME_MISMATCH", Status::ERROR_HOSTNAME_MISMATCH)]
        }
    }
}
impl crate::meta::GodotConvert for Status {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Status {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Status {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::StreamPeerTls;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for StreamPeerTls {
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