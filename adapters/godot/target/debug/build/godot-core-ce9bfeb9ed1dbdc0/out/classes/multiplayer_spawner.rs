#![doc = "Sidecar module for class [`MultiplayerSpawner`][crate::classes::MultiplayerSpawner].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MultiplayerSpawner` enums](https://docs.godotengine.org/en/stable/classes/class_multiplayerspawner.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MultiplayerSpawner.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`multiplayer_spawner`][crate::classes::multiplayer_spawner]: sidecar module with related enum/flag types\n* [`IMultiplayerSpawner`][crate::classes::IMultiplayerSpawner]: virtual methods\n* [`SignalsOfMultiplayerSpawner`][crate::classes::multiplayer_spawner::SignalsOfMultiplayerSpawner]: signal collection\n\n\nSee also [Godot docs for `MultiplayerSpawner`](https://docs.godotengine.org/en/stable/classes/class_multiplayerspawner.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`MultiplayerSpawner::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MultiplayerSpawner {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`MultiplayerSpawner`][crate::classes::MultiplayerSpawner].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `MultiplayerSpawner` methods](https://docs.godotengine.org/en/stable/classes/class_multiplayerspawner.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMultiplayerSpawner: crate::obj::GodotClass < Base = MultiplayerSpawner > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MultiplayerSpawner {
        pub fn add_spawnable_scene(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5500usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSpawner", "add_spawnable_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_spawnable_scene_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5501usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSpawner", "get_spawnable_scene_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_spawnable_scene(&self, index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5502usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSpawner", "get_spawnable_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_spawnable_scenes(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5503usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSpawner", "clear_spawnable_scenes", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn spawn_full(&mut self, data: RefArg < Variant >,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >,);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5504usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSpawner", "spawn", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::spawn_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn spawn(&mut self,) -> Option < Gd < crate::classes::Node > > {
            self.spawn_ex() . done()
        }
        #[inline]
        pub fn spawn_ex < 'a > (&'a mut self,) -> ExSpawn < 'a > {
            ExSpawn::new(self,)
        }
        pub fn get_spawn_path(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5505usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSpawner", "get_spawn_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_spawn_path(&mut self, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5506usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSpawner", "set_spawn_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_spawn_limit(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5507usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSpawner", "get_spawn_limit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_spawn_limit(&mut self, limit: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (limit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5508usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSpawner", "set_spawn_limit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_spawn_function(&self,) -> Callable {
            type CallRet = Callable;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5509usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSpawner", "get_spawn_function", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_spawn_function(&mut self, spawn_function: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(spawn_function),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5510usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerSpawner", "set_spawn_function", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for MultiplayerSpawner {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("MultiplayerSpawner"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MultiplayerSpawner {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for MultiplayerSpawner {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MultiplayerSpawner {
        
    }
    impl crate::obj::cap::GodotDefault for MultiplayerSpawner {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MultiplayerSpawner {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MultiplayerSpawner {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MultiplayerSpawner`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_MultiplayerSpawner__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MultiplayerSpawner > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`MultiplayerSpawner::spawn_ex`][super::MultiplayerSpawner::spawn_ex]."]
#[must_use]
pub struct ExSpawn < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::MultiplayerSpawner, data: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSpawn < 'a > {
    fn new(surround_object: &'a mut re_export::MultiplayerSpawner,) -> Self {
        let data = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, data: CowArg::Owned(data),
        }
    }
    #[inline]
    pub fn data(self, data: &'a Variant) -> Self {
        Self {
            data: CowArg::Borrowed(data), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Node > > {
        let Self {
            _phantom, surround_object, data,
        }
        = self;
        re_export::MultiplayerSpawner::spawn_full(surround_object, data.cow_as_arg(),)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::MultiplayerSpawner;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`MultiplayerSpawner`][crate::classes::MultiplayerSpawner] class."]
    pub struct SignalsOfMultiplayerSpawner < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfMultiplayerSpawner < 'c, C > {
        #[doc = "Signature: `(node: Gd<Node>)`"]
        pub fn despawned(&mut self) -> SigDespawned < 'c, C > {
            SigDespawned {
                typed: TypedSignal::extract(&mut self.__internal_obj, "despawned")
            }
        }
        #[doc = "Signature: `(node: Gd<Node>)`"]
        pub fn spawned(&mut self) -> SigSpawned < 'c, C > {
            SigSpawned {
                typed: TypedSignal::extract(&mut self.__internal_obj, "spawned")
            }
        }
    }
    type TypedSigDespawned < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigDespawned < 'c, C: WithSignals > {
        typed: TypedSigDespawned < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDespawned < 'c, C > {
        pub fn emit(&mut self, node: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((node,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDespawned < 'c, C > {
        type Target = TypedSigDespawned < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDespawned < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSpawned < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigSpawned < 'c, C: WithSignals > {
        typed: TypedSigSpawned < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSpawned < 'c, C > {
        pub fn emit(&mut self, node: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((node,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSpawned < 'c, C > {
        type Target = TypedSigSpawned < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSpawned < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for MultiplayerSpawner {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfMultiplayerSpawner < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfMultiplayerSpawner < 'c, C > {
        type Target = < < MultiplayerSpawner as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = MultiplayerSpawner;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfMultiplayerSpawner < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = MultiplayerSpawner;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}