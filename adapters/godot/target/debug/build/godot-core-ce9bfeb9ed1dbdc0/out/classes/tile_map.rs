#![doc = "Sidecar module for class [`TileMap`][crate::classes::TileMap].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileMap` enums](https://docs.godotengine.org/en/stable/classes/class_tilemap.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileMap.`\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`tile_map`][crate::classes::tile_map]: sidecar module with related enum/flag types\n* [`ITileMap`][crate::classes::ITileMap]: virtual methods\n* [`SignalsOfTileMap`][crate::classes::tile_map::SignalsOfTileMap]: signal collection\n\n\nSee also [Godot docs for `TileMap`](https://docs.godotengine.org/en/stable/classes/class_tilemap.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`TileMap::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileMap {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TileMap`][crate::classes::TileMap].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode2D`][crate::classes::INode2D] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `TileMap` methods](https://docs.godotengine.org/en/stable/classes/class_tilemap.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileMap: crate::obj::GodotClass < Base = TileMap > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn use_tile_data_runtime_update(&mut self, layer: i32, coords: Vector2i,) -> bool {
            unimplemented !()
        }
        fn tile_data_runtime_update(&mut self, layer: i32, coords: Vector2i, tile_data: Option < Gd < crate::classes::TileData > >,) {
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
    impl TileMap {
        pub fn set_navigation_map(&mut self, layer: i32, map: Rid,) {
            type CallRet = ();
            type CallParams = (i32, Rid,);
            let args = (layer, map,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9714usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_navigation_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_navigation_map(&self, layer: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9715usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_navigation_map", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn force_update_full(&mut self, layer: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9716usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "force_update", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::force_update_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn force_update(&mut self,) {
            self.force_update_ex() . done()
        }
        #[inline]
        pub fn force_update_ex < 'a > (&'a mut self,) -> ExForceUpdate < 'a > {
            ExForceUpdate::new(self,)
        }
        pub fn set_tileset(&mut self, tileset: impl AsArg < Option < Gd < crate::classes::TileSet >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TileSet > > >,);
            let args = (tileset.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9717usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_tileset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tileset(&self,) -> Option < Gd < crate::classes::TileSet > > {
            type CallRet = Option < Gd < crate::classes::TileSet > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9718usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_tileset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rendering_quadrant_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9719usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_rendering_quadrant_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rendering_quadrant_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9720usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_rendering_quadrant_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_layers_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9721usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_layers_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_layer(&mut self, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9722usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "add_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn move_layer(&mut self, layer: i32, to_position: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer, to_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9723usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "move_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_layer(&mut self, layer: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9724usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "remove_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_layer_name(&mut self, layer: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (layer, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9725usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_layer_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_layer_name(&self, layer: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9726usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_layer_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_layer_enabled(&mut self, layer: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9727usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_layer_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_layer_enabled(&self, layer: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9728usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "is_layer_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_layer_modulate(&mut self, layer: i32, modulate: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (layer, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9729usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_layer_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_layer_modulate(&self, layer: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9730usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_layer_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_layer_y_sort_enabled(&mut self, layer: i32, y_sort_enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer, y_sort_enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9731usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_layer_y_sort_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_layer_y_sort_enabled(&self, layer: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9732usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "is_layer_y_sort_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_layer_y_sort_origin(&mut self, layer: i32, y_sort_origin: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer, y_sort_origin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9733usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_layer_y_sort_origin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_layer_y_sort_origin(&self, layer: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9734usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_layer_y_sort_origin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_layer_z_index(&mut self, layer: i32, z_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (layer, z_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9735usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_layer_z_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_layer_z_index(&self, layer: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9736usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_layer_z_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_layer_navigation_enabled(&mut self, layer: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer, enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9737usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_layer_navigation_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_layer_navigation_enabled(&self, layer: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9738usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "is_layer_navigation_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_layer_navigation_map(&mut self, layer: i32, map: Rid,) {
            type CallRet = ();
            type CallParams = (i32, Rid,);
            let args = (layer, map,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9739usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_layer_navigation_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_layer_navigation_map(&self, layer: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9740usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_layer_navigation_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_animatable(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9741usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_collision_animatable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_collision_animatable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9742usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "is_collision_animatable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_visibility_mode(&mut self, collision_visibility_mode: crate::classes::tile_map::VisibilityMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::tile_map::VisibilityMode,);
            let args = (collision_visibility_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9743usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_collision_visibility_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_visibility_mode(&self,) -> crate::classes::tile_map::VisibilityMode {
            type CallRet = crate::classes::tile_map::VisibilityMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9744usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_collision_visibility_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_navigation_visibility_mode(&mut self, navigation_visibility_mode: crate::classes::tile_map::VisibilityMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::tile_map::VisibilityMode,);
            let args = (navigation_visibility_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9745usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_navigation_visibility_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_navigation_visibility_mode(&self,) -> crate::classes::tile_map::VisibilityMode {
            type CallRet = crate::classes::tile_map::VisibilityMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9746usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_navigation_visibility_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn set_cell_full(&mut self, layer: i32, coords: Vector2i, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i, i32, Vector2i, i32,);
            let args = (layer, coords, source_id, atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9747usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_cell", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_cell_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_cell(&mut self, layer: i32, coords: Vector2i,) {
            self.set_cell_ex(layer, coords,) . done()
        }
        #[inline]
        pub fn set_cell_ex < 'a > (&'a mut self, layer: i32, coords: Vector2i,) -> ExSetCell < 'a > {
            ExSetCell::new(self, layer, coords,)
        }
        pub fn erase_cell(&mut self, layer: i32, coords: Vector2i,) {
            type CallRet = ();
            type CallParams = (i32, Vector2i,);
            let args = (layer, coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9748usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "erase_cell", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_cell_source_id_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, Vector2i, bool,);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9749usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_cell_source_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_cell_source_id_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_cell_source_id(&self, layer: i32, coords: Vector2i,) -> i32 {
            self.get_cell_source_id_ex(layer, coords,) . done()
        }
        #[inline]
        pub fn get_cell_source_id_ex < 'a > (&'a self, layer: i32, coords: Vector2i,) -> ExGetCellSourceId < 'a > {
            ExGetCellSourceId::new(self, layer, coords,)
        }
        pub(crate) fn get_cell_atlas_coords_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (i32, Vector2i, bool,);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9750usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_cell_atlas_coords", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_cell_atlas_coords_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_cell_atlas_coords(&self, layer: i32, coords: Vector2i,) -> Vector2i {
            self.get_cell_atlas_coords_ex(layer, coords,) . done()
        }
        #[inline]
        pub fn get_cell_atlas_coords_ex < 'a > (&'a self, layer: i32, coords: Vector2i,) -> ExGetCellAtlasCoords < 'a > {
            ExGetCellAtlasCoords::new(self, layer, coords,)
        }
        pub(crate) fn get_cell_alternative_tile_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32, Vector2i, bool,);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9751usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_cell_alternative_tile", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_cell_alternative_tile_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_cell_alternative_tile(&self, layer: i32, coords: Vector2i,) -> i32 {
            self.get_cell_alternative_tile_ex(layer, coords,) . done()
        }
        #[inline]
        pub fn get_cell_alternative_tile_ex < 'a > (&'a self, layer: i32, coords: Vector2i,) -> ExGetCellAlternativeTile < 'a > {
            ExGetCellAlternativeTile::new(self, layer, coords,)
        }
        pub(crate) fn get_cell_tile_data_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> Option < Gd < crate::classes::TileData > > {
            type CallRet = Option < Gd < crate::classes::TileData > >;
            type CallParams = (i32, Vector2i, bool,);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9752usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_cell_tile_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_cell_tile_data_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_cell_tile_data(&self, layer: i32, coords: Vector2i,) -> Option < Gd < crate::classes::TileData > > {
            self.get_cell_tile_data_ex(layer, coords,) . done()
        }
        #[inline]
        pub fn get_cell_tile_data_ex < 'a > (&'a self, layer: i32, coords: Vector2i,) -> ExGetCellTileData < 'a > {
            ExGetCellTileData::new(self, layer, coords,)
        }
        pub(crate) fn is_cell_flipped_h_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, Vector2i, bool,);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9753usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "is_cell_flipped_h", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_cell_flipped_h_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_cell_flipped_h(&self, layer: i32, coords: Vector2i,) -> bool {
            self.is_cell_flipped_h_ex(layer, coords,) . done()
        }
        #[inline]
        pub fn is_cell_flipped_h_ex < 'a > (&'a self, layer: i32, coords: Vector2i,) -> ExIsCellFlippedH < 'a > {
            ExIsCellFlippedH::new(self, layer, coords,)
        }
        pub(crate) fn is_cell_flipped_v_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, Vector2i, bool,);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9754usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "is_cell_flipped_v", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_cell_flipped_v_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_cell_flipped_v(&self, layer: i32, coords: Vector2i,) -> bool {
            self.is_cell_flipped_v_ex(layer, coords,) . done()
        }
        #[inline]
        pub fn is_cell_flipped_v_ex < 'a > (&'a self, layer: i32, coords: Vector2i,) -> ExIsCellFlippedV < 'a > {
            ExIsCellFlippedV::new(self, layer, coords,)
        }
        pub(crate) fn is_cell_transposed_full(&self, layer: i32, coords: Vector2i, use_proxies: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, Vector2i, bool,);
            let args = (layer, coords, use_proxies,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9755usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "is_cell_transposed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::is_cell_transposed_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn is_cell_transposed(&self, layer: i32, coords: Vector2i,) -> bool {
            self.is_cell_transposed_ex(layer, coords,) . done()
        }
        #[inline]
        pub fn is_cell_transposed_ex < 'a > (&'a self, layer: i32, coords: Vector2i,) -> ExIsCellTransposed < 'a > {
            ExIsCellTransposed::new(self, layer, coords,)
        }
        pub fn get_coords_for_body_rid(&mut self, body: Rid,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9756usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_coords_for_body_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_layer_for_body_rid(&mut self, body: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (body,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9757usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_layer_for_body_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pattern(&mut self, layer: i32, coords_array: &Array < Vector2i >,) -> Option < Gd < crate::classes::TileMapPattern > > {
            type CallRet = Option < Gd < crate::classes::TileMapPattern > >;
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Array < Vector2i > >,);
            let args = (layer, RefArg::new(coords_array),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9758usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_pattern", Some(self.__validated_obj()), args,)
            }
        }
        pub fn map_pattern(&mut self, position_in_tilemap: Vector2i, coords_in_pattern: Vector2i, pattern: impl AsArg < Option < Gd < crate::classes::TileMapPattern >> >,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams < 'a0, > = (Vector2i, Vector2i, CowArg < 'a0, Option < Gd < crate::classes::TileMapPattern > > >,);
            let args = (position_in_tilemap, coords_in_pattern, pattern.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9759usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "map_pattern", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pattern(&mut self, layer: i32, position: Vector2i, pattern: impl AsArg < Option < Gd < crate::classes::TileMapPattern >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, Vector2i, CowArg < 'a0, Option < Gd < crate::classes::TileMapPattern > > >,);
            let args = (layer, position, pattern.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9760usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_pattern", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn set_cells_terrain_connect_full(&mut self, layer: i32, cells: RefArg < Array < Vector2i > >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Array < Vector2i > >, i32, i32, bool,);
            let args = (layer, cells, terrain_set, terrain, ignore_empty_terrains,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9761usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_cells_terrain_connect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_cells_terrain_connect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_cells_terrain_connect(&mut self, layer: i32, cells: &Array < Vector2i >, terrain_set: i32, terrain: i32,) {
            self.set_cells_terrain_connect_ex(layer, cells, terrain_set, terrain,) . done()
        }
        #[inline]
        pub fn set_cells_terrain_connect_ex < 'a > (&'a mut self, layer: i32, cells: &'a Array < Vector2i >, terrain_set: i32, terrain: i32,) -> ExSetCellsTerrainConnect < 'a > {
            ExSetCellsTerrainConnect::new(self, layer, cells, terrain_set, terrain,)
        }
        pub(crate) fn set_cells_terrain_path_full(&mut self, layer: i32, path: RefArg < Array < Vector2i > >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Array < Vector2i > >, i32, i32, bool,);
            let args = (layer, path, terrain_set, terrain, ignore_empty_terrains,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9762usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "set_cells_terrain_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_cells_terrain_path_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_cells_terrain_path(&mut self, layer: i32, path: &Array < Vector2i >, terrain_set: i32, terrain: i32,) {
            self.set_cells_terrain_path_ex(layer, path, terrain_set, terrain,) . done()
        }
        #[inline]
        pub fn set_cells_terrain_path_ex < 'a > (&'a mut self, layer: i32, path: &'a Array < Vector2i >, terrain_set: i32, terrain: i32,) -> ExSetCellsTerrainPath < 'a > {
            ExSetCellsTerrainPath::new(self, layer, path, terrain_set, terrain,)
        }
        pub fn fix_invalid_tiles(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9763usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "fix_invalid_tiles", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_layer(&mut self, layer: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9764usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "clear_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9765usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn update_internals(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9766usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "update_internals", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn notify_runtime_tile_data_update_full(&mut self, layer: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9767usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "notify_runtime_tile_data_update", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::notify_runtime_tile_data_update_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn notify_runtime_tile_data_update(&mut self,) {
            self.notify_runtime_tile_data_update_ex() . done()
        }
        #[inline]
        pub fn notify_runtime_tile_data_update_ex < 'a > (&'a mut self,) -> ExNotifyRuntimeTileDataUpdate < 'a > {
            ExNotifyRuntimeTileDataUpdate::new(self,)
        }
        pub fn get_surrounding_cells(&mut self, coords: Vector2i,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (Vector2i,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9768usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_surrounding_cells", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_used_cells(&self, layer: i32,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (i32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9769usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_used_cells", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_used_cells_by_id_full(&self, layer: i32, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = (i32, i32, Vector2i, i32,);
            let args = (layer, source_id, atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9770usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_used_cells_by_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_used_cells_by_id_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_used_cells_by_id(&self, layer: i32,) -> Array < Vector2i > {
            self.get_used_cells_by_id_ex(layer,) . done()
        }
        #[inline]
        pub fn get_used_cells_by_id_ex < 'a > (&'a self, layer: i32,) -> ExGetUsedCellsById < 'a > {
            ExGetUsedCellsById::new(self, layer,)
        }
        pub fn get_used_rect(&self,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9771usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_used_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn map_to_local(&self, map_position: Vector2i,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (Vector2i,);
            let args = (map_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9772usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "map_to_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn local_to_map(&self, local_position: Vector2,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Vector2,);
            let args = (local_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9773usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "local_to_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_neighbor_cell(&self, coords: Vector2i, neighbor: crate::classes::tile_set::CellNeighbor,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Vector2i, crate::classes::tile_set::CellNeighbor,);
            let args = (coords, neighbor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9774usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMap", "get_neighbor_cell", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TileMap {
        type Base = crate::classes::Node2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TileMap"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileMap {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for TileMap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for TileMap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for TileMap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TileMap {
        
    }
    impl crate::obj::cap::GodotDefault for TileMap {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileMap {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileMap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TileMap`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TileMap__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TileMap > for $Class {
                
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
#[doc = "Default-param extender for [`TileMap::force_update_ex`][super::TileMap::force_update_ex]."]
#[must_use]
pub struct ExForceUpdate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileMap, layer: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExForceUpdate < 'a > {
    fn new(surround_object: &'a mut re_export::TileMap,) -> Self {
        let layer = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer,
        }
    }
    #[inline]
    pub fn layer(self, layer: i32) -> Self {
        Self {
            layer: layer, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, layer,
        }
        = self;
        re_export::TileMap::force_update_full(surround_object, layer,)
    }
}
#[doc = "Default-param extender for [`TileMap::set_cell_ex`][super::TileMap::set_cell_ex]."]
#[must_use]
pub struct ExSetCell < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileMap, layer: i32, coords: Vector2i, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCell < 'a > {
    fn new(surround_object: &'a mut re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        let source_id = - 1i32;
        let atlas_coords = Vector2i::new(- 1 as _, - 1 as _);
        let alternative_tile = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, coords: coords, source_id: source_id, atlas_coords: atlas_coords, alternative_tile: alternative_tile,
        }
    }
    #[inline]
    pub fn source_id(self, source_id: i32) -> Self {
        Self {
            source_id: source_id, .. self
        }
    }
    #[inline]
    pub fn atlas_coords(self, atlas_coords: Vector2i) -> Self {
        Self {
            atlas_coords: atlas_coords, .. self
        }
    }
    #[inline]
    pub fn alternative_tile(self, alternative_tile: i32) -> Self {
        Self {
            alternative_tile: alternative_tile, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, layer, coords, source_id, atlas_coords, alternative_tile,
        }
        = self;
        re_export::TileMap::set_cell_full(surround_object, layer, coords, source_id, atlas_coords, alternative_tile,)
    }
}
#[doc = "Default-param extender for [`TileMap::get_cell_source_id_ex`][super::TileMap::get_cell_source_id_ex]."]
#[must_use]
pub struct ExGetCellSourceId < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCellSourceId < 'a > {
    fn new(surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        let use_proxies = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, coords: coords, use_proxies: use_proxies,
        }
    }
    #[inline]
    pub fn use_proxies(self, use_proxies: bool) -> Self {
        Self {
            use_proxies: use_proxies, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, layer, coords, use_proxies,
        }
        = self;
        re_export::TileMap::get_cell_source_id_full(surround_object, layer, coords, use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::get_cell_atlas_coords_ex`][super::TileMap::get_cell_atlas_coords_ex]."]
#[must_use]
pub struct ExGetCellAtlasCoords < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCellAtlasCoords < 'a > {
    fn new(surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        let use_proxies = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, coords: coords, use_proxies: use_proxies,
        }
    }
    #[inline]
    pub fn use_proxies(self, use_proxies: bool) -> Self {
        Self {
            use_proxies: use_proxies, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Vector2i {
        let Self {
            _phantom, surround_object, layer, coords, use_proxies,
        }
        = self;
        re_export::TileMap::get_cell_atlas_coords_full(surround_object, layer, coords, use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::get_cell_alternative_tile_ex`][super::TileMap::get_cell_alternative_tile_ex]."]
#[must_use]
pub struct ExGetCellAlternativeTile < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCellAlternativeTile < 'a > {
    fn new(surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        let use_proxies = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, coords: coords, use_proxies: use_proxies,
        }
    }
    #[inline]
    pub fn use_proxies(self, use_proxies: bool) -> Self {
        Self {
            use_proxies: use_proxies, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, layer, coords, use_proxies,
        }
        = self;
        re_export::TileMap::get_cell_alternative_tile_full(surround_object, layer, coords, use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::get_cell_tile_data_ex`][super::TileMap::get_cell_tile_data_ex]."]
#[must_use]
pub struct ExGetCellTileData < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetCellTileData < 'a > {
    fn new(surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        let use_proxies = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, coords: coords, use_proxies: use_proxies,
        }
    }
    #[inline]
    pub fn use_proxies(self, use_proxies: bool) -> Self {
        Self {
            use_proxies: use_proxies, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TileData > > {
        let Self {
            _phantom, surround_object, layer, coords, use_proxies,
        }
        = self;
        re_export::TileMap::get_cell_tile_data_full(surround_object, layer, coords, use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::is_cell_flipped_h_ex`][super::TileMap::is_cell_flipped_h_ex]."]
#[must_use]
pub struct ExIsCellFlippedH < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsCellFlippedH < 'a > {
    fn new(surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        let use_proxies = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, coords: coords, use_proxies: use_proxies,
        }
    }
    #[inline]
    pub fn use_proxies(self, use_proxies: bool) -> Self {
        Self {
            use_proxies: use_proxies, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, layer, coords, use_proxies,
        }
        = self;
        re_export::TileMap::is_cell_flipped_h_full(surround_object, layer, coords, use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::is_cell_flipped_v_ex`][super::TileMap::is_cell_flipped_v_ex]."]
#[must_use]
pub struct ExIsCellFlippedV < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsCellFlippedV < 'a > {
    fn new(surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        let use_proxies = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, coords: coords, use_proxies: use_proxies,
        }
    }
    #[inline]
    pub fn use_proxies(self, use_proxies: bool) -> Self {
        Self {
            use_proxies: use_proxies, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, layer, coords, use_proxies,
        }
        = self;
        re_export::TileMap::is_cell_flipped_v_full(surround_object, layer, coords, use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::is_cell_transposed_ex`][super::TileMap::is_cell_transposed_ex]."]
#[must_use]
pub struct ExIsCellTransposed < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i, use_proxies: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIsCellTransposed < 'a > {
    fn new(surround_object: &'a re_export::TileMap, layer: i32, coords: Vector2i,) -> Self {
        let use_proxies = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, coords: coords, use_proxies: use_proxies,
        }
    }
    #[inline]
    pub fn use_proxies(self, use_proxies: bool) -> Self {
        Self {
            use_proxies: use_proxies, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, layer, coords, use_proxies,
        }
        = self;
        re_export::TileMap::is_cell_transposed_full(surround_object, layer, coords, use_proxies,)
    }
}
#[doc = "Default-param extender for [`TileMap::set_cells_terrain_connect_ex`][super::TileMap::set_cells_terrain_connect_ex]."]
#[must_use]
pub struct ExSetCellsTerrainConnect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileMap, layer: i32, cells: CowArg < 'a, Array < Vector2i > >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCellsTerrainConnect < 'a > {
    fn new(surround_object: &'a mut re_export::TileMap, layer: i32, cells: &'a Array < Vector2i >, terrain_set: i32, terrain: i32,) -> Self {
        let ignore_empty_terrains = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, cells: CowArg::Borrowed(cells), terrain_set: terrain_set, terrain: terrain, ignore_empty_terrains: ignore_empty_terrains,
        }
    }
    #[inline]
    pub fn ignore_empty_terrains(self, ignore_empty_terrains: bool) -> Self {
        Self {
            ignore_empty_terrains: ignore_empty_terrains, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, layer, cells, terrain_set, terrain, ignore_empty_terrains,
        }
        = self;
        re_export::TileMap::set_cells_terrain_connect_full(surround_object, layer, cells.cow_as_arg(), terrain_set, terrain, ignore_empty_terrains,)
    }
}
#[doc = "Default-param extender for [`TileMap::set_cells_terrain_path_ex`][super::TileMap::set_cells_terrain_path_ex]."]
#[must_use]
pub struct ExSetCellsTerrainPath < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileMap, layer: i32, path: CowArg < 'a, Array < Vector2i > >, terrain_set: i32, terrain: i32, ignore_empty_terrains: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCellsTerrainPath < 'a > {
    fn new(surround_object: &'a mut re_export::TileMap, layer: i32, path: &'a Array < Vector2i >, terrain_set: i32, terrain: i32,) -> Self {
        let ignore_empty_terrains = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, path: CowArg::Borrowed(path), terrain_set: terrain_set, terrain: terrain, ignore_empty_terrains: ignore_empty_terrains,
        }
    }
    #[inline]
    pub fn ignore_empty_terrains(self, ignore_empty_terrains: bool) -> Self {
        Self {
            ignore_empty_terrains: ignore_empty_terrains, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, layer, path, terrain_set, terrain, ignore_empty_terrains,
        }
        = self;
        re_export::TileMap::set_cells_terrain_path_full(surround_object, layer, path.cow_as_arg(), terrain_set, terrain, ignore_empty_terrains,)
    }
}
#[doc = "Default-param extender for [`TileMap::notify_runtime_tile_data_update_ex`][super::TileMap::notify_runtime_tile_data_update_ex]."]
#[must_use]
pub struct ExNotifyRuntimeTileDataUpdate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileMap, layer: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExNotifyRuntimeTileDataUpdate < 'a > {
    fn new(surround_object: &'a mut re_export::TileMap,) -> Self {
        let layer = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer,
        }
    }
    #[inline]
    pub fn layer(self, layer: i32) -> Self {
        Self {
            layer: layer, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, layer,
        }
        = self;
        re_export::TileMap::notify_runtime_tile_data_update_full(surround_object, layer,)
    }
}
#[doc = "Default-param extender for [`TileMap::get_used_cells_by_id_ex`][super::TileMap::get_used_cells_by_id_ex]."]
#[must_use]
pub struct ExGetUsedCellsById < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::TileMap, layer: i32, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetUsedCellsById < 'a > {
    fn new(surround_object: &'a re_export::TileMap, layer: i32,) -> Self {
        let source_id = - 1i32;
        let atlas_coords = Vector2i::new(- 1 as _, - 1 as _);
        let alternative_tile = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, source_id: source_id, atlas_coords: atlas_coords, alternative_tile: alternative_tile,
        }
    }
    #[inline]
    pub fn source_id(self, source_id: i32) -> Self {
        Self {
            source_id: source_id, .. self
        }
    }
    #[inline]
    pub fn atlas_coords(self, atlas_coords: Vector2i) -> Self {
        Self {
            atlas_coords: atlas_coords, .. self
        }
    }
    #[inline]
    pub fn alternative_tile(self, alternative_tile: i32) -> Self {
        Self {
            alternative_tile: alternative_tile, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Array < Vector2i > {
        let Self {
            _phantom, surround_object, layer, source_id, atlas_coords, alternative_tile,
        }
        = self;
        re_export::TileMap::get_used_cells_by_id_full(surround_object, layer, source_id, atlas_coords, alternative_tile,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VisibilityMode {
    ord: i32
}
impl VisibilityMode {
    #[doc(alias = "VISIBILITY_MODE_DEFAULT")]
    #[doc = "Godot enumerator name: `VISIBILITY_MODE_DEFAULT`"]
    pub const DEFAULT: VisibilityMode = VisibilityMode {
        ord: 0i32
    };
    #[doc(alias = "VISIBILITY_MODE_FORCE_HIDE")]
    #[doc = "Godot enumerator name: `VISIBILITY_MODE_FORCE_HIDE`"]
    pub const FORCE_HIDE: VisibilityMode = VisibilityMode {
        ord: 2i32
    };
    #[doc(alias = "VISIBILITY_MODE_FORCE_SHOW")]
    #[doc = "Godot enumerator name: `VISIBILITY_MODE_FORCE_SHOW`"]
    pub const FORCE_SHOW: VisibilityMode = VisibilityMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for VisibilityMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VisibilityMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VisibilityMode {
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
            Self::DEFAULT => "DEFAULT", Self::FORCE_HIDE => "FORCE_HIDE", Self::FORCE_SHOW => "FORCE_SHOW", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VisibilityMode::DEFAULT, VisibilityMode::FORCE_HIDE, VisibilityMode::FORCE_SHOW]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VisibilityMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "VISIBILITY_MODE_DEFAULT", VisibilityMode::DEFAULT), crate::meta::inspect::EnumConstant::new("FORCE_HIDE", "VISIBILITY_MODE_FORCE_HIDE", VisibilityMode::FORCE_HIDE), crate::meta::inspect::EnumConstant::new("FORCE_SHOW", "VISIBILITY_MODE_FORCE_SHOW", VisibilityMode::FORCE_SHOW)]
        }
    }
}
impl crate::meta::GodotConvert for VisibilityMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VisibilityMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VisibilityMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TileMap;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`TileMap`][crate::classes::TileMap] class."]
    pub struct SignalsOfTileMap < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfTileMap < 'c, C > {
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
    impl WithSignals for TileMap {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfTileMap < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfTileMap < 'c, C > {
        type Target = < < TileMap as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = TileMap;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfTileMap < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = TileMap;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}