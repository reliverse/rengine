#![doc = "Sidecar module for class [`InputEvent`][crate::classes::InputEvent].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputEvent` enums](https://docs.godotengine.org/en/stable/classes/class_inputevent.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputEvent.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`input_event`][crate::classes::input_event]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `InputEvent`](https://docs.godotengine.org/en/stable/classes/class_inputevent.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<InputEvent>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputEvent {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl InputEvent {
        pub fn set_device(&mut self, device: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (device,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4440usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "set_device", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_device(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4441usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "get_device", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn is_action_full(&self, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4442usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_action_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_action(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a,) -> ExIsAction < 'a > {
            ExIsAction::new(self, action,)
        }
        pub(crate) fn is_action_pressed_full(&self, action: CowArg < StringName >, allow_echo: bool, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool, bool,);
            let args = (action, allow_echo, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4443usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_action_pressed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_action_pressed_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_action_pressed(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_pressed_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_pressed_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a,) -> ExIsActionPressed < 'a > {
            ExIsActionPressed::new(self, action,)
        }
        pub(crate) fn is_action_released_full(&self, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4444usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_action_released", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_action_released_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_action_released(&self, action: impl AsArg < StringName >,) -> bool {
            self.is_action_released_ex(action,) . done()
        }
        #[inline]
        pub fn is_action_released_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a,) -> ExIsActionReleased < 'a > {
            ExIsActionReleased::new(self, action,)
        }
        pub(crate) fn get_action_strength_full(&self, action: CowArg < StringName >, exact_match: bool,) -> f32 {
            type CallRet = f32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4445usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "get_action_strength", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_action_strength_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_action_strength(&self, action: impl AsArg < StringName >,) -> f32 {
            self.get_action_strength_ex(action,) . done()
        }
        #[inline]
        pub fn get_action_strength_ex < 'a > (&'a self, action: impl AsArg < StringName > + 'a,) -> ExGetActionStrength < 'a > {
            ExGetActionStrength::new(self, action,)
        }
        pub fn is_canceled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4446usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_canceled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_pressed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4447usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_released(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4448usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_released", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_echo(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4449usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_echo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn as_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4450usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "as_text", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn is_match_full(&self, event: CowArg < Option < Gd < crate::classes::InputEvent > > >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::InputEvent > > >, bool,);
            let args = (event, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4451usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_match", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_match_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_match(&self, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> >,) -> bool {
            self.is_match_ex(event,) . done()
        }
        #[inline]
        pub fn is_match_ex < 'a > (&'a self, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> > + 'a,) -> ExIsMatch < 'a > {
            ExIsMatch::new(self, event,)
        }
        pub fn is_action_type(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4452usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "is_action_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn accumulate(&mut self, with_event: impl AsArg < Option < Gd < crate::classes::InputEvent >> >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::InputEvent > > >,);
            let args = (with_event.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4453usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "accumulate", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn xformed_by_full(&self, xform: Transform2D, local_ofs: Vector2,) -> Option < Gd < crate::classes::InputEvent > > {
            type CallRet = Option < Gd < crate::classes::InputEvent > >;
            type CallParams = (Transform2D, Vector2,);
            let args = (xform, local_ofs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4454usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEvent", "xformed_by", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::xformed_by_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn xformed_by(&self, xform: Transform2D,) -> Option < Gd < crate::classes::InputEvent > > {
            self.xformed_by_ex(xform,) . done()
        }
        #[inline]
        pub fn xformed_by_ex < 'a > (&'a self, xform: Transform2D,) -> ExXformedBy < 'a > {
            ExXformedBy::new(self, xform,)
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
        pub const DEVICE_ID_EMULATION: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for InputEvent {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("InputEvent"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputEvent {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for InputEvent {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for InputEvent {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for InputEvent {
        
    }
    impl std::ops::Deref for InputEvent {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputEvent {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_InputEvent__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `InputEvent` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`InputEvent::is_action_ex`][super::InputEvent::is_action_ex]."]
#[must_use]
pub struct ExIsAction < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::InputEvent, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsAction < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, action: impl AsArg < StringName > + 'a,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), exact_match: exact_match,
        }
    }
    #[inline]
    pub fn exact_match(self, exact_match: bool) -> Self {
        Self {
            exact_match: exact_match, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, action, exact_match,
        }
        = self;
        re_export::InputEvent::is_action_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::is_action_pressed_ex`][super::InputEvent::is_action_pressed_ex]."]
#[must_use]
pub struct ExIsActionPressed < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::InputEvent, action: CowArg < 'a, StringName >, allow_echo: bool, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionPressed < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, action: impl AsArg < StringName > + 'a,) -> Self {
        let allow_echo = false;
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), allow_echo: allow_echo, exact_match: exact_match,
        }
    }
    #[inline]
    pub fn allow_echo(self, allow_echo: bool) -> Self {
        Self {
            allow_echo: allow_echo, .. self
        }
    }
    #[inline]
    pub fn exact_match(self, exact_match: bool) -> Self {
        Self {
            exact_match: exact_match, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, action, allow_echo, exact_match,
        }
        = self;
        re_export::InputEvent::is_action_pressed_full(surround_object, action, allow_echo, exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::is_action_released_ex`][super::InputEvent::is_action_released_ex]."]
#[must_use]
pub struct ExIsActionReleased < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::InputEvent, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsActionReleased < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, action: impl AsArg < StringName > + 'a,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), exact_match: exact_match,
        }
    }
    #[inline]
    pub fn exact_match(self, exact_match: bool) -> Self {
        Self {
            exact_match: exact_match, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, action, exact_match,
        }
        = self;
        re_export::InputEvent::is_action_released_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::get_action_strength_ex`][super::InputEvent::get_action_strength_ex]."]
#[must_use]
pub struct ExGetActionStrength < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::InputEvent, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetActionStrength < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, action: impl AsArg < StringName > + 'a,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), exact_match: exact_match,
        }
    }
    #[inline]
    pub fn exact_match(self, exact_match: bool) -> Self {
        Self {
            exact_match: exact_match, .. self
        }
    }
    #[inline]
    pub fn done(self) -> f32 {
        let Self {
            _phantom, surround_object, action, exact_match,
        }
        = self;
        re_export::InputEvent::get_action_strength_full(surround_object, action, exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::is_match_ex`][super::InputEvent::is_match_ex]."]
#[must_use]
pub struct ExIsMatch < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::InputEvent, event: CowArg < 'a, Option < Gd < crate::classes::InputEvent > > >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsMatch < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> > + 'a,) -> Self {
        let exact_match = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, event: event.into_arg(), exact_match: exact_match,
        }
    }
    #[inline]
    pub fn exact_match(self, exact_match: bool) -> Self {
        Self {
            exact_match: exact_match, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, event, exact_match,
        }
        = self;
        re_export::InputEvent::is_match_full(surround_object, event, exact_match,)
    }
}
#[doc = "Default-param extender for [`InputEvent::xformed_by_ex`][super::InputEvent::xformed_by_ex]."]
#[must_use]
pub struct ExXformedBy < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::InputEvent, xform: Transform2D, local_ofs: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExXformedBy < 'a > {
    fn new(surround_object: &'a re_export::InputEvent, xform: Transform2D,) -> Self {
        let local_ofs = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, xform: xform, local_ofs: local_ofs,
        }
    }
    #[inline]
    pub fn local_ofs(self, local_ofs: Vector2) -> Self {
        Self {
            local_ofs: local_ofs, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::InputEvent > > {
        let Self {
            _phantom, surround_object, xform, local_ofs,
        }
        = self;
        re_export::InputEvent::xformed_by_full(surround_object, xform, local_ofs,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::InputEvent;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for InputEvent {
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