#![doc = "Sidecar module for class [`XmlParser`][crate::classes::XmlParser].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `XMLParser` enums](https://docs.godotengine.org/en/stable/classes/class_xmlparser.html#enumerations).\n\n"]
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
    #[doc = "Godot class `XMLParser.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`xml_parser`][crate::classes::xml_parser]: sidecar module with related enum/flag types\n* [`IXmlParser`][crate::classes::IXmlParser]: virtual methods\n\n\nSee also [Godot docs for `XMLParser`](https://docs.godotengine.org/en/stable/classes/class_xmlparser.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`XmlParser::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct XmlParser {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`XmlParser`][crate::classes::XmlParser].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `XMLParser` methods](https://docs.godotengine.org/en/stable/classes/class_xmlparser.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IXmlParser: crate::obj::GodotClass < Base = XmlParser > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl XmlParser {
        pub fn read(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "read", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_type(&mut self,) -> crate::classes::xml_parser::NodeType {
            type CallRet = crate::classes::xml_parser::NodeType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "get_node_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "get_node_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_data(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "get_node_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_offset(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "get_node_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_attribute_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "get_attribute_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_attribute_name(&self, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "get_attribute_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_attribute_value(&self, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "get_attribute_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_attribute(&self, name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "has_attribute", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_named_attribute_value(&self, name: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "get_named_attribute_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_named_attribute_value_safe(&self, name: impl AsArg < GString >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "get_named_attribute_value_safe", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_empty(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "is_empty", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_line(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "get_current_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn skip_section(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "skip_section", Some(self.__validated_obj()), args,)
            }
        }
        pub fn seek(&mut self, position: u64,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (u64,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "seek", Some(self.__validated_obj()), args,)
            }
        }
        pub fn open(&mut self, file: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "open", Some(self.__validated_obj()), args,)
            }
        }
        pub fn open_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "XmlParser", "open_buffer", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for XmlParser {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("XMLParser"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for XmlParser {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for XmlParser {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for XmlParser {
        
    }
    impl crate::obj::cap::GodotDefault for XmlParser {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for XmlParser {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for XmlParser {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`XmlParser`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_XmlParser__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::XmlParser > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct NodeType {
    ord: i32
}
impl NodeType {
    #[doc(alias = "NODE_NONE")]
    #[doc = "Godot enumerator name: `NODE_NONE`"]
    pub const NONE: NodeType = NodeType {
        ord: 0i32
    };
    #[doc(alias = "NODE_ELEMENT")]
    #[doc = "Godot enumerator name: `NODE_ELEMENT`"]
    pub const ELEMENT: NodeType = NodeType {
        ord: 1i32
    };
    #[doc(alias = "NODE_ELEMENT_END")]
    #[doc = "Godot enumerator name: `NODE_ELEMENT_END`"]
    pub const ELEMENT_END: NodeType = NodeType {
        ord: 2i32
    };
    #[doc(alias = "NODE_TEXT")]
    #[doc = "Godot enumerator name: `NODE_TEXT`"]
    pub const TEXT: NodeType = NodeType {
        ord: 3i32
    };
    #[doc(alias = "NODE_COMMENT")]
    #[doc = "Godot enumerator name: `NODE_COMMENT`"]
    pub const COMMENT: NodeType = NodeType {
        ord: 4i32
    };
    #[doc(alias = "NODE_CDATA")]
    #[doc = "Godot enumerator name: `NODE_CDATA`"]
    pub const CDATA: NodeType = NodeType {
        ord: 5i32
    };
    #[doc(alias = "NODE_UNKNOWN")]
    #[doc = "Godot enumerator name: `NODE_UNKNOWN`"]
    pub const UNKNOWN: NodeType = NodeType {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("NodeType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for NodeType {
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
            Self::NONE => "NONE", Self::ELEMENT => "ELEMENT", Self::ELEMENT_END => "ELEMENT_END", Self::TEXT => "TEXT", Self::COMMENT => "COMMENT", Self::CDATA => "CDATA", Self::UNKNOWN => "UNKNOWN", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[NodeType::NONE, NodeType::ELEMENT, NodeType::ELEMENT_END, NodeType::TEXT, NodeType::COMMENT, NodeType::CDATA, NodeType::UNKNOWN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < NodeType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "NODE_NONE", NodeType::NONE), crate::meta::inspect::EnumConstant::new("ELEMENT", "NODE_ELEMENT", NodeType::ELEMENT), crate::meta::inspect::EnumConstant::new("ELEMENT_END", "NODE_ELEMENT_END", NodeType::ELEMENT_END), crate::meta::inspect::EnumConstant::new("TEXT", "NODE_TEXT", NodeType::TEXT), crate::meta::inspect::EnumConstant::new("COMMENT", "NODE_COMMENT", NodeType::COMMENT), crate::meta::inspect::EnumConstant::new("CDATA", "NODE_CDATA", NodeType::CDATA), crate::meta::inspect::EnumConstant::new("UNKNOWN", "NODE_UNKNOWN", NodeType::UNKNOWN)]
        }
    }
}
impl crate::meta::GodotConvert for NodeType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for NodeType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for NodeType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::XmlParser;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for XmlParser {
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