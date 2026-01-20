#![doc = "Sidecar module for class [`GltfState`][crate::classes::GltfState].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFState` enums](https://docs.godotengine.org/en/stable/classes/class_gltfstate.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFState.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`IGltfState`][crate::classes::IGltfState]: virtual methods\n\n\nSee also [Godot docs for `GLTFState`](https://docs.godotengine.org/en/stable/classes/class_gltfstate.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GltfState::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfState {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GltfState`][crate::classes::GltfState].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GLTFState` methods](https://docs.godotengine.org/en/stable/classes/class_gltfstate.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfState: crate::obj::GodotClass < Base = GltfState > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GltfState {
        pub fn add_used_extension(&mut self, extension_name: impl AsArg < GString >, required: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (extension_name.into_arg(), required,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3754usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "add_used_extension", Some(self.__validated_obj()), args,)
            }
        }
        pub fn append_data_to_buffers(&mut self, data: &PackedByteArray, deduplication: bool,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >, bool,);
            let args = (RefArg::new(data), deduplication,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3755usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "append_data_to_buffers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn append_gltf_node(&mut self, gltf_node: impl AsArg < Option < Gd < crate::classes::GltfNode >> >, godot_scene_node: impl AsArg < Option < Gd < crate::classes::Node >> >, parent_node_index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::GltfNode > > >, CowArg < 'a1, Option < Gd < crate::classes::Node > > >, i32,);
            let args = (gltf_node.into_arg(), godot_scene_node.into_arg(), parent_node_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3756usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "append_gltf_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_json(&mut self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3757usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_json", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_json(&mut self, json: &VarDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >,);
            let args = (RefArg::new(json),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3758usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_json", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_major_version(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3759usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_major_version", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_major_version(&mut self, major_version: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (major_version,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3760usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_major_version", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_minor_version(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3761usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_minor_version", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_minor_version(&mut self, minor_version: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (minor_version,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3762usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_minor_version", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_copyright(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3763usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_copyright", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_copyright(&mut self, copyright: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (copyright.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3764usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_copyright", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_glb_data(&mut self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3765usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_glb_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_glb_data(&mut self, glb_data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(glb_data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3766usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_glb_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_use_named_skin_binds(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3767usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_use_named_skin_binds", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_use_named_skin_binds(&mut self, use_named_skin_binds: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (use_named_skin_binds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3768usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_use_named_skin_binds", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_nodes(&mut self,) -> Array < Gd < crate::classes::GltfNode > > {
            type CallRet = Array < Gd < crate::classes::GltfNode > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3769usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_nodes(&mut self, nodes: &Array < Gd < crate::classes::GltfNode > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfNode > > >,);
            let args = (RefArg::new(nodes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3770usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_buffers(&mut self,) -> Array < PackedByteArray > {
            type CallRet = Array < PackedByteArray >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3771usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_buffers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_buffers(&mut self, buffers: &Array < PackedByteArray >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < PackedByteArray > >,);
            let args = (RefArg::new(buffers),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3772usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_buffers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_buffer_views(&mut self,) -> Array < Gd < crate::classes::GltfBufferView > > {
            type CallRet = Array < Gd < crate::classes::GltfBufferView > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3773usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_buffer_views", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_buffer_views(&mut self, buffer_views: &Array < Gd < crate::classes::GltfBufferView > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfBufferView > > >,);
            let args = (RefArg::new(buffer_views),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3774usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_buffer_views", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_accessors(&mut self,) -> Array < Gd < crate::classes::GltfAccessor > > {
            type CallRet = Array < Gd < crate::classes::GltfAccessor > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3775usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_accessors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_accessors(&mut self, accessors: &Array < Gd < crate::classes::GltfAccessor > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfAccessor > > >,);
            let args = (RefArg::new(accessors),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3776usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_accessors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_meshes(&mut self,) -> Array < Gd < crate::classes::GltfMesh > > {
            type CallRet = Array < Gd < crate::classes::GltfMesh > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3777usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_meshes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_meshes(&mut self, meshes: &Array < Gd < crate::classes::GltfMesh > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfMesh > > >,);
            let args = (RefArg::new(meshes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3778usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_meshes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_animation_players_count(&mut self, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3779usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_animation_players_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_animation_player(&mut self, idx: i32,) -> Option < Gd < crate::classes::AnimationPlayer > > {
            type CallRet = Option < Gd < crate::classes::AnimationPlayer > >;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3780usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_animation_player", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_materials(&mut self,) -> Array < Gd < crate::classes::Material > > {
            type CallRet = Array < Gd < crate::classes::Material > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3781usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_materials", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_materials(&mut self, materials: &Array < Gd < crate::classes::Material > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::Material > > >,);
            let args = (RefArg::new(materials),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3782usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_materials", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scene_name(&mut self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3783usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_scene_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scene_name(&mut self, scene_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (scene_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3784usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_scene_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_base_path(&mut self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3785usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_base_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_base_path(&mut self, base_path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (base_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3786usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_base_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_filename(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3787usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_filename", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_filename(&mut self, filename: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (filename.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3788usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_filename", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_nodes(&mut self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3789usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_root_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_root_nodes(&mut self, root_nodes: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedInt32Array >,);
            let args = (RefArg::new(root_nodes),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3790usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_root_nodes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_textures(&mut self,) -> Array < Gd < crate::classes::GltfTexture > > {
            type CallRet = Array < Gd < crate::classes::GltfTexture > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3791usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_textures", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_textures(&mut self, textures: &Array < Gd < crate::classes::GltfTexture > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfTexture > > >,);
            let args = (RefArg::new(textures),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3792usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_textures", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_samplers(&mut self,) -> Array < Gd < crate::classes::GltfTextureSampler > > {
            type CallRet = Array < Gd < crate::classes::GltfTextureSampler > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3793usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_texture_samplers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_samplers(&mut self, texture_samplers: &Array < Gd < crate::classes::GltfTextureSampler > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfTextureSampler > > >,);
            let args = (RefArg::new(texture_samplers),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3794usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_texture_samplers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_images(&mut self,) -> Array < Gd < crate::classes::Texture2D > > {
            type CallRet = Array < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3795usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_images", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_images(&mut self, images: &Array < Gd < crate::classes::Texture2D > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::Texture2D > > >,);
            let args = (RefArg::new(images),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3796usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_images", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_skins(&mut self,) -> Array < Gd < crate::classes::GltfSkin > > {
            type CallRet = Array < Gd < crate::classes::GltfSkin > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3797usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_skins", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_skins(&mut self, skins: &Array < Gd < crate::classes::GltfSkin > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfSkin > > >,);
            let args = (RefArg::new(skins),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3798usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_skins", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cameras(&mut self,) -> Array < Gd < crate::classes::GltfCamera > > {
            type CallRet = Array < Gd < crate::classes::GltfCamera > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3799usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_cameras", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_cameras(&mut self, cameras: &Array < Gd < crate::classes::GltfCamera > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfCamera > > >,);
            let args = (RefArg::new(cameras),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3800usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_cameras", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lights(&mut self,) -> Array < Gd < crate::classes::GltfLight > > {
            type CallRet = Array < Gd < crate::classes::GltfLight > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3801usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_lights", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_lights(&mut self, lights: &Array < Gd < crate::classes::GltfLight > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfLight > > >,);
            let args = (RefArg::new(lights),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3802usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_lights", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_unique_names(&mut self,) -> Array < GString > {
            type CallRet = Array < GString >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3803usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_unique_names", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_unique_names(&mut self, unique_names: &Array < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < GString > >,);
            let args = (RefArg::new(unique_names),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3804usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_unique_names", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_unique_animation_names(&mut self,) -> Array < GString > {
            type CallRet = Array < GString >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3805usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_unique_animation_names", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_unique_animation_names(&mut self, unique_animation_names: &Array < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < GString > >,);
            let args = (RefArg::new(unique_animation_names),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3806usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_unique_animation_names", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_skeletons(&mut self,) -> Array < Gd < crate::classes::GltfSkeleton > > {
            type CallRet = Array < Gd < crate::classes::GltfSkeleton > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3807usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_skeletons", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_skeletons(&mut self, skeletons: &Array < Gd < crate::classes::GltfSkeleton > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfSkeleton > > >,);
            let args = (RefArg::new(skeletons),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3808usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_skeletons", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_create_animations(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3809usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_create_animations", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_create_animations(&mut self, create_animations: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (create_animations,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3810usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_create_animations", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_import_as_skeleton_bones(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3811usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_import_as_skeleton_bones", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_import_as_skeleton_bones(&mut self, import_as_skeleton_bones: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (import_as_skeleton_bones,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3812usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_import_as_skeleton_bones", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_animations(&mut self,) -> Array < Gd < crate::classes::GltfAnimation > > {
            type CallRet = Array < Gd < crate::classes::GltfAnimation > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3813usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_animations", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_animations(&mut self, animations: &Array < Gd < crate::classes::GltfAnimation > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::GltfAnimation > > >,);
            let args = (RefArg::new(animations),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3814usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_animations", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scene_node(&mut self, idx: i32,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3815usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_scene_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_index(&mut self, scene_node: impl AsArg < Option < Gd < crate::classes::Node >> >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (scene_node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3816usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_node_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_additional_data(&mut self, extension_name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (extension_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3817usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_additional_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_additional_data(&mut self, extension_name: impl AsArg < StringName >, additional_data: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (extension_name.into_arg(), RefArg::new(additional_data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3818usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_additional_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_handle_binary_image(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3819usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_handle_binary_image", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_handle_binary_image(&mut self, method: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (method,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3820usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_handle_binary_image", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bake_fps(&mut self, value: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3821usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "set_bake_fps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bake_fps(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3822usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfState", "get_bake_fps", Some(self.__validated_obj()), args,)
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
        pub const HANDLE_BINARY_DISCARD_TEXTURES: i32 = 0i32;
        pub const HANDLE_BINARY_EXTRACT_TEXTURES: i32 = 1i32;
        pub const HANDLE_BINARY_EMBED_AS_BASISU: i32 = 2i32;
        pub const HANDLE_BINARY_EMBED_AS_UNCOMPRESSED: i32 = 3i32;
        
    }
    impl crate::obj::GodotClass for GltfState {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GLTFState"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfState {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for GltfState {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GltfState {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GltfState {
        
    }
    impl crate::obj::cap::GodotDefault for GltfState {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfState {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfState {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GltfState`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GltfState__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfState > for $Class {
                
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GltfState;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for GltfState {
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