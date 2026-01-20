#![doc = "Sidecar module for class [`StyleBoxFlat`][crate::classes::StyleBoxFlat].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `StyleBoxFlat` enums](https://docs.godotengine.org/en/stable/classes/class_styleboxflat.html#enumerations).\n\n"]
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
    #[doc = "Godot class `StyleBoxFlat.`\n\nInherits [`StyleBox`][crate::classes::StyleBox].\n\nRelated symbols:\n\n* [`IStyleBoxFlat`][crate::classes::IStyleBoxFlat]: virtual methods\n\n\nSee also [Godot docs for `StyleBoxFlat`](https://docs.godotengine.org/en/stable/classes/class_styleboxflat.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`StyleBoxFlat::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct StyleBoxFlat {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`StyleBoxFlat`][crate::classes::StyleBoxFlat].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IStyleBox`][crate::classes::IStyleBox] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `StyleBoxFlat` methods](https://docs.godotengine.org/en/stable/classes/class_styleboxflat.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IStyleBoxFlat: crate::obj::GodotClass < Base = StyleBoxFlat > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn draw(&self, to_canvas_item: Rid, rect: Rect2,);
        fn get_draw_rect(&self, rect: Rect2,) -> Rect2 {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn test_mask(&self, point: Vector2, rect: Rect2,) -> bool {
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
    impl StyleBoxFlat {
        pub fn set_bg_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8605usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_bg_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bg_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8606usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_bg_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_border_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8607usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_border_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_border_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8608usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_border_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_border_width_all(&mut self, width: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8609usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_border_width_all", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_border_width_min(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8610usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_border_width_min", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_border_width(&mut self, margin: crate::global::Side, width: i32,) {
            type CallRet = ();
            type CallParams = (crate::global::Side, i32,);
            let args = (margin, width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8611usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_border_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_border_width(&self, margin: crate::global::Side,) -> i32 {
            type CallRet = i32;
            type CallParams = (crate::global::Side,);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8612usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_border_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_border_blend(&mut self, blend: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (blend,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8613usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_border_blend", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_border_blend(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8614usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_border_blend", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_corner_radius_all(&mut self, radius: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8615usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_corner_radius_all", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_corner_radius(&mut self, corner: crate::global::Corner, radius: i32,) {
            type CallRet = ();
            type CallParams = (crate::global::Corner, i32,);
            let args = (corner, radius,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8616usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_corner_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_corner_radius(&self, corner: crate::global::Corner,) -> i32 {
            type CallRet = i32;
            type CallParams = (crate::global::Corner,);
            let args = (corner,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8617usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_corner_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_expand_margin(&mut self, margin: crate::global::Side, size: f32,) {
            type CallRet = ();
            type CallParams = (crate::global::Side, f32,);
            let args = (margin, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8618usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_expand_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_expand_margin_all(&mut self, size: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8619usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_expand_margin_all", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_expand_margin(&self, margin: crate::global::Side,) -> f32 {
            type CallRet = f32;
            type CallParams = (crate::global::Side,);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8620usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_expand_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_draw_center(&mut self, draw_center: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (draw_center,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8621usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_draw_center", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_draw_center_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8622usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "is_draw_center_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_skew(&mut self, skew: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (skew,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8623usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_skew", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_skew(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8624usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_skew", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8625usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_shadow_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadow_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8626usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_shadow_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8627usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_shadow_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadow_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8628usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_shadow_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_shadow_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8629usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_shadow_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shadow_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8630usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_shadow_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_anti_aliased(&mut self, anti_aliased: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (anti_aliased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8631usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_anti_aliased", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_anti_aliased(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8632usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "is_anti_aliased", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_aa_size(&mut self, size: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8633usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_aa_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_aa_size(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8634usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_aa_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_corner_detail(&mut self, detail: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (detail,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8635usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "set_corner_detail", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_corner_detail(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8636usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StyleBoxFlat", "get_corner_detail", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for StyleBoxFlat {
        type Base = crate::classes::StyleBox;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("StyleBoxFlat"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for StyleBoxFlat {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::StyleBox > for StyleBoxFlat {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for StyleBoxFlat {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for StyleBoxFlat {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for StyleBoxFlat {
        
    }
    impl crate::obj::cap::GodotDefault for StyleBoxFlat {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for StyleBoxFlat {
        type Target = crate::classes::StyleBox;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for StyleBoxFlat {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`StyleBoxFlat`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_StyleBoxFlat__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::StyleBoxFlat > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::StyleBox > for $Class {
                
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::StyleBoxFlat;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for StyleBoxFlat {
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