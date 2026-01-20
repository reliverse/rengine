#![doc = "Sidecar module for class [`ZipPacker`][crate::classes::ZipPacker].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ZIPPacker` enums](https://docs.godotengine.org/en/stable/classes/class_zippacker.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ZIPPacker.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`zip_packer`][crate::classes::zip_packer]: sidecar module with related enum/flag types\n* [`IZipPacker`][crate::classes::IZipPacker]: virtual methods\n\n\nSee also [Godot docs for `ZIPPacker`](https://docs.godotengine.org/en/stable/classes/class_zippacker.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ZipPacker::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ZipPacker {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ZipPacker`][crate::classes::ZipPacker].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `ZIPPacker` methods](https://docs.godotengine.org/en/stable/classes/class_zippacker.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IZipPacker: crate::obj::GodotClass < Base = ZipPacker > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ZipPacker {
        pub(crate) fn open_full(&mut self, path: CowArg < GString >, append: crate::classes::zip_packer::ZipAppend,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::zip_packer::ZipAppend,);
            let args = (path, append,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ZipPacker", "open", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::open_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn open(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            self.open_ex(path,) . done()
        }
        #[inline]
        pub fn open_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a,) -> ExOpen < 'a > {
            ExOpen::new(self, path,)
        }
        pub fn set_compression_level(&mut self, compression_level: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (compression_level,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ZipPacker", "set_compression_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_compression_level(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ZipPacker", "get_compression_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn start_file(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ZipPacker", "start_file", Some(self.__validated_obj()), args,)
            }
        }
        pub fn write_file(&mut self, data: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ZipPacker", "write_file", Some(self.__validated_obj()), args,)
            }
        }
        pub fn close_file(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11280usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ZipPacker", "close_file", Some(self.__validated_obj()), args,)
            }
        }
        pub fn close(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11281usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ZipPacker", "close", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ZipPacker {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ZIPPacker"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ZipPacker {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ZipPacker {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ZipPacker {
        
    }
    impl crate::obj::cap::GodotDefault for ZipPacker {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ZipPacker {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ZipPacker {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ZipPacker`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ZipPacker__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ZipPacker > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ZipPacker::open_ex`][super::ZipPacker::open_ex]."]
#[must_use]
pub struct ExOpen < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ZipPacker, path: CowArg < 'a, GString >, append: crate::classes::zip_packer::ZipAppend,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExOpen < 'a > {
    fn new(surround_object: &'a mut re_export::ZipPacker, path: impl AsArg < GString > + 'a,) -> Self {
        let append = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), append: append,
        }
    }
    #[inline]
    pub fn append(self, append: crate::classes::zip_packer::ZipAppend) -> Self {
        Self {
            append: append, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, path, append,
        }
        = self;
        re_export::ZipPacker::open_full(surround_object, path, append,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ZipAppend {
    ord: i32
}
impl ZipAppend {
    #[doc(alias = "APPEND_CREATE")]
    #[doc = "Godot enumerator name: `APPEND_CREATE`"]
    pub const CREATE: ZipAppend = ZipAppend {
        ord: 0i32
    };
    #[doc(alias = "APPEND_CREATEAFTER")]
    #[doc = "Godot enumerator name: `APPEND_CREATEAFTER`"]
    pub const CREATEAFTER: ZipAppend = ZipAppend {
        ord: 1i32
    };
    #[doc(alias = "APPEND_ADDINZIP")]
    #[doc = "Godot enumerator name: `APPEND_ADDINZIP`"]
    pub const ADDINZIP: ZipAppend = ZipAppend {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ZipAppend {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ZipAppend") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ZipAppend {
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
            Self::CREATE => "CREATE", Self::CREATEAFTER => "CREATEAFTER", Self::ADDINZIP => "ADDINZIP", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ZipAppend::CREATE, ZipAppend::CREATEAFTER, ZipAppend::ADDINZIP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ZipAppend >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CREATE", "APPEND_CREATE", ZipAppend::CREATE), crate::meta::inspect::EnumConstant::new("CREATEAFTER", "APPEND_CREATEAFTER", ZipAppend::CREATEAFTER), crate::meta::inspect::EnumConstant::new("ADDINZIP", "APPEND_ADDINZIP", ZipAppend::ADDINZIP)]
        }
    }
}
impl crate::meta::GodotConvert for ZipAppend {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ZipAppend {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ZipAppend {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompressionLevel {
    ord: i32
}
impl CompressionLevel {
    #[doc(alias = "COMPRESSION_DEFAULT")]
    #[doc = "Godot enumerator name: `COMPRESSION_DEFAULT`"]
    pub const DEFAULT: CompressionLevel = CompressionLevel {
        ord: - 1i32
    };
    #[doc(alias = "COMPRESSION_NONE")]
    #[doc = "Godot enumerator name: `COMPRESSION_NONE`"]
    pub const NONE: CompressionLevel = CompressionLevel {
        ord: 0i32
    };
    #[doc(alias = "COMPRESSION_FAST")]
    #[doc = "Godot enumerator name: `COMPRESSION_FAST`"]
    pub const FAST: CompressionLevel = CompressionLevel {
        ord: 1i32
    };
    #[doc(alias = "COMPRESSION_BEST")]
    #[doc = "Godot enumerator name: `COMPRESSION_BEST`"]
    pub const BEST: CompressionLevel = CompressionLevel {
        ord: 9i32
    };
    
}
impl std::fmt::Debug for CompressionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CompressionLevel") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CompressionLevel {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ - 1i32 | ord @ 0i32 | ord @ 1i32 | ord @ 9i32 => Some(Self {
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
            Self::DEFAULT => "DEFAULT", Self::NONE => "NONE", Self::FAST => "FAST", Self::BEST => "BEST", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CompressionLevel::DEFAULT, CompressionLevel::NONE, CompressionLevel::FAST, CompressionLevel::BEST]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CompressionLevel >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "COMPRESSION_DEFAULT", CompressionLevel::DEFAULT), crate::meta::inspect::EnumConstant::new("NONE", "COMPRESSION_NONE", CompressionLevel::NONE), crate::meta::inspect::EnumConstant::new("FAST", "COMPRESSION_FAST", CompressionLevel::FAST), crate::meta::inspect::EnumConstant::new("BEST", "COMPRESSION_BEST", CompressionLevel::BEST)]
        }
    }
}
impl crate::meta::GodotConvert for CompressionLevel {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CompressionLevel {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CompressionLevel {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ZipPacker;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for ZipPacker {
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