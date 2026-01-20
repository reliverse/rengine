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
pub struct InnerVector2i < 'a > {
    _outer_lifetime: std::marker::PhantomData < &'a() >, sys_ptr: sys::GDExtensionTypePtr,
}
impl < 'a > InnerVector2i < 'a > {
    pub fn from_outer(outer: &Vector2i) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData, sys_ptr: sys::SysPtr::force_mut(outer.sys()),
        }
    }
    pub fn aspect(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(164usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "aspect", self.sys_ptr, args)
        }
    }
    pub fn max_axis_index(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(165usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "max_axis_index", self.sys_ptr, args)
        }
    }
    pub fn min_axis_index(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(166usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "min_axis_index", self.sys_ptr, args)
        }
    }
    pub fn distance_to(&self, to: Vector2i,) -> f64 {
        type CallRet = f64;
        type CallParams = (Vector2i,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(167usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "distance_to", self.sys_ptr, args)
        }
    }
    pub fn distance_squared_to(&self, to: Vector2i,) -> i64 {
        type CallRet = i64;
        type CallParams = (Vector2i,);
        let args = (to,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(168usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "distance_squared_to", self.sys_ptr, args)
        }
    }
    pub fn length(&self,) -> f64 {
        type CallRet = f64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(169usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "length", self.sys_ptr, args)
        }
    }
    pub fn length_squared(&self,) -> i64 {
        type CallRet = i64;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(170usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "length_squared", self.sys_ptr, args)
        }
    }
    pub fn sign(&self,) -> Vector2i {
        type CallRet = Vector2i;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(171usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "sign", self.sys_ptr, args)
        }
    }
    pub fn abs(&self,) -> Vector2i {
        type CallRet = Vector2i;
        type CallParams = ();
        let args = ();
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(172usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "abs", self.sys_ptr, args)
        }
    }
    pub fn clamp(&self, min: Vector2i, max: Vector2i,) -> Vector2i {
        type CallRet = Vector2i;
        type CallParams = (Vector2i, Vector2i,);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(173usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "clamp", self.sys_ptr, args)
        }
    }
    pub fn clampi(&self, min: i64, max: i64,) -> Vector2i {
        type CallRet = Vector2i;
        type CallParams = (i64, i64,);
        let args = (min, max,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(174usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "clampi", self.sys_ptr, args)
        }
    }
    pub fn snapped(&self, step: Vector2i,) -> Vector2i {
        type CallRet = Vector2i;
        type CallParams = (Vector2i,);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(175usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "snapped", self.sys_ptr, args)
        }
    }
    pub fn snappedi(&self, step: i64,) -> Vector2i {
        type CallRet = Vector2i;
        type CallParams = (i64,);
        let args = (step,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(176usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "snappedi", self.sys_ptr, args)
        }
    }
    pub fn min(&self, with: Vector2i,) -> Vector2i {
        type CallRet = Vector2i;
        type CallParams = (Vector2i,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(177usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "min", self.sys_ptr, args)
        }
    }
    pub fn mini(&self, with: i64,) -> Vector2i {
        type CallRet = Vector2i;
        type CallParams = (i64,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(178usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "mini", self.sys_ptr, args)
        }
    }
    pub fn max(&self, with: Vector2i,) -> Vector2i {
        type CallRet = Vector2i;
        type CallParams = (Vector2i,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(179usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "max", self.sys_ptr, args)
        }
    }
    pub fn maxi(&self, with: i64,) -> Vector2i {
        type CallRet = Vector2i;
        type CallParams = (i64,);
        let args = (with,);
        unsafe {
            let method_bind = sys::builtin_method_table() . fptr_by_index(180usize);
            Signature::< CallParams, CallRet > ::out_builtin_ptrcall(method_bind, "Vector2i", "maxi", self.sys_ptr, args)
        }
    }
}
impl Vector2i {
    
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