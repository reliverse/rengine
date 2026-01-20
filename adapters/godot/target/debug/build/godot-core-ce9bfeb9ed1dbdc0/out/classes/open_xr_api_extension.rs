#![doc = "Sidecar module for class [`OpenXrApiExtension`][crate::classes::OpenXrApiExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OpenXRAPIExtension` enums](https://docs.godotengine.org/en/stable/classes/class_openxrapiextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OpenXRAPIExtension.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`open_xr_api_extension`][crate::classes::open_xr_api_extension]: sidecar module with related enum/flag types\n* [`IOpenXrApiExtension`][crate::classes::IOpenXrApiExtension]: virtual methods\n\n\nSee also [Godot docs for `OpenXRAPIExtension`](https://docs.godotengine.org/en/stable/classes/class_openxrapiextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`OpenXrApiExtension::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OpenXrApiExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`OpenXrApiExtension`][crate::classes::OpenXrApiExtension].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `OpenXRAPIExtension` methods](https://docs.godotengine.org/en/stable/classes/class_openxrapiextension.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOpenXrApiExtension: crate::obj::GodotClass < Base = OpenXrApiExtension > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl OpenXrApiExtension {
        pub fn get_instance(&mut self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5901usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_instance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_system_id(&mut self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5902usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_system_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_session(&mut self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5903usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_session", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        pub unsafe fn transform_from_pose(&mut self, pose: * const c_void,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (* const c_void,);
            let args = (pose,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5904usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "transform_from_pose", Some(self.__validated_obj()), args,)
            }
        }
        pub fn xr_result(&mut self, result: u64, format: impl AsArg < GString >, args: &VarArray,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (u64, CowArg < 'a0, GString >, RefArg < 'a1, VarArray >,);
            let args = (result, format.into_arg(), RefArg::new(args),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5905usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "xr_result", Some(self.__validated_obj()), args,)
            }
        }
        pub fn openxr_is_enabled(check_run_in_editor: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (bool,);
            let args = (check_run_in_editor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5906usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "openxr_is_enabled", None, args,)
            }
        }
        pub fn get_instance_proc_addr(&mut self, name: impl AsArg < GString >,) -> u64 {
            type CallRet = u64;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5907usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_instance_proc_addr", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_error_string(&mut self, result: u64,) -> GString {
            type CallRet = GString;
            type CallParams = (u64,);
            let args = (result,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5908usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_error_string", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_swapchain_format_name(&mut self, swapchain_format: i64,) -> GString {
            type CallRet = GString;
            type CallParams = (i64,);
            let args = (swapchain_format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5909usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_swapchain_format_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_object_name(&mut self, object_type: i64, object_handle: u64, object_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i64, u64, CowArg < 'a0, GString >,);
            let args = (object_type, object_handle, object_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5910usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "set_object_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn begin_debug_label_region(&mut self, label_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (label_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5911usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "begin_debug_label_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn end_debug_label_region(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5912usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "end_debug_label_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn insert_debug_label(&mut self, label_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (label_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5913usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "insert_debug_label", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_initialized(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5914usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "is_initialized", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_running(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5915usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "is_running", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        pub unsafe fn set_custom_play_space(&mut self, space: * const c_void,) {
            type CallRet = ();
            type CallParams = (* const c_void,);
            let args = (space,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5916usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "set_custom_play_space", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_play_space(&mut self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5917usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_play_space", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_predicted_display_time(&mut self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5918usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_predicted_display_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_next_frame_time(&mut self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5919usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_next_frame_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn can_render(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5920usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "can_render", Some(self.__validated_obj()), args,)
            }
        }
        pub fn find_action(&mut self, name: impl AsArg < GString >, action_set: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, Rid,);
            let args = (name.into_arg(), action_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5921usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "find_action", Some(self.__validated_obj()), args,)
            }
        }
        pub fn action_get_handle(&mut self, action: Rid,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid,);
            let args = (action,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5922usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "action_get_handle", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hand_tracker(&mut self, hand_index: i32,) -> u64 {
            type CallRet = u64;
            type CallParams = (i32,);
            let args = (hand_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5923usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_hand_tracker", Some(self.__validated_obj()), args,)
            }
        }
        pub fn register_composition_layer_provider(&mut self, extension: impl AsArg < Option < Gd < crate::classes::OpenXrExtensionWrapper >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrExtensionWrapper > > >,);
            let args = (extension.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5924usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "register_composition_layer_provider", Some(self.__validated_obj()), args,)
            }
        }
        pub fn unregister_composition_layer_provider(&mut self, extension: impl AsArg < Option < Gd < crate::classes::OpenXrExtensionWrapper >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrExtensionWrapper > > >,);
            let args = (extension.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5925usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "unregister_composition_layer_provider", Some(self.__validated_obj()), args,)
            }
        }
        pub fn register_projection_views_extension(&mut self, extension: impl AsArg < Option < Gd < crate::classes::OpenXrExtensionWrapper >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrExtensionWrapper > > >,);
            let args = (extension.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5926usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "register_projection_views_extension", Some(self.__validated_obj()), args,)
            }
        }
        pub fn unregister_projection_views_extension(&mut self, extension: impl AsArg < Option < Gd < crate::classes::OpenXrExtensionWrapper >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrExtensionWrapper > > >,);
            let args = (extension.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5927usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "unregister_projection_views_extension", Some(self.__validated_obj()), args,)
            }
        }
        pub fn register_frame_info_extension(&mut self, extension: impl AsArg < Option < Gd < crate::classes::OpenXrExtensionWrapper >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrExtensionWrapper > > >,);
            let args = (extension.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5928usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "register_frame_info_extension", Some(self.__validated_obj()), args,)
            }
        }
        pub fn unregister_frame_info_extension(&mut self, extension: impl AsArg < Option < Gd < crate::classes::OpenXrExtensionWrapper >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::OpenXrExtensionWrapper > > >,);
            let args = (extension.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5929usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "unregister_frame_info_extension", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_render_state_z_near(&mut self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5930usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_render_state_z_near", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_render_state_z_far(&mut self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5931usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_render_state_z_far", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_velocity_texture(&mut self, render_target: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (render_target,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5932usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "set_velocity_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_velocity_depth_texture(&mut self, render_target: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (render_target,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5933usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "set_velocity_depth_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_velocity_target_size(&mut self, target_size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (target_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5934usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "set_velocity_target_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_supported_swapchain_formats(&mut self,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5935usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_supported_swapchain_formats", Some(self.__validated_obj()), args,)
            }
        }
        pub fn openxr_swapchain_create(&mut self, create_flags: u64, usage_flags: u64, swapchain_format: i64, width: u32, height: u32, sample_count: u32, array_size: u32,) -> u64 {
            type CallRet = u64;
            type CallParams = (u64, u64, i64, u32, u32, u32, u32,);
            let args = (create_flags, usage_flags, swapchain_format, width, height, sample_count, array_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5936usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "openxr_swapchain_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn openxr_swapchain_free(&mut self, swapchain: u64,) {
            type CallRet = ();
            type CallParams = (u64,);
            let args = (swapchain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5937usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "openxr_swapchain_free", Some(self.__validated_obj()), args,)
            }
        }
        pub fn openxr_swapchain_get_swapchain(&mut self, swapchain: u64,) -> u64 {
            type CallRet = u64;
            type CallParams = (u64,);
            let args = (swapchain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5938usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "openxr_swapchain_get_swapchain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn openxr_swapchain_acquire(&mut self, swapchain: u64,) {
            type CallRet = ();
            type CallParams = (u64,);
            let args = (swapchain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5939usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "openxr_swapchain_acquire", Some(self.__validated_obj()), args,)
            }
        }
        pub fn openxr_swapchain_get_image(&mut self, swapchain: u64,) -> Rid {
            type CallRet = Rid;
            type CallParams = (u64,);
            let args = (swapchain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5940usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "openxr_swapchain_get_image", Some(self.__validated_obj()), args,)
            }
        }
        pub fn openxr_swapchain_release(&mut self, swapchain: u64,) {
            type CallRet = ();
            type CallParams = (u64,);
            let args = (swapchain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5941usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "openxr_swapchain_release", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_projection_layer(&mut self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5942usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "get_projection_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_render_region(&mut self, render_region: Rect2i,) {
            type CallRet = ();
            type CallParams = (Rect2i,);
            let args = (render_region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5943usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "set_render_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_emulate_environment_blend_mode_alpha_blend(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5944usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "set_emulate_environment_blend_mode_alpha_blend", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_environment_blend_mode_alpha_supported(&mut self,) -> crate::classes::open_xr_api_extension::OpenXrAlphaBlendModeSupport {
            type CallRet = crate::classes::open_xr_api_extension::OpenXrAlphaBlendModeSupport;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5945usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OpenXrApiExtension", "is_environment_blend_mode_alpha_supported", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for OpenXrApiExtension {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OpenXRAPIExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OpenXrApiExtension {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for OpenXrApiExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OpenXrApiExtension {
        
    }
    impl crate::obj::cap::GodotDefault for OpenXrApiExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OpenXrApiExtension {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OpenXrApiExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OpenXrApiExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OpenXrApiExtension__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OpenXrApiExtension > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `OpenXRAlphaBlendModeSupport`."]
pub struct OpenXrAlphaBlendModeSupport {
    ord: i32
}
impl OpenXrAlphaBlendModeSupport {
    #[doc(alias = "OPENXR_ALPHA_BLEND_MODE_SUPPORT_NONE")]
    #[doc = "Godot enumerator name: `OPENXR_ALPHA_BLEND_MODE_SUPPORT_NONE`"]
    pub const NONE: OpenXrAlphaBlendModeSupport = OpenXrAlphaBlendModeSupport {
        ord: 0i32
    };
    #[doc(alias = "OPENXR_ALPHA_BLEND_MODE_SUPPORT_REAL")]
    #[doc = "Godot enumerator name: `OPENXR_ALPHA_BLEND_MODE_SUPPORT_REAL`"]
    pub const REAL: OpenXrAlphaBlendModeSupport = OpenXrAlphaBlendModeSupport {
        ord: 1i32
    };
    #[doc(alias = "OPENXR_ALPHA_BLEND_MODE_SUPPORT_EMULATING")]
    #[doc = "Godot enumerator name: `OPENXR_ALPHA_BLEND_MODE_SUPPORT_EMULATING`"]
    pub const EMULATING: OpenXrAlphaBlendModeSupport = OpenXrAlphaBlendModeSupport {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for OpenXrAlphaBlendModeSupport {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("OpenXrAlphaBlendModeSupport") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for OpenXrAlphaBlendModeSupport {
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
            Self::NONE => "NONE", Self::REAL => "REAL", Self::EMULATING => "EMULATING", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[OpenXrAlphaBlendModeSupport::NONE, OpenXrAlphaBlendModeSupport::REAL, OpenXrAlphaBlendModeSupport::EMULATING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < OpenXrAlphaBlendModeSupport >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "OPENXR_ALPHA_BLEND_MODE_SUPPORT_NONE", OpenXrAlphaBlendModeSupport::NONE), crate::meta::inspect::EnumConstant::new("REAL", "OPENXR_ALPHA_BLEND_MODE_SUPPORT_REAL", OpenXrAlphaBlendModeSupport::REAL), crate::meta::inspect::EnumConstant::new("EMULATING", "OPENXR_ALPHA_BLEND_MODE_SUPPORT_EMULATING", OpenXrAlphaBlendModeSupport::EMULATING)]
        }
    }
}
impl crate::meta::GodotConvert for OpenXrAlphaBlendModeSupport {
    type Via = i32;
    
}
impl crate::meta::ToGodot for OpenXrAlphaBlendModeSupport {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for OpenXrAlphaBlendModeSupport {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OpenXrApiExtension;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for OpenXrApiExtension {
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