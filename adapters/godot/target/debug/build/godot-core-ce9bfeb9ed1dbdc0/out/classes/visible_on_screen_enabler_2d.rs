#![doc = "Sidecar module for class [`VisibleOnScreenEnabler2D`][crate::classes::VisibleOnScreenEnabler2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisibleOnScreenEnabler2D` enums](https://docs.godotengine.org/en/stable/classes/class_visibleonscreenenabler2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisibleOnScreenEnabler2D.`\n\nInherits [`VisibleOnScreenNotifier2D`][crate::classes::VisibleOnScreenNotifier2D].\n\nRelated symbols:\n\n* [`visible_on_screen_enabler_2d`][crate::classes::visible_on_screen_enabler_2d]: sidecar module with related enum/flag types\n* [`IVisibleOnScreenEnabler2D`][crate::classes::IVisibleOnScreenEnabler2D]: virtual methods\n\n\nSee also [Godot docs for `VisibleOnScreenEnabler2D`](https://docs.godotengine.org/en/stable/classes/class_visibleonscreenenabler2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`VisibleOnScreenEnabler2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisibleOnScreenEnabler2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`VisibleOnScreenEnabler2D`][crate::classes::VisibleOnScreenEnabler2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IVisibleOnScreenNotifier2D`][crate::classes::IVisibleOnScreenNotifier2D] > [`INode2D`][crate::classes::INode2D] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `VisibleOnScreenEnabler2D` methods](https://docs.godotengine.org/en/stable/classes/class_visibleonscreenenabler2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisibleOnScreenEnabler2D: crate::obj::GodotClass < Base = VisibleOnScreenEnabler2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
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
    impl VisibleOnScreenEnabler2D {
        pub fn set_enable_mode(&mut self, mode: crate::classes::visible_on_screen_enabler_2d::EnableMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::visible_on_screen_enabler_2d::EnableMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10566usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisibleOnScreenEnabler2D", "set_enable_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_enable_mode(&mut self,) -> crate::classes::visible_on_screen_enabler_2d::EnableMode {
            type CallRet = crate::classes::visible_on_screen_enabler_2d::EnableMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10567usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisibleOnScreenEnabler2D", "get_enable_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enable_node_path(&mut self, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10568usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisibleOnScreenEnabler2D", "set_enable_node_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_enable_node_path(&mut self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10569usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisibleOnScreenEnabler2D", "get_enable_node_path", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for VisibleOnScreenEnabler2D {
        type Base = crate::classes::VisibleOnScreenNotifier2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VisibleOnScreenEnabler2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisibleOnScreenEnabler2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisibleOnScreenNotifier2D > for VisibleOnScreenEnabler2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for VisibleOnScreenEnabler2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for VisibleOnScreenEnabler2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for VisibleOnScreenEnabler2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisibleOnScreenEnabler2D {
        
    }
    impl crate::obj::cap::GodotDefault for VisibleOnScreenEnabler2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisibleOnScreenEnabler2D {
        type Target = crate::classes::VisibleOnScreenNotifier2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisibleOnScreenEnabler2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VisibleOnScreenEnabler2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisibleOnScreenEnabler2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisibleOnScreenEnabler2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisibleOnScreenNotifier2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node2D > for $Class {
                
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
pub struct EnableMode {
    ord: i32
}
impl EnableMode {
    #[doc(alias = "ENABLE_MODE_INHERIT")]
    #[doc = "Godot enumerator name: `ENABLE_MODE_INHERIT`"]
    pub const INHERIT: EnableMode = EnableMode {
        ord: 0i32
    };
    #[doc(alias = "ENABLE_MODE_ALWAYS")]
    #[doc = "Godot enumerator name: `ENABLE_MODE_ALWAYS`"]
    pub const ALWAYS: EnableMode = EnableMode {
        ord: 1i32
    };
    #[doc(alias = "ENABLE_MODE_WHEN_PAUSED")]
    #[doc = "Godot enumerator name: `ENABLE_MODE_WHEN_PAUSED`"]
    pub const WHEN_PAUSED: EnableMode = EnableMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for EnableMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnableMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnableMode {
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
            Self::INHERIT => "INHERIT", Self::ALWAYS => "ALWAYS", Self::WHEN_PAUSED => "WHEN_PAUSED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnableMode::INHERIT, EnableMode::ALWAYS, EnableMode::WHEN_PAUSED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnableMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INHERIT", "ENABLE_MODE_INHERIT", EnableMode::INHERIT), crate::meta::inspect::EnumConstant::new("ALWAYS", "ENABLE_MODE_ALWAYS", EnableMode::ALWAYS), crate::meta::inspect::EnumConstant::new("WHEN_PAUSED", "ENABLE_MODE_WHEN_PAUSED", EnableMode::WHEN_PAUSED)]
        }
    }
}
impl crate::meta::GodotConvert for EnableMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnableMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnableMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VisibleOnScreenEnabler2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::visible_on_screen_notifier_2d::SignalsOfVisibleOnScreenNotifier2D;
    impl WithSignals for VisibleOnScreenEnabler2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfVisibleOnScreenNotifier2D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}