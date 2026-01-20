#![doc = "Sidecar module for class [`TileSet`][crate::classes::TileSet].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileSet` enums](https://docs.godotengine.org/en/stable/classes/class_tileset.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileSet.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`tile_set`][crate::classes::tile_set]: sidecar module with related enum/flag types\n* [`ITileSet`][crate::classes::ITileSet]: virtual methods\n\n\nSee also [Godot docs for `TileSet`](https://docs.godotengine.org/en/stable/classes/class_tileset.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TileSet::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileSet {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TileSet`][crate::classes::TileSet].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `TileSet` methods](https://docs.godotengine.org/en/stable/classes/class_tileset.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileSet: crate::obj::GodotClass < Base = TileSet > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TileSet {
        pub fn get_next_source_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9840usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_next_source_id", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_source_full(&mut self, source: CowArg < Option < Gd < crate::classes::TileSetSource > > >, atlas_source_id_override: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TileSetSource > > >, i32,);
            let args = (source, atlas_source_id_override,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9841usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "add_source", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_source_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_source(&mut self, source: impl AsArg < Option < Gd < crate::classes::TileSetSource >> >,) -> i32 {
            self.add_source_ex(source,) . done()
        }
        #[inline]
        pub fn add_source_ex < 'a > (&'a mut self, source: impl AsArg < Option < Gd < crate::classes::TileSetSource >> > + 'a,) -> ExAddSource < 'a > {
            ExAddSource::new(self, source,)
        }
        pub fn remove_source(&mut self, source_id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9842usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_source_id(&mut self, source_id: i32, new_source_id: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (source_id, new_source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9843usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_source_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_source_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9844usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_source_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_source_id(&self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9845usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_source_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_source(&self, source_id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9846usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "has_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_source(&self, source_id: i32,) -> Option < Gd < crate::classes::TileSetSource > > {
            type CallRet = Option < Gd < crate::classes::TileSetSource > >;
            type CallParams = (i32,);
            let args = (source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9847usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tile_shape(&mut self, shape: crate::classes::tile_set::TileShape,) {
            type CallRet = ();
            type CallParams = (crate::classes::tile_set::TileShape,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9848usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_tile_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tile_shape(&self,) -> crate::classes::tile_set::TileShape {
            type CallRet = crate::classes::tile_set::TileShape;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9849usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_tile_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tile_layout(&mut self, layout: crate::classes::tile_set::TileLayout,) {
            type CallRet = ();
            type CallParams = (crate::classes::tile_set::TileLayout,);
            let args = (layout,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9850usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_tile_layout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tile_layout(&self,) -> crate::classes::tile_set::TileLayout {
            type CallRet = crate::classes::tile_set::TileLayout;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9851usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_tile_layout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tile_offset_axis(&mut self, alignment: crate::classes::tile_set::TileOffsetAxis,) {
            type CallRet = ();
            type CallParams = (crate::classes::tile_set::TileOffsetAxis,);
            let args = (alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9852usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_tile_offset_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tile_offset_axis(&self,) -> crate::classes::tile_set::TileOffsetAxis {
            type CallRet = crate::classes::tile_set::TileOffsetAxis;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9853usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_tile_offset_axis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tile_size(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9854usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_tile_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tile_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9855usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_tile_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_uv_clipping(&mut self, uv_clipping: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (uv_clipping,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9856usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_uv_clipping", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_uv_clipping(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9857usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "is_uv_clipping", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_occlusion_layers_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9858usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_occlusion_layers_count", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_occlusion_layer_full(&mut self, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "add_occlusion_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_occlusion_layer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_occlusion_layer(&mut self,) {
            self.add_occlusion_layer_ex() . done()
        }
        #[inline]
        pub fn add_occlusion_layer_ex < 'a > (&'a mut self,) -> ExAddOcclusionLayer < 'a > {
            ExAddOcclusionLayer::new(self,)
        }
        pub fn move_occlusion_layer(&mut self, layer_index: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "move_occlusion_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_occlusion_layer(&mut self, layer_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9861usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_occlusion_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_occlusion_layer_light_mask(&mut self, layer_index: i32, light_mask: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer_index, light_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9862usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_occlusion_layer_light_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_occlusion_layer_light_mask(&self, layer_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9863usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_occlusion_layer_light_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_occlusion_layer_sdf_collision(&mut self, layer_index: i32, sdf_collision: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer_index, sdf_collision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9864usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_occlusion_layer_sdf_collision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_occlusion_layer_sdf_collision(&self, layer_index: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9865usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_occlusion_layer_sdf_collision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_layers_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9866usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_physics_layers_count", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_physics_layer_full(&mut self, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9867usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "add_physics_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_physics_layer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_physics_layer(&mut self,) {
            self.add_physics_layer_ex() . done()
        }
        #[inline]
        pub fn add_physics_layer_ex < 'a > (&'a mut self,) -> ExAddPhysicsLayer < 'a > {
            ExAddPhysicsLayer::new(self,)
        }
        pub fn move_physics_layer(&mut self, layer_index: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9868usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "move_physics_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_physics_layer(&mut self, layer_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9869usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_physics_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_layer_collision_layer(&mut self, layer_index: i32, layer: u32,) {
            type CallRet = ();
            type CallParams = (i32, u32,);
            let args = (layer_index, layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9870usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_physics_layer_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_layer_collision_layer(&self, layer_index: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9871usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_physics_layer_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_layer_collision_mask(&mut self, layer_index: i32, mask: u32,) {
            type CallRet = ();
            type CallParams = (i32, u32,);
            let args = (layer_index, mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9872usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_physics_layer_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_layer_collision_mask(&self, layer_index: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9873usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_physics_layer_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_layer_collision_priority(&mut self, layer_index: i32, priority: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (layer_index, priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9874usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_physics_layer_collision_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_layer_collision_priority(&self, layer_index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9875usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_physics_layer_collision_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_layer_physics_material(&mut self, layer_index: i32, physics_material: impl AsArg < Option < Gd < crate::classes::PhysicsMaterial >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::PhysicsMaterial > > >,);
            let args = (layer_index, physics_material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9876usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_physics_layer_physics_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_layer_physics_material(&self, layer_index: i32,) -> Option < Gd < crate::classes::PhysicsMaterial > > {
            type CallRet = Option < Gd < crate::classes::PhysicsMaterial > >;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9877usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_physics_layer_physics_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_terrain_sets_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9878usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_terrain_sets_count", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_terrain_set_full(&mut self, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9879usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "add_terrain_set", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_terrain_set_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_terrain_set(&mut self,) {
            self.add_terrain_set_ex() . done()
        }
        #[inline]
        pub fn add_terrain_set_ex < 'a > (&'a mut self,) -> ExAddTerrainSet < 'a > {
            ExAddTerrainSet::new(self,)
        }
        pub fn move_terrain_set(&mut self, terrain_set: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (terrain_set, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9880usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "move_terrain_set", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_terrain_set(&mut self, terrain_set: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (terrain_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9881usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_terrain_set", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_terrain_set_mode(&mut self, terrain_set: i32, mode: crate::classes::tile_set::TerrainMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::tile_set::TerrainMode,);
            let args = (terrain_set, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9882usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_terrain_set_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_terrain_set_mode(&self, terrain_set: i32,) -> crate::classes::tile_set::TerrainMode {
            type CallRet = crate::classes::tile_set::TerrainMode;
            type CallParams = (i32,);
            let args = (terrain_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9883usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_terrain_set_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_terrains_count(&self, terrain_set: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (terrain_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9884usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_terrains_count", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_terrain_full(&mut self, terrain_set: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (terrain_set, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9885usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "add_terrain", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_terrain_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_terrain(&mut self, terrain_set: i32,) {
            self.add_terrain_ex(terrain_set,) . done()
        }
        #[inline]
        pub fn add_terrain_ex < 'a > (&'a mut self, terrain_set: i32,) -> ExAddTerrain < 'a > {
            ExAddTerrain::new(self, terrain_set,)
        }
        pub fn move_terrain(&mut self, terrain_set: i32, terrain_index: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32, i32,);
            let args = (terrain_set, terrain_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9886usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "move_terrain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_terrain(&mut self, terrain_set: i32, terrain_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (terrain_set, terrain_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9887usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_terrain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_terrain_name(&mut self, terrain_set: i32, terrain_index: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, CowArg < 'a0, GString >,);
            let args = (terrain_set, terrain_index, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9888usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_terrain_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_terrain_name(&self, terrain_set: i32, terrain_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32, i32,);
            let args = (terrain_set, terrain_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9889usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_terrain_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_terrain_color(&mut self, terrain_set: i32, terrain_index: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, i32, Color,);
            let args = (terrain_set, terrain_index, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9890usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_terrain_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_terrain_color(&self, terrain_set: i32, terrain_index: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32, i32,);
            let args = (terrain_set, terrain_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9891usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_terrain_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_navigation_layers_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9892usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_navigation_layers_count", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_navigation_layer_full(&mut self, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9893usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "add_navigation_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_navigation_layer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_navigation_layer(&mut self,) {
            self.add_navigation_layer_ex() . done()
        }
        #[inline]
        pub fn add_navigation_layer_ex < 'a > (&'a mut self,) -> ExAddNavigationLayer < 'a > {
            ExAddNavigationLayer::new(self,)
        }
        pub fn move_navigation_layer(&mut self, layer_index: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9894usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "move_navigation_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_navigation_layer(&mut self, layer_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9895usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_navigation_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_navigation_layer_layers(&mut self, layer_index: i32, layers: u32,) {
            type CallRet = ();
            type CallParams = (i32, u32,);
            let args = (layer_index, layers,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9896usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_navigation_layer_layers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_navigation_layer_layers(&self, layer_index: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9897usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_navigation_layer_layers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_navigation_layer_layer_value(&mut self, layer_index: i32, layer_number: i32, value: bool,) {
            type CallRet = ();
            type CallParams = (i32, i32, bool,);
            let args = (layer_index, layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9898usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_navigation_layer_layer_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_navigation_layer_layer_value(&self, layer_index: i32, layer_number: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, i32,);
            let args = (layer_index, layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9899usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_navigation_layer_layer_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_data_layers_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9900usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_custom_data_layers_count", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_custom_data_layer_full(&mut self, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9901usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "add_custom_data_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_custom_data_layer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_custom_data_layer(&mut self,) {
            self.add_custom_data_layer_ex() . done()
        }
        #[inline]
        pub fn add_custom_data_layer_ex < 'a > (&'a mut self,) -> ExAddCustomDataLayer < 'a > {
            ExAddCustomDataLayer::new(self,)
        }
        pub fn move_custom_data_layer(&mut self, layer_index: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer_index, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9902usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "move_custom_data_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_custom_data_layer(&mut self, layer_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9903usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_custom_data_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_data_layer_by_name(&self, layer_name: impl AsArg < GString >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (layer_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9904usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_custom_data_layer_by_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_data_layer_name(&mut self, layer_index: i32, layer_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (layer_index, layer_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9905usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_custom_data_layer_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_custom_data_layer_by_name(&self, layer_name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (layer_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9906usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "has_custom_data_layer_by_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_data_layer_name(&self, layer_index: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9907usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_custom_data_layer_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_custom_data_layer_type(&mut self, layer_index: i32, layer_type: VariantType,) {
            type CallRet = ();
            type CallParams = (i32, VariantType,);
            let args = (layer_index, layer_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9908usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_custom_data_layer_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_data_layer_type(&self, layer_index: i32,) -> VariantType {
            type CallRet = VariantType;
            type CallParams = (i32,);
            let args = (layer_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9909usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_custom_data_layer_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_source_level_tile_proxy(&mut self, source_from: i32, source_to: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (source_from, source_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9910usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_source_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_source_level_tile_proxy(&mut self, source_from: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (source_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9911usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_source_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_source_level_tile_proxy(&mut self, source_from: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (source_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9912usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "has_source_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_source_level_tile_proxy(&mut self, source_from: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (source_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9913usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_source_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_coords_level_tile_proxy(&mut self, p_source_from: i32, coords_from: Vector2i, source_to: i32, coords_to: Vector2i,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32, Vector2i,);
            let args = (p_source_from, coords_from, source_to, coords_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9914usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_coords_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_coords_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32, Vector2i,);
            let args = (source_from, coords_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9915usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_coords_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_coords_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, Vector2i,);
            let args = (source_from, coords_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9916usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "has_coords_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_coords_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i,);
            let args = (source_from, coords_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9917usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_coords_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_alternative_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i, alternative_from: i32, source_to: i32, coords_to: Vector2i, alternative_to: i32,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32, i32, Vector2i, i32,);
            let args = (source_from, coords_from, alternative_from, source_to, coords_to, alternative_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9918usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "set_alternative_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_alternative_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i, alternative_from: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32, Vector2i, i32,);
            let args = (source_from, coords_from, alternative_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9919usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_alternative_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_alternative_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i, alternative_from: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, Vector2i, i32,);
            let args = (source_from, coords_from, alternative_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9920usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "has_alternative_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_alternative_level_tile_proxy(&mut self, source_from: i32, coords_from: Vector2i, alternative_from: i32,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32,);
            let args = (source_from, coords_from, alternative_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9921usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_alternative_level_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn map_tile_proxy(&self, source_from: i32, coords_from: Vector2i, alternative_from: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32, Vector2i, i32,);
            let args = (source_from, coords_from, alternative_from,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9922usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "map_tile_proxy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn cleanup_invalid_tile_proxies(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9923usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "cleanup_invalid_tile_proxies", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_tile_proxies(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9924usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "clear_tile_proxies", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_pattern_full(&mut self, pattern: CowArg < Option < Gd < crate::classes::TileMapPattern > > >, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TileMapPattern > > >, i32,);
            let args = (pattern, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9925usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "add_pattern", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_pattern_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_pattern(&mut self, pattern: impl AsArg < Option < Gd < crate::classes::TileMapPattern >> >,) -> i32 {
            self.add_pattern_ex(pattern,) . done()
        }
        #[inline]
        pub fn add_pattern_ex < 'a > (&'a mut self, pattern: impl AsArg < Option < Gd < crate::classes::TileMapPattern >> > + 'a,) -> ExAddPattern < 'a > {
            ExAddPattern::new(self, pattern,)
        }
        pub(crate) fn get_pattern_full(&mut self, index: i32,) -> Option < Gd < crate::classes::TileMapPattern > > {
            type CallRet = Option < Gd < crate::classes::TileMapPattern > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9926usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_pattern", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_pattern_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_pattern(&mut self,) -> Option < Gd < crate::classes::TileMapPattern > > {
            self.get_pattern_ex() . done()
        }
        #[inline]
        pub fn get_pattern_ex < 'a > (&'a mut self,) -> ExGetPattern < 'a > {
            ExGetPattern::new(self,)
        }
        pub fn remove_pattern(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9927usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "remove_pattern", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_patterns_count(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9928usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSet", "get_patterns_count", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TileSet {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TileSet"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileSet {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for TileSet {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TileSet {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TileSet {
        
    }
    impl crate::obj::cap::GodotDefault for TileSet {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileSet {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileSet {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TileSet`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TileSet__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TileSet > for $Class {
                
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
#[doc = "Default-param extender for [`TileSet::add_source_ex`][super::TileSet::add_source_ex]."]
#[must_use]
pub struct ExAddSource < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, source: CowArg < 'a, Option < Gd < crate::classes::TileSetSource > > >, atlas_source_id_override: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSource < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet, source: impl AsArg < Option < Gd < crate::classes::TileSetSource >> > + 'a,) -> Self {
        let atlas_source_id_override = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, source: source.into_arg(), atlas_source_id_override: atlas_source_id_override,
        }
    }
    #[inline]
    pub fn atlas_source_id_override(self, atlas_source_id_override: i32) -> Self {
        Self {
            atlas_source_id_override: atlas_source_id_override, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, source, atlas_source_id_override,
        }
        = self;
        re_export::TileSet::add_source_full(surround_object, source, atlas_source_id_override,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_occlusion_layer_ex`][super::TileSet::add_occlusion_layer_ex]."]
#[must_use]
pub struct ExAddOcclusionLayer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddOcclusionLayer < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, to_position,
        }
        = self;
        re_export::TileSet::add_occlusion_layer_full(surround_object, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_physics_layer_ex`][super::TileSet::add_physics_layer_ex]."]
#[must_use]
pub struct ExAddPhysicsLayer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPhysicsLayer < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, to_position,
        }
        = self;
        re_export::TileSet::add_physics_layer_full(surround_object, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_terrain_set_ex`][super::TileSet::add_terrain_set_ex]."]
#[must_use]
pub struct ExAddTerrainSet < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTerrainSet < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, to_position,
        }
        = self;
        re_export::TileSet::add_terrain_set_full(surround_object, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_terrain_ex`][super::TileSet::add_terrain_ex]."]
#[must_use]
pub struct ExAddTerrain < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, terrain_set: i32, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTerrain < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet, terrain_set: i32,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, terrain_set: terrain_set, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, terrain_set, to_position,
        }
        = self;
        re_export::TileSet::add_terrain_full(surround_object, terrain_set, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_navigation_layer_ex`][super::TileSet::add_navigation_layer_ex]."]
#[must_use]
pub struct ExAddNavigationLayer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddNavigationLayer < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, to_position,
        }
        = self;
        re_export::TileSet::add_navigation_layer_full(surround_object, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_custom_data_layer_ex`][super::TileSet::add_custom_data_layer_ex]."]
#[must_use]
pub struct ExAddCustomDataLayer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, to_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCustomDataLayer < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        let to_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, to_position: to_position,
        }
    }
    #[inline]
    pub fn to_position(self, to_position: i32) -> Self {
        Self {
            to_position: to_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, to_position,
        }
        = self;
        re_export::TileSet::add_custom_data_layer_full(surround_object, to_position,)
    }
}
#[doc = "Default-param extender for [`TileSet::add_pattern_ex`][super::TileSet::add_pattern_ex]."]
#[must_use]
pub struct ExAddPattern < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, pattern: CowArg < 'a, Option < Gd < crate::classes::TileMapPattern > > >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPattern < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet, pattern: impl AsArg < Option < Gd < crate::classes::TileMapPattern >> > + 'a,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, pattern: pattern.into_arg(), index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, pattern, index,
        }
        = self;
        re_export::TileSet::add_pattern_full(surround_object, pattern, index,)
    }
}
#[doc = "Default-param extender for [`TileSet::get_pattern_ex`][super::TileSet::get_pattern_ex]."]
#[must_use]
pub struct ExGetPattern < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSet, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetPattern < 'a > {
    fn new(surround_object: &'a mut re_export::TileSet,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TileMapPattern > > {
        let Self {
            _phantom, surround_object, index,
        }
        = self;
        re_export::TileSet::get_pattern_full(surround_object, index,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TileShape {
    ord: i32
}
impl TileShape {
    #[doc(alias = "TILE_SHAPE_SQUARE")]
    #[doc = "Godot enumerator name: `TILE_SHAPE_SQUARE`"]
    pub const SQUARE: TileShape = TileShape {
        ord: 0i32
    };
    #[doc(alias = "TILE_SHAPE_ISOMETRIC")]
    #[doc = "Godot enumerator name: `TILE_SHAPE_ISOMETRIC`"]
    pub const ISOMETRIC: TileShape = TileShape {
        ord: 1i32
    };
    #[doc(alias = "TILE_SHAPE_HALF_OFFSET_SQUARE")]
    #[doc = "Godot enumerator name: `TILE_SHAPE_HALF_OFFSET_SQUARE`"]
    pub const HALF_OFFSET_SQUARE: TileShape = TileShape {
        ord: 2i32
    };
    #[doc(alias = "TILE_SHAPE_HEXAGON")]
    #[doc = "Godot enumerator name: `TILE_SHAPE_HEXAGON`"]
    pub const HEXAGON: TileShape = TileShape {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TileShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TileShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TileShape {
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
            Self::SQUARE => "SQUARE", Self::ISOMETRIC => "ISOMETRIC", Self::HALF_OFFSET_SQUARE => "HALF_OFFSET_SQUARE", Self::HEXAGON => "HEXAGON", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TileShape::SQUARE, TileShape::ISOMETRIC, TileShape::HALF_OFFSET_SQUARE, TileShape::HEXAGON]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TileShape >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SQUARE", "TILE_SHAPE_SQUARE", TileShape::SQUARE), crate::meta::inspect::EnumConstant::new("ISOMETRIC", "TILE_SHAPE_ISOMETRIC", TileShape::ISOMETRIC), crate::meta::inspect::EnumConstant::new("HALF_OFFSET_SQUARE", "TILE_SHAPE_HALF_OFFSET_SQUARE", TileShape::HALF_OFFSET_SQUARE), crate::meta::inspect::EnumConstant::new("HEXAGON", "TILE_SHAPE_HEXAGON", TileShape::HEXAGON)]
        }
    }
}
impl crate::meta::GodotConvert for TileShape {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TileShape {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TileShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TileLayout {
    ord: i32
}
impl TileLayout {
    #[doc(alias = "TILE_LAYOUT_STACKED")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_STACKED`"]
    pub const STACKED: TileLayout = TileLayout {
        ord: 0i32
    };
    #[doc(alias = "TILE_LAYOUT_STACKED_OFFSET")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_STACKED_OFFSET`"]
    pub const STACKED_OFFSET: TileLayout = TileLayout {
        ord: 1i32
    };
    #[doc(alias = "TILE_LAYOUT_STAIRS_RIGHT")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_STAIRS_RIGHT`"]
    pub const STAIRS_RIGHT: TileLayout = TileLayout {
        ord: 2i32
    };
    #[doc(alias = "TILE_LAYOUT_STAIRS_DOWN")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_STAIRS_DOWN`"]
    pub const STAIRS_DOWN: TileLayout = TileLayout {
        ord: 3i32
    };
    #[doc(alias = "TILE_LAYOUT_DIAMOND_RIGHT")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_DIAMOND_RIGHT`"]
    pub const DIAMOND_RIGHT: TileLayout = TileLayout {
        ord: 4i32
    };
    #[doc(alias = "TILE_LAYOUT_DIAMOND_DOWN")]
    #[doc = "Godot enumerator name: `TILE_LAYOUT_DIAMOND_DOWN`"]
    pub const DIAMOND_DOWN: TileLayout = TileLayout {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for TileLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TileLayout") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TileLayout {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::STACKED => "STACKED", Self::STACKED_OFFSET => "STACKED_OFFSET", Self::STAIRS_RIGHT => "STAIRS_RIGHT", Self::STAIRS_DOWN => "STAIRS_DOWN", Self::DIAMOND_RIGHT => "DIAMOND_RIGHT", Self::DIAMOND_DOWN => "DIAMOND_DOWN", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TileLayout::STACKED, TileLayout::STACKED_OFFSET, TileLayout::STAIRS_RIGHT, TileLayout::STAIRS_DOWN, TileLayout::DIAMOND_RIGHT, TileLayout::DIAMOND_DOWN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TileLayout >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STACKED", "TILE_LAYOUT_STACKED", TileLayout::STACKED), crate::meta::inspect::EnumConstant::new("STACKED_OFFSET", "TILE_LAYOUT_STACKED_OFFSET", TileLayout::STACKED_OFFSET), crate::meta::inspect::EnumConstant::new("STAIRS_RIGHT", "TILE_LAYOUT_STAIRS_RIGHT", TileLayout::STAIRS_RIGHT), crate::meta::inspect::EnumConstant::new("STAIRS_DOWN", "TILE_LAYOUT_STAIRS_DOWN", TileLayout::STAIRS_DOWN), crate::meta::inspect::EnumConstant::new("DIAMOND_RIGHT", "TILE_LAYOUT_DIAMOND_RIGHT", TileLayout::DIAMOND_RIGHT), crate::meta::inspect::EnumConstant::new("DIAMOND_DOWN", "TILE_LAYOUT_DIAMOND_DOWN", TileLayout::DIAMOND_DOWN)]
        }
    }
}
impl crate::meta::GodotConvert for TileLayout {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TileLayout {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TileLayout {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TileOffsetAxis {
    ord: i32
}
impl TileOffsetAxis {
    #[doc(alias = "TILE_OFFSET_AXIS_HORIZONTAL")]
    #[doc = "Godot enumerator name: `TILE_OFFSET_AXIS_HORIZONTAL`"]
    pub const HORIZONTAL: TileOffsetAxis = TileOffsetAxis {
        ord: 0i32
    };
    #[doc(alias = "TILE_OFFSET_AXIS_VERTICAL")]
    #[doc = "Godot enumerator name: `TILE_OFFSET_AXIS_VERTICAL`"]
    pub const VERTICAL: TileOffsetAxis = TileOffsetAxis {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for TileOffsetAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TileOffsetAxis") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TileOffsetAxis {
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
            Self::HORIZONTAL => "HORIZONTAL", Self::VERTICAL => "VERTICAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TileOffsetAxis::HORIZONTAL, TileOffsetAxis::VERTICAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TileOffsetAxis >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("HORIZONTAL", "TILE_OFFSET_AXIS_HORIZONTAL", TileOffsetAxis::HORIZONTAL), crate::meta::inspect::EnumConstant::new("VERTICAL", "TILE_OFFSET_AXIS_VERTICAL", TileOffsetAxis::VERTICAL)]
        }
    }
}
impl crate::meta::GodotConvert for TileOffsetAxis {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TileOffsetAxis {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TileOffsetAxis {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CellNeighbor {
    ord: i32
}
impl CellNeighbor {
    #[doc(alias = "CELL_NEIGHBOR_RIGHT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_RIGHT_SIDE`"]
    pub const RIGHT_SIDE: CellNeighbor = CellNeighbor {
        ord: 0i32
    };
    #[doc(alias = "CELL_NEIGHBOR_RIGHT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_RIGHT_CORNER`"]
    pub const RIGHT_CORNER: CellNeighbor = CellNeighbor {
        ord: 1i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_RIGHT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_RIGHT_SIDE`"]
    pub const BOTTOM_RIGHT_SIDE: CellNeighbor = CellNeighbor {
        ord: 2i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_RIGHT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_RIGHT_CORNER`"]
    pub const BOTTOM_RIGHT_CORNER: CellNeighbor = CellNeighbor {
        ord: 3i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_SIDE`"]
    pub const BOTTOM_SIDE: CellNeighbor = CellNeighbor {
        ord: 4i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_CORNER`"]
    pub const BOTTOM_CORNER: CellNeighbor = CellNeighbor {
        ord: 5i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_LEFT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_LEFT_SIDE`"]
    pub const BOTTOM_LEFT_SIDE: CellNeighbor = CellNeighbor {
        ord: 6i32
    };
    #[doc(alias = "CELL_NEIGHBOR_BOTTOM_LEFT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_BOTTOM_LEFT_CORNER`"]
    pub const BOTTOM_LEFT_CORNER: CellNeighbor = CellNeighbor {
        ord: 7i32
    };
    #[doc(alias = "CELL_NEIGHBOR_LEFT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_LEFT_SIDE`"]
    pub const LEFT_SIDE: CellNeighbor = CellNeighbor {
        ord: 8i32
    };
    #[doc(alias = "CELL_NEIGHBOR_LEFT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_LEFT_CORNER`"]
    pub const LEFT_CORNER: CellNeighbor = CellNeighbor {
        ord: 9i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_LEFT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_LEFT_SIDE`"]
    pub const TOP_LEFT_SIDE: CellNeighbor = CellNeighbor {
        ord: 10i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_LEFT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_LEFT_CORNER`"]
    pub const TOP_LEFT_CORNER: CellNeighbor = CellNeighbor {
        ord: 11i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_SIDE`"]
    pub const TOP_SIDE: CellNeighbor = CellNeighbor {
        ord: 12i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_CORNER`"]
    pub const TOP_CORNER: CellNeighbor = CellNeighbor {
        ord: 13i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_RIGHT_SIDE")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_RIGHT_SIDE`"]
    pub const TOP_RIGHT_SIDE: CellNeighbor = CellNeighbor {
        ord: 14i32
    };
    #[doc(alias = "CELL_NEIGHBOR_TOP_RIGHT_CORNER")]
    #[doc = "Godot enumerator name: `CELL_NEIGHBOR_TOP_RIGHT_CORNER`"]
    pub const TOP_RIGHT_CORNER: CellNeighbor = CellNeighbor {
        ord: 15i32
    };
    
}
impl std::fmt::Debug for CellNeighbor {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CellNeighbor") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CellNeighbor {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 => Some(Self {
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
            Self::RIGHT_SIDE => "RIGHT_SIDE", Self::RIGHT_CORNER => "RIGHT_CORNER", Self::BOTTOM_RIGHT_SIDE => "BOTTOM_RIGHT_SIDE", Self::BOTTOM_RIGHT_CORNER => "BOTTOM_RIGHT_CORNER", Self::BOTTOM_SIDE => "BOTTOM_SIDE", Self::BOTTOM_CORNER => "BOTTOM_CORNER", Self::BOTTOM_LEFT_SIDE => "BOTTOM_LEFT_SIDE", Self::BOTTOM_LEFT_CORNER => "BOTTOM_LEFT_CORNER", Self::LEFT_SIDE => "LEFT_SIDE", Self::LEFT_CORNER => "LEFT_CORNER", Self::TOP_LEFT_SIDE => "TOP_LEFT_SIDE", Self::TOP_LEFT_CORNER => "TOP_LEFT_CORNER", Self::TOP_SIDE => "TOP_SIDE", Self::TOP_CORNER => "TOP_CORNER", Self::TOP_RIGHT_SIDE => "TOP_RIGHT_SIDE", Self::TOP_RIGHT_CORNER => "TOP_RIGHT_CORNER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CellNeighbor::RIGHT_SIDE, CellNeighbor::RIGHT_CORNER, CellNeighbor::BOTTOM_RIGHT_SIDE, CellNeighbor::BOTTOM_RIGHT_CORNER, CellNeighbor::BOTTOM_SIDE, CellNeighbor::BOTTOM_CORNER, CellNeighbor::BOTTOM_LEFT_SIDE, CellNeighbor::BOTTOM_LEFT_CORNER, CellNeighbor::LEFT_SIDE, CellNeighbor::LEFT_CORNER, CellNeighbor::TOP_LEFT_SIDE, CellNeighbor::TOP_LEFT_CORNER, CellNeighbor::TOP_SIDE, CellNeighbor::TOP_CORNER, CellNeighbor::TOP_RIGHT_SIDE, CellNeighbor::TOP_RIGHT_CORNER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CellNeighbor >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RIGHT_SIDE", "CELL_NEIGHBOR_RIGHT_SIDE", CellNeighbor::RIGHT_SIDE), crate::meta::inspect::EnumConstant::new("RIGHT_CORNER", "CELL_NEIGHBOR_RIGHT_CORNER", CellNeighbor::RIGHT_CORNER), crate::meta::inspect::EnumConstant::new("BOTTOM_RIGHT_SIDE", "CELL_NEIGHBOR_BOTTOM_RIGHT_SIDE", CellNeighbor::BOTTOM_RIGHT_SIDE), crate::meta::inspect::EnumConstant::new("BOTTOM_RIGHT_CORNER", "CELL_NEIGHBOR_BOTTOM_RIGHT_CORNER", CellNeighbor::BOTTOM_RIGHT_CORNER), crate::meta::inspect::EnumConstant::new("BOTTOM_SIDE", "CELL_NEIGHBOR_BOTTOM_SIDE", CellNeighbor::BOTTOM_SIDE), crate::meta::inspect::EnumConstant::new("BOTTOM_CORNER", "CELL_NEIGHBOR_BOTTOM_CORNER", CellNeighbor::BOTTOM_CORNER), crate::meta::inspect::EnumConstant::new("BOTTOM_LEFT_SIDE", "CELL_NEIGHBOR_BOTTOM_LEFT_SIDE", CellNeighbor::BOTTOM_LEFT_SIDE), crate::meta::inspect::EnumConstant::new("BOTTOM_LEFT_CORNER", "CELL_NEIGHBOR_BOTTOM_LEFT_CORNER", CellNeighbor::BOTTOM_LEFT_CORNER), crate::meta::inspect::EnumConstant::new("LEFT_SIDE", "CELL_NEIGHBOR_LEFT_SIDE", CellNeighbor::LEFT_SIDE), crate::meta::inspect::EnumConstant::new("LEFT_CORNER", "CELL_NEIGHBOR_LEFT_CORNER", CellNeighbor::LEFT_CORNER), crate::meta::inspect::EnumConstant::new("TOP_LEFT_SIDE", "CELL_NEIGHBOR_TOP_LEFT_SIDE", CellNeighbor::TOP_LEFT_SIDE), crate::meta::inspect::EnumConstant::new("TOP_LEFT_CORNER", "CELL_NEIGHBOR_TOP_LEFT_CORNER", CellNeighbor::TOP_LEFT_CORNER), crate::meta::inspect::EnumConstant::new("TOP_SIDE", "CELL_NEIGHBOR_TOP_SIDE", CellNeighbor::TOP_SIDE), crate::meta::inspect::EnumConstant::new("TOP_CORNER", "CELL_NEIGHBOR_TOP_CORNER", CellNeighbor::TOP_CORNER), crate::meta::inspect::EnumConstant::new("TOP_RIGHT_SIDE", "CELL_NEIGHBOR_TOP_RIGHT_SIDE", CellNeighbor::TOP_RIGHT_SIDE), crate::meta::inspect::EnumConstant::new("TOP_RIGHT_CORNER", "CELL_NEIGHBOR_TOP_RIGHT_CORNER", CellNeighbor::TOP_RIGHT_CORNER)]
        }
    }
}
impl crate::meta::GodotConvert for CellNeighbor {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CellNeighbor {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CellNeighbor {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TerrainMode {
    ord: i32
}
impl TerrainMode {
    #[doc(alias = "TERRAIN_MODE_MATCH_CORNERS_AND_SIDES")]
    #[doc = "Godot enumerator name: `TERRAIN_MODE_MATCH_CORNERS_AND_SIDES`"]
    pub const CORNERS_AND_SIDES: TerrainMode = TerrainMode {
        ord: 0i32
    };
    #[doc(alias = "TERRAIN_MODE_MATCH_CORNERS")]
    #[doc = "Godot enumerator name: `TERRAIN_MODE_MATCH_CORNERS`"]
    pub const CORNERS: TerrainMode = TerrainMode {
        ord: 1i32
    };
    #[doc(alias = "TERRAIN_MODE_MATCH_SIDES")]
    #[doc = "Godot enumerator name: `TERRAIN_MODE_MATCH_SIDES`"]
    pub const SIDES: TerrainMode = TerrainMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TerrainMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TerrainMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TerrainMode {
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
            Self::CORNERS_AND_SIDES => "CORNERS_AND_SIDES", Self::CORNERS => "CORNERS", Self::SIDES => "SIDES", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TerrainMode::CORNERS_AND_SIDES, TerrainMode::CORNERS, TerrainMode::SIDES]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TerrainMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CORNERS_AND_SIDES", "TERRAIN_MODE_MATCH_CORNERS_AND_SIDES", TerrainMode::CORNERS_AND_SIDES), crate::meta::inspect::EnumConstant::new("CORNERS", "TERRAIN_MODE_MATCH_CORNERS", TerrainMode::CORNERS), crate::meta::inspect::EnumConstant::new("SIDES", "TERRAIN_MODE_MATCH_SIDES", TerrainMode::SIDES)]
        }
    }
}
impl crate::meta::GodotConvert for TerrainMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TerrainMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TerrainMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TileSet;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for TileSet {
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