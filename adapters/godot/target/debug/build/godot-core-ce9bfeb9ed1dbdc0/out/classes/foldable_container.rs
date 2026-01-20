#![doc = "Sidecar module for class [`FoldableContainer`][crate::classes::FoldableContainer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FoldableContainer` enums](https://docs.godotengine.org/en/stable/classes/class_foldablecontainer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `FoldableContainer.`\n\nInherits [`Container`][crate::classes::Container].\n\nRelated symbols:\n\n* [`foldable_container`][crate::classes::foldable_container]: sidecar module with related enum/flag types\n* [`IFoldableContainer`][crate::classes::IFoldableContainer]: virtual methods\n* [`SignalsOfFoldableContainer`][crate::classes::foldable_container::SignalsOfFoldableContainer]: signal collection\n\n\nSee also [Godot docs for `FoldableContainer`](https://docs.godotengine.org/en/stable/classes/class_foldablecontainer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`FoldableContainer::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct FoldableContainer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`FoldableContainer`][crate::classes::FoldableContainer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IContainer`][crate::classes::IContainer] > [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `FoldableContainer` methods](https://docs.godotengine.org/en/stable/classes/class_foldablecontainer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IFoldableContainer: crate::obj::GodotClass < Base = FoldableContainer > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ContainerNotification) {
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
        fn get_allowed_size_flags_horizontal(&self,) -> PackedInt32Array {
            unimplemented !()
        }
        fn get_allowed_size_flags_vertical(&self,) -> PackedInt32Array {
            unimplemented !()
        }
        fn has_point(&self, point: Vector2,) -> bool {
            unimplemented !()
        }
        fn structured_text_parser(&self, args: VarArray, text: GString,) -> Array < Vector3i > {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_tooltip(&self, at_position: Vector2,) -> GString {
            unimplemented !()
        }
        fn get_drag_data(&mut self, at_position: Vector2,) -> Variant {
            unimplemented !()
        }
        fn can_drop_data(&self, at_position: Vector2, data: Variant,) -> bool {
            unimplemented !()
        }
        fn drop_data(&mut self, at_position: Vector2, data: Variant,) {
            unimplemented !()
        }
        fn make_custom_tooltip(&self, for_text: GString,) -> Option < Gd < crate::classes::Object > > {
            unimplemented !()
        }
        fn accessibility_get_contextual_info(&self,) -> GString {
            unimplemented !()
        }
        fn get_accessibility_container_name(&self, node: Option < Gd < crate::classes::Node > >,) -> GString {
            unimplemented !()
        }
        fn gui_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
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
    impl FoldableContainer {
        pub fn fold(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3323usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "fold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn expand(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "expand", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_folded(&mut self, folded: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (folded,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "set_folded", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_folded(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "is_folded", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_foldable_group(&mut self, button_group: impl AsArg < Option < Gd < crate::classes::FoldableGroup >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::FoldableGroup > > >,);
            let args = (button_group.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "set_foldable_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_foldable_group(&self,) -> Option < Gd < crate::classes::FoldableGroup > > {
            type CallRet = Option < Gd < crate::classes::FoldableGroup > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "get_foldable_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_title(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "set_title", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_title(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "get_title", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_title_alignment(&mut self, alignment: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (crate::global::HorizontalAlignment,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "set_title_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_title_alignment(&self,) -> crate::global::HorizontalAlignment {
            type CallRet = crate::global::HorizontalAlignment;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "get_title_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "set_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "get_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_title_text_direction(&mut self, text_direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::TextDirection,);
            let args = (text_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "set_title_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_title_text_direction(&self,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "get_title_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_title_text_overrun_behavior(&mut self, overrun_behavior: crate::classes::text_server::OverrunBehavior,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::OverrunBehavior,);
            let args = (overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "set_title_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_title_text_overrun_behavior(&self,) -> crate::classes::text_server::OverrunBehavior {
            type CallRet = crate::classes::text_server::OverrunBehavior;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "get_title_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_title_position(&mut self, title_position: crate::classes::foldable_container::TitlePosition,) {
            type CallRet = ();
            type CallParams = (crate::classes::foldable_container::TitlePosition,);
            let args = (title_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "set_title_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_title_position(&self,) -> crate::classes::foldable_container::TitlePosition {
            type CallRet = crate::classes::foldable_container::TitlePosition;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "get_title_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_title_bar_control(&mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (control.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "add_title_bar_control", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_title_bar_control(&mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (control.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3342usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "FoldableContainer", "remove_title_bar_control", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for FoldableContainer {
        type Base = crate::classes::Container;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("FoldableContainer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for FoldableContainer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Container > for FoldableContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for FoldableContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for FoldableContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for FoldableContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for FoldableContainer {
        
    }
    impl crate::obj::cap::GodotDefault for FoldableContainer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for FoldableContainer {
        type Target = crate::classes::Container;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for FoldableContainer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`FoldableContainer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_FoldableContainer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::FoldableContainer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Container > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Control > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
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
pub struct TitlePosition {
    ord: i32
}
impl TitlePosition {
    #[doc(alias = "POSITION_TOP")]
    #[doc = "Godot enumerator name: `POSITION_TOP`"]
    pub const TOP: TitlePosition = TitlePosition {
        ord: 0i32
    };
    #[doc(alias = "POSITION_BOTTOM")]
    #[doc = "Godot enumerator name: `POSITION_BOTTOM`"]
    pub const BOTTOM: TitlePosition = TitlePosition {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for TitlePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TitlePosition") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TitlePosition {
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
            Self::TOP => "TOP", Self::BOTTOM => "BOTTOM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TitlePosition::TOP, TitlePosition::BOTTOM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TitlePosition >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TOP", "POSITION_TOP", TitlePosition::TOP), crate::meta::inspect::EnumConstant::new("BOTTOM", "POSITION_BOTTOM", TitlePosition::BOTTOM)]
        }
    }
}
impl crate::meta::GodotConvert for TitlePosition {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TitlePosition {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TitlePosition {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::FoldableContainer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`FoldableContainer`][crate::classes::FoldableContainer] class."]
    pub struct SignalsOfFoldableContainer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfFoldableContainer < 'c, C > {
        #[doc = "Signature: `(is_folded: bool)`"]
        pub fn folding_changed(&mut self) -> SigFoldingChanged < 'c, C > {
            SigFoldingChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "folding_changed")
            }
        }
    }
    type TypedSigFoldingChanged < 'c, C > = TypedSignal < 'c, C, (bool,) >;
    pub struct SigFoldingChanged < 'c, C: WithSignals > {
        typed: TypedSigFoldingChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFoldingChanged < 'c, C > {
        pub fn emit(&mut self, is_folded: bool,) {
            self.typed.emit_tuple((is_folded,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFoldingChanged < 'c, C > {
        type Target = TypedSigFoldingChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFoldingChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for FoldableContainer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfFoldableContainer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfFoldableContainer < 'c, C > {
        type Target = < < FoldableContainer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = FoldableContainer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfFoldableContainer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = FoldableContainer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}