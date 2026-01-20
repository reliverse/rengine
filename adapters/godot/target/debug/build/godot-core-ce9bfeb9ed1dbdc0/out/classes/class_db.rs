#![doc = "Sidecar module for class [`ClassDb`][crate::classes::ClassDb].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ClassDB` enums](https://docs.godotengine.org/en/stable/classes/class_classdb.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ClassDB.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`class_db`][crate::classes::class_db]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `ClassDB`](https://docs.godotengine.org/en/stable/classes/class_classdb.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ClassDb {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl ClassDb {
        pub fn get_class_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "get_class_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inheriters_from_class(&self, class: impl AsArg < StringName >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "get_inheriters_from_class", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_parent_class(&self, class: impl AsArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "get_parent_class", Some(self.__validated_obj()), args,)
            }
        }
        pub fn class_exists(&self, class: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_exists", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_parent_class(&self, class: impl AsArg < StringName >, inherits: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (class.into_arg(), inherits.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "is_parent_class", Some(self.__validated_obj()), args,)
            }
        }
        pub fn can_instantiate(&self, class: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "can_instantiate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instantiate(&self, class: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "instantiate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn class_get_api_type(&self, class: impl AsArg < StringName >,) -> crate::classes::class_db::ApiType {
            type CallRet = crate::classes::class_db::ApiType;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_api_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn class_has_signal(&self, class: impl AsArg < StringName >, signal: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (class.into_arg(), signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_has_signal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn class_get_signal(&self, class: impl AsArg < StringName >, signal: impl AsArg < StringName >,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (class.into_arg(), signal.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_signal", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn class_get_signal_list_full(&self, class: CowArg < StringName >, no_inheritance: bool,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (class, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_signal_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_get_signal_list_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_get_signal_list(&self, class: impl AsArg < StringName >,) -> Array < VarDictionary > {
            self.class_get_signal_list_ex(class,) . done()
        }
        #[inline]
        pub fn class_get_signal_list_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a,) -> ExClassGetSignalList < 'a > {
            ExClassGetSignalList::new(self, class,)
        }
        pub(crate) fn class_get_property_list_full(&self, class: CowArg < StringName >, no_inheritance: bool,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (class, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_property_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_get_property_list_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_get_property_list(&self, class: impl AsArg < StringName >,) -> Array < VarDictionary > {
            self.class_get_property_list_ex(class,) . done()
        }
        #[inline]
        pub fn class_get_property_list_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a,) -> ExClassGetPropertyList < 'a > {
            ExClassGetPropertyList::new(self, class,)
        }
        pub fn class_get_property_getter(&mut self, class: impl AsArg < StringName >, property: impl AsArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (class.into_arg(), property.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_property_getter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn class_get_property_setter(&mut self, class: impl AsArg < StringName >, property: impl AsArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (class.into_arg(), property.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_property_setter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn class_get_property(&self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, property: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, CowArg < 'a1, StringName >,);
            let args = (object.into_arg(), property.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_property", Some(self.__validated_obj()), args,)
            }
        }
        pub fn class_set_property(&self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, property: impl AsArg < StringName >, value: &Variant,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, CowArg < 'a1, StringName >, RefArg < 'a2, Variant >,);
            let args = (object.into_arg(), property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_set_property", Some(self.__validated_obj()), args,)
            }
        }
        pub fn class_get_property_default_value(&self, class: impl AsArg < StringName >, property: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (class.into_arg(), property.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_property_default_value", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn class_has_method_full(&self, class: CowArg < StringName >, method: CowArg < StringName >, no_inheritance: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, bool,);
            let args = (class, method, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_has_method", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_has_method_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_has_method(&self, class: impl AsArg < StringName >, method: impl AsArg < StringName >,) -> bool {
            self.class_has_method_ex(class, method,) . done()
        }
        #[inline]
        pub fn class_has_method_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a, method: impl AsArg < StringName > + 'a,) -> ExClassHasMethod < 'a > {
            ExClassHasMethod::new(self, class, method,)
        }
        pub(crate) fn class_get_method_argument_count_full(&self, class: CowArg < StringName >, method: CowArg < StringName >, no_inheritance: bool,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, bool,);
            let args = (class, method, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_method_argument_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_get_method_argument_count_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_get_method_argument_count(&self, class: impl AsArg < StringName >, method: impl AsArg < StringName >,) -> i32 {
            self.class_get_method_argument_count_ex(class, method,) . done()
        }
        #[inline]
        pub fn class_get_method_argument_count_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a, method: impl AsArg < StringName > + 'a,) -> ExClassGetMethodArgumentCount < 'a > {
            ExClassGetMethodArgumentCount::new(self, class, method,)
        }
        pub(crate) fn class_get_method_list_full(&self, class: CowArg < StringName >, no_inheritance: bool,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (class, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_method_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_get_method_list_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_get_method_list(&self, class: impl AsArg < StringName >,) -> Array < VarDictionary > {
            self.class_get_method_list_ex(class,) . done()
        }
        #[inline]
        pub fn class_get_method_list_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a,) -> ExClassGetMethodList < 'a > {
            ExClassGetMethodList::new(self, class,)
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn class_call_static(&mut self, class: impl AsArg < StringName >, method: impl AsArg < StringName >, varargs: &[Variant]) -> Variant {
            Self::try_class_call_static(self, class, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_class_call_static(&mut self, class: impl AsArg < StringName >, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < Variant, crate::meta::error::CallError > {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (class.into_arg(), method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2123usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "ClassDb", "class_call_static", Some(self.__validated_obj()), args, varargs)
            }
        }
        pub(crate) fn class_get_integer_constant_list_full(&self, class: CowArg < StringName >, no_inheritance: bool,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (class, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_integer_constant_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_get_integer_constant_list_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_get_integer_constant_list(&self, class: impl AsArg < StringName >,) -> PackedStringArray {
            self.class_get_integer_constant_list_ex(class,) . done()
        }
        #[inline]
        pub fn class_get_integer_constant_list_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a,) -> ExClassGetIntegerConstantList < 'a > {
            ExClassGetIntegerConstantList::new(self, class,)
        }
        pub fn class_has_integer_constant(&self, class: impl AsArg < StringName >, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (class.into_arg(), name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_has_integer_constant", Some(self.__validated_obj()), args,)
            }
        }
        pub fn class_get_integer_constant(&self, class: impl AsArg < StringName >, name: impl AsArg < StringName >,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (class.into_arg(), name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_integer_constant", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn class_has_enum_full(&self, class: CowArg < StringName >, name: CowArg < StringName >, no_inheritance: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, bool,);
            let args = (class, name, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_has_enum", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_has_enum_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_has_enum(&self, class: impl AsArg < StringName >, name: impl AsArg < StringName >,) -> bool {
            self.class_has_enum_ex(class, name,) . done()
        }
        #[inline]
        pub fn class_has_enum_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a, name: impl AsArg < StringName > + 'a,) -> ExClassHasEnum < 'a > {
            ExClassHasEnum::new(self, class, name,)
        }
        pub(crate) fn class_get_enum_list_full(&self, class: CowArg < StringName >, no_inheritance: bool,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (class, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_enum_list", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_get_enum_list_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_get_enum_list(&self, class: impl AsArg < StringName >,) -> PackedStringArray {
            self.class_get_enum_list_ex(class,) . done()
        }
        #[inline]
        pub fn class_get_enum_list_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a,) -> ExClassGetEnumList < 'a > {
            ExClassGetEnumList::new(self, class,)
        }
        pub(crate) fn class_get_enum_constants_full(&self, class: CowArg < StringName >, enum_: CowArg < StringName >, no_inheritance: bool,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, bool,);
            let args = (class, enum_, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_enum_constants", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_get_enum_constants_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_get_enum_constants(&self, class: impl AsArg < StringName >, enum_: impl AsArg < StringName >,) -> PackedStringArray {
            self.class_get_enum_constants_ex(class, enum_,) . done()
        }
        #[inline]
        pub fn class_get_enum_constants_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a, enum_: impl AsArg < StringName > + 'a,) -> ExClassGetEnumConstants < 'a > {
            ExClassGetEnumConstants::new(self, class, enum_,)
        }
        pub(crate) fn class_get_integer_constant_enum_full(&self, class: CowArg < StringName >, name: CowArg < StringName >, no_inheritance: bool,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, bool,);
            let args = (class, name, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "class_get_integer_constant_enum", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::class_get_integer_constant_enum_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn class_get_integer_constant_enum(&self, class: impl AsArg < StringName >, name: impl AsArg < StringName >,) -> StringName {
            self.class_get_integer_constant_enum_ex(class, name,) . done()
        }
        #[inline]
        pub fn class_get_integer_constant_enum_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a, name: impl AsArg < StringName > + 'a,) -> ExClassGetIntegerConstantEnum < 'a > {
            ExClassGetIntegerConstantEnum::new(self, class, name,)
        }
        pub(crate) fn is_class_enum_bitfield_full(&self, class: CowArg < StringName >, enum_: CowArg < StringName >, no_inheritance: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, bool,);
            let args = (class, enum_, no_inheritance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "is_class_enum_bitfield", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_class_enum_bitfield_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_class_enum_bitfield(&self, class: impl AsArg < StringName >, enum_: impl AsArg < StringName >,) -> bool {
            self.is_class_enum_bitfield_ex(class, enum_,) . done()
        }
        #[inline]
        pub fn is_class_enum_bitfield_ex < 'a > (&'a self, class: impl AsArg < StringName > + 'a, enum_: impl AsArg < StringName > + 'a,) -> ExIsClassEnumBitfield < 'a > {
            ExIsClassEnumBitfield::new(self, class, enum_,)
        }
        pub fn is_class_enabled(&self, class: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ClassDb", "is_class_enabled", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ClassDb {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ClassDB"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ClassDb {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ClassDb {
        
    }
    impl crate::obj::Singleton for ClassDb {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"ClassDB"))
            }
        }
    }
    impl std::ops::Deref for ClassDb {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ClassDb {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ClassDb__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `ClassDb` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_signal_list_ex`][super::ClassDb::class_get_signal_list_ex]."]
#[must_use]
pub struct ExClassGetSignalList < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetSignalList < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < VarDictionary > {
        let Self {
            _phantom, surround_object, class, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_get_signal_list_full(surround_object, class, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_property_list_ex`][super::ClassDb::class_get_property_list_ex]."]
#[must_use]
pub struct ExClassGetPropertyList < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetPropertyList < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < VarDictionary > {
        let Self {
            _phantom, surround_object, class, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_get_property_list_full(surround_object, class, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_has_method_ex`][super::ClassDb::class_has_method_ex]."]
#[must_use]
pub struct ExClassHasMethod < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, method: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassHasMethod < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a, method: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), method: method.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, class, method, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_has_method_full(surround_object, class, method, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_method_argument_count_ex`][super::ClassDb::class_get_method_argument_count_ex]."]
#[must_use]
pub struct ExClassGetMethodArgumentCount < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, method: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetMethodArgumentCount < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a, method: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), method: method.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, class, method, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_get_method_argument_count_full(surround_object, class, method, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_method_list_ex`][super::ClassDb::class_get_method_list_ex]."]
#[must_use]
pub struct ExClassGetMethodList < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetMethodList < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < VarDictionary > {
        let Self {
            _phantom, surround_object, class, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_get_method_list_full(surround_object, class, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_integer_constant_list_ex`][super::ClassDb::class_get_integer_constant_list_ex]."]
#[must_use]
pub struct ExClassGetIntegerConstantList < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetIntegerConstantList < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, surround_object, class, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_get_integer_constant_list_full(surround_object, class, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_has_enum_ex`][super::ClassDb::class_has_enum_ex]."]
#[must_use]
pub struct ExClassHasEnum < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, name: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassHasEnum < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a, name: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), name: name.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, class, name, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_has_enum_full(surround_object, class, name, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_enum_list_ex`][super::ClassDb::class_get_enum_list_ex]."]
#[must_use]
pub struct ExClassGetEnumList < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetEnumList < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, surround_object, class, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_get_enum_list_full(surround_object, class, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_enum_constants_ex`][super::ClassDb::class_get_enum_constants_ex]."]
#[must_use]
pub struct ExClassGetEnumConstants < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, enum_: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetEnumConstants < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a, enum_: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), enum_: enum_.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, surround_object, class, enum_, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_get_enum_constants_full(surround_object, class, enum_, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::class_get_integer_constant_enum_ex`][super::ClassDb::class_get_integer_constant_enum_ex]."]
#[must_use]
pub struct ExClassGetIntegerConstantEnum < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, name: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClassGetIntegerConstantEnum < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a, name: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), name: name.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> StringName {
        let Self {
            _phantom, surround_object, class, name, no_inheritance,
        }
        = self;
        re_export::ClassDb::class_get_integer_constant_enum_full(surround_object, class, name, no_inheritance,)
    }
}
#[doc = "Default-param extender for [`ClassDb::is_class_enum_bitfield_ex`][super::ClassDb::is_class_enum_bitfield_ex]."]
#[must_use]
pub struct ExIsClassEnumBitfield < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ClassDb, class: CowArg < 'a, StringName >, enum_: CowArg < 'a, StringName >, no_inheritance: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsClassEnumBitfield < 'a > {
    fn new(surround_object: &'a re_export::ClassDb, class: impl AsArg < StringName > + 'a, enum_: impl AsArg < StringName > + 'a,) -> Self {
        let no_inheritance = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, class: class.into_arg(), enum_: enum_.into_arg(), no_inheritance: no_inheritance,
        }
    }
    #[inline]
    pub fn no_inheritance(self, no_inheritance: bool) -> Self {
        Self {
            no_inheritance: no_inheritance, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, class, enum_, no_inheritance,
        }
        = self;
        re_export::ClassDb::is_class_enum_bitfield_full(surround_object, class, enum_, no_inheritance,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `APIType`."]
pub struct ApiType {
    ord: i32
}
impl ApiType {
    #[doc(alias = "API_CORE")]
    #[doc = "Godot enumerator name: `API_CORE`"]
    pub const CORE: ApiType = ApiType {
        ord: 0i32
    };
    #[doc(alias = "API_EDITOR")]
    #[doc = "Godot enumerator name: `API_EDITOR`"]
    pub const EDITOR: ApiType = ApiType {
        ord: 1i32
    };
    #[doc(alias = "API_EXTENSION")]
    #[doc = "Godot enumerator name: `API_EXTENSION`"]
    pub const EXTENSION: ApiType = ApiType {
        ord: 2i32
    };
    #[doc(alias = "API_EDITOR_EXTENSION")]
    #[doc = "Godot enumerator name: `API_EDITOR_EXTENSION`"]
    pub const EDITOR_EXTENSION: ApiType = ApiType {
        ord: 3i32
    };
    #[doc(alias = "API_NONE")]
    #[doc = "Godot enumerator name: `API_NONE`"]
    pub const NONE: ApiType = ApiType {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ApiType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ApiType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ApiType {
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
            Self::CORE => "CORE", Self::EDITOR => "EDITOR", Self::EXTENSION => "EXTENSION", Self::EDITOR_EXTENSION => "EDITOR_EXTENSION", Self::NONE => "NONE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ApiType::CORE, ApiType::EDITOR, ApiType::EXTENSION, ApiType::EDITOR_EXTENSION, ApiType::NONE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ApiType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CORE", "API_CORE", ApiType::CORE), crate::meta::inspect::EnumConstant::new("EDITOR", "API_EDITOR", ApiType::EDITOR), crate::meta::inspect::EnumConstant::new("EXTENSION", "API_EXTENSION", ApiType::EXTENSION), crate::meta::inspect::EnumConstant::new("EDITOR_EXTENSION", "API_EDITOR_EXTENSION", ApiType::EDITOR_EXTENSION), crate::meta::inspect::EnumConstant::new("NONE", "API_NONE", ApiType::NONE)]
        }
    }
}
impl crate::meta::GodotConvert for ApiType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ApiType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ApiType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ClassDb;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for ClassDb {
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