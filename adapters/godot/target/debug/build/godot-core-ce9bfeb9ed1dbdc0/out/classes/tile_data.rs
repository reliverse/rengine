#![doc = "Sidecar module for class [`TileData`][crate::classes::TileData].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileData` enums](https://docs.godotengine.org/en/stable/classes/class_tiledata.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileData.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`tile_data`][crate::classes::tile_data]: sidecar module with related enum/flag types\n* [`ITileData`][crate::classes::ITileData]: virtual methods\n* [`SignalsOfTileData`][crate::classes::tile_data::SignalsOfTileData]: signal collection\n\n\nSee also [Godot docs for `TileData`](https://docs.godotengine.org/en/stable/classes/class_tiledata.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`TileData::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileData {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TileData`][crate::classes::TileData].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `TileData` methods](https://docs.godotengine.org/en/stable/classes/class_tiledata.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileData: crate::obj::GodotClass < Base = TileData > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TileData {
        pub fn set_flip_h(&mut self, flip_h: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (flip_h,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9662usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_flip_h", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_flip_h(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9663usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_flip_h", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flip_v(&mut self, flip_v: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (flip_v,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9664usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_flip_v", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_flip_v(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9665usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_flip_v", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transpose(&mut self, transpose: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (transpose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9666usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_transpose", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transpose(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9667usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_transpose", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_material(&mut self, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_material(&self,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9669usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_origin(&mut self, texture_origin: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (texture_origin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_texture_origin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_origin(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_texture_origin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_modulate(&mut self, modulate: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_modulate(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9673usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_z_index(&mut self, z_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (z_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9674usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_z_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_z_index(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9675usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_z_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_y_sort_origin(&mut self, y_sort_origin: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (y_sort_origin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9676usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_y_sort_origin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_y_sort_origin(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9677usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_y_sort_origin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_occluder_polygons_count(&mut self, layer_id: i32, polygons_count: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer_id, polygons_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9678usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_occluder_polygons_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_occluder_polygons_count(&self, layer_id: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (layer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9679usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_occluder_polygons_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_occluder_polygon(&mut self, layer_id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9680usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "add_occluder_polygon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_occluder_polygon(&mut self, layer_id: i32, polygon_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer_id, polygon_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9681usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "remove_occluder_polygon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_occluder_polygon(&mut self, layer_id: i32, polygon_index: i32, polygon: impl AsArg < Option < Gd < crate::classes::OccluderPolygon2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, Option < Gd < crate::classes::OccluderPolygon2D > > >,);
            let args = (layer_id, polygon_index, polygon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9682usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_occluder_polygon", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_occluder_polygon_full(&self, layer_id: i32, polygon_index: i32, flip_h: bool, flip_v: bool, transpose: bool,) -> Option < Gd < crate::classes::OccluderPolygon2D > > {
            type CallRet = Option < Gd < crate::classes::OccluderPolygon2D > >;
            type CallParams = (i32, i32, bool, bool, bool,);
            let args = (layer_id, polygon_index, flip_h, flip_v, transpose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9683usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_occluder_polygon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_occluder_polygon_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_occluder_polygon(&self, layer_id: i32, polygon_index: i32,) -> Option < Gd < crate::classes::OccluderPolygon2D > > {
            self.get_occluder_polygon_ex(layer_id, polygon_index,) . done()
        }
        #[inline]
        pub fn get_occluder_polygon_ex < 'a > (&'a self, layer_id: i32, polygon_index: i32,) -> ExGetOccluderPolygon < 'a > {
            ExGetOccluderPolygon::new(self, layer_id, polygon_index,)
        }
        pub fn set_occluder(&mut self, layer_id: i32, occluder_polygon: impl AsArg < Option < Gd < crate::classes::OccluderPolygon2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::OccluderPolygon2D > > >,);
            let args = (layer_id, occluder_polygon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9684usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_occluder", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_occluder_full(&self, layer_id: i32, flip_h: bool, flip_v: bool, transpose: bool,) -> Option < Gd < crate::classes::OccluderPolygon2D > > {
            type CallRet = Option < Gd < crate::classes::OccluderPolygon2D > >;
            type CallParams = (i32, bool, bool, bool,);
            let args = (layer_id, flip_h, flip_v, transpose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9685usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_occluder", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_occluder_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_occluder(&self, layer_id: i32,) -> Option < Gd < crate::classes::OccluderPolygon2D > > {
            self.get_occluder_ex(layer_id,) . done()
        }
        #[inline]
        pub fn get_occluder_ex < 'a > (&'a self, layer_id: i32,) -> ExGetOccluder < 'a > {
            ExGetOccluder::new(self, layer_id,)
        }
        pub fn set_constant_linear_velocity(&mut self, layer_id: i32, velocity: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2,);
            let args = (layer_id, velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9686usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_constant_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_constant_linear_velocity(&self, layer_id: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (layer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9687usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_constant_linear_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_constant_angular_velocity(&mut self, layer_id: i32, velocity: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (layer_id, velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9688usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_constant_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_constant_angular_velocity(&self, layer_id: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (layer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9689usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_constant_angular_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_polygons_count(&mut self, layer_id: i32, polygons_count: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer_id, polygons_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9690usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_collision_polygons_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_polygons_count(&self, layer_id: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (layer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9691usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_collision_polygons_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_collision_polygon(&mut self, layer_id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9692usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "add_collision_polygon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_collision_polygon(&mut self, layer_id: i32, polygon_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer_id, polygon_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9693usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "remove_collision_polygon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_polygon_points(&mut self, layer_id: i32, polygon_index: i32, polygon: &PackedVector2Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, RefArg < 'a0, PackedVector2Array >,);
            let args = (layer_id, polygon_index, RefArg::new(polygon),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9694usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_collision_polygon_points", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_polygon_points(&self, layer_id: i32, polygon_index: i32,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = (i32, i32,);
            let args = (layer_id, polygon_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9695usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_collision_polygon_points", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_polygon_one_way(&mut self, layer_id: i32, polygon_index: i32, one_way: bool,) {
            type CallRet = ();
            type CallParams = (i32, i32, bool,);
            let args = (layer_id, polygon_index, one_way,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9696usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_collision_polygon_one_way", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_collision_polygon_one_way(&self, layer_id: i32, polygon_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, i32,);
            let args = (layer_id, polygon_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9697usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "is_collision_polygon_one_way", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_polygon_one_way_margin(&mut self, layer_id: i32, polygon_index: i32, one_way_margin: f32,) {
            type CallRet = ();
            type CallParams = (i32, i32, f32,);
            let args = (layer_id, polygon_index, one_way_margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9698usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_collision_polygon_one_way_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_polygon_one_way_margin(&self, layer_id: i32, polygon_index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (layer_id, polygon_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9699usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_collision_polygon_one_way_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_terrain_set(&mut self, terrain_set: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (terrain_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9700usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_terrain_set", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_terrain_set(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9701usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_terrain_set", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_terrain(&mut self, terrain: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (terrain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9702usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_terrain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_terrain(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9703usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_terrain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_terrain_peering_bit(&mut self, peering_bit: crate::classes::tile_set::CellNeighbor, terrain: i32,) {
            type CallRet = ();
            type CallParams = (crate::classes::tile_set::CellNeighbor, i32,);
            let args = (peering_bit, terrain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9704usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_terrain_peering_bit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_terrain_peering_bit(&self, peering_bit: crate::classes::tile_set::CellNeighbor,) -> i32 {
            type CallRet = i32;
            type CallParams = (crate::classes::tile_set::CellNeighbor,);
            let args = (peering_bit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9705usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_terrain_peering_bit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_valid_terrain_peering_bit(&self, peering_bit: crate::classes::tile_set::CellNeighbor,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::tile_set::CellNeighbor,);
            let args = (peering_bit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9706usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "is_valid_terrain_peering_bit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_probability(&mut self, probability: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (probability,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9707usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_probability", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_probability(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9708usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_probability", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_data(&mut self, layer_name: impl AsArg < GString >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, Variant >,);
            let args = (layer_name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9709usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_custom_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_data(&self, layer_name: impl AsArg < GString >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (layer_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9710usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_custom_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_custom_data(&self, layer_name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (layer_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9711usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "has_custom_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_data_by_layer_id(&mut self, layer_id: i32, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (layer_id, RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9712usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "set_custom_data_by_layer_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_data_by_layer_id(&self, layer_id: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (layer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9713usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileData", "get_custom_data_by_layer_id", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TileData {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TileData"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileData {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TileData {
        
    }
    impl crate::obj::cap::GodotDefault for TileData {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileData {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileData {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TileData`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TileData__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TileData > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TileData::get_occluder_polygon_ex`][super::TileData::get_occluder_polygon_ex]."]
#[must_use]
pub struct ExGetOccluderPolygon < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TileData, layer_id: i32, polygon_index: i32, flip_h: bool, flip_v: bool, transpose: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetOccluderPolygon < 'a > {
    fn new(surround_object: &'a re_export::TileData, layer_id: i32, polygon_index: i32,) -> Self {
        let flip_h = false;
        let flip_v = false;
        let transpose = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer_id: layer_id, polygon_index: polygon_index, flip_h: flip_h, flip_v: flip_v, transpose: transpose,
        }
    }
    #[inline]
    pub fn flip_h(self, flip_h: bool) -> Self {
        Self {
            flip_h: flip_h, .. self
        }
    }
    #[inline]
    pub fn flip_v(self, flip_v: bool) -> Self {
        Self {
            flip_v: flip_v, .. self
        }
    }
    #[inline]
    pub fn transpose(self, transpose: bool) -> Self {
        Self {
            transpose: transpose, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::OccluderPolygon2D > > {
        let Self {
            _phantom, surround_object, layer_id, polygon_index, flip_h, flip_v, transpose,
        }
        = self;
        re_export::TileData::get_occluder_polygon_full(surround_object, layer_id, polygon_index, flip_h, flip_v, transpose,)
    }
}
#[doc = "Default-param extender for [`TileData::get_occluder_ex`][super::TileData::get_occluder_ex]."]
#[must_use]
pub struct ExGetOccluder < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TileData, layer_id: i32, flip_h: bool, flip_v: bool, transpose: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetOccluder < 'a > {
    fn new(surround_object: &'a re_export::TileData, layer_id: i32,) -> Self {
        let flip_h = false;
        let flip_v = false;
        let transpose = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer_id: layer_id, flip_h: flip_h, flip_v: flip_v, transpose: transpose,
        }
    }
    #[inline]
    pub fn flip_h(self, flip_h: bool) -> Self {
        Self {
            flip_h: flip_h, .. self
        }
    }
    #[inline]
    pub fn flip_v(self, flip_v: bool) -> Self {
        Self {
            flip_v: flip_v, .. self
        }
    }
    #[inline]
    pub fn transpose(self, transpose: bool) -> Self {
        Self {
            transpose: transpose, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::OccluderPolygon2D > > {
        let Self {
            _phantom, surround_object, layer_id, flip_h, flip_v, transpose,
        }
        = self;
        re_export::TileData::get_occluder_full(surround_object, layer_id, flip_h, flip_v, transpose,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TileData;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`TileData`][crate::classes::TileData] class."]
    pub struct SignalsOfTileData < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfTileData < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn changed(&mut self) -> SigChanged < 'c, C > {
            SigChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "changed")
            }
        }
    }
    type TypedSigChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigChanged < 'c, C: WithSignals > {
        typed: TypedSigChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigChanged < 'c, C > {
        type Target = TypedSigChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for TileData {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfTileData < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfTileData < 'c, C > {
        type Target = < < TileData as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = TileData;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfTileData < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = TileData;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}