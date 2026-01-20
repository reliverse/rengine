#![doc = "Sidecar module for class [`SubViewport`][crate::classes::SubViewport].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SubViewport` enums](https://docs.godotengine.org/en/stable/classes/class_subviewport.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SubViewport.`\n\nInherits [`Viewport`][crate::classes::Viewport].\n\nRelated symbols:\n\n* [`sub_viewport`][crate::classes::sub_viewport]: sidecar module with related enum/flag types\n* [`ISubViewport`][crate::classes::ISubViewport]: virtual methods\n\n\nSee also [Godot docs for `SubViewport`](https://docs.godotengine.org/en/stable/classes/class_subviewport.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`SubViewport::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SubViewport {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`SubViewport`][crate::classes::SubViewport].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IViewport`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `SubViewport` methods](https://docs.godotengine.org/en/stable/classes/class_subviewport.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISubViewport: crate::obj::GodotClass < Base = SubViewport > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SubViewport {
        pub fn set_size(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8665usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SubViewport", "set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8666usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SubViewport", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_size_2d_override(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8667usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SubViewport", "set_size_2d_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size_2d_override(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SubViewport", "get_size_2d_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_size_2d_override_stretch(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8669usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SubViewport", "set_size_2d_override_stretch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_size_2d_override_stretch_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SubViewport", "is_size_2d_override_stretch_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_update_mode(&mut self, mode: crate::classes::sub_viewport::UpdateMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::sub_viewport::UpdateMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SubViewport", "set_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_update_mode(&self,) -> crate::classes::sub_viewport::UpdateMode {
            type CallRet = crate::classes::sub_viewport::UpdateMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SubViewport", "get_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_clear_mode(&mut self, mode: crate::classes::sub_viewport::ClearMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::sub_viewport::ClearMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8673usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SubViewport", "set_clear_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_clear_mode(&self,) -> crate::classes::sub_viewport::ClearMode {
            type CallRet = crate::classes::sub_viewport::ClearMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8674usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SubViewport", "get_clear_mode", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for SubViewport {
        type Base = crate::classes::Viewport;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("SubViewport"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SubViewport {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Viewport > for SubViewport {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for SubViewport {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SubViewport {
        
    }
    impl crate::obj::cap::GodotDefault for SubViewport {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SubViewport {
        type Target = crate::classes::Viewport;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SubViewport {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SubViewport`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_SubViewport__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SubViewport > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Viewport > for $Class {
                
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
pub struct ClearMode {
    ord: i32
}
impl ClearMode {
    #[doc(alias = "CLEAR_MODE_ALWAYS")]
    #[doc = "Godot enumerator name: `CLEAR_MODE_ALWAYS`"]
    pub const ALWAYS: ClearMode = ClearMode {
        ord: 0i32
    };
    #[doc(alias = "CLEAR_MODE_NEVER")]
    #[doc = "Godot enumerator name: `CLEAR_MODE_NEVER`"]
    pub const NEVER: ClearMode = ClearMode {
        ord: 1i32
    };
    #[doc(alias = "CLEAR_MODE_ONCE")]
    #[doc = "Godot enumerator name: `CLEAR_MODE_ONCE`"]
    pub const ONCE: ClearMode = ClearMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ClearMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ClearMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ClearMode {
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
            Self::ALWAYS => "ALWAYS", Self::NEVER => "NEVER", Self::ONCE => "ONCE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ClearMode::ALWAYS, ClearMode::NEVER, ClearMode::ONCE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ClearMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ALWAYS", "CLEAR_MODE_ALWAYS", ClearMode::ALWAYS), crate::meta::inspect::EnumConstant::new("NEVER", "CLEAR_MODE_NEVER", ClearMode::NEVER), crate::meta::inspect::EnumConstant::new("ONCE", "CLEAR_MODE_ONCE", ClearMode::ONCE)]
        }
    }
}
impl crate::meta::GodotConvert for ClearMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ClearMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ClearMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct UpdateMode {
    ord: i32
}
impl UpdateMode {
    #[doc(alias = "UPDATE_DISABLED")]
    #[doc = "Godot enumerator name: `UPDATE_DISABLED`"]
    pub const DISABLED: UpdateMode = UpdateMode {
        ord: 0i32
    };
    #[doc(alias = "UPDATE_ONCE")]
    #[doc = "Godot enumerator name: `UPDATE_ONCE`"]
    pub const ONCE: UpdateMode = UpdateMode {
        ord: 1i32
    };
    #[doc(alias = "UPDATE_WHEN_VISIBLE")]
    #[doc = "Godot enumerator name: `UPDATE_WHEN_VISIBLE`"]
    pub const WHEN_VISIBLE: UpdateMode = UpdateMode {
        ord: 2i32
    };
    #[doc(alias = "UPDATE_WHEN_PARENT_VISIBLE")]
    #[doc = "Godot enumerator name: `UPDATE_WHEN_PARENT_VISIBLE`"]
    pub const WHEN_PARENT_VISIBLE: UpdateMode = UpdateMode {
        ord: 3i32
    };
    #[doc(alias = "UPDATE_ALWAYS")]
    #[doc = "Godot enumerator name: `UPDATE_ALWAYS`"]
    pub const ALWAYS: UpdateMode = UpdateMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for UpdateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("UpdateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for UpdateMode {
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
            Self::DISABLED => "DISABLED", Self::ONCE => "ONCE", Self::WHEN_VISIBLE => "WHEN_VISIBLE", Self::WHEN_PARENT_VISIBLE => "WHEN_PARENT_VISIBLE", Self::ALWAYS => "ALWAYS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[UpdateMode::DISABLED, UpdateMode::ONCE, UpdateMode::WHEN_VISIBLE, UpdateMode::WHEN_PARENT_VISIBLE, UpdateMode::ALWAYS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < UpdateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "UPDATE_DISABLED", UpdateMode::DISABLED), crate::meta::inspect::EnumConstant::new("ONCE", "UPDATE_ONCE", UpdateMode::ONCE), crate::meta::inspect::EnumConstant::new("WHEN_VISIBLE", "UPDATE_WHEN_VISIBLE", UpdateMode::WHEN_VISIBLE), crate::meta::inspect::EnumConstant::new("WHEN_PARENT_VISIBLE", "UPDATE_WHEN_PARENT_VISIBLE", UpdateMode::WHEN_PARENT_VISIBLE), crate::meta::inspect::EnumConstant::new("ALWAYS", "UPDATE_ALWAYS", UpdateMode::ALWAYS)]
        }
    }
}
impl crate::meta::GodotConvert for UpdateMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for UpdateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for UpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::SubViewport;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::viewport::SignalsOfViewport;
    impl WithSignals for SubViewport {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfViewport < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}