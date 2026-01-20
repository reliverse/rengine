#![doc = "Sidecar module for class [`Object`][crate::classes::Object].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Object` enums](https://docs.godotengine.org/en/stable/classes/class_object.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Object.`\n\nThis is the base class for all other classes at the root of the hierarchy. Every instance of `Object` can be stored in a [`Gd`][crate::obj::Gd] smart pointer.\n\nRelated symbols:\n\n* [`object`][crate::classes::object]: sidecar module with related enum/flag types\n* [`IObject`][crate::classes::IObject]: virtual methods\n* [`SignalsOfObject`][crate::classes::object::SignalsOfObject]: signal collection\n* [`ObjectNotification`][crate::classes::notify::ObjectNotification]: notification type\n\n\nSee also [Godot docs for `Object`](https://docs.godotengine.org/en/stable/classes/class_object.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Object::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Object {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Object`][crate::classes::Object].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nSee also [Godot docs for `Object` methods](https://docs.godotengine.org/en/stable/classes/class_object.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IObject: crate::obj::GodotClass < Base = Object > + crate::private::You_forgot_the_attribute__godot_api {
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
    #[doc = "Notification type for class [`Object`][crate::classes::Object]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[doc = r""]
    #[doc = r" Contains the [`Unknown`][Self::Unknown] variant for forward compatibility."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum ObjectNotification {
        POSTINITIALIZE = 0i32, PREDELETE = 1i32, EXTENSION_RELOADED = 2i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        #[doc = r""]
        #[doc = r" This is also necessary if you develop an extension on a Godot version and want to be forward-compatible with newer"]
        #[doc = r" versions. If Godot adds new notifications, they will be unknown to your extension, but you can still handle them."]
        Unknown(i32),
    }
    impl From < i32 > for ObjectNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                0i32 => Self::POSTINITIALIZE, 1i32 => Self::PREDELETE, 2i32 => Self::EXTENSION_RELOADED, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < ObjectNotification > for i32 {
        fn from(notification: ObjectNotification) -> i32 {
            match notification {
                ObjectNotification::POSTINITIALIZE => 0i32, ObjectNotification::PREDELETE => 1i32, ObjectNotification::EXTENSION_RELOADED => 2i32, ObjectNotification::Unknown(int) => int,
            }
        }
    }
    impl Object {
        pub fn get_class(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_class", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_class(&self, class: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (class.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "is_class", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set(&mut self, property: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "set", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get(&self, property: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (property.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_indexed(&mut self, property_path: impl AsArg < NodePath >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, NodePath >, RefArg < 'a1, Variant >,);
            let args = (property_path.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "set_indexed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_indexed(&self, property_path: impl AsArg < NodePath >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (property_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_indexed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_property_list(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_property_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_method_list(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_method_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn property_can_revert(&self, property: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (property.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "property_can_revert", Some(self.__validated_obj()), args,)
            }
        }
        pub fn property_get_revert(&self, property: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (property.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "property_get_revert", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn notification(&mut self, what: i32, reversed: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (what, reversed,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "notification", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn to_string(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "to_string", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn raw_set_script(&mut self, script: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
            let args = (RefArg::new(script),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "raw_set_script", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn raw_get_script(&self,) -> Variant {
            type CallRet = Variant;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "raw_get_script", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_meta(&mut self, name: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "set_meta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_meta(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "remove_meta", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_meta_full(&self, name: CowArg < StringName >, default: RefArg < Variant >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (name, default,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_meta", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_meta_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_meta(&self, name: impl AsArg < StringName >,) -> Variant {
            self.get_meta_ex(name,) . done()
        }
        #[inline]
        pub fn get_meta_ex < 'a > (&'a self, name: impl AsArg < StringName > + 'a,) -> ExGetMeta < 'a > {
            ExGetMeta::new(self, name,)
        }
        pub fn has_meta(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "has_meta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_meta_list(&self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_meta_list", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_user_signal_full(&mut self, signal: CowArg < GString >, arguments: RefArg < VarArray >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, VarArray >,);
            let args = (signal, arguments,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "add_user_signal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_user_signal_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_user_signal(&mut self, signal: impl AsArg < GString >,) {
            self.add_user_signal_ex(signal,) . done()
        }
        #[inline]
        pub fn add_user_signal_ex < 'a > (&'a mut self, signal: impl AsArg < GString > + 'a,) -> ExAddUserSignal < 'a > {
            ExAddUserSignal::new(self, signal,)
        }
        pub fn has_user_signal(&self, signal: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "has_user_signal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_user_signal(&mut self, signal: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "remove_user_signal", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn emit_signal(&mut self, signal: impl AsArg < StringName >, varargs: &[Variant]) -> crate::global::Error {
            Self::try_emit_signal(self, signal, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_emit_signal(&mut self, signal: impl AsArg < StringName >, varargs: &[Variant]) -> Result < crate::global::Error, crate::meta::error::CallError > {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(148usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "Object", "emit_signal", Some(self.__validated_obj()), args, varargs)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn call(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Variant {
            Self::try_call(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_call(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < Variant, crate::meta::error::CallError > {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(149usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "Object", "call", Some(self.__validated_obj()), args, varargs)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn call_deferred(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Variant {
            Self::try_call_deferred(self, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_call_deferred(&mut self, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < Variant, crate::meta::error::CallError > {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(150usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "Object", "call_deferred", Some(self.__validated_obj()), args, varargs)
            }
        }
        pub fn set_deferred(&mut self, property: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "set_deferred", Some(self.__validated_obj()), args,)
            }
        }
        pub fn callv(&mut self, method: impl AsArg < StringName >, arg_array: &VarArray,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, VarArray >,);
            let args = (method.into_arg(), RefArg::new(arg_array),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "callv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_method(&self, method: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "has_method", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_method_argument_count(&self, method: impl AsArg < StringName >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (method.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_method_argument_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_signal(&self, signal: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "has_signal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_signal_list(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_signal_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_signal_connection_list(&self, signal: impl AsArg < StringName >,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_signal_connection_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_incoming_connections(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_incoming_connections", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn raw_connect_full(&mut self, signal: CowArg < StringName >, callable: RefArg < Callable >, flags: u32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Callable >, u32,);
            let args = (signal, callable, flags,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "raw_connect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::raw_connect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub(crate) fn raw_connect(&mut self, signal: impl AsArg < StringName >, callable: &Callable,) -> crate::global::Error {
            self.raw_connect_ex(signal, callable,) . done()
        }
        #[inline]
        pub(crate) fn raw_connect_ex < 'a > (&'a mut self, signal: impl AsArg < StringName > + 'a, callable: &'a Callable,) -> ExRawConnect < 'a > {
            ExRawConnect::new(self, signal, callable,)
        }
        pub fn disconnect(&mut self, signal: impl AsArg < StringName >, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Callable >,);
            let args = (signal.into_arg(), RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "disconnect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_connected(&self, signal: impl AsArg < StringName >, callable: &Callable,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Callable >,);
            let args = (signal.into_arg(), RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "is_connected", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_connections(&self, signal: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "has_connections", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_block_signals(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "set_block_signals", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_blocking_signals(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "is_blocking_signals", Some(self.__validated_obj()), args,)
            }
        }
        pub fn notify_property_list_changed(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "notify_property_list_changed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_message_translation(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "set_message_translation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn can_translate_messages(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "can_translate_messages", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn tr_full(&self, message: CowArg < StringName >, context: CowArg < StringName >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (message, context,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "tr", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::tr_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn tr(&self, message: impl AsArg < StringName >,) -> GString {
            self.tr_ex(message,) . done()
        }
        #[inline]
        pub fn tr_ex < 'a > (&'a self, message: impl AsArg < StringName > + 'a,) -> ExTr < 'a > {
            ExTr::new(self, message,)
        }
        pub(crate) fn tr_n_full(&self, message: CowArg < StringName >, plural_message: CowArg < StringName >, n: i32, context: CowArg < StringName >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, i32, CowArg < 'a2, StringName >,);
            let args = (message, plural_message, n, context,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "tr_n", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::tr_n_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn tr_n(&self, message: impl AsArg < StringName >, plural_message: impl AsArg < StringName >, n: i32,) -> GString {
            self.tr_n_ex(message, plural_message, n,) . done()
        }
        #[inline]
        pub fn tr_n_ex < 'a > (&'a self, message: impl AsArg < StringName > + 'a, plural_message: impl AsArg < StringName > + 'a, n: i32,) -> ExTrN < 'a > {
            ExTrN::new(self, message, plural_message, n,)
        }
        pub fn get_translation_domain(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "get_translation_domain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_translation_domain(&mut self, domain: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (domain.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "set_translation_domain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_queued_for_deletion(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "is_queued_for_deletion", Some(self.__validated_obj()), args,)
            }
        }
        pub fn cancel_free(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Object", "cancel_free", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: ObjectNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: ObjectNotification) {
            self.notification(i32::from(what), true);
            
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
        pub(crate) const NOTIFICATION_POSTINITIALIZE: i32 = 0i32;
        pub(crate) const NOTIFICATION_PREDELETE: i32 = 1i32;
        pub(crate) const NOTIFICATION_EXTENSION_RELOADED: i32 = 2i32;
        
    }
    impl crate::obj::GodotClass for Object {
        type Base = crate::obj::NoBase;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Object"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Core;
        
    }
    unsafe impl crate::obj::Bounds for Object {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemDynamic;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    impl crate::obj::cap::GodotDefault for Object {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Object`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Object__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Object::get_meta_ex`][super::Object::get_meta_ex]."]
#[must_use]
pub struct ExGetMeta < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Object, name: CowArg < 'a, StringName >, default: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetMeta < 'a > {
    fn new(surround_object: &'a re_export::Object, name: impl AsArg < StringName > + 'a,) -> Self {
        let default = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), default: CowArg::Owned(default),
        }
    }
    #[inline]
    pub fn default(self, default: &'a Variant) -> Self {
        Self {
            default: CowArg::Borrowed(default), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, name, default,
        }
        = self;
        re_export::Object::get_meta_full(surround_object, name, default.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`Object::add_user_signal_ex`][super::Object::add_user_signal_ex]."]
#[must_use]
pub struct ExAddUserSignal < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Object, signal: CowArg < 'a, GString >, arguments: CowArg < 'a, VarArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddUserSignal < 'a > {
    fn new(surround_object: &'a mut re_export::Object, signal: impl AsArg < GString > + 'a,) -> Self {
        let arguments = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, signal: signal.into_arg(), arguments: CowArg::Owned(arguments),
        }
    }
    #[inline]
    pub fn arguments(self, arguments: &'a VarArray) -> Self {
        Self {
            arguments: CowArg::Borrowed(arguments), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, signal, arguments,
        }
        = self;
        re_export::Object::add_user_signal_full(surround_object, signal, arguments.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`Object::raw_connect_ex`][super::Object::raw_connect_ex]."]
#[must_use]
pub(crate) struct ExRawConnect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Object, signal: CowArg < 'a, StringName >, callable: CowArg < 'a, Callable >, flags: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRawConnect < 'a > {
    fn new(surround_object: &'a mut re_export::Object, signal: impl AsArg < StringName > + 'a, callable: &'a Callable,) -> Self {
        let flags = 0u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, signal: signal.into_arg(), callable: CowArg::Borrowed(callable), flags: flags,
        }
    }
    #[inline]
    pub fn flags(self, flags: u32) -> Self {
        Self {
            flags: flags, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, signal, callable, flags,
        }
        = self;
        re_export::Object::raw_connect_full(surround_object, signal, callable.cow_as_arg(), flags,)
    }
}
#[doc = "Default-param extender for [`Object::tr_ex`][super::Object::tr_ex]."]
#[must_use]
pub struct ExTr < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Object, message: CowArg < 'a, StringName >, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTr < 'a > {
    fn new(surround_object: &'a re_export::Object, message: impl AsArg < StringName > + 'a,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, message, context,
        }
        = self;
        re_export::Object::tr_full(surround_object, message, context,)
    }
}
#[doc = "Default-param extender for [`Object::tr_n_ex`][super::Object::tr_n_ex]."]
#[must_use]
pub struct ExTrN < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Object, message: CowArg < 'a, StringName >, plural_message: CowArg < 'a, StringName >, n: i32, context: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTrN < 'a > {
    fn new(surround_object: &'a re_export::Object, message: impl AsArg < StringName > + 'a, plural_message: impl AsArg < StringName > + 'a, n: i32,) -> Self {
        let context = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: message.into_arg(), plural_message: plural_message.into_arg(), n: n, context: CowArg::Owned(context),
        }
    }
    #[inline]
    pub fn context(self, context: impl AsArg < StringName > + 'a) -> Self {
        Self {
            context: context.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, message, plural_message, n, context,
        }
        = self;
        re_export::Object::tr_n_full(surround_object, message, plural_message, n, context,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ConnectFlags {
    ord: u64
}
impl ConnectFlags {
    #[doc(alias = "CONNECT_DEFERRED")]
    #[doc = "Godot enumerator name: `CONNECT_DEFERRED`"]
    pub const DEFERRED: ConnectFlags = ConnectFlags {
        ord: 1u64
    };
    #[doc(alias = "CONNECT_PERSIST")]
    #[doc = "Godot enumerator name: `CONNECT_PERSIST`"]
    pub const PERSIST: ConnectFlags = ConnectFlags {
        ord: 2u64
    };
    #[doc(alias = "CONNECT_ONE_SHOT")]
    #[doc = "Godot enumerator name: `CONNECT_ONE_SHOT`"]
    pub const ONE_SHOT: ConnectFlags = ConnectFlags {
        ord: 4u64
    };
    #[doc(alias = "CONNECT_REFERENCE_COUNTED")]
    #[doc = "Godot enumerator name: `CONNECT_REFERENCE_COUNTED`"]
    pub const REFERENCE_COUNTED: ConnectFlags = ConnectFlags {
        ord: 8u64
    };
    #[doc(alias = "CONNECT_APPEND_SOURCE_OBJECT")]
    #[doc = "Godot enumerator name: `CONNECT_APPEND_SOURCE_OBJECT`"]
    pub const APPEND_SOURCE_OBJECT: ConnectFlags = ConnectFlags {
        ord: 16u64
    };
    
}
impl std::fmt::Debug for ConnectFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::DEFERRED => "DEFERRED", Self::PERSIST => "PERSIST", Self::ONE_SHOT => "ONE_SHOT", Self::REFERENCE_COUNTED => "REFERENCE_COUNTED", Self::APPEND_SOURCE_OBJECT => "APPEND_SOURCE_OBJECT", _ => {
                f.debug_struct("ConnectFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for ConnectFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ConnectFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFERRED", "CONNECT_DEFERRED", ConnectFlags::DEFERRED), crate::meta::inspect::EnumConstant::new("PERSIST", "CONNECT_PERSIST", ConnectFlags::PERSIST), crate::meta::inspect::EnumConstant::new("ONE_SHOT", "CONNECT_ONE_SHOT", ConnectFlags::ONE_SHOT), crate::meta::inspect::EnumConstant::new("REFERENCE_COUNTED", "CONNECT_REFERENCE_COUNTED", ConnectFlags::REFERENCE_COUNTED), crate::meta::inspect::EnumConstant::new("APPEND_SOURCE_OBJECT", "CONNECT_APPEND_SOURCE_OBJECT", ConnectFlags::APPEND_SOURCE_OBJECT)]
        }
    }
}
impl std::ops::BitOr for ConnectFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for ConnectFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for ConnectFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for ConnectFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ConnectFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Object;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Object`][crate::classes::Object] class."]
    pub struct SignalsOfObject < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfObject < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn script_changed(&mut self) -> SigScriptChanged < 'c, C > {
            SigScriptChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "script_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn property_list_changed(&mut self) -> SigPropertyListChanged < 'c, C > {
            SigPropertyListChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "property_list_changed")
            }
        }
    }
    type TypedSigScriptChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigScriptChanged < 'c, C: WithSignals > {
        typed: TypedSigScriptChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigScriptChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigScriptChanged < 'c, C > {
        type Target = TypedSigScriptChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigScriptChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigPropertyListChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigPropertyListChanged < 'c, C: WithSignals > {
        typed: TypedSigPropertyListChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPropertyListChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPropertyListChanged < 'c, C > {
        type Target = TypedSigPropertyListChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPropertyListChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Object {
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