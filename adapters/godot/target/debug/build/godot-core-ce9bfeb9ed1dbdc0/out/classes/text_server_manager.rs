#![doc = "Sidecar module for class [`TextServerManager`][crate::classes::TextServerManager].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextServerManager` enums](https://docs.godotengine.org/en/stable/classes/class_textservermanager.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextServerManager.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`text_server_manager`][crate::classes::text_server_manager]: sidecar module with related enum/flag types\n* [`SignalsOfTextServerManager`][crate::classes::text_server_manager::SignalsOfTextServerManager]: signal collection\n\n\nSee also [Godot docs for `TextServerManager`](https://docs.godotengine.org/en/stable/classes/class_textservermanager.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextServerManager {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl TextServerManager {
        pub fn add_interface(&mut self, interface: impl AsArg < Option < Gd < crate::classes::TextServer >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TextServer > > >,);
            let args = (interface.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9487usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServerManager", "add_interface", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_interface_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9488usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServerManager", "get_interface_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_interface(&mut self, interface: impl AsArg < Option < Gd < crate::classes::TextServer >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TextServer > > >,);
            let args = (interface.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9489usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServerManager", "remove_interface", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_interface(&self, idx: i32,) -> Option < Gd < crate::classes::TextServer > > {
            type CallRet = Option < Gd < crate::classes::TextServer > >;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9490usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServerManager", "get_interface", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_interfaces(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9491usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServerManager", "get_interfaces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn find_interface(&self, name: impl AsArg < GString >,) -> Option < Gd < crate::classes::TextServer > > {
            type CallRet = Option < Gd < crate::classes::TextServer > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9492usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServerManager", "find_interface", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_primary_interface(&mut self, index: impl AsArg < Option < Gd < crate::classes::TextServer >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TextServer > > >,);
            let args = (index.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9493usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServerManager", "set_primary_interface", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_primary_interface(&self,) -> Option < Gd < crate::classes::TextServer > > {
            type CallRet = Option < Gd < crate::classes::TextServer > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9494usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextServerManager", "get_primary_interface", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TextServerManager {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TextServerManager"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextServerManager {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextServerManager {
        
    }
    impl crate::obj::Singleton for TextServerManager {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"TextServerManager"))
            }
        }
    }
    impl std::ops::Deref for TextServerManager {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextServerManager {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TextServerManager__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `TextServerManager` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TextServerManager;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`TextServerManager`][crate::classes::TextServerManager] class."]
    pub struct SignalsOfTextServerManager < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfTextServerManager < 'c, C > {
        #[doc = "Signature: `(interface_name: StringName)`"]
        pub fn interface_added(&mut self) -> SigInterfaceAdded < 'c, C > {
            SigInterfaceAdded {
                typed: TypedSignal::extract(&mut self.__internal_obj, "interface_added")
            }
        }
        #[doc = "Signature: `(interface_name: StringName)`"]
        pub fn interface_removed(&mut self) -> SigInterfaceRemoved < 'c, C > {
            SigInterfaceRemoved {
                typed: TypedSignal::extract(&mut self.__internal_obj, "interface_removed")
            }
        }
    }
    type TypedSigInterfaceAdded < 'c, C > = TypedSignal < 'c, C, (StringName,) >;
    pub struct SigInterfaceAdded < 'c, C: WithSignals > {
        typed: TypedSigInterfaceAdded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigInterfaceAdded < 'c, C > {
        pub fn emit(&mut self, interface_name: StringName,) {
            self.typed.emit_tuple((interface_name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigInterfaceAdded < 'c, C > {
        type Target = TypedSigInterfaceAdded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigInterfaceAdded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigInterfaceRemoved < 'c, C > = TypedSignal < 'c, C, (StringName,) >;
    pub struct SigInterfaceRemoved < 'c, C: WithSignals > {
        typed: TypedSigInterfaceRemoved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigInterfaceRemoved < 'c, C > {
        pub fn emit(&mut self, interface_name: StringName,) {
            self.typed.emit_tuple((interface_name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigInterfaceRemoved < 'c, C > {
        type Target = TypedSigInterfaceRemoved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigInterfaceRemoved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for TextServerManager {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfTextServerManager < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfTextServerManager < 'c, C > {
        type Target = < < TextServerManager as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = TextServerManager;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfTextServerManager < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = TextServerManager;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}