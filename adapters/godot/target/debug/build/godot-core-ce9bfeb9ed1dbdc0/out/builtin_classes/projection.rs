use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, ClassId, CowArg, InParamTuple, OutParamTuple, ParamTuple, RefArg, Signature
};
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
#[repr(transparent)]
pub struct InnerProjection < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerProjection < 'a > {
    pub fn from_outer(outer: &Projection) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn create_depth_correction(flip_y: bool,) -> Projection {
        type CallRet = Projection;
        type CallParams = (bool,);
        let args = (flip_y,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(431usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_depth_correction", std::ptr::null_mut(), args)
        }
    }
    pub fn create_light_atlas_rect(rect: Rect2,) -> Projection {
        type CallRet = Projection;
        type CallParams = (Rect2,);
        let args = (rect,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(432usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_light_atlas_rect", std::ptr::null_mut(), args)
        }
    }
    pub fn create_perspective(fovy: f64, aspect: f64, z_near: f64, z_far: f64, flip_fov: bool,) -> Projection {
        type CallRet = Projection;
        type CallParams = (f64, f64, f64, f64, bool,);
        let args = (fovy, aspect, z_near, z_far, flip_fov,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(433usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_perspective", std::ptr::null_mut(), args)
        }
    }
    pub fn create_perspective_hmd(fovy: f64, aspect: f64, z_near: f64, z_far: f64, flip_fov: bool, eye: i64, intraocular_dist: f64, convergence_dist: f64,) -> Projection {
        type CallRet = Projection;
        type CallParams = (f64, f64, f64, f64, bool, i64, f64, f64,);
        let args = (fovy, aspect, z_near, z_far, flip_fov, eye, intraocular_dist, convergence_dist,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(434usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_perspective_hmd", std::ptr::null_mut(), args)
        }
    }
    pub fn create_for_hmd(eye: i64, aspect: f64, intraocular_dist: f64, display_width: f64, display_to_lens: f64, oversample: f64, z_near: f64, z_far: f64,) -> Projection {
        type CallRet = Projection;
        type CallParams = (i64, f64, f64, f64, f64, f64, f64, f64,);
        let args = (eye, aspect, intraocular_dist, display_width, display_to_lens, oversample, z_near, z_far,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(435usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_for_hmd", std::ptr::null_mut(), args)
        }
    }
    pub fn create_orthogonal(left: f64, right: f64, bottom: f64, top: f64, z_near: f64, z_far: f64,) -> Projection {
        type CallRet = Projection;
        type CallParams = (f64, f64, f64, f64, f64, f64,);
        let args = (left, right, bottom, top, z_near, z_far,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(436usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_orthogonal", std::ptr::null_mut(), args)
        }
    }
    pub fn create_orthogonal_aspect(size: f64, aspect: f64, z_near: f64, z_far: f64, flip_fov: bool,) -> Projection {
        type CallRet = Projection;
        type CallParams = (f64, f64, f64, f64, bool,);
        let args = (size, aspect, z_near, z_far, flip_fov,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(437usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_orthogonal_aspect", std::ptr::null_mut(), args)
        }
    }
    pub fn create_frustum(left: f64, right: f64, bottom: f64, top: f64, z_near: f64, z_far: f64,) -> Projection {
        type CallRet = Projection;
        type CallParams = (f64, f64, f64, f64, f64, f64,);
        let args = (left, right, bottom, top, z_near, z_far,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(438usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_frustum", std::ptr::null_mut(), args)
        }
    }
    pub fn create_frustum_aspect(size: f64, aspect: f64, offset: Vector2, z_near: f64, z_far: f64, flip_fov: bool,) -> Projection {
        type CallRet = Projection;
        type CallParams = (f64, f64, Vector2, f64, f64, bool,);
        let args = (size, aspect, offset, z_near, z_far, flip_fov,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(439usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_frustum_aspect", std::ptr::null_mut(), args)
        }
    }
    pub fn create_fit_aabb(aabb: Aabb,) -> Projection {
        type CallRet = Projection;
        type CallParams = (Aabb,);
        let args = (aabb,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(440usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "create_fit_aabb", std::ptr::null_mut(), args)
        }
    }
    pub fn determinant(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(441usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "determinant", self.sys_ptr, args)
        }
    }
    pub fn perspective_znear_adjusted(&self, new_znear: f64,) -> Projection {
        type CallRet = Projection;
        type CallParams = (f64,);
        let args = (new_znear,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(442usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "perspective_znear_adjusted", self.sys_ptr, args)
        }
    }
    pub fn get_projection_plane(&self, plane: i64,) -> Plane {
        type CallRet = Plane;
        type CallParams = (i64,);
        let args = (plane,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(443usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_projection_plane", self.sys_ptr, args)
        }
    }
    pub fn flipped_y(&self,) -> Projection {
        type CallRet = Projection;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(444usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "flipped_y", self.sys_ptr, args)
        }
    }
    pub fn jitter_offseted(&self, offset: Vector2,) -> Projection {
        type CallRet = Projection;
        type CallParams = (Vector2,);
        let args = (offset,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(445usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "jitter_offseted", self.sys_ptr, args)
        }
    }
    pub fn get_fovy(fovx: f64, aspect: f64,) -> f64 {
        type CallRet = f64;
        type CallParams = (f64, f64,);
        let args = (fovx, aspect,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(446usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_fovy", std::ptr::null_mut(), args)
        }
    }
    pub fn get_z_far(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(447usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_z_far", self.sys_ptr, args)
        }
    }
    pub fn get_z_near(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(448usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_z_near", self.sys_ptr, args)
        }
    }
    pub fn get_aspect(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(449usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_aspect", self.sys_ptr, args)
        }
    }
    pub fn get_fov(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(450usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_fov", self.sys_ptr, args)
        }
    }
    pub fn is_orthogonal(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(451usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "is_orthogonal", self.sys_ptr, args)
        }
    }
    pub fn get_viewport_half_extents(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(452usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_viewport_half_extents", self.sys_ptr, args)
        }
    }
    pub fn get_far_plane_half_extents(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(453usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_far_plane_half_extents", self.sys_ptr, args)
        }
    }
    pub fn inverse(&self,) -> Projection {
        type CallRet = Projection;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(454usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "inverse", self.sys_ptr, args)
        }
    }
    pub fn get_pixels_per_meter(&self, for_pixel_width: i64,) -> i64 {
        type CallRet = i64;
        type CallParams = (i64,);
        let args = (for_pixel_width,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(455usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_pixels_per_meter", self.sys_ptr, args)
        }
    }
    pub fn get_lod_multiplier(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(456usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Projection", "get_lod_multiplier", self.sys_ptr, args)
        }
    }
}
impl Projection {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Planes {
    ord: i32
}
impl Planes {
    #[doc(alias = "PLANE_NEAR")]
    #[doc = "Godot enumerator name: `PLANE_NEAR`"]
    pub const NEAR: Planes = Planes {
        ord: 0i32
    };
    #[doc(alias = "PLANE_FAR")]
    #[doc = "Godot enumerator name: `PLANE_FAR`"]
    pub const FAR: Planes = Planes {
        ord: 1i32
    };
    #[doc(alias = "PLANE_LEFT")]
    #[doc = "Godot enumerator name: `PLANE_LEFT`"]
    pub const LEFT: Planes = Planes {
        ord: 2i32
    };
    #[doc(alias = "PLANE_TOP")]
    #[doc = "Godot enumerator name: `PLANE_TOP`"]
    pub const TOP: Planes = Planes {
        ord: 3i32
    };
    #[doc(alias = "PLANE_RIGHT")]
    #[doc = "Godot enumerator name: `PLANE_RIGHT`"]
    pub const RIGHT: Planes = Planes {
        ord: 4i32
    };
    #[doc(alias = "PLANE_BOTTOM")]
    #[doc = "Godot enumerator name: `PLANE_BOTTOM`"]
    pub const BOTTOM: Planes = Planes {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for Planes {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Planes") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Planes {
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
            Self::NEAR => "NEAR", Self::FAR => "FAR", Self::LEFT => "LEFT", Self::TOP => "TOP", Self::RIGHT => "RIGHT", Self::BOTTOM => "BOTTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Planes::NEAR, Planes::FAR, Planes::LEFT, Planes::TOP, Planes::RIGHT, Planes::BOTTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Planes >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEAR", "PLANE_NEAR", Planes::NEAR), crate::meta::inspect::EnumConstant::new("FAR", "PLANE_FAR", Planes::FAR), crate::meta::inspect::EnumConstant::new("LEFT", "PLANE_LEFT", Planes::LEFT), crate::meta::inspect::EnumConstant::new("TOP", "PLANE_TOP", Planes::TOP), crate::meta::inspect::EnumConstant::new("RIGHT", "PLANE_RIGHT", Planes::RIGHT), crate::meta::inspect::EnumConstant::new("BOTTOM", "PLANE_BOTTOM", Planes::BOTTOM)]
        }
    }
}
impl crate::meta::GodotConvert for Planes {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Planes {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Planes {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}