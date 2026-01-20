#![doc = "Sidecar module for class [`VisualShaderNode`][crate::classes::VisualShaderNode].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNode` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernode.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNode.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`visual_shader_node`][crate::classes::visual_shader_node]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `VisualShaderNode`](https://docs.godotengine.org/en/stable/classes/class_visualshadernode.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<VisualShaderNode>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNode {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl VisualShaderNode {
        pub fn get_default_input_port(&self, type_: crate::classes::visual_shader_node::PortType,) -> i32 {
            type CallRet = i32;
            type CallParams = (crate::classes::visual_shader_node::PortType,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10613usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNode", "get_default_input_port", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_output_port_for_preview(&mut self, port: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10614usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNode", "set_output_port_for_preview", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_output_port_for_preview(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10615usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNode", "get_output_port_for_preview", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn set_input_port_default_value_full(&mut self, port: i32, value: RefArg < Variant >, prev_value: RefArg < Variant >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (i32, RefArg < 'a0, Variant >, RefArg < 'a1, Variant >,);
            let args = (port, value, prev_value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10616usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNode", "set_input_port_default_value", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_input_port_default_value_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_input_port_default_value(&mut self, port: i32, value: &Variant,) {
            self.set_input_port_default_value_ex(port, value,) . done()
        }
        #[inline]
        pub fn set_input_port_default_value_ex < 'a > (&'a mut self, port: i32, value: &'a Variant,) -> ExSetInputPortDefaultValue < 'a > {
            ExSetInputPortDefaultValue::new(self, port, value,)
        }
        pub fn get_input_port_default_value(&self, port: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10617usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNode", "get_input_port_default_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_input_port_default_value(&mut self, port: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10618usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNode", "remove_input_port_default_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_default_input_values(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10619usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNode", "clear_default_input_values", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_default_input_values(&mut self, values: &VarArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
            let args = (RefArg::new(values),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10620usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNode", "set_default_input_values", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_default_input_values(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10621usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNode", "get_default_input_values", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_frame(&mut self, frame: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10622usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNode", "set_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frame(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10623usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNode", "get_frame", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for VisualShaderNode {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VisualShaderNode"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNode {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNode {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNode {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNode {
        
    }
    impl std::ops::Deref for VisualShaderNode {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNode {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisualShaderNode__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `VisualShaderNode` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`VisualShaderNode::set_input_port_default_value_ex`][super::VisualShaderNode::set_input_port_default_value_ex]."]
#[must_use]
pub struct ExSetInputPortDefaultValue < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::VisualShaderNode, port: i32, value: CowArg < 'a, Variant >, prev_value: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetInputPortDefaultValue < 'a > {
    fn new(surround_object: &'a mut re_export::VisualShaderNode, port: i32, value: &'a Variant,) -> Self {
        let prev_value = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, port: port, value: CowArg::Borrowed(value), prev_value: CowArg::Owned(prev_value),
        }
    }
    #[inline]
    pub fn prev_value(self, prev_value: &'a Variant) -> Self {
        Self {
            prev_value: CowArg::Borrowed(prev_value), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, port, value, prev_value,
        }
        = self;
        re_export::VisualShaderNode::set_input_port_default_value_full(surround_object, port, value.cow_as_arg(), prev_value.cow_as_arg(),)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PortType {
    ord: i32
}
impl PortType {
    #[doc(alias = "PORT_TYPE_SCALAR")]
    #[doc = "Godot enumerator name: `PORT_TYPE_SCALAR`"]
    pub const SCALAR: PortType = PortType {
        ord: 0i32
    };
    #[doc(alias = "PORT_TYPE_SCALAR_INT")]
    #[doc = "Godot enumerator name: `PORT_TYPE_SCALAR_INT`"]
    pub const SCALAR_INT: PortType = PortType {
        ord: 1i32
    };
    #[doc(alias = "PORT_TYPE_SCALAR_UINT")]
    #[doc = "Godot enumerator name: `PORT_TYPE_SCALAR_UINT`"]
    pub const SCALAR_UINT: PortType = PortType {
        ord: 2i32
    };
    #[doc(alias = "PORT_TYPE_VECTOR_2D")]
    #[doc = "Godot enumerator name: `PORT_TYPE_VECTOR_2D`"]
    pub const VECTOR_2D: PortType = PortType {
        ord: 3i32
    };
    #[doc(alias = "PORT_TYPE_VECTOR_3D")]
    #[doc = "Godot enumerator name: `PORT_TYPE_VECTOR_3D`"]
    pub const VECTOR_3D: PortType = PortType {
        ord: 4i32
    };
    #[doc(alias = "PORT_TYPE_VECTOR_4D")]
    #[doc = "Godot enumerator name: `PORT_TYPE_VECTOR_4D`"]
    pub const VECTOR_4D: PortType = PortType {
        ord: 5i32
    };
    #[doc(alias = "PORT_TYPE_BOOLEAN")]
    #[doc = "Godot enumerator name: `PORT_TYPE_BOOLEAN`"]
    pub const BOOLEAN: PortType = PortType {
        ord: 6i32
    };
    #[doc(alias = "PORT_TYPE_TRANSFORM")]
    #[doc = "Godot enumerator name: `PORT_TYPE_TRANSFORM`"]
    pub const TRANSFORM: PortType = PortType {
        ord: 7i32
    };
    #[doc(alias = "PORT_TYPE_SAMPLER")]
    #[doc = "Godot enumerator name: `PORT_TYPE_SAMPLER`"]
    pub const SAMPLER: PortType = PortType {
        ord: 8i32
    };
    #[doc(alias = "PORT_TYPE_MAX")]
    #[doc = "Godot enumerator name: `PORT_TYPE_MAX`"]
    pub const MAX: PortType = PortType {
        ord: 9i32
    };
    
}
impl std::fmt::Debug for PortType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PortType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PortType {
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
            Self::SCALAR => "SCALAR", Self::SCALAR_INT => "SCALAR_INT", Self::SCALAR_UINT => "SCALAR_UINT", Self::VECTOR_2D => "VECTOR_2D", Self::VECTOR_3D => "VECTOR_3D", Self::VECTOR_4D => "VECTOR_4D", Self::BOOLEAN => "BOOLEAN", Self::TRANSFORM => "TRANSFORM", Self::SAMPLER => "SAMPLER", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PortType::SCALAR, PortType::SCALAR_INT, PortType::SCALAR_UINT, PortType::VECTOR_2D, PortType::VECTOR_3D, PortType::VECTOR_4D, PortType::BOOLEAN, PortType::TRANSFORM, PortType::SAMPLER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PortType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SCALAR", "PORT_TYPE_SCALAR", PortType::SCALAR), crate::meta::inspect::EnumConstant::new("SCALAR_INT", "PORT_TYPE_SCALAR_INT", PortType::SCALAR_INT), crate::meta::inspect::EnumConstant::new("SCALAR_UINT", "PORT_TYPE_SCALAR_UINT", PortType::SCALAR_UINT), crate::meta::inspect::EnumConstant::new("VECTOR_2D", "PORT_TYPE_VECTOR_2D", PortType::VECTOR_2D), crate::meta::inspect::EnumConstant::new("VECTOR_3D", "PORT_TYPE_VECTOR_3D", PortType::VECTOR_3D), crate::meta::inspect::EnumConstant::new("VECTOR_4D", "PORT_TYPE_VECTOR_4D", PortType::VECTOR_4D), crate::meta::inspect::EnumConstant::new("BOOLEAN", "PORT_TYPE_BOOLEAN", PortType::BOOLEAN), crate::meta::inspect::EnumConstant::new("TRANSFORM", "PORT_TYPE_TRANSFORM", PortType::TRANSFORM), crate::meta::inspect::EnumConstant::new("SAMPLER", "PORT_TYPE_SAMPLER", PortType::SAMPLER), crate::meta::inspect::EnumConstant::new("MAX", "PORT_TYPE_MAX", PortType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for PortType {
    const ENUMERATOR_COUNT: usize = 9usize;
    
}
impl crate::meta::GodotConvert for PortType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PortType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PortType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VisualShaderNode;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VisualShaderNode {
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