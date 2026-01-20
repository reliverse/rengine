#![doc = "Sidecar module for class [`TextureRect`][crate::classes::TextureRect].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextureRect` enums](https://docs.godotengine.org/en/stable/classes/class_texturerect.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TextureRect.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`texture_rect`][crate::classes::texture_rect]: sidecar module with related enum/flag types\n* [`ITextureRect`][crate::classes::ITextureRect]: virtual methods\n\n\nSee also [Godot docs for `TextureRect`](https://docs.godotengine.org/en/stable/classes/class_texturerect.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`TextureRect::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TextureRect {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TextureRect`][crate::classes::TextureRect].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `TextureRect` methods](https://docs.godotengine.org/en/stable/classes/class_texturerect.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITextureRect: crate::obj::GodotClass < Base = TextureRect > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ControlNotification) {
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
        fn has_point(&self, point: Vector2,) -> bool {
            unimplemented !()
        }
        fn structured_text_parser(&self, args: VarArray, text: GString,) -> Array < Vector3i > {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_tooltip(&self, at_position: Vector2,) -> GString {
            unimplemented !()
        }
        fn get_drag_data(&mut self, at_position: Vector2,) -> Variant {
            unimplemented !()
        }
        fn can_drop_data(&self, at_position: Vector2, data: Variant,) -> bool {
            unimplemented !()
        }
        fn drop_data(&mut self, at_position: Vector2, data: Variant,) {
            unimplemented !()
        }
        fn make_custom_tooltip(&self, for_text: GString,) -> Option < Gd < crate::classes::Object > > {
            unimplemented !()
        }
        fn accessibility_get_contextual_info(&self,) -> GString {
            unimplemented !()
        }
        fn get_accessibility_container_name(&self, node: Option < Gd < crate::classes::Node > >,) -> GString {
            unimplemented !()
        }
        fn gui_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
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
    impl TextureRect {
        pub fn set_texture(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9571usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureRect", "set_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9572usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureRect", "get_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_expand_mode(&mut self, expand_mode: crate::classes::texture_rect::ExpandMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::texture_rect::ExpandMode,);
            let args = (expand_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9573usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureRect", "set_expand_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_expand_mode(&self,) -> crate::classes::texture_rect::ExpandMode {
            type CallRet = crate::classes::texture_rect::ExpandMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9574usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureRect", "get_expand_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flip_h(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9575usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureRect", "set_flip_h", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_flipped_h(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9576usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureRect", "is_flipped_h", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_flip_v(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9577usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureRect", "set_flip_v", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_flipped_v(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9578usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureRect", "is_flipped_v", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stretch_mode(&mut self, stretch_mode: crate::classes::texture_rect::StretchMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::texture_rect::StretchMode,);
            let args = (stretch_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9579usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureRect", "set_stretch_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stretch_mode(&self,) -> crate::classes::texture_rect::StretchMode {
            type CallRet = crate::classes::texture_rect::StretchMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9580usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TextureRect", "get_stretch_mode", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TextureRect {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TextureRect"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TextureRect {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for TextureRect {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for TextureRect {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for TextureRect {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TextureRect {
        
    }
    impl crate::obj::cap::GodotDefault for TextureRect {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TextureRect {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TextureRect {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TextureRect`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TextureRect__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TextureRect > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Control > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
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
pub struct ExpandMode {
    ord: i32
}
impl ExpandMode {
    #[doc(alias = "EXPAND_KEEP_SIZE")]
    #[doc = "Godot enumerator name: `EXPAND_KEEP_SIZE`"]
    pub const KEEP_SIZE: ExpandMode = ExpandMode {
        ord: 0i32
    };
    #[doc(alias = "EXPAND_IGNORE_SIZE")]
    #[doc = "Godot enumerator name: `EXPAND_IGNORE_SIZE`"]
    pub const IGNORE_SIZE: ExpandMode = ExpandMode {
        ord: 1i32
    };
    #[doc(alias = "EXPAND_FIT_WIDTH")]
    #[doc = "Godot enumerator name: `EXPAND_FIT_WIDTH`"]
    pub const FIT_WIDTH: ExpandMode = ExpandMode {
        ord: 2i32
    };
    #[doc(alias = "EXPAND_FIT_WIDTH_PROPORTIONAL")]
    #[doc = "Godot enumerator name: `EXPAND_FIT_WIDTH_PROPORTIONAL`"]
    pub const FIT_WIDTH_PROPORTIONAL: ExpandMode = ExpandMode {
        ord: 3i32
    };
    #[doc(alias = "EXPAND_FIT_HEIGHT")]
    #[doc = "Godot enumerator name: `EXPAND_FIT_HEIGHT`"]
    pub const FIT_HEIGHT: ExpandMode = ExpandMode {
        ord: 4i32
    };
    #[doc(alias = "EXPAND_FIT_HEIGHT_PROPORTIONAL")]
    #[doc = "Godot enumerator name: `EXPAND_FIT_HEIGHT_PROPORTIONAL`"]
    pub const FIT_HEIGHT_PROPORTIONAL: ExpandMode = ExpandMode {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for ExpandMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ExpandMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ExpandMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::KEEP_SIZE => "KEEP_SIZE", Self::IGNORE_SIZE => "IGNORE_SIZE", Self::FIT_WIDTH => "FIT_WIDTH", Self::FIT_WIDTH_PROPORTIONAL => "FIT_WIDTH_PROPORTIONAL", Self::FIT_HEIGHT => "FIT_HEIGHT", Self::FIT_HEIGHT_PROPORTIONAL => "FIT_HEIGHT_PROPORTIONAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ExpandMode::KEEP_SIZE, ExpandMode::IGNORE_SIZE, ExpandMode::FIT_WIDTH, ExpandMode::FIT_WIDTH_PROPORTIONAL, ExpandMode::FIT_HEIGHT, ExpandMode::FIT_HEIGHT_PROPORTIONAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ExpandMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("KEEP_SIZE", "EXPAND_KEEP_SIZE", ExpandMode::KEEP_SIZE), crate::meta::inspect::EnumConstant::new("IGNORE_SIZE", "EXPAND_IGNORE_SIZE", ExpandMode::IGNORE_SIZE), crate::meta::inspect::EnumConstant::new("FIT_WIDTH", "EXPAND_FIT_WIDTH", ExpandMode::FIT_WIDTH), crate::meta::inspect::EnumConstant::new("FIT_WIDTH_PROPORTIONAL", "EXPAND_FIT_WIDTH_PROPORTIONAL", ExpandMode::FIT_WIDTH_PROPORTIONAL), crate::meta::inspect::EnumConstant::new("FIT_HEIGHT", "EXPAND_FIT_HEIGHT", ExpandMode::FIT_HEIGHT), crate::meta::inspect::EnumConstant::new("FIT_HEIGHT_PROPORTIONAL", "EXPAND_FIT_HEIGHT_PROPORTIONAL", ExpandMode::FIT_HEIGHT_PROPORTIONAL)]
        }
    }
}
impl crate::meta::GodotConvert for ExpandMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ExpandMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ExpandMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct StretchMode {
    ord: i32
}
impl StretchMode {
    #[doc(alias = "STRETCH_SCALE")]
    #[doc = "Godot enumerator name: `STRETCH_SCALE`"]
    pub const SCALE: StretchMode = StretchMode {
        ord: 0i32
    };
    #[doc(alias = "STRETCH_TILE")]
    #[doc = "Godot enumerator name: `STRETCH_TILE`"]
    pub const TILE: StretchMode = StretchMode {
        ord: 1i32
    };
    #[doc(alias = "STRETCH_KEEP")]
    #[doc = "Godot enumerator name: `STRETCH_KEEP`"]
    pub const KEEP: StretchMode = StretchMode {
        ord: 2i32
    };
    #[doc(alias = "STRETCH_KEEP_CENTERED")]
    #[doc = "Godot enumerator name: `STRETCH_KEEP_CENTERED`"]
    pub const KEEP_CENTERED: StretchMode = StretchMode {
        ord: 3i32
    };
    #[doc(alias = "STRETCH_KEEP_ASPECT")]
    #[doc = "Godot enumerator name: `STRETCH_KEEP_ASPECT`"]
    pub const KEEP_ASPECT: StretchMode = StretchMode {
        ord: 4i32
    };
    #[doc(alias = "STRETCH_KEEP_ASPECT_CENTERED")]
    #[doc = "Godot enumerator name: `STRETCH_KEEP_ASPECT_CENTERED`"]
    pub const KEEP_ASPECT_CENTERED: StretchMode = StretchMode {
        ord: 5i32
    };
    #[doc(alias = "STRETCH_KEEP_ASPECT_COVERED")]
    #[doc = "Godot enumerator name: `STRETCH_KEEP_ASPECT_COVERED`"]
    pub const KEEP_ASPECT_COVERED: StretchMode = StretchMode {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for StretchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("StretchMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for StretchMode {
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
            Self::SCALE => "SCALE", Self::TILE => "TILE", Self::KEEP => "KEEP", Self::KEEP_CENTERED => "KEEP_CENTERED", Self::KEEP_ASPECT => "KEEP_ASPECT", Self::KEEP_ASPECT_CENTERED => "KEEP_ASPECT_CENTERED", Self::KEEP_ASPECT_COVERED => "KEEP_ASPECT_COVERED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[StretchMode::SCALE, StretchMode::TILE, StretchMode::KEEP, StretchMode::KEEP_CENTERED, StretchMode::KEEP_ASPECT, StretchMode::KEEP_ASPECT_CENTERED, StretchMode::KEEP_ASPECT_COVERED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < StretchMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SCALE", "STRETCH_SCALE", StretchMode::SCALE), crate::meta::inspect::EnumConstant::new("TILE", "STRETCH_TILE", StretchMode::TILE), crate::meta::inspect::EnumConstant::new("KEEP", "STRETCH_KEEP", StretchMode::KEEP), crate::meta::inspect::EnumConstant::new("KEEP_CENTERED", "STRETCH_KEEP_CENTERED", StretchMode::KEEP_CENTERED), crate::meta::inspect::EnumConstant::new("KEEP_ASPECT", "STRETCH_KEEP_ASPECT", StretchMode::KEEP_ASPECT), crate::meta::inspect::EnumConstant::new("KEEP_ASPECT_CENTERED", "STRETCH_KEEP_ASPECT_CENTERED", StretchMode::KEEP_ASPECT_CENTERED), crate::meta::inspect::EnumConstant::new("KEEP_ASPECT_COVERED", "STRETCH_KEEP_ASPECT_COVERED", StretchMode::KEEP_ASPECT_COVERED)]
        }
    }
}
impl crate::meta::GodotConvert for StretchMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for StretchMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for StretchMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TextureRect;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::control::SignalsOfControl;
    impl WithSignals for TextureRect {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfControl < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}