#![doc = "Sidecar module for class [`CharFxTransform`][crate::classes::CharFxTransform].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CharFXTransform` enums](https://docs.godotengine.org/en/stable/classes/class_charfxtransform.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CharFXTransform.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`ICharFxTransform`][crate::classes::ICharFxTransform]: virtual methods\n\n\nSee also [Godot docs for `CharFXTransform`](https://docs.godotengine.org/en/stable/classes/class_charfxtransform.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`CharFxTransform::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CharFxTransform {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`CharFxTransform`][crate::classes::CharFxTransform].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `CharFXTransform` methods](https://docs.godotengine.org/en/stable/classes/class_charfxtransform.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICharFxTransform: crate::obj::GodotClass < Base = CharFxTransform > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl CharFxTransform {
        pub fn get_transform(&mut self,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1978usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transform(&mut self, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Transform2D,);
            let args = (transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1979usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_range(&mut self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1980usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_range(&mut self, range: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (range,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1981usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_elapsed_time(&mut self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1982usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_elapsed_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_elapsed_time(&mut self, time: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1983usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_elapsed_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_visible(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1984usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "is_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility(&mut self, visibility: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visibility,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1985usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_visibility", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_outline(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1986usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "is_outline", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_outline(&mut self, outline: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (outline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1987usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_outline", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_offset(&mut self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1988usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1989usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color(&mut self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1990usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1991usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_environment(&mut self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1992usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_environment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_environment(&mut self, environment: &VarDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >,);
            let args = (RefArg::new(environment),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1993usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_environment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glyph_index(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1994usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_glyph_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glyph_index(&mut self, glyph_index: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (glyph_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1995usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_glyph_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_relative_index(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1996usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_relative_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_relative_index(&mut self, relative_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (relative_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1997usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_relative_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glyph_count(&self,) -> u8 {
            type CallRet = u8;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1998usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_glyph_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glyph_count(&mut self, glyph_count: u8,) {
            type CallRet = ();
            type CallParams = (u8,);
            let args = (glyph_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1999usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_glyph_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glyph_flags(&self,) -> crate::classes::text_server::GraphemeFlag {
            type CallRet = crate::classes::text_server::GraphemeFlag;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2000usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_glyph_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glyph_flags(&mut self, glyph_flags: crate::classes::text_server::GraphemeFlag,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::GraphemeFlag,);
            let args = (glyph_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2001usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_glyph_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2002usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "get_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font(&mut self, font: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (font,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2003usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CharFxTransform", "set_font", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CharFxTransform {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CharFXTransform"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CharFxTransform {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for CharFxTransform {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CharFxTransform {
        
    }
    impl crate::obj::cap::GodotDefault for CharFxTransform {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CharFxTransform {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CharFxTransform {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CharFxTransform`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CharFxTransform__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CharFxTransform > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
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
    use super::re_export::CharFxTransform;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for CharFxTransform {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfObject < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}