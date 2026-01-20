#![doc = "Sidecar module for class [`Time`][crate::classes::Time].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Time` enums](https://docs.godotengine.org/en/stable/classes/class_time.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Time.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`time`][crate::classes::time]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `Time`](https://docs.godotengine.org/en/stable/classes/class_time.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Time {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Time {
        pub fn get_datetime_dict_from_unix_time(&self, unix_time_val: i64,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i64,);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_datetime_dict_from_unix_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_date_dict_from_unix_time(&self, unix_time_val: i64,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i64,);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_date_dict_from_unix_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_time_dict_from_unix_time(&self, unix_time_val: i64,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i64,);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_time_dict_from_unix_time", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_datetime_string_from_unix_time_full(&self, unix_time_val: i64, use_space: bool,) -> GString {
            type CallRet = GString;
            type CallParams = (i64, bool,);
            let args = (unix_time_val, use_space,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_datetime_string_from_unix_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_datetime_string_from_unix_time_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_datetime_string_from_unix_time(&self, unix_time_val: i64,) -> GString {
            self.get_datetime_string_from_unix_time_ex(unix_time_val,) . done()
        }
        #[inline]
        pub fn get_datetime_string_from_unix_time_ex < 'a > (&'a self, unix_time_val: i64,) -> ExGetDatetimeStringFromUnixTime < 'a > {
            ExGetDatetimeStringFromUnixTime::new(self, unix_time_val,)
        }
        pub fn get_date_string_from_unix_time(&self, unix_time_val: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (i64,);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_date_string_from_unix_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_time_string_from_unix_time(&self, unix_time_val: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (i64,);
            let args = (unix_time_val,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_time_string_from_unix_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_datetime_dict_from_datetime_string(&self, datetime: impl AsArg < GString >, weekday: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (datetime.into_arg(), weekday,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_datetime_dict_from_datetime_string", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_datetime_string_from_datetime_dict(&self, datetime: &VarDictionary, use_space: bool,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >, bool,);
            let args = (RefArg::new(datetime), use_space,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_datetime_string_from_datetime_dict", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_unix_time_from_datetime_dict(&self, datetime: &VarDictionary,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >,);
            let args = (RefArg::new(datetime),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_unix_time_from_datetime_dict", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_unix_time_from_datetime_string(&self, datetime: impl AsArg < GString >,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (datetime.into_arg(),);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_unix_time_from_datetime_string", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_offset_string_from_offset_minutes(&self, offset_minutes: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (i64,);
            let args = (offset_minutes,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_offset_string_from_offset_minutes", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_datetime_dict_from_system_full(&self, utc: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (bool,);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_datetime_dict_from_system", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_datetime_dict_from_system_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_datetime_dict_from_system(&self,) -> VarDictionary {
            self.get_datetime_dict_from_system_ex() . done()
        }
        #[inline]
        pub fn get_datetime_dict_from_system_ex < 'a > (&'a self,) -> ExGetDatetimeDictFromSystem < 'a > {
            ExGetDatetimeDictFromSystem::new(self,)
        }
        pub(crate) fn get_date_dict_from_system_full(&self, utc: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (bool,);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_date_dict_from_system", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_date_dict_from_system_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_date_dict_from_system(&self,) -> VarDictionary {
            self.get_date_dict_from_system_ex() . done()
        }
        #[inline]
        pub fn get_date_dict_from_system_ex < 'a > (&'a self,) -> ExGetDateDictFromSystem < 'a > {
            ExGetDateDictFromSystem::new(self,)
        }
        pub(crate) fn get_time_dict_from_system_full(&self, utc: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (bool,);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_time_dict_from_system", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_time_dict_from_system_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_time_dict_from_system(&self,) -> VarDictionary {
            self.get_time_dict_from_system_ex() . done()
        }
        #[inline]
        pub fn get_time_dict_from_system_ex < 'a > (&'a self,) -> ExGetTimeDictFromSystem < 'a > {
            ExGetTimeDictFromSystem::new(self,)
        }
        pub(crate) fn get_datetime_string_from_system_full(&self, utc: bool, use_space: bool,) -> GString {
            type CallRet = GString;
            type CallParams = (bool, bool,);
            let args = (utc, use_space,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_datetime_string_from_system", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_datetime_string_from_system_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_datetime_string_from_system(&self,) -> GString {
            self.get_datetime_string_from_system_ex() . done()
        }
        #[inline]
        pub fn get_datetime_string_from_system_ex < 'a > (&'a self,) -> ExGetDatetimeStringFromSystem < 'a > {
            ExGetDatetimeStringFromSystem::new(self,)
        }
        pub(crate) fn get_date_string_from_system_full(&self, utc: bool,) -> GString {
            type CallRet = GString;
            type CallParams = (bool,);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_date_string_from_system", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_date_string_from_system_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_date_string_from_system(&self,) -> GString {
            self.get_date_string_from_system_ex() . done()
        }
        #[inline]
        pub fn get_date_string_from_system_ex < 'a > (&'a self,) -> ExGetDateStringFromSystem < 'a > {
            ExGetDateStringFromSystem::new(self,)
        }
        pub(crate) fn get_time_string_from_system_full(&self, utc: bool,) -> GString {
            type CallRet = GString;
            type CallParams = (bool,);
            let args = (utc,);
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_time_string_from_system", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_time_string_from_system_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_time_string_from_system(&self,) -> GString {
            self.get_time_string_from_system_ex() . done()
        }
        #[inline]
        pub fn get_time_string_from_system_ex < 'a > (&'a self,) -> ExGetTimeStringFromSystem < 'a > {
            ExGetTimeStringFromSystem::new(self,)
        }
        pub fn get_time_zone_from_system(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_time_zone_from_system", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_unix_time_from_system(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_unix_time_from_system", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ticks_msec(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_ticks_msec", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ticks_usec(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_core_api() . fptr_by_index(238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Time", "get_ticks_usec", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Time {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Time"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Core;
        
    }
    unsafe impl crate::obj::Bounds for Time {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Time {
        
    }
    impl crate::obj::Singleton for Time {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"Time"))
            }
        }
    }
    impl std::ops::Deref for Time {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Time {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Time__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Time` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Time::get_datetime_string_from_unix_time_ex`][super::Time::get_datetime_string_from_unix_time_ex]."]
#[must_use]
pub struct ExGetDatetimeStringFromUnixTime < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Time, unix_time_val: i64, use_space: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDatetimeStringFromUnixTime < 'a > {
    fn new(surround_object: &'a re_export::Time, unix_time_val: i64,) -> Self {
        let use_space = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, unix_time_val: unix_time_val, use_space: use_space,
        }
    }
    #[inline]
    pub fn use_space(self, use_space: bool) -> Self {
        Self {
            use_space: use_space, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, unix_time_val, use_space,
        }
        = self;
        re_export::Time::get_datetime_string_from_unix_time_full(surround_object, unix_time_val, use_space,)
    }
}
#[doc = "Default-param extender for [`Time::get_datetime_dict_from_system_ex`][super::Time::get_datetime_dict_from_system_ex]."]
#[must_use]
pub struct ExGetDatetimeDictFromSystem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDatetimeDictFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        let utc = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn done(self) -> VarDictionary {
        let Self {
            _phantom, surround_object, utc,
        }
        = self;
        re_export::Time::get_datetime_dict_from_system_full(surround_object, utc,)
    }
}
#[doc = "Default-param extender for [`Time::get_date_dict_from_system_ex`][super::Time::get_date_dict_from_system_ex]."]
#[must_use]
pub struct ExGetDateDictFromSystem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDateDictFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        let utc = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn done(self) -> VarDictionary {
        let Self {
            _phantom, surround_object, utc,
        }
        = self;
        re_export::Time::get_date_dict_from_system_full(surround_object, utc,)
    }
}
#[doc = "Default-param extender for [`Time::get_time_dict_from_system_ex`][super::Time::get_time_dict_from_system_ex]."]
#[must_use]
pub struct ExGetTimeDictFromSystem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetTimeDictFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        let utc = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn done(self) -> VarDictionary {
        let Self {
            _phantom, surround_object, utc,
        }
        = self;
        re_export::Time::get_time_dict_from_system_full(surround_object, utc,)
    }
}
#[doc = "Default-param extender for [`Time::get_datetime_string_from_system_ex`][super::Time::get_datetime_string_from_system_ex]."]
#[must_use]
pub struct ExGetDatetimeStringFromSystem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Time, utc: bool, use_space: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDatetimeStringFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        let utc = false;
        let use_space = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc, use_space: use_space,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn use_space(self, use_space: bool) -> Self {
        Self {
            use_space: use_space, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, utc, use_space,
        }
        = self;
        re_export::Time::get_datetime_string_from_system_full(surround_object, utc, use_space,)
    }
}
#[doc = "Default-param extender for [`Time::get_date_string_from_system_ex`][super::Time::get_date_string_from_system_ex]."]
#[must_use]
pub struct ExGetDateStringFromSystem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDateStringFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        let utc = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, utc,
        }
        = self;
        re_export::Time::get_date_string_from_system_full(surround_object, utc,)
    }
}
#[doc = "Default-param extender for [`Time::get_time_string_from_system_ex`][super::Time::get_time_string_from_system_ex]."]
#[must_use]
pub struct ExGetTimeStringFromSystem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Time, utc: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetTimeStringFromSystem < 'a > {
    fn new(surround_object: &'a re_export::Time,) -> Self {
        let utc = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, utc: utc,
        }
    }
    #[inline]
    pub fn utc(self, utc: bool) -> Self {
        Self {
            utc: utc, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, utc,
        }
        = self;
        re_export::Time::get_time_string_from_system_full(surround_object, utc,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Month {
    ord: i32
}
impl Month {
    #[doc(alias = "MONTH_JANUARY")]
    #[doc = "Godot enumerator name: `MONTH_JANUARY`"]
    pub const JANUARY: Month = Month {
        ord: 1i32
    };
    #[doc(alias = "MONTH_FEBRUARY")]
    #[doc = "Godot enumerator name: `MONTH_FEBRUARY`"]
    pub const FEBRUARY: Month = Month {
        ord: 2i32
    };
    #[doc(alias = "MONTH_MARCH")]
    #[doc = "Godot enumerator name: `MONTH_MARCH`"]
    pub const MARCH: Month = Month {
        ord: 3i32
    };
    #[doc(alias = "MONTH_APRIL")]
    #[doc = "Godot enumerator name: `MONTH_APRIL`"]
    pub const APRIL: Month = Month {
        ord: 4i32
    };
    #[doc(alias = "MONTH_MAY")]
    #[doc = "Godot enumerator name: `MONTH_MAY`"]
    pub const MAY: Month = Month {
        ord: 5i32
    };
    #[doc(alias = "MONTH_JUNE")]
    #[doc = "Godot enumerator name: `MONTH_JUNE`"]
    pub const JUNE: Month = Month {
        ord: 6i32
    };
    #[doc(alias = "MONTH_JULY")]
    #[doc = "Godot enumerator name: `MONTH_JULY`"]
    pub const JULY: Month = Month {
        ord: 7i32
    };
    #[doc(alias = "MONTH_AUGUST")]
    #[doc = "Godot enumerator name: `MONTH_AUGUST`"]
    pub const AUGUST: Month = Month {
        ord: 8i32
    };
    #[doc(alias = "MONTH_SEPTEMBER")]
    #[doc = "Godot enumerator name: `MONTH_SEPTEMBER`"]
    pub const SEPTEMBER: Month = Month {
        ord: 9i32
    };
    #[doc(alias = "MONTH_OCTOBER")]
    #[doc = "Godot enumerator name: `MONTH_OCTOBER`"]
    pub const OCTOBER: Month = Month {
        ord: 10i32
    };
    #[doc(alias = "MONTH_NOVEMBER")]
    #[doc = "Godot enumerator name: `MONTH_NOVEMBER`"]
    pub const NOVEMBER: Month = Month {
        ord: 11i32
    };
    #[doc(alias = "MONTH_DECEMBER")]
    #[doc = "Godot enumerator name: `MONTH_DECEMBER`"]
    pub const DECEMBER: Month = Month {
        ord: 12i32
    };
    
}
impl std::fmt::Debug for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Month") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Month {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 => Some(Self {
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
            Self::JANUARY => "JANUARY", Self::FEBRUARY => "FEBRUARY", Self::MARCH => "MARCH", Self::APRIL => "APRIL", Self::MAY => "MAY", Self::JUNE => "JUNE", Self::JULY => "JULY", Self::AUGUST => "AUGUST", Self::SEPTEMBER => "SEPTEMBER", Self::OCTOBER => "OCTOBER", Self::NOVEMBER => "NOVEMBER", Self::DECEMBER => "DECEMBER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Month::JANUARY, Month::FEBRUARY, Month::MARCH, Month::APRIL, Month::MAY, Month::JUNE, Month::JULY, Month::AUGUST, Month::SEPTEMBER, Month::OCTOBER, Month::NOVEMBER, Month::DECEMBER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Month >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("JANUARY", "MONTH_JANUARY", Month::JANUARY), crate::meta::inspect::EnumConstant::new("FEBRUARY", "MONTH_FEBRUARY", Month::FEBRUARY), crate::meta::inspect::EnumConstant::new("MARCH", "MONTH_MARCH", Month::MARCH), crate::meta::inspect::EnumConstant::new("APRIL", "MONTH_APRIL", Month::APRIL), crate::meta::inspect::EnumConstant::new("MAY", "MONTH_MAY", Month::MAY), crate::meta::inspect::EnumConstant::new("JUNE", "MONTH_JUNE", Month::JUNE), crate::meta::inspect::EnumConstant::new("JULY", "MONTH_JULY", Month::JULY), crate::meta::inspect::EnumConstant::new("AUGUST", "MONTH_AUGUST", Month::AUGUST), crate::meta::inspect::EnumConstant::new("SEPTEMBER", "MONTH_SEPTEMBER", Month::SEPTEMBER), crate::meta::inspect::EnumConstant::new("OCTOBER", "MONTH_OCTOBER", Month::OCTOBER), crate::meta::inspect::EnumConstant::new("NOVEMBER", "MONTH_NOVEMBER", Month::NOVEMBER), crate::meta::inspect::EnumConstant::new("DECEMBER", "MONTH_DECEMBER", Month::DECEMBER)]
        }
    }
}
impl crate::meta::GodotConvert for Month {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Month {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Month {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Weekday {
    ord: i32
}
impl Weekday {
    #[doc(alias = "WEEKDAY_SUNDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_SUNDAY`"]
    pub const SUNDAY: Weekday = Weekday {
        ord: 0i32
    };
    #[doc(alias = "WEEKDAY_MONDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_MONDAY`"]
    pub const MONDAY: Weekday = Weekday {
        ord: 1i32
    };
    #[doc(alias = "WEEKDAY_TUESDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_TUESDAY`"]
    pub const TUESDAY: Weekday = Weekday {
        ord: 2i32
    };
    #[doc(alias = "WEEKDAY_WEDNESDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_WEDNESDAY`"]
    pub const WEDNESDAY: Weekday = Weekday {
        ord: 3i32
    };
    #[doc(alias = "WEEKDAY_THURSDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_THURSDAY`"]
    pub const THURSDAY: Weekday = Weekday {
        ord: 4i32
    };
    #[doc(alias = "WEEKDAY_FRIDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_FRIDAY`"]
    pub const FRIDAY: Weekday = Weekday {
        ord: 5i32
    };
    #[doc(alias = "WEEKDAY_SATURDAY")]
    #[doc = "Godot enumerator name: `WEEKDAY_SATURDAY`"]
    pub const SATURDAY: Weekday = Weekday {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for Weekday {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Weekday") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Weekday {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::SUNDAY => "SUNDAY", Self::MONDAY => "MONDAY", Self::TUESDAY => "TUESDAY", Self::WEDNESDAY => "WEDNESDAY", Self::THURSDAY => "THURSDAY", Self::FRIDAY => "FRIDAY", Self::SATURDAY => "SATURDAY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Weekday::SUNDAY, Weekday::MONDAY, Weekday::TUESDAY, Weekday::WEDNESDAY, Weekday::THURSDAY, Weekday::FRIDAY, Weekday::SATURDAY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Weekday >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SUNDAY", "WEEKDAY_SUNDAY", Weekday::SUNDAY), crate::meta::inspect::EnumConstant::new("MONDAY", "WEEKDAY_MONDAY", Weekday::MONDAY), crate::meta::inspect::EnumConstant::new("TUESDAY", "WEEKDAY_TUESDAY", Weekday::TUESDAY), crate::meta::inspect::EnumConstant::new("WEDNESDAY", "WEEKDAY_WEDNESDAY", Weekday::WEDNESDAY), crate::meta::inspect::EnumConstant::new("THURSDAY", "WEEKDAY_THURSDAY", Weekday::THURSDAY), crate::meta::inspect::EnumConstant::new("FRIDAY", "WEEKDAY_FRIDAY", Weekday::FRIDAY), crate::meta::inspect::EnumConstant::new("SATURDAY", "WEEKDAY_SATURDAY", Weekday::SATURDAY)]
        }
    }
}
impl crate::meta::GodotConvert for Weekday {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Weekday {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Weekday {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Time;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Time {
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