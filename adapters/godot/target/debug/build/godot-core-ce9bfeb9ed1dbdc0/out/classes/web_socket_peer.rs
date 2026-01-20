#![doc = "Sidecar module for class [`WebSocketPeer`][crate::classes::WebSocketPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebSocketPeer` enums](https://docs.godotengine.org/en/stable/classes/class_websocketpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WebSocketPeer.`\n\nInherits [`PacketPeer`][crate::classes::PacketPeer].\n\nRelated symbols:\n\n* [`web_socket_peer`][crate::classes::web_socket_peer]: sidecar module with related enum/flag types\n* [`IWebSocketPeer`][crate::classes::IWebSocketPeer]: virtual methods\n\n\nSee also [Godot docs for `WebSocketPeer`](https://docs.godotengine.org/en/stable/classes/class_websocketpeer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`WebSocketPeer::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WebSocketPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`WebSocketPeer`][crate::classes::WebSocketPeer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IPacketPeer`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `WebSocketPeer` methods](https://docs.godotengine.org/en/stable/classes/class_websocketpeer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWebSocketPeer: crate::obj::GodotClass < Base = WebSocketPeer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl WebSocketPeer {
        pub(crate) fn connect_to_url_full(&mut self, url: CowArg < GString >, tls_client_options: CowArg < Option < Gd < crate::classes::TlsOptions > > >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::TlsOptions > > >,);
            let args = (url, tls_client_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10941usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "connect_to_url", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::connect_to_url_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn connect_to_url(&mut self, url: impl AsArg < GString >,) -> crate::global::Error {
            self.connect_to_url_ex(url,) . done()
        }
        #[inline]
        pub fn connect_to_url_ex < 'a > (&'a mut self, url: impl AsArg < GString > + 'a,) -> ExConnectToUrl < 'a > {
            ExConnectToUrl::new(self, url,)
        }
        pub fn accept_stream(&mut self, stream: impl AsArg < Option < Gd < crate::classes::StreamPeer >> >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::StreamPeer > > >,);
            let args = (stream.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10942usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "accept_stream", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn send_full(&mut self, message: RefArg < PackedByteArray >, write_mode: crate::classes::web_socket_peer::WriteMode,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >, crate::classes::web_socket_peer::WriteMode,);
            let args = (message, write_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10943usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "send", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::send_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn send(&mut self, message: &PackedByteArray,) -> crate::global::Error {
            self.send_ex(message,) . done()
        }
        #[inline]
        pub fn send_ex < 'a > (&'a mut self, message: &'a PackedByteArray,) -> ExSend < 'a > {
            ExSend::new(self, message,)
        }
        pub fn send_text(&mut self, message: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (message.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10944usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "send_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn was_string_packet(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10945usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "was_string_packet", Some(self.__validated_obj()), args,)
            }
        }
        pub fn poll(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10946usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "poll", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn close_full(&mut self, code: i32, reason: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (code, reason,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10947usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "close", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::close_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn close(&mut self,) {
            self.close_ex() . done()
        }
        #[inline]
        pub fn close_ex < 'a > (&'a mut self,) -> ExClose < 'a > {
            ExClose::new(self,)
        }
        pub fn get_connected_host(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10948usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_connected_host", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_connected_port(&self,) -> u16 {
            type CallRet = u16;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10949usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_connected_port", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selected_protocol(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10950usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_selected_protocol", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_requested_url(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10951usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_requested_url", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_no_delay(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10952usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "set_no_delay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_outbound_buffered_amount(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10953usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_current_outbound_buffered_amount", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ready_state(&self,) -> crate::classes::web_socket_peer::State {
            type CallRet = crate::classes::web_socket_peer::State;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10954usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_ready_state", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_close_code(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10955usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_close_code", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_close_reason(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10956usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_close_reason", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_supported_protocols(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10957usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_supported_protocols", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_supported_protocols(&mut self, protocols: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(protocols),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10958usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "set_supported_protocols", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_handshake_headers(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10959usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_handshake_headers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_handshake_headers(&mut self, protocols: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(protocols),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10960usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "set_handshake_headers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inbound_buffer_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10961usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_inbound_buffer_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_inbound_buffer_size(&mut self, buffer_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10962usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "set_inbound_buffer_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_outbound_buffer_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10963usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_outbound_buffer_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_outbound_buffer_size(&mut self, buffer_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10964usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "set_outbound_buffer_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_queued_packets(&mut self, buffer_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10965usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "set_max_queued_packets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_queued_packets(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10966usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_max_queued_packets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_heartbeat_interval(&mut self, interval: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (interval,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10967usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "set_heartbeat_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_heartbeat_interval(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10968usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_heartbeat_interval", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for WebSocketPeer {
        type Base = crate::classes::PacketPeer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("WebSocketPeer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WebSocketPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PacketPeer > for WebSocketPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for WebSocketPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for WebSocketPeer {
        
    }
    impl crate::obj::cap::GodotDefault for WebSocketPeer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for WebSocketPeer {
        type Target = crate::classes::PacketPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WebSocketPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`WebSocketPeer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_WebSocketPeer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::WebSocketPeer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PacketPeer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`WebSocketPeer::connect_to_url_ex`][super::WebSocketPeer::connect_to_url_ex]."]
#[must_use]
pub struct ExConnectToUrl < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::WebSocketPeer, url: CowArg < 'a, GString >, tls_client_options: CowArg < 'a, Option < Gd < crate::classes::TlsOptions > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExConnectToUrl < 'a > {
    fn new(surround_object: &'a mut re_export::WebSocketPeer, url: impl AsArg < GString > + 'a,) -> Self {
        let tls_client_options = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, url: url.into_arg(), tls_client_options: tls_client_options.into_arg(),
        }
    }
    #[inline]
    pub fn tls_client_options(self, tls_client_options: impl AsArg < Option < Gd < crate::classes::TlsOptions >> > + 'a) -> Self {
        Self {
            tls_client_options: tls_client_options.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, url, tls_client_options,
        }
        = self;
        re_export::WebSocketPeer::connect_to_url_full(surround_object, url, tls_client_options,)
    }
}
#[doc = "Default-param extender for [`WebSocketPeer::send_ex`][super::WebSocketPeer::send_ex]."]
#[must_use]
pub struct ExSend < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::WebSocketPeer, message: CowArg < 'a, PackedByteArray >, write_mode: crate::classes::web_socket_peer::WriteMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSend < 'a > {
    fn new(surround_object: &'a mut re_export::WebSocketPeer, message: &'a PackedByteArray,) -> Self {
        let write_mode = crate::obj::EngineEnum::from_ord(1);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: CowArg::Borrowed(message), write_mode: write_mode,
        }
    }
    #[inline]
    pub fn write_mode(self, write_mode: crate::classes::web_socket_peer::WriteMode) -> Self {
        Self {
            write_mode: write_mode, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, message, write_mode,
        }
        = self;
        re_export::WebSocketPeer::send_full(surround_object, message.cow_as_arg(), write_mode,)
    }
}
#[doc = "Default-param extender for [`WebSocketPeer::close_ex`][super::WebSocketPeer::close_ex]."]
#[must_use]
pub struct ExClose < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::WebSocketPeer, code: i32, reason: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClose < 'a > {
    fn new(surround_object: &'a mut re_export::WebSocketPeer,) -> Self {
        let code = 1000i32;
        let reason = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, code: code, reason: CowArg::Owned(reason),
        }
    }
    #[inline]
    pub fn code(self, code: i32) -> Self {
        Self {
            code: code, .. self
        }
    }
    #[inline]
    pub fn reason(self, reason: impl AsArg < GString > + 'a) -> Self {
        Self {
            reason: reason.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, code, reason,
        }
        = self;
        re_export::WebSocketPeer::close_full(surround_object, code, reason,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct WriteMode {
    ord: i32
}
impl WriteMode {
    #[doc(alias = "WRITE_MODE_TEXT")]
    #[doc = "Godot enumerator name: `WRITE_MODE_TEXT`"]
    pub const TEXT: WriteMode = WriteMode {
        ord: 0i32
    };
    #[doc(alias = "WRITE_MODE_BINARY")]
    #[doc = "Godot enumerator name: `WRITE_MODE_BINARY`"]
    pub const BINARY: WriteMode = WriteMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for WriteMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("WriteMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for WriteMode {
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
            Self::TEXT => "TEXT", Self::BINARY => "BINARY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[WriteMode::TEXT, WriteMode::BINARY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < WriteMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TEXT", "WRITE_MODE_TEXT", WriteMode::TEXT), crate::meta::inspect::EnumConstant::new("BINARY", "WRITE_MODE_BINARY", WriteMode::BINARY)]
        }
    }
}
impl crate::meta::GodotConvert for WriteMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for WriteMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for WriteMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct State {
    ord: i32
}
impl State {
    #[doc(alias = "STATE_CONNECTING")]
    #[doc = "Godot enumerator name: `STATE_CONNECTING`"]
    pub const CONNECTING: State = State {
        ord: 0i32
    };
    #[doc(alias = "STATE_OPEN")]
    #[doc = "Godot enumerator name: `STATE_OPEN`"]
    pub const OPEN: State = State {
        ord: 1i32
    };
    #[doc(alias = "STATE_CLOSING")]
    #[doc = "Godot enumerator name: `STATE_CLOSING`"]
    pub const CLOSING: State = State {
        ord: 2i32
    };
    #[doc(alias = "STATE_CLOSED")]
    #[doc = "Godot enumerator name: `STATE_CLOSED`"]
    pub const CLOSED: State = State {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("State") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for State {
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
            Self::CONNECTING => "CONNECTING", Self::OPEN => "OPEN", Self::CLOSING => "CLOSING", Self::CLOSED => "CLOSED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[State::CONNECTING, State::OPEN, State::CLOSING, State::CLOSED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < State >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CONNECTING", "STATE_CONNECTING", State::CONNECTING), crate::meta::inspect::EnumConstant::new("OPEN", "STATE_OPEN", State::OPEN), crate::meta::inspect::EnumConstant::new("CLOSING", "STATE_CLOSING", State::CLOSING), crate::meta::inspect::EnumConstant::new("CLOSED", "STATE_CLOSED", State::CLOSED)]
        }
    }
}
impl crate::meta::GodotConvert for State {
    type Via = i32;
    
}
impl crate::meta::ToGodot for State {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for State {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::WebSocketPeer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for WebSocketPeer {
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