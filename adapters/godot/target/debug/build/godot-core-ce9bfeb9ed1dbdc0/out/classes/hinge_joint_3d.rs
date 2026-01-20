#![doc = "Sidecar module for class [`HingeJoint3D`][crate::classes::HingeJoint3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `HingeJoint3D` enums](https://docs.godotengine.org/en/stable/classes/class_hingejoint3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `HingeJoint3D.`\n\nInherits [`Joint3D`][crate::classes::Joint3D].\n\nRelated symbols:\n\n* [`hinge_joint_3d`][crate::classes::hinge_joint_3d]: sidecar module with related enum/flag types\n* [`IHingeJoint3D`][crate::classes::IHingeJoint3D]: virtual methods\n\n\nSee also [Godot docs for `HingeJoint3D`](https://docs.godotengine.org/en/stable/classes/class_hingejoint3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`HingeJoint3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct HingeJoint3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`HingeJoint3D`][crate::classes::HingeJoint3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IJoint3D`~~ > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `HingeJoint3D` methods](https://docs.godotengine.org/en/stable/classes/class_hingejoint3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IHingeJoint3D: crate::obj::GodotClass < Base = HingeJoint3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
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
    impl HingeJoint3D {
        pub fn set_param(&mut self, param: crate::classes::hinge_joint_3d::Param, value: f32,) {
            type CallRet = ();
            type CallParams = (crate::classes::hinge_joint_3d::Param, f32,);
            let args = (param, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HingeJoint3D", "set_param", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_param(&self, param: crate::classes::hinge_joint_3d::Param,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::classes::hinge_joint_3d::Param,);
            let args = (param,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HingeJoint3D", "get_param", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flag(&mut self, flag: crate::classes::hinge_joint_3d::Flag, enabled: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::hinge_joint_3d::Flag, bool,);
            let args = (flag, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HingeJoint3D", "set_flag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_flag(&self, flag: crate::classes::hinge_joint_3d::Flag,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::hinge_joint_3d::Flag,);
            let args = (flag,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "HingeJoint3D", "get_flag", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for HingeJoint3D {
        type Base = crate::classes::Joint3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("HingeJoint3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for HingeJoint3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Joint3D > for HingeJoint3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for HingeJoint3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for HingeJoint3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for HingeJoint3D {
        
    }
    impl crate::obj::cap::GodotDefault for HingeJoint3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for HingeJoint3D {
        type Target = crate::classes::Joint3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for HingeJoint3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`HingeJoint3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_HingeJoint3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::HingeJoint3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Joint3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node3D > for $Class {
                
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
pub struct Param {
    ord: i32
}
impl Param {
    #[doc(alias = "PARAM_BIAS")]
    #[doc = "Godot enumerator name: `PARAM_BIAS`"]
    pub const BIAS: Param = Param {
        ord: 0i32
    };
    #[doc(alias = "PARAM_LIMIT_UPPER")]
    #[doc = "Godot enumerator name: `PARAM_LIMIT_UPPER`"]
    pub const LIMIT_UPPER: Param = Param {
        ord: 1i32
    };
    #[doc(alias = "PARAM_LIMIT_LOWER")]
    #[doc = "Godot enumerator name: `PARAM_LIMIT_LOWER`"]
    pub const LIMIT_LOWER: Param = Param {
        ord: 2i32
    };
    #[doc(alias = "PARAM_LIMIT_BIAS")]
    #[doc = "Godot enumerator name: `PARAM_LIMIT_BIAS`"]
    pub const LIMIT_BIAS: Param = Param {
        ord: 3i32
    };
    #[doc(alias = "PARAM_LIMIT_SOFTNESS")]
    #[doc = "Godot enumerator name: `PARAM_LIMIT_SOFTNESS`"]
    pub const LIMIT_SOFTNESS: Param = Param {
        ord: 4i32
    };
    #[doc(alias = "PARAM_LIMIT_RELAXATION")]
    #[doc = "Godot enumerator name: `PARAM_LIMIT_RELAXATION`"]
    pub const LIMIT_RELAXATION: Param = Param {
        ord: 5i32
    };
    #[doc(alias = "PARAM_MOTOR_TARGET_VELOCITY")]
    #[doc = "Godot enumerator name: `PARAM_MOTOR_TARGET_VELOCITY`"]
    pub const MOTOR_TARGET_VELOCITY: Param = Param {
        ord: 6i32
    };
    #[doc(alias = "PARAM_MOTOR_MAX_IMPULSE")]
    #[doc = "Godot enumerator name: `PARAM_MOTOR_MAX_IMPULSE`"]
    pub const MOTOR_MAX_IMPULSE: Param = Param {
        ord: 7i32
    };
    #[doc(alias = "PARAM_MAX")]
    #[doc = "Godot enumerator name: `PARAM_MAX`"]
    pub const MAX: Param = Param {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Param") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Param {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::BIAS => "BIAS", Self::LIMIT_UPPER => "LIMIT_UPPER", Self::LIMIT_LOWER => "LIMIT_LOWER", Self::LIMIT_BIAS => "LIMIT_BIAS", Self::LIMIT_SOFTNESS => "LIMIT_SOFTNESS", Self::LIMIT_RELAXATION => "LIMIT_RELAXATION", Self::MOTOR_TARGET_VELOCITY => "MOTOR_TARGET_VELOCITY", Self::MOTOR_MAX_IMPULSE => "MOTOR_MAX_IMPULSE", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Param::BIAS, Param::LIMIT_UPPER, Param::LIMIT_LOWER, Param::LIMIT_BIAS, Param::LIMIT_SOFTNESS, Param::LIMIT_RELAXATION, Param::MOTOR_TARGET_VELOCITY, Param::MOTOR_MAX_IMPULSE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Param >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BIAS", "PARAM_BIAS", Param::BIAS), crate::meta::inspect::EnumConstant::new("LIMIT_UPPER", "PARAM_LIMIT_UPPER", Param::LIMIT_UPPER), crate::meta::inspect::EnumConstant::new("LIMIT_LOWER", "PARAM_LIMIT_LOWER", Param::LIMIT_LOWER), crate::meta::inspect::EnumConstant::new("LIMIT_BIAS", "PARAM_LIMIT_BIAS", Param::LIMIT_BIAS), crate::meta::inspect::EnumConstant::new("LIMIT_SOFTNESS", "PARAM_LIMIT_SOFTNESS", Param::LIMIT_SOFTNESS), crate::meta::inspect::EnumConstant::new("LIMIT_RELAXATION", "PARAM_LIMIT_RELAXATION", Param::LIMIT_RELAXATION), crate::meta::inspect::EnumConstant::new("MOTOR_TARGET_VELOCITY", "PARAM_MOTOR_TARGET_VELOCITY", Param::MOTOR_TARGET_VELOCITY), crate::meta::inspect::EnumConstant::new("MOTOR_MAX_IMPULSE", "PARAM_MOTOR_MAX_IMPULSE", Param::MOTOR_MAX_IMPULSE), crate::meta::inspect::EnumConstant::new("MAX", "PARAM_MAX", Param::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Param {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for Param {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Param {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Param {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Flag {
    ord: i32
}
impl Flag {
    #[doc(alias = "FLAG_USE_LIMIT")]
    #[doc = "Godot enumerator name: `FLAG_USE_LIMIT`"]
    pub const USE_LIMIT: Flag = Flag {
        ord: 0i32
    };
    #[doc(alias = "FLAG_ENABLE_MOTOR")]
    #[doc = "Godot enumerator name: `FLAG_ENABLE_MOTOR`"]
    pub const ENABLE_MOTOR: Flag = Flag {
        ord: 1i32
    };
    #[doc(alias = "FLAG_MAX")]
    #[doc = "Godot enumerator name: `FLAG_MAX`"]
    pub const MAX: Flag = Flag {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Flag") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Flag {
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
            Self::USE_LIMIT => "USE_LIMIT", Self::ENABLE_MOTOR => "ENABLE_MOTOR", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Flag::USE_LIMIT, Flag::ENABLE_MOTOR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Flag >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("USE_LIMIT", "FLAG_USE_LIMIT", Flag::USE_LIMIT), crate::meta::inspect::EnumConstant::new("ENABLE_MOTOR", "FLAG_ENABLE_MOTOR", Flag::ENABLE_MOTOR), crate::meta::inspect::EnumConstant::new("MAX", "FLAG_MAX", Flag::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Flag {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for Flag {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Flag {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Flag {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::HingeJoint3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for HingeJoint3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}