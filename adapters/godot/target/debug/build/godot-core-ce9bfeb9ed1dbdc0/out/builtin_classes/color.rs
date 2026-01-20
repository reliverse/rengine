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
pub struct InnerColor < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerColor < 'a > {
    pub fn from_outer(outer: &Color) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn to_argb32(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(457usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "to_argb32", self.sys_ptr, args)
        }
    }
    pub fn to_abgr32(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(458usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "to_abgr32", self.sys_ptr, args)
        }
    }
    pub fn to_rgba32(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(459usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "to_rgba32", self.sys_ptr, args)
        }
    }
    pub fn to_argb64(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(460usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "to_argb64", self.sys_ptr, args)
        }
    }
    pub fn to_abgr64(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(461usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "to_abgr64", self.sys_ptr, args)
        }
    }
    pub fn to_rgba64(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(462usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "to_rgba64", self.sys_ptr, args)
        }
    }
    pub fn to_html(&self, with_alpha: bool,) -> GString {
        type CallRet = GString;
        type CallParams = (bool,);
        let args = (with_alpha,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(463usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "to_html", self.sys_ptr, args)
        }
    }
    pub fn clamp(&self, min: Color, max: Color,) -> Color {
        type CallRet = Color;
        type CallParams = (Color, Color,);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(464usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "clamp", self.sys_ptr, args)
        }
    }
    pub fn inverted(&self,) -> Color {
        type CallRet = Color;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(465usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "inverted", self.sys_ptr, args)
        }
    }
    pub fn lerp(&self, to: Color, weight: f64,) -> Color {
        type CallRet = Color;
        type CallParams = (Color, f64,);
        let args = (to, weight,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(466usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "lerp", self.sys_ptr, args)
        }
    }
    pub fn lightened(&self, amount: f64,) -> Color {
        type CallRet = Color;
        type CallParams = (f64,);
        let args = (amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(467usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "lightened", self.sys_ptr, args)
        }
    }
    pub fn darkened(&self, amount: f64,) -> Color {
        type CallRet = Color;
        type CallParams = (f64,);
        let args = (amount,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(468usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "darkened", self.sys_ptr, args)
        }
    }
    pub fn blend(&self, over: Color,) -> Color {
        type CallRet = Color;
        type CallParams = (Color,);
        let args = (over,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(469usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "blend", self.sys_ptr, args)
        }
    }
    pub fn get_luminance(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(470usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "get_luminance", self.sys_ptr, args)
        }
    }
    pub fn srgb_to_linear(&self,) -> Color {
        type CallRet = Color;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(471usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "srgb_to_linear", self.sys_ptr, args)
        }
    }
    pub fn linear_to_srgb(&self,) -> Color {
        type CallRet = Color;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(472usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "linear_to_srgb", self.sys_ptr, args)
        }
    }
    pub fn is_equal_approx(&self, to: Color,) -> bool {
        type CallRet = bool;
        type CallParams = (Color,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(473usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "is_equal_approx", self.sys_ptr, args)
        }
    }
    pub fn hex(hex: i64,) -> Color {
        type CallRet = Color;
        type CallParams = (i64,);
        let args = (hex,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(474usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "hex", std::ptr::null_mut(), args)
        }
    }
    pub fn hex64(hex: i64,) -> Color {
        type CallRet = Color;
        type CallParams = (i64,);
        let args = (hex,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(475usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "hex64", std::ptr::null_mut(), args)
        }
    }
    pub fn html(rgba: impl AsArg < GString >,) -> Color {
        type CallRet = Color;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (rgba.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(476usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "html", std::ptr::null_mut(), args)
        }
    }
    pub fn html_is_valid(color: impl AsArg < GString >,) -> bool {
        type CallRet = bool;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
        let args = (color.into_arg(),);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(477usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "html_is_valid", std::ptr::null_mut(), args)
        }
    }
    pub fn from_string(str: impl AsArg < GString >, default: Color,) -> Color {
        type CallRet = Color;
        type CallParams < 'a0, > = (CowArg < 'a0, GString >, Color,);
        let args = (str.into_arg(), default,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(478usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "from_string", std::ptr::null_mut(), args)
        }
    }
    pub fn from_hsv(h: f64, s: f64, v: f64, alpha: f64,) -> Color {
        type CallRet = Color;
        type CallParams = (f64, f64, f64, f64,);
        let args = (h, s, v, alpha,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(479usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "from_hsv", std::ptr::null_mut(), args)
        }
    }
    pub fn from_ok_hsl(h: f64, s: f64, l: f64, alpha: f64,) -> Color {
        type CallRet = Color;
        type CallParams = (f64, f64, f64, f64,);
        let args = (h, s, l, alpha,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(480usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "from_ok_hsl", std::ptr::null_mut(), args)
        }
    }
    pub fn from_rgbe9995(rgbe: i64,) -> Color {
        type CallRet = Color;
        type CallParams = (i64,);
        let args = (rgbe,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(481usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "from_rgbe9995", std::ptr::null_mut(), args)
        }
    }
    pub fn from_rgba8(r8: i64, g8: i64, b8: i64, a8: i64,) -> Color {
        type CallRet = Color;
        type CallParams = (i64, i64, i64, i64,);
        let args = (r8, g8, b8, a8,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(482usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Color", "from_rgba8", std::ptr::null_mut(), args)
        }
    }
}
impl Color {
    
}