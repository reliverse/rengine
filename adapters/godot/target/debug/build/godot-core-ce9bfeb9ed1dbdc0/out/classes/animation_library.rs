#![doc = "Sidecar module for class [`AnimationLibrary`][crate::classes::AnimationLibrary].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationLibrary` enums](https://docs.godotengine.org/en/stable/classes/class_animationlibrary.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationLibrary.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`animation_library`][crate::classes::animation_library]: sidecar module with related enum/flag types\n* [`IAnimationLibrary`][crate::classes::IAnimationLibrary]: virtual methods\n* [`SignalsOfAnimationLibrary`][crate::classes::animation_library::SignalsOfAnimationLibrary]: signal collection\n\n\nSee also [Godot docs for `AnimationLibrary`](https://docs.godotengine.org/en/stable/classes/class_animationlibrary.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AnimationLibrary::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationLibrary {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AnimationLibrary`][crate::classes::AnimationLibrary].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AnimationLibrary` methods](https://docs.godotengine.org/en/stable/classes/class_animationlibrary.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationLibrary: crate::obj::GodotClass < Base = AnimationLibrary > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AnimationLibrary {
        pub fn add_animation(&mut self, name: impl AsArg < StringName >, animation: impl AsArg < Option < Gd < crate::classes::Animation >> >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::Animation > > >,);
            let args = (name.into_arg(), animation.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationLibrary", "add_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_animation(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationLibrary", "remove_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rename_animation(&mut self, name: impl AsArg < StringName >, newname: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), newname.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationLibrary", "rename_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_animation(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationLibrary", "has_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_animation(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Animation > > {
            type CallRet = Option < Gd < crate::classes::Animation > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationLibrary", "get_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_animation_list(&self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationLibrary", "get_animation_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_animation_list_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationLibrary", "get_animation_list_size", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AnimationLibrary {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AnimationLibrary"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationLibrary {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AnimationLibrary {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AnimationLibrary {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationLibrary {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationLibrary {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationLibrary {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationLibrary {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimationLibrary`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AnimationLibrary__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationLibrary > for $Class {
                
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AnimationLibrary;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`AnimationLibrary`][crate::classes::AnimationLibrary] class."]
    pub struct SignalsOfAnimationLibrary < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfAnimationLibrary < 'c, C > {
        #[doc = "Signature: `(name: StringName)`"]
        pub fn animation_added(&mut self) -> SigAnimationAdded < 'c, C > {
            SigAnimationAdded {
                typed: TypedSignal::extract(&mut self.__internal_obj, "animation_added")
            }
        }
        #[doc = "Signature: `(name: StringName)`"]
        pub fn animation_removed(&mut self) -> SigAnimationRemoved < 'c, C > {
            SigAnimationRemoved {
                typed: TypedSignal::extract(&mut self.__internal_obj, "animation_removed")
            }
        }
        #[doc = "Signature: `(name: StringName, to_name: StringName)`"]
        pub fn animation_renamed(&mut self) -> SigAnimationRenamed < 'c, C > {
            SigAnimationRenamed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "animation_renamed")
            }
        }
        #[doc = "Signature: `(name: StringName)`"]
        pub fn animation_changed(&mut self) -> SigAnimationChanged < 'c, C > {
            SigAnimationChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "animation_changed")
            }
        }
    }
    type TypedSigAnimationAdded < 'c, C > = TypedSignal < 'c, C, (StringName,) >;
    pub struct SigAnimationAdded < 'c, C: WithSignals > {
        typed: TypedSigAnimationAdded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationAdded < 'c, C > {
        pub fn emit(&mut self, name: StringName,) {
            self.typed.emit_tuple((name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationAdded < 'c, C > {
        type Target = TypedSigAnimationAdded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationAdded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAnimationRemoved < 'c, C > = TypedSignal < 'c, C, (StringName,) >;
    pub struct SigAnimationRemoved < 'c, C: WithSignals > {
        typed: TypedSigAnimationRemoved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationRemoved < 'c, C > {
        pub fn emit(&mut self, name: StringName,) {
            self.typed.emit_tuple((name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationRemoved < 'c, C > {
        type Target = TypedSigAnimationRemoved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationRemoved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAnimationRenamed < 'c, C > = TypedSignal < 'c, C, (StringName, StringName,) >;
    pub struct SigAnimationRenamed < 'c, C: WithSignals > {
        typed: TypedSigAnimationRenamed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationRenamed < 'c, C > {
        pub fn emit(&mut self, name: StringName, to_name: StringName,) {
            self.typed.emit_tuple((name, to_name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationRenamed < 'c, C > {
        type Target = TypedSigAnimationRenamed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationRenamed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAnimationChanged < 'c, C > = TypedSignal < 'c, C, (StringName,) >;
    pub struct SigAnimationChanged < 'c, C: WithSignals > {
        typed: TypedSigAnimationChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationChanged < 'c, C > {
        pub fn emit(&mut self, name: StringName,) {
            self.typed.emit_tuple((name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationChanged < 'c, C > {
        type Target = TypedSigAnimationChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for AnimationLibrary {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAnimationLibrary < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfAnimationLibrary < 'c, C > {
        type Target = < < AnimationLibrary as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = AnimationLibrary;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfAnimationLibrary < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = AnimationLibrary;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}