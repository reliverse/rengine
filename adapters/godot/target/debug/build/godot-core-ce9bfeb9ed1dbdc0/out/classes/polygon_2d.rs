#![doc = "Sidecar module for class [`Polygon2D`][crate::classes::Polygon2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Polygon2D` enums](https://docs.godotengine.org/en/stable/classes/class_polygon2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Polygon2D.`\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`IPolygon2D`][crate::classes::IPolygon2D]: virtual methods\n\n\nSee also [Godot docs for `Polygon2D`](https://docs.godotengine.org/en/stable/classes/class_polygon2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Polygon2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Polygon2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Polygon2D`][crate::classes::Polygon2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode2D`][crate::classes::INode2D] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Polygon2D` methods](https://docs.godotengine.org/en/stable/classes/class_polygon2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPolygon2D: crate::obj::GodotClass < Base = Polygon2D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Polygon2D {
        pub fn set_polygon(&mut self, polygon: &PackedVector2Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >,);
            let args = (RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6689usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_polygon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_polygon(&self,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6690usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_polygon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_uv(&mut self, uv: &PackedVector2Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >,);
            let args = (RefArg::new(uv),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_uv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_uv(&self,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_uv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_polygons(&mut self, polygons: &VarArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarArray >,);
            let args = (RefArg::new(polygons),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_polygons", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_polygons(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_polygons", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_vertex_colors(&mut self, vertex_colors: &PackedColorArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedColorArray >,);
            let args = (RefArg::new(vertex_colors),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_vertex_colors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vertex_colors(&self,) -> PackedColorArray {
            type CallRet = PackedColorArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_vertex_colors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_offset(&mut self, texture_offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (texture_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_texture_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6702usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_texture_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_rotation(&mut self, texture_rotation: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (texture_rotation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6703usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_texture_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_rotation(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_texture_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_scale(&mut self, texture_scale: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (texture_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_texture_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_scale(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_texture_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_invert_enabled(&mut self, invert: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (invert,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6707usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_invert_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_invert_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6708usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_invert_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_antialiased(&mut self, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6709usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_antialiased", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_antialiased(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6710usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_antialiased", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_invert_border(&mut self, invert_border: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (invert_border,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6711usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_invert_border", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_invert_border(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6712usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_invert_border", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_offset(&mut self, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6713usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_offset(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6714usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_bone(&mut self, path: impl AsArg < NodePath >, weights: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, NodePath >, RefArg < 'a1, PackedFloat32Array >,);
            let args = (path.into_arg(), RefArg::new(weights),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6715usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "add_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bone_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6716usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_bone_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bone_path(&self, index: i32,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6717usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_bone_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bone_weights(&self, index: i32,) -> PackedFloat32Array {
            type CallRet = PackedFloat32Array;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6718usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_bone_weights", Some(self.__validated_obj()), args,)
            }
        }
        pub fn erase_bone(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6719usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "erase_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_bones(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6720usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "clear_bones", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bone_path(&mut self, index: i32, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, NodePath >,);
            let args = (index, path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6721usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_bone_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bone_weights(&mut self, index: i32, weights: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, PackedFloat32Array >,);
            let args = (index, RefArg::new(weights),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6722usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_bone_weights", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_skeleton(&mut self, skeleton: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (skeleton.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6723usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_skeleton", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_skeleton(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6724usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_skeleton", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_internal_vertex_count(&mut self, internal_vertex_count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (internal_vertex_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6725usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "set_internal_vertex_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_internal_vertex_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6726usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Polygon2D", "get_internal_vertex_count", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Polygon2D {
        type Base = crate::classes::Node2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Polygon2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Polygon2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for Polygon2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Polygon2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Polygon2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Polygon2D {
        
    }
    impl crate::obj::cap::GodotDefault for Polygon2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Polygon2D {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Polygon2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Polygon2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Polygon2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Polygon2D > for $Class {
                
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
    use super::re_export::Polygon2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::canvas_item::SignalsOfCanvasItem;
    impl WithSignals for Polygon2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCanvasItem < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}