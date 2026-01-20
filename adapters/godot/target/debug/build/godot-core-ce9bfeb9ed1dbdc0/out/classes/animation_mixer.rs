#![doc = "Sidecar module for class [`AnimationMixer`][crate::classes::AnimationMixer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationMixer` enums](https://docs.godotengine.org/en/stable/classes/class_animationmixer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimationMixer.`\n\nInherits [`Node`][crate::classes::Node].\n\nRelated symbols:\n\n* [`animation_mixer`][crate::classes::animation_mixer]: sidecar module with related enum/flag types\n* [`SignalsOfAnimationMixer`][crate::classes::animation_mixer::SignalsOfAnimationMixer]: signal collection\n\n\nSee also [Godot docs for `AnimationMixer`](https://docs.godotengine.org/en/stable/classes/class_animationmixer.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<AnimationMixer>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimationMixer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl AnimationMixer {
        pub fn add_animation_library(&mut self, name: impl AsArg < StringName >, library: impl AsArg < Option < Gd < crate::classes::AnimationLibrary >> >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::AnimationLibrary > > >,);
            let args = (name.into_arg(), library.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "add_animation_library", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_animation_library(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "remove_animation_library", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rename_animation_library(&mut self, name: impl AsArg < StringName >, newname: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), newname.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "rename_animation_library", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_animation_library(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "has_animation_library", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_animation_library(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::AnimationLibrary > > {
            type CallRet = Option < Gd < crate::classes::AnimationLibrary > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_animation_library", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_animation_library_list(&self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_animation_library_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_animation(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "has_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_animation(&self, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Animation > > {
            type CallRet = Option < Gd < crate::classes::Animation > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_animation_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_animation_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_active(&mut self, active: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_active(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "is_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_deterministic(&mut self, deterministic: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (deterministic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_deterministic", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_deterministic(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "is_deterministic", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_root_node(&mut self, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(280usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_root_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_node(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(281usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_node", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_callback_mode_process(&mut self, mode: crate::classes::animation_mixer::AnimationCallbackModeProcess,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation_mixer::AnimationCallbackModeProcess,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(282usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_callback_mode_process", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_callback_mode_process(&self,) -> crate::classes::animation_mixer::AnimationCallbackModeProcess {
            type CallRet = crate::classes::animation_mixer::AnimationCallbackModeProcess;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(283usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_callback_mode_process", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_callback_mode_method(&mut self, mode: crate::classes::animation_mixer::AnimationCallbackModeMethod,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation_mixer::AnimationCallbackModeMethod,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(284usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_callback_mode_method", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_callback_mode_method(&self,) -> crate::classes::animation_mixer::AnimationCallbackModeMethod {
            type CallRet = crate::classes::animation_mixer::AnimationCallbackModeMethod;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(285usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_callback_mode_method", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_callback_mode_discrete(&mut self, mode: crate::classes::animation_mixer::AnimationCallbackModeDiscrete,) {
            type CallRet = ();
            type CallParams = (crate::classes::animation_mixer::AnimationCallbackModeDiscrete,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(286usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_callback_mode_discrete", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_callback_mode_discrete(&self,) -> crate::classes::animation_mixer::AnimationCallbackModeDiscrete {
            type CallRet = crate::classes::animation_mixer::AnimationCallbackModeDiscrete;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(287usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_callback_mode_discrete", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_audio_max_polyphony(&mut self, max_polyphony: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_polyphony,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(288usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_audio_max_polyphony", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_audio_max_polyphony(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(289usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_audio_max_polyphony", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_root_motion_track(&mut self, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(290usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_root_motion_track", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_motion_track(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(291usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_track", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_root_motion_local(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(292usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_root_motion_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_root_motion_local(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(293usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "is_root_motion_local", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_motion_position(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(294usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_motion_rotation(&self,) -> Quaternion {
            type CallRet = Quaternion;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(295usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_rotation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_motion_scale(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(296usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_motion_position_accumulator(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(297usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_position_accumulator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_motion_rotation_accumulator(&self,) -> Quaternion {
            type CallRet = Quaternion;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(298usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_rotation_accumulator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_motion_scale_accumulator(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(299usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "get_root_motion_scale_accumulator", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_caches(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(300usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "clear_caches", Some(self.__validated_obj()), args,)
            }
        }
        pub fn advance(&mut self, delta: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (delta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(301usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "advance", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn capture_full(&mut self, name: CowArg < StringName >, duration: f64, trans_type: crate::classes::tween::TransitionType, ease_type: crate::classes::tween::EaseType,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f64, crate::classes::tween::TransitionType, crate::classes::tween::EaseType,);
            let args = (name, duration, trans_type, ease_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(302usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "capture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::capture_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn capture(&mut self, name: impl AsArg < StringName >, duration: f64,) {
            self.capture_ex(name, duration,) . done()
        }
        #[inline]
        pub fn capture_ex < 'a > (&'a mut self, name: impl AsArg < StringName > + 'a, duration: f64,) -> ExCapture < 'a > {
            ExCapture::new(self, name, duration,)
        }
        pub fn set_reset_on_save_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(303usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "set_reset_on_save_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_reset_on_save_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(304usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "is_reset_on_save_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn find_animation(&self, animation: impl AsArg < Option < Gd < crate::classes::Animation >> >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Animation > > >,);
            let args = (animation.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(305usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "find_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn find_animation_library(&self, animation: impl AsArg < Option < Gd < crate::classes::Animation >> >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Animation > > >,);
            let args = (animation.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimationMixer", "find_animation_library", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AnimationMixer {
        type Base = crate::classes::Node;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AnimationMixer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimationMixer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for AnimationMixer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimationMixer {
        
    }
    impl std::ops::Deref for AnimationMixer {
        type Target = crate::classes::Node;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimationMixer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AnimationMixer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `AnimationMixer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`AnimationMixer::capture_ex`][super::AnimationMixer::capture_ex]."]
#[must_use]
pub struct ExCapture < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimationMixer, name: CowArg < 'a, StringName >, duration: f64, trans_type: crate::classes::tween::TransitionType, ease_type: crate::classes::tween::EaseType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCapture < 'a > {
    fn new(surround_object: &'a mut re_export::AnimationMixer, name: impl AsArg < StringName > + 'a, duration: f64,) -> Self {
        let trans_type = crate::obj::EngineEnum::from_ord(0);
        let ease_type = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), duration: duration, trans_type: trans_type, ease_type: ease_type,
        }
    }
    #[inline]
    pub fn trans_type(self, trans_type: crate::classes::tween::TransitionType) -> Self {
        Self {
            trans_type: trans_type, .. self
        }
    }
    #[inline]
    pub fn ease_type(self, ease_type: crate::classes::tween::EaseType) -> Self {
        Self {
            ease_type: ease_type, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, duration, trans_type, ease_type,
        }
        = self;
        re_export::AnimationMixer::capture_full(surround_object, name, duration, trans_type, ease_type,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnimationCallbackModeProcess {
    ord: i32
}
impl AnimationCallbackModeProcess {
    #[doc(alias = "ANIMATION_CALLBACK_MODE_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_PROCESS_PHYSICS`"]
    pub const PHYSICS: AnimationCallbackModeProcess = AnimationCallbackModeProcess {
        ord: 0i32
    };
    #[doc(alias = "ANIMATION_CALLBACK_MODE_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_PROCESS_IDLE`"]
    pub const IDLE: AnimationCallbackModeProcess = AnimationCallbackModeProcess {
        ord: 1i32
    };
    #[doc(alias = "ANIMATION_CALLBACK_MODE_PROCESS_MANUAL")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_PROCESS_MANUAL`"]
    pub const MANUAL: AnimationCallbackModeProcess = AnimationCallbackModeProcess {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AnimationCallbackModeProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnimationCallbackModeProcess") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnimationCallbackModeProcess {
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
            Self::PHYSICS => "PHYSICS", Self::IDLE => "IDLE", Self::MANUAL => "MANUAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AnimationCallbackModeProcess::PHYSICS, AnimationCallbackModeProcess::IDLE, AnimationCallbackModeProcess::MANUAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AnimationCallbackModeProcess >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PHYSICS", "ANIMATION_CALLBACK_MODE_PROCESS_PHYSICS", AnimationCallbackModeProcess::PHYSICS), crate::meta::inspect::EnumConstant::new("IDLE", "ANIMATION_CALLBACK_MODE_PROCESS_IDLE", AnimationCallbackModeProcess::IDLE), crate::meta::inspect::EnumConstant::new("MANUAL", "ANIMATION_CALLBACK_MODE_PROCESS_MANUAL", AnimationCallbackModeProcess::MANUAL)]
        }
    }
}
impl crate::meta::GodotConvert for AnimationCallbackModeProcess {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AnimationCallbackModeProcess {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnimationCallbackModeProcess {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnimationCallbackModeMethod {
    ord: i32
}
impl AnimationCallbackModeMethod {
    #[doc(alias = "ANIMATION_CALLBACK_MODE_METHOD_DEFERRED")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_METHOD_DEFERRED`"]
    pub const DEFERRED: AnimationCallbackModeMethod = AnimationCallbackModeMethod {
        ord: 0i32
    };
    #[doc(alias = "ANIMATION_CALLBACK_MODE_METHOD_IMMEDIATE")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_METHOD_IMMEDIATE`"]
    pub const IMMEDIATE: AnimationCallbackModeMethod = AnimationCallbackModeMethod {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for AnimationCallbackModeMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnimationCallbackModeMethod") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnimationCallbackModeMethod {
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
            Self::DEFERRED => "DEFERRED", Self::IMMEDIATE => "IMMEDIATE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AnimationCallbackModeMethod::DEFERRED, AnimationCallbackModeMethod::IMMEDIATE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AnimationCallbackModeMethod >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFERRED", "ANIMATION_CALLBACK_MODE_METHOD_DEFERRED", AnimationCallbackModeMethod::DEFERRED), crate::meta::inspect::EnumConstant::new("IMMEDIATE", "ANIMATION_CALLBACK_MODE_METHOD_IMMEDIATE", AnimationCallbackModeMethod::IMMEDIATE)]
        }
    }
}
impl crate::meta::GodotConvert for AnimationCallbackModeMethod {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AnimationCallbackModeMethod {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnimationCallbackModeMethod {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AnimationCallbackModeDiscrete {
    ord: i32
}
impl AnimationCallbackModeDiscrete {
    #[doc(alias = "ANIMATION_CALLBACK_MODE_DISCRETE_DOMINANT")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_DISCRETE_DOMINANT`"]
    pub const DOMINANT: AnimationCallbackModeDiscrete = AnimationCallbackModeDiscrete {
        ord: 0i32
    };
    #[doc(alias = "ANIMATION_CALLBACK_MODE_DISCRETE_RECESSIVE")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_DISCRETE_RECESSIVE`"]
    pub const RECESSIVE: AnimationCallbackModeDiscrete = AnimationCallbackModeDiscrete {
        ord: 1i32
    };
    #[doc(alias = "ANIMATION_CALLBACK_MODE_DISCRETE_FORCE_CONTINUOUS")]
    #[doc = "Godot enumerator name: `ANIMATION_CALLBACK_MODE_DISCRETE_FORCE_CONTINUOUS`"]
    pub const FORCE_CONTINUOUS: AnimationCallbackModeDiscrete = AnimationCallbackModeDiscrete {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AnimationCallbackModeDiscrete {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AnimationCallbackModeDiscrete") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AnimationCallbackModeDiscrete {
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
            Self::DOMINANT => "DOMINANT", Self::RECESSIVE => "RECESSIVE", Self::FORCE_CONTINUOUS => "FORCE_CONTINUOUS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AnimationCallbackModeDiscrete::DOMINANT, AnimationCallbackModeDiscrete::RECESSIVE, AnimationCallbackModeDiscrete::FORCE_CONTINUOUS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AnimationCallbackModeDiscrete >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DOMINANT", "ANIMATION_CALLBACK_MODE_DISCRETE_DOMINANT", AnimationCallbackModeDiscrete::DOMINANT), crate::meta::inspect::EnumConstant::new("RECESSIVE", "ANIMATION_CALLBACK_MODE_DISCRETE_RECESSIVE", AnimationCallbackModeDiscrete::RECESSIVE), crate::meta::inspect::EnumConstant::new("FORCE_CONTINUOUS", "ANIMATION_CALLBACK_MODE_DISCRETE_FORCE_CONTINUOUS", AnimationCallbackModeDiscrete::FORCE_CONTINUOUS)]
        }
    }
}
impl crate::meta::GodotConvert for AnimationCallbackModeDiscrete {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AnimationCallbackModeDiscrete {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AnimationCallbackModeDiscrete {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AnimationMixer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`AnimationMixer`][crate::classes::AnimationMixer] class."]
    pub struct SignalsOfAnimationMixer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfAnimationMixer < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn animation_list_changed(&mut self) -> SigAnimationListChanged < 'c, C > {
            SigAnimationListChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "animation_list_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn animation_libraries_updated(&mut self) -> SigAnimationLibrariesUpdated < 'c, C > {
            SigAnimationLibrariesUpdated {
                typed: TypedSignal::extract(&mut self.__internal_obj, "animation_libraries_updated")
            }
        }
        #[doc = "Signature: `(anim_name: StringName)`"]
        pub fn animation_finished(&mut self) -> SigAnimationFinished < 'c, C > {
            SigAnimationFinished {
                typed: TypedSignal::extract(&mut self.__internal_obj, "animation_finished")
            }
        }
        #[doc = "Signature: `(anim_name: StringName)`"]
        pub fn animation_started(&mut self) -> SigAnimationStarted < 'c, C > {
            SigAnimationStarted {
                typed: TypedSignal::extract(&mut self.__internal_obj, "animation_started")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn caches_cleared(&mut self) -> SigCachesCleared < 'c, C > {
            SigCachesCleared {
                typed: TypedSignal::extract(&mut self.__internal_obj, "caches_cleared")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn mixer_applied(&mut self) -> SigMixerApplied < 'c, C > {
            SigMixerApplied {
                typed: TypedSignal::extract(&mut self.__internal_obj, "mixer_applied")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn mixer_updated(&mut self) -> SigMixerUpdated < 'c, C > {
            SigMixerUpdated {
                typed: TypedSignal::extract(&mut self.__internal_obj, "mixer_updated")
            }
        }
    }
    type TypedSigAnimationListChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigAnimationListChanged < 'c, C: WithSignals > {
        typed: TypedSigAnimationListChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationListChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationListChanged < 'c, C > {
        type Target = TypedSigAnimationListChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationListChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAnimationLibrariesUpdated < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigAnimationLibrariesUpdated < 'c, C: WithSignals > {
        typed: TypedSigAnimationLibrariesUpdated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationLibrariesUpdated < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationLibrariesUpdated < 'c, C > {
        type Target = TypedSigAnimationLibrariesUpdated < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationLibrariesUpdated < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAnimationFinished < 'c, C > = TypedSignal < 'c, C, (StringName,) >;
    pub struct SigAnimationFinished < 'c, C: WithSignals > {
        typed: TypedSigAnimationFinished < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationFinished < 'c, C > {
        pub fn emit(&mut self, anim_name: StringName,) {
            self.typed.emit_tuple((anim_name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationFinished < 'c, C > {
        type Target = TypedSigAnimationFinished < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationFinished < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAnimationStarted < 'c, C > = TypedSignal < 'c, C, (StringName,) >;
    pub struct SigAnimationStarted < 'c, C: WithSignals > {
        typed: TypedSigAnimationStarted < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationStarted < 'c, C > {
        pub fn emit(&mut self, anim_name: StringName,) {
            self.typed.emit_tuple((anim_name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationStarted < 'c, C > {
        type Target = TypedSigAnimationStarted < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationStarted < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCachesCleared < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigCachesCleared < 'c, C: WithSignals > {
        typed: TypedSigCachesCleared < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCachesCleared < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCachesCleared < 'c, C > {
        type Target = TypedSigCachesCleared < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCachesCleared < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMixerApplied < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigMixerApplied < 'c, C: WithSignals > {
        typed: TypedSigMixerApplied < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMixerApplied < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMixerApplied < 'c, C > {
        type Target = TypedSigMixerApplied < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMixerApplied < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMixerUpdated < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigMixerUpdated < 'c, C: WithSignals > {
        typed: TypedSigMixerUpdated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMixerUpdated < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMixerUpdated < 'c, C > {
        type Target = TypedSigMixerUpdated < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMixerUpdated < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for AnimationMixer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAnimationMixer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfAnimationMixer < 'c, C > {
        type Target = < < AnimationMixer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = AnimationMixer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfAnimationMixer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = AnimationMixer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}