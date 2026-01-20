#![doc = "Sidecar module for class [`Slider`][crate::classes::Slider].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Slider` enums](https://docs.godotengine.org/en/stable/classes/class_slider.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Slider.`\n\nInherits [`Range`][crate::classes::Range].\n\nRelated symbols:\n\n* [`slider`][crate::classes::slider]: sidecar module with related enum/flag types\n* [`SignalsOfSlider`][crate::classes::slider::SignalsOfSlider]: signal collection\n\n\nSee also [Godot docs for `Slider`](https://docs.godotengine.org/en/stable/classes/class_slider.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Slider>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Slider {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Slider {
        pub fn set_ticks(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "set_ticks", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ticks(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "get_ticks", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ticks_on_borders(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "get_ticks_on_borders", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ticks_on_borders(&mut self, ticks_on_border: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (ticks_on_border,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "set_ticks_on_borders", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ticks_position(&self,) -> crate::classes::slider::TickPosition {
            type CallRet = crate::classes::slider::TickPosition;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "get_ticks_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ticks_position(&mut self, ticks_on_border: crate::classes::slider::TickPosition,) {
            type CallRet = ();
            type CallParams = (crate::classes::slider::TickPosition,);
            let args = (ticks_on_border,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "set_ticks_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_editable(&mut self, editable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (editable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "set_editable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_editable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "is_editable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scrollable(&mut self, scrollable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (scrollable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "set_scrollable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_scrollable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Slider", "is_scrollable", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Slider {
        type Base = crate::classes::Range;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Slider"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Slider {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Range > for Slider {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for Slider {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Slider {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Slider {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Slider {
        
    }
    impl std::ops::Deref for Slider {
        type Target = crate::classes::Range;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Slider {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Slider__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Slider` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TickPosition {
    ord: i32
}
impl TickPosition {
    #[doc(alias = "TICK_POSITION_BOTTOM_RIGHT")]
    #[doc = "Godot enumerator name: `TICK_POSITION_BOTTOM_RIGHT`"]
    pub const BOTTOM_RIGHT: TickPosition = TickPosition {
        ord: 0i32
    };
    #[doc(alias = "TICK_POSITION_TOP_LEFT")]
    #[doc = "Godot enumerator name: `TICK_POSITION_TOP_LEFT`"]
    pub const TOP_LEFT: TickPosition = TickPosition {
        ord: 1i32
    };
    #[doc(alias = "TICK_POSITION_BOTH")]
    #[doc = "Godot enumerator name: `TICK_POSITION_BOTH`"]
    pub const BOTH: TickPosition = TickPosition {
        ord: 2i32
    };
    #[doc(alias = "TICK_POSITION_CENTER")]
    #[doc = "Godot enumerator name: `TICK_POSITION_CENTER`"]
    pub const CENTER: TickPosition = TickPosition {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TickPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TickPosition") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TickPosition {
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
            Self::BOTTOM_RIGHT => "BOTTOM_RIGHT", Self::TOP_LEFT => "TOP_LEFT", Self::BOTH => "BOTH", Self::CENTER => "CENTER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TickPosition::BOTTOM_RIGHT, TickPosition::TOP_LEFT, TickPosition::BOTH, TickPosition::CENTER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TickPosition >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BOTTOM_RIGHT", "TICK_POSITION_BOTTOM_RIGHT", TickPosition::BOTTOM_RIGHT), crate::meta::inspect::EnumConstant::new("TOP_LEFT", "TICK_POSITION_TOP_LEFT", TickPosition::TOP_LEFT), crate::meta::inspect::EnumConstant::new("BOTH", "TICK_POSITION_BOTH", TickPosition::BOTH), crate::meta::inspect::EnumConstant::new("CENTER", "TICK_POSITION_CENTER", TickPosition::CENTER)]
        }
    }
}
impl crate::meta::GodotConvert for TickPosition {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TickPosition {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TickPosition {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Slider;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Slider`][crate::classes::Slider] class."]
    pub struct SignalsOfSlider < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfSlider < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn drag_started(&mut self) -> SigDragStarted < 'c, C > {
            SigDragStarted {
                typed: TypedSignal::extract(&mut self.__internal_obj, "drag_started")
            }
        }
        #[doc = "Signature: `(value_changed: bool)`"]
        pub fn drag_ended(&mut self) -> SigDragEnded < 'c, C > {
            SigDragEnded {
                typed: TypedSignal::extract(&mut self.__internal_obj, "drag_ended")
            }
        }
    }
    type TypedSigDragStarted < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigDragStarted < 'c, C: WithSignals > {
        typed: TypedSigDragStarted < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDragStarted < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDragStarted < 'c, C > {
        type Target = TypedSigDragStarted < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDragStarted < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigDragEnded < 'c, C > = TypedSignal < 'c, C, (bool,) >;
    pub struct SigDragEnded < 'c, C: WithSignals > {
        typed: TypedSigDragEnded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDragEnded < 'c, C > {
        pub fn emit(&mut self, value_changed: bool,) {
            self.typed.emit_tuple((value_changed,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDragEnded < 'c, C > {
        type Target = TypedSigDragEnded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDragEnded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Slider {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfSlider < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfSlider < 'c, C > {
        type Target = < < Slider as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Slider;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfSlider < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Slider;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}