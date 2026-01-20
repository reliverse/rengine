#![doc = "Sidecar module for class [`MultiplayerSynchronizer`][crate::classes::MultiplayerSynchronizer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MultiplayerSynchronizer` enums](https://docs.godotengine.org/en/stable/classes/class_multiplayersynchronizer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MultiplayerSynchronizer.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`multiplayer_synchronizer`][crate::classes::multiplayer_synchronizer]: sidecar module with related enum/flag types\n* [`IMultiplayerSynchronizer`][crate::classes::IMultiplayerSynchronizer]: virtual methods\n* [`SignalsOfMultiplayerSynchronizer`][crate::classes::multiplayer_synchronizer::SignalsOfMultiplayerSynchronizer]: signal collection\n\n\nSee also [Godot docs for `MultiplayerSynchronizer`](https://docs.godotengine.org/en/stable/classes/class_multiplayersynchronizer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`MultiplayerSynchronizer::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MultiplayerSynchronizer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`MultiplayerSynchronizer`][crate::classes::MultiplayerSynchronizer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `MultiplayerSynchronizer` methods](https://docs.godotengine.org/en/stable/classes/class_multiplayersynchronizer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMultiplayerSynchronizer: crate::obj::GodotClass < Base = MultiplayerSynchronizer > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: NodeNotification) {
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
        fn process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) {
            unimplemented !()
        }
        fn enter_tree(&mut self,) {
            unimplemented !()
        }
        fn exit_tree(&mut self,) {
            unimplemented !()
        }
        fn ready(&mut self,) {
            unimplemented !()
        }
        fn get_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_accessibility_configuration_warnings(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn shortcut_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn unhandled_key_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn get_focused_accessibility_element(&self,) -> Rid {
            unimplemented !()
        }
    }
    impl MultiplayerSynchronizer {
        pub fn set_root_path(&mut self, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5511usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "set_root_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_path(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5512usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "get_root_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_replication_interval(&mut self, milliseconds: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (milliseconds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5513usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "set_replication_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_replication_interval(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5514usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "get_replication_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_delta_interval(&mut self, milliseconds: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (milliseconds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5515usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "set_delta_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_delta_interval(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5516usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "get_delta_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_replication_config(&mut self, config: impl AsArg < Option < Gd < crate::classes::SceneReplicationConfig >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::SceneReplicationConfig > > >,);
            let args = (config.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5517usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "set_replication_config", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_replication_config(&mut self,) -> Option < Gd < crate::classes::SceneReplicationConfig > > {
            type CallRet = Option < Gd < crate::classes::SceneReplicationConfig > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5518usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "get_replication_config", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_update_mode(&mut self, mode: crate::classes::multiplayer_synchronizer::VisibilityUpdateMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::multiplayer_synchronizer::VisibilityUpdateMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5519usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "set_visibility_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_update_mode(&self,) -> crate::classes::multiplayer_synchronizer::VisibilityUpdateMode {
            type CallRet = crate::classes::multiplayer_synchronizer::VisibilityUpdateMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5520usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "get_visibility_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn update_visibility_full(&mut self, for_peer: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (for_peer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5521usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "update_visibility", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::update_visibility_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn update_visibility(&mut self,) {
            self.update_visibility_ex() . done()
        }
        #[inline]
        pub fn update_visibility_ex < 'a > (&'a mut self,) -> ExUpdateVisibility < 'a > {
            ExUpdateVisibility::new(self,)
        }
        pub fn set_visibility_public(&mut self, visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5522usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "set_visibility_public", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_visibility_public(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5523usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "is_visibility_public", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_visibility_filter(&mut self, filter: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(filter),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5524usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "add_visibility_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_visibility_filter(&mut self, filter: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(filter),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5525usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "remove_visibility_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_for(&mut self, peer: i32, visible: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (peer, visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5526usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "set_visibility_for", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_for(&self, peer: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (peer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5527usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSynchronizer", "get_visibility_for", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for MultiplayerSynchronizer {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("MultiplayerSynchronizer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MultiplayerSynchronizer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for MultiplayerSynchronizer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MultiplayerSynchronizer {
        
    }
    impl crate::obj::cap::GodotDefault for MultiplayerSynchronizer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MultiplayerSynchronizer {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MultiplayerSynchronizer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MultiplayerSynchronizer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_MultiplayerSynchronizer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MultiplayerSynchronizer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`MultiplayerSynchronizer::update_visibility_ex`][super::MultiplayerSynchronizer::update_visibility_ex]."]
#[must_use]
pub struct ExUpdateVisibility < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::MultiplayerSynchronizer, for_peer: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExUpdateVisibility < 'a > {
    fn new(surround_object: &'a mut re_export::MultiplayerSynchronizer,) -> Self {
        let for_peer = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, for_peer: for_peer,
        }
    }
    #[inline]
    pub fn for_peer(self, for_peer: i32) -> Self {
        Self {
            for_peer: for_peer, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, for_peer,
        }
        = self;
        re_export::MultiplayerSynchronizer::update_visibility_full(surround_object, for_peer,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VisibilityUpdateMode {
    ord: i32
}
impl VisibilityUpdateMode {
    #[doc(alias = "VISIBILITY_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `VISIBILITY_PROCESS_IDLE`"]
    pub const IDLE: VisibilityUpdateMode = VisibilityUpdateMode {
        ord: 0i32
    };
    #[doc(alias = "VISIBILITY_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `VISIBILITY_PROCESS_PHYSICS`"]
    pub const PHYSICS: VisibilityUpdateMode = VisibilityUpdateMode {
        ord: 1i32
    };
    #[doc(alias = "VISIBILITY_PROCESS_NONE")]
    #[doc = "Godot enumerator name: `VISIBILITY_PROCESS_NONE`"]
    pub const NONE: VisibilityUpdateMode = VisibilityUpdateMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for VisibilityUpdateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VisibilityUpdateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VisibilityUpdateMode {
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
            Self::IDLE => "IDLE", Self::PHYSICS => "PHYSICS", Self::NONE => "NONE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VisibilityUpdateMode::IDLE, VisibilityUpdateMode::PHYSICS, VisibilityUpdateMode::NONE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VisibilityUpdateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("IDLE", "VISIBILITY_PROCESS_IDLE", VisibilityUpdateMode::IDLE), crate::meta::inspect::EnumConstant::new("PHYSICS", "VISIBILITY_PROCESS_PHYSICS", VisibilityUpdateMode::PHYSICS), crate::meta::inspect::EnumConstant::new("NONE", "VISIBILITY_PROCESS_NONE", VisibilityUpdateMode::NONE)]
        }
    }
}
impl crate::meta::GodotConvert for VisibilityUpdateMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VisibilityUpdateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VisibilityUpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::MultiplayerSynchronizer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`MultiplayerSynchronizer`][crate::classes::MultiplayerSynchronizer] class."]
    pub struct SignalsOfMultiplayerSynchronizer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfMultiplayerSynchronizer < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn synchronized(&mut self) -> SigSynchronized < 'c, C > {
            SigSynchronized {
                typed: TypedSignal::extract(&mut self.__internal_obj, "synchronized")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn delta_synchronized(&mut self) -> SigDeltaSynchronized < 'c, C > {
            SigDeltaSynchronized {
                typed: TypedSignal::extract(&mut self.__internal_obj, "delta_synchronized")
            }
        }
        #[doc = "Signature: `(for_peer: i64)`"]
        pub fn visibility_changed(&mut self) -> SigVisibilityChanged < 'c, C > {
            SigVisibilityChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "visibility_changed")
            }
        }
    }
    type TypedSigSynchronized < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSynchronized < 'c, C: WithSignals > {
        typed: TypedSigSynchronized < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSynchronized < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSynchronized < 'c, C > {
        type Target = TypedSigSynchronized < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSynchronized < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigDeltaSynchronized < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigDeltaSynchronized < 'c, C: WithSignals > {
        typed: TypedSigDeltaSynchronized < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDeltaSynchronized < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDeltaSynchronized < 'c, C > {
        type Target = TypedSigDeltaSynchronized < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDeltaSynchronized < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigVisibilityChanged < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigVisibilityChanged < 'c, C: WithSignals > {
        typed: TypedSigVisibilityChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigVisibilityChanged < 'c, C > {
        pub fn emit(&mut self, for_peer: i64,) {
            self.typed.emit_tuple((for_peer,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigVisibilityChanged < 'c, C > {
        type Target = TypedSigVisibilityChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigVisibilityChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for MultiplayerSynchronizer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfMultiplayerSynchronizer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfMultiplayerSynchronizer < 'c, C > {
        type Target = < < MultiplayerSynchronizer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = MultiplayerSynchronizer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfMultiplayerSynchronizer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = MultiplayerSynchronizer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}