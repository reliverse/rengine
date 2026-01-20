#![doc = "Sidecar module for class [`InputMap`][crate::classes::InputMap].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputMap` enums](https://docs.godotengine.org/en/stable/classes/class_inputmap.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputMap.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`input_map`][crate::classes::input_map]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `InputMap`](https://docs.godotengine.org/en/stable/classes/class_inputmap.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputMap {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl InputMap {
        pub fn has_action(&self, action: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4582usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "has_action", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_actions(&mut self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4583usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "get_actions", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_action_full(&mut self, action: CowArg < StringName >, deadzone: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f32,);
            let args = (action, deadzone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4584usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "add_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_action_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_action(&mut self, action: impl AsArg < StringName >,) {
            self.add_action_ex(action,) . done()
        }
        #[inline]
        pub fn add_action_ex < 'a > (&'a mut self, action: impl AsArg < StringName > + 'a,) -> ExAddAction < 'a > {
            ExAddAction::new(self, action,)
        }
        pub fn erase_action(&mut self, action: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4585usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "erase_action", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_action_description(&self, action: impl AsArg < StringName >,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4586usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "get_action_description", Some(self.__validated_obj()), args,)
            }
        }
        pub fn action_set_deadzone(&mut self, action: impl AsArg < StringName >, deadzone: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f32,);
            let args = (action.into_arg(), deadzone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4587usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "action_set_deadzone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn action_get_deadzone(&mut self, action: impl AsArg < StringName >,) -> f32 {
            type CallRet = f32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4588usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "action_get_deadzone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn action_add_event(&mut self, action: impl AsArg < StringName >, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::InputEvent > > >,);
            let args = (action.into_arg(), event.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4589usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "action_add_event", Some(self.__validated_obj()), args,)
            }
        }
        pub fn action_has_event(&mut self, action: impl AsArg < StringName >, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::InputEvent > > >,);
            let args = (action.into_arg(), event.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4590usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "action_has_event", Some(self.__validated_obj()), args,)
            }
        }
        pub fn action_erase_event(&mut self, action: impl AsArg < StringName >, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::InputEvent > > >,);
            let args = (action.into_arg(), event.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4591usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "action_erase_event", Some(self.__validated_obj()), args,)
            }
        }
        pub fn action_erase_events(&mut self, action: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4592usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "action_erase_events", Some(self.__validated_obj()), args,)
            }
        }
        pub fn action_get_events(&mut self, action: impl AsArg < StringName >,) -> Array < Gd < crate::classes::InputEvent > > {
            type CallRet = Array < Gd < crate::classes::InputEvent > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (action.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4593usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "action_get_events", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn event_is_action_full(&self, event: CowArg < Option < Gd < crate::classes::InputEvent > > >, action: CowArg < StringName >, exact_match: bool,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::InputEvent > > >, CowArg < 'a1, StringName >, bool,);
            let args = (event, action, exact_match,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4594usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "event_is_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::event_is_action_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn event_is_action(&self, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> >, action: impl AsArg < StringName >,) -> bool {
            self.event_is_action_ex(event, action,) . done()
        }
        #[inline]
        pub fn event_is_action_ex < 'a > (&'a self, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> > + 'a, action: impl AsArg < StringName > + 'a,) -> ExEventIsAction < 'a > {
            ExEventIsAction::new(self, event, action,)
        }
        pub fn load_from_project_settings(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4595usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputMap", "load_from_project_settings", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for InputMap {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("InputMap"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputMap {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for InputMap {
        
    }
    impl crate::obj::Singleton for InputMap {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"InputMap"))
            }
        }
    }
    impl std::ops::Deref for InputMap {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputMap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_InputMap__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `InputMap` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`InputMap::add_action_ex`][super::InputMap::add_action_ex]."]
#[must_use]
pub struct ExAddAction < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::InputMap, action: CowArg < 'a, StringName >, deadzone: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddAction < 'a > {
    fn new(surround_object: &'a mut re_export::InputMap, action: impl AsArg < StringName > + 'a,) -> Self {
        let deadzone = 0.2f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: action.into_arg(), deadzone: deadzone,
        }
    }
    #[inline]
    pub fn deadzone(self, deadzone: f32) -> Self {
        Self {
            deadzone: deadzone, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, action, deadzone,
        }
        = self;
        re_export::InputMap::add_action_full(surround_object, action, deadzone,)
    }
}
#[doc = "Default-param extender for [`InputMap::event_is_action_ex`][super::InputMap::event_is_action_ex]."]
#[must_use]
pub struct ExEventIsAction < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::InputMap, event: CowArg < 'a, Option < Gd < crate::classes::InputEvent > > >, action: CowArg < 'a, StringName >, exact_match: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExEventIsAction < 'a > {
    fn new(surround_object: &'a re_export::InputMap, event: impl AsArg < Option < Gd < crate::classes::InputEvent >> > + 'a, action: impl AsArg < StringName > + 'a,) -> Self {
        let exact_match = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, event: event.into_arg(), action: action.into_arg(), exact_match: exact_match,
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
            _phantom, surround_object, event, action, exact_match,
        }
        = self;
        re_export::InputMap::event_is_action_full(surround_object, event, action, exact_match,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::InputMap;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for InputMap {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfObject < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}