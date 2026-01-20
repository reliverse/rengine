#![doc = "Sidecar module for class [`MenuBar`][crate::classes::MenuBar].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MenuBar` enums](https://docs.godotengine.org/en/stable/classes/class_menubar.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MenuBar.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`IMenuBar`][crate::classes::IMenuBar]: virtual methods\n\n\nSee also [Godot docs for `MenuBar`](https://docs.godotengine.org/en/stable/classes/class_menubar.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`MenuBar::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MenuBar {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`MenuBar`][crate::classes::MenuBar].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `MenuBar` methods](https://docs.godotengine.org/en/stable/classes/class_menubar.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMenuBar: crate::obj::GodotClass < Base = MenuBar > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MenuBar {
        pub fn set_switch_on_hover(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "set_switch_on_hover", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_switch_on_hover(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "is_switch_on_hover", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_disable_shortcuts(&mut self, disabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "set_disable_shortcuts", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_prefer_global_menu(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "set_prefer_global_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_prefer_global_menu(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "is_prefer_global_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_native_menu(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "is_native_menu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_menu_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "get_menu_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_direction(&mut self, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (crate::classes::control::TextDirection,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "set_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_direction(&self,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "get_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_language(&mut self, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "set_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_language(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "get_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flat(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "set_flat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_flat(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "is_flat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_start_index(&mut self, enabled: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "set_start_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_start_index(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "get_start_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_menu_title(&mut self, menu: i32, title: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (menu, title.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "set_menu_title", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_menu_title(&self, menu: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (menu,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "get_menu_title", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_menu_tooltip(&mut self, menu: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (menu, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "set_menu_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_menu_tooltip(&self, menu: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (menu,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "get_menu_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_menu_disabled(&mut self, menu: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (menu, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "set_menu_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_menu_disabled(&self, menu: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (menu,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "is_menu_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_menu_hidden(&mut self, menu: i32, hidden: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (menu, hidden,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "set_menu_hidden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_menu_hidden(&self, menu: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (menu,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "is_menu_hidden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_menu_popup(&self, menu: i32,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallRet = Option < Gd < crate::classes::PopupMenu > >;
            type CallParams = (i32,);
            let args = (menu,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MenuBar", "get_menu_popup", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for MenuBar {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("MenuBar"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MenuBar {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for MenuBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for MenuBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for MenuBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MenuBar {
        
    }
    impl crate::obj::cap::GodotDefault for MenuBar {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MenuBar {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MenuBar {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MenuBar`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_MenuBar__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MenuBar > for $Class {
                
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::MenuBar;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::control::SignalsOfControl;
    impl WithSignals for MenuBar {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfControl < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}