#![doc = "Sidecar module for class [`RdPipelineDepthStencilState`][crate::classes::RdPipelineDepthStencilState].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RDPipelineDepthStencilState` enums](https://docs.godotengine.org/en/stable/classes/class_rdpipelinedepthstencilstate.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RDPipelineDepthStencilState.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`IRdPipelineDepthStencilState`][crate::classes::IRdPipelineDepthStencilState]: virtual methods\n\n\nSee also [Godot docs for `RDPipelineDepthStencilState`](https://docs.godotengine.org/en/stable/classes/class_rdpipelinedepthstencilstate.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`RdPipelineDepthStencilState::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RdPipelineDepthStencilState {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`RdPipelineDepthStencilState`][crate::classes::RdPipelineDepthStencilState].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `RDPipelineDepthStencilState` methods](https://docs.godotengine.org/en/stable/classes/class_rdpipelinedepthstencilstate.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRdPipelineDepthStencilState: crate::obj::GodotClass < Base = RdPipelineDepthStencilState > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl RdPipelineDepthStencilState {
        pub fn set_enable_depth_test(&mut self, p_member: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6950usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_enable_depth_test", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_enable_depth_test(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6951usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_enable_depth_test", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enable_depth_write(&mut self, p_member: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6952usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_enable_depth_write", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_enable_depth_write(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6953usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_enable_depth_write", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_depth_compare_operator(&mut self, p_member: crate::classes::rendering_device::CompareOperator,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::CompareOperator,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6954usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_depth_compare_operator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_depth_compare_operator(&self,) -> crate::classes::rendering_device::CompareOperator {
            type CallRet = crate::classes::rendering_device::CompareOperator;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6955usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_depth_compare_operator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enable_depth_range(&mut self, p_member: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6956usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_enable_depth_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_enable_depth_range(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6957usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_enable_depth_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_depth_range_min(&mut self, p_member: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6958usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_depth_range_min", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_depth_range_min(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6959usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_depth_range_min", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_depth_range_max(&mut self, p_member: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6960usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_depth_range_max", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_depth_range_max(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6961usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_depth_range_max", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enable_stencil(&mut self, p_member: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6962usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_enable_stencil", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_enable_stencil(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6963usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_enable_stencil", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_front_op_fail(&mut self, p_member: crate::classes::rendering_device::StencilOperation,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::StencilOperation,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6964usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_front_op_fail", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_front_op_fail(&self,) -> crate::classes::rendering_device::StencilOperation {
            type CallRet = crate::classes::rendering_device::StencilOperation;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6965usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_front_op_fail", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_front_op_pass(&mut self, p_member: crate::classes::rendering_device::StencilOperation,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::StencilOperation,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6966usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_front_op_pass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_front_op_pass(&self,) -> crate::classes::rendering_device::StencilOperation {
            type CallRet = crate::classes::rendering_device::StencilOperation;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6967usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_front_op_pass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_front_op_depth_fail(&mut self, p_member: crate::classes::rendering_device::StencilOperation,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::StencilOperation,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6968usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_front_op_depth_fail", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_front_op_depth_fail(&self,) -> crate::classes::rendering_device::StencilOperation {
            type CallRet = crate::classes::rendering_device::StencilOperation;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6969usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_front_op_depth_fail", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_front_op_compare(&mut self, p_member: crate::classes::rendering_device::CompareOperator,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::CompareOperator,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6970usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_front_op_compare", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_front_op_compare(&self,) -> crate::classes::rendering_device::CompareOperator {
            type CallRet = crate::classes::rendering_device::CompareOperator;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6971usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_front_op_compare", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_front_op_compare_mask(&mut self, p_member: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6972usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_front_op_compare_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_front_op_compare_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6973usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_front_op_compare_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_front_op_write_mask(&mut self, p_member: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6974usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_front_op_write_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_front_op_write_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6975usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_front_op_write_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_front_op_reference(&mut self, p_member: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6976usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_front_op_reference", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_front_op_reference(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6977usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_front_op_reference", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_back_op_fail(&mut self, p_member: crate::classes::rendering_device::StencilOperation,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::StencilOperation,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6978usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_back_op_fail", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_back_op_fail(&self,) -> crate::classes::rendering_device::StencilOperation {
            type CallRet = crate::classes::rendering_device::StencilOperation;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6979usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_back_op_fail", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_back_op_pass(&mut self, p_member: crate::classes::rendering_device::StencilOperation,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::StencilOperation,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6980usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_back_op_pass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_back_op_pass(&self,) -> crate::classes::rendering_device::StencilOperation {
            type CallRet = crate::classes::rendering_device::StencilOperation;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6981usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_back_op_pass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_back_op_depth_fail(&mut self, p_member: crate::classes::rendering_device::StencilOperation,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::StencilOperation,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6982usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_back_op_depth_fail", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_back_op_depth_fail(&self,) -> crate::classes::rendering_device::StencilOperation {
            type CallRet = crate::classes::rendering_device::StencilOperation;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6983usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_back_op_depth_fail", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_back_op_compare(&mut self, p_member: crate::classes::rendering_device::CompareOperator,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::CompareOperator,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6984usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_back_op_compare", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_back_op_compare(&self,) -> crate::classes::rendering_device::CompareOperator {
            type CallRet = crate::classes::rendering_device::CompareOperator;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6985usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_back_op_compare", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_back_op_compare_mask(&mut self, p_member: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6986usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_back_op_compare_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_back_op_compare_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6987usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_back_op_compare_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_back_op_write_mask(&mut self, p_member: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6988usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_back_op_write_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_back_op_write_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6989usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_back_op_write_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_back_op_reference(&mut self, p_member: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (p_member,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6990usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "set_back_op_reference", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_back_op_reference(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6991usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RdPipelineDepthStencilState", "get_back_op_reference", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for RdPipelineDepthStencilState {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("RDPipelineDepthStencilState"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RdPipelineDepthStencilState {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for RdPipelineDepthStencilState {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RdPipelineDepthStencilState {
        
    }
    impl crate::obj::cap::GodotDefault for RdPipelineDepthStencilState {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RdPipelineDepthStencilState {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RdPipelineDepthStencilState {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RdPipelineDepthStencilState`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RdPipelineDepthStencilState__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RdPipelineDepthStencilState > for $Class {
                
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
    use super::re_export::RdPipelineDepthStencilState;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for RdPipelineDepthStencilState {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfObject < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}