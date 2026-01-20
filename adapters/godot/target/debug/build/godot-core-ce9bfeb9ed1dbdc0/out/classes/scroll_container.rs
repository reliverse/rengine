#![doc = "Sidecar module for class [`ScrollContainer`][crate::classes::ScrollContainer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ScrollContainer` enums](https://docs.godotengine.org/en/stable/classes/class_scrollcontainer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ScrollContainer.`\n\nInherits [`Container`][crate::classes::Container].\n\nRelated symbols:\n\n* [`scroll_container`][crate::classes::scroll_container]: sidecar module with related enum/flag types\n* [`IScrollContainer`][crate::classes::IScrollContainer]: virtual methods\n* [`SignalsOfScrollContainer`][crate::classes::scroll_container::SignalsOfScrollContainer]: signal collection\n\n\nSee also [Godot docs for `ScrollContainer`](https://docs.godotengine.org/en/stable/classes/class_scrollcontainer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`ScrollContainer::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ScrollContainer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ScrollContainer`][crate::classes::ScrollContainer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IContainer`][crate::classes::IContainer] > [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `ScrollContainer` methods](https://docs.godotengine.org/en/stable/classes/class_scrollcontainer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IScrollContainer: crate::obj::GodotClass < Base = ScrollContainer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ScrollContainer {
        pub fn set_h_scroll(&mut self, value: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7929usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "set_h_scroll", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_h_scroll(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7930usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "get_h_scroll", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_v_scroll(&mut self, value: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7931usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "set_v_scroll", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_v_scroll(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7932usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "get_v_scroll", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_horizontal_custom_step(&mut self, value: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7933usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "set_horizontal_custom_step", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_horizontal_custom_step(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7934usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "get_horizontal_custom_step", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertical_custom_step(&mut self, value: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7935usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "set_vertical_custom_step", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertical_custom_step(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7936usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "get_vertical_custom_step", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_horizontal_scroll_mode(&mut self, enable: crate::classes::scroll_container::ScrollMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::scroll_container::ScrollMode,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7937usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "set_horizontal_scroll_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_horizontal_scroll_mode(&self,) -> crate::classes::scroll_container::ScrollMode {
            type CallRet = crate::classes::scroll_container::ScrollMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7938usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "get_horizontal_scroll_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertical_scroll_mode(&mut self, enable: crate::classes::scroll_container::ScrollMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::scroll_container::ScrollMode,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7939usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "set_vertical_scroll_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertical_scroll_mode(&self,) -> crate::classes::scroll_container::ScrollMode {
            type CallRet = crate::classes::scroll_container::ScrollMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7940usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "get_vertical_scroll_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_deadzone(&mut self, deadzone: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (deadzone,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7941usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "set_deadzone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_deadzone(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7942usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "get_deadzone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_follow_focus(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7943usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "set_follow_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_following_focus(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7944usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "is_following_focus", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_h_scroll_bar(&mut self,) -> Option < Gd < crate::classes::HScrollBar > > {
            type CallRet = Option < Gd < crate::classes::HScrollBar > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7945usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "get_h_scroll_bar", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_v_scroll_bar(&mut self,) -> Option < Gd < crate::classes::VScrollBar > > {
            type CallRet = Option < Gd < crate::classes::VScrollBar > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7946usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "get_v_scroll_bar", Some(self.__validated_obj()), args,)
            }
        }
        pub fn ensure_control_visible(&mut self, control: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >,);
            let args = (control.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7947usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "ensure_control_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_focus_border(&mut self, draw: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (draw,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7948usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "set_draw_focus_border", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_draw_focus_border(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7949usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ScrollContainer", "get_draw_focus_border", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ScrollContainer {
        type Base = crate::classes::Container;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ScrollContainer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ScrollContainer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Container > for ScrollContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for ScrollContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for ScrollContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for ScrollContainer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ScrollContainer {
        
    }
    impl crate::obj::cap::GodotDefault for ScrollContainer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ScrollContainer {
        type Target = crate::classes::Container;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ScrollContainer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ScrollContainer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ScrollContainer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ScrollContainer > for $Class {
                
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
pub struct ScrollMode {
    ord: i32
}
impl ScrollMode {
    #[doc(alias = "SCROLL_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `SCROLL_MODE_DISABLED`"]
    pub const DISABLED: ScrollMode = ScrollMode {
        ord: 0i32
    };
    #[doc(alias = "SCROLL_MODE_AUTO")]
    #[doc = "Godot enumerator name: `SCROLL_MODE_AUTO`"]
    pub const AUTO: ScrollMode = ScrollMode {
        ord: 1i32
    };
    #[doc(alias = "SCROLL_MODE_SHOW_ALWAYS")]
    #[doc = "Godot enumerator name: `SCROLL_MODE_SHOW_ALWAYS`"]
    pub const SHOW_ALWAYS: ScrollMode = ScrollMode {
        ord: 2i32
    };
    #[doc(alias = "SCROLL_MODE_SHOW_NEVER")]
    #[doc = "Godot enumerator name: `SCROLL_MODE_SHOW_NEVER`"]
    pub const SHOW_NEVER: ScrollMode = ScrollMode {
        ord: 3i32
    };
    #[doc(alias = "SCROLL_MODE_RESERVE")]
    #[doc = "Godot enumerator name: `SCROLL_MODE_RESERVE`"]
    pub const RESERVE: ScrollMode = ScrollMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ScrollMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ScrollMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ScrollMode {
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
            Self::DISABLED => "DISABLED", Self::AUTO => "AUTO", Self::SHOW_ALWAYS => "SHOW_ALWAYS", Self::SHOW_NEVER => "SHOW_NEVER", Self::RESERVE => "RESERVE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ScrollMode::DISABLED, ScrollMode::AUTO, ScrollMode::SHOW_ALWAYS, ScrollMode::SHOW_NEVER, ScrollMode::RESERVE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ScrollMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "SCROLL_MODE_DISABLED", ScrollMode::DISABLED), crate::meta::inspect::EnumConstant::new("AUTO", "SCROLL_MODE_AUTO", ScrollMode::AUTO), crate::meta::inspect::EnumConstant::new("SHOW_ALWAYS", "SCROLL_MODE_SHOW_ALWAYS", ScrollMode::SHOW_ALWAYS), crate::meta::inspect::EnumConstant::new("SHOW_NEVER", "SCROLL_MODE_SHOW_NEVER", ScrollMode::SHOW_NEVER), crate::meta::inspect::EnumConstant::new("RESERVE", "SCROLL_MODE_RESERVE", ScrollMode::RESERVE)]
        }
    }
}
impl crate::meta::GodotConvert for ScrollMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ScrollMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ScrollMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ScrollContainer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`ScrollContainer`][crate::classes::ScrollContainer] class."]
    pub struct SignalsOfScrollContainer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfScrollContainer < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn scroll_started(&mut self) -> SigScrollStarted < 'c, C > {
            SigScrollStarted {
                typed: TypedSignal::extract(&mut self.__internal_obj, "scroll_started")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn scroll_ended(&mut self) -> SigScrollEnded < 'c, C > {
            SigScrollEnded {
                typed: TypedSignal::extract(&mut self.__internal_obj, "scroll_ended")
            }
        }
    }
    type TypedSigScrollStarted < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigScrollStarted < 'c, C: WithSignals > {
        typed: TypedSigScrollStarted < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigScrollStarted < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigScrollStarted < 'c, C > {
        type Target = TypedSigScrollStarted < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigScrollStarted < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigScrollEnded < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigScrollEnded < 'c, C: WithSignals > {
        typed: TypedSigScrollEnded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigScrollEnded < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigScrollEnded < 'c, C > {
        type Target = TypedSigScrollEnded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigScrollEnded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for ScrollContainer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfScrollContainer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfScrollContainer < 'c, C > {
        type Target = < < ScrollContainer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = ScrollContainer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfScrollContainer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = ScrollContainer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}