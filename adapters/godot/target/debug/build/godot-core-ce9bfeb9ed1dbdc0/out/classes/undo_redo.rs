#![doc = "Sidecar module for class [`UndoRedo`][crate::classes::UndoRedo].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `UndoRedo` enums](https://docs.godotengine.org/en/stable/classes/class_undoredo.html#enumerations).\n\n"]
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
    #[doc = "Godot class `UndoRedo.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`undo_redo`][crate::classes::undo_redo]: sidecar module with related enum/flag types\n* [`IUndoRedo`][crate::classes::IUndoRedo]: virtual methods\n* [`SignalsOfUndoRedo`][crate::classes::undo_redo::SignalsOfUndoRedo]: signal collection\n\n\nSee also [Godot docs for `UndoRedo`](https://docs.godotengine.org/en/stable/classes/class_undoredo.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`UndoRedo::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct UndoRedo {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`UndoRedo`][crate::classes::UndoRedo].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `UndoRedo` methods](https://docs.godotengine.org/en/stable/classes/class_undoredo.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IUndoRedo: crate::obj::GodotClass < Base = UndoRedo > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl UndoRedo {
        pub(crate) fn create_action_full(&mut self, name: CowArg < GString >, merge_mode: crate::classes::undo_redo::MergeMode, backward_undo_ops: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::undo_redo::MergeMode, bool,);
            let args = (name, merge_mode, backward_undo_ops,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "create_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_action_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_action(&mut self, name: impl AsArg < GString >,) {
            self.create_action_ex(name,) . done()
        }
        #[inline]
        pub fn create_action_ex < 'a > (&'a mut self, name: impl AsArg < GString > + 'a,) -> ExCreateAction < 'a > {
            ExCreateAction::new(self, name,)
        }
        pub(crate) fn commit_action_full(&mut self, execute: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (execute,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "commit_action", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::commit_action_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn commit_action(&mut self,) {
            self.commit_action_ex() . done()
        }
        #[inline]
        pub fn commit_action_ex < 'a > (&'a mut self,) -> ExCommitAction < 'a > {
            ExCommitAction::new(self,)
        }
        pub fn is_committing_action(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "is_committing_action", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_do_method(&mut self, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10342usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "add_do_method", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_undo_method(&mut self, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10343usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "add_undo_method", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_do_property(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, property: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, CowArg < 'a1, StringName >, RefArg < 'a2, Variant >,);
            let args = (object.into_arg(), property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10344usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "add_do_property", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_undo_property(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, property: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, CowArg < 'a1, StringName >, RefArg < 'a2, Variant >,);
            let args = (object.into_arg(), property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10345usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "add_undo_property", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_do_reference(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >,);
            let args = (object.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10346usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "add_do_reference", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_undo_reference(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >,);
            let args = (object.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10347usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "add_undo_reference", Some(self.__validated_obj()), args,)
            }
        }
        pub fn start_force_keep_in_merge_ends(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10348usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "start_force_keep_in_merge_ends", Some(self.__validated_obj()), args,)
            }
        }
        pub fn end_force_keep_in_merge_ends(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10349usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "end_force_keep_in_merge_ends", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_history_count(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10350usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "get_history_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_action(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10351usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "get_current_action", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_action_name(&mut self, id: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10352usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "get_action_name", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn clear_history_full(&mut self, increase_version: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (increase_version,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10353usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "clear_history", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::clear_history_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn clear_history(&mut self,) {
            self.clear_history_ex() . done()
        }
        #[inline]
        pub fn clear_history_ex < 'a > (&'a mut self,) -> ExClearHistory < 'a > {
            ExClearHistory::new(self,)
        }
        pub fn get_current_action_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10354usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "get_current_action_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_undo(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10355usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "has_undo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_redo(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10356usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "has_redo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_version(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10357usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "get_version", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_steps(&mut self, max_steps: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_steps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10358usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "set_max_steps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_steps(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "get_max_steps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn redo(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "redo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn undo(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10361usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UndoRedo", "undo", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for UndoRedo {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("UndoRedo"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for UndoRedo {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for UndoRedo {
        
    }
    impl crate::obj::cap::GodotDefault for UndoRedo {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for UndoRedo {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for UndoRedo {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`UndoRedo`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_UndoRedo__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::UndoRedo > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`UndoRedo::create_action_ex`][super::UndoRedo::create_action_ex]."]
#[must_use]
pub struct ExCreateAction < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::UndoRedo, name: CowArg < 'a, GString >, merge_mode: crate::classes::undo_redo::MergeMode, backward_undo_ops: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateAction < 'a > {
    fn new(surround_object: &'a mut re_export::UndoRedo, name: impl AsArg < GString > + 'a,) -> Self {
        let merge_mode = crate::obj::EngineEnum::from_ord(0);
        let backward_undo_ops = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), merge_mode: merge_mode, backward_undo_ops: backward_undo_ops,
        }
    }
    #[inline]
    pub fn merge_mode(self, merge_mode: crate::classes::undo_redo::MergeMode) -> Self {
        Self {
            merge_mode: merge_mode, .. self
        }
    }
    #[inline]
    pub fn backward_undo_ops(self, backward_undo_ops: bool) -> Self {
        Self {
            backward_undo_ops: backward_undo_ops, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, merge_mode, backward_undo_ops,
        }
        = self;
        re_export::UndoRedo::create_action_full(surround_object, name, merge_mode, backward_undo_ops,)
    }
}
#[doc = "Default-param extender for [`UndoRedo::commit_action_ex`][super::UndoRedo::commit_action_ex]."]
#[must_use]
pub struct ExCommitAction < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::UndoRedo, execute: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCommitAction < 'a > {
    fn new(surround_object: &'a mut re_export::UndoRedo,) -> Self {
        let execute = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, execute: execute,
        }
    }
    #[inline]
    pub fn execute(self, execute: bool) -> Self {
        Self {
            execute: execute, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, execute,
        }
        = self;
        re_export::UndoRedo::commit_action_full(surround_object, execute,)
    }
}
#[doc = "Default-param extender for [`UndoRedo::clear_history_ex`][super::UndoRedo::clear_history_ex]."]
#[must_use]
pub struct ExClearHistory < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::UndoRedo, increase_version: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClearHistory < 'a > {
    fn new(surround_object: &'a mut re_export::UndoRedo,) -> Self {
        let increase_version = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, increase_version: increase_version,
        }
    }
    #[inline]
    pub fn increase_version(self, increase_version: bool) -> Self {
        Self {
            increase_version: increase_version, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, increase_version,
        }
        = self;
        re_export::UndoRedo::clear_history_full(surround_object, increase_version,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MergeMode {
    ord: i32
}
impl MergeMode {
    #[doc(alias = "MERGE_DISABLE")]
    #[doc = "Godot enumerator name: `MERGE_DISABLE`"]
    pub const DISABLE: MergeMode = MergeMode {
        ord: 0i32
    };
    #[doc(alias = "MERGE_ENDS")]
    #[doc = "Godot enumerator name: `MERGE_ENDS`"]
    pub const ENDS: MergeMode = MergeMode {
        ord: 1i32
    };
    #[doc(alias = "MERGE_ALL")]
    #[doc = "Godot enumerator name: `MERGE_ALL`"]
    pub const ALL: MergeMode = MergeMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for MergeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MergeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MergeMode {
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
            Self::DISABLE => "DISABLE", Self::ENDS => "ENDS", Self::ALL => "ALL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MergeMode::DISABLE, MergeMode::ENDS, MergeMode::ALL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MergeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLE", "MERGE_DISABLE", MergeMode::DISABLE), crate::meta::inspect::EnumConstant::new("ENDS", "MERGE_ENDS", MergeMode::ENDS), crate::meta::inspect::EnumConstant::new("ALL", "MERGE_ALL", MergeMode::ALL)]
        }
    }
}
impl crate::meta::GodotConvert for MergeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MergeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MergeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::UndoRedo;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`UndoRedo`][crate::classes::UndoRedo] class."]
    pub struct SignalsOfUndoRedo < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfUndoRedo < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn version_changed(&mut self) -> SigVersionChanged < 'c, C > {
            SigVersionChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "version_changed")
            }
        }
    }
    type TypedSigVersionChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigVersionChanged < 'c, C: WithSignals > {
        typed: TypedSigVersionChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigVersionChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigVersionChanged < 'c, C > {
        type Target = TypedSigVersionChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigVersionChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for UndoRedo {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfUndoRedo < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfUndoRedo < 'c, C > {
        type Target = < < UndoRedo as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = UndoRedo;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfUndoRedo < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = UndoRedo;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}