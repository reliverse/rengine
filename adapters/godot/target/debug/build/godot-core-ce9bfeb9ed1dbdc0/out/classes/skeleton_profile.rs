#![doc = "Sidecar module for class [`SkeletonProfile`][crate::classes::SkeletonProfile].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SkeletonProfile` enums](https://docs.godotengine.org/en/stable/classes/class_skeletonprofile.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SkeletonProfile.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`skeleton_profile`][crate::classes::skeleton_profile]: sidecar module with related enum/flag types\n* [`ISkeletonProfile`][crate::classes::ISkeletonProfile]: virtual methods\n* [`SignalsOfSkeletonProfile`][crate::classes::skeleton_profile::SignalsOfSkeletonProfile]: signal collection\n\n\nSee also [Godot docs for `SkeletonProfile`](https://docs.godotengine.org/en/stable/classes/class_skeletonprofile.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`SkeletonProfile::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SkeletonProfile {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`SkeletonProfile`][crate::classes::SkeletonProfile].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `SkeletonProfile` methods](https://docs.godotengine.org/en/stable/classes/class_skeletonprofile.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISkeletonProfile: crate::obj::GodotClass < Base = SkeletonProfile > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SkeletonProfile {
        pub fn set_root_bone(&mut self, bone_name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_root_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_bone(&mut self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_root_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scale_base_bone(&mut self, bone_name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_scale_base_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scale_base_bone(&mut self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_scale_base_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_group_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_group_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_group_size(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_group_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_group_name(&self, group_idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (group_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_group_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_group_name(&mut self, group_idx: i32, group_name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, StringName >,);
            let args = (group_idx, group_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_group_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture(&self, group_idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (group_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture(&mut self, group_idx: i32, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (group_idx, texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bone_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_bone_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bone_size(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_bone_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn find_bone(&self, bone_name: impl AsArg < StringName >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "find_bone", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bone_name(&self, bone_idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bone_name(&mut self, bone_idx: i32, bone_name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, StringName >,);
            let args = (bone_idx, bone_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_bone_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bone_parent(&self, bone_idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_bone_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bone_parent(&mut self, bone_idx: i32, bone_parent: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, StringName >,);
            let args = (bone_idx, bone_parent.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_bone_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tail_direction(&self, bone_idx: i32,) -> crate::classes::skeleton_profile::TailDirection {
            type CallRet = crate::classes::skeleton_profile::TailDirection;
            type CallParams = (i32,);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_tail_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tail_direction(&mut self, bone_idx: i32, tail_direction: crate::classes::skeleton_profile::TailDirection,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::skeleton_profile::TailDirection,);
            let args = (bone_idx, tail_direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_tail_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bone_tail(&self, bone_idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_bone_tail", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_bone_tail(&mut self, bone_idx: i32, bone_tail: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, StringName >,);
            let args = (bone_idx, bone_tail.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_bone_tail", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_reference_pose(&self, bone_idx: i32,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (i32,);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_reference_pose", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_reference_pose(&mut self, bone_idx: i32, bone_name: Transform3D,) {
            type CallRet = ();
            type CallParams = (i32, Transform3D,);
            let args = (bone_idx, bone_name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_reference_pose", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_handle_offset(&self, bone_idx: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_handle_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_handle_offset(&mut self, bone_idx: i32, handle_offset: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2,);
            let args = (bone_idx, handle_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_handle_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_group(&self, bone_idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "get_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_group(&mut self, bone_idx: i32, group: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, StringName >,);
            let args = (bone_idx, group.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_required(&self, bone_idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (bone_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "is_required", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_required(&mut self, bone_idx: i32, required: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (bone_idx, required,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SkeletonProfile", "set_required", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for SkeletonProfile {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("SkeletonProfile"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SkeletonProfile {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for SkeletonProfile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for SkeletonProfile {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SkeletonProfile {
        
    }
    impl crate::obj::cap::GodotDefault for SkeletonProfile {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SkeletonProfile {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SkeletonProfile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SkeletonProfile`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_SkeletonProfile__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SkeletonProfile > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TailDirection {
    ord: i32
}
impl TailDirection {
    #[doc(alias = "TAIL_DIRECTION_AVERAGE_CHILDREN")]
    #[doc = "Godot enumerator name: `TAIL_DIRECTION_AVERAGE_CHILDREN`"]
    pub const AVERAGE_CHILDREN: TailDirection = TailDirection {
        ord: 0i32
    };
    #[doc(alias = "TAIL_DIRECTION_SPECIFIC_CHILD")]
    #[doc = "Godot enumerator name: `TAIL_DIRECTION_SPECIFIC_CHILD`"]
    pub const SPECIFIC_CHILD: TailDirection = TailDirection {
        ord: 1i32
    };
    #[doc(alias = "TAIL_DIRECTION_END")]
    #[doc = "Godot enumerator name: `TAIL_DIRECTION_END`"]
    pub const END: TailDirection = TailDirection {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TailDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TailDirection") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TailDirection {
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
            Self::AVERAGE_CHILDREN => "AVERAGE_CHILDREN", Self::SPECIFIC_CHILD => "SPECIFIC_CHILD", Self::END => "END", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TailDirection::AVERAGE_CHILDREN, TailDirection::SPECIFIC_CHILD, TailDirection::END]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TailDirection >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("AVERAGE_CHILDREN", "TAIL_DIRECTION_AVERAGE_CHILDREN", TailDirection::AVERAGE_CHILDREN), crate::meta::inspect::EnumConstant::new("SPECIFIC_CHILD", "TAIL_DIRECTION_SPECIFIC_CHILD", TailDirection::SPECIFIC_CHILD), crate::meta::inspect::EnumConstant::new("END", "TAIL_DIRECTION_END", TailDirection::END)]
        }
    }
}
impl crate::meta::GodotConvert for TailDirection {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TailDirection {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TailDirection {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::SkeletonProfile;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`SkeletonProfile`][crate::classes::SkeletonProfile] class."]
    pub struct SignalsOfSkeletonProfile < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfSkeletonProfile < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn profile_updated(&mut self) -> SigProfileUpdated < 'c, C > {
            SigProfileUpdated {
                typed: TypedSignal::extract(&mut self.__internal_obj, "profile_updated")
            }
        }
    }
    type TypedSigProfileUpdated < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigProfileUpdated < 'c, C: WithSignals > {
        typed: TypedSigProfileUpdated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigProfileUpdated < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigProfileUpdated < 'c, C > {
        type Target = TypedSigProfileUpdated < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigProfileUpdated < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for SkeletonProfile {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfSkeletonProfile < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfSkeletonProfile < 'c, C > {
        type Target = < < SkeletonProfile as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = SkeletonProfile;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfSkeletonProfile < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = SkeletonProfile;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}