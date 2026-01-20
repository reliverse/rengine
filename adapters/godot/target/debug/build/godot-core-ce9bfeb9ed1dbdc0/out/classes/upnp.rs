#![doc = "Sidecar module for class [`Upnp`][crate::classes::Upnp].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `UPNP` enums](https://docs.godotengine.org/en/stable/classes/class_upnp.html#enumerations).\n\n"]
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
    #[doc = "Godot class `UPNP.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`upnp`][crate::classes::upnp]: sidecar module with related enum/flag types\n* [`IUpnp`][crate::classes::IUpnp]: virtual methods\n\n\nSee also [Godot docs for `UPNP`](https://docs.godotengine.org/en/stable/classes/class_upnp.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Upnp::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Upnp {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Upnp`][crate::classes::Upnp].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `UPNP` methods](https://docs.godotengine.org/en/stable/classes/class_upnp.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IUpnp: crate::obj::GodotClass < Base = Upnp > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Upnp {
        pub fn get_device_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "get_device_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_device(&self, index: i32,) -> Option < Gd < crate::classes::UpnpDevice > > {
            type CallRet = Option < Gd < crate::classes::UpnpDevice > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "get_device", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_device(&mut self, device: impl AsArg < Option < Gd < crate::classes::UpnpDevice >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::UpnpDevice > > >,);
            let args = (device.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10308usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "add_device", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_device(&mut self, index: i32, device: impl AsArg < Option < Gd < crate::classes::UpnpDevice >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::UpnpDevice > > >,);
            let args = (index, device.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10309usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "set_device", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_device(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10310usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "remove_device", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_devices(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10311usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "clear_devices", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gateway(&self,) -> Option < Gd < crate::classes::UpnpDevice > > {
            type CallRet = Option < Gd < crate::classes::UpnpDevice > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10312usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "get_gateway", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn discover_full(&mut self, timeout: i32, ttl: i32, device_filter: CowArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, GString >,);
            let args = (timeout, ttl, device_filter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10313usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "discover", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::discover_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn discover(&mut self,) -> i32 {
            self.discover_ex() . done()
        }
        #[inline]
        pub fn discover_ex < 'a > (&'a mut self,) -> ExDiscover < 'a > {
            ExDiscover::new(self,)
        }
        pub fn query_external_address(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10314usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "query_external_address", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_port_mapping_full(&self, port: i32, port_internal: i32, desc: CowArg < GString >, proto: CowArg < GString >, duration: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (i32, i32, CowArg < 'a0, GString >, CowArg < 'a1, GString >, i32,);
            let args = (port, port_internal, desc, proto, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10315usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "add_port_mapping", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_port_mapping_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_port_mapping(&self, port: i32,) -> i32 {
            self.add_port_mapping_ex(port,) . done()
        }
        #[inline]
        pub fn add_port_mapping_ex < 'a > (&'a self, port: i32,) -> ExAddPortMapping < 'a > {
            ExAddPortMapping::new(self, port,)
        }
        pub(crate) fn delete_port_mapping_full(&self, port: i32, proto: CowArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (port, proto,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10316usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "delete_port_mapping", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::delete_port_mapping_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn delete_port_mapping(&self, port: i32,) -> i32 {
            self.delete_port_mapping_ex(port,) . done()
        }
        #[inline]
        pub fn delete_port_mapping_ex < 'a > (&'a self, port: i32,) -> ExDeletePortMapping < 'a > {
            ExDeletePortMapping::new(self, port,)
        }
        pub fn set_discover_multicast_if(&mut self, m_if: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (m_if.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10317usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "set_discover_multicast_if", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_discover_multicast_if(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10318usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "get_discover_multicast_if", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_discover_local_port(&mut self, port: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10319usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "set_discover_local_port", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_discover_local_port(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10320usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "get_discover_local_port", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_discover_ipv6(&mut self, ipv6: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (ipv6,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10321usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "set_discover_ipv6", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_discover_ipv6(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Upnp", "is_discover_ipv6", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Upnp {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("UPNP"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Upnp {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Upnp {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Upnp {
        
    }
    impl crate::obj::cap::GodotDefault for Upnp {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Upnp {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Upnp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Upnp`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Upnp__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Upnp > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Upnp::discover_ex`][super::Upnp::discover_ex]."]
#[must_use]
pub struct ExDiscover < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Upnp, timeout: i32, ttl: i32, device_filter: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDiscover < 'a > {
    fn new(surround_object: &'a mut re_export::Upnp,) -> Self {
        let timeout = 2000i32;
        let ttl = 2i32;
        let device_filter = GString::from("InternetGatewayDevice");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, timeout: timeout, ttl: ttl, device_filter: CowArg::Owned(device_filter),
        }
    }
    #[inline]
    pub fn timeout(self, timeout: i32) -> Self {
        Self {
            timeout: timeout, .. self
        }
    }
    #[inline]
    pub fn ttl(self, ttl: i32) -> Self {
        Self {
            ttl: ttl, .. self
        }
    }
    #[inline]
    pub fn device_filter(self, device_filter: impl AsArg < GString > + 'a) -> Self {
        Self {
            device_filter: device_filter.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, timeout, ttl, device_filter,
        }
        = self;
        re_export::Upnp::discover_full(surround_object, timeout, ttl, device_filter,)
    }
}
#[doc = "Default-param extender for [`Upnp::add_port_mapping_ex`][super::Upnp::add_port_mapping_ex]."]
#[must_use]
pub struct ExAddPortMapping < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Upnp, port: i32, port_internal: i32, desc: CowArg < 'a, GString >, proto: CowArg < 'a, GString >, duration: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPortMapping < 'a > {
    fn new(surround_object: &'a re_export::Upnp, port: i32,) -> Self {
        let port_internal = 0i32;
        let desc = GString::from("");
        let proto = GString::from("UDP");
        let duration = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, port: port, port_internal: port_internal, desc: CowArg::Owned(desc), proto: CowArg::Owned(proto), duration: duration,
        }
    }
    #[inline]
    pub fn port_internal(self, port_internal: i32) -> Self {
        Self {
            port_internal: port_internal, .. self
        }
    }
    #[inline]
    pub fn desc(self, desc: impl AsArg < GString > + 'a) -> Self {
        Self {
            desc: desc.into_arg(), .. self
        }
    }
    #[inline]
    pub fn proto(self, proto: impl AsArg < GString > + 'a) -> Self {
        Self {
            proto: proto.into_arg(), .. self
        }
    }
    #[inline]
    pub fn duration(self, duration: i32) -> Self {
        Self {
            duration: duration, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, port, port_internal, desc, proto, duration,
        }
        = self;
        re_export::Upnp::add_port_mapping_full(surround_object, port, port_internal, desc, proto, duration,)
    }
}
#[doc = "Default-param extender for [`Upnp::delete_port_mapping_ex`][super::Upnp::delete_port_mapping_ex]."]
#[must_use]
pub struct ExDeletePortMapping < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Upnp, port: i32, proto: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDeletePortMapping < 'a > {
    fn new(surround_object: &'a re_export::Upnp, port: i32,) -> Self {
        let proto = GString::from("UDP");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, port: port, proto: CowArg::Owned(proto),
        }
    }
    #[inline]
    pub fn proto(self, proto: impl AsArg < GString > + 'a) -> Self {
        Self {
            proto: proto.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, port, proto,
        }
        = self;
        re_export::Upnp::delete_port_mapping_full(surround_object, port, proto,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `UPNPResult`."]
pub struct UpnpResult {
    ord: i32
}
impl UpnpResult {
    #[doc(alias = "UPNP_RESULT_SUCCESS")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_SUCCESS`"]
    pub const SUCCESS: UpnpResult = UpnpResult {
        ord: 0i32
    };
    #[doc(alias = "UPNP_RESULT_NOT_AUTHORIZED")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_NOT_AUTHORIZED`"]
    pub const NOT_AUTHORIZED: UpnpResult = UpnpResult {
        ord: 1i32
    };
    #[doc(alias = "UPNP_RESULT_PORT_MAPPING_NOT_FOUND")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_PORT_MAPPING_NOT_FOUND`"]
    pub const PORT_MAPPING_NOT_FOUND: UpnpResult = UpnpResult {
        ord: 2i32
    };
    #[doc(alias = "UPNP_RESULT_INCONSISTENT_PARAMETERS")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INCONSISTENT_PARAMETERS`"]
    pub const INCONSISTENT_PARAMETERS: UpnpResult = UpnpResult {
        ord: 3i32
    };
    #[doc(alias = "UPNP_RESULT_NO_SUCH_ENTRY_IN_ARRAY")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_NO_SUCH_ENTRY_IN_ARRAY`"]
    pub const NO_SUCH_ENTRY_IN_ARRAY: UpnpResult = UpnpResult {
        ord: 4i32
    };
    #[doc(alias = "UPNP_RESULT_ACTION_FAILED")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_ACTION_FAILED`"]
    pub const ACTION_FAILED: UpnpResult = UpnpResult {
        ord: 5i32
    };
    #[doc(alias = "UPNP_RESULT_SRC_IP_WILDCARD_NOT_PERMITTED")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_SRC_IP_WILDCARD_NOT_PERMITTED`"]
    pub const SRC_IP_WILDCARD_NOT_PERMITTED: UpnpResult = UpnpResult {
        ord: 6i32
    };
    #[doc(alias = "UPNP_RESULT_EXT_PORT_WILDCARD_NOT_PERMITTED")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_EXT_PORT_WILDCARD_NOT_PERMITTED`"]
    pub const EXT_PORT_WILDCARD_NOT_PERMITTED: UpnpResult = UpnpResult {
        ord: 7i32
    };
    #[doc(alias = "UPNP_RESULT_INT_PORT_WILDCARD_NOT_PERMITTED")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INT_PORT_WILDCARD_NOT_PERMITTED`"]
    pub const INT_PORT_WILDCARD_NOT_PERMITTED: UpnpResult = UpnpResult {
        ord: 8i32
    };
    #[doc(alias = "UPNP_RESULT_REMOTE_HOST_MUST_BE_WILDCARD")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_REMOTE_HOST_MUST_BE_WILDCARD`"]
    pub const REMOTE_HOST_MUST_BE_WILDCARD: UpnpResult = UpnpResult {
        ord: 9i32
    };
    #[doc(alias = "UPNP_RESULT_EXT_PORT_MUST_BE_WILDCARD")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_EXT_PORT_MUST_BE_WILDCARD`"]
    pub const EXT_PORT_MUST_BE_WILDCARD: UpnpResult = UpnpResult {
        ord: 10i32
    };
    #[doc(alias = "UPNP_RESULT_NO_PORT_MAPS_AVAILABLE")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_NO_PORT_MAPS_AVAILABLE`"]
    pub const NO_PORT_MAPS_AVAILABLE: UpnpResult = UpnpResult {
        ord: 11i32
    };
    #[doc(alias = "UPNP_RESULT_CONFLICT_WITH_OTHER_MECHANISM")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_CONFLICT_WITH_OTHER_MECHANISM`"]
    pub const CONFLICT_WITH_OTHER_MECHANISM: UpnpResult = UpnpResult {
        ord: 12i32
    };
    #[doc(alias = "UPNP_RESULT_CONFLICT_WITH_OTHER_MAPPING")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_CONFLICT_WITH_OTHER_MAPPING`"]
    pub const CONFLICT_WITH_OTHER_MAPPING: UpnpResult = UpnpResult {
        ord: 13i32
    };
    #[doc(alias = "UPNP_RESULT_SAME_PORT_VALUES_REQUIRED")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_SAME_PORT_VALUES_REQUIRED`"]
    pub const SAME_PORT_VALUES_REQUIRED: UpnpResult = UpnpResult {
        ord: 14i32
    };
    #[doc(alias = "UPNP_RESULT_ONLY_PERMANENT_LEASE_SUPPORTED")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_ONLY_PERMANENT_LEASE_SUPPORTED`"]
    pub const ONLY_PERMANENT_LEASE_SUPPORTED: UpnpResult = UpnpResult {
        ord: 15i32
    };
    #[doc(alias = "UPNP_RESULT_INVALID_GATEWAY")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INVALID_GATEWAY`"]
    pub const INVALID_GATEWAY: UpnpResult = UpnpResult {
        ord: 16i32
    };
    #[doc(alias = "UPNP_RESULT_INVALID_PORT")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INVALID_PORT`"]
    pub const INVALID_PORT: UpnpResult = UpnpResult {
        ord: 17i32
    };
    #[doc(alias = "UPNP_RESULT_INVALID_PROTOCOL")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INVALID_PROTOCOL`"]
    pub const INVALID_PROTOCOL: UpnpResult = UpnpResult {
        ord: 18i32
    };
    #[doc(alias = "UPNP_RESULT_INVALID_DURATION")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INVALID_DURATION`"]
    pub const INVALID_DURATION: UpnpResult = UpnpResult {
        ord: 19i32
    };
    #[doc(alias = "UPNP_RESULT_INVALID_ARGS")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INVALID_ARGS`"]
    pub const INVALID_ARGS: UpnpResult = UpnpResult {
        ord: 20i32
    };
    #[doc(alias = "UPNP_RESULT_INVALID_RESPONSE")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INVALID_RESPONSE`"]
    pub const INVALID_RESPONSE: UpnpResult = UpnpResult {
        ord: 21i32
    };
    #[doc(alias = "UPNP_RESULT_INVALID_PARAM")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_INVALID_PARAM`"]
    pub const INVALID_PARAM: UpnpResult = UpnpResult {
        ord: 22i32
    };
    #[doc(alias = "UPNP_RESULT_HTTP_ERROR")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_HTTP_ERROR`"]
    pub const HTTP_ERROR: UpnpResult = UpnpResult {
        ord: 23i32
    };
    #[doc(alias = "UPNP_RESULT_SOCKET_ERROR")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_SOCKET_ERROR`"]
    pub const SOCKET_ERROR: UpnpResult = UpnpResult {
        ord: 24i32
    };
    #[doc(alias = "UPNP_RESULT_MEM_ALLOC_ERROR")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_MEM_ALLOC_ERROR`"]
    pub const MEM_ALLOC_ERROR: UpnpResult = UpnpResult {
        ord: 25i32
    };
    #[doc(alias = "UPNP_RESULT_NO_GATEWAY")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_NO_GATEWAY`"]
    pub const NO_GATEWAY: UpnpResult = UpnpResult {
        ord: 26i32
    };
    #[doc(alias = "UPNP_RESULT_NO_DEVICES")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_NO_DEVICES`"]
    pub const NO_DEVICES: UpnpResult = UpnpResult {
        ord: 27i32
    };
    #[doc(alias = "UPNP_RESULT_UNKNOWN_ERROR")]
    #[doc = "Godot enumerator name: `UPNP_RESULT_UNKNOWN_ERROR`"]
    pub const UNKNOWN_ERROR: UpnpResult = UpnpResult {
        ord: 28i32
    };
    
}
impl std::fmt::Debug for UpnpResult {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("UpnpResult") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for UpnpResult {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 => Some(Self {
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
            Self::SUCCESS => "SUCCESS", Self::NOT_AUTHORIZED => "NOT_AUTHORIZED", Self::PORT_MAPPING_NOT_FOUND => "PORT_MAPPING_NOT_FOUND", Self::INCONSISTENT_PARAMETERS => "INCONSISTENT_PARAMETERS", Self::NO_SUCH_ENTRY_IN_ARRAY => "NO_SUCH_ENTRY_IN_ARRAY", Self::ACTION_FAILED => "ACTION_FAILED", Self::SRC_IP_WILDCARD_NOT_PERMITTED => "SRC_IP_WILDCARD_NOT_PERMITTED", Self::EXT_PORT_WILDCARD_NOT_PERMITTED => "EXT_PORT_WILDCARD_NOT_PERMITTED", Self::INT_PORT_WILDCARD_NOT_PERMITTED => "INT_PORT_WILDCARD_NOT_PERMITTED", Self::REMOTE_HOST_MUST_BE_WILDCARD => "REMOTE_HOST_MUST_BE_WILDCARD", Self::EXT_PORT_MUST_BE_WILDCARD => "EXT_PORT_MUST_BE_WILDCARD", Self::NO_PORT_MAPS_AVAILABLE => "NO_PORT_MAPS_AVAILABLE", Self::CONFLICT_WITH_OTHER_MECHANISM => "CONFLICT_WITH_OTHER_MECHANISM", Self::CONFLICT_WITH_OTHER_MAPPING => "CONFLICT_WITH_OTHER_MAPPING", Self::SAME_PORT_VALUES_REQUIRED => "SAME_PORT_VALUES_REQUIRED", Self::ONLY_PERMANENT_LEASE_SUPPORTED => "ONLY_PERMANENT_LEASE_SUPPORTED", Self::INVALID_GATEWAY => "INVALID_GATEWAY", Self::INVALID_PORT => "INVALID_PORT", Self::INVALID_PROTOCOL => "INVALID_PROTOCOL", Self::INVALID_DURATION => "INVALID_DURATION", Self::INVALID_ARGS => "INVALID_ARGS", Self::INVALID_RESPONSE => "INVALID_RESPONSE", Self::INVALID_PARAM => "INVALID_PARAM", Self::HTTP_ERROR => "HTTP_ERROR", Self::SOCKET_ERROR => "SOCKET_ERROR", Self::MEM_ALLOC_ERROR => "MEM_ALLOC_ERROR", Self::NO_GATEWAY => "NO_GATEWAY", Self::NO_DEVICES => "NO_DEVICES", Self::UNKNOWN_ERROR => "UNKNOWN_ERROR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[UpnpResult::SUCCESS, UpnpResult::NOT_AUTHORIZED, UpnpResult::PORT_MAPPING_NOT_FOUND, UpnpResult::INCONSISTENT_PARAMETERS, UpnpResult::NO_SUCH_ENTRY_IN_ARRAY, UpnpResult::ACTION_FAILED, UpnpResult::SRC_IP_WILDCARD_NOT_PERMITTED, UpnpResult::EXT_PORT_WILDCARD_NOT_PERMITTED, UpnpResult::INT_PORT_WILDCARD_NOT_PERMITTED, UpnpResult::REMOTE_HOST_MUST_BE_WILDCARD, UpnpResult::EXT_PORT_MUST_BE_WILDCARD, UpnpResult::NO_PORT_MAPS_AVAILABLE, UpnpResult::CONFLICT_WITH_OTHER_MECHANISM, UpnpResult::CONFLICT_WITH_OTHER_MAPPING, UpnpResult::SAME_PORT_VALUES_REQUIRED, UpnpResult::ONLY_PERMANENT_LEASE_SUPPORTED, UpnpResult::INVALID_GATEWAY, UpnpResult::INVALID_PORT, UpnpResult::INVALID_PROTOCOL, UpnpResult::INVALID_DURATION, UpnpResult::INVALID_ARGS, UpnpResult::INVALID_RESPONSE, UpnpResult::INVALID_PARAM, UpnpResult::HTTP_ERROR, UpnpResult::SOCKET_ERROR, UpnpResult::MEM_ALLOC_ERROR, UpnpResult::NO_GATEWAY, UpnpResult::NO_DEVICES, UpnpResult::UNKNOWN_ERROR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < UpnpResult >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SUCCESS", "UPNP_RESULT_SUCCESS", UpnpResult::SUCCESS), crate::meta::inspect::EnumConstant::new("NOT_AUTHORIZED", "UPNP_RESULT_NOT_AUTHORIZED", UpnpResult::NOT_AUTHORIZED), crate::meta::inspect::EnumConstant::new("PORT_MAPPING_NOT_FOUND", "UPNP_RESULT_PORT_MAPPING_NOT_FOUND", UpnpResult::PORT_MAPPING_NOT_FOUND), crate::meta::inspect::EnumConstant::new("INCONSISTENT_PARAMETERS", "UPNP_RESULT_INCONSISTENT_PARAMETERS", UpnpResult::INCONSISTENT_PARAMETERS), crate::meta::inspect::EnumConstant::new("NO_SUCH_ENTRY_IN_ARRAY", "UPNP_RESULT_NO_SUCH_ENTRY_IN_ARRAY", UpnpResult::NO_SUCH_ENTRY_IN_ARRAY), crate::meta::inspect::EnumConstant::new("ACTION_FAILED", "UPNP_RESULT_ACTION_FAILED", UpnpResult::ACTION_FAILED), crate::meta::inspect::EnumConstant::new("SRC_IP_WILDCARD_NOT_PERMITTED", "UPNP_RESULT_SRC_IP_WILDCARD_NOT_PERMITTED", UpnpResult::SRC_IP_WILDCARD_NOT_PERMITTED), crate::meta::inspect::EnumConstant::new("EXT_PORT_WILDCARD_NOT_PERMITTED", "UPNP_RESULT_EXT_PORT_WILDCARD_NOT_PERMITTED", UpnpResult::EXT_PORT_WILDCARD_NOT_PERMITTED), crate::meta::inspect::EnumConstant::new("INT_PORT_WILDCARD_NOT_PERMITTED", "UPNP_RESULT_INT_PORT_WILDCARD_NOT_PERMITTED", UpnpResult::INT_PORT_WILDCARD_NOT_PERMITTED), crate::meta::inspect::EnumConstant::new("REMOTE_HOST_MUST_BE_WILDCARD", "UPNP_RESULT_REMOTE_HOST_MUST_BE_WILDCARD", UpnpResult::REMOTE_HOST_MUST_BE_WILDCARD), crate::meta::inspect::EnumConstant::new("EXT_PORT_MUST_BE_WILDCARD", "UPNP_RESULT_EXT_PORT_MUST_BE_WILDCARD", UpnpResult::EXT_PORT_MUST_BE_WILDCARD), crate::meta::inspect::EnumConstant::new("NO_PORT_MAPS_AVAILABLE", "UPNP_RESULT_NO_PORT_MAPS_AVAILABLE", UpnpResult::NO_PORT_MAPS_AVAILABLE), crate::meta::inspect::EnumConstant::new("CONFLICT_WITH_OTHER_MECHANISM", "UPNP_RESULT_CONFLICT_WITH_OTHER_MECHANISM", UpnpResult::CONFLICT_WITH_OTHER_MECHANISM), crate::meta::inspect::EnumConstant::new("CONFLICT_WITH_OTHER_MAPPING", "UPNP_RESULT_CONFLICT_WITH_OTHER_MAPPING", UpnpResult::CONFLICT_WITH_OTHER_MAPPING), crate::meta::inspect::EnumConstant::new("SAME_PORT_VALUES_REQUIRED", "UPNP_RESULT_SAME_PORT_VALUES_REQUIRED", UpnpResult::SAME_PORT_VALUES_REQUIRED), crate::meta::inspect::EnumConstant::new("ONLY_PERMANENT_LEASE_SUPPORTED", "UPNP_RESULT_ONLY_PERMANENT_LEASE_SUPPORTED", UpnpResult::ONLY_PERMANENT_LEASE_SUPPORTED), crate::meta::inspect::EnumConstant::new("INVALID_GATEWAY", "UPNP_RESULT_INVALID_GATEWAY", UpnpResult::INVALID_GATEWAY), crate::meta::inspect::EnumConstant::new("INVALID_PORT", "UPNP_RESULT_INVALID_PORT", UpnpResult::INVALID_PORT), crate::meta::inspect::EnumConstant::new("INVALID_PROTOCOL", "UPNP_RESULT_INVALID_PROTOCOL", UpnpResult::INVALID_PROTOCOL), crate::meta::inspect::EnumConstant::new("INVALID_DURATION", "UPNP_RESULT_INVALID_DURATION", UpnpResult::INVALID_DURATION), crate::meta::inspect::EnumConstant::new("INVALID_ARGS", "UPNP_RESULT_INVALID_ARGS", UpnpResult::INVALID_ARGS), crate::meta::inspect::EnumConstant::new("INVALID_RESPONSE", "UPNP_RESULT_INVALID_RESPONSE", UpnpResult::INVALID_RESPONSE), crate::meta::inspect::EnumConstant::new("INVALID_PARAM", "UPNP_RESULT_INVALID_PARAM", UpnpResult::INVALID_PARAM), crate::meta::inspect::EnumConstant::new("HTTP_ERROR", "UPNP_RESULT_HTTP_ERROR", UpnpResult::HTTP_ERROR), crate::meta::inspect::EnumConstant::new("SOCKET_ERROR", "UPNP_RESULT_SOCKET_ERROR", UpnpResult::SOCKET_ERROR), crate::meta::inspect::EnumConstant::new("MEM_ALLOC_ERROR", "UPNP_RESULT_MEM_ALLOC_ERROR", UpnpResult::MEM_ALLOC_ERROR), crate::meta::inspect::EnumConstant::new("NO_GATEWAY", "UPNP_RESULT_NO_GATEWAY", UpnpResult::NO_GATEWAY), crate::meta::inspect::EnumConstant::new("NO_DEVICES", "UPNP_RESULT_NO_DEVICES", UpnpResult::NO_DEVICES), crate::meta::inspect::EnumConstant::new("UNKNOWN_ERROR", "UPNP_RESULT_UNKNOWN_ERROR", UpnpResult::UNKNOWN_ERROR)]
        }
    }
}
impl crate::meta::GodotConvert for UpnpResult {
    type Via = i32;
    
}
impl crate::meta::ToGodot for UpnpResult {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for UpnpResult {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Upnp;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Upnp {
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