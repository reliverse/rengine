#![doc = "Sidecar module for class [`ProgressBar`][crate::classes::ProgressBar].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ProgressBar` enums](https://docs.godotengine.org/en/stable/classes/class_progressbar.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ProgressBar.`\n\nInherits [`Range`][crate::classes::Range].\n\nRelated symbols:\n\n* [`progress_bar`][crate::classes::progress_bar]: sidecar module with related enum/flag types\n* [`IProgressBar`][crate::classes::IProgressBar]: virtual methods\n\n\nSee also [Godot docs for `ProgressBar`](https://docs.godotengine.org/en/stable/classes/class_progressbar.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`ProgressBar::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ProgressBar {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ProgressBar`][crate::classes::ProgressBar].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRange`][crate::classes::IRange] > [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `ProgressBar` methods](https://docs.godotengine.org/en/stable/classes/class_progressbar.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IProgressBar: crate::obj::GodotClass < Base = ProgressBar > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn value_changed(&mut self, new_value: f64,) {
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
    impl ProgressBar {
        pub fn set_fill_mode(&mut self, mode: crate::classes::progress_bar::FillMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::progress_bar::FillMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6886usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProgressBar", "set_fill_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fill_mode(&mut self,) -> crate::classes::progress_bar::FillMode {
            type CallRet = crate::classes::progress_bar::FillMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6887usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProgressBar", "get_fill_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_show_percentage(&mut self, visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6888usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProgressBar", "set_show_percentage", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_percentage_shown(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6889usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProgressBar", "is_percentage_shown", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_indeterminate(&mut self, indeterminate: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (indeterminate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6890usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProgressBar", "set_indeterminate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_indeterminate(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6891usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProgressBar", "is_indeterminate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_editor_preview_indeterminate(&mut self, preview_indeterminate: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (preview_indeterminate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6892usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProgressBar", "set_editor_preview_indeterminate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_editor_preview_indeterminate_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6893usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ProgressBar", "is_editor_preview_indeterminate_enabled", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ProgressBar {
        type Base = crate::classes::Range;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ProgressBar"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ProgressBar {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Range > for ProgressBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for ProgressBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for ProgressBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for ProgressBar {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ProgressBar {
        
    }
    impl crate::obj::cap::GodotDefault for ProgressBar {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ProgressBar {
        type Target = crate::classes::Range;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ProgressBar {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ProgressBar`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ProgressBar__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ProgressBar > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Range > for $Class {
                
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
pub struct FillMode {
    ord: i32
}
impl FillMode {
    #[doc(alias = "FILL_BEGIN_TO_END")]
    #[doc = "Godot enumerator name: `FILL_BEGIN_TO_END`"]
    pub const BEGIN_TO_END: FillMode = FillMode {
        ord: 0i32
    };
    #[doc(alias = "FILL_END_TO_BEGIN")]
    #[doc = "Godot enumerator name: `FILL_END_TO_BEGIN`"]
    pub const END_TO_BEGIN: FillMode = FillMode {
        ord: 1i32
    };
    #[doc(alias = "FILL_TOP_TO_BOTTOM")]
    #[doc = "Godot enumerator name: `FILL_TOP_TO_BOTTOM`"]
    pub const TOP_TO_BOTTOM: FillMode = FillMode {
        ord: 2i32
    };
    #[doc(alias = "FILL_BOTTOM_TO_TOP")]
    #[doc = "Godot enumerator name: `FILL_BOTTOM_TO_TOP`"]
    pub const BOTTOM_TO_TOP: FillMode = FillMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for FillMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FillMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FillMode {
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
            Self::BEGIN_TO_END => "BEGIN_TO_END", Self::END_TO_BEGIN => "END_TO_BEGIN", Self::TOP_TO_BOTTOM => "TOP_TO_BOTTOM", Self::BOTTOM_TO_TOP => "BOTTOM_TO_TOP", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FillMode::BEGIN_TO_END, FillMode::END_TO_BEGIN, FillMode::TOP_TO_BOTTOM, FillMode::BOTTOM_TO_TOP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FillMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BEGIN_TO_END", "FILL_BEGIN_TO_END", FillMode::BEGIN_TO_END), crate::meta::inspect::EnumConstant::new("END_TO_BEGIN", "FILL_END_TO_BEGIN", FillMode::END_TO_BEGIN), crate::meta::inspect::EnumConstant::new("TOP_TO_BOTTOM", "FILL_TOP_TO_BOTTOM", FillMode::TOP_TO_BOTTOM), crate::meta::inspect::EnumConstant::new("BOTTOM_TO_TOP", "FILL_BOTTOM_TO_TOP", FillMode::BOTTOM_TO_TOP)]
        }
    }
}
impl crate::meta::GodotConvert for FillMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FillMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FillMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ProgressBar;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::range::SignalsOfRange;
    impl WithSignals for ProgressBar {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfRange < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}