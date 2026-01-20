#![doc = "Sidecar module for class [`Curve2D`][crate::classes::Curve2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Curve2D` enums](https://docs.godotengine.org/en/stable/classes/class_curve2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Curve2D.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`curve_2d`][crate::classes::curve_2d]: sidecar module with related enum/flag types\n* [`ICurve2D`][crate::classes::ICurve2D]: virtual methods\n\n\nSee also [Godot docs for `Curve2D`](https://docs.godotengine.org/en/stable/classes/class_curve2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Curve2D::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Curve2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Curve2D`][crate::classes::Curve2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Curve2D` methods](https://docs.godotengine.org/en/stable/classes/class_curve2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICurve2D: crate::obj::GodotClass < Base = Curve2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Curve2D {
        pub fn get_point_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2711usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "get_point_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_point_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2712usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "set_point_count", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_point_full(&mut self, position: Vector2, in_: Vector2, out: Vector2, index: i32,) {
            type CallRet = ();
            type CallParams = (Vector2, Vector2, Vector2, i32,);
            let args = (position, in_, out, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2713usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "add_point", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_point_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_point(&mut self, position: Vector2,) {
            self.add_point_ex(position,) . done()
        }
        #[inline]
        pub fn add_point_ex < 'a > (&'a mut self, position: Vector2,) -> ExAddPoint < 'a > {
            ExAddPoint::new(self, position,)
        }
        pub fn set_point_position(&mut self, idx: i32, position: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2,);
            let args = (idx, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2714usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "set_point_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_point_position(&self, idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2715usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "get_point_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_point_in(&mut self, idx: i32, position: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2,);
            let args = (idx, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2716usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "set_point_in", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_point_in(&self, idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2717usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "get_point_in", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_point_out(&mut self, idx: i32, position: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2,);
            let args = (idx, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2718usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "set_point_out", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_point_out(&self, idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2719usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "get_point_out", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_point(&mut self, idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2720usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "remove_point", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_points(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2721usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "clear_points", Some(self.__validated_obj()), args,)
            }
        }
        pub fn sample(&self, idx: i32, t: f32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32, f32,);
            let args = (idx, t,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2722usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "sample", Some(self.__validated_obj()), args,)
            }
        }
        pub fn samplef(&self, fofs: f32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (f32,);
            let args = (fofs,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2723usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "samplef", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bake_interval(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2724usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "set_bake_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bake_interval(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2725usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "get_bake_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_baked_length(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2726usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "get_baked_length", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn sample_baked_full(&self, offset: f32, cubic: bool,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (f32, bool,);
            let args = (offset, cubic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2727usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "sample_baked", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::sample_baked_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn sample_baked(&self,) -> Vector2 {
            self.sample_baked_ex() . done()
        }
        #[inline]
        pub fn sample_baked_ex < 'a > (&'a self,) -> ExSampleBaked < 'a > {
            ExSampleBaked::new(self,)
        }
        pub(crate) fn sample_baked_with_rotation_full(&self, offset: f32, cubic: bool,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = (f32, bool,);
            let args = (offset, cubic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2728usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "sample_baked_with_rotation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::sample_baked_with_rotation_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn sample_baked_with_rotation(&self,) -> Transform2D {
            self.sample_baked_with_rotation_ex() . done()
        }
        #[inline]
        pub fn sample_baked_with_rotation_ex < 'a > (&'a self,) -> ExSampleBakedWithRotation < 'a > {
            ExSampleBakedWithRotation::new(self,)
        }
        pub fn get_baked_points(&self,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2729usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "get_baked_points", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_closest_point(&self, to_point: Vector2,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector2,);
            let args = (to_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2730usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "get_closest_point", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_closest_offset(&self, to_point: Vector2,) -> f32 {
            type CallRet = f32;
            type CallParams = (Vector2,);
            let args = (to_point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2731usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "get_closest_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn tessellate_full(&self, max_stages: i32, tolerance_degrees: f32,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = (i32, f32,);
            let args = (max_stages, tolerance_degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2732usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "tessellate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::tessellate_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn tessellate(&self,) -> PackedVector2Array {
            self.tessellate_ex() . done()
        }
        #[inline]
        pub fn tessellate_ex < 'a > (&'a self,) -> ExTessellate < 'a > {
            ExTessellate::new(self,)
        }
        pub(crate) fn tessellate_even_length_full(&self, max_stages: i32, tolerance_length: f32,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = (i32, f32,);
            let args = (max_stages, tolerance_length,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2733usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Curve2D", "tessellate_even_length", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::tessellate_even_length_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn tessellate_even_length(&self,) -> PackedVector2Array {
            self.tessellate_even_length_ex() . done()
        }
        #[inline]
        pub fn tessellate_even_length_ex < 'a > (&'a self,) -> ExTessellateEvenLength < 'a > {
            ExTessellateEvenLength::new(self,)
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
    impl crate::obj::GodotClass for Curve2D {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Curve2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Curve2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Curve2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Curve2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Curve2D {
        
    }
    impl crate::obj::cap::GodotDefault for Curve2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Curve2D {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Curve2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Curve2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Curve2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Curve2D > for $Class {
                
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
#[doc = "Default-param extender for [`Curve2D::add_point_ex`][super::Curve2D::add_point_ex]."]
#[must_use]
pub struct ExAddPoint < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Curve2D, position: Vector2, in_: Vector2, out: Vector2, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPoint < 'a > {
    fn new(surround_object: &'a mut re_export::Curve2D, position: Vector2,) -> Self {
        let in_ = Vector2::new(0 as _, 0 as _);
        let out = Vector2::new(0 as _, 0 as _);
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, in_: in_, out: out, index: index,
        }
    }
    #[inline]
    pub fn in_(self, in_: Vector2) -> Self {
        Self {
            in_: in_, .. self
        }
    }
    #[inline]
    pub fn out(self, out: Vector2) -> Self {
        Self {
            out: out, .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, in_, out, index,
        }
        = self;
        re_export::Curve2D::add_point_full(surround_object, position, in_, out, index,)
    }
}
#[doc = "Default-param extender for [`Curve2D::sample_baked_ex`][super::Curve2D::sample_baked_ex]."]
#[must_use]
pub struct ExSampleBaked < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Curve2D, offset: f32, cubic: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSampleBaked < 'a > {
    fn new(surround_object: &'a re_export::Curve2D,) -> Self {
        let offset = 0f32;
        let cubic = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, offset: offset, cubic: cubic,
        }
    }
    #[inline]
    pub fn offset(self, offset: f32) -> Self {
        Self {
            offset: offset, .. self
        }
    }
    #[inline]
    pub fn cubic(self, cubic: bool) -> Self {
        Self {
            cubic: cubic, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2 {
        let Self {
            _phantom, surround_object, offset, cubic,
        }
        = self;
        re_export::Curve2D::sample_baked_full(surround_object, offset, cubic,)
    }
}
#[doc = "Default-param extender for [`Curve2D::sample_baked_with_rotation_ex`][super::Curve2D::sample_baked_with_rotation_ex]."]
#[must_use]
pub struct ExSampleBakedWithRotation < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Curve2D, offset: f32, cubic: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSampleBakedWithRotation < 'a > {
    fn new(surround_object: &'a re_export::Curve2D,) -> Self {
        let offset = 0f32;
        let cubic = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, offset: offset, cubic: cubic,
        }
    }
    #[inline]
    pub fn offset(self, offset: f32) -> Self {
        Self {
            offset: offset, .. self
        }
    }
    #[inline]
    pub fn cubic(self, cubic: bool) -> Self {
        Self {
            cubic: cubic, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Transform2D {
        let Self {
            _phantom, surround_object, offset, cubic,
        }
        = self;
        re_export::Curve2D::sample_baked_with_rotation_full(surround_object, offset, cubic,)
    }
}
#[doc = "Default-param extender for [`Curve2D::tessellate_ex`][super::Curve2D::tessellate_ex]."]
#[must_use]
pub struct ExTessellate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Curve2D, max_stages: i32, tolerance_degrees: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTessellate < 'a > {
    fn new(surround_object: &'a re_export::Curve2D,) -> Self {
        let max_stages = 5i32;
        let tolerance_degrees = 4f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, max_stages: max_stages, tolerance_degrees: tolerance_degrees,
        }
    }
    #[inline]
    pub fn max_stages(self, max_stages: i32) -> Self {
        Self {
            max_stages: max_stages, .. self
        }
    }
    #[inline]
    pub fn tolerance_degrees(self, tolerance_degrees: f32) -> Self {
        Self {
            tolerance_degrees: tolerance_degrees, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedVector2Array {
        let Self {
            _phantom, surround_object, max_stages, tolerance_degrees,
        }
        = self;
        re_export::Curve2D::tessellate_full(surround_object, max_stages, tolerance_degrees,)
    }
}
#[doc = "Default-param extender for [`Curve2D::tessellate_even_length_ex`][super::Curve2D::tessellate_even_length_ex]."]
#[must_use]
pub struct ExTessellateEvenLength < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Curve2D, max_stages: i32, tolerance_length: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTessellateEvenLength < 'a > {
    fn new(surround_object: &'a re_export::Curve2D,) -> Self {
        let max_stages = 5i32;
        let tolerance_length = 20f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, max_stages: max_stages, tolerance_length: tolerance_length,
        }
    }
    #[inline]
    pub fn max_stages(self, max_stages: i32) -> Self {
        Self {
            max_stages: max_stages, .. self
        }
    }
    #[inline]
    pub fn tolerance_length(self, tolerance_length: f32) -> Self {
        Self {
            tolerance_length: tolerance_length, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedVector2Array {
        let Self {
            _phantom, surround_object, max_stages, tolerance_length,
        }
        = self;
        re_export::Curve2D::tessellate_even_length_full(surround_object, max_stages, tolerance_length,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Curve2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Curve2D {
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