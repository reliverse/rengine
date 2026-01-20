#![doc = "Sidecar module for class [`GeometryInstance3D`][crate::classes::GeometryInstance3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GeometryInstance3D` enums](https://docs.godotengine.org/en/stable/classes/class_geometryinstance3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GeometryInstance3D.`\n\nInherits [`VisualInstance3D`][crate::classes::VisualInstance3D].\n\nRelated symbols:\n\n* [`geometry_instance_3d`][crate::classes::geometry_instance_3d]: sidecar module with related enum/flag types\n* [`IGeometryInstance3D`][crate::classes::IGeometryInstance3D]: virtual methods\n\n\nSee also [Godot docs for `GeometryInstance3D`](https://docs.godotengine.org/en/stable/classes/class_geometryinstance3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`GeometryInstance3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GeometryInstance3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GeometryInstance3D`][crate::classes::GeometryInstance3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GeometryInstance3D` methods](https://docs.godotengine.org/en/stable/classes/class_geometryinstance3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGeometryInstance3D: crate::obj::GodotClass < Base = GeometryInstance3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GeometryInstance3D {
        pub fn set_material_override(&mut self, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4048usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_material_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_material_override(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4049usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_material_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_material_overlay(&mut self, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4050usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_material_overlay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_material_overlay(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4051usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_material_overlay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cast_shadows_setting(&mut self, shadow_casting_setting: crate::classes::geometry_instance_3d::ShadowCastingSetting,) {
            type CallRet = ();
            type CallParams = (crate::classes::geometry_instance_3d::ShadowCastingSetting,);
            let args = (shadow_casting_setting,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4052usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_cast_shadows_setting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cast_shadows_setting(&self,) -> crate::classes::geometry_instance_3d::ShadowCastingSetting {
            type CallRet = crate::classes::geometry_instance_3d::ShadowCastingSetting;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4053usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_cast_shadows_setting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lod_bias(&mut self, bias: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (bias,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4054usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_lod_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lod_bias(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4055usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_lod_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transparency(&mut self, transparency: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (transparency,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4056usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_transparency", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transparency(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4057usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_transparency", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_range_end_margin(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4058usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_visibility_range_end_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_range_end_margin(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4059usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_visibility_range_end_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_range_end(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4060usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_visibility_range_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_range_end(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4061usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_visibility_range_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_range_begin_margin(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4062usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_visibility_range_begin_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_range_begin_margin(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4063usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_visibility_range_begin_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_range_begin(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4064usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_visibility_range_begin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_range_begin(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4065usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_visibility_range_begin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visibility_range_fade_mode(&mut self, mode: crate::classes::geometry_instance_3d::VisibilityRangeFadeMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::geometry_instance_3d::VisibilityRangeFadeMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4066usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_visibility_range_fade_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visibility_range_fade_mode(&self,) -> crate::classes::geometry_instance_3d::VisibilityRangeFadeMode {
            type CallRet = crate::classes::geometry_instance_3d::VisibilityRangeFadeMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4067usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_visibility_range_fade_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_instance_shader_parameter(&mut self, name: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4068usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_instance_shader_parameter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_instance_shader_parameter(&self, name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4069usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_instance_shader_parameter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_extra_cull_margin(&mut self, margin: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4070usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_extra_cull_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_extra_cull_margin(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4071usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_extra_cull_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lightmap_texel_scale(&mut self, scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4072usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_lightmap_texel_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lightmap_texel_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4073usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_lightmap_texel_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lightmap_scale(&mut self, scale: crate::classes::geometry_instance_3d::LightmapScale,) {
            type CallRet = ();
            type CallParams = (crate::classes::geometry_instance_3d::LightmapScale,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4074usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_lightmap_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lightmap_scale(&self,) -> crate::classes::geometry_instance_3d::LightmapScale {
            type CallRet = crate::classes::geometry_instance_3d::LightmapScale;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4075usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_lightmap_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gi_mode(&mut self, mode: crate::classes::geometry_instance_3d::GiMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::geometry_instance_3d::GiMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4076usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_gi_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gi_mode(&self,) -> crate::classes::geometry_instance_3d::GiMode {
            type CallRet = crate::classes::geometry_instance_3d::GiMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4077usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_gi_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ignore_occlusion_culling(&mut self, ignore_culling: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (ignore_culling,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4078usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_ignore_occlusion_culling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_ignoring_occlusion_culling(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4079usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "is_ignoring_occlusion_culling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_aabb(&mut self, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Aabb,);
            let args = (aabb,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4080usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "set_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_aabb(&self,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4081usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GeometryInstance3D", "get_custom_aabb", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for GeometryInstance3D {
        type Base = crate::classes::VisualInstance3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GeometryInstance3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GeometryInstance3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for GeometryInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for GeometryInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for GeometryInstance3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GeometryInstance3D {
        
    }
    impl crate::obj::cap::GodotDefault for GeometryInstance3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GeometryInstance3D {
        type Target = crate::classes::VisualInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GeometryInstance3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GeometryInstance3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GeometryInstance3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GeometryInstance3D > for $Class {
                
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
pub struct ShadowCastingSetting {
    ord: i32
}
impl ShadowCastingSetting {
    #[doc(alias = "SHADOW_CASTING_SETTING_OFF")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_OFF`"]
    pub const OFF: ShadowCastingSetting = ShadowCastingSetting {
        ord: 0i32
    };
    #[doc(alias = "SHADOW_CASTING_SETTING_ON")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_ON`"]
    pub const ON: ShadowCastingSetting = ShadowCastingSetting {
        ord: 1i32
    };
    #[doc(alias = "SHADOW_CASTING_SETTING_DOUBLE_SIDED")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_DOUBLE_SIDED`"]
    pub const DOUBLE_SIDED: ShadowCastingSetting = ShadowCastingSetting {
        ord: 2i32
    };
    #[doc(alias = "SHADOW_CASTING_SETTING_SHADOWS_ONLY")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_SHADOWS_ONLY`"]
    pub const SHADOWS_ONLY: ShadowCastingSetting = ShadowCastingSetting {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ShadowCastingSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShadowCastingSetting") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShadowCastingSetting {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::OFF => "OFF", Self::ON => "ON", Self::DOUBLE_SIDED => "DOUBLE_SIDED", Self::SHADOWS_ONLY => "SHADOWS_ONLY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ShadowCastingSetting::OFF, ShadowCastingSetting::ON, ShadowCastingSetting::DOUBLE_SIDED, ShadowCastingSetting::SHADOWS_ONLY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ShadowCastingSetting >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OFF", "SHADOW_CASTING_SETTING_OFF", ShadowCastingSetting::OFF), crate::meta::inspect::EnumConstant::new("ON", "SHADOW_CASTING_SETTING_ON", ShadowCastingSetting::ON), crate::meta::inspect::EnumConstant::new("DOUBLE_SIDED", "SHADOW_CASTING_SETTING_DOUBLE_SIDED", ShadowCastingSetting::DOUBLE_SIDED), crate::meta::inspect::EnumConstant::new("SHADOWS_ONLY", "SHADOW_CASTING_SETTING_SHADOWS_ONLY", ShadowCastingSetting::SHADOWS_ONLY)]
        }
    }
}
impl crate::meta::GodotConvert for ShadowCastingSetting {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ShadowCastingSetting {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShadowCastingSetting {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `GIMode`."]
pub struct GiMode {
    ord: i32
}
impl GiMode {
    #[doc(alias = "GI_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `GI_MODE_DISABLED`"]
    pub const DISABLED: GiMode = GiMode {
        ord: 0i32
    };
    #[doc(alias = "GI_MODE_STATIC")]
    #[doc = "Godot enumerator name: `GI_MODE_STATIC`"]
    pub const STATIC: GiMode = GiMode {
        ord: 1i32
    };
    #[doc(alias = "GI_MODE_DYNAMIC")]
    #[doc = "Godot enumerator name: `GI_MODE_DYNAMIC`"]
    pub const DYNAMIC: GiMode = GiMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for GiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GiMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GiMode {
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
            Self::DISABLED => "DISABLED", Self::STATIC => "STATIC", Self::DYNAMIC => "DYNAMIC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[GiMode::DISABLED, GiMode::STATIC, GiMode::DYNAMIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GiMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "GI_MODE_DISABLED", GiMode::DISABLED), crate::meta::inspect::EnumConstant::new("STATIC", "GI_MODE_STATIC", GiMode::STATIC), crate::meta::inspect::EnumConstant::new("DYNAMIC", "GI_MODE_DYNAMIC", GiMode::DYNAMIC)]
        }
    }
}
impl crate::meta::GodotConvert for GiMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for GiMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GiMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightmapScale {
    ord: i32
}
impl LightmapScale {
    #[doc(alias = "LIGHTMAP_SCALE_1X")]
    #[doc = "Godot enumerator name: `LIGHTMAP_SCALE_1X`"]
    pub const SCALE_1X: LightmapScale = LightmapScale {
        ord: 0i32
    };
    #[doc(alias = "LIGHTMAP_SCALE_2X")]
    #[doc = "Godot enumerator name: `LIGHTMAP_SCALE_2X`"]
    pub const SCALE_2X: LightmapScale = LightmapScale {
        ord: 1i32
    };
    #[doc(alias = "LIGHTMAP_SCALE_4X")]
    #[doc = "Godot enumerator name: `LIGHTMAP_SCALE_4X`"]
    pub const SCALE_4X: LightmapScale = LightmapScale {
        ord: 2i32
    };
    #[doc(alias = "LIGHTMAP_SCALE_8X")]
    #[doc = "Godot enumerator name: `LIGHTMAP_SCALE_8X`"]
    pub const SCALE_8X: LightmapScale = LightmapScale {
        ord: 3i32
    };
    #[doc(alias = "LIGHTMAP_SCALE_MAX")]
    #[doc = "Godot enumerator name: `LIGHTMAP_SCALE_MAX`"]
    pub const MAX: LightmapScale = LightmapScale {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for LightmapScale {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightmapScale") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightmapScale {
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
            Self::SCALE_1X => "SCALE_1X", Self::SCALE_2X => "SCALE_2X", Self::SCALE_4X => "SCALE_4X", Self::SCALE_8X => "SCALE_8X", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightmapScale::SCALE_1X, LightmapScale::SCALE_2X, LightmapScale::SCALE_4X, LightmapScale::SCALE_8X]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightmapScale >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SCALE_1X", "LIGHTMAP_SCALE_1X", LightmapScale::SCALE_1X), crate::meta::inspect::EnumConstant::new("SCALE_2X", "LIGHTMAP_SCALE_2X", LightmapScale::SCALE_2X), crate::meta::inspect::EnumConstant::new("SCALE_4X", "LIGHTMAP_SCALE_4X", LightmapScale::SCALE_4X), crate::meta::inspect::EnumConstant::new("SCALE_8X", "LIGHTMAP_SCALE_8X", LightmapScale::SCALE_8X), crate::meta::inspect::EnumConstant::new("MAX", "LIGHTMAP_SCALE_MAX", LightmapScale::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for LightmapScale {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for LightmapScale {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LightmapScale {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightmapScale {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VisibilityRangeFadeMode {
    ord: i32
}
impl VisibilityRangeFadeMode {
    #[doc(alias = "VISIBILITY_RANGE_FADE_DISABLED")]
    #[doc = "Godot enumerator name: `VISIBILITY_RANGE_FADE_DISABLED`"]
    pub const DISABLED: VisibilityRangeFadeMode = VisibilityRangeFadeMode {
        ord: 0i32
    };
    #[doc(alias = "VISIBILITY_RANGE_FADE_SELF")]
    #[doc = "Godot enumerator name: `VISIBILITY_RANGE_FADE_SELF`"]
    pub const SELF: VisibilityRangeFadeMode = VisibilityRangeFadeMode {
        ord: 1i32
    };
    #[doc(alias = "VISIBILITY_RANGE_FADE_DEPENDENCIES")]
    #[doc = "Godot enumerator name: `VISIBILITY_RANGE_FADE_DEPENDENCIES`"]
    pub const DEPENDENCIES: VisibilityRangeFadeMode = VisibilityRangeFadeMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for VisibilityRangeFadeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VisibilityRangeFadeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VisibilityRangeFadeMode {
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
            Self::DISABLED => "DISABLED", Self::SELF => "SELF", Self::DEPENDENCIES => "DEPENDENCIES", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VisibilityRangeFadeMode::DISABLED, VisibilityRangeFadeMode::SELF, VisibilityRangeFadeMode::DEPENDENCIES]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VisibilityRangeFadeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VISIBILITY_RANGE_FADE_DISABLED", VisibilityRangeFadeMode::DISABLED), crate::meta::inspect::EnumConstant::new("SELF", "VISIBILITY_RANGE_FADE_SELF", VisibilityRangeFadeMode::SELF), crate::meta::inspect::EnumConstant::new("DEPENDENCIES", "VISIBILITY_RANGE_FADE_DEPENDENCIES", VisibilityRangeFadeMode::DEPENDENCIES)]
        }
    }
}
impl crate::meta::GodotConvert for VisibilityRangeFadeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VisibilityRangeFadeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VisibilityRangeFadeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GeometryInstance3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for GeometryInstance3D {
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