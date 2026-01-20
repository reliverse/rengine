#![doc = "Sidecar module for class [`BaseMaterial3D`][crate::classes::BaseMaterial3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `BaseMaterial3D` enums](https://docs.godotengine.org/en/stable/classes/class_basematerial3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `BaseMaterial3D.`\n\nInherits [`Material`][crate::classes::Material].\n\nRelated symbols:\n\n* [`base_material_3d`][crate::classes::base_material_3d]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `BaseMaterial3D`](https://docs.godotengine.org/en/stable/classes/class_basematerial3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<BaseMaterial3D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct BaseMaterial3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl BaseMaterial3D {
        pub fn set_albedo(&mut self, albedo: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (albedo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_albedo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_albedo(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_albedo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transparency(&mut self, transparency: crate::classes::base_material_3d::Transparency,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::Transparency,);
            let args = (transparency,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_transparency", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transparency(&self,) -> crate::classes::base_material_3d::Transparency {
            type CallRet = crate::classes::base_material_3d::Transparency;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_transparency", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_antialiasing(&mut self, alpha_aa: crate::classes::base_material_3d::AlphaAntiAliasing,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::AlphaAntiAliasing,);
            let args = (alpha_aa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_alpha_antialiasing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_antialiasing(&self,) -> crate::classes::base_material_3d::AlphaAntiAliasing {
            type CallRet = crate::classes::base_material_3d::AlphaAntiAliasing;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_alpha_antialiasing", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_antialiasing_edge(&mut self, edge: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (edge,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_alpha_antialiasing_edge", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_antialiasing_edge(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_alpha_antialiasing_edge", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shading_mode(&mut self, shading_mode: crate::classes::base_material_3d::ShadingMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::ShadingMode,);
            let args = (shading_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_shading_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shading_mode(&self,) -> crate::classes::base_material_3d::ShadingMode {
            type CallRet = crate::classes::base_material_3d::ShadingMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_shading_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_specular(&mut self, specular: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (specular,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_specular", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_specular(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_specular", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_metallic(&mut self, metallic: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (metallic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_metallic", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_metallic(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_metallic", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_roughness(&mut self, roughness: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (roughness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_roughness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_roughness(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_roughness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission(&mut self, emission: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (emission,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_emission", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_emission", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_energy_multiplier(&mut self, emission_energy_multiplier: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (emission_energy_multiplier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_emission_energy_multiplier", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_energy_multiplier(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_emission_energy_multiplier", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_intensity(&mut self, emission_energy_multiplier: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (emission_energy_multiplier,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_emission_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_intensity(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_emission_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_normal_scale(&mut self, normal_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (normal_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1148usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_normal_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_normal_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1149usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_normal_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rim(&mut self, rim: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (rim,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_rim", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rim(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_rim", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rim_tint(&mut self, rim_tint: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (rim_tint,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_rim_tint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rim_tint(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_rim_tint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_clearcoat(&mut self, clearcoat: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (clearcoat,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_clearcoat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_clearcoat(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_clearcoat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_clearcoat_roughness(&mut self, clearcoat_roughness: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (clearcoat_roughness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_clearcoat_roughness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_clearcoat_roughness(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_clearcoat_roughness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_anisotropy(&mut self, anisotropy: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (anisotropy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_anisotropy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_anisotropy(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_anisotropy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_heightmap_scale(&mut self, heightmap_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (heightmap_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_heightmap_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_heightmap_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_heightmap_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_subsurface_scattering_strength(&mut self, strength: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (strength,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_subsurface_scattering_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_subsurface_scattering_strength(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_subsurface_scattering_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transmittance_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_transmittance_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transmittance_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_transmittance_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transmittance_depth(&mut self, depth: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (depth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_transmittance_depth", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transmittance_depth(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_transmittance_depth", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transmittance_boost(&mut self, boost: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (boost,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_transmittance_boost", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transmittance_boost(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_transmittance_boost", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_backlight(&mut self, backlight: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (backlight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_backlight", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_backlight(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_backlight", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_refraction(&mut self, refraction: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (refraction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_refraction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_refraction(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_refraction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_point_size(&mut self, point_size: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (point_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_point_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_point_size(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_point_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_detail_uv(&mut self, detail_uv: crate::classes::base_material_3d::DetailUv,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::DetailUv,);
            let args = (detail_uv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_detail_uv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_detail_uv(&self,) -> crate::classes::base_material_3d::DetailUv {
            type CallRet = crate::classes::base_material_3d::DetailUv;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_detail_uv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_blend_mode(&mut self, blend_mode: crate::classes::base_material_3d::BlendMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::BlendMode,);
            let args = (blend_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_blend_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_blend_mode(&self,) -> crate::classes::base_material_3d::BlendMode {
            type CallRet = crate::classes::base_material_3d::BlendMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_blend_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_depth_draw_mode(&mut self, depth_draw_mode: crate::classes::base_material_3d::DepthDrawMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::DepthDrawMode,);
            let args = (depth_draw_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_depth_draw_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_depth_draw_mode(&self,) -> crate::classes::base_material_3d::DepthDrawMode {
            type CallRet = crate::classes::base_material_3d::DepthDrawMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_depth_draw_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_depth_test(&mut self, depth_test: crate::classes::base_material_3d::DepthTest,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::DepthTest,);
            let args = (depth_test,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_depth_test", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_depth_test(&self,) -> crate::classes::base_material_3d::DepthTest {
            type CallRet = crate::classes::base_material_3d::DepthTest;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_depth_test", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cull_mode(&mut self, cull_mode: crate::classes::base_material_3d::CullMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::CullMode,);
            let args = (cull_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_cull_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cull_mode(&self,) -> crate::classes::base_material_3d::CullMode {
            type CallRet = crate::classes::base_material_3d::CullMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_cull_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_diffuse_mode(&mut self, diffuse_mode: crate::classes::base_material_3d::DiffuseMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::DiffuseMode,);
            let args = (diffuse_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_diffuse_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_diffuse_mode(&self,) -> crate::classes::base_material_3d::DiffuseMode {
            type CallRet = crate::classes::base_material_3d::DiffuseMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_diffuse_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_specular_mode(&mut self, specular_mode: crate::classes::base_material_3d::SpecularMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::SpecularMode,);
            let args = (specular_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_specular_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_specular_mode(&self,) -> crate::classes::base_material_3d::SpecularMode {
            type CallRet = crate::classes::base_material_3d::SpecularMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_specular_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flag(&mut self, flag: crate::classes::base_material_3d::Flags, enable: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::Flags, bool,);
            let args = (flag, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_flag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_flag(&self, flag: crate::classes::base_material_3d::Flags,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::base_material_3d::Flags,);
            let args = (flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_flag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_filter(&mut self, mode: crate::classes::base_material_3d::TextureFilter,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::TextureFilter,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_filter(&self,) -> crate::classes::base_material_3d::TextureFilter {
            type CallRet = crate::classes::base_material_3d::TextureFilter;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_feature(&mut self, feature: crate::classes::base_material_3d::Feature, enable: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::Feature, bool,);
            let args = (feature, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_feature", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_feature(&self, feature: crate::classes::base_material_3d::Feature,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::base_material_3d::Feature,);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_feature", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture(&mut self, param: crate::classes::base_material_3d::TextureParam, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (crate::classes::base_material_3d::TextureParam, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (param, texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture(&self, param: crate::classes::base_material_3d::TextureParam,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (crate::classes::base_material_3d::TextureParam,);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_detail_blend_mode(&mut self, detail_blend_mode: crate::classes::base_material_3d::BlendMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::BlendMode,);
            let args = (detail_blend_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_detail_blend_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_detail_blend_mode(&self,) -> crate::classes::base_material_3d::BlendMode {
            type CallRet = crate::classes::base_material_3d::BlendMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_detail_blend_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_uv1_scale(&mut self, scale: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_uv1_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_uv1_scale(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_uv1_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_uv1_offset(&mut self, offset: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_uv1_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_uv1_offset(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_uv1_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_uv1_triplanar_blend_sharpness(&mut self, sharpness: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (sharpness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_uv1_triplanar_blend_sharpness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_uv1_triplanar_blend_sharpness(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_uv1_triplanar_blend_sharpness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_uv2_scale(&mut self, scale: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_uv2_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_uv2_scale(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_uv2_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_uv2_offset(&mut self, offset: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_uv2_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_uv2_offset(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_uv2_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_uv2_triplanar_blend_sharpness(&mut self, sharpness: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (sharpness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_uv2_triplanar_blend_sharpness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_uv2_triplanar_blend_sharpness(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_uv2_triplanar_blend_sharpness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_billboard_mode(&mut self, mode: crate::classes::base_material_3d::BillboardMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::BillboardMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_billboard_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_billboard_mode(&self,) -> crate::classes::base_material_3d::BillboardMode {
            type CallRet = crate::classes::base_material_3d::BillboardMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_billboard_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_particles_anim_h_frames(&mut self, frames: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_particles_anim_h_frames", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_particles_anim_h_frames(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_particles_anim_h_frames", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_particles_anim_v_frames(&mut self, frames: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_particles_anim_v_frames", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_particles_anim_v_frames(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_particles_anim_v_frames", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_particles_anim_loop(&mut self, loop_: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (loop_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_particles_anim_loop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_particles_anim_loop(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_particles_anim_loop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_heightmap_deep_parallax(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_heightmap_deep_parallax", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_heightmap_deep_parallax_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "is_heightmap_deep_parallax_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_heightmap_deep_parallax_min_layers(&mut self, layer: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_heightmap_deep_parallax_min_layers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_heightmap_deep_parallax_min_layers(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_heightmap_deep_parallax_min_layers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_heightmap_deep_parallax_max_layers(&mut self, layer: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_heightmap_deep_parallax_max_layers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_heightmap_deep_parallax_max_layers(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_heightmap_deep_parallax_max_layers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_heightmap_deep_parallax_flip_tangent(&mut self, flip: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (flip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_heightmap_deep_parallax_flip_tangent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_heightmap_deep_parallax_flip_tangent(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_heightmap_deep_parallax_flip_tangent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_heightmap_deep_parallax_flip_binormal(&mut self, flip: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (flip,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_heightmap_deep_parallax_flip_binormal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_heightmap_deep_parallax_flip_binormal(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_heightmap_deep_parallax_flip_binormal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_grow(&mut self, amount: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_grow", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_grow(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_grow", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emission_operator(&mut self, operator: crate::classes::base_material_3d::EmissionOperator,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::EmissionOperator,);
            let args = (operator,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_emission_operator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_emission_operator(&self,) -> crate::classes::base_material_3d::EmissionOperator {
            type CallRet = crate::classes::base_material_3d::EmissionOperator;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_emission_operator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ao_light_affect(&mut self, amount: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_ao_light_affect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ao_light_affect(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_ao_light_affect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_scissor_threshold(&mut self, threshold: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_alpha_scissor_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_scissor_threshold(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_alpha_scissor_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alpha_hash_scale(&mut self, threshold: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (threshold,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_alpha_hash_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alpha_hash_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_alpha_hash_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_grow_enabled(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_grow_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_grow_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "is_grow_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_metallic_texture_channel(&mut self, channel: crate::classes::base_material_3d::TextureChannel,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::TextureChannel,);
            let args = (channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_metallic_texture_channel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_metallic_texture_channel(&self,) -> crate::classes::base_material_3d::TextureChannel {
            type CallRet = crate::classes::base_material_3d::TextureChannel;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_metallic_texture_channel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_roughness_texture_channel(&mut self, channel: crate::classes::base_material_3d::TextureChannel,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::TextureChannel,);
            let args = (channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_roughness_texture_channel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_roughness_texture_channel(&self,) -> crate::classes::base_material_3d::TextureChannel {
            type CallRet = crate::classes::base_material_3d::TextureChannel;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_roughness_texture_channel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ao_texture_channel(&mut self, channel: crate::classes::base_material_3d::TextureChannel,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::TextureChannel,);
            let args = (channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_ao_texture_channel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ao_texture_channel(&self,) -> crate::classes::base_material_3d::TextureChannel {
            type CallRet = crate::classes::base_material_3d::TextureChannel;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_ao_texture_channel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_refraction_texture_channel(&mut self, channel: crate::classes::base_material_3d::TextureChannel,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::TextureChannel,);
            let args = (channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_refraction_texture_channel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_refraction_texture_channel(&self,) -> crate::classes::base_material_3d::TextureChannel {
            type CallRet = crate::classes::base_material_3d::TextureChannel;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_refraction_texture_channel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_proximity_fade_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_proximity_fade_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_proximity_fade_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "is_proximity_fade_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_proximity_fade_distance(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_proximity_fade_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_proximity_fade_distance(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_proximity_fade_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_msdf_pixel_range(&mut self, range: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_msdf_pixel_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_msdf_pixel_range(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_msdf_pixel_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_msdf_outline_size(&mut self, size: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_msdf_outline_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_msdf_outline_size(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_msdf_outline_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_distance_fade(&mut self, mode: crate::classes::base_material_3d::DistanceFadeMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::DistanceFadeMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_distance_fade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_distance_fade(&self,) -> crate::classes::base_material_3d::DistanceFadeMode {
            type CallRet = crate::classes::base_material_3d::DistanceFadeMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_distance_fade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_distance_fade_max_distance(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_distance_fade_max_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_distance_fade_max_distance(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_distance_fade_max_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_distance_fade_min_distance(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_distance_fade_min_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_distance_fade_min_distance(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_distance_fade_min_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_z_clip_scale(&mut self, scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_z_clip_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_z_clip_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_z_clip_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fov_override(&mut self, scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_fov_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fov_override(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_fov_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stencil_mode(&mut self, stencil_mode: crate::classes::base_material_3d::StencilMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::StencilMode,);
            let args = (stencil_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_stencil_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stencil_mode(&self,) -> crate::classes::base_material_3d::StencilMode {
            type CallRet = crate::classes::base_material_3d::StencilMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_stencil_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stencil_flags(&mut self, stencil_flags: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (stencil_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_stencil_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stencil_flags(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_stencil_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stencil_compare(&mut self, stencil_compare: crate::classes::base_material_3d::StencilCompare,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_material_3d::StencilCompare,);
            let args = (stencil_compare,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_stencil_compare", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stencil_compare(&self,) -> crate::classes::base_material_3d::StencilCompare {
            type CallRet = crate::classes::base_material_3d::StencilCompare;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_stencil_compare", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stencil_reference(&mut self, stencil_reference: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (stencil_reference,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_stencil_reference", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stencil_reference(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_stencil_reference", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stencil_effect_color(&mut self, stencil_color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (stencil_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_stencil_effect_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stencil_effect_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_stencil_effect_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stencil_effect_outline_thickness(&mut self, stencil_outline_thickness: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (stencil_outline_thickness,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "set_stencil_effect_outline_thickness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stencil_effect_outline_thickness(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseMaterial3D", "get_stencil_effect_outline_thickness", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for BaseMaterial3D {
        type Base = crate::classes::Material;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("BaseMaterial3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for BaseMaterial3D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Material > for BaseMaterial3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for BaseMaterial3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for BaseMaterial3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for BaseMaterial3D {
        
    }
    impl std::ops::Deref for BaseMaterial3D {
        type Target = crate::classes::Material;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for BaseMaterial3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_BaseMaterial3D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `BaseMaterial3D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureParam {
    ord: i32
}
impl TextureParam {
    #[doc(alias = "TEXTURE_ALBEDO")]
    #[doc = "Godot enumerator name: `TEXTURE_ALBEDO`"]
    pub const ALBEDO: TextureParam = TextureParam {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_METALLIC")]
    #[doc = "Godot enumerator name: `TEXTURE_METALLIC`"]
    pub const METALLIC: TextureParam = TextureParam {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_ROUGHNESS")]
    #[doc = "Godot enumerator name: `TEXTURE_ROUGHNESS`"]
    pub const ROUGHNESS: TextureParam = TextureParam {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_EMISSION")]
    #[doc = "Godot enumerator name: `TEXTURE_EMISSION`"]
    pub const EMISSION: TextureParam = TextureParam {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_NORMAL")]
    #[doc = "Godot enumerator name: `TEXTURE_NORMAL`"]
    pub const NORMAL: TextureParam = TextureParam {
        ord: 4i32
    };
    #[doc(alias = "TEXTURE_BENT_NORMAL")]
    #[doc = "Godot enumerator name: `TEXTURE_BENT_NORMAL`"]
    pub const BENT_NORMAL: TextureParam = TextureParam {
        ord: 18i32
    };
    #[doc(alias = "TEXTURE_RIM")]
    #[doc = "Godot enumerator name: `TEXTURE_RIM`"]
    pub const RIM: TextureParam = TextureParam {
        ord: 5i32
    };
    #[doc(alias = "TEXTURE_CLEARCOAT")]
    #[doc = "Godot enumerator name: `TEXTURE_CLEARCOAT`"]
    pub const CLEARCOAT: TextureParam = TextureParam {
        ord: 6i32
    };
    #[doc(alias = "TEXTURE_FLOWMAP")]
    #[doc = "Godot enumerator name: `TEXTURE_FLOWMAP`"]
    pub const FLOWMAP: TextureParam = TextureParam {
        ord: 7i32
    };
    #[doc(alias = "TEXTURE_AMBIENT_OCCLUSION")]
    #[doc = "Godot enumerator name: `TEXTURE_AMBIENT_OCCLUSION`"]
    pub const AMBIENT_OCCLUSION: TextureParam = TextureParam {
        ord: 8i32
    };
    #[doc(alias = "TEXTURE_HEIGHTMAP")]
    #[doc = "Godot enumerator name: `TEXTURE_HEIGHTMAP`"]
    pub const HEIGHTMAP: TextureParam = TextureParam {
        ord: 9i32
    };
    #[doc(alias = "TEXTURE_SUBSURFACE_SCATTERING")]
    #[doc = "Godot enumerator name: `TEXTURE_SUBSURFACE_SCATTERING`"]
    pub const SUBSURFACE_SCATTERING: TextureParam = TextureParam {
        ord: 10i32
    };
    #[doc(alias = "TEXTURE_SUBSURFACE_TRANSMITTANCE")]
    #[doc = "Godot enumerator name: `TEXTURE_SUBSURFACE_TRANSMITTANCE`"]
    pub const SUBSURFACE_TRANSMITTANCE: TextureParam = TextureParam {
        ord: 11i32
    };
    #[doc(alias = "TEXTURE_BACKLIGHT")]
    #[doc = "Godot enumerator name: `TEXTURE_BACKLIGHT`"]
    pub const BACKLIGHT: TextureParam = TextureParam {
        ord: 12i32
    };
    #[doc(alias = "TEXTURE_REFRACTION")]
    #[doc = "Godot enumerator name: `TEXTURE_REFRACTION`"]
    pub const REFRACTION: TextureParam = TextureParam {
        ord: 13i32
    };
    #[doc(alias = "TEXTURE_DETAIL_MASK")]
    #[doc = "Godot enumerator name: `TEXTURE_DETAIL_MASK`"]
    pub const DETAIL_MASK: TextureParam = TextureParam {
        ord: 14i32
    };
    #[doc(alias = "TEXTURE_DETAIL_ALBEDO")]
    #[doc = "Godot enumerator name: `TEXTURE_DETAIL_ALBEDO`"]
    pub const DETAIL_ALBEDO: TextureParam = TextureParam {
        ord: 15i32
    };
    #[doc(alias = "TEXTURE_DETAIL_NORMAL")]
    #[doc = "Godot enumerator name: `TEXTURE_DETAIL_NORMAL`"]
    pub const DETAIL_NORMAL: TextureParam = TextureParam {
        ord: 16i32
    };
    #[doc(alias = "TEXTURE_ORM")]
    #[doc = "Godot enumerator name: `TEXTURE_ORM`"]
    pub const ORM: TextureParam = TextureParam {
        ord: 17i32
    };
    #[doc(alias = "TEXTURE_MAX")]
    #[doc = "Godot enumerator name: `TEXTURE_MAX`"]
    pub const MAX: TextureParam = TextureParam {
        ord: 19i32
    };
    
}
impl std::fmt::Debug for TextureParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureParam {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 => Some(Self {
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
            Self::ALBEDO => "ALBEDO", Self::METALLIC => "METALLIC", Self::ROUGHNESS => "ROUGHNESS", Self::EMISSION => "EMISSION", Self::NORMAL => "NORMAL", Self::BENT_NORMAL => "BENT_NORMAL", Self::RIM => "RIM", Self::CLEARCOAT => "CLEARCOAT", Self::FLOWMAP => "FLOWMAP", Self::AMBIENT_OCCLUSION => "AMBIENT_OCCLUSION", Self::HEIGHTMAP => "HEIGHTMAP", Self::SUBSURFACE_SCATTERING => "SUBSURFACE_SCATTERING", Self::SUBSURFACE_TRANSMITTANCE => "SUBSURFACE_TRANSMITTANCE", Self::BACKLIGHT => "BACKLIGHT", Self::REFRACTION => "REFRACTION", Self::DETAIL_MASK => "DETAIL_MASK", Self::DETAIL_ALBEDO => "DETAIL_ALBEDO", Self::DETAIL_NORMAL => "DETAIL_NORMAL", Self::ORM => "ORM", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureParam::ALBEDO, TextureParam::METALLIC, TextureParam::ROUGHNESS, TextureParam::EMISSION, TextureParam::NORMAL, TextureParam::BENT_NORMAL, TextureParam::RIM, TextureParam::CLEARCOAT, TextureParam::FLOWMAP, TextureParam::AMBIENT_OCCLUSION, TextureParam::HEIGHTMAP, TextureParam::SUBSURFACE_SCATTERING, TextureParam::SUBSURFACE_TRANSMITTANCE, TextureParam::BACKLIGHT, TextureParam::REFRACTION, TextureParam::DETAIL_MASK, TextureParam::DETAIL_ALBEDO, TextureParam::DETAIL_NORMAL, TextureParam::ORM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureParam >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ALBEDO", "TEXTURE_ALBEDO", TextureParam::ALBEDO), crate::meta::inspect::EnumConstant::new("METALLIC", "TEXTURE_METALLIC", TextureParam::METALLIC), crate::meta::inspect::EnumConstant::new("ROUGHNESS", "TEXTURE_ROUGHNESS", TextureParam::ROUGHNESS), crate::meta::inspect::EnumConstant::new("EMISSION", "TEXTURE_EMISSION", TextureParam::EMISSION), crate::meta::inspect::EnumConstant::new("NORMAL", "TEXTURE_NORMAL", TextureParam::NORMAL), crate::meta::inspect::EnumConstant::new("BENT_NORMAL", "TEXTURE_BENT_NORMAL", TextureParam::BENT_NORMAL), crate::meta::inspect::EnumConstant::new("RIM", "TEXTURE_RIM", TextureParam::RIM), crate::meta::inspect::EnumConstant::new("CLEARCOAT", "TEXTURE_CLEARCOAT", TextureParam::CLEARCOAT), crate::meta::inspect::EnumConstant::new("FLOWMAP", "TEXTURE_FLOWMAP", TextureParam::FLOWMAP), crate::meta::inspect::EnumConstant::new("AMBIENT_OCCLUSION", "TEXTURE_AMBIENT_OCCLUSION", TextureParam::AMBIENT_OCCLUSION), crate::meta::inspect::EnumConstant::new("HEIGHTMAP", "TEXTURE_HEIGHTMAP", TextureParam::HEIGHTMAP), crate::meta::inspect::EnumConstant::new("SUBSURFACE_SCATTERING", "TEXTURE_SUBSURFACE_SCATTERING", TextureParam::SUBSURFACE_SCATTERING), crate::meta::inspect::EnumConstant::new("SUBSURFACE_TRANSMITTANCE", "TEXTURE_SUBSURFACE_TRANSMITTANCE", TextureParam::SUBSURFACE_TRANSMITTANCE), crate::meta::inspect::EnumConstant::new("BACKLIGHT", "TEXTURE_BACKLIGHT", TextureParam::BACKLIGHT), crate::meta::inspect::EnumConstant::new("REFRACTION", "TEXTURE_REFRACTION", TextureParam::REFRACTION), crate::meta::inspect::EnumConstant::new("DETAIL_MASK", "TEXTURE_DETAIL_MASK", TextureParam::DETAIL_MASK), crate::meta::inspect::EnumConstant::new("DETAIL_ALBEDO", "TEXTURE_DETAIL_ALBEDO", TextureParam::DETAIL_ALBEDO), crate::meta::inspect::EnumConstant::new("DETAIL_NORMAL", "TEXTURE_DETAIL_NORMAL", TextureParam::DETAIL_NORMAL), crate::meta::inspect::EnumConstant::new("ORM", "TEXTURE_ORM", TextureParam::ORM), crate::meta::inspect::EnumConstant::new("MAX", "TEXTURE_MAX", TextureParam::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for TextureParam {
    const ENUMERATOR_COUNT: usize = 19usize;
    
}
impl crate::meta::GodotConvert for TextureParam {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureParam {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureFilter {
    ord: i32
}
impl TextureFilter {
    #[doc(alias = "TEXTURE_FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_NEAREST`"]
    pub const NEAREST: TextureFilter = TextureFilter {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_LINEAR`"]
    pub const LINEAR: TextureFilter = TextureFilter {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_FILTER_NEAREST_WITH_MIPMAPS")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_NEAREST_WITH_MIPMAPS`"]
    pub const NEAREST_WITH_MIPMAPS: TextureFilter = TextureFilter {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_FILTER_LINEAR_WITH_MIPMAPS")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_LINEAR_WITH_MIPMAPS`"]
    pub const LINEAR_WITH_MIPMAPS: TextureFilter = TextureFilter {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC`"]
    pub const NEAREST_WITH_MIPMAPS_ANISOTROPIC: TextureFilter = TextureFilter {
        ord: 4i32
    };
    #[doc(alias = "TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC`"]
    pub const LINEAR_WITH_MIPMAPS_ANISOTROPIC: TextureFilter = TextureFilter {
        ord: 5i32
    };
    #[doc(alias = "TEXTURE_FILTER_MAX")]
    #[doc = "Godot enumerator name: `TEXTURE_FILTER_MAX`"]
    pub const MAX: TextureFilter = TextureFilter {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for TextureFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureFilter {
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
            Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", Self::NEAREST_WITH_MIPMAPS => "NEAREST_WITH_MIPMAPS", Self::LINEAR_WITH_MIPMAPS => "LINEAR_WITH_MIPMAPS", Self::NEAREST_WITH_MIPMAPS_ANISOTROPIC => "NEAREST_WITH_MIPMAPS_ANISOTROPIC", Self::LINEAR_WITH_MIPMAPS_ANISOTROPIC => "LINEAR_WITH_MIPMAPS_ANISOTROPIC", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureFilter::NEAREST, TextureFilter::LINEAR, TextureFilter::NEAREST_WITH_MIPMAPS, TextureFilter::LINEAR_WITH_MIPMAPS, TextureFilter::NEAREST_WITH_MIPMAPS_ANISOTROPIC, TextureFilter::LINEAR_WITH_MIPMAPS_ANISOTROPIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEAREST", "TEXTURE_FILTER_NEAREST", TextureFilter::NEAREST), crate::meta::inspect::EnumConstant::new("LINEAR", "TEXTURE_FILTER_LINEAR", TextureFilter::LINEAR), crate::meta::inspect::EnumConstant::new("NEAREST_WITH_MIPMAPS", "TEXTURE_FILTER_NEAREST_WITH_MIPMAPS", TextureFilter::NEAREST_WITH_MIPMAPS), crate::meta::inspect::EnumConstant::new("LINEAR_WITH_MIPMAPS", "TEXTURE_FILTER_LINEAR_WITH_MIPMAPS", TextureFilter::LINEAR_WITH_MIPMAPS), crate::meta::inspect::EnumConstant::new("NEAREST_WITH_MIPMAPS_ANISOTROPIC", "TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC", TextureFilter::NEAREST_WITH_MIPMAPS_ANISOTROPIC), crate::meta::inspect::EnumConstant::new("LINEAR_WITH_MIPMAPS_ANISOTROPIC", "TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC", TextureFilter::LINEAR_WITH_MIPMAPS_ANISOTROPIC), crate::meta::inspect::EnumConstant::new("MAX", "TEXTURE_FILTER_MAX", TextureFilter::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for TextureFilter {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for TextureFilter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `DetailUV`."]
pub struct DetailUv {
    ord: i32
}
impl DetailUv {
    #[doc(alias = "DETAIL_UV_1")]
    #[doc = "Godot enumerator name: `DETAIL_UV_1`"]
    pub const UV_1: DetailUv = DetailUv {
        ord: 0i32
    };
    #[doc(alias = "DETAIL_UV_2")]
    #[doc = "Godot enumerator name: `DETAIL_UV_2`"]
    pub const UV_2: DetailUv = DetailUv {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for DetailUv {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DetailUv") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DetailUv {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::UV_1 => "UV_1", Self::UV_2 => "UV_2", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DetailUv::UV_1, DetailUv::UV_2]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DetailUv >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UV_1", "DETAIL_UV_1", DetailUv::UV_1), crate::meta::inspect::EnumConstant::new("UV_2", "DETAIL_UV_2", DetailUv::UV_2)]
        }
    }
}
impl crate::meta::GodotConvert for DetailUv {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DetailUv {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DetailUv {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Transparency {
    ord: i32
}
impl Transparency {
    #[doc(alias = "TRANSPARENCY_DISABLED")]
    #[doc = "Godot enumerator name: `TRANSPARENCY_DISABLED`"]
    pub const DISABLED: Transparency = Transparency {
        ord: 0i32
    };
    #[doc(alias = "TRANSPARENCY_ALPHA")]
    #[doc = "Godot enumerator name: `TRANSPARENCY_ALPHA`"]
    pub const ALPHA: Transparency = Transparency {
        ord: 1i32
    };
    #[doc(alias = "TRANSPARENCY_ALPHA_SCISSOR")]
    #[doc = "Godot enumerator name: `TRANSPARENCY_ALPHA_SCISSOR`"]
    pub const ALPHA_SCISSOR: Transparency = Transparency {
        ord: 2i32
    };
    #[doc(alias = "TRANSPARENCY_ALPHA_HASH")]
    #[doc = "Godot enumerator name: `TRANSPARENCY_ALPHA_HASH`"]
    pub const ALPHA_HASH: Transparency = Transparency {
        ord: 3i32
    };
    #[doc(alias = "TRANSPARENCY_ALPHA_DEPTH_PRE_PASS")]
    #[doc = "Godot enumerator name: `TRANSPARENCY_ALPHA_DEPTH_PRE_PASS`"]
    pub const ALPHA_DEPTH_PRE_PASS: Transparency = Transparency {
        ord: 4i32
    };
    #[doc(alias = "TRANSPARENCY_MAX")]
    #[doc = "Godot enumerator name: `TRANSPARENCY_MAX`"]
    pub const MAX: Transparency = Transparency {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for Transparency {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Transparency") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Transparency {
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
            Self::DISABLED => "DISABLED", Self::ALPHA => "ALPHA", Self::ALPHA_SCISSOR => "ALPHA_SCISSOR", Self::ALPHA_HASH => "ALPHA_HASH", Self::ALPHA_DEPTH_PRE_PASS => "ALPHA_DEPTH_PRE_PASS", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Transparency::DISABLED, Transparency::ALPHA, Transparency::ALPHA_SCISSOR, Transparency::ALPHA_HASH, Transparency::ALPHA_DEPTH_PRE_PASS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Transparency >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "TRANSPARENCY_DISABLED", Transparency::DISABLED), crate::meta::inspect::EnumConstant::new("ALPHA", "TRANSPARENCY_ALPHA", Transparency::ALPHA), crate::meta::inspect::EnumConstant::new("ALPHA_SCISSOR", "TRANSPARENCY_ALPHA_SCISSOR", Transparency::ALPHA_SCISSOR), crate::meta::inspect::EnumConstant::new("ALPHA_HASH", "TRANSPARENCY_ALPHA_HASH", Transparency::ALPHA_HASH), crate::meta::inspect::EnumConstant::new("ALPHA_DEPTH_PRE_PASS", "TRANSPARENCY_ALPHA_DEPTH_PRE_PASS", Transparency::ALPHA_DEPTH_PRE_PASS), crate::meta::inspect::EnumConstant::new("MAX", "TRANSPARENCY_MAX", Transparency::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Transparency {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for Transparency {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Transparency {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Transparency {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShadingMode {
    ord: i32
}
impl ShadingMode {
    #[doc(alias = "SHADING_MODE_UNSHADED")]
    #[doc = "Godot enumerator name: `SHADING_MODE_UNSHADED`"]
    pub const UNSHADED: ShadingMode = ShadingMode {
        ord: 0i32
    };
    #[doc(alias = "SHADING_MODE_PER_PIXEL")]
    #[doc = "Godot enumerator name: `SHADING_MODE_PER_PIXEL`"]
    pub const PER_PIXEL: ShadingMode = ShadingMode {
        ord: 1i32
    };
    #[doc(alias = "SHADING_MODE_PER_VERTEX")]
    #[doc = "Godot enumerator name: `SHADING_MODE_PER_VERTEX`"]
    pub const PER_VERTEX: ShadingMode = ShadingMode {
        ord: 2i32
    };
    #[doc(alias = "SHADING_MODE_MAX")]
    #[doc = "Godot enumerator name: `SHADING_MODE_MAX`"]
    pub const MAX: ShadingMode = ShadingMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ShadingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShadingMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShadingMode {
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
            Self::UNSHADED => "UNSHADED", Self::PER_PIXEL => "PER_PIXEL", Self::PER_VERTEX => "PER_VERTEX", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ShadingMode::UNSHADED, ShadingMode::PER_PIXEL, ShadingMode::PER_VERTEX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ShadingMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNSHADED", "SHADING_MODE_UNSHADED", ShadingMode::UNSHADED), crate::meta::inspect::EnumConstant::new("PER_PIXEL", "SHADING_MODE_PER_PIXEL", ShadingMode::PER_PIXEL), crate::meta::inspect::EnumConstant::new("PER_VERTEX", "SHADING_MODE_PER_VERTEX", ShadingMode::PER_VERTEX), crate::meta::inspect::EnumConstant::new("MAX", "SHADING_MODE_MAX", ShadingMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ShadingMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ShadingMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ShadingMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShadingMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Feature {
    ord: i32
}
impl Feature {
    #[doc(alias = "FEATURE_EMISSION")]
    #[doc = "Godot enumerator name: `FEATURE_EMISSION`"]
    pub const EMISSION: Feature = Feature {
        ord: 0i32
    };
    #[doc(alias = "FEATURE_NORMAL_MAPPING")]
    #[doc = "Godot enumerator name: `FEATURE_NORMAL_MAPPING`"]
    pub const NORMAL_MAPPING: Feature = Feature {
        ord: 1i32
    };
    #[doc(alias = "FEATURE_RIM")]
    #[doc = "Godot enumerator name: `FEATURE_RIM`"]
    pub const RIM: Feature = Feature {
        ord: 2i32
    };
    #[doc(alias = "FEATURE_CLEARCOAT")]
    #[doc = "Godot enumerator name: `FEATURE_CLEARCOAT`"]
    pub const CLEARCOAT: Feature = Feature {
        ord: 3i32
    };
    #[doc(alias = "FEATURE_ANISOTROPY")]
    #[doc = "Godot enumerator name: `FEATURE_ANISOTROPY`"]
    pub const ANISOTROPY: Feature = Feature {
        ord: 4i32
    };
    #[doc(alias = "FEATURE_AMBIENT_OCCLUSION")]
    #[doc = "Godot enumerator name: `FEATURE_AMBIENT_OCCLUSION`"]
    pub const AMBIENT_OCCLUSION: Feature = Feature {
        ord: 5i32
    };
    #[doc(alias = "FEATURE_HEIGHT_MAPPING")]
    #[doc = "Godot enumerator name: `FEATURE_HEIGHT_MAPPING`"]
    pub const HEIGHT_MAPPING: Feature = Feature {
        ord: 6i32
    };
    #[doc(alias = "FEATURE_SUBSURFACE_SCATTERING")]
    #[doc = "Godot enumerator name: `FEATURE_SUBSURFACE_SCATTERING`"]
    pub const SUBSURFACE_SCATTERING: Feature = Feature {
        ord: 7i32
    };
    #[doc(alias = "FEATURE_SUBSURFACE_TRANSMITTANCE")]
    #[doc = "Godot enumerator name: `FEATURE_SUBSURFACE_TRANSMITTANCE`"]
    pub const SUBSURFACE_TRANSMITTANCE: Feature = Feature {
        ord: 8i32
    };
    #[doc(alias = "FEATURE_BACKLIGHT")]
    #[doc = "Godot enumerator name: `FEATURE_BACKLIGHT`"]
    pub const BACKLIGHT: Feature = Feature {
        ord: 9i32
    };
    #[doc(alias = "FEATURE_REFRACTION")]
    #[doc = "Godot enumerator name: `FEATURE_REFRACTION`"]
    pub const REFRACTION: Feature = Feature {
        ord: 10i32
    };
    #[doc(alias = "FEATURE_DETAIL")]
    #[doc = "Godot enumerator name: `FEATURE_DETAIL`"]
    pub const DETAIL: Feature = Feature {
        ord: 11i32
    };
    #[doc(alias = "FEATURE_BENT_NORMAL_MAPPING")]
    #[doc = "Godot enumerator name: `FEATURE_BENT_NORMAL_MAPPING`"]
    pub const BENT_NORMAL_MAPPING: Feature = Feature {
        ord: 12i32
    };
    #[doc(alias = "FEATURE_MAX")]
    #[doc = "Godot enumerator name: `FEATURE_MAX`"]
    pub const MAX: Feature = Feature {
        ord: 13i32
    };
    
}
impl std::fmt::Debug for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Feature") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Feature {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
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
            Self::EMISSION => "EMISSION", Self::NORMAL_MAPPING => "NORMAL_MAPPING", Self::RIM => "RIM", Self::CLEARCOAT => "CLEARCOAT", Self::ANISOTROPY => "ANISOTROPY", Self::AMBIENT_OCCLUSION => "AMBIENT_OCCLUSION", Self::HEIGHT_MAPPING => "HEIGHT_MAPPING", Self::SUBSURFACE_SCATTERING => "SUBSURFACE_SCATTERING", Self::SUBSURFACE_TRANSMITTANCE => "SUBSURFACE_TRANSMITTANCE", Self::BACKLIGHT => "BACKLIGHT", Self::REFRACTION => "REFRACTION", Self::DETAIL => "DETAIL", Self::BENT_NORMAL_MAPPING => "BENT_NORMAL_MAPPING", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Feature::EMISSION, Feature::NORMAL_MAPPING, Feature::RIM, Feature::CLEARCOAT, Feature::ANISOTROPY, Feature::AMBIENT_OCCLUSION, Feature::HEIGHT_MAPPING, Feature::SUBSURFACE_SCATTERING, Feature::SUBSURFACE_TRANSMITTANCE, Feature::BACKLIGHT, Feature::REFRACTION, Feature::DETAIL, Feature::BENT_NORMAL_MAPPING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Feature >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("EMISSION", "FEATURE_EMISSION", Feature::EMISSION), crate::meta::inspect::EnumConstant::new("NORMAL_MAPPING", "FEATURE_NORMAL_MAPPING", Feature::NORMAL_MAPPING), crate::meta::inspect::EnumConstant::new("RIM", "FEATURE_RIM", Feature::RIM), crate::meta::inspect::EnumConstant::new("CLEARCOAT", "FEATURE_CLEARCOAT", Feature::CLEARCOAT), crate::meta::inspect::EnumConstant::new("ANISOTROPY", "FEATURE_ANISOTROPY", Feature::ANISOTROPY), crate::meta::inspect::EnumConstant::new("AMBIENT_OCCLUSION", "FEATURE_AMBIENT_OCCLUSION", Feature::AMBIENT_OCCLUSION), crate::meta::inspect::EnumConstant::new("HEIGHT_MAPPING", "FEATURE_HEIGHT_MAPPING", Feature::HEIGHT_MAPPING), crate::meta::inspect::EnumConstant::new("SUBSURFACE_SCATTERING", "FEATURE_SUBSURFACE_SCATTERING", Feature::SUBSURFACE_SCATTERING), crate::meta::inspect::EnumConstant::new("SUBSURFACE_TRANSMITTANCE", "FEATURE_SUBSURFACE_TRANSMITTANCE", Feature::SUBSURFACE_TRANSMITTANCE), crate::meta::inspect::EnumConstant::new("BACKLIGHT", "FEATURE_BACKLIGHT", Feature::BACKLIGHT), crate::meta::inspect::EnumConstant::new("REFRACTION", "FEATURE_REFRACTION", Feature::REFRACTION), crate::meta::inspect::EnumConstant::new("DETAIL", "FEATURE_DETAIL", Feature::DETAIL), crate::meta::inspect::EnumConstant::new("BENT_NORMAL_MAPPING", "FEATURE_BENT_NORMAL_MAPPING", Feature::BENT_NORMAL_MAPPING), crate::meta::inspect::EnumConstant::new("MAX", "FEATURE_MAX", Feature::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Feature {
    const ENUMERATOR_COUNT: usize = 13usize;
    
}
impl crate::meta::GodotConvert for Feature {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Feature {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Feature {
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
    #[doc(alias = "BLEND_MODE_MIX")]
    #[doc = "Godot enumerator name: `BLEND_MODE_MIX`"]
    pub const MIX: BlendMode = BlendMode {
        ord: 0i32
    };
    #[doc(alias = "BLEND_MODE_ADD")]
    #[doc = "Godot enumerator name: `BLEND_MODE_ADD`"]
    pub const ADD: BlendMode = BlendMode {
        ord: 1i32
    };
    #[doc(alias = "BLEND_MODE_SUB")]
    #[doc = "Godot enumerator name: `BLEND_MODE_SUB`"]
    pub const SUB: BlendMode = BlendMode {
        ord: 2i32
    };
    #[doc(alias = "BLEND_MODE_MUL")]
    #[doc = "Godot enumerator name: `BLEND_MODE_MUL`"]
    pub const MUL: BlendMode = BlendMode {
        ord: 3i32
    };
    #[doc(alias = "BLEND_MODE_PREMULT_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_MODE_PREMULT_ALPHA`"]
    pub const PREMULT_ALPHA: BlendMode = BlendMode {
        ord: 4i32
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
            Self::MIX => "MIX", Self::ADD => "ADD", Self::SUB => "SUB", Self::MUL => "MUL", Self::PREMULT_ALPHA => "PREMULT_ALPHA", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BlendMode::MIX, BlendMode::ADD, BlendMode::SUB, BlendMode::MUL, BlendMode::PREMULT_ALPHA]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BlendMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("MIX", "BLEND_MODE_MIX", BlendMode::MIX), crate::meta::inspect::EnumConstant::new("ADD", "BLEND_MODE_ADD", BlendMode::ADD), crate::meta::inspect::EnumConstant::new("SUB", "BLEND_MODE_SUB", BlendMode::SUB), crate::meta::inspect::EnumConstant::new("MUL", "BLEND_MODE_MUL", BlendMode::MUL), crate::meta::inspect::EnumConstant::new("PREMULT_ALPHA", "BLEND_MODE_PREMULT_ALPHA", BlendMode::PREMULT_ALPHA)]
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AlphaAntiAliasing {
    ord: i32
}
impl AlphaAntiAliasing {
    #[doc(alias = "ALPHA_ANTIALIASING_OFF")]
    #[doc = "Godot enumerator name: `ALPHA_ANTIALIASING_OFF`"]
    pub const OFF: AlphaAntiAliasing = AlphaAntiAliasing {
        ord: 0i32
    };
    #[doc(alias = "ALPHA_ANTIALIASING_ALPHA_TO_COVERAGE")]
    #[doc = "Godot enumerator name: `ALPHA_ANTIALIASING_ALPHA_TO_COVERAGE`"]
    pub const ALPHA_TO_COVERAGE: AlphaAntiAliasing = AlphaAntiAliasing {
        ord: 1i32
    };
    #[doc(alias = "ALPHA_ANTIALIASING_ALPHA_TO_COVERAGE_AND_TO_ONE")]
    #[doc = "Godot enumerator name: `ALPHA_ANTIALIASING_ALPHA_TO_COVERAGE_AND_TO_ONE`"]
    pub const ALPHA_TO_COVERAGE_AND_TO_ONE: AlphaAntiAliasing = AlphaAntiAliasing {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AlphaAntiAliasing {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AlphaAntiAliasing") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AlphaAntiAliasing {
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
            Self::OFF => "OFF", Self::ALPHA_TO_COVERAGE => "ALPHA_TO_COVERAGE", Self::ALPHA_TO_COVERAGE_AND_TO_ONE => "ALPHA_TO_COVERAGE_AND_TO_ONE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AlphaAntiAliasing::OFF, AlphaAntiAliasing::ALPHA_TO_COVERAGE, AlphaAntiAliasing::ALPHA_TO_COVERAGE_AND_TO_ONE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AlphaAntiAliasing >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OFF", "ALPHA_ANTIALIASING_OFF", AlphaAntiAliasing::OFF), crate::meta::inspect::EnumConstant::new("ALPHA_TO_COVERAGE", "ALPHA_ANTIALIASING_ALPHA_TO_COVERAGE", AlphaAntiAliasing::ALPHA_TO_COVERAGE), crate::meta::inspect::EnumConstant::new("ALPHA_TO_COVERAGE_AND_TO_ONE", "ALPHA_ANTIALIASING_ALPHA_TO_COVERAGE_AND_TO_ONE", AlphaAntiAliasing::ALPHA_TO_COVERAGE_AND_TO_ONE)]
        }
    }
}
impl crate::meta::GodotConvert for AlphaAntiAliasing {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AlphaAntiAliasing {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AlphaAntiAliasing {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DepthDrawMode {
    ord: i32
}
impl DepthDrawMode {
    #[doc(alias = "DEPTH_DRAW_OPAQUE_ONLY")]
    #[doc = "Godot enumerator name: `DEPTH_DRAW_OPAQUE_ONLY`"]
    pub const OPAQUE_ONLY: DepthDrawMode = DepthDrawMode {
        ord: 0i32
    };
    #[doc(alias = "DEPTH_DRAW_ALWAYS")]
    #[doc = "Godot enumerator name: `DEPTH_DRAW_ALWAYS`"]
    pub const ALWAYS: DepthDrawMode = DepthDrawMode {
        ord: 1i32
    };
    #[doc(alias = "DEPTH_DRAW_DISABLED")]
    #[doc = "Godot enumerator name: `DEPTH_DRAW_DISABLED`"]
    pub const DISABLED: DepthDrawMode = DepthDrawMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DepthDrawMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DepthDrawMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DepthDrawMode {
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
            Self::OPAQUE_ONLY => "OPAQUE_ONLY", Self::ALWAYS => "ALWAYS", Self::DISABLED => "DISABLED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DepthDrawMode::OPAQUE_ONLY, DepthDrawMode::ALWAYS, DepthDrawMode::DISABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DepthDrawMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OPAQUE_ONLY", "DEPTH_DRAW_OPAQUE_ONLY", DepthDrawMode::OPAQUE_ONLY), crate::meta::inspect::EnumConstant::new("ALWAYS", "DEPTH_DRAW_ALWAYS", DepthDrawMode::ALWAYS), crate::meta::inspect::EnumConstant::new("DISABLED", "DEPTH_DRAW_DISABLED", DepthDrawMode::DISABLED)]
        }
    }
}
impl crate::meta::GodotConvert for DepthDrawMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DepthDrawMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DepthDrawMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DepthTest {
    ord: i32
}
impl DepthTest {
    #[doc(alias = "DEPTH_TEST_DEFAULT")]
    #[doc = "Godot enumerator name: `DEPTH_TEST_DEFAULT`"]
    pub const DEFAULT: DepthTest = DepthTest {
        ord: 0i32
    };
    #[doc(alias = "DEPTH_TEST_INVERTED")]
    #[doc = "Godot enumerator name: `DEPTH_TEST_INVERTED`"]
    pub const INVERTED: DepthTest = DepthTest {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for DepthTest {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DepthTest") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DepthTest {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::DEFAULT => "DEFAULT", Self::INVERTED => "INVERTED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DepthTest::DEFAULT, DepthTest::INVERTED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DepthTest >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "DEPTH_TEST_DEFAULT", DepthTest::DEFAULT), crate::meta::inspect::EnumConstant::new("INVERTED", "DEPTH_TEST_INVERTED", DepthTest::INVERTED)]
        }
    }
}
impl crate::meta::GodotConvert for DepthTest {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DepthTest {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DepthTest {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CullMode {
    ord: i32
}
impl CullMode {
    #[doc(alias = "CULL_BACK")]
    #[doc = "Godot enumerator name: `CULL_BACK`"]
    pub const BACK: CullMode = CullMode {
        ord: 0i32
    };
    #[doc(alias = "CULL_FRONT")]
    #[doc = "Godot enumerator name: `CULL_FRONT`"]
    pub const FRONT: CullMode = CullMode {
        ord: 1i32
    };
    #[doc(alias = "CULL_DISABLED")]
    #[doc = "Godot enumerator name: `CULL_DISABLED`"]
    pub const DISABLED: CullMode = CullMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for CullMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CullMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CullMode {
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
            Self::BACK => "BACK", Self::FRONT => "FRONT", Self::DISABLED => "DISABLED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CullMode::BACK, CullMode::FRONT, CullMode::DISABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CullMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BACK", "CULL_BACK", CullMode::BACK), crate::meta::inspect::EnumConstant::new("FRONT", "CULL_FRONT", CullMode::FRONT), crate::meta::inspect::EnumConstant::new("DISABLED", "CULL_DISABLED", CullMode::DISABLED)]
        }
    }
}
impl crate::meta::GodotConvert for CullMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CullMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CullMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Flags {
    ord: i32
}
impl Flags {
    #[doc(alias = "FLAG_DISABLE_DEPTH_TEST")]
    #[doc = "Godot enumerator name: `FLAG_DISABLE_DEPTH_TEST`"]
    pub const DISABLE_DEPTH_TEST: Flags = Flags {
        ord: 0i32
    };
    #[doc(alias = "FLAG_ALBEDO_FROM_VERTEX_COLOR")]
    #[doc = "Godot enumerator name: `FLAG_ALBEDO_FROM_VERTEX_COLOR`"]
    pub const ALBEDO_FROM_VERTEX_COLOR: Flags = Flags {
        ord: 1i32
    };
    #[doc(alias = "FLAG_SRGB_VERTEX_COLOR")]
    #[doc = "Godot enumerator name: `FLAG_SRGB_VERTEX_COLOR`"]
    pub const SRGB_VERTEX_COLOR: Flags = Flags {
        ord: 2i32
    };
    #[doc(alias = "FLAG_USE_POINT_SIZE")]
    #[doc = "Godot enumerator name: `FLAG_USE_POINT_SIZE`"]
    pub const USE_POINT_SIZE: Flags = Flags {
        ord: 3i32
    };
    #[doc(alias = "FLAG_FIXED_SIZE")]
    #[doc = "Godot enumerator name: `FLAG_FIXED_SIZE`"]
    pub const FIXED_SIZE: Flags = Flags {
        ord: 4i32
    };
    #[doc(alias = "FLAG_BILLBOARD_KEEP_SCALE")]
    #[doc = "Godot enumerator name: `FLAG_BILLBOARD_KEEP_SCALE`"]
    pub const BILLBOARD_KEEP_SCALE: Flags = Flags {
        ord: 5i32
    };
    #[doc(alias = "FLAG_UV1_USE_TRIPLANAR")]
    #[doc = "Godot enumerator name: `FLAG_UV1_USE_TRIPLANAR`"]
    pub const UV1_USE_TRIPLANAR: Flags = Flags {
        ord: 6i32
    };
    #[doc(alias = "FLAG_UV2_USE_TRIPLANAR")]
    #[doc = "Godot enumerator name: `FLAG_UV2_USE_TRIPLANAR`"]
    pub const UV2_USE_TRIPLANAR: Flags = Flags {
        ord: 7i32
    };
    #[doc(alias = "FLAG_UV1_USE_WORLD_TRIPLANAR")]
    #[doc = "Godot enumerator name: `FLAG_UV1_USE_WORLD_TRIPLANAR`"]
    pub const UV1_USE_WORLD_TRIPLANAR: Flags = Flags {
        ord: 8i32
    };
    #[doc(alias = "FLAG_UV2_USE_WORLD_TRIPLANAR")]
    #[doc = "Godot enumerator name: `FLAG_UV2_USE_WORLD_TRIPLANAR`"]
    pub const UV2_USE_WORLD_TRIPLANAR: Flags = Flags {
        ord: 9i32
    };
    #[doc(alias = "FLAG_AO_ON_UV2")]
    #[doc = "Godot enumerator name: `FLAG_AO_ON_UV2`"]
    pub const AO_ON_UV2: Flags = Flags {
        ord: 10i32
    };
    #[doc(alias = "FLAG_EMISSION_ON_UV2")]
    #[doc = "Godot enumerator name: `FLAG_EMISSION_ON_UV2`"]
    pub const EMISSION_ON_UV2: Flags = Flags {
        ord: 11i32
    };
    #[doc(alias = "FLAG_ALBEDO_TEXTURE_FORCE_SRGB")]
    #[doc = "Godot enumerator name: `FLAG_ALBEDO_TEXTURE_FORCE_SRGB`"]
    pub const ALBEDO_TEXTURE_FORCE_SRGB: Flags = Flags {
        ord: 12i32
    };
    #[doc(alias = "FLAG_DONT_RECEIVE_SHADOWS")]
    #[doc = "Godot enumerator name: `FLAG_DONT_RECEIVE_SHADOWS`"]
    pub const DONT_RECEIVE_SHADOWS: Flags = Flags {
        ord: 13i32
    };
    #[doc(alias = "FLAG_DISABLE_AMBIENT_LIGHT")]
    #[doc = "Godot enumerator name: `FLAG_DISABLE_AMBIENT_LIGHT`"]
    pub const DISABLE_AMBIENT_LIGHT: Flags = Flags {
        ord: 14i32
    };
    #[doc(alias = "FLAG_USE_SHADOW_TO_OPACITY")]
    #[doc = "Godot enumerator name: `FLAG_USE_SHADOW_TO_OPACITY`"]
    pub const USE_SHADOW_TO_OPACITY: Flags = Flags {
        ord: 15i32
    };
    #[doc(alias = "FLAG_USE_TEXTURE_REPEAT")]
    #[doc = "Godot enumerator name: `FLAG_USE_TEXTURE_REPEAT`"]
    pub const USE_TEXTURE_REPEAT: Flags = Flags {
        ord: 16i32
    };
    #[doc(alias = "FLAG_INVERT_HEIGHTMAP")]
    #[doc = "Godot enumerator name: `FLAG_INVERT_HEIGHTMAP`"]
    pub const INVERT_HEIGHTMAP: Flags = Flags {
        ord: 17i32
    };
    #[doc(alias = "FLAG_SUBSURFACE_MODE_SKIN")]
    #[doc = "Godot enumerator name: `FLAG_SUBSURFACE_MODE_SKIN`"]
    pub const SUBSURFACE_MODE_SKIN: Flags = Flags {
        ord: 18i32
    };
    #[doc(alias = "FLAG_PARTICLE_TRAILS_MODE")]
    #[doc = "Godot enumerator name: `FLAG_PARTICLE_TRAILS_MODE`"]
    pub const PARTICLE_TRAILS_MODE: Flags = Flags {
        ord: 19i32
    };
    #[doc(alias = "FLAG_ALBEDO_TEXTURE_MSDF")]
    #[doc = "Godot enumerator name: `FLAG_ALBEDO_TEXTURE_MSDF`"]
    pub const ALBEDO_TEXTURE_MSDF: Flags = Flags {
        ord: 20i32
    };
    #[doc(alias = "FLAG_DISABLE_FOG")]
    #[doc = "Godot enumerator name: `FLAG_DISABLE_FOG`"]
    pub const DISABLE_FOG: Flags = Flags {
        ord: 21i32
    };
    #[doc(alias = "FLAG_DISABLE_SPECULAR_OCCLUSION")]
    #[doc = "Godot enumerator name: `FLAG_DISABLE_SPECULAR_OCCLUSION`"]
    pub const DISABLE_SPECULAR_OCCLUSION: Flags = Flags {
        ord: 22i32
    };
    #[doc(alias = "FLAG_USE_Z_CLIP_SCALE")]
    #[doc = "Godot enumerator name: `FLAG_USE_Z_CLIP_SCALE`"]
    pub const USE_Z_CLIP_SCALE: Flags = Flags {
        ord: 23i32
    };
    #[doc(alias = "FLAG_USE_FOV_OVERRIDE")]
    #[doc = "Godot enumerator name: `FLAG_USE_FOV_OVERRIDE`"]
    pub const USE_FOV_OVERRIDE: Flags = Flags {
        ord: 24i32
    };
    #[doc(alias = "FLAG_MAX")]
    #[doc = "Godot enumerator name: `FLAG_MAX`"]
    pub const MAX: Flags = Flags {
        ord: 25i32
    };
    
}
impl std::fmt::Debug for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Flags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Flags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 => Some(Self {
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
            Self::DISABLE_DEPTH_TEST => "DISABLE_DEPTH_TEST", Self::ALBEDO_FROM_VERTEX_COLOR => "ALBEDO_FROM_VERTEX_COLOR", Self::SRGB_VERTEX_COLOR => "SRGB_VERTEX_COLOR", Self::USE_POINT_SIZE => "USE_POINT_SIZE", Self::FIXED_SIZE => "FIXED_SIZE", Self::BILLBOARD_KEEP_SCALE => "BILLBOARD_KEEP_SCALE", Self::UV1_USE_TRIPLANAR => "UV1_USE_TRIPLANAR", Self::UV2_USE_TRIPLANAR => "UV2_USE_TRIPLANAR", Self::UV1_USE_WORLD_TRIPLANAR => "UV1_USE_WORLD_TRIPLANAR", Self::UV2_USE_WORLD_TRIPLANAR => "UV2_USE_WORLD_TRIPLANAR", Self::AO_ON_UV2 => "AO_ON_UV2", Self::EMISSION_ON_UV2 => "EMISSION_ON_UV2", Self::ALBEDO_TEXTURE_FORCE_SRGB => "ALBEDO_TEXTURE_FORCE_SRGB", Self::DONT_RECEIVE_SHADOWS => "DONT_RECEIVE_SHADOWS", Self::DISABLE_AMBIENT_LIGHT => "DISABLE_AMBIENT_LIGHT", Self::USE_SHADOW_TO_OPACITY => "USE_SHADOW_TO_OPACITY", Self::USE_TEXTURE_REPEAT => "USE_TEXTURE_REPEAT", Self::INVERT_HEIGHTMAP => "INVERT_HEIGHTMAP", Self::SUBSURFACE_MODE_SKIN => "SUBSURFACE_MODE_SKIN", Self::PARTICLE_TRAILS_MODE => "PARTICLE_TRAILS_MODE", Self::ALBEDO_TEXTURE_MSDF => "ALBEDO_TEXTURE_MSDF", Self::DISABLE_FOG => "DISABLE_FOG", Self::DISABLE_SPECULAR_OCCLUSION => "DISABLE_SPECULAR_OCCLUSION", Self::USE_Z_CLIP_SCALE => "USE_Z_CLIP_SCALE", Self::USE_FOV_OVERRIDE => "USE_FOV_OVERRIDE", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Flags::DISABLE_DEPTH_TEST, Flags::ALBEDO_FROM_VERTEX_COLOR, Flags::SRGB_VERTEX_COLOR, Flags::USE_POINT_SIZE, Flags::FIXED_SIZE, Flags::BILLBOARD_KEEP_SCALE, Flags::UV1_USE_TRIPLANAR, Flags::UV2_USE_TRIPLANAR, Flags::UV1_USE_WORLD_TRIPLANAR, Flags::UV2_USE_WORLD_TRIPLANAR, Flags::AO_ON_UV2, Flags::EMISSION_ON_UV2, Flags::ALBEDO_TEXTURE_FORCE_SRGB, Flags::DONT_RECEIVE_SHADOWS, Flags::DISABLE_AMBIENT_LIGHT, Flags::USE_SHADOW_TO_OPACITY, Flags::USE_TEXTURE_REPEAT, Flags::INVERT_HEIGHTMAP, Flags::SUBSURFACE_MODE_SKIN, Flags::PARTICLE_TRAILS_MODE, Flags::ALBEDO_TEXTURE_MSDF, Flags::DISABLE_FOG, Flags::DISABLE_SPECULAR_OCCLUSION, Flags::USE_Z_CLIP_SCALE, Flags::USE_FOV_OVERRIDE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Flags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLE_DEPTH_TEST", "FLAG_DISABLE_DEPTH_TEST", Flags::DISABLE_DEPTH_TEST), crate::meta::inspect::EnumConstant::new("ALBEDO_FROM_VERTEX_COLOR", "FLAG_ALBEDO_FROM_VERTEX_COLOR", Flags::ALBEDO_FROM_VERTEX_COLOR), crate::meta::inspect::EnumConstant::new("SRGB_VERTEX_COLOR", "FLAG_SRGB_VERTEX_COLOR", Flags::SRGB_VERTEX_COLOR), crate::meta::inspect::EnumConstant::new("USE_POINT_SIZE", "FLAG_USE_POINT_SIZE", Flags::USE_POINT_SIZE), crate::meta::inspect::EnumConstant::new("FIXED_SIZE", "FLAG_FIXED_SIZE", Flags::FIXED_SIZE), crate::meta::inspect::EnumConstant::new("BILLBOARD_KEEP_SCALE", "FLAG_BILLBOARD_KEEP_SCALE", Flags::BILLBOARD_KEEP_SCALE), crate::meta::inspect::EnumConstant::new("UV1_USE_TRIPLANAR", "FLAG_UV1_USE_TRIPLANAR", Flags::UV1_USE_TRIPLANAR), crate::meta::inspect::EnumConstant::new("UV2_USE_TRIPLANAR", "FLAG_UV2_USE_TRIPLANAR", Flags::UV2_USE_TRIPLANAR), crate::meta::inspect::EnumConstant::new("UV1_USE_WORLD_TRIPLANAR", "FLAG_UV1_USE_WORLD_TRIPLANAR", Flags::UV1_USE_WORLD_TRIPLANAR), crate::meta::inspect::EnumConstant::new("UV2_USE_WORLD_TRIPLANAR", "FLAG_UV2_USE_WORLD_TRIPLANAR", Flags::UV2_USE_WORLD_TRIPLANAR), crate::meta::inspect::EnumConstant::new("AO_ON_UV2", "FLAG_AO_ON_UV2", Flags::AO_ON_UV2), crate::meta::inspect::EnumConstant::new("EMISSION_ON_UV2", "FLAG_EMISSION_ON_UV2", Flags::EMISSION_ON_UV2), crate::meta::inspect::EnumConstant::new("ALBEDO_TEXTURE_FORCE_SRGB", "FLAG_ALBEDO_TEXTURE_FORCE_SRGB", Flags::ALBEDO_TEXTURE_FORCE_SRGB), crate::meta::inspect::EnumConstant::new("DONT_RECEIVE_SHADOWS", "FLAG_DONT_RECEIVE_SHADOWS", Flags::DONT_RECEIVE_SHADOWS), crate::meta::inspect::EnumConstant::new("DISABLE_AMBIENT_LIGHT", "FLAG_DISABLE_AMBIENT_LIGHT", Flags::DISABLE_AMBIENT_LIGHT), crate::meta::inspect::EnumConstant::new("USE_SHADOW_TO_OPACITY", "FLAG_USE_SHADOW_TO_OPACITY", Flags::USE_SHADOW_TO_OPACITY), crate::meta::inspect::EnumConstant::new("USE_TEXTURE_REPEAT", "FLAG_USE_TEXTURE_REPEAT", Flags::USE_TEXTURE_REPEAT), crate::meta::inspect::EnumConstant::new("INVERT_HEIGHTMAP", "FLAG_INVERT_HEIGHTMAP", Flags::INVERT_HEIGHTMAP), crate::meta::inspect::EnumConstant::new("SUBSURFACE_MODE_SKIN", "FLAG_SUBSURFACE_MODE_SKIN", Flags::SUBSURFACE_MODE_SKIN), crate::meta::inspect::EnumConstant::new("PARTICLE_TRAILS_MODE", "FLAG_PARTICLE_TRAILS_MODE", Flags::PARTICLE_TRAILS_MODE), crate::meta::inspect::EnumConstant::new("ALBEDO_TEXTURE_MSDF", "FLAG_ALBEDO_TEXTURE_MSDF", Flags::ALBEDO_TEXTURE_MSDF), crate::meta::inspect::EnumConstant::new("DISABLE_FOG", "FLAG_DISABLE_FOG", Flags::DISABLE_FOG), crate::meta::inspect::EnumConstant::new("DISABLE_SPECULAR_OCCLUSION", "FLAG_DISABLE_SPECULAR_OCCLUSION", Flags::DISABLE_SPECULAR_OCCLUSION), crate::meta::inspect::EnumConstant::new("USE_Z_CLIP_SCALE", "FLAG_USE_Z_CLIP_SCALE", Flags::USE_Z_CLIP_SCALE), crate::meta::inspect::EnumConstant::new("USE_FOV_OVERRIDE", "FLAG_USE_FOV_OVERRIDE", Flags::USE_FOV_OVERRIDE), crate::meta::inspect::EnumConstant::new("MAX", "FLAG_MAX", Flags::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Flags {
    const ENUMERATOR_COUNT: usize = 25usize;
    
}
impl crate::meta::GodotConvert for Flags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Flags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Flags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DiffuseMode {
    ord: i32
}
impl DiffuseMode {
    #[doc(alias = "DIFFUSE_BURLEY")]
    #[doc = "Godot enumerator name: `DIFFUSE_BURLEY`"]
    pub const BURLEY: DiffuseMode = DiffuseMode {
        ord: 0i32
    };
    #[doc(alias = "DIFFUSE_LAMBERT")]
    #[doc = "Godot enumerator name: `DIFFUSE_LAMBERT`"]
    pub const LAMBERT: DiffuseMode = DiffuseMode {
        ord: 1i32
    };
    #[doc(alias = "DIFFUSE_LAMBERT_WRAP")]
    #[doc = "Godot enumerator name: `DIFFUSE_LAMBERT_WRAP`"]
    pub const LAMBERT_WRAP: DiffuseMode = DiffuseMode {
        ord: 2i32
    };
    #[doc(alias = "DIFFUSE_TOON")]
    #[doc = "Godot enumerator name: `DIFFUSE_TOON`"]
    pub const TOON: DiffuseMode = DiffuseMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for DiffuseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DiffuseMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DiffuseMode {
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
            Self::BURLEY => "BURLEY", Self::LAMBERT => "LAMBERT", Self::LAMBERT_WRAP => "LAMBERT_WRAP", Self::TOON => "TOON", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DiffuseMode::BURLEY, DiffuseMode::LAMBERT, DiffuseMode::LAMBERT_WRAP, DiffuseMode::TOON]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DiffuseMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BURLEY", "DIFFUSE_BURLEY", DiffuseMode::BURLEY), crate::meta::inspect::EnumConstant::new("LAMBERT", "DIFFUSE_LAMBERT", DiffuseMode::LAMBERT), crate::meta::inspect::EnumConstant::new("LAMBERT_WRAP", "DIFFUSE_LAMBERT_WRAP", DiffuseMode::LAMBERT_WRAP), crate::meta::inspect::EnumConstant::new("TOON", "DIFFUSE_TOON", DiffuseMode::TOON)]
        }
    }
}
impl crate::meta::GodotConvert for DiffuseMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DiffuseMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DiffuseMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpecularMode {
    ord: i32
}
impl SpecularMode {
    #[doc(alias = "SPECULAR_SCHLICK_GGX")]
    #[doc = "Godot enumerator name: `SPECULAR_SCHLICK_GGX`"]
    pub const SCHLICK_GGX: SpecularMode = SpecularMode {
        ord: 0i32
    };
    #[doc(alias = "SPECULAR_TOON")]
    #[doc = "Godot enumerator name: `SPECULAR_TOON`"]
    pub const TOON: SpecularMode = SpecularMode {
        ord: 1i32
    };
    #[doc(alias = "SPECULAR_DISABLED")]
    #[doc = "Godot enumerator name: `SPECULAR_DISABLED`"]
    pub const DISABLED: SpecularMode = SpecularMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for SpecularMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SpecularMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SpecularMode {
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
            Self::SCHLICK_GGX => "SCHLICK_GGX", Self::TOON => "TOON", Self::DISABLED => "DISABLED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SpecularMode::SCHLICK_GGX, SpecularMode::TOON, SpecularMode::DISABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SpecularMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SCHLICK_GGX", "SPECULAR_SCHLICK_GGX", SpecularMode::SCHLICK_GGX), crate::meta::inspect::EnumConstant::new("TOON", "SPECULAR_TOON", SpecularMode::TOON), crate::meta::inspect::EnumConstant::new("DISABLED", "SPECULAR_DISABLED", SpecularMode::DISABLED)]
        }
    }
}
impl crate::meta::GodotConvert for SpecularMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SpecularMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SpecularMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BillboardMode {
    ord: i32
}
impl BillboardMode {
    #[doc(alias = "BILLBOARD_DISABLED")]
    #[doc = "Godot enumerator name: `BILLBOARD_DISABLED`"]
    pub const DISABLED: BillboardMode = BillboardMode {
        ord: 0i32
    };
    #[doc(alias = "BILLBOARD_ENABLED")]
    #[doc = "Godot enumerator name: `BILLBOARD_ENABLED`"]
    pub const ENABLED: BillboardMode = BillboardMode {
        ord: 1i32
    };
    #[doc(alias = "BILLBOARD_FIXED_Y")]
    #[doc = "Godot enumerator name: `BILLBOARD_FIXED_Y`"]
    pub const FIXED_Y: BillboardMode = BillboardMode {
        ord: 2i32
    };
    #[doc(alias = "BILLBOARD_PARTICLES")]
    #[doc = "Godot enumerator name: `BILLBOARD_PARTICLES`"]
    pub const PARTICLES: BillboardMode = BillboardMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for BillboardMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BillboardMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BillboardMode {
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
            Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::FIXED_Y => "FIXED_Y", Self::PARTICLES => "PARTICLES", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BillboardMode::DISABLED, BillboardMode::ENABLED, BillboardMode::FIXED_Y, BillboardMode::PARTICLES]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BillboardMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "BILLBOARD_DISABLED", BillboardMode::DISABLED), crate::meta::inspect::EnumConstant::new("ENABLED", "BILLBOARD_ENABLED", BillboardMode::ENABLED), crate::meta::inspect::EnumConstant::new("FIXED_Y", "BILLBOARD_FIXED_Y", BillboardMode::FIXED_Y), crate::meta::inspect::EnumConstant::new("PARTICLES", "BILLBOARD_PARTICLES", BillboardMode::PARTICLES)]
        }
    }
}
impl crate::meta::GodotConvert for BillboardMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BillboardMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BillboardMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureChannel {
    ord: i32
}
impl TextureChannel {
    #[doc(alias = "TEXTURE_CHANNEL_RED")]
    #[doc = "Godot enumerator name: `TEXTURE_CHANNEL_RED`"]
    pub const RED: TextureChannel = TextureChannel {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_CHANNEL_GREEN")]
    #[doc = "Godot enumerator name: `TEXTURE_CHANNEL_GREEN`"]
    pub const GREEN: TextureChannel = TextureChannel {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_CHANNEL_BLUE")]
    #[doc = "Godot enumerator name: `TEXTURE_CHANNEL_BLUE`"]
    pub const BLUE: TextureChannel = TextureChannel {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_CHANNEL_ALPHA")]
    #[doc = "Godot enumerator name: `TEXTURE_CHANNEL_ALPHA`"]
    pub const ALPHA: TextureChannel = TextureChannel {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_CHANNEL_GRAYSCALE")]
    #[doc = "Godot enumerator name: `TEXTURE_CHANNEL_GRAYSCALE`"]
    pub const GRAYSCALE: TextureChannel = TextureChannel {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for TextureChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureChannel") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureChannel {
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
            Self::RED => "RED", Self::GREEN => "GREEN", Self::BLUE => "BLUE", Self::ALPHA => "ALPHA", Self::GRAYSCALE => "GRAYSCALE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureChannel::RED, TextureChannel::GREEN, TextureChannel::BLUE, TextureChannel::ALPHA, TextureChannel::GRAYSCALE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureChannel >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RED", "TEXTURE_CHANNEL_RED", TextureChannel::RED), crate::meta::inspect::EnumConstant::new("GREEN", "TEXTURE_CHANNEL_GREEN", TextureChannel::GREEN), crate::meta::inspect::EnumConstant::new("BLUE", "TEXTURE_CHANNEL_BLUE", TextureChannel::BLUE), crate::meta::inspect::EnumConstant::new("ALPHA", "TEXTURE_CHANNEL_ALPHA", TextureChannel::ALPHA), crate::meta::inspect::EnumConstant::new("GRAYSCALE", "TEXTURE_CHANNEL_GRAYSCALE", TextureChannel::GRAYSCALE)]
        }
    }
}
impl crate::meta::GodotConvert for TextureChannel {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureChannel {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureChannel {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EmissionOperator {
    ord: i32
}
impl EmissionOperator {
    #[doc(alias = "EMISSION_OP_ADD")]
    #[doc = "Godot enumerator name: `EMISSION_OP_ADD`"]
    pub const ADD: EmissionOperator = EmissionOperator {
        ord: 0i32
    };
    #[doc(alias = "EMISSION_OP_MULTIPLY")]
    #[doc = "Godot enumerator name: `EMISSION_OP_MULTIPLY`"]
    pub const MULTIPLY: EmissionOperator = EmissionOperator {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for EmissionOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EmissionOperator") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EmissionOperator {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::ADD => "ADD", Self::MULTIPLY => "MULTIPLY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EmissionOperator::ADD, EmissionOperator::MULTIPLY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EmissionOperator >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ADD", "EMISSION_OP_ADD", EmissionOperator::ADD), crate::meta::inspect::EnumConstant::new("MULTIPLY", "EMISSION_OP_MULTIPLY", EmissionOperator::MULTIPLY)]
        }
    }
}
impl crate::meta::GodotConvert for EmissionOperator {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EmissionOperator {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EmissionOperator {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DistanceFadeMode {
    ord: i32
}
impl DistanceFadeMode {
    #[doc(alias = "DISTANCE_FADE_DISABLED")]
    #[doc = "Godot enumerator name: `DISTANCE_FADE_DISABLED`"]
    pub const DISABLED: DistanceFadeMode = DistanceFadeMode {
        ord: 0i32
    };
    #[doc(alias = "DISTANCE_FADE_PIXEL_ALPHA")]
    #[doc = "Godot enumerator name: `DISTANCE_FADE_PIXEL_ALPHA`"]
    pub const PIXEL_ALPHA: DistanceFadeMode = DistanceFadeMode {
        ord: 1i32
    };
    #[doc(alias = "DISTANCE_FADE_PIXEL_DITHER")]
    #[doc = "Godot enumerator name: `DISTANCE_FADE_PIXEL_DITHER`"]
    pub const PIXEL_DITHER: DistanceFadeMode = DistanceFadeMode {
        ord: 2i32
    };
    #[doc(alias = "DISTANCE_FADE_OBJECT_DITHER")]
    #[doc = "Godot enumerator name: `DISTANCE_FADE_OBJECT_DITHER`"]
    pub const OBJECT_DITHER: DistanceFadeMode = DistanceFadeMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for DistanceFadeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DistanceFadeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DistanceFadeMode {
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
            Self::DISABLED => "DISABLED", Self::PIXEL_ALPHA => "PIXEL_ALPHA", Self::PIXEL_DITHER => "PIXEL_DITHER", Self::OBJECT_DITHER => "OBJECT_DITHER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DistanceFadeMode::DISABLED, DistanceFadeMode::PIXEL_ALPHA, DistanceFadeMode::PIXEL_DITHER, DistanceFadeMode::OBJECT_DITHER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DistanceFadeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "DISTANCE_FADE_DISABLED", DistanceFadeMode::DISABLED), crate::meta::inspect::EnumConstant::new("PIXEL_ALPHA", "DISTANCE_FADE_PIXEL_ALPHA", DistanceFadeMode::PIXEL_ALPHA), crate::meta::inspect::EnumConstant::new("PIXEL_DITHER", "DISTANCE_FADE_PIXEL_DITHER", DistanceFadeMode::PIXEL_DITHER), crate::meta::inspect::EnumConstant::new("OBJECT_DITHER", "DISTANCE_FADE_OBJECT_DITHER", DistanceFadeMode::OBJECT_DITHER)]
        }
    }
}
impl crate::meta::GodotConvert for DistanceFadeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DistanceFadeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DistanceFadeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct StencilMode {
    ord: i32
}
impl StencilMode {
    #[doc(alias = "STENCIL_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `STENCIL_MODE_DISABLED`"]
    pub const DISABLED: StencilMode = StencilMode {
        ord: 0i32
    };
    #[doc(alias = "STENCIL_MODE_OUTLINE")]
    #[doc = "Godot enumerator name: `STENCIL_MODE_OUTLINE`"]
    pub const OUTLINE: StencilMode = StencilMode {
        ord: 1i32
    };
    #[doc(alias = "STENCIL_MODE_XRAY")]
    #[doc = "Godot enumerator name: `STENCIL_MODE_XRAY`"]
    pub const XRAY: StencilMode = StencilMode {
        ord: 2i32
    };
    #[doc(alias = "STENCIL_MODE_CUSTOM")]
    #[doc = "Godot enumerator name: `STENCIL_MODE_CUSTOM`"]
    pub const CUSTOM: StencilMode = StencilMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for StencilMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("StencilMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for StencilMode {
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
            Self::DISABLED => "DISABLED", Self::OUTLINE => "OUTLINE", Self::XRAY => "XRAY", Self::CUSTOM => "CUSTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[StencilMode::DISABLED, StencilMode::OUTLINE, StencilMode::XRAY, StencilMode::CUSTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < StencilMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "STENCIL_MODE_DISABLED", StencilMode::DISABLED), crate::meta::inspect::EnumConstant::new("OUTLINE", "STENCIL_MODE_OUTLINE", StencilMode::OUTLINE), crate::meta::inspect::EnumConstant::new("XRAY", "STENCIL_MODE_XRAY", StencilMode::XRAY), crate::meta::inspect::EnumConstant::new("CUSTOM", "STENCIL_MODE_CUSTOM", StencilMode::CUSTOM)]
        }
    }
}
impl crate::meta::GodotConvert for StencilMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for StencilMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for StencilMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct StencilFlags {
    ord: i32
}
impl StencilFlags {
    #[doc(alias = "STENCIL_FLAG_READ")]
    #[doc = "Godot enumerator name: `STENCIL_FLAG_READ`"]
    pub const READ: StencilFlags = StencilFlags {
        ord: 1i32
    };
    #[doc(alias = "STENCIL_FLAG_WRITE")]
    #[doc = "Godot enumerator name: `STENCIL_FLAG_WRITE`"]
    pub const WRITE: StencilFlags = StencilFlags {
        ord: 2i32
    };
    #[doc(alias = "STENCIL_FLAG_WRITE_DEPTH_FAIL")]
    #[doc = "Godot enumerator name: `STENCIL_FLAG_WRITE_DEPTH_FAIL`"]
    pub const WRITE_DEPTH_FAIL: StencilFlags = StencilFlags {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for StencilFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("StencilFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for StencilFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 => Some(Self {
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
            Self::READ => "READ", Self::WRITE => "WRITE", Self::WRITE_DEPTH_FAIL => "WRITE_DEPTH_FAIL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[StencilFlags::READ, StencilFlags::WRITE, StencilFlags::WRITE_DEPTH_FAIL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < StencilFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("READ", "STENCIL_FLAG_READ", StencilFlags::READ), crate::meta::inspect::EnumConstant::new("WRITE", "STENCIL_FLAG_WRITE", StencilFlags::WRITE), crate::meta::inspect::EnumConstant::new("WRITE_DEPTH_FAIL", "STENCIL_FLAG_WRITE_DEPTH_FAIL", StencilFlags::WRITE_DEPTH_FAIL)]
        }
    }
}
impl crate::meta::GodotConvert for StencilFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for StencilFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for StencilFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct StencilCompare {
    ord: i32
}
impl StencilCompare {
    #[doc(alias = "STENCIL_COMPARE_ALWAYS")]
    #[doc = "Godot enumerator name: `STENCIL_COMPARE_ALWAYS`"]
    pub const ALWAYS: StencilCompare = StencilCompare {
        ord: 0i32
    };
    #[doc(alias = "STENCIL_COMPARE_LESS")]
    #[doc = "Godot enumerator name: `STENCIL_COMPARE_LESS`"]
    pub const LESS: StencilCompare = StencilCompare {
        ord: 1i32
    };
    #[doc(alias = "STENCIL_COMPARE_EQUAL")]
    #[doc = "Godot enumerator name: `STENCIL_COMPARE_EQUAL`"]
    pub const EQUAL: StencilCompare = StencilCompare {
        ord: 2i32
    };
    #[doc(alias = "STENCIL_COMPARE_LESS_OR_EQUAL")]
    #[doc = "Godot enumerator name: `STENCIL_COMPARE_LESS_OR_EQUAL`"]
    pub const LESS_OR_EQUAL: StencilCompare = StencilCompare {
        ord: 3i32
    };
    #[doc(alias = "STENCIL_COMPARE_GREATER")]
    #[doc = "Godot enumerator name: `STENCIL_COMPARE_GREATER`"]
    pub const GREATER: StencilCompare = StencilCompare {
        ord: 4i32
    };
    #[doc(alias = "STENCIL_COMPARE_NOT_EQUAL")]
    #[doc = "Godot enumerator name: `STENCIL_COMPARE_NOT_EQUAL`"]
    pub const NOT_EQUAL: StencilCompare = StencilCompare {
        ord: 5i32
    };
    #[doc(alias = "STENCIL_COMPARE_GREATER_OR_EQUAL")]
    #[doc = "Godot enumerator name: `STENCIL_COMPARE_GREATER_OR_EQUAL`"]
    pub const GREATER_OR_EQUAL: StencilCompare = StencilCompare {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for StencilCompare {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("StencilCompare") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for StencilCompare {
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
            Self::ALWAYS => "ALWAYS", Self::LESS => "LESS", Self::EQUAL => "EQUAL", Self::LESS_OR_EQUAL => "LESS_OR_EQUAL", Self::GREATER => "GREATER", Self::NOT_EQUAL => "NOT_EQUAL", Self::GREATER_OR_EQUAL => "GREATER_OR_EQUAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[StencilCompare::ALWAYS, StencilCompare::LESS, StencilCompare::EQUAL, StencilCompare::LESS_OR_EQUAL, StencilCompare::GREATER, StencilCompare::NOT_EQUAL, StencilCompare::GREATER_OR_EQUAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < StencilCompare >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ALWAYS", "STENCIL_COMPARE_ALWAYS", StencilCompare::ALWAYS), crate::meta::inspect::EnumConstant::new("LESS", "STENCIL_COMPARE_LESS", StencilCompare::LESS), crate::meta::inspect::EnumConstant::new("EQUAL", "STENCIL_COMPARE_EQUAL", StencilCompare::EQUAL), crate::meta::inspect::EnumConstant::new("LESS_OR_EQUAL", "STENCIL_COMPARE_LESS_OR_EQUAL", StencilCompare::LESS_OR_EQUAL), crate::meta::inspect::EnumConstant::new("GREATER", "STENCIL_COMPARE_GREATER", StencilCompare::GREATER), crate::meta::inspect::EnumConstant::new("NOT_EQUAL", "STENCIL_COMPARE_NOT_EQUAL", StencilCompare::NOT_EQUAL), crate::meta::inspect::EnumConstant::new("GREATER_OR_EQUAL", "STENCIL_COMPARE_GREATER_OR_EQUAL", StencilCompare::GREATER_OR_EQUAL)]
        }
    }
}
impl crate::meta::GodotConvert for StencilCompare {
    type Via = i32;
    
}
impl crate::meta::ToGodot for StencilCompare {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for StencilCompare {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::BaseMaterial3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for BaseMaterial3D {
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