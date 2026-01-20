#![doc = "Sidecar module for class [`Translation`][crate::classes::Translation].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Translation` enums](https://docs.godotengine.org/en/stable/classes/class_translation.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Translation.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`translation`][crate::classes::translation]: sidecar module with related enum/flag types\n* [`ITranslation`][crate::classes::ITranslation]: virtual methods\n\n\nSee also [Godot docs for `Translation`](https://docs.godotengine.org/en/stable/classes/class_translation.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Translation::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Translation {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Translation`][crate::classes::Translation].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Translation` methods](https://docs.godotengine.org/en/stable/classes/class_translation.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITranslation: crate::obj::GodotClass < Base = Translation > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_plural_message(&self, src_message: StringName, src_plural_message: StringName, n: i32, context: StringName,) -> StringName {
            unimplemented !()
        }
        fn get_message(&self, src_message: StringName, context: StringName,) -> StringName {
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
    impl Translation {
        pub fn set_locale(&mut self, locale: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (locale.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10030usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "set_locale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_locale(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10031usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "get_locale", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_message_full(&mut self, src_message: CowArg < StringName >, xlated_message: CowArg < StringName >, context: CowArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >,);
            let args = (src_message, xlated_message, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10032usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "add_message", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_message_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_message(&mut self, src_message: impl AsArg < StringName >, xlated_message: impl AsArg < StringName >,) {
            self.add_message_ex(src_message, xlated_message,) . done()
        }
        #[inline]
        pub fn add_message_ex < 'a > (&'a mut self, src_message: impl AsArg < StringName > + 'a, xlated_message: impl AsArg < StringName > + 'a,) -> ExAddMessage < 'a > {
            ExAddMessage::new(self, src_message, xlated_message,)
        }
        pub(crate) fn add_plural_message_full(&mut self, src_message: CowArg < StringName >, xlated_messages: RefArg < PackedStringArray >, context: CowArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, RefArg < 'a1, PackedStringArray >, CowArg < 'a2, StringName >,);
            let args = (src_message, xlated_messages, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10033usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "add_plural_message", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_plural_message_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_plural_message(&mut self, src_message: impl AsArg < StringName >, xlated_messages: &PackedStringArray,) {
            self.add_plural_message_ex(src_message, xlated_messages,) . done()
        }
        #[inline]
        pub fn add_plural_message_ex < 'a > (&'a mut self, src_message: impl AsArg < StringName > + 'a, xlated_messages: &'a PackedStringArray,) -> ExAddPluralMessage < 'a > {
            ExAddPluralMessage::new(self, src_message, xlated_messages,)
        }
        pub(crate) fn get_message_full(&self, src_message: CowArg < StringName >, context: CowArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (src_message, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10034usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "get_message", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_message_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_message(&self, src_message: impl AsArg < StringName >,) -> StringName {
            self.get_message_ex(src_message,) . done()
        }
        #[inline]
        pub fn get_message_ex < 'a > (&'a self, src_message: impl AsArg < StringName > + 'a,) -> ExGetMessage < 'a > {
            ExGetMessage::new(self, src_message,)
        }
        pub(crate) fn get_plural_message_full(&self, src_message: CowArg < StringName >, src_plural_message: CowArg < StringName >, n: i32, context: CowArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, i32, CowArg < 'a2, StringName >,);
            let args = (src_message, src_plural_message, n, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10035usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "get_plural_message", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_plural_message_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_plural_message(&self, src_message: impl AsArg < StringName >, src_plural_message: impl AsArg < StringName >, n: i32,) -> StringName {
            self.get_plural_message_ex(src_message, src_plural_message, n,) . done()
        }
        #[inline]
        pub fn get_plural_message_ex < 'a > (&'a self, src_message: impl AsArg < StringName > + 'a, src_plural_message: impl AsArg < StringName > + 'a, n: i32,) -> ExGetPluralMessage < 'a > {
            ExGetPluralMessage::new(self, src_message, src_plural_message, n,)
        }
        pub(crate) fn erase_message_full(&mut self, src_message: CowArg < StringName >, context: CowArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (src_message, context,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10036usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "erase_message", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::erase_message_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn erase_message(&mut self, src_message: impl AsArg < StringName >,) {
            self.erase_message_ex(src_message,) . done()
        }
        #[inline]
        pub fn erase_message_ex < 'a > (&'a mut self, src_message: impl AsArg < StringName > + 'a,) -> ExEraseMessage < 'a > {
            ExEraseMessage::new(self, src_message,)
        }
        pub fn get_message_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10037usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "get_message_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_translated_message_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10038usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "get_translated_message_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_message_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10039usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Translation", "get_message_count", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Translation {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Translation"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Translation {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Translation {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Translation {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Translation {
        
    }
    impl crate::obj::cap::GodotDefault for Translation {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Translation {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Translation {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Translation`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Translation__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Translation > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Translation::add_message_ex`][super::Translation::add_message_ex]."]
#[must_use]
pub struct ExAddMessage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Translation, src_message: CowArg < 'a, StringName >, xlated_message: CowArg < 'a, StringName >, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddMessage < 'a > {
    fn new(surround_object: &'a mut re_export::Translation, src_message: impl AsArg < StringName > + 'a, xlated_message: impl AsArg < StringName > + 'a,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, src_message: src_message.into_arg(), xlated_message: xlated_message.into_arg(), context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, src_message, xlated_message, context,
        }
        = self;
        re_export::Translation::add_message_full(surround_object, src_message, xlated_message, context,)
    }
}
#[doc = "Default-param extender for [`Translation::add_plural_message_ex`][super::Translation::add_plural_message_ex]."]
#[must_use]
pub struct ExAddPluralMessage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Translation, src_message: CowArg < 'a, StringName >, xlated_messages: CowArg < 'a, PackedStringArray >, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPluralMessage < 'a > {
    fn new(surround_object: &'a mut re_export::Translation, src_message: impl AsArg < StringName > + 'a, xlated_messages: &'a PackedStringArray,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, src_message: src_message.into_arg(), xlated_messages: CowArg::Borrowed(xlated_messages), context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, src_message, xlated_messages, context,
        }
        = self;
        re_export::Translation::add_plural_message_full(surround_object, src_message, xlated_messages.cow_as_arg(), context,)
    }
}
#[doc = "Default-param extender for [`Translation::get_message_ex`][super::Translation::get_message_ex]."]
#[must_use]
pub struct ExGetMessage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Translation, src_message: CowArg < 'a, StringName >, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetMessage < 'a > {
    fn new(surround_object: &'a re_export::Translation, src_message: impl AsArg < StringName > + 'a,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, src_message: src_message.into_arg(), context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> StringName {
        let Self {
            _phantom, surround_object, src_message, context,
        }
        = self;
        re_export::Translation::get_message_full(surround_object, src_message, context,)
    }
}
#[doc = "Default-param extender for [`Translation::get_plural_message_ex`][super::Translation::get_plural_message_ex]."]
#[must_use]
pub struct ExGetPluralMessage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Translation, src_message: CowArg < 'a, StringName >, src_plural_message: CowArg < 'a, StringName >, n: i32, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetPluralMessage < 'a > {
    fn new(surround_object: &'a re_export::Translation, src_message: impl AsArg < StringName > + 'a, src_plural_message: impl AsArg < StringName > + 'a, n: i32,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, src_message: src_message.into_arg(), src_plural_message: src_plural_message.into_arg(), n: n, context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> StringName {
        let Self {
            _phantom, surround_object, src_message, src_plural_message, n, context,
        }
        = self;
        re_export::Translation::get_plural_message_full(surround_object, src_message, src_plural_message, n, context,)
    }
}
#[doc = "Default-param extender for [`Translation::erase_message_ex`][super::Translation::erase_message_ex]."]
#[must_use]
pub struct ExEraseMessage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Translation, src_message: CowArg < 'a, StringName >, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExEraseMessage < 'a > {
    fn new(surround_object: &'a mut re_export::Translation, src_message: impl AsArg < StringName > + 'a,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, src_message: src_message.into_arg(), context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, src_message, context,
        }
        = self;
        re_export::Translation::erase_message_full(surround_object, src_message, context,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Translation;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Translation {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfResource < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}