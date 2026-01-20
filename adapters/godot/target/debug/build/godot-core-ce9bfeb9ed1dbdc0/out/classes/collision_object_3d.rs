#![doc = "Sidecar module for class [`CollisionObject3D`][crate::classes::CollisionObject3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CollisionObject3D` enums](https://docs.godotengine.org/en/stable/classes/class_collisionobject3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CollisionObject3D.`\n\nInherits [`Node3D`][crate::classes::Node3D].\n\nRelated symbols:\n\n* [`collision_object_3d`][crate::classes::collision_object_3d]: sidecar module with related enum/flag types\n* [`SignalsOfCollisionObject3D`][crate::classes::collision_object_3d::SignalsOfCollisionObject3D]: signal collection\n\n\nSee also [Godot docs for `CollisionObject3D`](https://docs.godotengine.org/en/stable/classes/class_collisionobject3d.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<CollisionObject3D>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CollisionObject3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl CollisionObject3D {
        pub fn set_collision_layer(&mut self, layer: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "set_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_layer(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "get_collision_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_mask(&mut self, mask: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (mask,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2308usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "set_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_mask(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2309usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "get_collision_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_layer_value(&mut self, layer_number: i32, value: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2310usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "set_collision_layer_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_layer_value(&self, layer_number: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2311usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "get_collision_layer_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_mask_value(&mut self, layer_number: i32, value: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (layer_number, value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2312usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "set_collision_mask_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_mask_value(&self, layer_number: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (layer_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2313usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "get_collision_mask_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_collision_priority(&mut self, priority: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (priority,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2314usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "set_collision_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_collision_priority(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2315usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "get_collision_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_disable_mode(&mut self, mode: crate::classes::collision_object_3d::DisableMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::collision_object_3d::DisableMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2316usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "set_disable_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_disable_mode(&self,) -> crate::classes::collision_object_3d::DisableMode {
            type CallRet = crate::classes::collision_object_3d::DisableMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2317usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "get_disable_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ray_pickable(&mut self, ray_pickable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (ray_pickable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2318usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "set_ray_pickable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_ray_pickable(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2319usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "is_ray_pickable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_capture_input_on_drag(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2320usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "set_capture_input_on_drag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_capture_input_on_drag(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2321usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "get_capture_input_on_drag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rid(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "get_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_shape_owner(&mut self, owner: impl AsArg < Option < Gd < crate::classes::Object >> >,) -> u32 {
            type CallRet = u32;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >,);
            let args = (owner.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2323usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "create_shape_owner", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_shape_owner(&mut self, owner_id: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "remove_shape_owner", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shape_owners(&mut self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "get_shape_owners", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shape_owner_set_transform(&mut self, owner_id: u32, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (u32, Transform3D,);
            let args = (owner_id, transform,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "shape_owner_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shape_owner_get_transform(&self, owner_id: u32,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (u32,);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "shape_owner_get_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shape_owner_get_owner(&self, owner_id: u32,) -> Option < Gd < crate::classes::Object > > {
            type CallRet = Option < Gd < crate::classes::Object > >;
            type CallParams = (u32,);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "shape_owner_get_owner", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shape_owner_set_disabled(&mut self, owner_id: u32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (u32, bool,);
            let args = (owner_id, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "shape_owner_set_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_shape_owner_disabled(&self, owner_id: u32,) -> bool {
            type CallRet = bool;
            type CallParams = (u32,);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "is_shape_owner_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shape_owner_add_shape(&mut self, owner_id: u32, shape: impl AsArg < Option < Gd < crate::classes::Shape3D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (u32, CowArg < 'a0, Option < Gd < crate::classes::Shape3D > > >,);
            let args = (owner_id, shape.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "shape_owner_add_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shape_owner_get_shape_count(&self, owner_id: u32,) -> i32 {
            type CallRet = i32;
            type CallParams = (u32,);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "shape_owner_get_shape_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shape_owner_get_shape(&self, owner_id: u32, shape_id: i32,) -> Option < Gd < crate::classes::Shape3D > > {
            type CallRet = Option < Gd < crate::classes::Shape3D > >;
            type CallParams = (u32, i32,);
            let args = (owner_id, shape_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "shape_owner_get_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shape_owner_get_shape_index(&self, owner_id: u32, shape_id: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (u32, i32,);
            let args = (owner_id, shape_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "shape_owner_get_shape_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shape_owner_remove_shape(&mut self, owner_id: u32, shape_id: i32,) {
            type CallRet = ();
            type CallParams = (u32, i32,);
            let args = (owner_id, shape_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "shape_owner_remove_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shape_owner_clear_shapes(&mut self, owner_id: u32,) {
            type CallRet = ();
            type CallParams = (u32,);
            let args = (owner_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "shape_owner_clear_shapes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shape_find_owner(&self, shape_index: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (i32,);
            let args = (shape_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CollisionObject3D", "shape_find_owner", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CollisionObject3D {
        type Base = crate::classes::Node3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CollisionObject3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CollisionObject3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for CollisionObject3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for CollisionObject3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CollisionObject3D {
        
    }
    impl std::ops::Deref for CollisionObject3D {
        type Target = crate::classes::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CollisionObject3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CollisionObject3D__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `CollisionObject3D` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DisableMode {
    ord: i32
}
impl DisableMode {
    #[doc(alias = "DISABLE_MODE_REMOVE")]
    #[doc = "Godot enumerator name: `DISABLE_MODE_REMOVE`"]
    pub const REMOVE: DisableMode = DisableMode {
        ord: 0i32
    };
    #[doc(alias = "DISABLE_MODE_MAKE_STATIC")]
    #[doc = "Godot enumerator name: `DISABLE_MODE_MAKE_STATIC`"]
    pub const MAKE_STATIC: DisableMode = DisableMode {
        ord: 1i32
    };
    #[doc(alias = "DISABLE_MODE_KEEP_ACTIVE")]
    #[doc = "Godot enumerator name: `DISABLE_MODE_KEEP_ACTIVE`"]
    pub const KEEP_ACTIVE: DisableMode = DisableMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DisableMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DisableMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DisableMode {
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
            Self::REMOVE => "REMOVE", Self::MAKE_STATIC => "MAKE_STATIC", Self::KEEP_ACTIVE => "KEEP_ACTIVE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DisableMode::REMOVE, DisableMode::MAKE_STATIC, DisableMode::KEEP_ACTIVE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DisableMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("REMOVE", "DISABLE_MODE_REMOVE", DisableMode::REMOVE), crate::meta::inspect::EnumConstant::new("MAKE_STATIC", "DISABLE_MODE_MAKE_STATIC", DisableMode::MAKE_STATIC), crate::meta::inspect::EnumConstant::new("KEEP_ACTIVE", "DISABLE_MODE_KEEP_ACTIVE", DisableMode::KEEP_ACTIVE)]
        }
    }
}
impl crate::meta::GodotConvert for DisableMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DisableMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DisableMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CollisionObject3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`CollisionObject3D`][crate::classes::CollisionObject3D] class."]
    pub struct SignalsOfCollisionObject3D < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfCollisionObject3D < 'c, C > {
        #[doc = "Signature: `(camera: Gd<Node>, event: Gd<InputEvent>, event_position: Vector3, normal: Vector3, shape_idx: i64)`"]
        pub fn input_event(&mut self) -> SigInputEvent < 'c, C > {
            SigInputEvent {
                typed: TypedSignal::extract(&mut self.__internal_obj, "input_event")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn mouse_entered(&mut self) -> SigMouseEntered < 'c, C > {
            SigMouseEntered {
                typed: TypedSignal::extract(&mut self.__internal_obj, "mouse_entered")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn mouse_exited(&mut self) -> SigMouseExited < 'c, C > {
            SigMouseExited {
                typed: TypedSignal::extract(&mut self.__internal_obj, "mouse_exited")
            }
        }
    }
    type TypedSigInputEvent < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >, Gd < crate::classes::InputEvent >, Vector3, Vector3, i64,) >;
    pub struct SigInputEvent < 'c, C: WithSignals > {
        typed: TypedSigInputEvent < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigInputEvent < 'c, C > {
        pub fn emit(&mut self, camera: Gd < crate::classes::Node >, event: Gd < crate::classes::InputEvent >, event_position: Vector3, normal: Vector3, shape_idx: i64,) {
            self.typed.emit_tuple((camera, event, event_position, normal, shape_idx,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigInputEvent < 'c, C > {
        type Target = TypedSigInputEvent < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigInputEvent < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMouseEntered < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigMouseEntered < 'c, C: WithSignals > {
        typed: TypedSigMouseEntered < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMouseEntered < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMouseEntered < 'c, C > {
        type Target = TypedSigMouseEntered < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMouseEntered < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMouseExited < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigMouseExited < 'c, C: WithSignals > {
        typed: TypedSigMouseExited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMouseExited < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMouseExited < 'c, C > {
        type Target = TypedSigMouseExited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMouseExited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for CollisionObject3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCollisionObject3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfCollisionObject3D < 'c, C > {
        type Target = < < CollisionObject3D as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = CollisionObject3D;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfCollisionObject3D < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = CollisionObject3D;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}