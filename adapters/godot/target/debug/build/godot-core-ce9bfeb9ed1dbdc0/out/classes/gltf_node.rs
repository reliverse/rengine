#![doc = "Sidecar module for class [`GltfNode`][crate::classes::GltfNode].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFNode` enums](https://docs.godotengine.org/en/stable/classes/class_gltfnode.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFNode.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`gltf_node`][crate::classes::gltf_node]: sidecar module with related enum/flag types\n* [`IGltfNode`][crate::classes::IGltfNode]: virtual methods\n\n\nSee also [Godot docs for `GLTFNode`](https://docs.godotengine.org/en/stable/classes/class_gltfnode.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GltfNode::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfNode {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GltfNode`][crate::classes::GltfNode].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GLTFNode` methods](https://docs.godotengine.org/en/stable/classes/class_gltfnode.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfNode: crate::obj::GodotClass < Base = GltfNode > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl GltfNode {
        pub fn get_original_name(&mut self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3623usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "get_original_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_original_name(&mut self, original_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (original_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3624usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "set_original_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_parent(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3625usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "get_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_parent(&mut self, parent: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (parent,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3626usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "set_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_height(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3627usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "get_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_height(&mut self, height: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3628usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "set_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_xform(&mut self,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3629usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "get_xform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_xform(&mut self, xform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Transform3D,);
            let args = (xform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3630usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "set_xform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mesh(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3631usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "get_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mesh(&mut self, mesh: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3632usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "set_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_camera(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3633usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "get_camera", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_camera(&mut self, camera: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (camera,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3634usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "set_camera", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_skin(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3635usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "get_skin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_skin(&mut self, skin: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (skin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3636usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "set_skin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_skeleton(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3637usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "get_skeleton", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_skeleton(&mut self, skeleton: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (skeleton,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3638usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "set_skeleton", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_position(&mut self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3639usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "get_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_position(&mut self, position: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3640usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "set_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rotation(&mut self,) -> Quaternion {
            type CallRet = Quaternion;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3641usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "get_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_rotation(&mut self, rotation: Quaternion,) {
            type CallRet = ();
            type CallParams = (Quaternion,);
            let args = (rotation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3642usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "set_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scale(&mut self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3643usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "get_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scale(&mut self, scale: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3644usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "set_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_children(&mut self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3645usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "get_children", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_children(&mut self, children: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedInt32Array >,);
            let args = (RefArg::new(children),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3646usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "set_children", Some(self.__validated_obj()), args,)
            }
        }
        pub fn append_child_index(&mut self, child_index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (child_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3647usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "append_child_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_light(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3648usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "get_light", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_light(&mut self, light: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (light,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3649usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "set_light", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_visible(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3650usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "get_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_visible(&mut self, visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3651usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "set_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_additional_data(&mut self, extension_name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (extension_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3652usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "get_additional_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_additional_data(&mut self, extension_name: impl AsArg < StringName >, additional_data: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (extension_name.into_arg(), RefArg::new(additional_data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3653usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "set_additional_data", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_scene_node_path_full(&mut self, gltf_state: CowArg < Option < Gd < crate::classes::GltfState > > >, handle_skeletons: bool,) -> NodePath {
            type CallRet = NodePath;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::GltfState > > >, bool,);
            let args = (gltf_state, handle_skeletons,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3654usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfNode", "get_scene_node_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_scene_node_path_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_scene_node_path(&mut self, gltf_state: impl AsArg < Option < Gd < crate::classes::GltfState >> >,) -> NodePath {
            self.get_scene_node_path_ex(gltf_state,) . done()
        }
        #[inline]
        pub fn get_scene_node_path_ex < 'a > (&'a mut self, gltf_state: impl AsArg < Option < Gd < crate::classes::GltfState >> > + 'a,) -> ExGetSceneNodePath < 'a > {
            ExGetSceneNodePath::new(self, gltf_state,)
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
    impl crate::obj::GodotClass for GltfNode {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GLTFNode"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfNode {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for GltfNode {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GltfNode {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GltfNode {
        
    }
    impl crate::obj::cap::GodotDefault for GltfNode {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfNode {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfNode {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GltfNode`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GltfNode__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfNode > for $Class {
                
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
#[doc = "Default-param extender for [`GltfNode::get_scene_node_path_ex`][super::GltfNode::get_scene_node_path_ex]."]
#[must_use]
pub struct ExGetSceneNodePath < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::GltfNode, gltf_state: CowArg < 'a, Option < Gd < crate::classes::GltfState > > >, handle_skeletons: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSceneNodePath < 'a > {
    fn new(surround_object: &'a mut re_export::GltfNode, gltf_state: impl AsArg < Option < Gd < crate::classes::GltfState >> > + 'a,) -> Self {
        let handle_skeletons = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, gltf_state: gltf_state.into_arg(), handle_skeletons: handle_skeletons,
        }
    }
    #[inline]
    pub fn handle_skeletons(self, handle_skeletons: bool) -> Self {
        Self {
            handle_skeletons: handle_skeletons, .. self
        }
    }
    #[inline]
    pub fn done(self) -> NodePath {
        let Self {
            _phantom, surround_object, gltf_state, handle_skeletons,
        }
        = self;
        re_export::GltfNode::get_scene_node_path_full(surround_object, gltf_state, handle_skeletons,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GltfNode;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for GltfNode {
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