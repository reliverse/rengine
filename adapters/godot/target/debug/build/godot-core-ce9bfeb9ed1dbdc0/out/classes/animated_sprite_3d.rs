#![doc = "Sidecar module for class [`AnimatedSprite3D`][crate::classes::AnimatedSprite3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimatedSprite3D` enums](https://docs.godotengine.org/en/stable/classes/class_animatedsprite3d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AnimatedSprite3D.`\n\nInherits [`SpriteBase3D`][crate::classes::SpriteBase3D].\n\nRelated symbols:\n\n* [`animated_sprite_3d`][crate::classes::animated_sprite_3d]: sidecar module with related enum/flag types\n* [`IAnimatedSprite3D`][crate::classes::IAnimatedSprite3D]: virtual methods\n* [`SignalsOfAnimatedSprite3D`][crate::classes::animated_sprite_3d::SignalsOfAnimatedSprite3D]: signal collection\n\n\nSee also [Godot docs for `AnimatedSprite3D`](https://docs.godotengine.org/en/stable/classes/class_animatedsprite3d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`AnimatedSprite3D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AnimatedSprite3D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AnimatedSprite3D`][crate::classes::AnimatedSprite3D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`ISpriteBase3D`~~ > [`IGeometryInstance3D`][crate::classes::IGeometryInstance3D] > [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `AnimatedSprite3D` methods](https://docs.godotengine.org/en/stable/classes/class_animatedsprite3d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAnimatedSprite3D: crate::obj::GodotClass < Base = AnimatedSprite3D > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AnimatedSprite3D {
        pub fn set_sprite_frames(&mut self, sprite_frames: impl AsArg < Option < Gd < crate::classes::SpriteFrames >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::SpriteFrames > > >,);
            let args = (sprite_frames.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "set_sprite_frames", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sprite_frames(&self,) -> Option < Gd < crate::classes::SpriteFrames > > {
            type CallRet = Option < Gd < crate::classes::SpriteFrames > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "get_sprite_frames", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_animation(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "set_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_animation(&self,) -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "get_animation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autoplay(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "set_autoplay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_autoplay(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "get_autoplay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_playing(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(148usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "is_playing", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn play_full(&mut self, name: CowArg < StringName >, custom_speed: f32, from_end: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f32, bool,);
            let args = (name, custom_speed, from_end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(149usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "play", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::play_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn play(&mut self,) {
            self.play_ex() . done()
        }
        #[inline]
        pub fn play_ex < 'a > (&'a mut self,) -> ExPlay < 'a > {
            ExPlay::new(self,)
        }
        pub(crate) fn play_backwards_full(&mut self, name: CowArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "play_backwards", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::play_backwards_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn play_backwards(&mut self,) {
            self.play_backwards_ex() . done()
        }
        #[inline]
        pub fn play_backwards_ex < 'a > (&'a mut self,) -> ExPlayBackwards < 'a > {
            ExPlayBackwards::new(self,)
        }
        pub fn pause(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "pause", Some(self.__validated_obj()), args,)
            }
        }
        pub fn stop(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "stop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_frame(&mut self, frame: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (frame,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "set_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frame(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "get_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_frame_progress(&mut self, progress: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (progress,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "set_frame_progress", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frame_progress(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "get_frame_progress", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_frame_and_progress(&mut self, frame: i32, progress: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (frame, progress,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "set_frame_and_progress", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_speed_scale(&mut self, speed_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (speed_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "set_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_speed_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "get_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_playing_speed(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AnimatedSprite3D", "get_playing_speed", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AnimatedSprite3D {
        type Base = crate::classes::SpriteBase3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AnimatedSprite3D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AnimatedSprite3D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::SpriteBase3D > for AnimatedSprite3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::GeometryInstance3D > for AnimatedSprite3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for AnimatedSprite3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for AnimatedSprite3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for AnimatedSprite3D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AnimatedSprite3D {
        
    }
    impl crate::obj::cap::GodotDefault for AnimatedSprite3D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AnimatedSprite3D {
        type Target = crate::classes::SpriteBase3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AnimatedSprite3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AnimatedSprite3D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AnimatedSprite3D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AnimatedSprite3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::SpriteBase3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::GeometryInstance3D > for $Class {
                
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
#[doc = "Default-param extender for [`AnimatedSprite3D::play_ex`][super::AnimatedSprite3D::play_ex]."]
#[must_use]
pub struct ExPlay < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimatedSprite3D, name: CowArg < 'a, StringName >, custom_speed: f32, from_end: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlay < 'a > {
    fn new(surround_object: &'a mut re_export::AnimatedSprite3D,) -> Self {
        let name = StringName::from("");
        let custom_speed = 1f32;
        let from_end = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name), custom_speed: custom_speed, from_end: from_end,
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < StringName > + 'a) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn custom_speed(self, custom_speed: f32) -> Self {
        Self {
            custom_speed: custom_speed, .. self
        }
    }
    #[inline]
    pub fn from_end(self, from_end: bool) -> Self {
        Self {
            from_end: from_end, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, custom_speed, from_end,
        }
        = self;
        re_export::AnimatedSprite3D::play_full(surround_object, name, custom_speed, from_end,)
    }
}
#[doc = "Default-param extender for [`AnimatedSprite3D::play_backwards_ex`][super::AnimatedSprite3D::play_backwards_ex]."]
#[must_use]
pub struct ExPlayBackwards < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AnimatedSprite3D, name: CowArg < 'a, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPlayBackwards < 'a > {
    fn new(surround_object: &'a mut re_export::AnimatedSprite3D,) -> Self {
        let name = StringName::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: CowArg::Owned(name),
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < StringName > + 'a) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name,
        }
        = self;
        re_export::AnimatedSprite3D::play_backwards_full(surround_object, name,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AnimatedSprite3D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`AnimatedSprite3D`][crate::classes::AnimatedSprite3D] class."]
    pub struct SignalsOfAnimatedSprite3D < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfAnimatedSprite3D < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn sprite_frames_changed(&mut self) -> SigSpriteFramesChanged < 'c, C > {
            SigSpriteFramesChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "sprite_frames_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn animation_changed(&mut self) -> SigAnimationChanged < 'c, C > {
            SigAnimationChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "animation_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn frame_changed(&mut self) -> SigFrameChanged < 'c, C > {
            SigFrameChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "frame_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn animation_looped(&mut self) -> SigAnimationLooped < 'c, C > {
            SigAnimationLooped {
                typed: TypedSignal::extract(&mut self.__internal_obj, "animation_looped")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn animation_finished(&mut self) -> SigAnimationFinished < 'c, C > {
            SigAnimationFinished {
                typed: TypedSignal::extract(&mut self.__internal_obj, "animation_finished")
            }
        }
    }
    type TypedSigSpriteFramesChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSpriteFramesChanged < 'c, C: WithSignals > {
        typed: TypedSigSpriteFramesChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSpriteFramesChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSpriteFramesChanged < 'c, C > {
        type Target = TypedSigSpriteFramesChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSpriteFramesChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAnimationChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigAnimationChanged < 'c, C: WithSignals > {
        typed: TypedSigAnimationChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationChanged < 'c, C > {
        type Target = TypedSigAnimationChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFrameChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFrameChanged < 'c, C: WithSignals > {
        typed: TypedSigFrameChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFrameChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFrameChanged < 'c, C > {
        type Target = TypedSigFrameChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFrameChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAnimationLooped < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigAnimationLooped < 'c, C: WithSignals > {
        typed: TypedSigAnimationLooped < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationLooped < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigAnimationLooped < 'c, C > {
        type Target = TypedSigAnimationLooped < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigAnimationLooped < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigAnimationFinished < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigAnimationFinished < 'c, C: WithSignals > {
        typed: TypedSigAnimationFinished < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigAnimationFinished < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
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
    use crate::obj::WithSignals;
    impl WithSignals for AnimatedSprite3D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAnimatedSprite3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfAnimatedSprite3D < 'c, C > {
        type Target = < < AnimatedSprite3D as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = AnimatedSprite3D;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfAnimatedSprite3D < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = AnimatedSprite3D;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}