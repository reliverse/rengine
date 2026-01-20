#![doc = "Sidecar module for class [`SceneState`][crate::classes::SceneState].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SceneState` enums](https://docs.godotengine.org/en/stable/classes/class_scenestate.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SceneState.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`scene_state`][crate::classes::scene_state]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `SceneState`](https://docs.godotengine.org/en/stable/classes/class_scenestate.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<SceneState>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SceneState {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl SceneState {
        pub fn get_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7822usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_base_scene_state(&self,) -> Option < Gd < crate::classes::SceneState > > {
            type CallRet = Option < Gd < crate::classes::SceneState > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7823usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_base_scene_state", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7824usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_type(&self, idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7825usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_name(&self, idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7826usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_name", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_node_path_full(&self, idx: i32, for_parent: bool,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = (i32, bool,);
            let args = (idx, for_parent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7827usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_node_path_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_node_path(&self, idx: i32,) -> NodePath {
            self.get_node_path_ex(idx,) . done()
        }
        #[inline]
        pub fn get_node_path_ex < 'a > (&'a self, idx: i32,) -> ExGetNodePath < 'a > {
            ExGetNodePath::new(self, idx,)
        }
        pub fn get_node_owner_path(&self, idx: i32,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7828usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_owner_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_node_instance_placeholder(&self, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7829usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "is_node_instance_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_instance_placeholder(&self, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7830usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_instance_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_instance(&self, idx: i32,) -> Option < Gd < crate::classes::PackedScene > > {
            type CallRet = Option < Gd < crate::classes::PackedScene > >;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7831usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_instance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_groups(&self, idx: i32,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7832usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_groups", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_index(&self, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7833usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_property_count(&self, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7834usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_property_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_property_name(&self, idx: i32, prop_idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32, i32,);
            let args = (idx, prop_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7835usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_property_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_property_value(&self, idx: i32, prop_idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32, i32,);
            let args = (idx, prop_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7836usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_node_property_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_connection_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7837usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_connection_source(&self, idx: i32,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7838usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_connection_signal(&self, idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7839usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_signal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_connection_target(&self, idx: i32,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7840usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_target", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_connection_method(&self, idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7841usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_method", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_connection_flags(&self, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7842usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_connection_binds(&self, idx: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7843usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_binds", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_connection_unbinds(&self, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7844usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneState", "get_connection_unbinds", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for SceneState {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("SceneState"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SceneState {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for SceneState {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SceneState {
        
    }
    impl std::ops::Deref for SceneState {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SceneState {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_SceneState__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `SceneState` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`SceneState::get_node_path_ex`][super::SceneState::get_node_path_ex]."]
#[must_use]
pub struct ExGetNodePath < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::SceneState, idx: i32, for_parent: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetNodePath < 'a > {
    fn new(surround_object: &'a re_export::SceneState, idx: i32,) -> Self {
        let for_parent = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, idx: idx, for_parent: for_parent,
        }
    }
    #[inline]
    pub fn for_parent(self, for_parent: bool) -> Self {
        Self {
            for_parent: for_parent, .. self
        }
    }
    #[inline]
    pub fn done(self) -> NodePath {
        let Self {
            _phantom, surround_object, idx, for_parent,
        }
        = self;
        re_export::SceneState::get_node_path_full(surround_object, idx, for_parent,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GenEditState {
    ord: i32
}
impl GenEditState {
    #[doc(alias = "GEN_EDIT_STATE_DISABLED")]
    #[doc = "Godot enumerator name: `GEN_EDIT_STATE_DISABLED`"]
    pub const DISABLED: GenEditState = GenEditState {
        ord: 0i32
    };
    #[doc(alias = "GEN_EDIT_STATE_INSTANCE")]
    #[doc = "Godot enumerator name: `GEN_EDIT_STATE_INSTANCE`"]
    pub const INSTANCE: GenEditState = GenEditState {
        ord: 1i32
    };
    #[doc(alias = "GEN_EDIT_STATE_MAIN")]
    #[doc = "Godot enumerator name: `GEN_EDIT_STATE_MAIN`"]
    pub const MAIN: GenEditState = GenEditState {
        ord: 2i32
    };
    #[doc(alias = "GEN_EDIT_STATE_MAIN_INHERITED")]
    #[doc = "Godot enumerator name: `GEN_EDIT_STATE_MAIN_INHERITED`"]
    pub const MAIN_INHERITED: GenEditState = GenEditState {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for GenEditState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GenEditState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GenEditState {
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
            Self::DISABLED => "DISABLED", Self::INSTANCE => "INSTANCE", Self::MAIN => "MAIN", Self::MAIN_INHERITED => "MAIN_INHERITED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[GenEditState::DISABLED, GenEditState::INSTANCE, GenEditState::MAIN, GenEditState::MAIN_INHERITED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GenEditState >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "GEN_EDIT_STATE_DISABLED", GenEditState::DISABLED), crate::meta::inspect::EnumConstant::new("INSTANCE", "GEN_EDIT_STATE_INSTANCE", GenEditState::INSTANCE), crate::meta::inspect::EnumConstant::new("MAIN", "GEN_EDIT_STATE_MAIN", GenEditState::MAIN), crate::meta::inspect::EnumConstant::new("MAIN_INHERITED", "GEN_EDIT_STATE_MAIN_INHERITED", GenEditState::MAIN_INHERITED)]
        }
    }
}
impl crate::meta::GodotConvert for GenEditState {
    type Via = i32;
    
}
impl crate::meta::ToGodot for GenEditState {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GenEditState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::SceneState;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for SceneState {
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