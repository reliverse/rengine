#![doc = "Sidecar module for class [`AnimationNodeBlendTree`][crate::classes::AnimationNodeBlendTree].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeBlendTree` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendtree.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNodeBlendTree.`\n\nInherits [`AnimationRootNode`][crate::classes::AnimationRootNode].\n\nRelated symbols:\n\n* [`animation_node_blend_tree`][crate::classes::animation_node_blend_tree]: sidecar module with related enum/flag types\n* [`IAnimationNodeBlendTree`][crate::classes::IAnimationNodeBlendTree]: virtual methods\n* [`SignalsOfAnimationNodeBlendTree`][crate::classes::animation_node_blend_tree::SignalsOfAnimationNodeBlendTree]: signal collection\n\n\nSee also [Godot docs for `AnimationNodeBlendTree`](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendtree.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AnimationNodeBlendTree::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNodeBlendTree {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AnimationNodeBlendTree`][crate::classes::AnimationNodeBlendTree].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IAnimationRootNode`][crate::classes::IAnimationRootNode] > [`IAnimationNode`][crate::classes::IAnimationNode] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AnimationNodeBlendTree` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendtree.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNodeBlendTree: crate::obj::GodotClass < Base = AnimationNodeBlendTree > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AnimationNodeBlendTree {
        pub(crate) fn add_node_full(&mut self, name: CowArg < StringName >, node: CowArg < Option < Gd < crate::classes::AnimationNode > > >, position: Vector2,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::AnimationNode > > >, Vector2,);
            let args = (name, node, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(386usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendTree", "add_node", Some(self.__validated_obj()), args,)
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
        pub fn get_node(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::AnimationNode > > {
            type CallRet = Option < Gd < crate::classes::AnimationNode > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(387usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendTree", "get_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_node(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(388usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendTree", "remove_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rename_node(&mut self, name: impl AsArg < StringName >, new_name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), new_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(389usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendTree", "rename_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_node(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(390usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendTree", "has_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn connect_node(&mut self, input_node: impl AsArg < StringName >, input_index: i32, output_node: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, i32, CowArg < 'a1, StringName >,);
            let args = (input_node.into_arg(), input_index, output_node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(391usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendTree", "connect_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn disconnect_node(&mut self, input_node: impl AsArg < StringName >, input_index: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, i32,);
            let args = (input_node.into_arg(), input_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(392usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendTree", "disconnect_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_list(&self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(393usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendTree", "get_node_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_node_position(&mut self, name: impl AsArg < StringName >, position: Vector2,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, Vector2,);
            let args = (name.into_arg(), position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(394usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendTree", "set_node_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_position(&self, name: impl AsArg < StringName >,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(395usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendTree", "get_node_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_graph_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(396usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendTree", "set_graph_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_graph_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(397usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendTree", "get_graph_offset", Some(self.__validated_obj()), args,)
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
        pub const CONNECTION_OK: i32 = 0i32;
        pub const CONNECTION_ERROR_NO_INPUT: i32 = 1i32;
        pub const CONNECTION_ERROR_NO_INPUT_INDEX: i32 = 2i32;
        pub const CONNECTION_ERROR_NO_OUTPUT: i32 = 3i32;
        pub const CONNECTION_ERROR_SAME_NODE: i32 = 4i32;
        pub const CONNECTION_ERROR_CONNECTION_EXISTS: i32 = 5i32;
        
    }
    impl crate::obj::GodotClass for AnimationNodeBlendTree {
        type Base = crate::classes::AnimationRootNode;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AnimationNodeBlendTree"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNodeBlendTree {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationRootNode > for AnimationNodeBlendTree {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationNode > for AnimationNodeBlendTree {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AnimationNodeBlendTree {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AnimationNodeBlendTree {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationNodeBlendTree {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNodeBlendTree {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNodeBlendTree {
        type Target = crate::classes::AnimationRootNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNodeBlendTree {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimationNodeBlendTree`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AnimationNodeBlendTree__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationNodeBlendTree > for $Class {
                
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
#[doc = "Default-param extender for [`AnimationNodeBlendTree::add_node_ex`][super::AnimationNodeBlendTree::add_node_ex]."]
#[must_use]
pub struct ExAddNode < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimationNodeBlendTree, name: CowArg < 'a, StringName >, node: CowArg < 'a, Option < Gd < crate::classes::AnimationNode > > >, position: Vector2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddNode < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationNodeBlendTree, name: impl AsArg < StringName > + 'a, node: impl AsArg < Option < Gd < crate::classes::AnimationNode >> > + 'a,) -> Self {
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
        re_export::AnimationNodeBlendTree::add_node_full(surround_object, name, node, position,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AnimationNodeBlendTree;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`AnimationNodeBlendTree`][crate::classes::AnimationNodeBlendTree] class."]
    pub struct SignalsOfAnimationNodeBlendTree < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfAnimationNodeBlendTree < 'c, C > {
        #[doc = "Signature: `(node_name: StringName)`"]
        pub fn node_changed(&mut self) -> SigNodeChanged < 'c, C > {
            SigNodeChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "node_changed")
            }
        }
    }
    type TypedSigNodeChanged < 'c, C > = TypedSignal < 'c, C, (StringName,) >;
    pub struct SigNodeChanged < 'c, C: WithSignals > {
        typed: TypedSigNodeChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigNodeChanged < 'c, C > {
        pub fn emit(&mut self, node_name: StringName,) {
            self.typed.emit_tuple((node_name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigNodeChanged < 'c, C > {
        type Target = TypedSigNodeChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigNodeChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for AnimationNodeBlendTree {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAnimationNodeBlendTree < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfAnimationNodeBlendTree < 'c, C > {
        type Target = < < AnimationNodeBlendTree as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = AnimationNodeBlendTree;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfAnimationNodeBlendTree < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = AnimationNodeBlendTree;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}