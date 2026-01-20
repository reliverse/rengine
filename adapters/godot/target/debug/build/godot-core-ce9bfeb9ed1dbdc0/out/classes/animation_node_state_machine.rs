#![doc = "Sidecar module for class [`AnimationNodeStateMachine`][crate::classes::AnimationNodeStateMachine].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeStateMachine` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachine.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNodeStateMachine.`\n\nInherits [`AnimationRootNode`][crate::classes::AnimationRootNode].\n\nRelated symbols:\n\n* [`animation_node_state_machine`][crate::classes::animation_node_state_machine]: sidecar module with related enum/flag types\n* [`IAnimationNodeStateMachine`][crate::classes::IAnimationNodeStateMachine]: virtual methods\n\n\nSee also [Godot docs for `AnimationNodeStateMachine`](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachine.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AnimationNodeStateMachine::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNodeStateMachine {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AnimationNodeStateMachine`][crate::classes::AnimationNodeStateMachine].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IAnimationRootNode`][crate::classes::IAnimationRootNode] > [`IAnimationNode`][crate::classes::IAnimationNode] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AnimationNodeStateMachine` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachine.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNodeStateMachine: crate::obj::GodotClass < Base = AnimationNodeStateMachine > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AnimationNodeStateMachine {
        pub(crate) fn add_node_full(&mut self, name: CowArg < StringName >, node: CowArg < Option < Gd < crate::classes::AnimationNode > > >, position: Vector2,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::AnimationNode > > >, Vector2,);
            let args = (name, node, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "add_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_node_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_node(&mut self, name: impl AsArg < StringName >, node: impl AsArg < Option < Gd < crate::classes::AnimationNode >> >,) {
            self.add_node_ex(name, node,) . done()
        }
        #[inline]
        pub fn add_node_ex < 'a > (&'a mut self, name: impl AsArg < StringName > + 'a, node: impl AsArg < Option < Gd < crate::classes::AnimationNode >> > + 'a,) -> ExAddNode < 'a > {
            ExAddNode::new(self, name, node,)
        }
        pub fn replace_node(&mut self, name: impl AsArg < StringName >, node: impl AsArg < Option < Gd < crate::classes::AnimationNode >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::AnimationNode > > >,);
            let args = (name.into_arg(), node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "replace_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::AnimationNode > > {
            type CallRet = Option < Gd < crate::classes::AnimationNode > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(420usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_node(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(421usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "remove_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rename_node(&mut self, name: impl AsArg < StringName >, new_name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), new_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(422usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "rename_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_node(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(423usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "has_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_name(&self, node: impl AsArg < Option < Gd < crate::classes::AnimationNode >> >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::AnimationNode > > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(424usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_node_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_list(&self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(425usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_node_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_node_position(&mut self, name: impl AsArg < StringName >, position: Vector2,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, Vector2,);
            let args = (name.into_arg(), position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(426usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "set_node_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_position(&self, name: impl AsArg < StringName >,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(427usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_node_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_transition(&self, from: impl AsArg < StringName >, to: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (from.into_arg(), to.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(428usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "has_transition", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_transition(&mut self, from: impl AsArg < StringName >, to: impl AsArg < StringName >, transition: impl AsArg < Option < Gd < crate::classes::AnimationNodeStateMachineTransition >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, Option < Gd < crate::classes::AnimationNodeStateMachineTransition > > >,);
            let args = (from.into_arg(), to.into_arg(), transition.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(429usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "add_transition", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transition(&self, idx: i32,) -> Option < Gd < crate::classes::AnimationNodeStateMachineTransition > > {
            type CallRet = Option < Gd < crate::classes::AnimationNodeStateMachineTransition > >;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(430usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_transition", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transition_from(&self, idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(431usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_transition_from", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transition_to(&self, idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(432usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_transition_to", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transition_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(433usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_transition_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_transition_by_index(&mut self, idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(434usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "remove_transition_by_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_transition(&mut self, from: impl AsArg < StringName >, to: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (from.into_arg(), to.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(435usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "remove_transition", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_graph_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(436usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "set_graph_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_graph_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(437usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_graph_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_state_machine_type(&mut self, state_machine_type: crate::classes::animation_node_state_machine::StateMachineType,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation_node_state_machine::StateMachineType,);
            let args = (state_machine_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(438usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "set_state_machine_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_state_machine_type(&self,) -> crate::classes::animation_node_state_machine::StateMachineType {
            type CallRet = crate::classes::animation_node_state_machine::StateMachineType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(439usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "get_state_machine_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_transition_to_self(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(440usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "set_allow_transition_to_self", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_allow_transition_to_self(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(441usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "is_allow_transition_to_self", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_reset_ends(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(442usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "set_reset_ends", Some(self.__validated_obj()), args,)
            }
        }
        pub fn are_ends_reset(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(443usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeStateMachine", "are_ends_reset", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AnimationNodeStateMachine {
        type Base = crate::classes::AnimationRootNode;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AnimationNodeStateMachine"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNodeStateMachine {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationRootNode > for AnimationNodeStateMachine {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationNode > for AnimationNodeStateMachine {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AnimationNodeStateMachine {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AnimationNodeStateMachine {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationNodeStateMachine {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNodeStateMachine {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNodeStateMachine {
        type Target = crate::classes::AnimationRootNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNodeStateMachine {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimationNodeStateMachine`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AnimationNodeStateMachine__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationNodeStateMachine > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationRootNode > for $Class {
                
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
#[doc = "Default-param extender for [`AnimationNodeStateMachine::add_node_ex`][super::AnimationNodeStateMachine::add_node_ex]."]
#[must_use]
pub struct ExAddNode < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimationNodeStateMachine, name: CowArg < 'a, StringName >, node: CowArg < 'a, Option < Gd < crate::classes::AnimationNode > > >, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddNode < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationNodeStateMachine, name: impl AsArg < StringName > + 'a, node: impl AsArg < Option < Gd < crate::classes::AnimationNode >> > + 'a,) -> Self {
        let position = Vector2::new(0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), node: node.into_arg(), position: position,
        }
    }
    #[inline]
    pub fn position(self, position: Vector2) -> Self {
        Self {
            position: position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, node, position,
        }
        = self;
        re_export::AnimationNodeStateMachine::add_node_full(surround_object, name, node, position,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct StateMachineType {
    ord: i32
}
impl StateMachineType {
    #[doc(alias = "STATE_MACHINE_TYPE_ROOT")]
    #[doc = "Godot enumerator name: `STATE_MACHINE_TYPE_ROOT`"]
    pub const ROOT: StateMachineType = StateMachineType {
        ord: 0i32
    };
    #[doc(alias = "STATE_MACHINE_TYPE_NESTED")]
    #[doc = "Godot enumerator name: `STATE_MACHINE_TYPE_NESTED`"]
    pub const NESTED: StateMachineType = StateMachineType {
        ord: 1i32
    };
    #[doc(alias = "STATE_MACHINE_TYPE_GROUPED")]
    #[doc = "Godot enumerator name: `STATE_MACHINE_TYPE_GROUPED`"]
    pub const GROUPED: StateMachineType = StateMachineType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for StateMachineType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("StateMachineType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for StateMachineType {
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
            Self::ROOT => "ROOT", Self::NESTED => "NESTED", Self::GROUPED => "GROUPED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[StateMachineType::ROOT, StateMachineType::NESTED, StateMachineType::GROUPED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < StateMachineType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ROOT", "STATE_MACHINE_TYPE_ROOT", StateMachineType::ROOT), crate::meta::inspect::EnumConstant::new("NESTED", "STATE_MACHINE_TYPE_NESTED", StateMachineType::NESTED), crate::meta::inspect::EnumConstant::new("GROUPED", "STATE_MACHINE_TYPE_GROUPED", StateMachineType::GROUPED)]
        }
    }
}
impl crate::meta::GodotConvert for StateMachineType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for StateMachineType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for StateMachineType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AnimationNodeStateMachine;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::animation_node::SignalsOfAnimationNode;
    impl WithSignals for AnimationNodeStateMachine {
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