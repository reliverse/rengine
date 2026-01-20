#![doc = "Sidecar module for class [`Light2D`][crate::classes::Light2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Light2D` enums](https://docs.godotengine.org/en/stable/classes/class_light2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Light2D.`\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`light_2d`][crate::classes::light_2d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `Light2D`](https://docs.godotengine.org/en/stable/classes/class_light2d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Light2D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Light2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Light2D {
        pub fn set_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4891usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "set_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4892usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "is_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_editor_only(&mut self, editor_only: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (editor_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4893usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "set_editor_only", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_editor_only(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4894usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "is_editor_only", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4895usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "set_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4896usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "get_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_energy(&mut self, energy: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4897usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "set_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_energy(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4898usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "get_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_z_range_min(&mut self, z: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (z,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4899usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "set_z_range_min", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_z_range_min(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4900usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "get_z_range_min", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_z_range_max(&mut self, z: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (z,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4901usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "set_z_range_max", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_z_range_max(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4902usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "get_z_range_max", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_layer_range_min(&mut self, layer: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4903usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "set_layer_range_min", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_layer_range_min(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4904usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "get_layer_range_min", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_layer_range_max(&mut self, layer: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4905usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "set_layer_range_max", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_layer_range_max(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4906usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "get_layer_range_max", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_cull_mask(&mut self, item_cull_mask: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (item_cull_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4907usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "set_item_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_cull_mask(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4908usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "get_item_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_shadow_cull_mask(&mut self, item_shadow_cull_mask: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (item_shadow_cull_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4909usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "set_item_shadow_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_shadow_cull_mask(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4910usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "get_item_shadow_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4911usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "set_shadow_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_shadow_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4912usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "is_shadow_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_smooth(&mut self, smooth: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (smooth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4913usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "set_shadow_smooth", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadow_smooth(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4914usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "get_shadow_smooth", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_filter(&mut self, filter: crate::classes::light_2d::ShadowFilter,) {
            type CallRet = ();
            type CallParams = (crate::classes::light_2d::ShadowFilter,);
            let args = (filter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4915usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "set_shadow_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadow_filter(&self,) -> crate::classes::light_2d::ShadowFilter {
            type CallRet = crate::classes::light_2d::ShadowFilter;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4916usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "get_shadow_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_color(&mut self, shadow_color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (shadow_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4917usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "set_shadow_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadow_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4918usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "get_shadow_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_blend_mode(&mut self, mode: crate::classes::light_2d::BlendMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::light_2d::BlendMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4919usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "set_blend_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_blend_mode(&self,) -> crate::classes::light_2d::BlendMode {
            type CallRet = crate::classes::light_2d::BlendMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4920usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "get_blend_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_height(&mut self, height: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4921usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "set_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_height(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4922usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light2D", "get_height", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Light2D {
        type Base = crate::classes::Node2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Light2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Light2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for Light2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Light2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Light2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Light2D {
        
    }
    impl std::ops::Deref for Light2D {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Light2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Light2D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Light2D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShadowFilter {
    ord: i32
}
impl ShadowFilter {
    #[doc(alias = "SHADOW_FILTER_NONE")]
    #[doc = "Godot enumerator name: `SHADOW_FILTER_NONE`"]
    pub const NONE: ShadowFilter = ShadowFilter {
        ord: 0i32
    };
    #[doc(alias = "SHADOW_FILTER_PCF5")]
    #[doc = "Godot enumerator name: `SHADOW_FILTER_PCF5`"]
    pub const PCF5: ShadowFilter = ShadowFilter {
        ord: 1i32
    };
    #[doc(alias = "SHADOW_FILTER_PCF13")]
    #[doc = "Godot enumerator name: `SHADOW_FILTER_PCF13`"]
    pub const PCF13: ShadowFilter = ShadowFilter {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ShadowFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShadowFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShadowFilter {
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
            Self::NONE => "NONE", Self::PCF5 => "PCF5", Self::PCF13 => "PCF13", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ShadowFilter::NONE, ShadowFilter::PCF5, ShadowFilter::PCF13]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ShadowFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "SHADOW_FILTER_NONE", ShadowFilter::NONE), crate::meta::inspect::EnumConstant::new("PCF5", "SHADOW_FILTER_PCF5", ShadowFilter::PCF5), crate::meta::inspect::EnumConstant::new("PCF13", "SHADOW_FILTER_PCF13", ShadowFilter::PCF13)]
        }
    }
}
impl crate::meta::GodotConvert for ShadowFilter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ShadowFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShadowFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlendMode {
    ord: i32
}
impl BlendMode {
    #[doc(alias = "BLEND_MODE_ADD")]
    #[doc = "Godot enumerator name: `BLEND_MODE_ADD`"]
    pub const ADD: BlendMode = BlendMode {
        ord: 0i32
    };
    #[doc(alias = "BLEND_MODE_SUB")]
    #[doc = "Godot enumerator name: `BLEND_MODE_SUB`"]
    pub const SUB: BlendMode = BlendMode {
        ord: 1i32
    };
    #[doc(alias = "BLEND_MODE_MIX")]
    #[doc = "Godot enumerator name: `BLEND_MODE_MIX`"]
    pub const MIX: BlendMode = BlendMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for BlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BlendMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BlendMode {
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
            Self::ADD => "ADD", Self::SUB => "SUB", Self::MIX => "MIX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BlendMode::ADD, BlendMode::SUB, BlendMode::MIX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BlendMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ADD", "BLEND_MODE_ADD", BlendMode::ADD), crate::meta::inspect::EnumConstant::new("SUB", "BLEND_MODE_SUB", BlendMode::SUB), crate::meta::inspect::EnumConstant::new("MIX", "BLEND_MODE_MIX", BlendMode::MIX)]
        }
    }
}
impl crate::meta::GodotConvert for BlendMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BlendMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Light2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::canvas_item::SignalsOfCanvasItem;
    impl WithSignals for Light2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCanvasItem < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}