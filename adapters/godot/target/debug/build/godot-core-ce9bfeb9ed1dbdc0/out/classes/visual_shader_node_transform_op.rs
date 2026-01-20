#![doc = "Sidecar module for class [`VisualShaderNodeTransformOp`][crate::classes::VisualShaderNodeTransformOp].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeTransformOp` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetransformop.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeTransformOp.`\n\nInherits [`VisualShaderNode`][crate::classes::VisualShaderNode].\n\nRelated symbols:\n\n* [`visual_shader_node_transform_op`][crate::classes::visual_shader_node_transform_op]: sidecar module with related enum/flag types\n* [`IVisualShaderNodeTransformOp`][crate::classes::IVisualShaderNodeTransformOp]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeTransformOp`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetransformop.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`VisualShaderNodeTransformOp::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeTransformOp {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`VisualShaderNodeTransformOp`][crate::classes::VisualShaderNodeTransformOp].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IVisualShaderNode`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `VisualShaderNodeTransformOp` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetransformop.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeTransformOp: crate::obj::GodotClass < Base = VisualShaderNodeTransformOp > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VisualShaderNodeTransformOp {
        pub fn set_operator(&mut self, op: crate::classes::visual_shader_node_transform_op::Operator,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_transform_op::Operator,);
            let args = (op,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10810usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTransformOp", "set_operator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_operator(&self,) -> crate::classes::visual_shader_node_transform_op::Operator {
            type CallRet = crate::classes::visual_shader_node_transform_op::Operator;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10811usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTransformOp", "get_operator", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeTransformOp {
        type Base = crate::classes::VisualShaderNode;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VisualShaderNodeTransformOp"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeTransformOp {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeTransformOp {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeTransformOp {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeTransformOp {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeTransformOp {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShaderNodeTransformOp {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShaderNodeTransformOp {
        type Target = crate::classes::VisualShaderNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeTransformOp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VisualShaderNodeTransformOp`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisualShaderNodeTransformOp__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNodeTransformOp > for $Class {
                
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
pub struct Operator {
    ord: i32
}
impl Operator {
    #[doc(alias = "OP_AxB")]
    #[doc = "Godot enumerator name: `OP_AxB`"]
    pub const AxB: Operator = Operator {
        ord: 0i32
    };
    #[doc(alias = "OP_BxA")]
    #[doc = "Godot enumerator name: `OP_BxA`"]
    pub const BxA: Operator = Operator {
        ord: 1i32
    };
    #[doc(alias = "OP_AxB_COMP")]
    #[doc = "Godot enumerator name: `OP_AxB_COMP`"]
    pub const AxB_COMP: Operator = Operator {
        ord: 2i32
    };
    #[doc(alias = "OP_BxA_COMP")]
    #[doc = "Godot enumerator name: `OP_BxA_COMP`"]
    pub const BxA_COMP: Operator = Operator {
        ord: 3i32
    };
    #[doc(alias = "OP_ADD")]
    #[doc = "Godot enumerator name: `OP_ADD`"]
    pub const ADD: Operator = Operator {
        ord: 4i32
    };
    #[doc(alias = "OP_A_MINUS_B")]
    #[doc = "Godot enumerator name: `OP_A_MINUS_B`"]
    pub const A_MINUS_B: Operator = Operator {
        ord: 5i32
    };
    #[doc(alias = "OP_B_MINUS_A")]
    #[doc = "Godot enumerator name: `OP_B_MINUS_A`"]
    pub const B_MINUS_A: Operator = Operator {
        ord: 6i32
    };
    #[doc(alias = "OP_A_DIV_B")]
    #[doc = "Godot enumerator name: `OP_A_DIV_B`"]
    pub const A_DIV_B: Operator = Operator {
        ord: 7i32
    };
    #[doc(alias = "OP_B_DIV_A")]
    #[doc = "Godot enumerator name: `OP_B_DIV_A`"]
    pub const B_DIV_A: Operator = Operator {
        ord: 8i32
    };
    #[doc(alias = "OP_MAX")]
    #[doc = "Godot enumerator name: `OP_MAX`"]
    pub const MAX: Operator = Operator {
        ord: 9i32
    };
    
}
impl std::fmt::Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Operator") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Operator {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
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
            Self::AxB => "AxB", Self::BxA => "BxA", Self::AxB_COMP => "AxB_COMP", Self::BxA_COMP => "BxA_COMP", Self::ADD => "ADD", Self::A_MINUS_B => "A_MINUS_B", Self::B_MINUS_A => "B_MINUS_A", Self::A_DIV_B => "A_DIV_B", Self::B_DIV_A => "B_DIV_A", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Operator::AxB, Operator::BxA, Operator::AxB_COMP, Operator::BxA_COMP, Operator::ADD, Operator::A_MINUS_B, Operator::B_MINUS_A, Operator::A_DIV_B, Operator::B_DIV_A]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Operator >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("AxB", "OP_AxB", Operator::AxB), crate::meta::inspect::EnumConstant::new("BxA", "OP_BxA", Operator::BxA), crate::meta::inspect::EnumConstant::new("AxB_COMP", "OP_AxB_COMP", Operator::AxB_COMP), crate::meta::inspect::EnumConstant::new("BxA_COMP", "OP_BxA_COMP", Operator::BxA_COMP), crate::meta::inspect::EnumConstant::new("ADD", "OP_ADD", Operator::ADD), crate::meta::inspect::EnumConstant::new("A_MINUS_B", "OP_A_MINUS_B", Operator::A_MINUS_B), crate::meta::inspect::EnumConstant::new("B_MINUS_A", "OP_B_MINUS_A", Operator::B_MINUS_A), crate::meta::inspect::EnumConstant::new("A_DIV_B", "OP_A_DIV_B", Operator::A_DIV_B), crate::meta::inspect::EnumConstant::new("B_DIV_A", "OP_B_DIV_A", Operator::B_DIV_A), crate::meta::inspect::EnumConstant::new("MAX", "OP_MAX", Operator::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Operator {
    const ENUMERATOR_COUNT: usize = 9usize;
    
}
impl crate::meta::GodotConvert for Operator {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Operator {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Operator {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VisualShaderNodeTransformOp;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VisualShaderNodeTransformOp {
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