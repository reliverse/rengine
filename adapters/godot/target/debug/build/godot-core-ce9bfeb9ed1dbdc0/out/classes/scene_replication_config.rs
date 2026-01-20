#![doc = "Sidecar module for class [`SceneReplicationConfig`][crate::classes::SceneReplicationConfig].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SceneReplicationConfig` enums](https://docs.godotengine.org/en/stable/classes/class_scenereplicationconfig.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SceneReplicationConfig.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`scene_replication_config`][crate::classes::scene_replication_config]: sidecar module with related enum/flag types\n* [`ISceneReplicationConfig`][crate::classes::ISceneReplicationConfig]: virtual methods\n\n\nSee also [Godot docs for `SceneReplicationConfig`](https://docs.godotengine.org/en/stable/classes/class_scenereplicationconfig.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`SceneReplicationConfig::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SceneReplicationConfig {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`SceneReplicationConfig`][crate::classes::SceneReplicationConfig].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `SceneReplicationConfig` methods](https://docs.godotengine.org/en/stable/classes/class_scenereplicationconfig.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISceneReplicationConfig: crate::obj::GodotClass < Base = SceneReplicationConfig > + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `Base<T>` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::classes::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: ObjectNotification) {
            unimplemented !()
        }
        #[doc = r" Called whenever [`get()`](crate::classes::Object::get) is called or Godot gets the value of a property."]
        #[doc = r""]
        #[doc = r" Should return the given `property`'s value as `Some(value)`, or `None` if the property should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-get)."]
        fn get_property(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`set()`](crate::classes::Object::set) is called or Godot sets the value of a property."]
        #[doc = r""]
        #[doc = r" Should set `property` to the given `value` and return `true`, or return `false` to indicate the `property`"]
        #[doc = r" should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_set`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-set)."]
        fn set_property(&mut self, property: StringName, value: Variant) -> bool {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`get_property_list()`](crate::classes::Object::get_property_list) is called, the returned vector here is"]
        #[doc = r" appended to the existing list of properties."]
        #[doc = r""]
        #[doc = r" This should mainly be used for advanced purposes, such as dynamically updating the property list in the editor."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get_property_list`](https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-get-property-list)"]
        #[cfg(since_api = "4.3")]
        #[cfg_attr(published_docs, doc(cfg(since_api = "4.3")))]
        fn get_property_list(&mut self) -> Vec < crate::meta::PropertyInfo > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot retrieves value of property. Allows to customize existing properties."]
        #[doc = r" Every property info goes through this method, except properties **added** with `get_property_list()`."]
        #[doc = r""]
        #[doc = r" Exposed `property` here is a shared mutable reference obtained (and returned to) from Godot."]
        #[doc = r""]
        #[doc = r" See also in the Godot docs:"]
        #[doc = r" * [`Object::_validate_property`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-validate-property)"]
        fn validate_property(&self, property: &mut crate::meta::PropertyInfo) {
            unimplemented !()
        }
        #[doc = r" Called by Godot to tell if a property has a custom revert or not."]
        #[doc = r""]
        #[doc = r" Return `None` for no custom revert, and return `Some(value)` to specify the custom revert."]
        #[doc = r""]
        #[doc = r" This is a combination of Godot's [`Object::_property_get_revert`] and [`Object::_property_can_revert`]. This means that this"]
        #[doc = r" function will usually be called twice by Godot to find the revert."]
        #[doc = r""]
        #[doc = r" Note that this should be a _pure_ function. That is, it should always return the same value for a property as long as `self`"]
        #[doc = r" remains unchanged. Otherwise, this may lead to unexpected (safe) behavior."]
        #[doc = r""]
        #[doc = r" [`Object::_property_get_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-get-revert"]
        #[doc = r" [`Object::_property_can_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-can-revert"]
        #[doc(alias = "property_can_revert")]
        fn property_get_revert(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        fn setup_local_to_scene(&mut self,) {
            unimplemented !()
        }
        fn get_rid(&self,) -> Rid {
            unimplemented !()
        }
        fn reset_state(&mut self,) {
            unimplemented !()
        }
        fn set_path_cache(&self, path: GString,) {
            unimplemented !()
        }
    }
    impl SceneReplicationConfig {
        pub fn get_properties(&self,) -> Array < NodePath > {
            type CallRet = Array < NodePath >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7809usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneReplicationConfig", "get_properties", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_property_full(&mut self, path: CowArg < NodePath >, index: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >, i32,);
            let args = (path, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7810usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneReplicationConfig", "add_property", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_property_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_property(&mut self, path: impl AsArg < NodePath >,) {
            self.add_property_ex(path,) . done()
        }
        #[inline]
        pub fn add_property_ex < 'a > (&'a mut self, path: impl AsArg < NodePath > + 'a,) -> ExAddProperty < 'a > {
            ExAddProperty::new(self, path,)
        }
        pub fn has_property(&self, path: impl AsArg < NodePath >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7811usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneReplicationConfig", "has_property", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_property(&mut self, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7812usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneReplicationConfig", "remove_property", Some(self.__validated_obj()), args,)
            }
        }
        pub fn property_get_index(&self, path: impl AsArg < NodePath >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7813usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneReplicationConfig", "property_get_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn property_get_spawn(&mut self, path: impl AsArg < NodePath >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7814usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneReplicationConfig", "property_get_spawn", Some(self.__validated_obj()), args,)
            }
        }
        pub fn property_set_spawn(&mut self, path: impl AsArg < NodePath >, enabled: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >, bool,);
            let args = (path.into_arg(), enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7815usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneReplicationConfig", "property_set_spawn", Some(self.__validated_obj()), args,)
            }
        }
        pub fn property_get_replication_mode(&mut self, path: impl AsArg < NodePath >,) -> crate::classes::scene_replication_config::ReplicationMode {
            type CallRet = crate::classes::scene_replication_config::ReplicationMode;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7816usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneReplicationConfig", "property_get_replication_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn property_set_replication_mode(&mut self, path: impl AsArg < NodePath >, mode: crate::classes::scene_replication_config::ReplicationMode,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >, crate::classes::scene_replication_config::ReplicationMode,);
            let args = (path.into_arg(), mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7817usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneReplicationConfig", "property_set_replication_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn property_get_sync(&mut self, path: impl AsArg < NodePath >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7818usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneReplicationConfig", "property_get_sync", Some(self.__validated_obj()), args,)
            }
        }
        pub fn property_set_sync(&mut self, path: impl AsArg < NodePath >, enabled: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >, bool,);
            let args = (path.into_arg(), enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7819usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneReplicationConfig", "property_set_sync", Some(self.__validated_obj()), args,)
            }
        }
        pub fn property_get_watch(&mut self, path: impl AsArg < NodePath >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7820usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneReplicationConfig", "property_get_watch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn property_set_watch(&mut self, path: impl AsArg < NodePath >, enabled: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >, bool,);
            let args = (path.into_arg(), enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7821usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneReplicationConfig", "property_set_watch", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for SceneReplicationConfig {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("SceneReplicationConfig"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SceneReplicationConfig {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for SceneReplicationConfig {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for SceneReplicationConfig {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SceneReplicationConfig {
        
    }
    impl crate::obj::cap::GodotDefault for SceneReplicationConfig {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SceneReplicationConfig {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SceneReplicationConfig {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SceneReplicationConfig`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_SceneReplicationConfig__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SceneReplicationConfig > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Resource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`SceneReplicationConfig::add_property_ex`][super::SceneReplicationConfig::add_property_ex]."]
#[must_use]
pub struct ExAddProperty < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SceneReplicationConfig, path: CowArg < 'a, NodePath >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddProperty < 'a > {
    fn new(surround_object: &'a mut re_export::SceneReplicationConfig, path: impl AsArg < NodePath > + 'a,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, path, index,
        }
        = self;
        re_export::SceneReplicationConfig::add_property_full(surround_object, path, index,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ReplicationMode {
    ord: i32
}
impl ReplicationMode {
    #[doc(alias = "REPLICATION_MODE_NEVER")]
    #[doc = "Godot enumerator name: `REPLICATION_MODE_NEVER`"]
    pub const NEVER: ReplicationMode = ReplicationMode {
        ord: 0i32
    };
    #[doc(alias = "REPLICATION_MODE_ALWAYS")]
    #[doc = "Godot enumerator name: `REPLICATION_MODE_ALWAYS`"]
    pub const ALWAYS: ReplicationMode = ReplicationMode {
        ord: 1i32
    };
    #[doc(alias = "REPLICATION_MODE_ON_CHANGE")]
    #[doc = "Godot enumerator name: `REPLICATION_MODE_ON_CHANGE`"]
    pub const ON_CHANGE: ReplicationMode = ReplicationMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ReplicationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ReplicationMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ReplicationMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
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
            Self::NEVER => "NEVER", Self::ALWAYS => "ALWAYS", Self::ON_CHANGE => "ON_CHANGE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ReplicationMode::NEVER, ReplicationMode::ALWAYS, ReplicationMode::ON_CHANGE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ReplicationMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEVER", "REPLICATION_MODE_NEVER", ReplicationMode::NEVER), crate::meta::inspect::EnumConstant::new("ALWAYS", "REPLICATION_MODE_ALWAYS", ReplicationMode::ALWAYS), crate::meta::inspect::EnumConstant::new("ON_CHANGE", "REPLICATION_MODE_ON_CHANGE", ReplicationMode::ON_CHANGE)]
        }
    }
}
impl crate::meta::GodotConvert for ReplicationMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ReplicationMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ReplicationMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::SceneReplicationConfig;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for SceneReplicationConfig {
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