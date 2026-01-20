#![doc = "Sidecar module for class [`CsgShape3D`][crate::classes::CsgShape3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CSGShape3D` enums](https://docs.godotengine.org/en/stable/classes/class_csgshape3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CSGShape3D.`\n\nInherits [`GeometryInstance3D`][crate::classes::GeometryInstance3D].\n\nRelated symbols:\n\n* [`csg_shape_3d`][crate::classes::csg_shape_3d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `CSGShape3D`](https://docs.godotengine.org/en/stable/classes/class_csgshape3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<CsgShape3D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CsgShape3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl CsgShape3D {
        pub fn is_root_shape(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1600usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "is_root_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_operation(&mut self, operation: crate::classes::csg_shape_3d::Operation,) {
            type CallRet = ();
            type CallParams = (crate::classes::csg_shape_3d::Operation,);
            let args = (operation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1601usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_operation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_operation(&self,) -> crate::classes::csg_shape_3d::Operation {
            type CallRet = crate::classes::csg_shape_3d::Operation;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1602usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "get_operation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_snap(&mut self, snap: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (snap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1603usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_snap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_snap(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1604usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "get_snap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_collision(&mut self, operation: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (operation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1605usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_use_collision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_collision(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1606usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "is_using_collision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_layer(&mut self, layer: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1607usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_layer(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1608usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "get_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_mask(&mut self, mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1609usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1610usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "get_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_mask_value(&mut self, layer_number: i32, value: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1611usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_collision_mask_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_mask_value(&self, layer_number: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1612usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "get_collision_mask_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_layer_value(&mut self, layer_number: i32, value: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1613usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_collision_layer_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_layer_value(&self, layer_number: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1614usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "get_collision_layer_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_priority(&mut self, priority: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1615usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_collision_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_priority(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1616usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "get_collision_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn bake_collision_shape(&mut self,) -> Option < Gd < crate::classes::ConcavePolygonShape3D > > {
            type CallRet = Option < Gd < crate::classes::ConcavePolygonShape3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1617usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "bake_collision_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_calculate_tangents(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1618usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "set_calculate_tangents", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_calculating_tangents(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1619usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "is_calculating_tangents", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_meshes(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1620usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "get_meshes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn bake_static_mesh(&mut self,) -> Option < Gd < crate::classes::ArrayMesh > > {
            type CallRet = Option < Gd < crate::classes::ArrayMesh > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1621usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgShape3D", "bake_static_mesh", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CsgShape3D {
        type Base = crate::classes::GeometryInstance3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CSGShape3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CsgShape3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for CsgShape3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for CsgShape3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for CsgShape3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CsgShape3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CsgShape3D {
        
    }
    impl std::ops::Deref for CsgShape3D {
        type Target = crate::classes::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CsgShape3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CsgShape3D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `CsgShape3D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Operation {
    ord: i32
}
impl Operation {
    #[doc(alias = "OPERATION_UNION")]
    #[doc = "Godot enumerator name: `OPERATION_UNION`"]
    pub const UNION: Operation = Operation {
        ord: 0i32
    };
    #[doc(alias = "OPERATION_INTERSECTION")]
    #[doc = "Godot enumerator name: `OPERATION_INTERSECTION`"]
    pub const INTERSECTION: Operation = Operation {
        ord: 1i32
    };
    #[doc(alias = "OPERATION_SUBTRACTION")]
    #[doc = "Godot enumerator name: `OPERATION_SUBTRACTION`"]
    pub const SUBTRACTION: Operation = Operation {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Operation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Operation {
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
            Self::UNION => "UNION", Self::INTERSECTION => "INTERSECTION", Self::SUBTRACTION => "SUBTRACTION", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Operation::UNION, Operation::INTERSECTION, Operation::SUBTRACTION]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Operation >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNION", "OPERATION_UNION", Operation::UNION), crate::meta::inspect::EnumConstant::new("INTERSECTION", "OPERATION_INTERSECTION", Operation::INTERSECTION), crate::meta::inspect::EnumConstant::new("SUBTRACTION", "OPERATION_SUBTRACTION", Operation::SUBTRACTION)]
        }
    }
}
impl crate::meta::GodotConvert for Operation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Operation {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Operation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CsgShape3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for CsgShape3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}