#![doc = "Sidecar module for class [`VisualShaderNodeTexture`][crate::classes::VisualShaderNodeTexture].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeTexture` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetexture.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeTexture.`\n\nInherits [`VisualShaderNode`][crate::classes::VisualShaderNode].\n\nRelated symbols:\n\n* [`visual_shader_node_texture`][crate::classes::visual_shader_node_texture]: sidecar module with related enum/flag types\n* [`IVisualShaderNodeTexture`][crate::classes::IVisualShaderNodeTexture]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeTexture`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetexture.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`VisualShaderNodeTexture::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeTexture {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`VisualShaderNodeTexture`][crate::classes::VisualShaderNodeTexture].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IVisualShaderNode`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `VisualShaderNodeTexture` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetexture.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVisualShaderNodeTexture: crate::obj::GodotClass < Base = VisualShaderNodeTexture > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl VisualShaderNodeTexture {
        pub fn set_source(&mut self, value: crate::classes::visual_shader_node_texture::Source,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_texture::Source,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10786usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTexture", "set_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_source(&self,) -> crate::classes::visual_shader_node_texture::Source {
            type CallRet = crate::classes::visual_shader_node_texture::Source;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10787usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTexture", "get_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture(&mut self, value: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (value.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10788usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTexture", "set_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10789usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTexture", "get_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_type(&mut self, value: crate::classes::visual_shader_node_texture::TextureType,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_texture::TextureType,);
            let args = (value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10790usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTexture", "set_texture_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_type(&self,) -> crate::classes::visual_shader_node_texture::TextureType {
            type CallRet = crate::classes::visual_shader_node_texture::TextureType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10791usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTexture", "get_texture_type", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeTexture {
        type Base = crate::classes::VisualShaderNode;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VisualShaderNodeTexture"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeTexture {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeTexture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeTexture {
        
    }
    impl crate::obj::cap::GodotDefault for VisualShaderNodeTexture {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VisualShaderNodeTexture {
        type Target = crate::classes::VisualShaderNode;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeTexture {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VisualShaderNodeTexture`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisualShaderNodeTexture__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNodeTexture > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualShaderNode > for $Class {
                
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
pub struct Source {
    ord: i32
}
impl Source {
    #[doc(alias = "SOURCE_TEXTURE")]
    #[doc = "Godot enumerator name: `SOURCE_TEXTURE`"]
    pub const TEXTURE: Source = Source {
        ord: 0i32
    };
    #[doc(alias = "SOURCE_SCREEN")]
    #[doc = "Godot enumerator name: `SOURCE_SCREEN`"]
    pub const SCREEN: Source = Source {
        ord: 1i32
    };
    pub const SOURCE_2D_TEXTURE: Source = Source {
        ord: 2i32
    };
    pub const SOURCE_2D_NORMAL: Source = Source {
        ord: 3i32
    };
    #[doc(alias = "SOURCE_DEPTH")]
    #[doc = "Godot enumerator name: `SOURCE_DEPTH`"]
    pub const DEPTH: Source = Source {
        ord: 4i32
    };
    #[doc(alias = "SOURCE_PORT")]
    #[doc = "Godot enumerator name: `SOURCE_PORT`"]
    pub const PORT: Source = Source {
        ord: 5i32
    };
    pub const SOURCE_3D_NORMAL: Source = Source {
        ord: 6i32
    };
    #[doc(alias = "SOURCE_ROUGHNESS")]
    #[doc = "Godot enumerator name: `SOURCE_ROUGHNESS`"]
    pub const ROUGHNESS: Source = Source {
        ord: 7i32
    };
    #[doc(alias = "SOURCE_MAX")]
    #[doc = "Godot enumerator name: `SOURCE_MAX`"]
    pub const MAX: Source = Source {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Source") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Source {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::TEXTURE => "TEXTURE", Self::SCREEN => "SCREEN", Self::SOURCE_2D_TEXTURE => "SOURCE_2D_TEXTURE", Self::SOURCE_2D_NORMAL => "SOURCE_2D_NORMAL", Self::DEPTH => "DEPTH", Self::PORT => "PORT", Self::SOURCE_3D_NORMAL => "SOURCE_3D_NORMAL", Self::ROUGHNESS => "ROUGHNESS", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Source::TEXTURE, Source::SCREEN, Source::SOURCE_2D_TEXTURE, Source::SOURCE_2D_NORMAL, Source::DEPTH, Source::PORT, Source::SOURCE_3D_NORMAL, Source::ROUGHNESS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Source >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TEXTURE", "SOURCE_TEXTURE", Source::TEXTURE), crate::meta::inspect::EnumConstant::new("SCREEN", "SOURCE_SCREEN", Source::SCREEN), crate::meta::inspect::EnumConstant::new("SOURCE_2D_TEXTURE", "SOURCE_2D_TEXTURE", Source::SOURCE_2D_TEXTURE), crate::meta::inspect::EnumConstant::new("SOURCE_2D_NORMAL", "SOURCE_2D_NORMAL", Source::SOURCE_2D_NORMAL), crate::meta::inspect::EnumConstant::new("DEPTH", "SOURCE_DEPTH", Source::DEPTH), crate::meta::inspect::EnumConstant::new("PORT", "SOURCE_PORT", Source::PORT), crate::meta::inspect::EnumConstant::new("SOURCE_3D_NORMAL", "SOURCE_3D_NORMAL", Source::SOURCE_3D_NORMAL), crate::meta::inspect::EnumConstant::new("ROUGHNESS", "SOURCE_ROUGHNESS", Source::ROUGHNESS), crate::meta::inspect::EnumConstant::new("MAX", "SOURCE_MAX", Source::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Source {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for Source {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Source {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Source {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureType {
    ord: i32
}
impl TextureType {
    #[doc(alias = "TYPE_DATA")]
    #[doc = "Godot enumerator name: `TYPE_DATA`"]
    pub const DATA: TextureType = TextureType {
        ord: 0i32
    };
    #[doc(alias = "TYPE_COLOR")]
    #[doc = "Godot enumerator name: `TYPE_COLOR`"]
    pub const COLOR: TextureType = TextureType {
        ord: 1i32
    };
    #[doc(alias = "TYPE_NORMAL_MAP")]
    #[doc = "Godot enumerator name: `TYPE_NORMAL_MAP`"]
    pub const NORMAL_MAP: TextureType = TextureType {
        ord: 2i32
    };
    #[doc(alias = "TYPE_MAX")]
    #[doc = "Godot enumerator name: `TYPE_MAX`"]
    pub const MAX: TextureType = TextureType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TextureType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
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
            Self::DATA => "DATA", Self::COLOR => "COLOR", Self::NORMAL_MAP => "NORMAL_MAP", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureType::DATA, TextureType::COLOR, TextureType::NORMAL_MAP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DATA", "TYPE_DATA", TextureType::DATA), crate::meta::inspect::EnumConstant::new("COLOR", "TYPE_COLOR", TextureType::COLOR), crate::meta::inspect::EnumConstant::new("NORMAL_MAP", "TYPE_NORMAL_MAP", TextureType::NORMAL_MAP), crate::meta::inspect::EnumConstant::new("MAX", "TYPE_MAX", TextureType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for TextureType {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for TextureType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VisualShaderNodeTexture;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VisualShaderNodeTexture {
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