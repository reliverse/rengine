#![doc = "Sidecar module for class [`AnimationTree`][crate::classes::AnimationTree].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationTree` enums](https://docs.godotengine.org/en/stable/classes/class_animationtree.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationTree.`\n\nInherits [`AnimationMixer`][crate::classes::AnimationMixer].\n\nRelated symbols:\n\n* [`animation_tree`][crate::classes::animation_tree]: sidecar module with related enum/flag types\n* [`IAnimationTree`][crate::classes::IAnimationTree]: virtual methods\n* [`SignalsOfAnimationTree`][crate::classes::animation_tree::SignalsOfAnimationTree]: signal collection\n\n\nSee also [Godot docs for `AnimationTree`](https://docs.godotengine.org/en/stable/classes/class_animationtree.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`AnimationTree::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationTree {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AnimationTree`][crate::classes::AnimationTree].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IAnimationMixer`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `AnimationTree` methods](https://docs.godotengine.org/en/stable/classes/class_animationtree.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationTree: crate::obj::GodotClass < Base = AnimationTree > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn post_process_key_value(&self, animation: Option < Gd < crate::classes::Animation > >, track: i32, value: Variant, object_id: u64, object_sub_idx: i32,) -> Variant {
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
    impl AnimationTree {
        pub fn set_tree_root(&mut self, animation_node: impl AsArg < Option < Gd < crate::classes::AnimationRootNode >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::AnimationRootNode > > >,);
            let args = (animation_node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(542usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationTree", "set_tree_root", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tree_root(&self,) -> Option < Gd < crate::classes::AnimationRootNode > > {
            type CallRet = Option < Gd < crate::classes::AnimationRootNode > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(543usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationTree", "get_tree_root", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_advance_expression_base_node(&mut self, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(544usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationTree", "set_advance_expression_base_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_advance_expression_base_node(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(545usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationTree", "get_advance_expression_base_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_animation_player(&mut self, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(546usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationTree", "set_animation_player", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_animation_player(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(547usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationTree", "get_animation_player", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_process_callback(&mut self, mode: crate::classes::animation_tree::AnimationProcessCallback,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation_tree::AnimationProcessCallback,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(548usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationTree", "set_process_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_process_callback(&self,) -> crate::classes::animation_tree::AnimationProcessCallback {
            type CallRet = crate::classes::animation_tree::AnimationProcessCallback;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(549usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationTree", "get_process_callback", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AnimationTree {
        type Base = crate::classes::AnimationMixer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AnimationTree"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationTree {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationMixer > for AnimationTree {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for AnimationTree {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationTree {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationTree {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationTree {
        type Target = crate::classes::AnimationMixer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationTree {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimationTree`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AnimationTree__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationTree > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationMixer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnimationProcessCallback {
    ord: i32
}
impl AnimationProcessCallback {
    #[doc(alias = "ANIMATION_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `ANIMATION_PROCESS_PHYSICS`"]
    pub const PHYSICS: AnimationProcessCallback = AnimationProcessCallback {
        ord: 0i32
    };
    #[doc(alias = "ANIMATION_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `ANIMATION_PROCESS_IDLE`"]
    pub const IDLE: AnimationProcessCallback = AnimationProcessCallback {
        ord: 1i32
    };
    #[doc(alias = "ANIMATION_PROCESS_MANUAL")]
    #[doc = "Godot enumerator name: `ANIMATION_PROCESS_MANUAL`"]
    pub const MANUAL: AnimationProcessCallback = AnimationProcessCallback {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AnimationProcessCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnimationProcessCallback") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnimationProcessCallback {
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
            Self::PHYSICS => "PHYSICS", Self::IDLE => "IDLE", Self::MANUAL => "MANUAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AnimationProcessCallback::PHYSICS, AnimationProcessCallback::IDLE, AnimationProcessCallback::MANUAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AnimationProcessCallback >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PHYSICS", "ANIMATION_PROCESS_PHYSICS", AnimationProcessCallback::PHYSICS), crate::meta::inspect::EnumConstant::new("IDLE", "ANIMATION_PROCESS_IDLE", AnimationProcessCallback::IDLE), crate::meta::inspect::EnumConstant::new("MANUAL", "ANIMATION_PROCESS_MANUAL", AnimationProcessCallback::MANUAL)]
        }
    }
}
impl crate::meta::GodotConvert for AnimationProcessCallback {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AnimationProcessCallback {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnimationProcessCallback {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AnimationTree;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`AnimationTree`][crate::classes::AnimationTree] class."]
    pub struct SignalsOfAnimationTree < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfAnimationTree < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn animation_player_changed(&mut self) -> SigAnimationPlayerChanged < 'c, C > {
            SigAnimationPlayerChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "animation_player_changed")
            }
        }
    }
    type TypedSigAnimationPlayerChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigAnimationPlayerChanged < 'c, C: WithSignals > {
        typed: TypedSigAnimationPlayerChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationPlayerChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationPlayerChanged < 'c, C > {
        type Target = TypedSigAnimationPlayerChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationPlayerChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for AnimationTree {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAnimationTree < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfAnimationTree < 'c, C > {
        type Target = < < AnimationTree as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = AnimationTree;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfAnimationTree < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = AnimationTree;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}