#![doc = "Sidecar module for class [`GridMap`][crate::classes::GridMap].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GridMap` enums](https://docs.godotengine.org/en/stable/classes/class_gridmap.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GridMap.`\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`grid_map`][crate::classes::grid_map]: sidecar module with related enum/flag types\n* [`IGridMap`][crate::classes::IGridMap]: virtual methods\n* [`SignalsOfGridMap`][crate::classes::grid_map::SignalsOfGridMap]: signal collection\n\n\nSee also [Godot docs for `GridMap`](https://docs.godotengine.org/en/stable/classes/class_gridmap.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`GridMap::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GridMap {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GridMap`][crate::classes::GridMap].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GridMap` methods](https://docs.godotengine.org/en/stable/classes/class_gridmap.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGridMap: crate::obj::GodotClass < Base = GridMap > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GridMap {
        pub fn set_collision_layer(&mut self, layer: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_layer(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_mask(&mut self, mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_mask_value(&mut self, layer_number: i32, value: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_collision_mask_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_mask_value(&self, layer_number: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_collision_mask_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_layer_value(&mut self, layer_number: i32, value: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_collision_layer_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_layer_value(&self, layer_number: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_collision_layer_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_priority(&mut self, priority: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_collision_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_priority(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_collision_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_physics_material(&mut self, material: impl AsArg < Option < Gd < crate::classes::PhysicsMaterial >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::PhysicsMaterial > > >,);
            let args = (material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_physics_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_physics_material(&self,) -> Option < Gd < crate::classes::PhysicsMaterial > > {
            type CallRet = Option < Gd < crate::classes::PhysicsMaterial > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_physics_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bake_navigation(&mut self, bake_navigation: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (bake_navigation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_bake_navigation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_baking_navigation(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "is_baking_navigation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_navigation_map(&mut self, navigation_map: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (navigation_map,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_navigation_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_navigation_map(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_navigation_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mesh_library(&mut self, mesh_library: impl AsArg < Option < Gd < crate::classes::MeshLibrary >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::MeshLibrary > > >,);
            let args = (mesh_library.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_mesh_library", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mesh_library(&self,) -> Option < Gd < crate::classes::MeshLibrary > > {
            type CallRet = Option < Gd < crate::classes::MeshLibrary > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_mesh_library", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cell_size(&mut self, size: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_cell_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cell_size(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_cell_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cell_scale(&mut self, scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_cell_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cell_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_cell_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_octant_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_octant_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_octant_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_octant_size", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn set_cell_item_full(&mut self, position: Vector3i, item: i32, orientation: i32,) {
            type CallRet = ();
            type CallParams = (Vector3i, i32, i32,);
            let args = (position, item, orientation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_cell_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_cell_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_cell_item(&mut self, position: Vector3i, item: i32,) {
            self.set_cell_item_ex(position, item,) . done()
        }
        #[inline]
        pub fn set_cell_item_ex < 'a > (&'a mut self, position: Vector3i, item: i32,) -> ExSetCellItem < 'a > {
            ExSetCellItem::new(self, position, item,)
        }
        pub fn get_cell_item(&self, position: Vector3i,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector3i,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_cell_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cell_item_orientation(&self, position: Vector3i,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector3i,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_cell_item_orientation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cell_item_basis(&self, position: Vector3i,) -> Basis {
            type CallRet = Basis;
            type CallParams = (Vector3i,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_cell_item_basis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_basis_with_orthogonal_index(&self, index: i32,) -> Basis {
            type CallRet = Basis;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4148usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_basis_with_orthogonal_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_orthogonal_index_from_basis(&self, basis: Basis,) -> i32 {
            type CallRet = i32;
            type CallParams = (Basis,);
            let args = (basis,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4149usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_orthogonal_index_from_basis", Some(self.__validated_obj()), args,)
            }
        }
        pub fn local_to_map(&self, local_position: Vector3,) -> Vector3i {
            type CallRet = Vector3i;
            type CallParams = (Vector3,);
            let args = (local_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "local_to_map", Some(self.__validated_obj()), args,)
            }
        }
        pub fn map_to_local(&self, map_position: Vector3i,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = (Vector3i,);
            let args = (map_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "map_to_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn resource_changed(&mut self, resource: impl AsArg < Option < Gd < crate::classes::Resource >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Resource > > >,);
            let args = (resource.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "resource_changed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_center_x(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_center_x", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_x(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_center_x", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_center_y(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_center_y", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_y(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_center_y", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_center_z(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "set_center_z", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_center_z(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_center_z", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_used_cells(&self,) -> Array < Vector3i > {
            type CallRet = Array < Vector3i >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_used_cells", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_used_cells_by_item(&self, item: i32,) -> Array < Vector3i > {
            type CallRet = Array < Vector3i >;
            type CallParams = (i32,);
            let args = (item,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_used_cells_by_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_meshes(&self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_meshes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bake_meshes(&mut self,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_bake_meshes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bake_mesh_instance(&mut self, idx: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "get_bake_mesh_instance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_baked_meshes(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "clear_baked_meshes", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn make_baked_meshes_full(&mut self, gen_lightmap_uv: bool, lightmap_uv_texel_size: f32,) {
            type CallRet = ();
            type CallParams = (bool, f32,);
            let args = (gen_lightmap_uv, lightmap_uv_texel_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GridMap", "make_baked_meshes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::make_baked_meshes_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn make_baked_meshes(&mut self,) {
            self.make_baked_meshes_ex() . done()
        }
        #[inline]
        pub fn make_baked_meshes_ex < 'a > (&'a mut self,) -> ExMakeBakedMeshes < 'a > {
            ExMakeBakedMeshes::new(self,)
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
        pub const INVALID_CELL_ITEM: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for GridMap {
        type Base = crate::classes::Node3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GridMap"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GridMap {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for GridMap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for GridMap {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GridMap {
        
    }
    impl crate::obj::cap::GodotDefault for GridMap {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GridMap {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GridMap {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GridMap`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GridMap__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GridMap > for $Class {
                
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
#[doc = "Default-param extender for [`GridMap::set_cell_item_ex`][super::GridMap::set_cell_item_ex]."]
#[must_use]
pub struct ExSetCellItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::GridMap, position: Vector3i, item: i32, orientation: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCellItem < 'a > {
    fn new(surround_object: &'a mut re_export::GridMap, position: Vector3i, item: i32,) -> Self {
        let orientation = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, item: item, orientation: orientation,
        }
    }
    #[inline]
    pub fn orientation(self, orientation: i32) -> Self {
        Self {
            orientation: orientation, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, item, orientation,
        }
        = self;
        re_export::GridMap::set_cell_item_full(surround_object, position, item, orientation,)
    }
}
#[doc = "Default-param extender for [`GridMap::make_baked_meshes_ex`][super::GridMap::make_baked_meshes_ex]."]
#[must_use]
pub struct ExMakeBakedMeshes < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::GridMap, gen_lightmap_uv: bool, lightmap_uv_texel_size: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMakeBakedMeshes < 'a > {
    fn new(surround_object: &'a mut re_export::GridMap,) -> Self {
        let gen_lightmap_uv = false;
        let lightmap_uv_texel_size = 0.1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, gen_lightmap_uv: gen_lightmap_uv, lightmap_uv_texel_size: lightmap_uv_texel_size,
        }
    }
    #[inline]
    pub fn gen_lightmap_uv(self, gen_lightmap_uv: bool) -> Self {
        Self {
            gen_lightmap_uv: gen_lightmap_uv, .. self
        }
    }
    #[inline]
    pub fn lightmap_uv_texel_size(self, lightmap_uv_texel_size: f32) -> Self {
        Self {
            lightmap_uv_texel_size: lightmap_uv_texel_size, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, gen_lightmap_uv, lightmap_uv_texel_size,
        }
        = self;
        re_export::GridMap::make_baked_meshes_full(surround_object, gen_lightmap_uv, lightmap_uv_texel_size,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GridMap;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`GridMap`][crate::classes::GridMap] class."]
    pub struct SignalsOfGridMap < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfGridMap < 'c, C > {
        #[doc = "Signature: `(cell_size: Vector3)`"]
        pub fn cell_size_changed(&mut self) -> SigCellSizeChanged < 'c, C > {
            SigCellSizeChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "cell_size_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn changed(&mut self) -> SigChanged < 'c, C > {
            SigChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "changed")
            }
        }
    }
    type TypedSigCellSizeChanged < 'c, C > = TypedSignal < 'c, C, (Vector3,) >;
    pub struct SigCellSizeChanged < 'c, C: WithSignals > {
        typed: TypedSigCellSizeChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCellSizeChanged < 'c, C > {
        pub fn emit(&mut self, cell_size: Vector3,) {
            self.typed.emit_tuple((cell_size,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCellSizeChanged < 'c, C > {
        type Target = TypedSigCellSizeChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCellSizeChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
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
    impl WithSignals for GridMap {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfGridMap < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfGridMap < 'c, C > {
        type Target = < < GridMap as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = GridMap;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfGridMap < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = GridMap;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}