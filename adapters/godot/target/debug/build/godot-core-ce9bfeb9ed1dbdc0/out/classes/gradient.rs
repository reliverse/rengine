#![doc = "Sidecar module for class [`Gradient`][crate::classes::Gradient].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Gradient` enums](https://docs.godotengine.org/en/stable/classes/class_gradient.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Gradient.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`gradient`][crate::classes::gradient]: sidecar module with related enum/flag types\n* [`IGradient`][crate::classes::IGradient]: virtual methods\n\n\nSee also [Godot docs for `Gradient`](https://docs.godotengine.org/en/stable/classes/class_gradient.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Gradient::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Gradient {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Gradient`][crate::classes::Gradient].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Gradient` methods](https://docs.godotengine.org/en/stable/classes/class_gradient.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGradient: crate::obj::GodotClass < Base = Gradient > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Gradient {
        pub fn add_point(&mut self, offset: f32, color: Color,) {
            type CallRet = ();
            type CallParams = (f32, Color,);
            let args = (offset, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4082usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "add_point", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_point(&mut self, point: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4083usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "remove_point", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_offset(&mut self, point: i32, offset: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (point, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4084usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "set_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_offset(&mut self, point: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4085usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "get_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reverse(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4086usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "reverse", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color(&mut self, point: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (point, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4087usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "set_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color(&mut self, point: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4088usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "get_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn sample(&mut self, offset: f32,) -> Color {
            type CallRet = Color;
            type CallParams = (f32,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4089usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "sample", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_point_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4090usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "get_point_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_offsets(&mut self, offsets: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedFloat32Array >,);
            let args = (RefArg::new(offsets),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4091usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "set_offsets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_offsets(&self,) -> PackedFloat32Array {
            type CallRet = PackedFloat32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4092usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "get_offsets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_colors(&mut self, colors: &PackedColorArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedColorArray >,);
            let args = (RefArg::new(colors),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4093usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "set_colors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_colors(&self,) -> PackedColorArray {
            type CallRet = PackedColorArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4094usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "get_colors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_interpolation_mode(&mut self, interpolation_mode: crate::classes::gradient::InterpolationMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::gradient::InterpolationMode,);
            let args = (interpolation_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4095usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "set_interpolation_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_interpolation_mode(&mut self,) -> crate::classes::gradient::InterpolationMode {
            type CallRet = crate::classes::gradient::InterpolationMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4096usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "get_interpolation_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_interpolation_color_space(&mut self, interpolation_color_space: crate::classes::gradient::ColorSpace,) {
            type CallRet = ();
            type CallParams = (crate::classes::gradient::ColorSpace,);
            let args = (interpolation_color_space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4097usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "set_interpolation_color_space", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_interpolation_color_space(&mut self,) -> crate::classes::gradient::ColorSpace {
            type CallRet = crate::classes::gradient::ColorSpace;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4098usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Gradient", "get_interpolation_color_space", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Gradient {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Gradient"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Gradient {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Gradient {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Gradient {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Gradient {
        
    }
    impl crate::obj::cap::GodotDefault for Gradient {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Gradient {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Gradient {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Gradient`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Gradient__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Gradient > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct InterpolationMode {
    ord: i32
}
impl InterpolationMode {
    #[doc(alias = "GRADIENT_INTERPOLATE_LINEAR")]
    #[doc = "Godot enumerator name: `GRADIENT_INTERPOLATE_LINEAR`"]
    pub const LINEAR: InterpolationMode = InterpolationMode {
        ord: 0i32
    };
    #[doc(alias = "GRADIENT_INTERPOLATE_CONSTANT")]
    #[doc = "Godot enumerator name: `GRADIENT_INTERPOLATE_CONSTANT`"]
    pub const CONSTANT: InterpolationMode = InterpolationMode {
        ord: 1i32
    };
    #[doc(alias = "GRADIENT_INTERPOLATE_CUBIC")]
    #[doc = "Godot enumerator name: `GRADIENT_INTERPOLATE_CUBIC`"]
    pub const CUBIC: InterpolationMode = InterpolationMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for InterpolationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("InterpolationMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for InterpolationMode {
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
            Self::LINEAR => "LINEAR", Self::CONSTANT => "CONSTANT", Self::CUBIC => "CUBIC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[InterpolationMode::LINEAR, InterpolationMode::CONSTANT, InterpolationMode::CUBIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < InterpolationMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LINEAR", "GRADIENT_INTERPOLATE_LINEAR", InterpolationMode::LINEAR), crate::meta::inspect::EnumConstant::new("CONSTANT", "GRADIENT_INTERPOLATE_CONSTANT", InterpolationMode::CONSTANT), crate::meta::inspect::EnumConstant::new("CUBIC", "GRADIENT_INTERPOLATE_CUBIC", InterpolationMode::CUBIC)]
        }
    }
}
impl crate::meta::GodotConvert for InterpolationMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for InterpolationMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for InterpolationMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ColorSpace {
    ord: i32
}
impl ColorSpace {
    #[doc(alias = "GRADIENT_COLOR_SPACE_SRGB")]
    #[doc = "Godot enumerator name: `GRADIENT_COLOR_SPACE_SRGB`"]
    pub const SRGB: ColorSpace = ColorSpace {
        ord: 0i32
    };
    #[doc(alias = "GRADIENT_COLOR_SPACE_LINEAR_SRGB")]
    #[doc = "Godot enumerator name: `GRADIENT_COLOR_SPACE_LINEAR_SRGB`"]
    pub const LINEAR_SRGB: ColorSpace = ColorSpace {
        ord: 1i32
    };
    #[doc(alias = "GRADIENT_COLOR_SPACE_OKLAB")]
    #[doc = "Godot enumerator name: `GRADIENT_COLOR_SPACE_OKLAB`"]
    pub const OKLAB: ColorSpace = ColorSpace {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ColorSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ColorSpace") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ColorSpace {
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
            Self::SRGB => "SRGB", Self::LINEAR_SRGB => "LINEAR_SRGB", Self::OKLAB => "OKLAB", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ColorSpace::SRGB, ColorSpace::LINEAR_SRGB, ColorSpace::OKLAB]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ColorSpace >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SRGB", "GRADIENT_COLOR_SPACE_SRGB", ColorSpace::SRGB), crate::meta::inspect::EnumConstant::new("LINEAR_SRGB", "GRADIENT_COLOR_SPACE_LINEAR_SRGB", ColorSpace::LINEAR_SRGB), crate::meta::inspect::EnumConstant::new("OKLAB", "GRADIENT_COLOR_SPACE_OKLAB", ColorSpace::OKLAB)]
        }
    }
}
impl crate::meta::GodotConvert for ColorSpace {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ColorSpace {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ColorSpace {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Gradient;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Gradient {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfResource < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}