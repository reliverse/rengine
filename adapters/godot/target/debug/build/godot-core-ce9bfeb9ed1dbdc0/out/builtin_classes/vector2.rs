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
pub struct InnerVector2 < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerVector2 < 'a > {
    pub fn from_outer(outer: &Vector2) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn angle(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(116usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "angle", self.sys_ptr, args)
        }
    }
    pub fn angle_to(&self, to: Vector2,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector2,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(117usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "angle_to", self.sys_ptr, args)
        }
    }
    pub fn angle_to_point(&self, to: Vector2,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector2,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(118usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "angle_to_point", self.sys_ptr, args)
        }
    }
    pub fn direction_to(&self, to: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(119usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "direction_to", self.sys_ptr, args)
        }
    }
    pub fn distance_to(&self, to: Vector2,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector2,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(120usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "distance_to", self.sys_ptr, args)
        }
    }
    pub fn distance_squared_to(&self, to: Vector2,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector2,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(121usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "distance_squared_to", self.sys_ptr, args)
        }
    }
    pub fn length(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(122usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "length", self.sys_ptr, args)
        }
    }
    pub fn length_squared(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(123usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "length_squared", self.sys_ptr, args)
        }
    }
    pub fn limit_length(&self, length: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (f64,);
        let args = (length,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(124usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "limit_length", self.sys_ptr, args)
        }
    }
    pub fn normalized(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(125usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "normalized", self.sys_ptr, args)
        }
    }
    pub fn is_normalized(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(126usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "is_normalized", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, to: Vector2,) -> bool {
        type CallRet = bool;
        type CallParams = (Vector2,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(127usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn is_zero_approx(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(128usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "is_zero_approx", self.sys_ptr, args)
        }
    }
    pub fn is_finite(&self,) -> bool {
        type CallRet = bool;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(129usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "is_finite", self.sys_ptr, args)
        }
    }
    pub fn posmod(&self, mod_: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (f64,);
        let args = (mod_,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(130usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "posmod", self.sys_ptr, args)
        }
    }
    pub fn posmodv(&self, modv: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (modv,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(131usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "posmodv", self.sys_ptr, args)
        }
    }
    pub fn project(&self, b: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (b,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(132usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "project", self.sys_ptr, args)
        }
    }
    pub fn lerp(&self, to: Vector2, weight: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2, f64,);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(133usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "lerp", self.sys_ptr, args)
        }
    }
    pub fn slerp(&self, to: Vector2, weight: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2, f64,);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(134usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "slerp", self.sys_ptr, args)
        }
    }
    pub fn cubic_interpolate(&self, b: Vector2, pre_a: Vector2, post_b: Vector2, weight: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2, Vector2, Vector2, f64,);
        let args = (b, pre_a, post_b, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(135usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "cubic_interpolate", self.sys_ptr, args)
        }
    }
    pub fn cubic_interpolate_in_time(&self, b: Vector2, pre_a: Vector2, post_b: Vector2, weight: f64, b_t: f64, pre_a_t: f64, post_b_t: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2, Vector2, Vector2, f64, f64, f64, f64,);
        let args = (b, pre_a, post_b, weight, b_t, pre_a_t, post_b_t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(136usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "cubic_interpolate_in_time", self.sys_ptr, args)
        }
    }
    pub fn bezier_interpolate(&self, control_1: Vector2, control_2: Vector2, end: Vector2, t: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2, Vector2, Vector2, f64,);
        let args = (control_1, control_2, end, t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(137usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "bezier_interpolate", self.sys_ptr, args)
        }
    }
    pub fn bezier_derivative(&self, control_1: Vector2, control_2: Vector2, end: Vector2, t: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2, Vector2, Vector2, f64,);
        let args = (control_1, control_2, end, t,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(138usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "bezier_derivative", self.sys_ptr, args)
        }
    }
    pub fn max_axis_index(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(139usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "max_axis_index", self.sys_ptr, args)
        }
    }
    pub fn min_axis_index(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(140usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "min_axis_index", self.sys_ptr, args)
        }
    }
    pub fn move_toward(&self, to: Vector2, delta: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2, f64,);
        let args = (to, delta,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(141usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "move_toward", self.sys_ptr, args)
        }
    }
    pub fn rotated(&self, angle: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (f64,);
        let args = (angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(142usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "rotated", self.sys_ptr, args)
        }
    }
    pub fn orthogonal(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(143usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "orthogonal", self.sys_ptr, args)
        }
    }
    pub fn floor(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(144usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "floor", self.sys_ptr, args)
        }
    }
    pub fn ceil(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(145usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "ceil", self.sys_ptr, args)
        }
    }
    pub fn round(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(146usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "round", self.sys_ptr, args)
        }
    }
    pub fn aspect(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(147usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "aspect", self.sys_ptr, args)
        }
    }
    pub fn dot(&self, with: Vector2,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector2,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(148usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "dot", self.sys_ptr, args)
        }
    }
    pub fn slide(&self, n: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (n,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(149usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "slide", self.sys_ptr, args)
        }
    }
    pub fn bounce(&self, n: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (n,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(150usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "bounce", self.sys_ptr, args)
        }
    }
    pub fn reflect(&self, line: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (line,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(151usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "reflect", self.sys_ptr, args)
        }
    }
    pub fn cross(&self, with: Vector2,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector2,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(152usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "cross", self.sys_ptr, args)
        }
    }
    pub fn abs(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(153usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "abs", self.sys_ptr, args)
        }
    }
    pub fn sign(&self,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(154usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "sign", self.sys_ptr, args)
        }
    }
    pub fn clamp(&self, min: Vector2, max: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2, Vector2,);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(155usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "clamp", self.sys_ptr, args)
        }
    }
    pub fn clampf(&self, min: f64, max: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (f64, f64,);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(156usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "clampf", self.sys_ptr, args)
        }
    }
    pub fn snapped(&self, step: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(157usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "snapped", self.sys_ptr, args)
        }
    }
    pub fn snappedf(&self, step: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (f64,);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(158usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "snappedf", self.sys_ptr, args)
        }
    }
    pub fn min(&self, with: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(159usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "min", self.sys_ptr, args)
        }
    }
    pub fn minf(&self, with: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (f64,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(160usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "minf", self.sys_ptr, args)
        }
    }
    pub fn max(&self, with: Vector2,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (Vector2,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(161usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "max", self.sys_ptr, args)
        }
    }
    pub fn maxf(&self, with: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (f64,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(162usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "maxf", self.sys_ptr, args)
        }
    }
    pub fn from_angle(angle: f64,) -> Vector2 {
        type CallRet = Vector2;
        type CallParams = (f64,);
        let args = (angle,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(163usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2", "from_angle", std::ptr::null_mut(), args)
        }
    }
}
impl Vector2 {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Axis {
    ord: i32
}
impl Axis {
    #[doc(alias = "AXIS_X")]
    #[doc = "Godot enumerator name: `AXIS_X`"]
    pub const X: Axis = Axis {
        ord: 0i32
    };
    #[doc(alias = "AXIS_Y")]
    #[doc = "Godot enumerator name: `AXIS_Y`"]
    pub const Y: Axis = Axis {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Axis") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Axis {
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
            Self::X => "X", Self::Y => "Y", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Axis::X, Axis::Y]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Axis >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("X", "AXIS_X", Axis::X), crate::meta::inspect::EnumConstant::new("Y", "AXIS_Y", Axis::Y)]
        }
    }
}
impl crate::meta::GodotConvert for Axis {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Axis {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Axis {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}