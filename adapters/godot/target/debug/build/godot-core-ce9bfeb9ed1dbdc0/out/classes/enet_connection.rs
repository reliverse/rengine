#![doc = "Sidecar module for class [`ENetConnection`][crate::classes::ENetConnection].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ENetConnection` enums](https://docs.godotengine.org/en/stable/classes/class_enetconnection.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ENetConnection.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`enet_connection`][crate::classes::enet_connection]: sidecar module with related enum/flag types\n* [`IENetConnection`][crate::classes::IENetConnection]: virtual methods\n\n\nSee also [Godot docs for `ENetConnection`](https://docs.godotengine.org/en/stable/classes/class_enetconnection.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ENetConnection::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ENetConnection {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ENetConnection`][crate::classes::ENetConnection].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `ENetConnection` methods](https://docs.godotengine.org/en/stable/classes/class_enetconnection.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IENetConnection: crate::obj::GodotClass < Base = ENetConnection > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ENetConnection {
        pub(crate) fn create_host_bound_full(&mut self, bind_address: CowArg < GString >, bind_port: i32, max_peers: i32, max_channels: i32, in_bandwidth: i32, out_bandwidth: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, i32, i32, i32, i32,);
            let args = (bind_address, bind_port, max_peers, max_channels, in_bandwidth, out_bandwidth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2890usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "create_host_bound", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_host_bound_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_host_bound(&mut self, bind_address: impl AsArg < GString >, bind_port: i32,) -> crate::global::Error {
            self.create_host_bound_ex(bind_address, bind_port,) . done()
        }
        #[inline]
        pub fn create_host_bound_ex < 'a > (&'a mut self, bind_address: impl AsArg < GString > + 'a, bind_port: i32,) -> ExCreateHostBound < 'a > {
            ExCreateHostBound::new(self, bind_address, bind_port,)
        }
        pub(crate) fn create_host_full(&mut self, max_peers: i32, max_channels: i32, in_bandwidth: i32, out_bandwidth: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (i32, i32, i32, i32,);
            let args = (max_peers, max_channels, in_bandwidth, out_bandwidth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2891usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "create_host", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_host_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_host(&mut self,) -> crate::global::Error {
            self.create_host_ex() . done()
        }
        #[inline]
        pub fn create_host_ex < 'a > (&'a mut self,) -> ExCreateHost < 'a > {
            ExCreateHost::new(self,)
        }
        pub fn destroy(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2892usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "destroy", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn connect_to_host_full(&mut self, address: CowArg < GString >, port: i32, channels: i32, data: i32,) -> Option < Gd < crate::classes::ENetPacketPeer > > {
            type CallRet = Option < Gd < crate::classes::ENetPacketPeer > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, i32, i32,);
            let args = (address, port, channels, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2893usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "connect_to_host", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::connect_to_host_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn connect_to_host(&mut self, address: impl AsArg < GString >, port: i32,) -> Option < Gd < crate::classes::ENetPacketPeer > > {
            self.connect_to_host_ex(address, port,) . done()
        }
        #[inline]
        pub fn connect_to_host_ex < 'a > (&'a mut self, address: impl AsArg < GString > + 'a, port: i32,) -> ExConnectToHost < 'a > {
            ExConnectToHost::new(self, address, port,)
        }
        pub(crate) fn service_full(&mut self, timeout: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32,);
            let args = (timeout,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2894usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "service", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::service_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn service(&mut self,) -> VarArray {
            self.service_ex() . done()
        }
        #[inline]
        pub fn service_ex < 'a > (&'a mut self,) -> ExService < 'a > {
            ExService::new(self,)
        }
        pub fn flush(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2895usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "flush", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn bandwidth_limit_full(&mut self, in_bandwidth: i32, out_bandwidth: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (in_bandwidth, out_bandwidth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2896usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "bandwidth_limit", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::bandwidth_limit_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn bandwidth_limit(&mut self,) {
            self.bandwidth_limit_ex() . done()
        }
        #[inline]
        pub fn bandwidth_limit_ex < 'a > (&'a mut self,) -> ExBandwidthLimit < 'a > {
            ExBandwidthLimit::new(self,)
        }
        pub fn channel_limit(&mut self, limit: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (limit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2897usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "channel_limit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn broadcast(&mut self, channel: i32, packet: &PackedByteArray, flags: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, PackedByteArray >, i32,);
            let args = (channel, RefArg::new(packet), flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2898usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "broadcast", Some(self.__validated_obj()), args,)
            }
        }
        pub fn compress(&mut self, mode: crate::classes::enet_connection::CompressionMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::enet_connection::CompressionMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2899usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "compress", Some(self.__validated_obj()), args,)
            }
        }
        pub fn dtls_server_setup(&mut self, server_options: impl AsArg < Option < Gd < crate::classes::TlsOptions >> >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TlsOptions > > >,);
            let args = (server_options.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2900usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "dtls_server_setup", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn dtls_client_setup_full(&mut self, hostname: CowArg < GString >, client_options: CowArg < Option < Gd < crate::classes::TlsOptions > > >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::TlsOptions > > >,);
            let args = (hostname, client_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2901usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "dtls_client_setup", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::dtls_client_setup_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn dtls_client_setup(&mut self, hostname: impl AsArg < GString >,) -> crate::global::Error {
            self.dtls_client_setup_ex(hostname,) . done()
        }
        #[inline]
        pub fn dtls_client_setup_ex < 'a > (&'a mut self, hostname: impl AsArg < GString > + 'a,) -> ExDtlsClientSetup < 'a > {
            ExDtlsClientSetup::new(self, hostname,)
        }
        pub fn refuse_new_connections(&mut self, refuse: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (refuse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2902usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "refuse_new_connections", Some(self.__validated_obj()), args,)
            }
        }
        pub fn pop_statistic(&mut self, statistic: crate::classes::enet_connection::HostStatistic,) -> f64 {
            type CallRet = f64;
            type CallParams = (crate::classes::enet_connection::HostStatistic,);
            let args = (statistic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2903usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "pop_statistic", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_channels(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2904usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "get_max_channels", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_local_port(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2905usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "get_local_port", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_peers(&mut self,) -> Array < Gd < crate::classes::ENetPacketPeer > > {
            type CallRet = Array < Gd < crate::classes::ENetPacketPeer > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2906usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "get_peers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn socket_send(&mut self, destination_address: impl AsArg < GString >, destination_port: i32, packet: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, RefArg < 'a1, PackedByteArray >,);
            let args = (destination_address.into_arg(), destination_port, RefArg::new(packet),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2907usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetConnection", "socket_send", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ENetConnection {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ENetConnection"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ENetConnection {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ENetConnection {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ENetConnection {
        
    }
    impl crate::obj::cap::GodotDefault for ENetConnection {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ENetConnection {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ENetConnection {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ENetConnection`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ENetConnection__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ENetConnection > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ENetConnection::create_host_bound_ex`][super::ENetConnection::create_host_bound_ex]."]
#[must_use]
pub struct ExCreateHostBound < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ENetConnection, bind_address: CowArg < 'a, GString >, bind_port: i32, max_peers: i32, max_channels: i32, in_bandwidth: i32, out_bandwidth: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateHostBound < 'a > {
    fn new(surround_object: &'a mut re_export::ENetConnection, bind_address: impl AsArg < GString > + 'a, bind_port: i32,) -> Self {
        let max_peers = 32i32;
        let max_channels = 0i32;
        let in_bandwidth = 0i32;
        let out_bandwidth = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bind_address: bind_address.into_arg(), bind_port: bind_port, max_peers: max_peers, max_channels: max_channels, in_bandwidth: in_bandwidth, out_bandwidth: out_bandwidth,
        }
    }
    #[inline]
    pub fn max_peers(self, max_peers: i32) -> Self {
        Self {
            max_peers: max_peers, .. self
        }
    }
    #[inline]
    pub fn max_channels(self, max_channels: i32) -> Self {
        Self {
            max_channels: max_channels, .. self
        }
    }
    #[inline]
    pub fn in_bandwidth(self, in_bandwidth: i32) -> Self {
        Self {
            in_bandwidth: in_bandwidth, .. self
        }
    }
    #[inline]
    pub fn out_bandwidth(self, out_bandwidth: i32) -> Self {
        Self {
            out_bandwidth: out_bandwidth, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, bind_address, bind_port, max_peers, max_channels, in_bandwidth, out_bandwidth,
        }
        = self;
        re_export::ENetConnection::create_host_bound_full(surround_object, bind_address, bind_port, max_peers, max_channels, in_bandwidth, out_bandwidth,)
    }
}
#[doc = "Default-param extender for [`ENetConnection::create_host_ex`][super::ENetConnection::create_host_ex]."]
#[must_use]
pub struct ExCreateHost < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ENetConnection, max_peers: i32, max_channels: i32, in_bandwidth: i32, out_bandwidth: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateHost < 'a > {
    fn new(surround_object: &'a mut re_export::ENetConnection,) -> Self {
        let max_peers = 32i32;
        let max_channels = 0i32;
        let in_bandwidth = 0i32;
        let out_bandwidth = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, max_peers: max_peers, max_channels: max_channels, in_bandwidth: in_bandwidth, out_bandwidth: out_bandwidth,
        }
    }
    #[inline]
    pub fn max_peers(self, max_peers: i32) -> Self {
        Self {
            max_peers: max_peers, .. self
        }
    }
    #[inline]
    pub fn max_channels(self, max_channels: i32) -> Self {
        Self {
            max_channels: max_channels, .. self
        }
    }
    #[inline]
    pub fn in_bandwidth(self, in_bandwidth: i32) -> Self {
        Self {
            in_bandwidth: in_bandwidth, .. self
        }
    }
    #[inline]
    pub fn out_bandwidth(self, out_bandwidth: i32) -> Self {
        Self {
            out_bandwidth: out_bandwidth, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, max_peers, max_channels, in_bandwidth, out_bandwidth,
        }
        = self;
        re_export::ENetConnection::create_host_full(surround_object, max_peers, max_channels, in_bandwidth, out_bandwidth,)
    }
}
#[doc = "Default-param extender for [`ENetConnection::connect_to_host_ex`][super::ENetConnection::connect_to_host_ex]."]
#[must_use]
pub struct ExConnectToHost < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ENetConnection, address: CowArg < 'a, GString >, port: i32, channels: i32, data: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConnectToHost < 'a > {
    fn new(surround_object: &'a mut re_export::ENetConnection, address: impl AsArg < GString > + 'a, port: i32,) -> Self {
        let channels = 0i32;
        let data = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, address: address.into_arg(), port: port, channels: channels, data: data,
        }
    }
    #[inline]
    pub fn channels(self, channels: i32) -> Self {
        Self {
            channels: channels, .. self
        }
    }
    #[inline]
    pub fn data(self, data: i32) -> Self {
        Self {
            data: data, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::ENetPacketPeer > > {
        let Self {
            _phantom, surround_object, address, port, channels, data,
        }
        = self;
        re_export::ENetConnection::connect_to_host_full(surround_object, address, port, channels, data,)
    }
}
#[doc = "Default-param extender for [`ENetConnection::service_ex`][super::ENetConnection::service_ex]."]
#[must_use]
pub struct ExService < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ENetConnection, timeout: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExService < 'a > {
    fn new(surround_object: &'a mut re_export::ENetConnection,) -> Self {
        let timeout = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, timeout: timeout,
        }
    }
    #[inline]
    pub fn timeout(self, timeout: i32) -> Self {
        Self {
            timeout: timeout, .. self
        }
    }
    #[inline]
    pub fn done(self) -> VarArray {
        let Self {
            _phantom, surround_object, timeout,
        }
        = self;
        re_export::ENetConnection::service_full(surround_object, timeout,)
    }
}
#[doc = "Default-param extender for [`ENetConnection::bandwidth_limit_ex`][super::ENetConnection::bandwidth_limit_ex]."]
#[must_use]
pub struct ExBandwidthLimit < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ENetConnection, in_bandwidth: i32, out_bandwidth: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBandwidthLimit < 'a > {
    fn new(surround_object: &'a mut re_export::ENetConnection,) -> Self {
        let in_bandwidth = 0i32;
        let out_bandwidth = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, in_bandwidth: in_bandwidth, out_bandwidth: out_bandwidth,
        }
    }
    #[inline]
    pub fn in_bandwidth(self, in_bandwidth: i32) -> Self {
        Self {
            in_bandwidth: in_bandwidth, .. self
        }
    }
    #[inline]
    pub fn out_bandwidth(self, out_bandwidth: i32) -> Self {
        Self {
            out_bandwidth: out_bandwidth, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, in_bandwidth, out_bandwidth,
        }
        = self;
        re_export::ENetConnection::bandwidth_limit_full(surround_object, in_bandwidth, out_bandwidth,)
    }
}
#[doc = "Default-param extender for [`ENetConnection::dtls_client_setup_ex`][super::ENetConnection::dtls_client_setup_ex]."]
#[must_use]
pub struct ExDtlsClientSetup < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ENetConnection, hostname: CowArg < 'a, GString >, client_options: CowArg < 'a, Option < Gd < crate::classes::TlsOptions > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDtlsClientSetup < 'a > {
    fn new(surround_object: &'a mut re_export::ENetConnection, hostname: impl AsArg < GString > + 'a,) -> Self {
        let client_options = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, hostname: hostname.into_arg(), client_options: client_options.into_arg(),
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
            _phantom, surround_object, hostname, client_options,
        }
        = self;
        re_export::ENetConnection::dtls_client_setup_full(surround_object, hostname, client_options,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompressionMode {
    ord: i32
}
impl CompressionMode {
    #[doc(alias = "COMPRESS_NONE")]
    #[doc = "Godot enumerator name: `COMPRESS_NONE`"]
    pub const NONE: CompressionMode = CompressionMode {
        ord: 0i32
    };
    #[doc(alias = "COMPRESS_RANGE_CODER")]
    #[doc = "Godot enumerator name: `COMPRESS_RANGE_CODER`"]
    pub const RANGE_CODER: CompressionMode = CompressionMode {
        ord: 1i32
    };
    #[doc(alias = "COMPRESS_FASTLZ")]
    #[doc = "Godot enumerator name: `COMPRESS_FASTLZ`"]
    pub const FASTLZ: CompressionMode = CompressionMode {
        ord: 2i32
    };
    #[doc(alias = "COMPRESS_ZLIB")]
    #[doc = "Godot enumerator name: `COMPRESS_ZLIB`"]
    pub const ZLIB: CompressionMode = CompressionMode {
        ord: 3i32
    };
    #[doc(alias = "COMPRESS_ZSTD")]
    #[doc = "Godot enumerator name: `COMPRESS_ZSTD`"]
    pub const ZSTD: CompressionMode = CompressionMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for CompressionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CompressionMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CompressionMode {
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
            Self::NONE => "NONE", Self::RANGE_CODER => "RANGE_CODER", Self::FASTLZ => "FASTLZ", Self::ZLIB => "ZLIB", Self::ZSTD => "ZSTD", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CompressionMode::NONE, CompressionMode::RANGE_CODER, CompressionMode::FASTLZ, CompressionMode::ZLIB, CompressionMode::ZSTD]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CompressionMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "COMPRESS_NONE", CompressionMode::NONE), crate::meta::inspect::EnumConstant::new("RANGE_CODER", "COMPRESS_RANGE_CODER", CompressionMode::RANGE_CODER), crate::meta::inspect::EnumConstant::new("FASTLZ", "COMPRESS_FASTLZ", CompressionMode::FASTLZ), crate::meta::inspect::EnumConstant::new("ZLIB", "COMPRESS_ZLIB", CompressionMode::ZLIB), crate::meta::inspect::EnumConstant::new("ZSTD", "COMPRESS_ZSTD", CompressionMode::ZSTD)]
        }
    }
}
impl crate::meta::GodotConvert for CompressionMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CompressionMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CompressionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EventType {
    ord: i32
}
impl EventType {
    #[doc(alias = "EVENT_ERROR")]
    #[doc = "Godot enumerator name: `EVENT_ERROR`"]
    pub const ERROR: EventType = EventType {
        ord: - 1i32
    };
    #[doc(alias = "EVENT_NONE")]
    #[doc = "Godot enumerator name: `EVENT_NONE`"]
    pub const NONE: EventType = EventType {
        ord: 0i32
    };
    #[doc(alias = "EVENT_CONNECT")]
    #[doc = "Godot enumerator name: `EVENT_CONNECT`"]
    pub const CONNECT: EventType = EventType {
        ord: 1i32
    };
    #[doc(alias = "EVENT_DISCONNECT")]
    #[doc = "Godot enumerator name: `EVENT_DISCONNECT`"]
    pub const DISCONNECT: EventType = EventType {
        ord: 2i32
    };
    #[doc(alias = "EVENT_RECEIVE")]
    #[doc = "Godot enumerator name: `EVENT_RECEIVE`"]
    pub const RECEIVE: EventType = EventType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EventType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EventType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ - 1i32 | ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::ERROR => "ERROR", Self::NONE => "NONE", Self::CONNECT => "CONNECT", Self::DISCONNECT => "DISCONNECT", Self::RECEIVE => "RECEIVE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EventType::ERROR, EventType::NONE, EventType::CONNECT, EventType::DISCONNECT, EventType::RECEIVE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EventType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ERROR", "EVENT_ERROR", EventType::ERROR), crate::meta::inspect::EnumConstant::new("NONE", "EVENT_NONE", EventType::NONE), crate::meta::inspect::EnumConstant::new("CONNECT", "EVENT_CONNECT", EventType::CONNECT), crate::meta::inspect::EnumConstant::new("DISCONNECT", "EVENT_DISCONNECT", EventType::DISCONNECT), crate::meta::inspect::EnumConstant::new("RECEIVE", "EVENT_RECEIVE", EventType::RECEIVE)]
        }
    }
}
impl crate::meta::GodotConvert for EventType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EventType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EventType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct HostStatistic {
    ord: i32
}
impl HostStatistic {
    #[doc(alias = "HOST_TOTAL_SENT_DATA")]
    #[doc = "Godot enumerator name: `HOST_TOTAL_SENT_DATA`"]
    pub const TOTAL_SENT_DATA: HostStatistic = HostStatistic {
        ord: 0i32
    };
    #[doc(alias = "HOST_TOTAL_SENT_PACKETS")]
    #[doc = "Godot enumerator name: `HOST_TOTAL_SENT_PACKETS`"]
    pub const TOTAL_SENT_PACKETS: HostStatistic = HostStatistic {
        ord: 1i32
    };
    #[doc(alias = "HOST_TOTAL_RECEIVED_DATA")]
    #[doc = "Godot enumerator name: `HOST_TOTAL_RECEIVED_DATA`"]
    pub const TOTAL_RECEIVED_DATA: HostStatistic = HostStatistic {
        ord: 2i32
    };
    #[doc(alias = "HOST_TOTAL_RECEIVED_PACKETS")]
    #[doc = "Godot enumerator name: `HOST_TOTAL_RECEIVED_PACKETS`"]
    pub const TOTAL_RECEIVED_PACKETS: HostStatistic = HostStatistic {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for HostStatistic {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("HostStatistic") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for HostStatistic {
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
            Self::TOTAL_SENT_DATA => "TOTAL_SENT_DATA", Self::TOTAL_SENT_PACKETS => "TOTAL_SENT_PACKETS", Self::TOTAL_RECEIVED_DATA => "TOTAL_RECEIVED_DATA", Self::TOTAL_RECEIVED_PACKETS => "TOTAL_RECEIVED_PACKETS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[HostStatistic::TOTAL_SENT_DATA, HostStatistic::TOTAL_SENT_PACKETS, HostStatistic::TOTAL_RECEIVED_DATA, HostStatistic::TOTAL_RECEIVED_PACKETS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < HostStatistic >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TOTAL_SENT_DATA", "HOST_TOTAL_SENT_DATA", HostStatistic::TOTAL_SENT_DATA), crate::meta::inspect::EnumConstant::new("TOTAL_SENT_PACKETS", "HOST_TOTAL_SENT_PACKETS", HostStatistic::TOTAL_SENT_PACKETS), crate::meta::inspect::EnumConstant::new("TOTAL_RECEIVED_DATA", "HOST_TOTAL_RECEIVED_DATA", HostStatistic::TOTAL_RECEIVED_DATA), crate::meta::inspect::EnumConstant::new("TOTAL_RECEIVED_PACKETS", "HOST_TOTAL_RECEIVED_PACKETS", HostStatistic::TOTAL_RECEIVED_PACKETS)]
        }
    }
}
impl crate::meta::GodotConvert for HostStatistic {
    type Via = i32;
    
}
impl crate::meta::ToGodot for HostStatistic {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for HostStatistic {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ENetConnection;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for ENetConnection {
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