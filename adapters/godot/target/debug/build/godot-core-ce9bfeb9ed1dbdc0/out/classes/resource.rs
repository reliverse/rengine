#![doc = "Sidecar module for class [`Resource`][crate::classes::Resource].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Resource` enums](https://docs.godotengine.org/en/stable/classes/class_resource.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Resource.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`resource`][crate::classes::resource]: sidecar module with related enum/flag types\n* [`IResource`][crate::classes::IResource]: virtual methods\n* [`SignalsOfResource`][crate::classes::resource::SignalsOfResource]: signal collection\n\n\nSee also [Godot docs for `Resource`](https://docs.godotengine.org/en/stable/classes/class_resource.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Resource::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Resource {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Resource`][crate::classes::Resource].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Resource` methods](https://docs.godotengine.org/en/stable/classes/class_resource.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IResource: crate::obj::GodotClass < Base = Resource > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
        fn get_rid(&self,) -> Rid {
            unimplemented !()
        }
        fn reset_state(&mut self,) {
            unimplemented !()
        }
        fn set_path_cache(&self, path: GString,) {
            unimplemented !()
        }
    }
    impl Resource {
        pub fn set_path(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "set_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn take_over_path(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "take_over_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "get_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_path_cache(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "set_path_cache", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_name(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "set_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "get_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rid(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "get_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_local_to_scene(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "set_local_to_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_local_to_scene(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "is_local_to_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_local_scene(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "get_local_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn setup_local_to_scene(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "setup_local_to_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reset_state(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "reset_state", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_id_for_path(&mut self, path: impl AsArg < GString >, id: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (path.into_arg(), id.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "set_id_for_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_id_for_path(&self, path: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "get_id_for_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_built_in(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "is_built_in", Some(self.__validated_obj()), args,)
            }
        }
        pub fn generate_scene_unique_id() -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "generate_scene_unique_id", None, args,)
            }
        }
        pub fn set_scene_unique_id(&mut self, id: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (id.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "set_scene_unique_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scene_unique_id(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "get_scene_unique_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn emit_changed(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "emit_changed", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn duplicate_full(&self, deep: bool,) -> Option < Gd < crate::classes::Resource > > {
            type CallRet = Option < Gd < crate::classes::Resource > >;
            type CallParams = (bool,);
            let args = (deep,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "duplicate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::duplicate_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn duplicate(&self,) -> Option < Gd < crate::classes::Resource > > {
            self.duplicate_ex() . done()
        }
        #[inline]
        pub fn duplicate_ex < 'a > (&'a self,) -> ExDuplicate < 'a > {
            ExDuplicate::new(self,)
        }
        pub(crate) fn duplicate_deep_full(&self, deep_subresources_mode: crate::classes::resource::DeepDuplicateMode,) -> Option < Gd < crate::classes::Resource > > {
            type CallRet = Option < Gd < crate::classes::Resource > >;
            type CallParams = (crate::classes::resource::DeepDuplicateMode,);
            let args = (deep_subresources_mode,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Resource", "duplicate_deep", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::duplicate_deep_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn duplicate_deep(&self,) -> Option < Gd < crate::classes::Resource > > {
            self.duplicate_deep_ex() . done()
        }
        #[inline]
        pub fn duplicate_deep_ex < 'a > (&'a self,) -> ExDuplicateDeep < 'a > {
            ExDuplicateDeep::new(self,)
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
    impl crate::obj::GodotClass for Resource {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Resource"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Core;
        
    }
    unsafe impl crate::obj::Bounds for Resource {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Resource {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Resource {
        
    }
    impl crate::obj::cap::GodotDefault for Resource {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Resource {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Resource {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Resource`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Resource__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Resource::duplicate_ex`][super::Resource::duplicate_ex]."]
#[must_use]
pub struct ExDuplicate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Resource, deep: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDuplicate < 'a > {
    fn new(surround_object: &'a re_export::Resource,) -> Self {
        let deep = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, deep: deep,
        }
    }
    #[inline]
    pub fn deep(self, deep: bool) -> Self {
        Self {
            deep: deep, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Resource > > {
        let Self {
            _phantom, surround_object, deep,
        }
        = self;
        re_export::Resource::duplicate_full(surround_object, deep,)
    }
}
#[doc = "Default-param extender for [`Resource::duplicate_deep_ex`][super::Resource::duplicate_deep_ex]."]
#[must_use]
pub struct ExDuplicateDeep < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Resource, deep_subresources_mode: crate::classes::resource::DeepDuplicateMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDuplicateDeep < 'a > {
    fn new(surround_object: &'a re_export::Resource,) -> Self {
        let deep_subresources_mode = crate::obj::EngineEnum::from_ord(1);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, deep_subresources_mode: deep_subresources_mode,
        }
    }
    #[inline]
    pub fn deep_subresources_mode(self, deep_subresources_mode: crate::classes::resource::DeepDuplicateMode) -> Self {
        Self {
            deep_subresources_mode: deep_subresources_mode, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Resource > > {
        let Self {
            _phantom, surround_object, deep_subresources_mode,
        }
        = self;
        re_export::Resource::duplicate_deep_full(surround_object, deep_subresources_mode,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DeepDuplicateMode {
    ord: i32
}
impl DeepDuplicateMode {
    #[doc(alias = "DEEP_DUPLICATE_NONE")]
    #[doc = "Godot enumerator name: `DEEP_DUPLICATE_NONE`"]
    pub const NONE: DeepDuplicateMode = DeepDuplicateMode {
        ord: 0i32
    };
    #[doc(alias = "DEEP_DUPLICATE_INTERNAL")]
    #[doc = "Godot enumerator name: `DEEP_DUPLICATE_INTERNAL`"]
    pub const INTERNAL: DeepDuplicateMode = DeepDuplicateMode {
        ord: 1i32
    };
    #[doc(alias = "DEEP_DUPLICATE_ALL")]
    #[doc = "Godot enumerator name: `DEEP_DUPLICATE_ALL`"]
    pub const ALL: DeepDuplicateMode = DeepDuplicateMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DeepDuplicateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DeepDuplicateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DeepDuplicateMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
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
            Self::NONE => "NONE", Self::INTERNAL => "INTERNAL", Self::ALL => "ALL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DeepDuplicateMode::NONE, DeepDuplicateMode::INTERNAL, DeepDuplicateMode::ALL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DeepDuplicateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "DEEP_DUPLICATE_NONE", DeepDuplicateMode::NONE), crate::meta::inspect::EnumConstant::new("INTERNAL", "DEEP_DUPLICATE_INTERNAL", DeepDuplicateMode::INTERNAL), crate::meta::inspect::EnumConstant::new("ALL", "DEEP_DUPLICATE_ALL", DeepDuplicateMode::ALL)]
        }
    }
}
impl crate::meta::GodotConvert for DeepDuplicateMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DeepDuplicateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DeepDuplicateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Resource;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Resource`][crate::classes::Resource] class."]
    pub struct SignalsOfResource < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfResource < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn changed(&mut self) -> SigChanged < 'c, C > {
            SigChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn setup_local_to_scene_requested(&mut self) -> SigSetupLocalToSceneRequested < 'c, C > {
            SigSetupLocalToSceneRequested {
                typed: TypedSignal::extract(&mut self.__internal_obj, "setup_local_to_scene_requested")
            }
        }
    }
    type TypedSigChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigChanged < 'c, C: WithSignals > {
        typed: TypedSigChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigChanged < 'c, C > {
        type Target = TypedSigChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSetupLocalToSceneRequested < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSetupLocalToSceneRequested < 'c, C: WithSignals > {
        typed: TypedSigSetupLocalToSceneRequested < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSetupLocalToSceneRequested < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSetupLocalToSceneRequested < 'c, C > {
        type Target = TypedSigSetupLocalToSceneRequested < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSetupLocalToSceneRequested < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Resource {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfResource < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfResource < 'c, C > {
        type Target = < < Resource as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Resource;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfResource < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Resource;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}