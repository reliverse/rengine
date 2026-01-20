#![doc = "Sidecar module for class [`SpriteBase3D`][crate::classes::SpriteBase3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SpriteBase3D` enums](https://docs.godotengine.org/en/stable/classes/class_spritebase3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SpriteBase3D.`\n\nInherits [`GeometryInstance3D`][crate::classes::GeometryInstance3D].\n\nRelated symbols:\n\n* [`sprite_base_3d`][crate::classes::sprite_base_3d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `SpriteBase3D`](https://docs.godotengine.org/en/stable/classes/class_spritebase3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<SpriteBase3D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SpriteBase3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl SpriteBase3D {
        pub fn set_centered(&mut self, centered: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (centered,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8465usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_centered", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_centered(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8466usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "is_centered", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8467usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8468usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flip_h(&mut self, flip_h: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (flip_h,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8469usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_flip_h", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_flipped_h(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8470usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "is_flipped_h", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flip_v(&mut self, flip_v: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (flip_v,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8471usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_flip_v", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_flipped_v(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8472usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "is_flipped_v", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_modulate(&mut self, modulate: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8473usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_modulate(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8474usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_render_priority(&mut self, priority: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8475usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_render_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_render_priority(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8476usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_render_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pixel_size(&mut self, pixel_size: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (pixel_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8477usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_pixel_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pixel_size(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8478usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_pixel_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_axis(&mut self, axis: Vector3Axis,) {
            type CallRet = ();
            type CallParams = (Vector3Axis,);
            let args = (axis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8479usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_axis(&self,) -> Vector3Axis {
            type CallRet = Vector3Axis;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8480usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_flag(&mut self, flag: crate::classes::sprite_base_3d::DrawFlags, enabled: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::sprite_base_3d::DrawFlags, bool,);
            let args = (flag, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8481usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_draw_flag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_draw_flag(&self, flag: crate::classes::sprite_base_3d::DrawFlags,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::sprite_base_3d::DrawFlags,);
            let args = (flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8482usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_draw_flag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_cut_mode(&mut self, mode: crate::classes::sprite_base_3d::AlphaCutMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::sprite_base_3d::AlphaCutMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8483usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_alpha_cut_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_cut_mode(&self,) -> crate::classes::sprite_base_3d::AlphaCutMode {
            type CallRet = crate::classes::sprite_base_3d::AlphaCutMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8484usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_alpha_cut_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_scissor_threshold(&mut self, threshold: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8485usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_alpha_scissor_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_scissor_threshold(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8486usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_alpha_scissor_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_hash_scale(&mut self, threshold: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8487usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_alpha_hash_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_hash_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8488usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_alpha_hash_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_antialiasing(&mut self, alpha_aa: crate::classes::base_material_3d::AlphaAntiAliasing,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::AlphaAntiAliasing,);
            let args = (alpha_aa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8489usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_alpha_antialiasing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_antialiasing(&self,) -> crate::classes::base_material_3d::AlphaAntiAliasing {
            type CallRet = crate::classes::base_material_3d::AlphaAntiAliasing;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8490usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_alpha_antialiasing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_antialiasing_edge(&mut self, edge: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (edge,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8491usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_alpha_antialiasing_edge", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_antialiasing_edge(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8492usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_alpha_antialiasing_edge", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_billboard_mode(&mut self, mode: crate::classes::base_material_3d::BillboardMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::BillboardMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8493usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_billboard_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_billboard_mode(&self,) -> crate::classes::base_material_3d::BillboardMode {
            type CallRet = crate::classes::base_material_3d::BillboardMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8494usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_billboard_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_filter(&mut self, mode: crate::classes::base_material_3d::TextureFilter,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::TextureFilter,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8495usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "set_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_filter(&self,) -> crate::classes::base_material_3d::TextureFilter {
            type CallRet = crate::classes::base_material_3d::TextureFilter;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8496usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_rect(&self,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8497usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "get_item_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn generate_triangle_mesh(&self,) -> Option < Gd < crate::classes::TriangleMesh > > {
            type CallRet = Option < Gd < crate::classes::TriangleMesh > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8498usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteBase3D", "generate_triangle_mesh", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for SpriteBase3D {
        type Base = crate::classes::GeometryInstance3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("SpriteBase3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SpriteBase3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for SpriteBase3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for SpriteBase3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for SpriteBase3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for SpriteBase3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SpriteBase3D {
        
    }
    impl std::ops::Deref for SpriteBase3D {
        type Target = crate::classes::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SpriteBase3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_SpriteBase3D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `SpriteBase3D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DrawFlags {
    ord: i32
}
impl DrawFlags {
    #[doc(alias = "FLAG_TRANSPARENT")]
    #[doc = "Godot enumerator name: `FLAG_TRANSPARENT`"]
    pub const TRANSPARENT: DrawFlags = DrawFlags {
        ord: 0i32
    };
    #[doc(alias = "FLAG_SHADED")]
    #[doc = "Godot enumerator name: `FLAG_SHADED`"]
    pub const SHADED: DrawFlags = DrawFlags {
        ord: 1i32
    };
    #[doc(alias = "FLAG_DOUBLE_SIDED")]
    #[doc = "Godot enumerator name: `FLAG_DOUBLE_SIDED`"]
    pub const DOUBLE_SIDED: DrawFlags = DrawFlags {
        ord: 2i32
    };
    #[doc(alias = "FLAG_DISABLE_DEPTH_TEST")]
    #[doc = "Godot enumerator name: `FLAG_DISABLE_DEPTH_TEST`"]
    pub const DISABLE_DEPTH_TEST: DrawFlags = DrawFlags {
        ord: 3i32
    };
    #[doc(alias = "FLAG_FIXED_SIZE")]
    #[doc = "Godot enumerator name: `FLAG_FIXED_SIZE`"]
    pub const FIXED_SIZE: DrawFlags = DrawFlags {
        ord: 4i32
    };
    #[doc(alias = "FLAG_MAX")]
    #[doc = "Godot enumerator name: `FLAG_MAX`"]
    pub const MAX: DrawFlags = DrawFlags {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for DrawFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DrawFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DrawFlags {
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
            Self::TRANSPARENT => "TRANSPARENT", Self::SHADED => "SHADED", Self::DOUBLE_SIDED => "DOUBLE_SIDED", Self::DISABLE_DEPTH_TEST => "DISABLE_DEPTH_TEST", Self::FIXED_SIZE => "FIXED_SIZE", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DrawFlags::TRANSPARENT, DrawFlags::SHADED, DrawFlags::DOUBLE_SIDED, DrawFlags::DISABLE_DEPTH_TEST, DrawFlags::FIXED_SIZE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DrawFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TRANSPARENT", "FLAG_TRANSPARENT", DrawFlags::TRANSPARENT), crate::meta::inspect::EnumConstant::new("SHADED", "FLAG_SHADED", DrawFlags::SHADED), crate::meta::inspect::EnumConstant::new("DOUBLE_SIDED", "FLAG_DOUBLE_SIDED", DrawFlags::DOUBLE_SIDED), crate::meta::inspect::EnumConstant::new("DISABLE_DEPTH_TEST", "FLAG_DISABLE_DEPTH_TEST", DrawFlags::DISABLE_DEPTH_TEST), crate::meta::inspect::EnumConstant::new("FIXED_SIZE", "FLAG_FIXED_SIZE", DrawFlags::FIXED_SIZE), crate::meta::inspect::EnumConstant::new("MAX", "FLAG_MAX", DrawFlags::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for DrawFlags {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for DrawFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DrawFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DrawFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AlphaCutMode {
    ord: i32
}
impl AlphaCutMode {
    #[doc(alias = "ALPHA_CUT_DISABLED")]
    #[doc = "Godot enumerator name: `ALPHA_CUT_DISABLED`"]
    pub const DISABLED: AlphaCutMode = AlphaCutMode {
        ord: 0i32
    };
    #[doc(alias = "ALPHA_CUT_DISCARD")]
    #[doc = "Godot enumerator name: `ALPHA_CUT_DISCARD`"]
    pub const DISCARD: AlphaCutMode = AlphaCutMode {
        ord: 1i32
    };
    #[doc(alias = "ALPHA_CUT_OPAQUE_PREPASS")]
    #[doc = "Godot enumerator name: `ALPHA_CUT_OPAQUE_PREPASS`"]
    pub const OPAQUE_PREPASS: AlphaCutMode = AlphaCutMode {
        ord: 2i32
    };
    #[doc(alias = "ALPHA_CUT_HASH")]
    #[doc = "Godot enumerator name: `ALPHA_CUT_HASH`"]
    pub const HASH: AlphaCutMode = AlphaCutMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for AlphaCutMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AlphaCutMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AlphaCutMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::DISCARD => "DISCARD", Self::OPAQUE_PREPASS => "OPAQUE_PREPASS", Self::HASH => "HASH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AlphaCutMode::DISABLED, AlphaCutMode::DISCARD, AlphaCutMode::OPAQUE_PREPASS, AlphaCutMode::HASH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AlphaCutMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "ALPHA_CUT_DISABLED", AlphaCutMode::DISABLED), crate::meta::inspect::EnumConstant::new("DISCARD", "ALPHA_CUT_DISCARD", AlphaCutMode::DISCARD), crate::meta::inspect::EnumConstant::new("OPAQUE_PREPASS", "ALPHA_CUT_OPAQUE_PREPASS", AlphaCutMode::OPAQUE_PREPASS), crate::meta::inspect::EnumConstant::new("HASH", "ALPHA_CUT_HASH", AlphaCutMode::HASH)]
        }
    }
}
impl crate::meta::GodotConvert for AlphaCutMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AlphaCutMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AlphaCutMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::SpriteBase3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for SpriteBase3D {
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