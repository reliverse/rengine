#![doc = "Sidecar module for class [`InstancePlaceholder`][crate::classes::InstancePlaceholder].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InstancePlaceholder` enums](https://docs.godotengine.org/en/stable/classes/class_instanceplaceholder.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InstancePlaceholder.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`instance_placeholder`][crate::classes::instance_placeholder]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `InstancePlaceholder`](https://docs.godotengine.org/en/stable/classes/class_instanceplaceholder.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<InstancePlaceholder>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InstancePlaceholder {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl InstancePlaceholder {
        pub(crate) fn get_stored_values_full(&mut self, with_order: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (bool,);
            let args = (with_order,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4596usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InstancePlaceholder", "get_stored_values", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_stored_values_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_stored_values(&mut self,) -> VarDictionary {
            self.get_stored_values_ex() . done()
        }
        #[inline]
        pub fn get_stored_values_ex < 'a > (&'a mut self,) -> ExGetStoredValues < 'a > {
            ExGetStoredValues::new(self,)
        }
        pub(crate) fn create_instance_full(&mut self, replace: bool, custom_scene: CowArg < Option < Gd < crate::classes::PackedScene > > >,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams < 'a0, > = (bool, CowArg < 'a0, Option < Gd < crate::classes::PackedScene > > >,);
            let args = (replace, custom_scene,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4597usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InstancePlaceholder", "create_instance", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_instance_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_instance(&mut self,) -> Option < Gd < crate::classes::Node > > {
            self.create_instance_ex() . done()
        }
        #[inline]
        pub fn create_instance_ex < 'a > (&'a mut self,) -> ExCreateInstance < 'a > {
            ExCreateInstance::new(self,)
        }
        pub fn get_instance_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4598usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InstancePlaceholder", "get_instance_path", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for InstancePlaceholder {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("InstancePlaceholder"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InstancePlaceholder {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for InstancePlaceholder {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for InstancePlaceholder {
        
    }
    impl std::ops::Deref for InstancePlaceholder {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InstancePlaceholder {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_InstancePlaceholder__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `InstancePlaceholder` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`InstancePlaceholder::get_stored_values_ex`][super::InstancePlaceholder::get_stored_values_ex]."]
#[must_use]
pub struct ExGetStoredValues < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::InstancePlaceholder, with_order: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetStoredValues < 'a > {
    fn new(surround_object: &'a mut re_export::InstancePlaceholder,) -> Self {
        let with_order = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, with_order: with_order,
        }
    }
    #[inline]
    pub fn with_order(self, with_order: bool) -> Self {
        Self {
            with_order: with_order, .. self
        }
    }
    #[inline]
    pub fn done(self) -> VarDictionary {
        let Self {
            _phantom, surround_object, with_order,
        }
        = self;
        re_export::InstancePlaceholder::get_stored_values_full(surround_object, with_order,)
    }
}
#[doc = "Default-param extender for [`InstancePlaceholder::create_instance_ex`][super::InstancePlaceholder::create_instance_ex]."]
#[must_use]
pub struct ExCreateInstance < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::InstancePlaceholder, replace: bool, custom_scene: CowArg < 'a, Option < Gd < crate::classes::PackedScene > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateInstance < 'a > {
    fn new(surround_object: &'a mut re_export::InstancePlaceholder,) -> Self {
        let replace = false;
        let custom_scene = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, replace: replace, custom_scene: custom_scene.into_arg(),
        }
    }
    #[inline]
    pub fn replace(self, replace: bool) -> Self {
        Self {
            replace: replace, .. self
        }
    }
    #[inline]
    pub fn custom_scene(self, custom_scene: impl AsArg < Option < Gd < crate::classes::PackedScene >> > + 'a) -> Self {
        Self {
            custom_scene: custom_scene.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, replace, custom_scene,
        }
        = self;
        re_export::InstancePlaceholder::create_instance_full(surround_object, replace, custom_scene,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::InstancePlaceholder;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node::SignalsOfNode;
    impl WithSignals for InstancePlaceholder {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}