#![doc = "Sidecar module for class [`VisualShaderNodeCompare`][crate::classes::VisualShaderNodeCompare].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeCompare` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodecompare.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeCompare.`\n\nInherits [`VisualShaderNode`][crate::classes::VisualShaderNode].\n\nRelated symbols:\n\n* [`visual_shader_node_compare`][crate::classes::visual_shader_node_compare]: sidecar module with related enum/flag types\n* [`IVisualShaderNodeCompare`][crate::classes::IVisualShaderNodeCompare]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeCompare`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodecompare.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`VisualShaderNodeCompare::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeCompare {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`VisualShaderNodeCompare`][crate::classes::VisualShaderNodeCompare].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IVisualShaderNode`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `VisualShaderNodeCompare` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodecompare.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeCompare: crate::obj::GodotClass < Base = VisualShaderNodeCompare > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VisualShaderNodeCompare {
        pub fn set_comparison_type(&mut self, type_: crate::classes::visual_shader_node_compare::ComparisonType,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_compare::ComparisonType,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10646usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeCompare", "set_comparison_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_comparison_type(&self,) -> crate::classes::visual_shader_node_compare::ComparisonType {
            type CallRet = crate::classes::visual_shader_node_compare::ComparisonType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10647usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeCompare", "get_comparison_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_function(&mut self, func: crate::classes::visual_shader_node_compare::Function,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_compare::Function,);
            let args = (func,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10648usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeCompare", "set_function", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_function(&self,) -> crate::classes::visual_shader_node_compare::Function {
            type CallRet = crate::classes::visual_shader_node_compare::Function;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10649usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeCompare", "get_function", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_condition(&mut self, condition: crate::classes::visual_shader_node_compare::Condition,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_compare::Condition,);
            let args = (condition,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10650usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeCompare", "set_condition", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_condition(&self,) -> crate::classes::visual_shader_node_compare::Condition {
            type CallRet = crate::classes::visual_shader_node_compare::Condition;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10651usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeCompare", "get_condition", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeCompare {
        type Base = crate::classes::VisualShaderNode;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VisualShaderNodeCompare"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeCompare {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeCompare {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeCompare {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeCompare {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeCompare {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShaderNodeCompare {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShaderNodeCompare {
        type Target = crate::classes::VisualShaderNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeCompare {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VisualShaderNodeCompare`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisualShaderNodeCompare__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNodeCompare > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNode > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ComparisonType {
    ord: i32
}
impl ComparisonType {
    #[doc(alias = "CTYPE_SCALAR")]
    #[doc = "Godot enumerator name: `CTYPE_SCALAR`"]
    pub const SCALAR: ComparisonType = ComparisonType {
        ord: 0i32
    };
    #[doc(alias = "CTYPE_SCALAR_INT")]
    #[doc = "Godot enumerator name: `CTYPE_SCALAR_INT`"]
    pub const SCALAR_INT: ComparisonType = ComparisonType {
        ord: 1i32
    };
    #[doc(alias = "CTYPE_SCALAR_UINT")]
    #[doc = "Godot enumerator name: `CTYPE_SCALAR_UINT`"]
    pub const SCALAR_UINT: ComparisonType = ComparisonType {
        ord: 2i32
    };
    #[doc(alias = "CTYPE_VECTOR_2D")]
    #[doc = "Godot enumerator name: `CTYPE_VECTOR_2D`"]
    pub const VECTOR_2D: ComparisonType = ComparisonType {
        ord: 3i32
    };
    #[doc(alias = "CTYPE_VECTOR_3D")]
    #[doc = "Godot enumerator name: `CTYPE_VECTOR_3D`"]
    pub const VECTOR_3D: ComparisonType = ComparisonType {
        ord: 4i32
    };
    #[doc(alias = "CTYPE_VECTOR_4D")]
    #[doc = "Godot enumerator name: `CTYPE_VECTOR_4D`"]
    pub const VECTOR_4D: ComparisonType = ComparisonType {
        ord: 5i32
    };
    #[doc(alias = "CTYPE_BOOLEAN")]
    #[doc = "Godot enumerator name: `CTYPE_BOOLEAN`"]
    pub const BOOLEAN: ComparisonType = ComparisonType {
        ord: 6i32
    };
    #[doc(alias = "CTYPE_TRANSFORM")]
    #[doc = "Godot enumerator name: `CTYPE_TRANSFORM`"]
    pub const TRANSFORM: ComparisonType = ComparisonType {
        ord: 7i32
    };
    #[doc(alias = "CTYPE_MAX")]
    #[doc = "Godot enumerator name: `CTYPE_MAX`"]
    pub const MAX: ComparisonType = ComparisonType {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for ComparisonType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ComparisonType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ComparisonType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::SCALAR => "SCALAR", Self::SCALAR_INT => "SCALAR_INT", Self::SCALAR_UINT => "SCALAR_UINT", Self::VECTOR_2D => "VECTOR_2D", Self::VECTOR_3D => "VECTOR_3D", Self::VECTOR_4D => "VECTOR_4D", Self::BOOLEAN => "BOOLEAN", Self::TRANSFORM => "TRANSFORM", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ComparisonType::SCALAR, ComparisonType::SCALAR_INT, ComparisonType::SCALAR_UINT, ComparisonType::VECTOR_2D, ComparisonType::VECTOR_3D, ComparisonType::VECTOR_4D, ComparisonType::BOOLEAN, ComparisonType::TRANSFORM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ComparisonType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SCALAR", "CTYPE_SCALAR", ComparisonType::SCALAR), crate::meta::inspect::EnumConstant::new("SCALAR_INT", "CTYPE_SCALAR_INT", ComparisonType::SCALAR_INT), crate::meta::inspect::EnumConstant::new("SCALAR_UINT", "CTYPE_SCALAR_UINT", ComparisonType::SCALAR_UINT), crate::meta::inspect::EnumConstant::new("VECTOR_2D", "CTYPE_VECTOR_2D", ComparisonType::VECTOR_2D), crate::meta::inspect::EnumConstant::new("VECTOR_3D", "CTYPE_VECTOR_3D", ComparisonType::VECTOR_3D), crate::meta::inspect::EnumConstant::new("VECTOR_4D", "CTYPE_VECTOR_4D", ComparisonType::VECTOR_4D), crate::meta::inspect::EnumConstant::new("BOOLEAN", "CTYPE_BOOLEAN", ComparisonType::BOOLEAN), crate::meta::inspect::EnumConstant::new("TRANSFORM", "CTYPE_TRANSFORM", ComparisonType::TRANSFORM), crate::meta::inspect::EnumConstant::new("MAX", "CTYPE_MAX", ComparisonType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ComparisonType {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for ComparisonType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ComparisonType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ComparisonType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Function {
    ord: i32
}
impl Function {
    #[doc(alias = "FUNC_EQUAL")]
    #[doc = "Godot enumerator name: `FUNC_EQUAL`"]
    pub const EQUAL: Function = Function {
        ord: 0i32
    };
    #[doc(alias = "FUNC_NOT_EQUAL")]
    #[doc = "Godot enumerator name: `FUNC_NOT_EQUAL`"]
    pub const NOT_EQUAL: Function = Function {
        ord: 1i32
    };
    #[doc(alias = "FUNC_GREATER_THAN")]
    #[doc = "Godot enumerator name: `FUNC_GREATER_THAN`"]
    pub const GREATER_THAN: Function = Function {
        ord: 2i32
    };
    #[doc(alias = "FUNC_GREATER_THAN_EQUAL")]
    #[doc = "Godot enumerator name: `FUNC_GREATER_THAN_EQUAL`"]
    pub const GREATER_THAN_EQUAL: Function = Function {
        ord: 3i32
    };
    #[doc(alias = "FUNC_LESS_THAN")]
    #[doc = "Godot enumerator name: `FUNC_LESS_THAN`"]
    pub const LESS_THAN: Function = Function {
        ord: 4i32
    };
    #[doc(alias = "FUNC_LESS_THAN_EQUAL")]
    #[doc = "Godot enumerator name: `FUNC_LESS_THAN_EQUAL`"]
    pub const LESS_THAN_EQUAL: Function = Function {
        ord: 5i32
    };
    #[doc(alias = "FUNC_MAX")]
    #[doc = "Godot enumerator name: `FUNC_MAX`"]
    pub const MAX: Function = Function {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Function") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Function {
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
            Self::EQUAL => "EQUAL", Self::NOT_EQUAL => "NOT_EQUAL", Self::GREATER_THAN => "GREATER_THAN", Self::GREATER_THAN_EQUAL => "GREATER_THAN_EQUAL", Self::LESS_THAN => "LESS_THAN", Self::LESS_THAN_EQUAL => "LESS_THAN_EQUAL", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Function::EQUAL, Function::NOT_EQUAL, Function::GREATER_THAN, Function::GREATER_THAN_EQUAL, Function::LESS_THAN, Function::LESS_THAN_EQUAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Function >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("EQUAL", "FUNC_EQUAL", Function::EQUAL), crate::meta::inspect::EnumConstant::new("NOT_EQUAL", "FUNC_NOT_EQUAL", Function::NOT_EQUAL), crate::meta::inspect::EnumConstant::new("GREATER_THAN", "FUNC_GREATER_THAN", Function::GREATER_THAN), crate::meta::inspect::EnumConstant::new("GREATER_THAN_EQUAL", "FUNC_GREATER_THAN_EQUAL", Function::GREATER_THAN_EQUAL), crate::meta::inspect::EnumConstant::new("LESS_THAN", "FUNC_LESS_THAN", Function::LESS_THAN), crate::meta::inspect::EnumConstant::new("LESS_THAN_EQUAL", "FUNC_LESS_THAN_EQUAL", Function::LESS_THAN_EQUAL), crate::meta::inspect::EnumConstant::new("MAX", "FUNC_MAX", Function::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Function {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for Function {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Function {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Function {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Condition {
    ord: i32
}
impl Condition {
    #[doc(alias = "COND_ALL")]
    #[doc = "Godot enumerator name: `COND_ALL`"]
    pub const ALL: Condition = Condition {
        ord: 0i32
    };
    #[doc(alias = "COND_ANY")]
    #[doc = "Godot enumerator name: `COND_ANY`"]
    pub const ANY: Condition = Condition {
        ord: 1i32
    };
    #[doc(alias = "COND_MAX")]
    #[doc = "Godot enumerator name: `COND_MAX`"]
    pub const MAX: Condition = Condition {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Condition") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Condition {
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
            Self::ALL => "ALL", Self::ANY => "ANY", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Condition::ALL, Condition::ANY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Condition >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ALL", "COND_ALL", Condition::ALL), crate::meta::inspect::EnumConstant::new("ANY", "COND_ANY", Condition::ANY), crate::meta::inspect::EnumConstant::new("MAX", "COND_MAX", Condition::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Condition {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for Condition {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Condition {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Condition {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VisualShaderNodeCompare;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VisualShaderNodeCompare {
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