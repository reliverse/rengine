#![doc = "Sidecar module for class [`CsgPolygon3D`][crate::classes::CsgPolygon3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CSGPolygon3D` enums](https://docs.godotengine.org/en/stable/classes/class_csgpolygon3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CSGPolygon3D.`\n\nInherits [`CsgPrimitive3D`][crate::classes::CsgPrimitive3D].\n\nRelated symbols:\n\n* [`csg_polygon_3d`][crate::classes::csg_polygon_3d]: sidecar module with related enum/flag types\n* [`ICsgPolygon3D`][crate::classes::ICsgPolygon3D]: virtual methods\n\n\nSee also [Godot docs for `CSGPolygon3D`](https://docs.godotengine.org/en/stable/classes/class_csgpolygon3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`CsgPolygon3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CsgPolygon3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`CsgPolygon3D`][crate::classes::CsgPolygon3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`ICsgPrimitive3D`~~ > ~~`ICsgShape3D`~~ > [`IGeometryInstance3D`][crate::classes::IGeometryInstance3D] > [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `CSGPolygon3D` methods](https://docs.godotengine.org/en/stable/classes/class_csgpolygon3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICsgPolygon3D: crate::obj::GodotClass < Base = CsgPolygon3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl CsgPolygon3D {
        pub fn set_polygon(&mut self, polygon: &PackedVector2Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >,);
            let args = (RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1564usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_polygon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_polygon(&self,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1565usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_polygon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mode(&mut self, mode: crate::classes::csg_polygon_3d::Mode,) {
            type CallRet = ();
            type CallParams = (crate::classes::csg_polygon_3d::Mode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1566usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mode(&self,) -> crate::classes::csg_polygon_3d::Mode {
            type CallRet = crate::classes::csg_polygon_3d::Mode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1567usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_depth(&mut self, depth: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (depth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1568usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_depth", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_depth(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1569usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_depth", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_spin_degrees(&mut self, degrees: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1570usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_spin_degrees", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_spin_degrees(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1571usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_spin_degrees", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_spin_sides(&mut self, spin_sides: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (spin_sides,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1572usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_spin_sides", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_spin_sides(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1573usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_spin_sides", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_path_node(&mut self, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1574usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_path_node(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1575usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_path_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_path_interval_type(&mut self, interval_type: crate::classes::csg_polygon_3d::PathIntervalType,) {
            type CallRet = ();
            type CallParams = (crate::classes::csg_polygon_3d::PathIntervalType,);
            let args = (interval_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1576usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_interval_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_path_interval_type(&self,) -> crate::classes::csg_polygon_3d::PathIntervalType {
            type CallRet = crate::classes::csg_polygon_3d::PathIntervalType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1577usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_path_interval_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_path_interval(&mut self, interval: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (interval,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1578usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_path_interval(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1579usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_path_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_path_simplify_angle(&mut self, degrees: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (degrees,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1580usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_simplify_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_path_simplify_angle(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1581usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_path_simplify_angle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_path_rotation(&mut self, path_rotation: crate::classes::csg_polygon_3d::PathRotation,) {
            type CallRet = ();
            type CallParams = (crate::classes::csg_polygon_3d::PathRotation,);
            let args = (path_rotation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1582usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_path_rotation(&self,) -> crate::classes::csg_polygon_3d::PathRotation {
            type CallRet = crate::classes::csg_polygon_3d::PathRotation;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1583usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_path_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_path_rotation_accurate(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1584usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_rotation_accurate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_path_rotation_accurate(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1585usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_path_rotation_accurate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_path_local(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1586usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_path_local(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1587usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "is_path_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_path_continuous_u(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1588usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_continuous_u", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_path_continuous_u(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1589usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "is_path_continuous_u", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_path_u_distance(&mut self, distance: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (distance,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1590usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_u_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_path_u_distance(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1591usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_path_u_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_path_joined(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1592usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_path_joined", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_path_joined(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1593usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "is_path_joined", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_material(&mut self, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1594usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_material(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1595usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_smooth_faces(&mut self, smooth_faces: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (smooth_faces,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1596usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "set_smooth_faces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_smooth_faces(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(1597usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CsgPolygon3D", "get_smooth_faces", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CsgPolygon3D {
        type Base = crate::classes::CsgPrimitive3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CSGPolygon3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CsgPolygon3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CsgPrimitive3D > for CsgPolygon3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CsgShape3D > for CsgPolygon3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for CsgPolygon3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for CsgPolygon3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for CsgPolygon3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CsgPolygon3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CsgPolygon3D {
        
    }
    impl crate::obj::cap::GodotDefault for CsgPolygon3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CsgPolygon3D {
        type Target = crate::classes::CsgPrimitive3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CsgPolygon3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CsgPolygon3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CsgPolygon3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CsgPolygon3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CsgPrimitive3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CsgShape3D > for $Class {
                
            }
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
pub struct Mode {
    ord: i32
}
impl Mode {
    #[doc(alias = "MODE_DEPTH")]
    #[doc = "Godot enumerator name: `MODE_DEPTH`"]
    pub const DEPTH: Mode = Mode {
        ord: 0i32
    };
    #[doc(alias = "MODE_SPIN")]
    #[doc = "Godot enumerator name: `MODE_SPIN`"]
    pub const SPIN: Mode = Mode {
        ord: 1i32
    };
    #[doc(alias = "MODE_PATH")]
    #[doc = "Godot enumerator name: `MODE_PATH`"]
    pub const PATH: Mode = Mode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Mode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Mode {
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
            Self::DEPTH => "DEPTH", Self::SPIN => "SPIN", Self::PATH => "PATH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Mode::DEPTH, Mode::SPIN, Mode::PATH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Mode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEPTH", "MODE_DEPTH", Mode::DEPTH), crate::meta::inspect::EnumConstant::new("SPIN", "MODE_SPIN", Mode::SPIN), crate::meta::inspect::EnumConstant::new("PATH", "MODE_PATH", Mode::PATH)]
        }
    }
}
impl crate::meta::GodotConvert for Mode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Mode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Mode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PathRotation {
    ord: i32
}
impl PathRotation {
    #[doc(alias = "PATH_ROTATION_POLYGON")]
    #[doc = "Godot enumerator name: `PATH_ROTATION_POLYGON`"]
    pub const POLYGON: PathRotation = PathRotation {
        ord: 0i32
    };
    #[doc(alias = "PATH_ROTATION_PATH")]
    #[doc = "Godot enumerator name: `PATH_ROTATION_PATH`"]
    pub const PATH: PathRotation = PathRotation {
        ord: 1i32
    };
    #[doc(alias = "PATH_ROTATION_PATH_FOLLOW")]
    #[doc = "Godot enumerator name: `PATH_ROTATION_PATH_FOLLOW`"]
    pub const PATH_FOLLOW: PathRotation = PathRotation {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PathRotation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PathRotation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PathRotation {
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
            Self::POLYGON => "POLYGON", Self::PATH => "PATH", Self::PATH_FOLLOW => "PATH_FOLLOW", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PathRotation::POLYGON, PathRotation::PATH, PathRotation::PATH_FOLLOW]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PathRotation >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("POLYGON", "PATH_ROTATION_POLYGON", PathRotation::POLYGON), crate::meta::inspect::EnumConstant::new("PATH", "PATH_ROTATION_PATH", PathRotation::PATH), crate::meta::inspect::EnumConstant::new("PATH_FOLLOW", "PATH_ROTATION_PATH_FOLLOW", PathRotation::PATH_FOLLOW)]
        }
    }
}
impl crate::meta::GodotConvert for PathRotation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PathRotation {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PathRotation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PathIntervalType {
    ord: i32
}
impl PathIntervalType {
    #[doc(alias = "PATH_INTERVAL_DISTANCE")]
    #[doc = "Godot enumerator name: `PATH_INTERVAL_DISTANCE`"]
    pub const DISTANCE: PathIntervalType = PathIntervalType {
        ord: 0i32
    };
    #[doc(alias = "PATH_INTERVAL_SUBDIVIDE")]
    #[doc = "Godot enumerator name: `PATH_INTERVAL_SUBDIVIDE`"]
    pub const SUBDIVIDE: PathIntervalType = PathIntervalType {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for PathIntervalType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PathIntervalType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PathIntervalType {
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
            Self::DISTANCE => "DISTANCE", Self::SUBDIVIDE => "SUBDIVIDE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PathIntervalType::DISTANCE, PathIntervalType::SUBDIVIDE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PathIntervalType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISTANCE", "PATH_INTERVAL_DISTANCE", PathIntervalType::DISTANCE), crate::meta::inspect::EnumConstant::new("SUBDIVIDE", "PATH_INTERVAL_SUBDIVIDE", PathIntervalType::SUBDIVIDE)]
        }
    }
}
impl crate::meta::GodotConvert for PathIntervalType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PathIntervalType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PathIntervalType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CsgPolygon3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for CsgPolygon3D {
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