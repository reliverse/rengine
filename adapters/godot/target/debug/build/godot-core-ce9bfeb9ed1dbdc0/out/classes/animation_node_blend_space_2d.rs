#![doc = "Sidecar module for class [`AnimationNodeBlendSpace2D`][crate::classes::AnimationNodeBlendSpace2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeBlendSpace2D` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendspace2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationNodeBlendSpace2D.`\n\nInherits [`AnimationRootNode`][crate::classes::AnimationRootNode].\n\nRelated symbols:\n\n* [`animation_node_blend_space_2d`][crate::classes::animation_node_blend_space_2d]: sidecar module with related enum/flag types\n* [`IAnimationNodeBlendSpace2D`][crate::classes::IAnimationNodeBlendSpace2D]: virtual methods\n* [`SignalsOfAnimationNodeBlendSpace2D`][crate::classes::animation_node_blend_space_2d::SignalsOfAnimationNodeBlendSpace2D]: signal collection\n\n\nSee also [Godot docs for `AnimationNodeBlendSpace2D`](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendspace2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AnimationNodeBlendSpace2D::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationNodeBlendSpace2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AnimationNodeBlendSpace2D`][crate::classes::AnimationNodeBlendSpace2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IAnimationRootNode`][crate::classes::IAnimationRootNode] > [`IAnimationNode`][crate::classes::IAnimationNode] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AnimationNodeBlendSpace2D` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendspace2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimationNodeBlendSpace2D: crate::obj::GodotClass < Base = AnimationNodeBlendSpace2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_child_nodes(&self,) -> VarDictionary {
            unimplemented !()
        }
        fn get_parameter_list(&self,) -> VarArray {
            unimplemented !()
        }
        fn get_child_by_name(&self, name: StringName,) -> Option < Gd < crate::classes::AnimationNode > > {
            unimplemented !()
        }
        fn get_parameter_default_value(&self, parameter: StringName,) -> Variant {
            unimplemented !()
        }
        fn is_parameter_read_only(&self, parameter: StringName,) -> bool {
            unimplemented !()
        }
        fn process(&mut self, time: f64, seek: bool, is_external_seeking: bool, test_only: bool,) -> f64 {
            unimplemented !()
        }
        fn get_caption(&self,) -> GString {
            unimplemented !()
        }
        fn has_filter(&self,) -> bool {
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
    impl AnimationNodeBlendSpace2D {
        pub(crate) fn add_blend_point_full(&mut self, node: CowArg < Option < Gd < crate::classes::AnimationRootNode > > >, pos: Vector2, at_index: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::AnimationRootNode > > >, Vector2, i32,);
            let args = (node, pos, at_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "add_blend_point", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_blend_point_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_blend_point(&mut self, node: impl AsArg < Option < Gd < crate::classes::AnimationRootNode >> >, pos: Vector2,) {
            self.add_blend_point_ex(node, pos,) . done()
        }
        #[inline]
        pub fn add_blend_point_ex < 'a > (&'a mut self, node: impl AsArg < Option < Gd < crate::classes::AnimationRootNode >> > + 'a, pos: Vector2,) -> ExAddBlendPoint < 'a > {
            ExAddBlendPoint::new(self, node, pos,)
        }
        pub fn set_blend_point_position(&mut self, point: i32, pos: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2,);
            let args = (point, pos,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "set_blend_point_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_blend_point_position(&self, point: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(361usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "get_blend_point_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_blend_point_node(&mut self, point: i32, node: impl AsArg < Option < Gd < crate::classes::AnimationRootNode >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::AnimationRootNode > > >,);
            let args = (point, node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(362usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "set_blend_point_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_blend_point_node(&self, point: i32,) -> Option < Gd < crate::classes::AnimationRootNode > > {
            type CallRet = Option < Gd < crate::classes::AnimationRootNode > >;
            type CallParams = (i32,);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(363usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "get_blend_point_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_blend_point(&mut self, point: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(364usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "remove_blend_point", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_blend_point_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(365usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "get_blend_point_count", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_triangle_full(&mut self, x: i32, y: i32, z: i32, at_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32, i32, i32,);
            let args = (x, y, z, at_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(366usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "add_triangle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_triangle_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_triangle(&mut self, x: i32, y: i32, z: i32,) {
            self.add_triangle_ex(x, y, z,) . done()
        }
        #[inline]
        pub fn add_triangle_ex < 'a > (&'a mut self, x: i32, y: i32, z: i32,) -> ExAddTriangle < 'a > {
            ExAddTriangle::new(self, x, y, z,)
        }
        pub fn get_triangle_point(&mut self, triangle: i32, point: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, i32,);
            let args = (triangle, point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(367usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "get_triangle_point", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_triangle(&mut self, triangle: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (triangle,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(368usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "remove_triangle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_triangle_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(369usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "get_triangle_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_min_space(&mut self, min_space: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (min_space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(370usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "set_min_space", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_min_space(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(371usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "get_min_space", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_space(&mut self, max_space: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (max_space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(372usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "set_max_space", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_space(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(373usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "get_max_space", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_snap(&mut self, snap: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (snap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(374usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "set_snap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_snap(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(375usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "get_snap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_x_label(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(376usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "set_x_label", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_x_label(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(377usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "get_x_label", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_y_label(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(378usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "set_y_label", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_y_label(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(379usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "get_y_label", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_triangles(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(380usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "set_auto_triangles", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_auto_triangles(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(381usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "get_auto_triangles", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_blend_mode(&mut self, mode: crate::classes::animation_node_blend_space_2d::BlendMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation_node_blend_space_2d::BlendMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(382usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "set_blend_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_blend_mode(&self,) -> crate::classes::animation_node_blend_space_2d::BlendMode {
            type CallRet = crate::classes::animation_node_blend_space_2d::BlendMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(383usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "get_blend_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_sync(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(384usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "set_use_sync", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_using_sync(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(385usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationNodeBlendSpace2D", "is_using_sync", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AnimationNodeBlendSpace2D {
        type Base = crate::classes::AnimationRootNode;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AnimationNodeBlendSpace2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationNodeBlendSpace2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationRootNode > for AnimationNodeBlendSpace2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AnimationNode > for AnimationNodeBlendSpace2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AnimationNodeBlendSpace2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AnimationNodeBlendSpace2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationNodeBlendSpace2D {
        
    }
    impl crate::obj::cap::GodotDefault for AnimationNodeBlendSpace2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimationNodeBlendSpace2D {
        type Target = crate::classes::AnimationRootNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationNodeBlendSpace2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimationNodeBlendSpace2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AnimationNodeBlendSpace2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationNodeBlendSpace2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationRootNode > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimationNode > for $Class {
                
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
#[doc = "Default-param extender for [`AnimationNodeBlendSpace2D::add_blend_point_ex`][super::AnimationNodeBlendSpace2D::add_blend_point_ex]."]
#[must_use]
pub struct ExAddBlendPoint < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimationNodeBlendSpace2D, node: CowArg < 'a, Option < Gd < crate::classes::AnimationRootNode > > >, pos: Vector2, at_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddBlendPoint < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationNodeBlendSpace2D, node: impl AsArg < Option < Gd < crate::classes::AnimationRootNode >> > + 'a, pos: Vector2,) -> Self {
        let at_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, node: node.into_arg(), pos: pos, at_index: at_index,
        }
    }
    #[inline]
    pub fn at_index(self, at_index: i32) -> Self {
        Self {
            at_index: at_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, node, pos, at_index,
        }
        = self;
        re_export::AnimationNodeBlendSpace2D::add_blend_point_full(surround_object, node, pos, at_index,)
    }
}
#[doc = "Default-param extender for [`AnimationNodeBlendSpace2D::add_triangle_ex`][super::AnimationNodeBlendSpace2D::add_triangle_ex]."]
#[must_use]
pub struct ExAddTriangle < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimationNodeBlendSpace2D, x: i32, y: i32, z: i32, at_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTriangle < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationNodeBlendSpace2D, x: i32, y: i32, z: i32,) -> Self {
        let at_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, x: x, y: y, z: z, at_index: at_index,
        }
    }
    #[inline]
    pub fn at_index(self, at_index: i32) -> Self {
        Self {
            at_index: at_index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, x, y, z, at_index,
        }
        = self;
        re_export::AnimationNodeBlendSpace2D::add_triangle_full(surround_object, x, y, z, at_index,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlendMode {
    ord: i32
}
impl BlendMode {
    #[doc(alias = "BLEND_MODE_INTERPOLATED")]
    #[doc = "Godot enumerator name: `BLEND_MODE_INTERPOLATED`"]
    pub const INTERPOLATED: BlendMode = BlendMode {
        ord: 0i32
    };
    #[doc(alias = "BLEND_MODE_DISCRETE")]
    #[doc = "Godot enumerator name: `BLEND_MODE_DISCRETE`"]
    pub const DISCRETE: BlendMode = BlendMode {
        ord: 1i32
    };
    #[doc(alias = "BLEND_MODE_DISCRETE_CARRY")]
    #[doc = "Godot enumerator name: `BLEND_MODE_DISCRETE_CARRY`"]
    pub const DISCRETE_CARRY: BlendMode = BlendMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for BlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BlendMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BlendMode {
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
            Self::INTERPOLATED => "INTERPOLATED", Self::DISCRETE => "DISCRETE", Self::DISCRETE_CARRY => "DISCRETE_CARRY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BlendMode::INTERPOLATED, BlendMode::DISCRETE, BlendMode::DISCRETE_CARRY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BlendMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INTERPOLATED", "BLEND_MODE_INTERPOLATED", BlendMode::INTERPOLATED), crate::meta::inspect::EnumConstant::new("DISCRETE", "BLEND_MODE_DISCRETE", BlendMode::DISCRETE), crate::meta::inspect::EnumConstant::new("DISCRETE_CARRY", "BLEND_MODE_DISCRETE_CARRY", BlendMode::DISCRETE_CARRY)]
        }
    }
}
impl crate::meta::GodotConvert for BlendMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BlendMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AnimationNodeBlendSpace2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`AnimationNodeBlendSpace2D`][crate::classes::AnimationNodeBlendSpace2D] class."]
    pub struct SignalsOfAnimationNodeBlendSpace2D < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfAnimationNodeBlendSpace2D < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn triangles_updated(&mut self) -> SigTrianglesUpdated < 'c, C > {
            SigTrianglesUpdated {
                typed: TypedSignal::extract(&mut self.__internal_obj, "triangles_updated")
            }
        }
    }
    type TypedSigTrianglesUpdated < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTrianglesUpdated < 'c, C: WithSignals > {
        typed: TypedSigTrianglesUpdated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTrianglesUpdated < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTrianglesUpdated < 'c, C > {
        type Target = TypedSigTrianglesUpdated < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTrianglesUpdated < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for AnimationNodeBlendSpace2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAnimationNodeBlendSpace2D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfAnimationNodeBlendSpace2D < 'c, C > {
        type Target = < < AnimationNodeBlendSpace2D as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = AnimationNodeBlendSpace2D;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfAnimationNodeBlendSpace2D < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = AnimationNodeBlendSpace2D;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}