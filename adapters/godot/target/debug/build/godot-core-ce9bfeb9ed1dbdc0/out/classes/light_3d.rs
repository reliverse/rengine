#![doc = "Sidecar module for class [`Light3D`][crate::classes::Light3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Light3D` enums](https://docs.godotengine.org/en/stable/classes/class_light3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Light3D.`\n\nInherits [`VisualInstance3D`][crate::classes::VisualInstance3D].\n\nRelated symbols:\n\n* [`light_3d`][crate::classes::light_3d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `Light3D`](https://docs.godotengine.org/en/stable/classes/class_light3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Light3D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Light3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Light3D {
        pub fn set_editor_only(&mut self, editor_only: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (editor_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4923usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "set_editor_only", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_editor_only(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4924usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "is_editor_only", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_param(&mut self, param: crate::classes::light_3d::Param, value: f32,) {
            type CallRet = ();
            type CallParams = (crate::classes::light_3d::Param, f32,);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4925usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "set_param", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_param(&self, param: crate::classes::light_3d::Param,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::classes::light_3d::Param,);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4926usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "get_param", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4927usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "set_shadow", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_shadow(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4928usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "has_shadow", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_negative(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4929usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "set_negative", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_negative(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4930usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "is_negative", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cull_mask(&mut self, cull_mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (cull_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4931usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "set_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cull_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4932usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "get_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enable_distance_fade(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4933usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "set_enable_distance_fade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_distance_fade_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4934usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "is_distance_fade_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_distance_fade_begin(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4935usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "set_distance_fade_begin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_distance_fade_begin(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4936usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "get_distance_fade_begin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_distance_fade_shadow(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4937usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "set_distance_fade_shadow", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_distance_fade_shadow(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4938usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "get_distance_fade_shadow", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_distance_fade_length(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4939usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "set_distance_fade_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_distance_fade_length(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4940usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "get_distance_fade_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4941usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "set_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4942usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "get_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_reverse_cull_face(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4943usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "set_shadow_reverse_cull_face", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadow_reverse_cull_face(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4944usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "get_shadow_reverse_cull_face", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_caster_mask(&mut self, caster_mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (caster_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4945usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "set_shadow_caster_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadow_caster_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4946usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "get_shadow_caster_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bake_mode(&mut self, bake_mode: crate::classes::light_3d::BakeMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::light_3d::BakeMode,);
            let args = (bake_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4947usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "set_bake_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bake_mode(&self,) -> crate::classes::light_3d::BakeMode {
            type CallRet = crate::classes::light_3d::BakeMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4948usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "get_bake_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_projector(&mut self, projector: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (projector.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4949usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "set_projector", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_projector(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4950usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "get_projector", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_temperature(&mut self, temperature: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (temperature,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4951usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "set_temperature", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_temperature(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4952usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "get_temperature", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_correlated_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4953usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Light3D", "get_correlated_color", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Light3D {
        type Base = crate::classes::VisualInstance3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Light3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Light3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for Light3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for Light3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Light3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Light3D {
        
    }
    impl std::ops::Deref for Light3D {
        type Target = crate::classes::VisualInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Light3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Light3D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Light3D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Param {
    ord: i32
}
impl Param {
    #[doc(alias = "PARAM_ENERGY")]
    #[doc = "Godot enumerator name: `PARAM_ENERGY`"]
    pub const ENERGY: Param = Param {
        ord: 0i32
    };
    #[doc(alias = "PARAM_INDIRECT_ENERGY")]
    #[doc = "Godot enumerator name: `PARAM_INDIRECT_ENERGY`"]
    pub const INDIRECT_ENERGY: Param = Param {
        ord: 1i32
    };
    #[doc(alias = "PARAM_VOLUMETRIC_FOG_ENERGY")]
    #[doc = "Godot enumerator name: `PARAM_VOLUMETRIC_FOG_ENERGY`"]
    pub const VOLUMETRIC_FOG_ENERGY: Param = Param {
        ord: 2i32
    };
    #[doc(alias = "PARAM_SPECULAR")]
    #[doc = "Godot enumerator name: `PARAM_SPECULAR`"]
    pub const SPECULAR: Param = Param {
        ord: 3i32
    };
    #[doc(alias = "PARAM_RANGE")]
    #[doc = "Godot enumerator name: `PARAM_RANGE`"]
    pub const RANGE: Param = Param {
        ord: 4i32
    };
    #[doc(alias = "PARAM_SIZE")]
    #[doc = "Godot enumerator name: `PARAM_SIZE`"]
    pub const SIZE: Param = Param {
        ord: 5i32
    };
    #[doc(alias = "PARAM_ATTENUATION")]
    #[doc = "Godot enumerator name: `PARAM_ATTENUATION`"]
    pub const ATTENUATION: Param = Param {
        ord: 6i32
    };
    #[doc(alias = "PARAM_SPOT_ANGLE")]
    #[doc = "Godot enumerator name: `PARAM_SPOT_ANGLE`"]
    pub const SPOT_ANGLE: Param = Param {
        ord: 7i32
    };
    #[doc(alias = "PARAM_SPOT_ATTENUATION")]
    #[doc = "Godot enumerator name: `PARAM_SPOT_ATTENUATION`"]
    pub const SPOT_ATTENUATION: Param = Param {
        ord: 8i32
    };
    #[doc(alias = "PARAM_SHADOW_MAX_DISTANCE")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_MAX_DISTANCE`"]
    pub const SHADOW_MAX_DISTANCE: Param = Param {
        ord: 9i32
    };
    #[doc(alias = "PARAM_SHADOW_SPLIT_1_OFFSET")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_SPLIT_1_OFFSET`"]
    pub const SHADOW_SPLIT_1_OFFSET: Param = Param {
        ord: 10i32
    };
    #[doc(alias = "PARAM_SHADOW_SPLIT_2_OFFSET")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_SPLIT_2_OFFSET`"]
    pub const SHADOW_SPLIT_2_OFFSET: Param = Param {
        ord: 11i32
    };
    #[doc(alias = "PARAM_SHADOW_SPLIT_3_OFFSET")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_SPLIT_3_OFFSET`"]
    pub const SHADOW_SPLIT_3_OFFSET: Param = Param {
        ord: 12i32
    };
    #[doc(alias = "PARAM_SHADOW_FADE_START")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_FADE_START`"]
    pub const SHADOW_FADE_START: Param = Param {
        ord: 13i32
    };
    #[doc(alias = "PARAM_SHADOW_NORMAL_BIAS")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_NORMAL_BIAS`"]
    pub const SHADOW_NORMAL_BIAS: Param = Param {
        ord: 14i32
    };
    #[doc(alias = "PARAM_SHADOW_BIAS")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_BIAS`"]
    pub const SHADOW_BIAS: Param = Param {
        ord: 15i32
    };
    #[doc(alias = "PARAM_SHADOW_PANCAKE_SIZE")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_PANCAKE_SIZE`"]
    pub const SHADOW_PANCAKE_SIZE: Param = Param {
        ord: 16i32
    };
    #[doc(alias = "PARAM_SHADOW_OPACITY")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_OPACITY`"]
    pub const SHADOW_OPACITY: Param = Param {
        ord: 17i32
    };
    #[doc(alias = "PARAM_SHADOW_BLUR")]
    #[doc = "Godot enumerator name: `PARAM_SHADOW_BLUR`"]
    pub const SHADOW_BLUR: Param = Param {
        ord: 18i32
    };
    #[doc(alias = "PARAM_TRANSMITTANCE_BIAS")]
    #[doc = "Godot enumerator name: `PARAM_TRANSMITTANCE_BIAS`"]
    pub const TRANSMITTANCE_BIAS: Param = Param {
        ord: 19i32
    };
    #[doc(alias = "PARAM_INTENSITY")]
    #[doc = "Godot enumerator name: `PARAM_INTENSITY`"]
    pub const INTENSITY: Param = Param {
        ord: 20i32
    };
    #[doc(alias = "PARAM_MAX")]
    #[doc = "Godot enumerator name: `PARAM_MAX`"]
    pub const MAX: Param = Param {
        ord: 21i32
    };
    
}
impl std::fmt::Debug for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Param") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Param {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 => Some(Self {
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
            Self::ENERGY => "ENERGY", Self::INDIRECT_ENERGY => "INDIRECT_ENERGY", Self::VOLUMETRIC_FOG_ENERGY => "VOLUMETRIC_FOG_ENERGY", Self::SPECULAR => "SPECULAR", Self::RANGE => "RANGE", Self::SIZE => "SIZE", Self::ATTENUATION => "ATTENUATION", Self::SPOT_ANGLE => "SPOT_ANGLE", Self::SPOT_ATTENUATION => "SPOT_ATTENUATION", Self::SHADOW_MAX_DISTANCE => "SHADOW_MAX_DISTANCE", Self::SHADOW_SPLIT_1_OFFSET => "SHADOW_SPLIT_1_OFFSET", Self::SHADOW_SPLIT_2_OFFSET => "SHADOW_SPLIT_2_OFFSET", Self::SHADOW_SPLIT_3_OFFSET => "SHADOW_SPLIT_3_OFFSET", Self::SHADOW_FADE_START => "SHADOW_FADE_START", Self::SHADOW_NORMAL_BIAS => "SHADOW_NORMAL_BIAS", Self::SHADOW_BIAS => "SHADOW_BIAS", Self::SHADOW_PANCAKE_SIZE => "SHADOW_PANCAKE_SIZE", Self::SHADOW_OPACITY => "SHADOW_OPACITY", Self::SHADOW_BLUR => "SHADOW_BLUR", Self::TRANSMITTANCE_BIAS => "TRANSMITTANCE_BIAS", Self::INTENSITY => "INTENSITY", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Param::ENERGY, Param::INDIRECT_ENERGY, Param::VOLUMETRIC_FOG_ENERGY, Param::SPECULAR, Param::RANGE, Param::SIZE, Param::ATTENUATION, Param::SPOT_ANGLE, Param::SPOT_ATTENUATION, Param::SHADOW_MAX_DISTANCE, Param::SHADOW_SPLIT_1_OFFSET, Param::SHADOW_SPLIT_2_OFFSET, Param::SHADOW_SPLIT_3_OFFSET, Param::SHADOW_FADE_START, Param::SHADOW_NORMAL_BIAS, Param::SHADOW_BIAS, Param::SHADOW_PANCAKE_SIZE, Param::SHADOW_OPACITY, Param::SHADOW_BLUR, Param::TRANSMITTANCE_BIAS, Param::INTENSITY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Param >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ENERGY", "PARAM_ENERGY", Param::ENERGY), crate::meta::inspect::EnumConstant::new("INDIRECT_ENERGY", "PARAM_INDIRECT_ENERGY", Param::INDIRECT_ENERGY), crate::meta::inspect::EnumConstant::new("VOLUMETRIC_FOG_ENERGY", "PARAM_VOLUMETRIC_FOG_ENERGY", Param::VOLUMETRIC_FOG_ENERGY), crate::meta::inspect::EnumConstant::new("SPECULAR", "PARAM_SPECULAR", Param::SPECULAR), crate::meta::inspect::EnumConstant::new("RANGE", "PARAM_RANGE", Param::RANGE), crate::meta::inspect::EnumConstant::new("SIZE", "PARAM_SIZE", Param::SIZE), crate::meta::inspect::EnumConstant::new("ATTENUATION", "PARAM_ATTENUATION", Param::ATTENUATION), crate::meta::inspect::EnumConstant::new("SPOT_ANGLE", "PARAM_SPOT_ANGLE", Param::SPOT_ANGLE), crate::meta::inspect::EnumConstant::new("SPOT_ATTENUATION", "PARAM_SPOT_ATTENUATION", Param::SPOT_ATTENUATION), crate::meta::inspect::EnumConstant::new("SHADOW_MAX_DISTANCE", "PARAM_SHADOW_MAX_DISTANCE", Param::SHADOW_MAX_DISTANCE), crate::meta::inspect::EnumConstant::new("SHADOW_SPLIT_1_OFFSET", "PARAM_SHADOW_SPLIT_1_OFFSET", Param::SHADOW_SPLIT_1_OFFSET), crate::meta::inspect::EnumConstant::new("SHADOW_SPLIT_2_OFFSET", "PARAM_SHADOW_SPLIT_2_OFFSET", Param::SHADOW_SPLIT_2_OFFSET), crate::meta::inspect::EnumConstant::new("SHADOW_SPLIT_3_OFFSET", "PARAM_SHADOW_SPLIT_3_OFFSET", Param::SHADOW_SPLIT_3_OFFSET), crate::meta::inspect::EnumConstant::new("SHADOW_FADE_START", "PARAM_SHADOW_FADE_START", Param::SHADOW_FADE_START), crate::meta::inspect::EnumConstant::new("SHADOW_NORMAL_BIAS", "PARAM_SHADOW_NORMAL_BIAS", Param::SHADOW_NORMAL_BIAS), crate::meta::inspect::EnumConstant::new("SHADOW_BIAS", "PARAM_SHADOW_BIAS", Param::SHADOW_BIAS), crate::meta::inspect::EnumConstant::new("SHADOW_PANCAKE_SIZE", "PARAM_SHADOW_PANCAKE_SIZE", Param::SHADOW_PANCAKE_SIZE), crate::meta::inspect::EnumConstant::new("SHADOW_OPACITY", "PARAM_SHADOW_OPACITY", Param::SHADOW_OPACITY), crate::meta::inspect::EnumConstant::new("SHADOW_BLUR", "PARAM_SHADOW_BLUR", Param::SHADOW_BLUR), crate::meta::inspect::EnumConstant::new("TRANSMITTANCE_BIAS", "PARAM_TRANSMITTANCE_BIAS", Param::TRANSMITTANCE_BIAS), crate::meta::inspect::EnumConstant::new("INTENSITY", "PARAM_INTENSITY", Param::INTENSITY), crate::meta::inspect::EnumConstant::new("MAX", "PARAM_MAX", Param::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Param {
    const ENUMERATOR_COUNT: usize = 21usize;
    
}
impl crate::meta::GodotConvert for Param {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Param {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Param {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BakeMode {
    ord: i32
}
impl BakeMode {
    #[doc(alias = "BAKE_DISABLED")]
    #[doc = "Godot enumerator name: `BAKE_DISABLED`"]
    pub const DISABLED: BakeMode = BakeMode {
        ord: 0i32
    };
    #[doc(alias = "BAKE_STATIC")]
    #[doc = "Godot enumerator name: `BAKE_STATIC`"]
    pub const STATIC: BakeMode = BakeMode {
        ord: 1i32
    };
    #[doc(alias = "BAKE_DYNAMIC")]
    #[doc = "Godot enumerator name: `BAKE_DYNAMIC`"]
    pub const DYNAMIC: BakeMode = BakeMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for BakeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BakeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BakeMode {
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
            Self::DISABLED => "DISABLED", Self::STATIC => "STATIC", Self::DYNAMIC => "DYNAMIC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BakeMode::DISABLED, BakeMode::STATIC, BakeMode::DYNAMIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BakeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "BAKE_DISABLED", BakeMode::DISABLED), crate::meta::inspect::EnumConstant::new("STATIC", "BAKE_STATIC", BakeMode::STATIC), crate::meta::inspect::EnumConstant::new("DYNAMIC", "BAKE_DYNAMIC", BakeMode::DYNAMIC)]
        }
    }
}
impl crate::meta::GodotConvert for BakeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BakeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BakeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Light3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for Light3D {
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