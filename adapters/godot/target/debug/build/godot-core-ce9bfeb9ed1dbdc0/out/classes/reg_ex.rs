#![doc = "Sidecar module for class [`RegEx`][crate::classes::RegEx].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RegEx` enums](https://docs.godotengine.org/en/stable/classes/class_regex.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RegEx.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`reg_ex`][crate::classes::reg_ex]: sidecar module with related enum/flag types\n* [`IRegEx`][crate::classes::IRegEx]: virtual methods\n\n\nSee also [Godot docs for `RegEx`](https://docs.godotengine.org/en/stable/classes/class_regex.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`RegEx::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RegEx {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`RegEx`][crate::classes::RegEx].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `RegEx` methods](https://docs.godotengine.org/en/stable/classes/class_regex.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRegEx: crate::obj::GodotClass < Base = RegEx > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RegEx {
        pub(crate) fn create_from_string_full(pattern: CowArg < GString >, show_error: bool,) -> Option < Gd < crate::classes::RegEx > > {
            type CallRet = Option < Gd < crate::classes::RegEx > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (pattern, show_error,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegEx", "create_from_string", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_from_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_from_string(pattern: impl AsArg < GString >,) -> Option < Gd < crate::classes::RegEx > > {
            Self::create_from_string_ex(pattern,) . done()
        }
        #[inline]
        pub fn create_from_string_ex < 'a > (pattern: impl AsArg < GString > + 'a,) -> ExCreateFromString < 'a > {
            ExCreateFromString::new(pattern,)
        }
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegEx", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn compile_full(&mut self, pattern: CowArg < GString >, show_error: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (pattern, show_error,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegEx", "compile", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::compile_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn compile(&mut self, pattern: impl AsArg < GString >,) -> crate::global::Error {
            self.compile_ex(pattern,) . done()
        }
        #[inline]
        pub fn compile_ex < 'a > (&'a mut self, pattern: impl AsArg < GString > + 'a,) -> ExCompile < 'a > {
            ExCompile::new(self, pattern,)
        }
        pub(crate) fn search_full(&self, subject: CowArg < GString >, offset: i32, end: i32,) -> Option < Gd < crate::classes::RegExMatch > > {
            type CallRet = Option < Gd < crate::classes::RegExMatch > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, i32,);
            let args = (subject, offset, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegEx", "search", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::search_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn search(&self, subject: impl AsArg < GString >,) -> Option < Gd < crate::classes::RegExMatch > > {
            self.search_ex(subject,) . done()
        }
        #[inline]
        pub fn search_ex < 'a > (&'a self, subject: impl AsArg < GString > + 'a,) -> ExSearch < 'a > {
            ExSearch::new(self, subject,)
        }
        pub(crate) fn search_all_full(&self, subject: CowArg < GString >, offset: i32, end: i32,) -> Array < Gd < crate::classes::RegExMatch > > {
            type CallRet = Array < Gd < crate::classes::RegExMatch > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, i32,);
            let args = (subject, offset, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegEx", "search_all", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::search_all_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn search_all(&self, subject: impl AsArg < GString >,) -> Array < Gd < crate::classes::RegExMatch > > {
            self.search_all_ex(subject,) . done()
        }
        #[inline]
        pub fn search_all_ex < 'a > (&'a self, subject: impl AsArg < GString > + 'a,) -> ExSearchAll < 'a > {
            ExSearchAll::new(self, subject,)
        }
        pub(crate) fn sub_full(&self, subject: CowArg < GString >, replacement: CowArg < GString >, all: bool, offset: i32, end: i32,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, bool, i32, i32,);
            let args = (subject, replacement, all, offset, end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegEx", "sub", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::sub_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn sub(&self, subject: impl AsArg < GString >, replacement: impl AsArg < GString >,) -> GString {
            self.sub_ex(subject, replacement,) . done()
        }
        #[inline]
        pub fn sub_ex < 'a > (&'a self, subject: impl AsArg < GString > + 'a, replacement: impl AsArg < GString > + 'a,) -> ExSub < 'a > {
            ExSub::new(self, subject, replacement,)
        }
        pub fn is_valid(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegEx", "is_valid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pattern(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegEx", "get_pattern", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_group_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegEx", "get_group_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_names(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RegEx", "get_names", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for RegEx {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("RegEx"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RegEx {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for RegEx {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RegEx {
        
    }
    impl crate::obj::cap::GodotDefault for RegEx {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RegEx {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RegEx {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RegEx`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RegEx__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RegEx > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`RegEx::create_from_string_ex`][super::RegEx::create_from_string_ex]."]
#[must_use]
pub struct ExCreateFromString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, pattern: CowArg < 'a, GString >, show_error: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateFromString < 'a > {
    fn new(pattern: impl AsArg < GString > + 'a,) -> Self {
        let show_error = true;
        Self {
            _phantom: std::marker::PhantomData, pattern: pattern.into_arg(), show_error: show_error,
        }
    }
    #[inline]
    pub fn show_error(self, show_error: bool) -> Self {
        Self {
            show_error: show_error, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::RegEx > > {
        let Self {
            _phantom, pattern, show_error,
        }
        = self;
        re_export::RegEx::create_from_string_full(pattern, show_error,)
    }
}
#[doc = "Default-param extender for [`RegEx::compile_ex`][super::RegEx::compile_ex]."]
#[must_use]
pub struct ExCompile < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RegEx, pattern: CowArg < 'a, GString >, show_error: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCompile < 'a > {
    fn new(surround_object: &'a mut re_export::RegEx, pattern: impl AsArg < GString > + 'a,) -> Self {
        let show_error = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, pattern: pattern.into_arg(), show_error: show_error,
        }
    }
    #[inline]
    pub fn show_error(self, show_error: bool) -> Self {
        Self {
            show_error: show_error, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, pattern, show_error,
        }
        = self;
        re_export::RegEx::compile_full(surround_object, pattern, show_error,)
    }
}
#[doc = "Default-param extender for [`RegEx::search_ex`][super::RegEx::search_ex]."]
#[must_use]
pub struct ExSearch < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RegEx, subject: CowArg < 'a, GString >, offset: i32, end: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSearch < 'a > {
    fn new(surround_object: &'a re_export::RegEx, subject: impl AsArg < GString > + 'a,) -> Self {
        let offset = 0i32;
        let end = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, subject: subject.into_arg(), offset: offset, end: end,
        }
    }
    #[inline]
    pub fn offset(self, offset: i32) -> Self {
        Self {
            offset: offset, .. self
        }
    }
    #[inline]
    pub fn end(self, end: i32) -> Self {
        Self {
            end: end, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::RegExMatch > > {
        let Self {
            _phantom, surround_object, subject, offset, end,
        }
        = self;
        re_export::RegEx::search_full(surround_object, subject, offset, end,)
    }
}
#[doc = "Default-param extender for [`RegEx::search_all_ex`][super::RegEx::search_all_ex]."]
#[must_use]
pub struct ExSearchAll < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RegEx, subject: CowArg < 'a, GString >, offset: i32, end: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSearchAll < 'a > {
    fn new(surround_object: &'a re_export::RegEx, subject: impl AsArg < GString > + 'a,) -> Self {
        let offset = 0i32;
        let end = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, subject: subject.into_arg(), offset: offset, end: end,
        }
    }
    #[inline]
    pub fn offset(self, offset: i32) -> Self {
        Self {
            offset: offset, .. self
        }
    }
    #[inline]
    pub fn end(self, end: i32) -> Self {
        Self {
            end: end, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Gd < crate::classes::RegExMatch > > {
        let Self {
            _phantom, surround_object, subject, offset, end,
        }
        = self;
        re_export::RegEx::search_all_full(surround_object, subject, offset, end,)
    }
}
#[doc = "Default-param extender for [`RegEx::sub_ex`][super::RegEx::sub_ex]."]
#[must_use]
pub struct ExSub < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RegEx, subject: CowArg < 'a, GString >, replacement: CowArg < 'a, GString >, all: bool, offset: i32, end: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSub < 'a > {
    fn new(surround_object: &'a re_export::RegEx, subject: impl AsArg < GString > + 'a, replacement: impl AsArg < GString > + 'a,) -> Self {
        let all = false;
        let offset = 0i32;
        let end = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, subject: subject.into_arg(), replacement: replacement.into_arg(), all: all, offset: offset, end: end,
        }
    }
    #[inline]
    pub fn all(self, all: bool) -> Self {
        Self {
            all: all, .. self
        }
    }
    #[inline]
    pub fn offset(self, offset: i32) -> Self {
        Self {
            offset: offset, .. self
        }
    }
    #[inline]
    pub fn end(self, end: i32) -> Self {
        Self {
            end: end, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, subject, replacement, all, offset, end,
        }
        = self;
        re_export::RegEx::sub_full(surround_object, subject, replacement, all, offset, end,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::RegEx;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for RegEx {
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