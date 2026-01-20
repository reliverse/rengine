#![doc = "Sidecar module for class [`RegExMatch`][crate::classes::RegExMatch].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RegExMatch` enums](https://docs.godotengine.org/en/stable/classes/class_regexmatch.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RegExMatch.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`reg_ex_match`][crate::classes::reg_ex_match]: sidecar module with related enum/flag types\n* [`IRegExMatch`][crate::classes::IRegExMatch]: virtual methods\n\n\nSee also [Godot docs for `RegExMatch`](https://docs.godotengine.org/en/stable/classes/class_regexmatch.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`RegExMatch::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RegExMatch {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`RegExMatch`][crate::classes::RegExMatch].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `RegExMatch` methods](https://docs.godotengine.org/en/stable/classes/class_regexmatch.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRegExMatch: crate::obj::GodotClass < Base = RegExMatch > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RegExMatch {
        pub fn get_subject(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegExMatch", "get_subject", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_group_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegExMatch", "get_group_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_names(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegExMatch", "get_names", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_strings(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegExMatch", "get_strings", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_string_full(&self, name: RefArg < Variant >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegExMatch", "get_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_string(&self,) -> GString {
            self.get_string_ex() . done()
        }
        #[inline]
        pub fn get_string_ex < 'a > (&'a self,) -> ExGetString < 'a > {
            ExGetString::new(self,)
        }
        pub(crate) fn get_start_full(&self, name: RefArg < Variant >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegExMatch", "get_start", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_start_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_start(&self,) -> i32 {
            self.get_start_ex() . done()
        }
        #[inline]
        pub fn get_start_ex < 'a > (&'a self,) -> ExGetStart < 'a > {
            ExGetStart::new(self,)
        }
        pub(crate) fn get_end_full(&self, name: RefArg < Variant >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegExMatch", "get_end", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_end_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_end(&self,) -> i32 {
            self.get_end_ex() . done()
        }
        #[inline]
        pub fn get_end_ex < 'a > (&'a self,) -> ExGetEnd < 'a > {
            ExGetEnd::new(self,)
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
    impl crate::obj::GodotClass for RegExMatch {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("RegExMatch"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RegExMatch {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for RegExMatch {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RegExMatch {
        
    }
    impl crate::obj::cap::GodotDefault for RegExMatch {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RegExMatch {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RegExMatch {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RegExMatch`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RegExMatch__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RegExMatch > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`RegExMatch::get_string_ex`][super::RegExMatch::get_string_ex]."]
#[must_use]
pub struct ExGetString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RegExMatch, name: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetString < 'a > {
    fn new(surround_object: &'a re_export::RegExMatch,) -> Self {
        let name = Variant::from(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name),
        }
    }
    #[inline]
    pub fn name(self, name: &'a Variant) -> Self {
        Self {
            name: CowArg::Borrowed(name), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, name,
        }
        = self;
        re_export::RegExMatch::get_string_full(surround_object, name.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RegExMatch::get_start_ex`][super::RegExMatch::get_start_ex]."]
#[must_use]
pub struct ExGetStart < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RegExMatch, name: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetStart < 'a > {
    fn new(surround_object: &'a re_export::RegExMatch,) -> Self {
        let name = Variant::from(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name),
        }
    }
    #[inline]
    pub fn name(self, name: &'a Variant) -> Self {
        Self {
            name: CowArg::Borrowed(name), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, name,
        }
        = self;
        re_export::RegExMatch::get_start_full(surround_object, name.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RegExMatch::get_end_ex`][super::RegExMatch::get_end_ex]."]
#[must_use]
pub struct ExGetEnd < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RegExMatch, name: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetEnd < 'a > {
    fn new(surround_object: &'a re_export::RegExMatch,) -> Self {
        let name = Variant::from(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name),
        }
    }
    #[inline]
    pub fn name(self, name: &'a Variant) -> Self {
        Self {
            name: CowArg::Borrowed(name), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, name,
        }
        = self;
        re_export::RegExMatch::get_end_full(surround_object, name.cow_as_arg(),)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::RegExMatch;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for RegExMatch {
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