#![doc = "Sidecar module for class [`AnimationNodeOneShot`][crate::classes::AnimationNodeOneShot].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeOneShot` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodeoneshot.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNodeOneShot.`\n\nInherits [`AnimationNodeSync`][crate::classes::AnimationNodeSync].\n\nRelated symbols:\n\n* [`animation_node_one_shot`][crate::classes::animation_node_one_shot]: sidecar module with related enum/flag types\n* [`IAnimationNodeOneShot`][crate::classes::IAnimationNodeOneShot]: virtual methods\n\n\nSee also [Godot docs for `AnimationNodeOneShot`](https://docs.godotengine.org/en/stable/classes/class_animationnodeoneshot.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AnimationNodeOneShot::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNodeOneShot {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AnimationNodeOneShot`][crate::classes::AnimationNodeOneShot].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IAnimationNodeSync`][crate::classes::IAnimationNodeSync] > [`IAnimationNode`][crate::classes::IAnimationNode] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AnimationNodeOneShot` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodeoneshot.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNodeOneShot: crate::obj::GodotClass < Base = AnimationNodeOneShot > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_child_nodes(&self,) -> VarDictionary {
            unimplemented !()
        }
        fn get_parameter_list(&self,) -> VarArray {
            unimplemented !()
        }
        fn get_child_by_name(&self, name: StringName,) -> Option < Gd < crate::classes::AnimationNode > > {
            unimplemented !()
        }
        fn get_parameter_default_value(&self, parameter: StringName,) -> Variant {
            unimplemented !()
        }
        fn is_parameter_read_only(&self, parameter: StringName,) -> bool {
            unimplemented !()
        }
        fn process(&mut self, time: f64, seek: bool, is_external_seeking: bool, test_only: bool,) -> f64 {
            unimplemented !()
        }
        fn get_caption(&self,) -> GString {
            unimplemented !()
        }
        fn has_filter(&self,) -> bool {
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
    impl AnimationNodeOneShot {
        pub fn set_fadein_time(&mut self, time: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(400usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "set_fadein_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fadein_time(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(401usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "get_fadein_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fadein_curve(&mut self, curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Curve > > >,);
            let args = (curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(402usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "set_fadein_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fadein_curve(&self,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(403usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "get_fadein_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fadeout_time(&mut self, time: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "set_fadeout_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fadeout_time(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "get_fadeout_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fadeout_curve(&mut self, curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Curve > > >,);
            let args = (curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "set_fadeout_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fadeout_curve(&self,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "get_fadeout_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_break_loop_at_end(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "set_break_loop_at_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_loop_broken_at_end(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "is_loop_broken_at_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autorestart(&mut self, active: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "set_autorestart", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_autorestart(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "has_autorestart", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autorestart_delay(&mut self, time: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "set_autorestart_delay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autorestart_delay(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "get_autorestart_delay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autorestart_random_delay(&mut self, time: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "set_autorestart_random_delay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autorestart_random_delay(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "get_autorestart_random_delay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mix_mode(&mut self, mode: crate::classes::animation_node_one_shot::MixMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation_node_one_shot::MixMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "set_mix_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mix_mode(&self,) -> crate::classes::animation_node_one_shot::MixMode {
            type CallRet = crate::classes::animation_node_one_shot::MixMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeOneShot", "get_mix_mode", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AnimationNodeOneShot {
        type Base = crate::classes::AnimationNodeSync;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AnimationNodeOneShot"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNodeOneShot {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationNodeSync > for AnimationNodeOneShot {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationNode > for AnimationNodeOneShot {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AnimationNodeOneShot {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AnimationNodeOneShot {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationNodeOneShot {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNodeOneShot {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNodeOneShot {
        type Target = crate::classes::AnimationNodeSync;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNodeOneShot {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimationNodeOneShot`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AnimationNodeOneShot__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationNodeOneShot > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationNodeSync > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationNode > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct OneShotRequest {
    ord: i32
}
impl OneShotRequest {
    #[doc(alias = "ONE_SHOT_REQUEST_NONE")]
    #[doc = "Godot enumerator name: `ONE_SHOT_REQUEST_NONE`"]
    pub const NONE: OneShotRequest = OneShotRequest {
        ord: 0i32
    };
    #[doc(alias = "ONE_SHOT_REQUEST_FIRE")]
    #[doc = "Godot enumerator name: `ONE_SHOT_REQUEST_FIRE`"]
    pub const FIRE: OneShotRequest = OneShotRequest {
        ord: 1i32
    };
    #[doc(alias = "ONE_SHOT_REQUEST_ABORT")]
    #[doc = "Godot enumerator name: `ONE_SHOT_REQUEST_ABORT`"]
    pub const ABORT: OneShotRequest = OneShotRequest {
        ord: 2i32
    };
    #[doc(alias = "ONE_SHOT_REQUEST_FADE_OUT")]
    #[doc = "Godot enumerator name: `ONE_SHOT_REQUEST_FADE_OUT`"]
    pub const FADE_OUT: OneShotRequest = OneShotRequest {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for OneShotRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("OneShotRequest") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for OneShotRequest {
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
            Self::NONE => "NONE", Self::FIRE => "FIRE", Self::ABORT => "ABORT", Self::FADE_OUT => "FADE_OUT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[OneShotRequest::NONE, OneShotRequest::FIRE, OneShotRequest::ABORT, OneShotRequest::FADE_OUT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < OneShotRequest >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "ONE_SHOT_REQUEST_NONE", OneShotRequest::NONE), crate::meta::inspect::EnumConstant::new("FIRE", "ONE_SHOT_REQUEST_FIRE", OneShotRequest::FIRE), crate::meta::inspect::EnumConstant::new("ABORT", "ONE_SHOT_REQUEST_ABORT", OneShotRequest::ABORT), crate::meta::inspect::EnumConstant::new("FADE_OUT", "ONE_SHOT_REQUEST_FADE_OUT", OneShotRequest::FADE_OUT)]
        }
    }
}
impl crate::meta::GodotConvert for OneShotRequest {
    type Via = i32;
    
}
impl crate::meta::ToGodot for OneShotRequest {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for OneShotRequest {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MixMode {
    ord: i32
}
impl MixMode {
    #[doc(alias = "MIX_MODE_BLEND")]
    #[doc = "Godot enumerator name: `MIX_MODE_BLEND`"]
    pub const BLEND: MixMode = MixMode {
        ord: 0i32
    };
    #[doc(alias = "MIX_MODE_ADD")]
    #[doc = "Godot enumerator name: `MIX_MODE_ADD`"]
    pub const ADD: MixMode = MixMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for MixMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MixMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MixMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::BLEND => "BLEND", Self::ADD => "ADD", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MixMode::BLEND, MixMode::ADD]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MixMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BLEND", "MIX_MODE_BLEND", MixMode::BLEND), crate::meta::inspect::EnumConstant::new("ADD", "MIX_MODE_ADD", MixMode::ADD)]
        }
    }
}
impl crate::meta::GodotConvert for MixMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MixMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MixMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AnimationNodeOneShot;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::animation_node::SignalsOfAnimationNode;
    impl WithSignals for AnimationNodeOneShot {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAnimationNode < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}