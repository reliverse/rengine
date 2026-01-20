#![doc = "Sidecar module for class [`VisualShaderNodeTextureParameter`][crate::classes::VisualShaderNodeTextureParameter].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeTextureParameter` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetextureparameter.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VisualShaderNodeTextureParameter.`\n\nInherits [`VisualShaderNodeParameter`][crate::classes::VisualShaderNodeParameter].\n\nRelated symbols:\n\n* [`visual_shader_node_texture_parameter`][crate::classes::visual_shader_node_texture_parameter]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `VisualShaderNodeTextureParameter`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetextureparameter.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<VisualShaderNodeTextureParameter>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VisualShaderNodeTextureParameter {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl VisualShaderNodeTextureParameter {
        pub fn set_texture_type(&mut self, type_: crate::classes::visual_shader_node_texture_parameter::TextureType,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_texture_parameter::TextureType,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10796usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "set_texture_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_type(&self,) -> crate::classes::visual_shader_node_texture_parameter::TextureType {
            type CallRet = crate::classes::visual_shader_node_texture_parameter::TextureType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10797usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "get_texture_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color_default(&mut self, color: crate::classes::visual_shader_node_texture_parameter::ColorDefault,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_texture_parameter::ColorDefault,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10798usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "set_color_default", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color_default(&self,) -> crate::classes::visual_shader_node_texture_parameter::ColorDefault {
            type CallRet = crate::classes::visual_shader_node_texture_parameter::ColorDefault;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10799usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "get_color_default", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_filter(&mut self, filter: crate::classes::visual_shader_node_texture_parameter::TextureFilter,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_texture_parameter::TextureFilter,);
            let args = (filter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10800usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "set_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_filter(&self,) -> crate::classes::visual_shader_node_texture_parameter::TextureFilter {
            type CallRet = crate::classes::visual_shader_node_texture_parameter::TextureFilter;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10801usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "get_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_repeat(&mut self, repeat: crate::classes::visual_shader_node_texture_parameter::TextureRepeat,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_texture_parameter::TextureRepeat,);
            let args = (repeat,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10802usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "set_texture_repeat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_repeat(&self,) -> crate::classes::visual_shader_node_texture_parameter::TextureRepeat {
            type CallRet = crate::classes::visual_shader_node_texture_parameter::TextureRepeat;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10803usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "get_texture_repeat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_source(&mut self, source: crate::classes::visual_shader_node_texture_parameter::TextureSource,) {
            type CallRet = ();
            type CallParams = (crate::classes::visual_shader_node_texture_parameter::TextureSource,);
            let args = (source,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10804usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "set_texture_source", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_source(&self,) -> crate::classes::visual_shader_node_texture_parameter::TextureSource {
            type CallRet = crate::classes::visual_shader_node_texture_parameter::TextureSource;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10805usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VisualShaderNodeTextureParameter", "get_texture_source", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for VisualShaderNodeTextureParameter {
        type Base = crate::classes::VisualShaderNodeParameter;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VisualShaderNodeTextureParameter"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VisualShaderNodeTextureParameter {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNodeParameter > for VisualShaderNodeTextureParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualShaderNode > for VisualShaderNodeTextureParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VisualShaderNodeTextureParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VisualShaderNodeTextureParameter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VisualShaderNodeTextureParameter {
        
    }
    impl std::ops::Deref for VisualShaderNodeTextureParameter {
        type Target = crate::classes::VisualShaderNodeParameter;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeTextureParameter {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VisualShaderNodeTextureParameter__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `VisualShaderNodeTextureParameter` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
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
    #[doc(alias = "TYPE_ANISOTROPY")]
    #[doc = "Godot enumerator name: `TYPE_ANISOTROPY`"]
    pub const ANISOTROPY: TextureType = TextureType {
        ord: 3i32
    };
    #[doc(alias = "TYPE_MAX")]
    #[doc = "Godot enumerator name: `TYPE_MAX`"]
    pub const MAX: TextureType = TextureType {
        ord: 4i32
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
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::DATA => "DATA", Self::COLOR => "COLOR", Self::NORMAL_MAP => "NORMAL_MAP", Self::ANISOTROPY => "ANISOTROPY", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureType::DATA, TextureType::COLOR, TextureType::NORMAL_MAP, TextureType::ANISOTROPY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DATA", "TYPE_DATA", TextureType::DATA), crate::meta::inspect::EnumConstant::new("COLOR", "TYPE_COLOR", TextureType::COLOR), crate::meta::inspect::EnumConstant::new("NORMAL_MAP", "TYPE_NORMAL_MAP", TextureType::NORMAL_MAP), crate::meta::inspect::EnumConstant::new("ANISOTROPY", "TYPE_ANISOTROPY", TextureType::ANISOTROPY), crate::meta::inspect::EnumConstant::new("MAX", "TYPE_MAX", TextureType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for TextureType {
    const ENUMERATOR_COUNT: usize = 4usize;
    
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ColorDefault {
    ord: i32
}
impl ColorDefault {
    #[doc(alias = "COLOR_DEFAULT_WHITE")]
    #[doc = "Godot enumerator name: `COLOR_DEFAULT_WHITE`"]
    pub const WHITE: ColorDefault = ColorDefault {
        ord: 0i32
    };
    #[doc(alias = "COLOR_DEFAULT_BLACK")]
    #[doc = "Godot enumerator name: `COLOR_DEFAULT_BLACK`"]
    pub const BLACK: ColorDefault = ColorDefault {
        ord: 1i32
    };
    #[doc(alias = "COLOR_DEFAULT_TRANSPARENT")]
    #[doc = "Godot enumerator name: `COLOR_DEFAULT_TRANSPARENT`"]
    pub const TRANSPARENT: ColorDefault = ColorDefault {
        ord: 2i32
    };
    #[doc(alias = "COLOR_DEFAULT_MAX")]
    #[doc = "Godot enumerator name: `COLOR_DEFAULT_MAX`"]
    pub const MAX: ColorDefault = ColorDefault {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ColorDefault {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ColorDefault") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ColorDefault {
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
            Self::WHITE => "WHITE", Self::BLACK => "BLACK", Self::TRANSPARENT => "TRANSPARENT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ColorDefault::WHITE, ColorDefault::BLACK, ColorDefault::TRANSPARENT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ColorDefault >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("WHITE", "COLOR_DEFAULT_WHITE", ColorDefault::WHITE), crate::meta::inspect::EnumConstant::new("BLACK", "COLOR_DEFAULT_BLACK", ColorDefault::BLACK), crate::meta::inspect::EnumConstant::new("TRANSPARENT", "COLOR_DEFAULT_TRANSPARENT", ColorDefault::TRANSPARENT), crate::meta::inspect::EnumConstant::new("MAX", "COLOR_DEFAULT_MAX", ColorDefault::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ColorDefault {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ColorDefault {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ColorDefault {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ColorDefault {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureFilter {
    ord: i32
}
impl TextureFilter {
    #[doc(alias = "FILTER_DEFAULT")]
    #[doc = "Godot enumerator name: `FILTER_DEFAULT`"]
    pub const DEFAULT: TextureFilter = TextureFilter {
        ord: 0i32
    };
    #[doc(alias = "FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `FILTER_NEAREST`"]
    pub const NEAREST: TextureFilter = TextureFilter {
        ord: 1i32
    };
    #[doc(alias = "FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `FILTER_LINEAR`"]
    pub const LINEAR: TextureFilter = TextureFilter {
        ord: 2i32
    };
    #[doc(alias = "FILTER_NEAREST_MIPMAP")]
    #[doc = "Godot enumerator name: `FILTER_NEAREST_MIPMAP`"]
    pub const NEAREST_MIPMAP: TextureFilter = TextureFilter {
        ord: 3i32
    };
    #[doc(alias = "FILTER_LINEAR_MIPMAP")]
    #[doc = "Godot enumerator name: `FILTER_LINEAR_MIPMAP`"]
    pub const LINEAR_MIPMAP: TextureFilter = TextureFilter {
        ord: 4i32
    };
    #[doc(alias = "FILTER_NEAREST_MIPMAP_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `FILTER_NEAREST_MIPMAP_ANISOTROPIC`"]
    pub const NEAREST_MIPMAP_ANISOTROPIC: TextureFilter = TextureFilter {
        ord: 5i32
    };
    #[doc(alias = "FILTER_LINEAR_MIPMAP_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `FILTER_LINEAR_MIPMAP_ANISOTROPIC`"]
    pub const LINEAR_MIPMAP_ANISOTROPIC: TextureFilter = TextureFilter {
        ord: 6i32
    };
    #[doc(alias = "FILTER_MAX")]
    #[doc = "Godot enumerator name: `FILTER_MAX`"]
    pub const MAX: TextureFilter = TextureFilter {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for TextureFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureFilter {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
            Self::DEFAULT => "DEFAULT", Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", Self::NEAREST_MIPMAP => "NEAREST_MIPMAP", Self::LINEAR_MIPMAP => "LINEAR_MIPMAP", Self::NEAREST_MIPMAP_ANISOTROPIC => "NEAREST_MIPMAP_ANISOTROPIC", Self::LINEAR_MIPMAP_ANISOTROPIC => "LINEAR_MIPMAP_ANISOTROPIC", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureFilter::DEFAULT, TextureFilter::NEAREST, TextureFilter::LINEAR, TextureFilter::NEAREST_MIPMAP, TextureFilter::LINEAR_MIPMAP, TextureFilter::NEAREST_MIPMAP_ANISOTROPIC, TextureFilter::LINEAR_MIPMAP_ANISOTROPIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "FILTER_DEFAULT", TextureFilter::DEFAULT), crate::meta::inspect::EnumConstant::new("NEAREST", "FILTER_NEAREST", TextureFilter::NEAREST), crate::meta::inspect::EnumConstant::new("LINEAR", "FILTER_LINEAR", TextureFilter::LINEAR), crate::meta::inspect::EnumConstant::new("NEAREST_MIPMAP", "FILTER_NEAREST_MIPMAP", TextureFilter::NEAREST_MIPMAP), crate::meta::inspect::EnumConstant::new("LINEAR_MIPMAP", "FILTER_LINEAR_MIPMAP", TextureFilter::LINEAR_MIPMAP), crate::meta::inspect::EnumConstant::new("NEAREST_MIPMAP_ANISOTROPIC", "FILTER_NEAREST_MIPMAP_ANISOTROPIC", TextureFilter::NEAREST_MIPMAP_ANISOTROPIC), crate::meta::inspect::EnumConstant::new("LINEAR_MIPMAP_ANISOTROPIC", "FILTER_LINEAR_MIPMAP_ANISOTROPIC", TextureFilter::LINEAR_MIPMAP_ANISOTROPIC), crate::meta::inspect::EnumConstant::new("MAX", "FILTER_MAX", TextureFilter::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for TextureFilter {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for TextureFilter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureRepeat {
    ord: i32
}
impl TextureRepeat {
    #[doc(alias = "REPEAT_DEFAULT")]
    #[doc = "Godot enumerator name: `REPEAT_DEFAULT`"]
    pub const DEFAULT: TextureRepeat = TextureRepeat {
        ord: 0i32
    };
    #[doc(alias = "REPEAT_ENABLED")]
    #[doc = "Godot enumerator name: `REPEAT_ENABLED`"]
    pub const ENABLED: TextureRepeat = TextureRepeat {
        ord: 1i32
    };
    #[doc(alias = "REPEAT_DISABLED")]
    #[doc = "Godot enumerator name: `REPEAT_DISABLED`"]
    pub const DISABLED: TextureRepeat = TextureRepeat {
        ord: 2i32
    };
    #[doc(alias = "REPEAT_MAX")]
    #[doc = "Godot enumerator name: `REPEAT_MAX`"]
    pub const MAX: TextureRepeat = TextureRepeat {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TextureRepeat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureRepeat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureRepeat {
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
            Self::DEFAULT => "DEFAULT", Self::ENABLED => "ENABLED", Self::DISABLED => "DISABLED", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureRepeat::DEFAULT, TextureRepeat::ENABLED, TextureRepeat::DISABLED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureRepeat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "REPEAT_DEFAULT", TextureRepeat::DEFAULT), crate::meta::inspect::EnumConstant::new("ENABLED", "REPEAT_ENABLED", TextureRepeat::ENABLED), crate::meta::inspect::EnumConstant::new("DISABLED", "REPEAT_DISABLED", TextureRepeat::DISABLED), crate::meta::inspect::EnumConstant::new("MAX", "REPEAT_MAX", TextureRepeat::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for TextureRepeat {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for TextureRepeat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureRepeat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureRepeat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureSource {
    ord: i32
}
impl TextureSource {
    #[doc(alias = "SOURCE_NONE")]
    #[doc = "Godot enumerator name: `SOURCE_NONE`"]
    pub const NONE: TextureSource = TextureSource {
        ord: 0i32
    };
    #[doc(alias = "SOURCE_SCREEN")]
    #[doc = "Godot enumerator name: `SOURCE_SCREEN`"]
    pub const SCREEN: TextureSource = TextureSource {
        ord: 1i32
    };
    #[doc(alias = "SOURCE_DEPTH")]
    #[doc = "Godot enumerator name: `SOURCE_DEPTH`"]
    pub const DEPTH: TextureSource = TextureSource {
        ord: 2i32
    };
    #[doc(alias = "SOURCE_NORMAL_ROUGHNESS")]
    #[doc = "Godot enumerator name: `SOURCE_NORMAL_ROUGHNESS`"]
    pub const NORMAL_ROUGHNESS: TextureSource = TextureSource {
        ord: 3i32
    };
    #[doc(alias = "SOURCE_MAX")]
    #[doc = "Godot enumerator name: `SOURCE_MAX`"]
    pub const MAX: TextureSource = TextureSource {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for TextureSource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureSource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureSource {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::NONE => "NONE", Self::SCREEN => "SCREEN", Self::DEPTH => "DEPTH", Self::NORMAL_ROUGHNESS => "NORMAL_ROUGHNESS", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureSource::NONE, TextureSource::SCREEN, TextureSource::DEPTH, TextureSource::NORMAL_ROUGHNESS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureSource >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "SOURCE_NONE", TextureSource::NONE), crate::meta::inspect::EnumConstant::new("SCREEN", "SOURCE_SCREEN", TextureSource::SCREEN), crate::meta::inspect::EnumConstant::new("DEPTH", "SOURCE_DEPTH", TextureSource::DEPTH), crate::meta::inspect::EnumConstant::new("NORMAL_ROUGHNESS", "SOURCE_NORMAL_ROUGHNESS", TextureSource::NORMAL_ROUGHNESS), crate::meta::inspect::EnumConstant::new("MAX", "SOURCE_MAX", TextureSource::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for TextureSource {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for TextureSource {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureSource {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VisualShaderNodeTextureParameter;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VisualShaderNodeTextureParameter {
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