#![doc = "Sidecar module for class [`CameraFeed`][crate::classes::CameraFeed].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CameraFeed` enums](https://docs.godotengine.org/en/stable/classes/class_camerafeed.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CameraFeed.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`camera_feed`][crate::classes::camera_feed]: sidecar module with related enum/flag types\n* [`ICameraFeed`][crate::classes::ICameraFeed]: virtual methods\n* [`SignalsOfCameraFeed`][crate::classes::camera_feed::SignalsOfCameraFeed]: signal collection\n\n\nSee also [Godot docs for `CameraFeed`](https://docs.godotengine.org/en/stable/classes/class_camerafeed.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`CameraFeed::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CameraFeed {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`CameraFeed`][crate::classes::CameraFeed].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `CameraFeed` methods](https://docs.godotengine.org/en/stable/classes/class_camerafeed.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICameraFeed: crate::obj::GodotClass < Base = CameraFeed > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn activate_feed(&mut self,) -> bool {
            unimplemented !()
        }
        fn deactivate_feed(&mut self,) {
            unimplemented !()
        }
    }
    impl CameraFeed {
        pub fn get_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1789usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraFeed", "get_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_active(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1790usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraFeed", "is_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_active(&mut self, active: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1791usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraFeed", "set_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1792usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraFeed", "get_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_name(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1793usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraFeed", "set_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_position(&self,) -> crate::classes::camera_feed::FeedPosition {
            type CallRet = crate::classes::camera_feed::FeedPosition;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1794usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraFeed", "get_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_position(&mut self, position: crate::classes::camera_feed::FeedPosition,) {
            type CallRet = ();
            type CallParams = (crate::classes::camera_feed::FeedPosition,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1795usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraFeed", "set_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transform(&self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1796usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraFeed", "get_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transform(&mut self, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Transform2D,);
            let args = (transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1797usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraFeed", "set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rgb_image(&mut self, rgb_image: impl AsArg < Option < Gd < crate::classes::Image >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >,);
            let args = (rgb_image.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1798usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraFeed", "set_rgb_image", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ycbcr_image(&mut self, ycbcr_image: impl AsArg < Option < Gd < crate::classes::Image >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >,);
            let args = (ycbcr_image.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1799usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraFeed", "set_ycbcr_image", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_external(&mut self, width: i32, height: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (width, height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1800usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraFeed", "set_external", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_tex_id(&mut self, feed_image_type: crate::classes::camera_server::FeedImage,) -> u64 {
            type CallRet = u64;
            type CallParams = (crate::classes::camera_server::FeedImage,);
            let args = (feed_image_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1801usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraFeed", "get_texture_tex_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_datatype(&self,) -> crate::classes::camera_feed::FeedDataType {
            type CallRet = crate::classes::camera_feed::FeedDataType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1802usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraFeed", "get_datatype", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_formats(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1803usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraFeed", "get_formats", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_format(&mut self, index: i32, parameters: &VarDictionary,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (i32, RefArg < 'a0, VarDictionary >,);
            let args = (index, RefArg::new(parameters),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1804usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CameraFeed", "set_format", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CameraFeed {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CameraFeed"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CameraFeed {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for CameraFeed {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CameraFeed {
        
    }
    impl crate::obj::cap::GodotDefault for CameraFeed {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CameraFeed {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CameraFeed {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CameraFeed`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CameraFeed__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CameraFeed > for $Class {
                
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
pub struct FeedDataType {
    ord: i32
}
impl FeedDataType {
    #[doc(alias = "FEED_NOIMAGE")]
    #[doc = "Godot enumerator name: `FEED_NOIMAGE`"]
    pub const NOIMAGE: FeedDataType = FeedDataType {
        ord: 0i32
    };
    #[doc(alias = "FEED_RGB")]
    #[doc = "Godot enumerator name: `FEED_RGB`"]
    pub const RGB: FeedDataType = FeedDataType {
        ord: 1i32
    };
    #[doc(alias = "FEED_YCBCR")]
    #[doc = "Godot enumerator name: `FEED_YCBCR`"]
    pub const YCBCR: FeedDataType = FeedDataType {
        ord: 2i32
    };
    #[doc(alias = "FEED_YCBCR_SEP")]
    #[doc = "Godot enumerator name: `FEED_YCBCR_SEP`"]
    pub const YCBCR_SEP: FeedDataType = FeedDataType {
        ord: 3i32
    };
    #[doc(alias = "FEED_EXTERNAL")]
    #[doc = "Godot enumerator name: `FEED_EXTERNAL`"]
    pub const EXTERNAL: FeedDataType = FeedDataType {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for FeedDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FeedDataType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FeedDataType {
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
            Self::NOIMAGE => "NOIMAGE", Self::RGB => "RGB", Self::YCBCR => "YCBCR", Self::YCBCR_SEP => "YCBCR_SEP", Self::EXTERNAL => "EXTERNAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FeedDataType::NOIMAGE, FeedDataType::RGB, FeedDataType::YCBCR, FeedDataType::YCBCR_SEP, FeedDataType::EXTERNAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FeedDataType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NOIMAGE", "FEED_NOIMAGE", FeedDataType::NOIMAGE), crate::meta::inspect::EnumConstant::new("RGB", "FEED_RGB", FeedDataType::RGB), crate::meta::inspect::EnumConstant::new("YCBCR", "FEED_YCBCR", FeedDataType::YCBCR), crate::meta::inspect::EnumConstant::new("YCBCR_SEP", "FEED_YCBCR_SEP", FeedDataType::YCBCR_SEP), crate::meta::inspect::EnumConstant::new("EXTERNAL", "FEED_EXTERNAL", FeedDataType::EXTERNAL)]
        }
    }
}
impl crate::meta::GodotConvert for FeedDataType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FeedDataType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FeedDataType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FeedPosition {
    ord: i32
}
impl FeedPosition {
    #[doc(alias = "FEED_UNSPECIFIED")]
    #[doc = "Godot enumerator name: `FEED_UNSPECIFIED`"]
    pub const UNSPECIFIED: FeedPosition = FeedPosition {
        ord: 0i32
    };
    #[doc(alias = "FEED_FRONT")]
    #[doc = "Godot enumerator name: `FEED_FRONT`"]
    pub const FRONT: FeedPosition = FeedPosition {
        ord: 1i32
    };
    #[doc(alias = "FEED_BACK")]
    #[doc = "Godot enumerator name: `FEED_BACK`"]
    pub const BACK: FeedPosition = FeedPosition {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for FeedPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FeedPosition") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FeedPosition {
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
            Self::UNSPECIFIED => "UNSPECIFIED", Self::FRONT => "FRONT", Self::BACK => "BACK", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FeedPosition::UNSPECIFIED, FeedPosition::FRONT, FeedPosition::BACK]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FeedPosition >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNSPECIFIED", "FEED_UNSPECIFIED", FeedPosition::UNSPECIFIED), crate::meta::inspect::EnumConstant::new("FRONT", "FEED_FRONT", FeedPosition::FRONT), crate::meta::inspect::EnumConstant::new("BACK", "FEED_BACK", FeedPosition::BACK)]
        }
    }
}
impl crate::meta::GodotConvert for FeedPosition {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FeedPosition {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FeedPosition {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CameraFeed;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`CameraFeed`][crate::classes::CameraFeed] class."]
    pub struct SignalsOfCameraFeed < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfCameraFeed < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn frame_changed(&mut self) -> SigFrameChanged < 'c, C > {
            SigFrameChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "frame_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn format_changed(&mut self) -> SigFormatChanged < 'c, C > {
            SigFormatChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "format_changed")
            }
        }
    }
    type TypedSigFrameChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFrameChanged < 'c, C: WithSignals > {
        typed: TypedSigFrameChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFrameChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFrameChanged < 'c, C > {
        type Target = TypedSigFrameChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFrameChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFormatChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFormatChanged < 'c, C: WithSignals > {
        typed: TypedSigFormatChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFormatChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFormatChanged < 'c, C > {
        type Target = TypedSigFormatChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFormatChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for CameraFeed {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCameraFeed < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfCameraFeed < 'c, C > {
        type Target = < < CameraFeed as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = CameraFeed;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfCameraFeed < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = CameraFeed;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}