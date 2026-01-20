#![doc = "Sidecar module for class [`JsonRpc`][crate::classes::JsonRpc].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `JSONRPC` enums](https://docs.godotengine.org/en/stable/classes/class_jsonrpc.html#enumerations).\n\n"]
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
    #[doc = "Godot class `JSONRPC.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`json_rpc`][crate::classes::json_rpc]: sidecar module with related enum/flag types\n* [`IJsonRpc`][crate::classes::IJsonRpc]: virtual methods\n\n\nSee also [Godot docs for `JSONRPC`](https://docs.godotengine.org/en/stable/classes/class_jsonrpc.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`JsonRpc::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct JsonRpc {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`JsonRpc`][crate::classes::JsonRpc].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `JSONRPC` methods](https://docs.godotengine.org/en/stable/classes/class_jsonrpc.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IJsonRpc: crate::obj::GodotClass < Base = JsonRpc > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl JsonRpc {
        pub fn set_method(&mut self, name: impl AsArg < GString >, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, Callable >,);
            let args = (name.into_arg(), RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4689usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "JsonRpc", "set_method", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn process_action_full(&mut self, action: RefArg < Variant >, recurse: bool,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >, bool,);
            let args = (action, recurse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4690usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "JsonRpc", "process_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::process_action_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn process_action(&mut self, action: &Variant,) -> Variant {
            self.process_action_ex(action,) . done()
        }
        #[inline]
        pub fn process_action_ex < 'a > (&'a mut self, action: &'a Variant,) -> ExProcessAction < 'a > {
            ExProcessAction::new(self, action,)
        }
        pub fn process_string(&mut self, action: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "JsonRpc", "process_string", Some(self.__validated_obj()), args,)
            }
        }
        pub fn make_request(&mut self, method: impl AsArg < GString >, params: &Variant, id: &Variant,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, RefArg < 'a1, Variant >, RefArg < 'a2, Variant >,);
            let args = (method.into_arg(), RefArg::new(params), RefArg::new(id),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "JsonRpc", "make_request", Some(self.__validated_obj()), args,)
            }
        }
        pub fn make_response(&mut self, result: &Variant, id: &Variant,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >,);
            let args = (RefArg::new(result), RefArg::new(id),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "JsonRpc", "make_response", Some(self.__validated_obj()), args,)
            }
        }
        pub fn make_notification(&mut self, method: impl AsArg < GString >, params: &Variant,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, Variant >,);
            let args = (method.into_arg(), RefArg::new(params),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "JsonRpc", "make_notification", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn make_response_error_full(&self, code: i32, message: CowArg < GString >, id: RefArg < Variant >,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (i32, CowArg < 'a0, GString >, RefArg < 'a1, Variant >,);
            let args = (code, message, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "JsonRpc", "make_response_error", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::make_response_error_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn make_response_error(&self, code: i32, message: impl AsArg < GString >,) -> VarDictionary {
            self.make_response_error_ex(code, message,) . done()
        }
        #[inline]
        pub fn make_response_error_ex < 'a > (&'a self, code: i32, message: impl AsArg < GString > + 'a,) -> ExMakeResponseError < 'a > {
            ExMakeResponseError::new(self, code, message,)
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
    impl crate::obj::GodotClass for JsonRpc {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("JSONRPC"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for JsonRpc {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for JsonRpc {
        
    }
    impl crate::obj::cap::GodotDefault for JsonRpc {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for JsonRpc {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for JsonRpc {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`JsonRpc`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_JsonRpc__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::JsonRpc > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`JsonRpc::process_action_ex`][super::JsonRpc::process_action_ex]."]
#[must_use]
pub struct ExProcessAction < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::JsonRpc, action: CowArg < 'a, Variant >, recurse: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExProcessAction < 'a > {
    fn new(surround_object: &'a mut re_export::JsonRpc, action: &'a Variant,) -> Self {
        let recurse = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: CowArg::Borrowed(action), recurse: recurse,
        }
    }
    #[inline]
    pub fn recurse(self, recurse: bool) -> Self {
        Self {
            recurse: recurse, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, action, recurse,
        }
        = self;
        re_export::JsonRpc::process_action_full(surround_object, action.cow_as_arg(), recurse,)
    }
}
#[doc = "Default-param extender for [`JsonRpc::make_response_error_ex`][super::JsonRpc::make_response_error_ex]."]
#[must_use]
pub struct ExMakeResponseError < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::JsonRpc, code: i32, message: CowArg < 'a, GString >, id: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMakeResponseError < 'a > {
    fn new(surround_object: &'a re_export::JsonRpc, code: i32, message: impl AsArg < GString > + 'a,) -> Self {
        let id = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, code: code, message: message.into_arg(), id: CowArg::Owned(id),
        }
    }
    #[inline]
    pub fn id(self, id: &'a Variant) -> Self {
        Self {
            id: CowArg::Borrowed(id), .. self
        }
    }
    #[inline]
    pub fn done(self) -> VarDictionary {
        let Self {
            _phantom, surround_object, code, message, id,
        }
        = self;
        re_export::JsonRpc::make_response_error_full(surround_object, code, message, id.cow_as_arg(),)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ErrorCode {
    ord: i32
}
impl ErrorCode {
    pub const PARSE_ERROR: ErrorCode = ErrorCode {
        ord: - 32700i32
    };
    pub const INVALID_REQUEST: ErrorCode = ErrorCode {
        ord: - 32600i32
    };
    pub const METHOD_NOT_FOUND: ErrorCode = ErrorCode {
        ord: - 32601i32
    };
    pub const INVALID_PARAMS: ErrorCode = ErrorCode {
        ord: - 32602i32
    };
    pub const INTERNAL_ERROR: ErrorCode = ErrorCode {
        ord: - 32603i32
    };
    
}
impl std::fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ErrorCode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ErrorCode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ - 32700i32 | ord @ - 32603i32 | ord @ - 32602i32 | ord @ - 32601i32 | ord @ - 32600i32 => Some(Self {
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
            Self::PARSE_ERROR => "PARSE_ERROR", Self::INVALID_REQUEST => "INVALID_REQUEST", Self::METHOD_NOT_FOUND => "METHOD_NOT_FOUND", Self::INVALID_PARAMS => "INVALID_PARAMS", Self::INTERNAL_ERROR => "INTERNAL_ERROR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ErrorCode::PARSE_ERROR, ErrorCode::INVALID_REQUEST, ErrorCode::METHOD_NOT_FOUND, ErrorCode::INVALID_PARAMS, ErrorCode::INTERNAL_ERROR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ErrorCode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PARSE_ERROR", "PARSE_ERROR", ErrorCode::PARSE_ERROR), crate::meta::inspect::EnumConstant::new("INVALID_REQUEST", "INVALID_REQUEST", ErrorCode::INVALID_REQUEST), crate::meta::inspect::EnumConstant::new("METHOD_NOT_FOUND", "METHOD_NOT_FOUND", ErrorCode::METHOD_NOT_FOUND), crate::meta::inspect::EnumConstant::new("INVALID_PARAMS", "INVALID_PARAMS", ErrorCode::INVALID_PARAMS), crate::meta::inspect::EnumConstant::new("INTERNAL_ERROR", "INTERNAL_ERROR", ErrorCode::INTERNAL_ERROR)]
        }
    }
}
impl crate::meta::GodotConvert for ErrorCode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ErrorCode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ErrorCode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::JsonRpc;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for JsonRpc {
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