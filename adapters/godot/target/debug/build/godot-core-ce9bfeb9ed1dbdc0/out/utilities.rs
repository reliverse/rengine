use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, ClassId, CowArg, InParamTuple, OutParamTuple, ParamTuple, RefArg, Signature
};
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
pub fn sin(angle_rad: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (angle_rad,);
    unsafe {
        let utility_fn = sys::utility_function_table() . sin;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "sin", args)
    }
}
pub fn cos(angle_rad: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (angle_rad,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cos;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "cos", args)
    }
}
pub fn tan(angle_rad: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (angle_rad,);
    unsafe {
        let utility_fn = sys::utility_function_table() . tan;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "tan", args)
    }
}
pub fn sinh(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . sinh;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "sinh", args)
    }
}
pub fn cosh(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cosh;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "cosh", args)
    }
}
pub fn tanh(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . tanh;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "tanh", args)
    }
}
pub fn asin(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . asin;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "asin", args)
    }
}
pub fn acos(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . acos;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "acos", args)
    }
}
pub fn atan(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . atan;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "atan", args)
    }
}
pub fn atan2(y: f64, x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (y, x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . atan2;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "atan2", args)
    }
}
pub fn asinh(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . asinh;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "asinh", args)
    }
}
pub fn acosh(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . acosh;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "acosh", args)
    }
}
pub fn atanh(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . atanh;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "atanh", args)
    }
}
pub fn sqrt(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . sqrt;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "sqrt", args)
    }
}
pub fn fmod(x: f64, y: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (x, y,);
    unsafe {
        let utility_fn = sys::utility_function_table() . fmod;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "fmod", args)
    }
}
pub fn fposmod(x: f64, y: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (x, y,);
    unsafe {
        let utility_fn = sys::utility_function_table() . fposmod;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "fposmod", args)
    }
}
pub fn posmod(x: i64, y: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64, i64,);
    let args = (x, y,);
    unsafe {
        let utility_fn = sys::utility_function_table() . posmod;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "posmod", args)
    }
}
pub fn floor(x: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(x),);
    unsafe {
        let utility_fn = sys::utility_function_table() . floor;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "floor", args)
    }
}
pub fn floorf(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . floorf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "floorf", args)
    }
}
pub fn floori(x: f64,) -> i64 {
    type CallRet = i64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . floori;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "floori", args)
    }
}
pub fn ceil(x: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(x),);
    unsafe {
        let utility_fn = sys::utility_function_table() . ceil;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "ceil", args)
    }
}
pub fn ceilf(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . ceilf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "ceilf", args)
    }
}
pub fn ceili(x: f64,) -> i64 {
    type CallRet = i64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . ceili;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "ceili", args)
    }
}
pub fn round(x: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(x),);
    unsafe {
        let utility_fn = sys::utility_function_table() . round;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "round", args)
    }
}
pub fn roundf(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . roundf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "roundf", args)
    }
}
pub fn roundi(x: f64,) -> i64 {
    type CallRet = i64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . roundi;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "roundi", args)
    }
}
pub fn abs(x: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(x),);
    unsafe {
        let utility_fn = sys::utility_function_table() . abs;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "abs", args)
    }
}
pub fn absf(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . absf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "absf", args)
    }
}
pub fn absi(x: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . absi;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "absi", args)
    }
}
pub fn sign(x: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(x),);
    unsafe {
        let utility_fn = sys::utility_function_table() . sign;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "sign", args)
    }
}
pub fn signf(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . signf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "signf", args)
    }
}
pub fn signi(x: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . signi;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "signi", args)
    }
}
pub fn snapped(x: &Variant, step: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >,);
    let args = (RefArg::new(x), RefArg::new(step),);
    unsafe {
        let utility_fn = sys::utility_function_table() . snapped;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "snapped", args)
    }
}
pub fn snappedf(x: f64, step: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (x, step,);
    unsafe {
        let utility_fn = sys::utility_function_table() . snappedf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "snappedf", args)
    }
}
pub fn snappedi(x: f64, step: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (f64, i64,);
    let args = (x, step,);
    unsafe {
        let utility_fn = sys::utility_function_table() . snappedi;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "snappedi", args)
    }
}
pub fn pow(base: f64, exp: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (base, exp,);
    unsafe {
        let utility_fn = sys::utility_function_table() . pow;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "pow", args)
    }
}
pub fn log(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . log;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "log", args)
    }
}
pub fn exp(x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . exp;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "exp", args)
    }
}
pub fn is_nan(x: f64,) -> bool {
    type CallRet = bool;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_nan;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "is_nan", args)
    }
}
pub fn is_inf(x: f64,) -> bool {
    type CallRet = bool;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_inf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "is_inf", args)
    }
}
pub fn is_equal_approx(a: f64, b: f64,) -> bool {
    type CallRet = bool;
    type CallParams = (f64, f64,);
    let args = (a, b,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_equal_approx;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "is_equal_approx", args)
    }
}
pub fn is_zero_approx(x: f64,) -> bool {
    type CallRet = bool;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_zero_approx;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "is_zero_approx", args)
    }
}
pub fn is_finite(x: f64,) -> bool {
    type CallRet = bool;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_finite;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "is_finite", args)
    }
}
pub fn ease(x: f64, curve: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (x, curve,);
    unsafe {
        let utility_fn = sys::utility_function_table() . ease;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "ease", args)
    }
}
pub fn step_decimals(x: f64,) -> i64 {
    type CallRet = i64;
    type CallParams = (f64,);
    let args = (x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . step_decimals;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "step_decimals", args)
    }
}
pub fn lerp(from: &Variant, to: &Variant, weight: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >, RefArg < 'a2, Variant >,);
    let args = (RefArg::new(from), RefArg::new(to), RefArg::new(weight),);
    unsafe {
        let utility_fn = sys::utility_function_table() . lerp;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "lerp", args)
    }
}
pub fn lerpf(from: f64, to: f64, weight: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64,);
    let args = (from, to, weight,);
    unsafe {
        let utility_fn = sys::utility_function_table() . lerpf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "lerpf", args)
    }
}
pub fn cubic_interpolate(from: f64, to: f64, pre: f64, post: f64, weight: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64, f64, f64,);
    let args = (from, to, pre, post, weight,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cubic_interpolate;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "cubic_interpolate", args)
    }
}
pub fn cubic_interpolate_angle(from: f64, to: f64, pre: f64, post: f64, weight: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64, f64, f64,);
    let args = (from, to, pre, post, weight,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cubic_interpolate_angle;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "cubic_interpolate_angle", args)
    }
}
pub fn cubic_interpolate_in_time(from: f64, to: f64, pre: f64, post: f64, weight: f64, to_t: f64, pre_t: f64, post_t: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64, f64, f64, f64, f64, f64,);
    let args = (from, to, pre, post, weight, to_t, pre_t, post_t,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cubic_interpolate_in_time;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "cubic_interpolate_in_time", args)
    }
}
pub fn cubic_interpolate_angle_in_time(from: f64, to: f64, pre: f64, post: f64, weight: f64, to_t: f64, pre_t: f64, post_t: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64, f64, f64, f64, f64, f64,);
    let args = (from, to, pre, post, weight, to_t, pre_t, post_t,);
    unsafe {
        let utility_fn = sys::utility_function_table() . cubic_interpolate_angle_in_time;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "cubic_interpolate_angle_in_time", args)
    }
}
pub fn bezier_interpolate(start: f64, control_1: f64, control_2: f64, end: f64, t: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64, f64, f64,);
    let args = (start, control_1, control_2, end, t,);
    unsafe {
        let utility_fn = sys::utility_function_table() . bezier_interpolate;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "bezier_interpolate", args)
    }
}
pub fn bezier_derivative(start: f64, control_1: f64, control_2: f64, end: f64, t: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64, f64, f64,);
    let args = (start, control_1, control_2, end, t,);
    unsafe {
        let utility_fn = sys::utility_function_table() . bezier_derivative;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "bezier_derivative", args)
    }
}
pub fn angle_difference(from: f64, to: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (from, to,);
    unsafe {
        let utility_fn = sys::utility_function_table() . angle_difference;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "angle_difference", args)
    }
}
pub fn lerp_angle(from: f64, to: f64, weight: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64,);
    let args = (from, to, weight,);
    unsafe {
        let utility_fn = sys::utility_function_table() . lerp_angle;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "lerp_angle", args)
    }
}
pub fn inverse_lerp(from: f64, to: f64, weight: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64,);
    let args = (from, to, weight,);
    unsafe {
        let utility_fn = sys::utility_function_table() . inverse_lerp;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "inverse_lerp", args)
    }
}
pub fn remap(value: f64, istart: f64, istop: f64, ostart: f64, ostop: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64, f64, f64,);
    let args = (value, istart, istop, ostart, ostop,);
    unsafe {
        let utility_fn = sys::utility_function_table() . remap;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "remap", args)
    }
}
pub fn smoothstep(from: f64, to: f64, x: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64,);
    let args = (from, to, x,);
    unsafe {
        let utility_fn = sys::utility_function_table() . smoothstep;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "smoothstep", args)
    }
}
pub fn move_toward(from: f64, to: f64, delta: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64,);
    let args = (from, to, delta,);
    unsafe {
        let utility_fn = sys::utility_function_table() . move_toward;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "move_toward", args)
    }
}
pub fn rotate_toward(from: f64, to: f64, delta: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64,);
    let args = (from, to, delta,);
    unsafe {
        let utility_fn = sys::utility_function_table() . rotate_toward;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "rotate_toward", args)
    }
}
pub fn deg_to_rad(deg: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (deg,);
    unsafe {
        let utility_fn = sys::utility_function_table() . deg_to_rad;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "deg_to_rad", args)
    }
}
pub fn rad_to_deg(rad: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (rad,);
    unsafe {
        let utility_fn = sys::utility_function_table() . rad_to_deg;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "rad_to_deg", args)
    }
}
pub fn linear_to_db(lin: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (lin,);
    unsafe {
        let utility_fn = sys::utility_function_table() . linear_to_db;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "linear_to_db", args)
    }
}
pub fn db_to_linear(db: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64,);
    let args = (db,);
    unsafe {
        let utility_fn = sys::utility_function_table() . db_to_linear;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "db_to_linear", args)
    }
}
pub fn wrap(value: &Variant, min: &Variant, max: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >, RefArg < 'a2, Variant >,);
    let args = (RefArg::new(value), RefArg::new(min), RefArg::new(max),);
    unsafe {
        let utility_fn = sys::utility_function_table() . wrap;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "wrap", args)
    }
}
pub fn wrapi(value: i64, min: i64, max: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64, i64, i64,);
    let args = (value, min, max,);
    unsafe {
        let utility_fn = sys::utility_function_table() . wrapi;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "wrapi", args)
    }
}
pub fn wrapf(value: f64, min: f64, max: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64,);
    let args = (value, min, max,);
    unsafe {
        let utility_fn = sys::utility_function_table() . wrapf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "wrapf", args)
    }
}
pub fn max(arg1: &Variant, arg2: &Variant, varargs: &[Variant]) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >,);
    let args = (RefArg::new(arg1), RefArg::new(arg2),);
    unsafe {
        let utility_fn = sys::utility_function_table() . max;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "max", args, varargs)
    }
}
pub fn maxi(a: i64, b: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64, i64,);
    let args = (a, b,);
    unsafe {
        let utility_fn = sys::utility_function_table() . maxi;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "maxi", args)
    }
}
pub fn maxf(a: f64, b: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (a, b,);
    unsafe {
        let utility_fn = sys::utility_function_table() . maxf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "maxf", args)
    }
}
pub fn min(arg1: &Variant, arg2: &Variant, varargs: &[Variant]) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >,);
    let args = (RefArg::new(arg1), RefArg::new(arg2),);
    unsafe {
        let utility_fn = sys::utility_function_table() . min;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "min", args, varargs)
    }
}
pub fn mini(a: i64, b: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64, i64,);
    let args = (a, b,);
    unsafe {
        let utility_fn = sys::utility_function_table() . mini;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "mini", args)
    }
}
pub fn minf(a: f64, b: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (a, b,);
    unsafe {
        let utility_fn = sys::utility_function_table() . minf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "minf", args)
    }
}
pub fn clamp(value: &Variant, min: &Variant, max: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >, RefArg < 'a2, Variant >,);
    let args = (RefArg::new(value), RefArg::new(min), RefArg::new(max),);
    unsafe {
        let utility_fn = sys::utility_function_table() . clamp;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "clamp", args)
    }
}
pub fn clampi(value: i64, min: i64, max: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64, i64, i64,);
    let args = (value, min, max,);
    unsafe {
        let utility_fn = sys::utility_function_table() . clampi;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "clampi", args)
    }
}
pub fn clampf(value: f64, min: f64, max: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64, f64,);
    let args = (value, min, max,);
    unsafe {
        let utility_fn = sys::utility_function_table() . clampf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "clampf", args)
    }
}
pub fn nearest_po2(value: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64,);
    let args = (value,);
    unsafe {
        let utility_fn = sys::utility_function_table() . nearest_po2;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "nearest_po2", args)
    }
}
pub fn pingpong(value: f64, length: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (value, length,);
    unsafe {
        let utility_fn = sys::utility_function_table() . pingpong;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "pingpong", args)
    }
}
pub fn randomize() {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . randomize;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "randomize", args)
    }
}
pub fn randi() -> i64 {
    type CallRet = i64;
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . randi;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "randi", args)
    }
}
pub fn randf() -> f64 {
    type CallRet = f64;
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . randf;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "randf", args)
    }
}
pub fn randi_range(from: i64, to: i64,) -> i64 {
    type CallRet = i64;
    type CallParams = (i64, i64,);
    let args = (from, to,);
    unsafe {
        let utility_fn = sys::utility_function_table() . randi_range;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "randi_range", args)
    }
}
pub fn randf_range(from: f64, to: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (from, to,);
    unsafe {
        let utility_fn = sys::utility_function_table() . randf_range;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "randf_range", args)
    }
}
pub fn randfn(mean: f64, deviation: f64,) -> f64 {
    type CallRet = f64;
    type CallParams = (f64, f64,);
    let args = (mean, deviation,);
    unsafe {
        let utility_fn = sys::utility_function_table() . randfn;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "randfn", args)
    }
}
pub fn seed(base: i64,) {
    type CallRet = ();
    type CallParams = (i64,);
    let args = (base,);
    unsafe {
        let utility_fn = sys::utility_function_table() . seed;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "seed", args)
    }
}
pub fn rand_from_seed(seed: i64,) -> PackedInt64Array {
    type CallRet = PackedInt64Array;
    type CallParams = (i64,);
    let args = (seed,);
    unsafe {
        let utility_fn = sys::utility_function_table() . rand_from_seed;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "rand_from_seed", args)
    }
}
pub fn weakref(obj: &Variant,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(obj),);
    unsafe {
        let utility_fn = sys::utility_function_table() . weakref;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "weakref", args)
    }
}
pub fn typeof_(variable: &Variant,) -> i64 {
    type CallRet = i64;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(variable),);
    unsafe {
        let utility_fn = sys::utility_function_table() . typeof_;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "typeof", args)
    }
}
pub fn type_convert(variant: &Variant, type_: i64,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >, i64,);
    let args = (RefArg::new(variant), type_,);
    unsafe {
        let utility_fn = sys::utility_function_table() . type_convert;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "type_convert", args)
    }
}
pub fn str(varargs: &[Variant]) -> GString {
    type CallRet = GString;
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . str;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "str", args, varargs)
    }
}
pub fn error_string(error: i64,) -> GString {
    type CallRet = GString;
    type CallParams = (i64,);
    let args = (error,);
    unsafe {
        let utility_fn = sys::utility_function_table() . error_string;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "error_string", args)
    }
}
pub fn type_string(type_: i64,) -> GString {
    type CallRet = GString;
    type CallParams = (i64,);
    let args = (type_,);
    unsafe {
        let utility_fn = sys::utility_function_table() . type_string;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "type_string", args)
    }
}
pub fn print(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . print;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "print", args, varargs)
    }
}
pub fn print_rich(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . print_rich;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "print_rich", args, varargs)
    }
}
pub fn printerr(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . printerr;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "printerr", args, varargs)
    }
}
pub fn printt(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . printt;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "printt", args, varargs)
    }
}
pub fn prints(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . prints;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "prints", args, varargs)
    }
}
pub fn printraw(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . printraw;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "printraw", args, varargs)
    }
}
pub fn print_verbose(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . print_verbose;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "print_verbose", args, varargs)
    }
}
pub fn push_error(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . push_error;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "push_error", args, varargs)
    }
}
pub fn push_warning(varargs: &[Variant]) {
    type CallRet = ();
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . push_warning;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall_varargs(utility_fn, "push_warning", args, varargs)
    }
}
pub fn var_to_str(variable: &Variant,) -> GString {
    type CallRet = GString;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(variable),);
    unsafe {
        let utility_fn = sys::utility_function_table() . var_to_str;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "var_to_str", args)
    }
}
pub fn str_to_var(string: impl AsArg < GString >,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
    let args = (string.into_arg(),);
    unsafe {
        let utility_fn = sys::utility_function_table() . str_to_var;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "str_to_var", args)
    }
}
pub fn var_to_bytes(variable: &Variant,) -> PackedByteArray {
    type CallRet = PackedByteArray;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(variable),);
    unsafe {
        let utility_fn = sys::utility_function_table() . var_to_bytes;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "var_to_bytes", args)
    }
}
pub fn bytes_to_var(bytes: &PackedByteArray,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
    let args = (RefArg::new(bytes),);
    unsafe {
        let utility_fn = sys::utility_function_table() . bytes_to_var;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "bytes_to_var", args)
    }
}
pub fn var_to_bytes_with_objects(variable: &Variant,) -> PackedByteArray {
    type CallRet = PackedByteArray;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(variable),);
    unsafe {
        let utility_fn = sys::utility_function_table() . var_to_bytes_with_objects;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "var_to_bytes_with_objects", args)
    }
}
pub fn bytes_to_var_with_objects(bytes: &PackedByteArray,) -> Variant {
    type CallRet = Variant;
    type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
    let args = (RefArg::new(bytes),);
    unsafe {
        let utility_fn = sys::utility_function_table() . bytes_to_var_with_objects;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "bytes_to_var_with_objects", args)
    }
}
pub fn hash(variable: &Variant,) -> i64 {
    type CallRet = i64;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(variable),);
    unsafe {
        let utility_fn = sys::utility_function_table() . hash;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "hash", args)
    }
}
pub(crate) fn is_instance_id_valid(id: i64,) -> bool {
    type CallRet = bool;
    type CallParams = (i64,);
    let args = (id,);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_instance_id_valid;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "is_instance_id_valid", args)
    }
}
pub(crate) fn is_instance_valid(instance: &Variant,) -> bool {
    type CallRet = bool;
    type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
    let args = (RefArg::new(instance),);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_instance_valid;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "is_instance_valid", args)
    }
}
pub fn rid_allocate_id() -> i64 {
    type CallRet = i64;
    type CallParams = ();
    let args = ();
    unsafe {
        let utility_fn = sys::utility_function_table() . rid_allocate_id;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "rid_allocate_id", args)
    }
}
pub fn rid_from_int64(base: i64,) -> Rid {
    type CallRet = Rid;
    type CallParams = (i64,);
    let args = (base,);
    unsafe {
        let utility_fn = sys::utility_function_table() . rid_from_int64;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "rid_from_int64", args)
    }
}
pub fn is_same(a: &Variant, b: &Variant,) -> bool {
    type CallRet = bool;
    type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >,);
    let args = (RefArg::new(a), RefArg::new(b),);
    unsafe {
        let utility_fn = sys::utility_function_table() . is_same;
        Signature::< CallParams, CallRet > ::out_utility_ptrcall(utility_fn, "is_same", args)
    }
}