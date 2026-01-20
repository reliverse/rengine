#![doc = "Sidecar module for class [`GpuParticlesCollisionHeightField3D`][crate::classes::GpuParticlesCollisionHeightField3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GPUParticlesCollisionHeightField3D` enums](https://docs.godotengine.org/en/stable/classes/class_gpuparticlescollisionheightfield3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GPUParticlesCollisionHeightField3D.`\n\nInherits [`GpuParticlesCollision3D`][crate::classes::GpuParticlesCollision3D].\n\nRelated symbols:\n\n* [`gpu_particles_collision_height_field_3d`][crate::classes::gpu_particles_collision_height_field_3d]: sidecar module with related enum/flag types\n* [`IGpuParticlesCollisionHeightField3D`][crate::classes::IGpuParticlesCollisionHeightField3D]: virtual methods\n\n\nSee also [Godot docs for `GPUParticlesCollisionHeightField3D`](https://docs.godotengine.org/en/stable/classes/class_gpuparticlescollisionheightfield3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`GpuParticlesCollisionHeightField3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GpuParticlesCollisionHeightField3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GpuParticlesCollisionHeightField3D`][crate::classes::GpuParticlesCollisionHeightField3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IGpuParticlesCollision3D`~~ > [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `GPUParticlesCollisionHeightField3D` methods](https://docs.godotengine.org/en/stable/classes/class_gpuparticlescollisionheightfield3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGpuParticlesCollisionHeightField3D: crate::obj::GodotClass < Base = GpuParticlesCollisionHeightField3D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_aabb(&self,) -> Aabb {
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
    impl GpuParticlesCollisionHeightField3D {
        pub fn set_size(&mut self, size: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3971usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionHeightField3D", "set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3972usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionHeightField3D", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_resolution(&mut self, resolution: crate::classes::gpu_particles_collision_height_field_3d::Resolution,) {
            type CallRet = ();
            type CallParams = (crate::classes::gpu_particles_collision_height_field_3d::Resolution,);
            let args = (resolution,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3973usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionHeightField3D", "set_resolution", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_resolution(&self,) -> crate::classes::gpu_particles_collision_height_field_3d::Resolution {
            type CallRet = crate::classes::gpu_particles_collision_height_field_3d::Resolution;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3974usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionHeightField3D", "get_resolution", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_update_mode(&mut self, update_mode: crate::classes::gpu_particles_collision_height_field_3d::UpdateMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::gpu_particles_collision_height_field_3d::UpdateMode,);
            let args = (update_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3975usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionHeightField3D", "set_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_update_mode(&self,) -> crate::classes::gpu_particles_collision_height_field_3d::UpdateMode {
            type CallRet = crate::classes::gpu_particles_collision_height_field_3d::UpdateMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3976usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionHeightField3D", "get_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_heightfield_mask(&mut self, heightfield_mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (heightfield_mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3977usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionHeightField3D", "set_heightfield_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_heightfield_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3978usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionHeightField3D", "get_heightfield_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_heightfield_mask_value(&mut self, layer_number: i32, value: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3979usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionHeightField3D", "set_heightfield_mask_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_heightfield_mask_value(&self, layer_number: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3980usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionHeightField3D", "get_heightfield_mask_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_follow_camera_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3981usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionHeightField3D", "set_follow_camera_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_follow_camera_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3982usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GpuParticlesCollisionHeightField3D", "is_follow_camera_enabled", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for GpuParticlesCollisionHeightField3D {
        type Base = crate::classes::GpuParticlesCollision3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GPUParticlesCollisionHeightField3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GpuParticlesCollisionHeightField3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GpuParticlesCollision3D > for GpuParticlesCollisionHeightField3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for GpuParticlesCollisionHeightField3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for GpuParticlesCollisionHeightField3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for GpuParticlesCollisionHeightField3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GpuParticlesCollisionHeightField3D {
        
    }
    impl crate::obj::cap::GodotDefault for GpuParticlesCollisionHeightField3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GpuParticlesCollisionHeightField3D {
        type Target = crate::classes::GpuParticlesCollision3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GpuParticlesCollisionHeightField3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GpuParticlesCollisionHeightField3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GpuParticlesCollisionHeightField3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GpuParticlesCollisionHeightField3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::GpuParticlesCollision3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualInstance3D > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Resolution {
    ord: i32
}
impl Resolution {
    pub const RESOLUTION_256: Resolution = Resolution {
        ord: 0i32
    };
    pub const RESOLUTION_512: Resolution = Resolution {
        ord: 1i32
    };
    pub const RESOLUTION_1024: Resolution = Resolution {
        ord: 2i32
    };
    pub const RESOLUTION_2048: Resolution = Resolution {
        ord: 3i32
    };
    pub const RESOLUTION_4096: Resolution = Resolution {
        ord: 4i32
    };
    pub const RESOLUTION_8192: Resolution = Resolution {
        ord: 5i32
    };
    #[doc(alias = "RESOLUTION_MAX")]
    #[doc = "Godot enumerator name: `RESOLUTION_MAX`"]
    pub const MAX: Resolution = Resolution {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Resolution") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Resolution {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::RESOLUTION_256 => "RESOLUTION_256", Self::RESOLUTION_512 => "RESOLUTION_512", Self::RESOLUTION_1024 => "RESOLUTION_1024", Self::RESOLUTION_2048 => "RESOLUTION_2048", Self::RESOLUTION_4096 => "RESOLUTION_4096", Self::RESOLUTION_8192 => "RESOLUTION_8192", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Resolution::RESOLUTION_256, Resolution::RESOLUTION_512, Resolution::RESOLUTION_1024, Resolution::RESOLUTION_2048, Resolution::RESOLUTION_4096, Resolution::RESOLUTION_8192]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Resolution >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RESOLUTION_256", "RESOLUTION_256", Resolution::RESOLUTION_256), crate::meta::inspect::EnumConstant::new("RESOLUTION_512", "RESOLUTION_512", Resolution::RESOLUTION_512), crate::meta::inspect::EnumConstant::new("RESOLUTION_1024", "RESOLUTION_1024", Resolution::RESOLUTION_1024), crate::meta::inspect::EnumConstant::new("RESOLUTION_2048", "RESOLUTION_2048", Resolution::RESOLUTION_2048), crate::meta::inspect::EnumConstant::new("RESOLUTION_4096", "RESOLUTION_4096", Resolution::RESOLUTION_4096), crate::meta::inspect::EnumConstant::new("RESOLUTION_8192", "RESOLUTION_8192", Resolution::RESOLUTION_8192), crate::meta::inspect::EnumConstant::new("MAX", "RESOLUTION_MAX", Resolution::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Resolution {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for Resolution {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Resolution {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Resolution {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct UpdateMode {
    ord: i32
}
impl UpdateMode {
    #[doc(alias = "UPDATE_MODE_WHEN_MOVED")]
    #[doc = "Godot enumerator name: `UPDATE_MODE_WHEN_MOVED`"]
    pub const WHEN_MOVED: UpdateMode = UpdateMode {
        ord: 0i32
    };
    #[doc(alias = "UPDATE_MODE_ALWAYS")]
    #[doc = "Godot enumerator name: `UPDATE_MODE_ALWAYS`"]
    pub const ALWAYS: UpdateMode = UpdateMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for UpdateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("UpdateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for UpdateMode {
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
            Self::WHEN_MOVED => "WHEN_MOVED", Self::ALWAYS => "ALWAYS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[UpdateMode::WHEN_MOVED, UpdateMode::ALWAYS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < UpdateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("WHEN_MOVED", "UPDATE_MODE_WHEN_MOVED", UpdateMode::WHEN_MOVED), crate::meta::inspect::EnumConstant::new("ALWAYS", "UPDATE_MODE_ALWAYS", UpdateMode::ALWAYS)]
        }
    }
}
impl crate::meta::GodotConvert for UpdateMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for UpdateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for UpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GpuParticlesCollisionHeightField3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for GpuParticlesCollisionHeightField3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}