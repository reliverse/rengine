#![doc = "Sidecar module for class [`OpenXrCompositionLayer`][crate::classes::OpenXrCompositionLayer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRCompositionLayer` enums](https://docs.godotengine.org/en/stable/classes/class_openxrcompositionlayer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRCompositionLayer.`\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`open_xr_composition_layer`][crate::classes::open_xr_composition_layer]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `OpenXRCompositionLayer`](https://docs.godotengine.org/en/stable/classes/class_openxrcompositionlayer.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<OpenXrCompositionLayer>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrCompositionLayer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl OpenXrCompositionLayer {
        pub fn set_layer_viewport(&mut self, viewport: impl AsArg < Option < Gd < crate::classes::SubViewport >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::SubViewport > > >,);
            let args = (viewport.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5984usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_layer_viewport", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_layer_viewport(&self,) -> Option < Gd < crate::classes::SubViewport > > {
            type CallRet = Option < Gd < crate::classes::SubViewport > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5985usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_layer_viewport", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_android_surface(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5986usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_use_android_surface", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_use_android_surface(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5987usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_use_android_surface", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_android_surface_size(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5988usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_android_surface_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_android_surface_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5989usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_android_surface_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enable_hole_punch(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5990usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_enable_hole_punch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_enable_hole_punch(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5991usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_enable_hole_punch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sort_order(&mut self, order: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5992usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_sort_order", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sort_order(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5993usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_sort_order", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_blend(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5994usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_alpha_blend", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_blend(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5995usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_alpha_blend", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_natively_supported(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5996usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "is_natively_supported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_min_filter(&mut self, mode: crate::classes::open_xr_composition_layer::Filter,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_composition_layer::Filter,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5997usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_min_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_min_filter(&self,) -> crate::classes::open_xr_composition_layer::Filter {
            type CallRet = crate::classes::open_xr_composition_layer::Filter;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5998usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_min_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mag_filter(&mut self, mode: crate::classes::open_xr_composition_layer::Filter,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_composition_layer::Filter,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5999usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_mag_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mag_filter(&self,) -> crate::classes::open_xr_composition_layer::Filter {
            type CallRet = crate::classes::open_xr_composition_layer::Filter;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6000usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_mag_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mipmap_mode(&mut self, mode: crate::classes::open_xr_composition_layer::MipmapMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_composition_layer::MipmapMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6001usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_mipmap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mipmap_mode(&self,) -> crate::classes::open_xr_composition_layer::MipmapMode {
            type CallRet = crate::classes::open_xr_composition_layer::MipmapMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6002usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_mipmap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_horizontal_wrap(&mut self, mode: crate::classes::open_xr_composition_layer::Wrap,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_composition_layer::Wrap,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6003usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_horizontal_wrap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_horizontal_wrap(&self,) -> crate::classes::open_xr_composition_layer::Wrap {
            type CallRet = crate::classes::open_xr_composition_layer::Wrap;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6004usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_horizontal_wrap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertical_wrap(&mut self, mode: crate::classes::open_xr_composition_layer::Wrap,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_composition_layer::Wrap,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6005usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_vertical_wrap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertical_wrap(&self,) -> crate::classes::open_xr_composition_layer::Wrap {
            type CallRet = crate::classes::open_xr_composition_layer::Wrap;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6006usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_vertical_wrap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_red_swizzle(&mut self, mode: crate::classes::open_xr_composition_layer::Swizzle,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_composition_layer::Swizzle,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6007usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_red_swizzle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_red_swizzle(&self,) -> crate::classes::open_xr_composition_layer::Swizzle {
            type CallRet = crate::classes::open_xr_composition_layer::Swizzle;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6008usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_red_swizzle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_green_swizzle(&mut self, mode: crate::classes::open_xr_composition_layer::Swizzle,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_composition_layer::Swizzle,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6009usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_green_swizzle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_green_swizzle(&self,) -> crate::classes::open_xr_composition_layer::Swizzle {
            type CallRet = crate::classes::open_xr_composition_layer::Swizzle;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6010usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_green_swizzle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_blue_swizzle(&mut self, mode: crate::classes::open_xr_composition_layer::Swizzle,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_composition_layer::Swizzle,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6011usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_blue_swizzle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_blue_swizzle(&self,) -> crate::classes::open_xr_composition_layer::Swizzle {
            type CallRet = crate::classes::open_xr_composition_layer::Swizzle;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6012usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_blue_swizzle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_swizzle(&mut self, mode: crate::classes::open_xr_composition_layer::Swizzle,) {
            type CallRet = ();
            type CallParams = (crate::classes::open_xr_composition_layer::Swizzle,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6013usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_alpha_swizzle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_swizzle(&self,) -> crate::classes::open_xr_composition_layer::Swizzle {
            type CallRet = crate::classes::open_xr_composition_layer::Swizzle;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6014usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_alpha_swizzle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_anisotropy(&mut self, value: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6015usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_max_anisotropy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_anisotropy(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6016usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_max_anisotropy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_border_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6017usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "set_border_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_border_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6018usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "get_border_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn intersects_ray(&self, origin: Vector3, direction: Vector3,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector3, Vector3,);
            let args = (origin, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6019usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrCompositionLayer", "intersects_ray", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for OpenXrCompositionLayer {
        type Base = crate::classes::Node3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OpenXRCompositionLayer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrCompositionLayer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for OpenXrCompositionLayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for OpenXrCompositionLayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrCompositionLayer {
        
    }
    impl std::ops::Deref for OpenXrCompositionLayer {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrCompositionLayer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrCompositionLayer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `OpenXrCompositionLayer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Filter {
    ord: i32
}
impl Filter {
    #[doc(alias = "FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `FILTER_NEAREST`"]
    pub const NEAREST: Filter = Filter {
        ord: 0i32
    };
    #[doc(alias = "FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `FILTER_LINEAR`"]
    pub const LINEAR: Filter = Filter {
        ord: 1i32
    };
    #[doc(alias = "FILTER_CUBIC")]
    #[doc = "Godot enumerator name: `FILTER_CUBIC`"]
    pub const CUBIC: Filter = Filter {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Filter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Filter {
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
            Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", Self::CUBIC => "CUBIC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Filter::NEAREST, Filter::LINEAR, Filter::CUBIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Filter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEAREST", "FILTER_NEAREST", Filter::NEAREST), crate::meta::inspect::EnumConstant::new("LINEAR", "FILTER_LINEAR", Filter::LINEAR), crate::meta::inspect::EnumConstant::new("CUBIC", "FILTER_CUBIC", Filter::CUBIC)]
        }
    }
}
impl crate::meta::GodotConvert for Filter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Filter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Filter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MipmapMode {
    ord: i32
}
impl MipmapMode {
    #[doc(alias = "MIPMAP_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `MIPMAP_MODE_DISABLED`"]
    pub const DISABLED: MipmapMode = MipmapMode {
        ord: 0i32
    };
    #[doc(alias = "MIPMAP_MODE_NEAREST")]
    #[doc = "Godot enumerator name: `MIPMAP_MODE_NEAREST`"]
    pub const NEAREST: MipmapMode = MipmapMode {
        ord: 1i32
    };
    #[doc(alias = "MIPMAP_MODE_LINEAR")]
    #[doc = "Godot enumerator name: `MIPMAP_MODE_LINEAR`"]
    pub const LINEAR: MipmapMode = MipmapMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for MipmapMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MipmapMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MipmapMode {
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
            Self::DISABLED => "DISABLED", Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MipmapMode::DISABLED, MipmapMode::NEAREST, MipmapMode::LINEAR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MipmapMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "MIPMAP_MODE_DISABLED", MipmapMode::DISABLED), crate::meta::inspect::EnumConstant::new("NEAREST", "MIPMAP_MODE_NEAREST", MipmapMode::NEAREST), crate::meta::inspect::EnumConstant::new("LINEAR", "MIPMAP_MODE_LINEAR", MipmapMode::LINEAR)]
        }
    }
}
impl crate::meta::GodotConvert for MipmapMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MipmapMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MipmapMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Wrap {
    ord: i32
}
impl Wrap {
    #[doc(alias = "WRAP_CLAMP_TO_BORDER")]
    #[doc = "Godot enumerator name: `WRAP_CLAMP_TO_BORDER`"]
    pub const CLAMP_TO_BORDER: Wrap = Wrap {
        ord: 0i32
    };
    #[doc(alias = "WRAP_CLAMP_TO_EDGE")]
    #[doc = "Godot enumerator name: `WRAP_CLAMP_TO_EDGE`"]
    pub const CLAMP_TO_EDGE: Wrap = Wrap {
        ord: 1i32
    };
    #[doc(alias = "WRAP_REPEAT")]
    #[doc = "Godot enumerator name: `WRAP_REPEAT`"]
    pub const REPEAT: Wrap = Wrap {
        ord: 2i32
    };
    #[doc(alias = "WRAP_MIRRORED_REPEAT")]
    #[doc = "Godot enumerator name: `WRAP_MIRRORED_REPEAT`"]
    pub const MIRRORED_REPEAT: Wrap = Wrap {
        ord: 3i32
    };
    #[doc(alias = "WRAP_MIRROR_CLAMP_TO_EDGE")]
    #[doc = "Godot enumerator name: `WRAP_MIRROR_CLAMP_TO_EDGE`"]
    pub const MIRROR_CLAMP_TO_EDGE: Wrap = Wrap {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for Wrap {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Wrap") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Wrap {
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
            Self::CLAMP_TO_BORDER => "CLAMP_TO_BORDER", Self::CLAMP_TO_EDGE => "CLAMP_TO_EDGE", Self::REPEAT => "REPEAT", Self::MIRRORED_REPEAT => "MIRRORED_REPEAT", Self::MIRROR_CLAMP_TO_EDGE => "MIRROR_CLAMP_TO_EDGE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Wrap::CLAMP_TO_BORDER, Wrap::CLAMP_TO_EDGE, Wrap::REPEAT, Wrap::MIRRORED_REPEAT, Wrap::MIRROR_CLAMP_TO_EDGE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Wrap >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CLAMP_TO_BORDER", "WRAP_CLAMP_TO_BORDER", Wrap::CLAMP_TO_BORDER), crate::meta::inspect::EnumConstant::new("CLAMP_TO_EDGE", "WRAP_CLAMP_TO_EDGE", Wrap::CLAMP_TO_EDGE), crate::meta::inspect::EnumConstant::new("REPEAT", "WRAP_REPEAT", Wrap::REPEAT), crate::meta::inspect::EnumConstant::new("MIRRORED_REPEAT", "WRAP_MIRRORED_REPEAT", Wrap::MIRRORED_REPEAT), crate::meta::inspect::EnumConstant::new("MIRROR_CLAMP_TO_EDGE", "WRAP_MIRROR_CLAMP_TO_EDGE", Wrap::MIRROR_CLAMP_TO_EDGE)]
        }
    }
}
impl crate::meta::GodotConvert for Wrap {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Wrap {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Wrap {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Swizzle {
    ord: i32
}
impl Swizzle {
    #[doc(alias = "SWIZZLE_RED")]
    #[doc = "Godot enumerator name: `SWIZZLE_RED`"]
    pub const RED: Swizzle = Swizzle {
        ord: 0i32
    };
    #[doc(alias = "SWIZZLE_GREEN")]
    #[doc = "Godot enumerator name: `SWIZZLE_GREEN`"]
    pub const GREEN: Swizzle = Swizzle {
        ord: 1i32
    };
    #[doc(alias = "SWIZZLE_BLUE")]
    #[doc = "Godot enumerator name: `SWIZZLE_BLUE`"]
    pub const BLUE: Swizzle = Swizzle {
        ord: 2i32
    };
    #[doc(alias = "SWIZZLE_ALPHA")]
    #[doc = "Godot enumerator name: `SWIZZLE_ALPHA`"]
    pub const ALPHA: Swizzle = Swizzle {
        ord: 3i32
    };
    #[doc(alias = "SWIZZLE_ZERO")]
    #[doc = "Godot enumerator name: `SWIZZLE_ZERO`"]
    pub const ZERO: Swizzle = Swizzle {
        ord: 4i32
    };
    #[doc(alias = "SWIZZLE_ONE")]
    #[doc = "Godot enumerator name: `SWIZZLE_ONE`"]
    pub const ONE: Swizzle = Swizzle {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for Swizzle {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Swizzle") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Swizzle {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::RED => "RED", Self::GREEN => "GREEN", Self::BLUE => "BLUE", Self::ALPHA => "ALPHA", Self::ZERO => "ZERO", Self::ONE => "ONE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Swizzle::RED, Swizzle::GREEN, Swizzle::BLUE, Swizzle::ALPHA, Swizzle::ZERO, Swizzle::ONE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Swizzle >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RED", "SWIZZLE_RED", Swizzle::RED), crate::meta::inspect::EnumConstant::new("GREEN", "SWIZZLE_GREEN", Swizzle::GREEN), crate::meta::inspect::EnumConstant::new("BLUE", "SWIZZLE_BLUE", Swizzle::BLUE), crate::meta::inspect::EnumConstant::new("ALPHA", "SWIZZLE_ALPHA", Swizzle::ALPHA), crate::meta::inspect::EnumConstant::new("ZERO", "SWIZZLE_ZERO", Swizzle::ZERO), crate::meta::inspect::EnumConstant::new("ONE", "SWIZZLE_ONE", Swizzle::ONE)]
        }
    }
}
impl crate::meta::GodotConvert for Swizzle {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Swizzle {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Swizzle {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OpenXrCompositionLayer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for OpenXrCompositionLayer {
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