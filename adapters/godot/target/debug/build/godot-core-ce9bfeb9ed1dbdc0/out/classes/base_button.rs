#![doc = "Sidecar module for class [`BaseButton`][crate::classes::BaseButton].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `BaseButton` enums](https://docs.godotengine.org/en/stable/classes/class_basebutton.html#enumerations).\n\n"]
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
    #[doc = "Godot class `BaseButton.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`base_button`][crate::classes::base_button]: sidecar module with related enum/flag types\n* [`IBaseButton`][crate::classes::IBaseButton]: virtual methods\n* [`SignalsOfBaseButton`][crate::classes::base_button::SignalsOfBaseButton]: signal collection\n\n\nSee also [Godot docs for `BaseButton`](https://docs.godotengine.org/en/stable/classes/class_basebutton.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`BaseButton::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct BaseButton {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`BaseButton`][crate::classes::BaseButton].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `BaseButton` methods](https://docs.godotengine.org/en/stable/classes/class_basebutton.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IBaseButton: crate::obj::GodotClass < Base = BaseButton > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ControlNotification) {
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
        fn pressed(&mut self,) {
            unimplemented !()
        }
        fn toggled(&mut self, toggled_on: bool,) {
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
    impl BaseButton {
        pub fn set_pressed(&mut self, pressed: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "set_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_pressed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "is_pressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pressed_no_signal(&mut self, pressed: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (pressed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "set_pressed_no_signal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_hovered(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "is_hovered", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_toggle_mode(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "set_toggle_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_toggle_mode(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "is_toggle_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shortcut_in_tooltip(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "set_shortcut_in_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_shortcut_in_tooltip_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "is_shortcut_in_tooltip_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_disabled(&mut self, disabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "set_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_disabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "is_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_action_mode(&mut self, mode: crate::classes::base_button::ActionMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::base_button::ActionMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "set_action_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_action_mode(&self,) -> crate::classes::base_button::ActionMode {
            type CallRet = crate::classes::base_button::ActionMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "get_action_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_button_mask(&mut self, mask: crate::global::MouseButtonMask,) {
            type CallRet = ();
            type CallParams = (crate::global::MouseButtonMask,);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "set_button_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_button_mask(&self,) -> crate::global::MouseButtonMask {
            type CallRet = crate::global::MouseButtonMask;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "get_button_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_draw_mode(&self,) -> crate::classes::base_button::DrawMode {
            type CallRet = crate::classes::base_button::DrawMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "get_draw_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_keep_pressed_outside(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "set_keep_pressed_outside", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_keep_pressed_outside(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "is_keep_pressed_outside", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shortcut_feedback(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "set_shortcut_feedback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_shortcut_feedback(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "is_shortcut_feedback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shortcut(&mut self, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Shortcut > > >,);
            let args = (shortcut.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "set_shortcut", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shortcut(&self,) -> Option < Gd < crate::classes::Shortcut > > {
            type CallRet = Option < Gd < crate::classes::Shortcut > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "get_shortcut", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_button_group(&mut self, button_group: impl AsArg < Option < Gd < crate::classes::ButtonGroup >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::ButtonGroup > > >,);
            let args = (button_group.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "set_button_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_button_group(&self,) -> Option < Gd < crate::classes::ButtonGroup > > {
            type CallRet = Option < Gd < crate::classes::ButtonGroup > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "BaseButton", "get_button_group", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for BaseButton {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("BaseButton"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for BaseButton {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for BaseButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for BaseButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for BaseButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for BaseButton {
        
    }
    impl crate::obj::cap::GodotDefault for BaseButton {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for BaseButton {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for BaseButton {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`BaseButton`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_BaseButton__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::BaseButton > for $Class {
                
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
pub struct DrawMode {
    ord: i32
}
impl DrawMode {
    #[doc(alias = "DRAW_NORMAL")]
    #[doc = "Godot enumerator name: `DRAW_NORMAL`"]
    pub const NORMAL: DrawMode = DrawMode {
        ord: 0i32
    };
    #[doc(alias = "DRAW_PRESSED")]
    #[doc = "Godot enumerator name: `DRAW_PRESSED`"]
    pub const PRESSED: DrawMode = DrawMode {
        ord: 1i32
    };
    #[doc(alias = "DRAW_HOVER")]
    #[doc = "Godot enumerator name: `DRAW_HOVER`"]
    pub const HOVER: DrawMode = DrawMode {
        ord: 2i32
    };
    #[doc(alias = "DRAW_DISABLED")]
    #[doc = "Godot enumerator name: `DRAW_DISABLED`"]
    pub const DISABLED: DrawMode = DrawMode {
        ord: 3i32
    };
    #[doc(alias = "DRAW_HOVER_PRESSED")]
    #[doc = "Godot enumerator name: `DRAW_HOVER_PRESSED`"]
    pub const HOVER_PRESSED: DrawMode = DrawMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for DrawMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DrawMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DrawMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::NORMAL => "NORMAL", Self::PRESSED => "PRESSED", Self::HOVER => "HOVER", Self::DISABLED => "DISABLED", Self::HOVER_PRESSED => "HOVER_PRESSED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DrawMode::NORMAL, DrawMode::PRESSED, DrawMode::HOVER, DrawMode::DISABLED, DrawMode::HOVER_PRESSED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DrawMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NORMAL", "DRAW_NORMAL", DrawMode::NORMAL), crate::meta::inspect::EnumConstant::new("PRESSED", "DRAW_PRESSED", DrawMode::PRESSED), crate::meta::inspect::EnumConstant::new("HOVER", "DRAW_HOVER", DrawMode::HOVER), crate::meta::inspect::EnumConstant::new("DISABLED", "DRAW_DISABLED", DrawMode::DISABLED), crate::meta::inspect::EnumConstant::new("HOVER_PRESSED", "DRAW_HOVER_PRESSED", DrawMode::HOVER_PRESSED)]
        }
    }
}
impl crate::meta::GodotConvert for DrawMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DrawMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DrawMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ActionMode {
    ord: i32
}
impl ActionMode {
    #[doc(alias = "ACTION_MODE_BUTTON_PRESS")]
    #[doc = "Godot enumerator name: `ACTION_MODE_BUTTON_PRESS`"]
    pub const PRESS: ActionMode = ActionMode {
        ord: 0i32
    };
    #[doc(alias = "ACTION_MODE_BUTTON_RELEASE")]
    #[doc = "Godot enumerator name: `ACTION_MODE_BUTTON_RELEASE`"]
    pub const RELEASE: ActionMode = ActionMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for ActionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ActionMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ActionMode {
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
            Self::PRESS => "PRESS", Self::RELEASE => "RELEASE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ActionMode::PRESS, ActionMode::RELEASE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ActionMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PRESS", "ACTION_MODE_BUTTON_PRESS", ActionMode::PRESS), crate::meta::inspect::EnumConstant::new("RELEASE", "ACTION_MODE_BUTTON_RELEASE", ActionMode::RELEASE)]
        }
    }
}
impl crate::meta::GodotConvert for ActionMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ActionMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ActionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::BaseButton;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`BaseButton`][crate::classes::BaseButton] class."]
    pub struct SignalsOfBaseButton < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfBaseButton < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn pressed(&mut self) -> SigPressed < 'c, C > {
            SigPressed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "pressed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn button_up(&mut self) -> SigButtonUp < 'c, C > {
            SigButtonUp {
                typed: TypedSignal::extract(&mut self.__internal_obj, "button_up")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn button_down(&mut self) -> SigButtonDown < 'c, C > {
            SigButtonDown {
                typed: TypedSignal::extract(&mut self.__internal_obj, "button_down")
            }
        }
        #[doc = "Signature: `(toggled_on: bool)`"]
        pub fn toggled(&mut self) -> SigToggled < 'c, C > {
            SigToggled {
                typed: TypedSignal::extract(&mut self.__internal_obj, "toggled")
            }
        }
    }
    type TypedSigPressed < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigPressed < 'c, C: WithSignals > {
        typed: TypedSigPressed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPressed < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPressed < 'c, C > {
        type Target = TypedSigPressed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPressed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigButtonUp < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigButtonUp < 'c, C: WithSignals > {
        typed: TypedSigButtonUp < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigButtonUp < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigButtonUp < 'c, C > {
        type Target = TypedSigButtonUp < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigButtonUp < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigButtonDown < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigButtonDown < 'c, C: WithSignals > {
        typed: TypedSigButtonDown < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigButtonDown < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigButtonDown < 'c, C > {
        type Target = TypedSigButtonDown < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigButtonDown < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigToggled < 'c, C > = TypedSignal < 'c, C, (bool,) >;
    pub struct SigToggled < 'c, C: WithSignals > {
        typed: TypedSigToggled < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigToggled < 'c, C > {
        pub fn emit(&mut self, toggled_on: bool,) {
            self.typed.emit_tuple((toggled_on,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigToggled < 'c, C > {
        type Target = TypedSigToggled < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigToggled < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for BaseButton {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfBaseButton < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfBaseButton < 'c, C > {
        type Target = < < BaseButton as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = BaseButton;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfBaseButton < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = BaseButton;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}