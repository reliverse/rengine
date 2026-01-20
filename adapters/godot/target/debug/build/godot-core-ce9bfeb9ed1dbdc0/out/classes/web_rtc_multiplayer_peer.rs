#![doc = "Sidecar module for class [`WebRtcMultiplayerPeer`][crate::classes::WebRtcMultiplayerPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebRTCMultiplayerPeer` enums](https://docs.godotengine.org/en/stable/classes/class_webrtcmultiplayerpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WebRTCMultiplayerPeer.`\n\nInherits [`MultiplayerPeer`][crate::classes::MultiplayerPeer].\n\nRelated symbols:\n\n* [`web_rtc_multiplayer_peer`][crate::classes::web_rtc_multiplayer_peer]: sidecar module with related enum/flag types\n* [`IWebRtcMultiplayerPeer`][crate::classes::IWebRtcMultiplayerPeer]: virtual methods\n\n\nSee also [Godot docs for `WebRTCMultiplayerPeer`](https://docs.godotengine.org/en/stable/classes/class_webrtcmultiplayerpeer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`WebRtcMultiplayerPeer::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WebRtcMultiplayerPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`WebRtcMultiplayerPeer`][crate::classes::WebRtcMultiplayerPeer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IMultiplayerPeer`~~ > ~~`IPacketPeer`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `WebRTCMultiplayerPeer` methods](https://docs.godotengine.org/en/stable/classes/class_webrtcmultiplayerpeer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWebRtcMultiplayerPeer: crate::obj::GodotClass < Base = WebRtcMultiplayerPeer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl WebRtcMultiplayerPeer {
        pub(crate) fn create_server_full(&mut self, channels_config: RefArg < VarArray >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
            let args = (channels_config,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10904usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcMultiplayerPeer", "create_server", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_server_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_server(&mut self,) -> crate::global::Error {
            self.create_server_ex() . done()
        }
        #[inline]
        pub fn create_server_ex < 'a > (&'a mut self,) -> ExCreateServer < 'a > {
            ExCreateServer::new(self,)
        }
        pub(crate) fn create_client_full(&mut self, peer_id: i32, channels_config: RefArg < VarArray >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (i32, RefArg < 'a0, VarArray >,);
            let args = (peer_id, channels_config,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10905usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcMultiplayerPeer", "create_client", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_client_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_client(&mut self, peer_id: i32,) -> crate::global::Error {
            self.create_client_ex(peer_id,) . done()
        }
        #[inline]
        pub fn create_client_ex < 'a > (&'a mut self, peer_id: i32,) -> ExCreateClient < 'a > {
            ExCreateClient::new(self, peer_id,)
        }
        pub(crate) fn create_mesh_full(&mut self, peer_id: i32, channels_config: RefArg < VarArray >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (i32, RefArg < 'a0, VarArray >,);
            let args = (peer_id, channels_config,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10906usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcMultiplayerPeer", "create_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_mesh_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_mesh(&mut self, peer_id: i32,) -> crate::global::Error {
            self.create_mesh_ex(peer_id,) . done()
        }
        #[inline]
        pub fn create_mesh_ex < 'a > (&'a mut self, peer_id: i32,) -> ExCreateMesh < 'a > {
            ExCreateMesh::new(self, peer_id,)
        }
        pub(crate) fn add_peer_full(&mut self, peer: CowArg < Option < Gd < crate::classes::WebRtcPeerConnection > > >, peer_id: i32, unreliable_lifetime: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::WebRtcPeerConnection > > >, i32, i32,);
            let args = (peer, peer_id, unreliable_lifetime,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10907usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcMultiplayerPeer", "add_peer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_peer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_peer(&mut self, peer: impl AsArg < Option < Gd < crate::classes::WebRtcPeerConnection >> >, peer_id: i32,) -> crate::global::Error {
            self.add_peer_ex(peer, peer_id,) . done()
        }
        #[inline]
        pub fn add_peer_ex < 'a > (&'a mut self, peer: impl AsArg < Option < Gd < crate::classes::WebRtcPeerConnection >> > + 'a, peer_id: i32,) -> ExAddPeer < 'a > {
            ExAddPeer::new(self, peer, peer_id,)
        }
        pub fn remove_peer(&mut self, peer_id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (peer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10908usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcMultiplayerPeer", "remove_peer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_peer(&mut self, peer_id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (peer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10909usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcMultiplayerPeer", "has_peer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_peer(&mut self, peer_id: i32,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i32,);
            let args = (peer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10910usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcMultiplayerPeer", "get_peer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_peers(&mut self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10911usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcMultiplayerPeer", "get_peers", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for WebRtcMultiplayerPeer {
        type Base = crate::classes::MultiplayerPeer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("WebRTCMultiplayerPeer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WebRtcMultiplayerPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::MultiplayerPeer > for WebRtcMultiplayerPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PacketPeer > for WebRtcMultiplayerPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for WebRtcMultiplayerPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for WebRtcMultiplayerPeer {
        
    }
    impl crate::obj::cap::GodotDefault for WebRtcMultiplayerPeer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for WebRtcMultiplayerPeer {
        type Target = crate::classes::MultiplayerPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WebRtcMultiplayerPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`WebRtcMultiplayerPeer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_WebRtcMultiplayerPeer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::WebRtcMultiplayerPeer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::MultiplayerPeer > for $Class {
                
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
#[doc = "Default-param extender for [`WebRtcMultiplayerPeer::create_server_ex`][super::WebRtcMultiplayerPeer::create_server_ex]."]
#[must_use]
pub struct ExCreateServer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::WebRtcMultiplayerPeer, channels_config: CowArg < 'a, VarArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateServer < 'a > {
    fn new(surround_object: &'a mut re_export::WebRtcMultiplayerPeer,) -> Self {
        let channels_config = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, channels_config: CowArg::Owned(channels_config),
        }
    }
    #[inline]
    pub fn channels_config(self, channels_config: &'a VarArray) -> Self {
        Self {
            channels_config: CowArg::Borrowed(channels_config), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, channels_config,
        }
        = self;
        re_export::WebRtcMultiplayerPeer::create_server_full(surround_object, channels_config.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`WebRtcMultiplayerPeer::create_client_ex`][super::WebRtcMultiplayerPeer::create_client_ex]."]
#[must_use]
pub struct ExCreateClient < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::WebRtcMultiplayerPeer, peer_id: i32, channels_config: CowArg < 'a, VarArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateClient < 'a > {
    fn new(surround_object: &'a mut re_export::WebRtcMultiplayerPeer, peer_id: i32,) -> Self {
        let channels_config = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, peer_id: peer_id, channels_config: CowArg::Owned(channels_config),
        }
    }
    #[inline]
    pub fn channels_config(self, channels_config: &'a VarArray) -> Self {
        Self {
            channels_config: CowArg::Borrowed(channels_config), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, peer_id, channels_config,
        }
        = self;
        re_export::WebRtcMultiplayerPeer::create_client_full(surround_object, peer_id, channels_config.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`WebRtcMultiplayerPeer::create_mesh_ex`][super::WebRtcMultiplayerPeer::create_mesh_ex]."]
#[must_use]
pub struct ExCreateMesh < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::WebRtcMultiplayerPeer, peer_id: i32, channels_config: CowArg < 'a, VarArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateMesh < 'a > {
    fn new(surround_object: &'a mut re_export::WebRtcMultiplayerPeer, peer_id: i32,) -> Self {
        let channels_config = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, peer_id: peer_id, channels_config: CowArg::Owned(channels_config),
        }
    }
    #[inline]
    pub fn channels_config(self, channels_config: &'a VarArray) -> Self {
        Self {
            channels_config: CowArg::Borrowed(channels_config), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, peer_id, channels_config,
        }
        = self;
        re_export::WebRtcMultiplayerPeer::create_mesh_full(surround_object, peer_id, channels_config.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`WebRtcMultiplayerPeer::add_peer_ex`][super::WebRtcMultiplayerPeer::add_peer_ex]."]
#[must_use]
pub struct ExAddPeer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::WebRtcMultiplayerPeer, peer: CowArg < 'a, Option < Gd < crate::classes::WebRtcPeerConnection > > >, peer_id: i32, unreliable_lifetime: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPeer < 'a > {
    fn new(surround_object: &'a mut re_export::WebRtcMultiplayerPeer, peer: impl AsArg < Option < Gd < crate::classes::WebRtcPeerConnection >> > + 'a, peer_id: i32,) -> Self {
        let unreliable_lifetime = 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, peer: peer.into_arg(), peer_id: peer_id, unreliable_lifetime: unreliable_lifetime,
        }
    }
    #[inline]
    pub fn unreliable_lifetime(self, unreliable_lifetime: i32) -> Self {
        Self {
            unreliable_lifetime: unreliable_lifetime, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, peer, peer_id, unreliable_lifetime,
        }
        = self;
        re_export::WebRtcMultiplayerPeer::add_peer_full(surround_object, peer, peer_id, unreliable_lifetime,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::WebRtcMultiplayerPeer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::multiplayer_peer::SignalsOfMultiplayerPeer;
    impl WithSignals for WebRtcMultiplayerPeer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfMultiplayerPeer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}