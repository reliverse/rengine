#![doc = "Sidecar module for class [`Sprite2D`][crate::classes::Sprite2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Sprite2D` enums](https://docs.godotengine.org/en/stable/classes/class_sprite2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Sprite2D.`\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`sprite_2d`][crate::classes::sprite_2d]: sidecar module with related enum/flag types\n* [`ISprite2D`][crate::classes::ISprite2D]: virtual methods\n* [`SignalsOfSprite2D`][crate::classes::sprite_2d::SignalsOfSprite2D]: signal collection\n\n\nSee also [Godot docs for `Sprite2D`](https://docs.godotengine.org/en/stable/classes/class_sprite2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Sprite2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Sprite2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Sprite2D`][crate::classes::Sprite2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode2D`][crate::classes::INode2D] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Sprite2D` methods](https://docs.godotengine.org/en/stable/classes/class_sprite2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISprite2D: crate::obj::GodotClass < Base = Sprite2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Sprite2D {
        pub fn set_texture(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8425usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "set_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8426usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "get_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_centered(&mut self, centered: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (centered,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8427usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "set_centered", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_centered(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8428usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "is_centered", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8429usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "set_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8430usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "get_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flip_h(&mut self, flip_h: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (flip_h,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8431usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "set_flip_h", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_flipped_h(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8432usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "is_flipped_h", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flip_v(&mut self, flip_v: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (flip_v,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8433usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "set_flip_v", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_flipped_v(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8434usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "is_flipped_v", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_region_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8435usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "set_region_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_region_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8436usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "is_region_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_pixel_opaque(&self, pos: Vector2,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector2,);
            let args = (pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8437usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "is_pixel_opaque", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_region_rect(&mut self, rect: Rect2,) {
            type CallRet = ();
            type CallParams = (Rect2,);
            let args = (rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8438usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "set_region_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_region_rect(&self,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8439usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "get_region_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_region_filter_clip_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8440usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "set_region_filter_clip_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_region_filter_clip_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8441usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "is_region_filter_clip_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_frame(&mut self, frame: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8442usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "set_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frame(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8443usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "get_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_frame_coords(&mut self, coords: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8444usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "set_frame_coords", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frame_coords(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8445usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "get_frame_coords", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vframes(&mut self, vframes: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (vframes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8446usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "set_vframes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vframes(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8447usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "get_vframes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hframes(&mut self, hframes: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (hframes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8448usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "set_hframes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hframes(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8449usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "get_hframes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rect(&self,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8450usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Sprite2D", "get_rect", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Sprite2D {
        type Base = crate::classes::Node2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Sprite2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Sprite2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for Sprite2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Sprite2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Sprite2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Sprite2D {
        
    }
    impl crate::obj::cap::GodotDefault for Sprite2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Sprite2D {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Sprite2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Sprite2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Sprite2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Sprite2D > for $Class {
                
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Sprite2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Sprite2D`][crate::classes::Sprite2D] class."]
    pub struct SignalsOfSprite2D < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfSprite2D < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn frame_changed(&mut self) -> SigFrameChanged < 'c, C > {
            SigFrameChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "frame_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn texture_changed(&mut self) -> SigTextureChanged < 'c, C > {
            SigTextureChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "texture_changed")
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
    type TypedSigTextureChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTextureChanged < 'c, C: WithSignals > {
        typed: TypedSigTextureChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTextureChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTextureChanged < 'c, C > {
        type Target = TypedSigTextureChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTextureChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Sprite2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfSprite2D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfSprite2D < 'c, C > {
        type Target = < < Sprite2D as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Sprite2D;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfSprite2D < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Sprite2D;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}