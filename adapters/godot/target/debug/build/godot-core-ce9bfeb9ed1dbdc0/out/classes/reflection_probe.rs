#![doc = "Sidecar module for class [`ReflectionProbe`][crate::classes::ReflectionProbe].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ReflectionProbe` enums](https://docs.godotengine.org/en/stable/classes/class_reflectionprobe.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ReflectionProbe.`\n\nInherits [`VisualInstance3D`][crate::classes::VisualInstance3D].\n\nRelated symbols:\n\n* [`reflection_probe`][crate::classes::reflection_probe]: sidecar module with related enum/flag types\n* [`IReflectionProbe`][crate::classes::IReflectionProbe]: virtual methods\n\n\nSee also [Godot docs for `ReflectionProbe`](https://docs.godotengine.org/en/stable/classes/class_reflectionprobe.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`ReflectionProbe::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ReflectionProbe {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ReflectionProbe`][crate::classes::ReflectionProbe].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `ReflectionProbe` methods](https://docs.godotengine.org/en/stable/classes/class_reflectionprobe.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IReflectionProbe: crate::obj::GodotClass < Base = ReflectionProbe > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_aabb(&self,) -> Aabb {
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
    impl ReflectionProbe {
        pub fn set_intensity(&mut self, intensity: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (intensity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_intensity(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_blend_distance(&mut self, blend_distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (blend_distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_blend_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_blend_distance(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_blend_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ambient_mode(&mut self, ambient: crate::classes::reflection_probe::AmbientMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::reflection_probe::AmbientMode,);
            let args = (ambient,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_ambient_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ambient_mode(&self,) -> crate::classes::reflection_probe::AmbientMode {
            type CallRet = crate::classes::reflection_probe::AmbientMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_ambient_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ambient_color(&mut self, ambient: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (ambient,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_ambient_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ambient_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_ambient_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ambient_color_energy(&mut self, ambient_energy: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ambient_energy,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_ambient_color_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ambient_color_energy(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_ambient_color_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_distance(&mut self, max_distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (max_distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_max_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_distance(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_max_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mesh_lod_threshold(&mut self, ratio: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_mesh_lod_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mesh_lod_threshold(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_mesh_lod_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_size(&mut self, size: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_origin_offset(&mut self, origin_offset: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (origin_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_origin_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_origin_offset(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_origin_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_as_interior(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_as_interior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_set_as_interior(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "is_set_as_interior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enable_box_projection(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_enable_box_projection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_box_projection_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "is_box_projection_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enable_shadows(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_enable_shadows", Some(self.__validated_obj()), args,)
            }
        }
        pub fn are_shadows_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "are_shadows_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cull_mask(&mut self, layers: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (layers,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cull_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_reflection_mask(&mut self, layers: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (layers,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_reflection_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_reflection_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_reflection_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_update_mode(&mut self, mode: crate::classes::reflection_probe::UpdateMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::reflection_probe::UpdateMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "set_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_update_mode(&self,) -> crate::classes::reflection_probe::UpdateMode {
            type CallRet = crate::classes::reflection_probe::UpdateMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ReflectionProbe", "get_update_mode", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ReflectionProbe {
        type Base = crate::classes::VisualInstance3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ReflectionProbe"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ReflectionProbe {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for ReflectionProbe {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for ReflectionProbe {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for ReflectionProbe {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ReflectionProbe {
        
    }
    impl crate::obj::cap::GodotDefault for ReflectionProbe {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ReflectionProbe {
        type Target = crate::classes::VisualInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ReflectionProbe {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ReflectionProbe`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ReflectionProbe__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ReflectionProbe > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualInstance3D > for $Class {
                
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
pub struct UpdateMode {
    ord: i32
}
impl UpdateMode {
    #[doc(alias = "UPDATE_ONCE")]
    #[doc = "Godot enumerator name: `UPDATE_ONCE`"]
    pub const ONCE: UpdateMode = UpdateMode {
        ord: 0i32
    };
    #[doc(alias = "UPDATE_ALWAYS")]
    #[doc = "Godot enumerator name: `UPDATE_ALWAYS`"]
    pub const ALWAYS: UpdateMode = UpdateMode {
        ord: 1i32
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
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::ONCE => "ONCE", Self::ALWAYS => "ALWAYS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[UpdateMode::ONCE, UpdateMode::ALWAYS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < UpdateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ONCE", "UPDATE_ONCE", UpdateMode::ONCE), crate::meta::inspect::EnumConstant::new("ALWAYS", "UPDATE_ALWAYS", UpdateMode::ALWAYS)]
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AmbientMode {
    ord: i32
}
impl AmbientMode {
    #[doc(alias = "AMBIENT_DISABLED")]
    #[doc = "Godot enumerator name: `AMBIENT_DISABLED`"]
    pub const DISABLED: AmbientMode = AmbientMode {
        ord: 0i32
    };
    #[doc(alias = "AMBIENT_ENVIRONMENT")]
    #[doc = "Godot enumerator name: `AMBIENT_ENVIRONMENT`"]
    pub const ENVIRONMENT: AmbientMode = AmbientMode {
        ord: 1i32
    };
    #[doc(alias = "AMBIENT_COLOR")]
    #[doc = "Godot enumerator name: `AMBIENT_COLOR`"]
    pub const COLOR: AmbientMode = AmbientMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AmbientMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AmbientMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AmbientMode {
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
            Self::DISABLED => "DISABLED", Self::ENVIRONMENT => "ENVIRONMENT", Self::COLOR => "COLOR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AmbientMode::DISABLED, AmbientMode::ENVIRONMENT, AmbientMode::COLOR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AmbientMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "AMBIENT_DISABLED", AmbientMode::DISABLED), crate::meta::inspect::EnumConstant::new("ENVIRONMENT", "AMBIENT_ENVIRONMENT", AmbientMode::ENVIRONMENT), crate::meta::inspect::EnumConstant::new("COLOR", "AMBIENT_COLOR", AmbientMode::COLOR)]
        }
    }
}
impl crate::meta::GodotConvert for AmbientMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AmbientMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AmbientMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ReflectionProbe;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for ReflectionProbe {
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