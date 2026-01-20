#![doc = "Sidecar module for class [`OpenXrFutureExtension`][crate::classes::OpenXrFutureExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRFutureExtension` enums](https://docs.godotengine.org/en/stable/classes/class_openxrfutureextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRFutureExtension.`\n\nInherits [`OpenXrExtensionWrapper`][crate::classes::OpenXrExtensionWrapper].\n\nRelated symbols:\n\n* [`open_xr_future_extension`][crate::classes::open_xr_future_extension]: sidecar module with related enum/flag types\n* [`IOpenXrFutureExtension`][crate::classes::IOpenXrFutureExtension]: virtual methods\n\n\nSee also [Godot docs for `OpenXRFutureExtension`](https://docs.godotengine.org/en/stable/classes/class_openxrfutureextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`OpenXrFutureExtension::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrFutureExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`OpenXrFutureExtension`][crate::classes::OpenXrFutureExtension].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IOpenXrExtensionWrapper`][crate::classes::IOpenXrExtensionWrapper] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `OpenXRFutureExtension` methods](https://docs.godotengine.org/en/stable/classes/class_openxrfutureextension.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrFutureExtension: crate::obj::GodotClass < Base = OpenXrFutureExtension > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_requested_extensions(&mut self,) -> VarDictionary {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_system_properties_and_get_next_pointer_rawptr(&mut self, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_instance_create_info_and_get_next_pointer_rawptr(&mut self, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_session_create_and_get_next_pointer_rawptr(&mut self, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_swapchain_create_info_and_get_next_pointer_rawptr(&mut self, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_hand_joint_locations_and_get_next_pointer_rawptr(&mut self, hand_index: i32, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_projection_views_and_get_next_pointer_rawptr(&mut self, view_index: i32, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_frame_wait_info_and_get_next_pointer_rawptr(&mut self, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_frame_end_info_and_get_next_pointer_rawptr(&mut self, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_view_locate_info_and_get_next_pointer_rawptr(&mut self, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_reference_space_create_info_and_get_next_pointer_rawptr(&mut self, reference_space_type: i32, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        fn get_composition_layer_count(&mut self,) -> i32 {
            unimplemented !()
        }
        fn get_composition_layer(&mut self, index: i32,) -> u64 {
            unimplemented !()
        }
        fn get_composition_layer_order(&mut self, index: i32,) -> i32 {
            unimplemented !()
        }
        fn get_suggested_tracker_names(&mut self,) -> PackedStringArray {
            unimplemented !()
        }
        fn on_register_metadata(&mut self,) {
            unimplemented !()
        }
        fn on_before_instance_created(&mut self,) {
            unimplemented !()
        }
        fn on_instance_created(&mut self, instance: u64,) {
            unimplemented !()
        }
        fn on_instance_destroyed(&mut self,) {
            unimplemented !()
        }
        fn on_session_created(&mut self, session: u64,) {
            unimplemented !()
        }
        fn on_process(&mut self,) {
            unimplemented !()
        }
        fn on_sync_actions(&mut self,) {
            unimplemented !()
        }
        fn on_pre_render(&mut self,) {
            unimplemented !()
        }
        fn on_main_swapchains_created(&mut self,) {
            unimplemented !()
        }
        fn on_pre_draw_viewport(&mut self, viewport: Rid,) {
            unimplemented !()
        }
        fn on_post_draw_viewport(&mut self, viewport: Rid,) {
            unimplemented !()
        }
        fn on_session_destroyed(&mut self,) {
            unimplemented !()
        }
        fn on_state_idle(&mut self,) {
            unimplemented !()
        }
        fn on_state_ready(&mut self,) {
            unimplemented !()
        }
        fn on_state_synchronized(&mut self,) {
            unimplemented !()
        }
        fn on_state_visible(&mut self,) {
            unimplemented !()
        }
        fn on_state_focused(&mut self,) {
            unimplemented !()
        }
        fn on_state_stopping(&mut self,) {
            unimplemented !()
        }
        fn on_state_loss_pending(&mut self,) {
            unimplemented !()
        }
        fn on_state_exiting(&mut self,) {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn on_event_polled_rawptr(&mut self, event: * const c_void,) -> bool {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_viewport_composition_layer_and_get_next_pointer_rawptr(&mut self, layer: * const c_void, property_values: VarDictionary, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
        fn get_viewport_composition_layer_extension_properties(&mut self,) -> Array < VarDictionary > {
            unimplemented !()
        }
        fn get_viewport_composition_layer_extension_property_defaults(&mut self,) -> VarDictionary {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn on_viewport_composition_layer_destroyed_rawptr(&mut self, layer: * const c_void,) {
            unimplemented !()
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn set_android_surface_swapchain_create_info_and_get_next_pointer_rawptr(&mut self, property_values: VarDictionary, next_pointer: * mut c_void,) -> u64 {
            unimplemented !()
        }
    }
    impl OpenXrFutureExtension {
        pub fn is_active(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6060usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrFutureExtension", "is_active", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn register_future_full(&mut self, future: u64, on_success: RefArg < Callable >,) -> Option < Gd < crate::classes::OpenXrFutureResult > > {
            type CallRet = Option < Gd < crate::classes::OpenXrFutureResult > >;
            type CallParams < 'a0, > = (u64, RefArg < 'a0, Callable >,);
            let args = (future, on_success,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6061usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrFutureExtension", "register_future", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::register_future_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn register_future(&mut self, future: u64,) -> Option < Gd < crate::classes::OpenXrFutureResult > > {
            self.register_future_ex(future,) . done()
        }
        #[inline]
        pub fn register_future_ex < 'a > (&'a mut self, future: u64,) -> ExRegisterFuture < 'a > {
            ExRegisterFuture::new(self, future,)
        }
        pub fn cancel_future(&mut self, future: u64,) {
            type CallRet = ();
            type CallParams = (u64,);
            let args = (future,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6062usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrFutureExtension", "cancel_future", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for OpenXrFutureExtension {
        type Base = crate::classes::OpenXrExtensionWrapper;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OpenXRFutureExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrFutureExtension {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::OpenXrExtensionWrapper > for OpenXrFutureExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrFutureExtension {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrFutureExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrFutureExtension {
        type Target = crate::classes::OpenXrExtensionWrapper;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrFutureExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrFutureExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrFutureExtension__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrFutureExtension > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrExtensionWrapper > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`OpenXrFutureExtension::register_future_ex`][super::OpenXrFutureExtension::register_future_ex]."]
#[must_use]
pub struct ExRegisterFuture < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::OpenXrFutureExtension, future: u64, on_success: CowArg < 'a, Callable >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRegisterFuture < 'a > {
    fn new(surround_object: &'a mut re_export::OpenXrFutureExtension, future: u64,) -> Self {
        let on_success = Callable::invalid();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, future: future, on_success: CowArg::Owned(on_success),
        }
    }
    #[inline]
    pub fn on_success(self, on_success: &'a Callable) -> Self {
        Self {
            on_success: CowArg::Borrowed(on_success), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::OpenXrFutureResult > > {
        let Self {
            _phantom, surround_object, future, on_success,
        }
        = self;
        re_export::OpenXrFutureExtension::register_future_full(surround_object, future, on_success.cow_as_arg(),)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OpenXrFutureExtension;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for OpenXrFutureExtension {
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