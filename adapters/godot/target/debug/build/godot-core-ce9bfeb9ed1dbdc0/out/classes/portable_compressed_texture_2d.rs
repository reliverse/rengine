#![doc = "Sidecar module for class [`PortableCompressedTexture2D`][crate::classes::PortableCompressedTexture2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PortableCompressedTexture2D` enums](https://docs.godotengine.org/en/stable/classes/class_portablecompressedtexture2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PortableCompressedTexture2D.`\n\nInherits [`Texture2D`][crate::classes::Texture2D].\n\nRelated symbols:\n\n* [`portable_compressed_texture_2d`][crate::classes::portable_compressed_texture_2d]: sidecar module with related enum/flag types\n* [`IPortableCompressedTexture2D`][crate::classes::IPortableCompressedTexture2D]: virtual methods\n\n\nSee also [Godot docs for `PortableCompressedTexture2D`](https://docs.godotengine.org/en/stable/classes/class_portablecompressedtexture2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`PortableCompressedTexture2D::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PortableCompressedTexture2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`PortableCompressedTexture2D`][crate::classes::PortableCompressedTexture2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`ITexture2D`][crate::classes::ITexture2D] > [`ITexture`][crate::classes::ITexture] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `PortableCompressedTexture2D` methods](https://docs.godotengine.org/en/stable/classes/class_portablecompressedtexture2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPortableCompressedTexture2D: crate::obj::GodotClass < Base = PortableCompressedTexture2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_width(&self,) -> i32;
        fn get_height(&self,) -> i32;
        fn is_pixel_opaque(&self, x: i32, y: i32,) -> bool {
            unimplemented !()
        }
        fn has_alpha(&self,) -> bool {
            unimplemented !()
        }
        fn draw(&self, to_canvas_item: Rid, pos: Vector2, modulate: Color, transpose: bool,) {
            unimplemented !()
        }
        fn draw_rect(&self, to_canvas_item: Rid, rect: Rect2, tile: bool, modulate: Color, transpose: bool,) {
            unimplemented !()
        }
        fn draw_rect_region(&self, to_canvas_item: Rid, rect: Rect2, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,) {
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
    impl PortableCompressedTexture2D {
        pub(crate) fn create_from_image_full(&mut self, image: CowArg < Option < Gd < crate::classes::Image > > >, compression_mode: crate::classes::portable_compressed_texture_2d::CompressionMode, normal_map: bool, lossy_quality: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >, crate::classes::portable_compressed_texture_2d::CompressionMode, bool, f32,);
            let args = (image, compression_mode, normal_map, lossy_quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6826usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PortableCompressedTexture2D", "create_from_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_from_image_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_from_image(&mut self, image: impl AsArg < Option < Gd < crate::classes::Image >> >, compression_mode: crate::classes::portable_compressed_texture_2d::CompressionMode,) {
            self.create_from_image_ex(image, compression_mode,) . done()
        }
        #[inline]
        pub fn create_from_image_ex < 'a > (&'a mut self, image: impl AsArg < Option < Gd < crate::classes::Image >> > + 'a, compression_mode: crate::classes::portable_compressed_texture_2d::CompressionMode,) -> ExCreateFromImage < 'a > {
            ExCreateFromImage::new(self, image, compression_mode,)
        }
        pub fn get_format(&self,) -> crate::classes::image::Format {
            type CallRet = crate::classes::image::Format;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6827usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PortableCompressedTexture2D", "get_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_compression_mode(&self,) -> crate::classes::portable_compressed_texture_2d::CompressionMode {
            type CallRet = crate::classes::portable_compressed_texture_2d::CompressionMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6828usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PortableCompressedTexture2D", "get_compression_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_size_override(&mut self, size: Vector2,) {
            type CallRet = ();
            type CallParams = (Vector2,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6829usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PortableCompressedTexture2D", "set_size_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size_override(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6830usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PortableCompressedTexture2D", "get_size_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_keep_compressed_buffer(&mut self, keep: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (keep,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6831usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PortableCompressedTexture2D", "set_keep_compressed_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_keeping_compressed_buffer(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6832usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PortableCompressedTexture2D", "is_keeping_compressed_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_basisu_compressor_params(&mut self, uastc_level: i32, rdo_quality_loss: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (uastc_level, rdo_quality_loss,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6833usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PortableCompressedTexture2D", "set_basisu_compressor_params", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_keep_all_compressed_buffers(keep: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (keep,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6834usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PortableCompressedTexture2D", "set_keep_all_compressed_buffers", None, args,)
            }
        }
        pub fn is_keeping_all_compressed_buffers() -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6835usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PortableCompressedTexture2D", "is_keeping_all_compressed_buffers", None, args,)
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
    impl crate::obj::GodotClass for PortableCompressedTexture2D {
        type Base = crate::classes::Texture2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PortableCompressedTexture2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PortableCompressedTexture2D {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture2D > for PortableCompressedTexture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Texture > for PortableCompressedTexture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for PortableCompressedTexture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for PortableCompressedTexture2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PortableCompressedTexture2D {
        
    }
    impl crate::obj::cap::GodotDefault for PortableCompressedTexture2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PortableCompressedTexture2D {
        type Target = crate::classes::Texture2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PortableCompressedTexture2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PortableCompressedTexture2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PortableCompressedTexture2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PortableCompressedTexture2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Texture2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Texture > for $Class {
                
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
#[doc = "Default-param extender for [`PortableCompressedTexture2D::create_from_image_ex`][super::PortableCompressedTexture2D::create_from_image_ex]."]
#[must_use]
pub struct ExCreateFromImage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PortableCompressedTexture2D, image: CowArg < 'a, Option < Gd < crate::classes::Image > > >, compression_mode: crate::classes::portable_compressed_texture_2d::CompressionMode, normal_map: bool, lossy_quality: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateFromImage < 'a > {
    fn new(surround_object: &'a mut re_export::PortableCompressedTexture2D, image: impl AsArg < Option < Gd < crate::classes::Image >> > + 'a, compression_mode: crate::classes::portable_compressed_texture_2d::CompressionMode,) -> Self {
        let normal_map = false;
        let lossy_quality = 0.8f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, image: image.into_arg(), compression_mode: compression_mode, normal_map: normal_map, lossy_quality: lossy_quality,
        }
    }
    #[inline]
    pub fn normal_map(self, normal_map: bool) -> Self {
        Self {
            normal_map: normal_map, .. self
        }
    }
    #[inline]
    pub fn lossy_quality(self, lossy_quality: f32) -> Self {
        Self {
            lossy_quality: lossy_quality, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, image, compression_mode, normal_map, lossy_quality,
        }
        = self;
        re_export::PortableCompressedTexture2D::create_from_image_full(surround_object, image, compression_mode, normal_map, lossy_quality,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompressionMode {
    ord: i32
}
impl CompressionMode {
    #[doc(alias = "COMPRESSION_MODE_LOSSLESS")]
    #[doc = "Godot enumerator name: `COMPRESSION_MODE_LOSSLESS`"]
    pub const LOSSLESS: CompressionMode = CompressionMode {
        ord: 0i32
    };
    #[doc(alias = "COMPRESSION_MODE_LOSSY")]
    #[doc = "Godot enumerator name: `COMPRESSION_MODE_LOSSY`"]
    pub const LOSSY: CompressionMode = CompressionMode {
        ord: 1i32
    };
    #[doc(alias = "COMPRESSION_MODE_BASIS_UNIVERSAL")]
    #[doc = "Godot enumerator name: `COMPRESSION_MODE_BASIS_UNIVERSAL`"]
    pub const BASIS_UNIVERSAL: CompressionMode = CompressionMode {
        ord: 2i32
    };
    #[doc(alias = "COMPRESSION_MODE_S3TC")]
    #[doc = "Godot enumerator name: `COMPRESSION_MODE_S3TC`"]
    pub const S3TC: CompressionMode = CompressionMode {
        ord: 3i32
    };
    #[doc(alias = "COMPRESSION_MODE_ETC2")]
    #[doc = "Godot enumerator name: `COMPRESSION_MODE_ETC2`"]
    pub const ETC2: CompressionMode = CompressionMode {
        ord: 4i32
    };
    #[doc(alias = "COMPRESSION_MODE_BPTC")]
    #[doc = "Godot enumerator name: `COMPRESSION_MODE_BPTC`"]
    pub const BPTC: CompressionMode = CompressionMode {
        ord: 5i32
    };
    #[doc(alias = "COMPRESSION_MODE_ASTC")]
    #[doc = "Godot enumerator name: `COMPRESSION_MODE_ASTC`"]
    pub const ASTC: CompressionMode = CompressionMode {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for CompressionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CompressionMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CompressionMode {
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
            Self::LOSSLESS => "LOSSLESS", Self::LOSSY => "LOSSY", Self::BASIS_UNIVERSAL => "BASIS_UNIVERSAL", Self::S3TC => "S3TC", Self::ETC2 => "ETC2", Self::BPTC => "BPTC", Self::ASTC => "ASTC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CompressionMode::LOSSLESS, CompressionMode::LOSSY, CompressionMode::BASIS_UNIVERSAL, CompressionMode::S3TC, CompressionMode::ETC2, CompressionMode::BPTC, CompressionMode::ASTC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CompressionMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LOSSLESS", "COMPRESSION_MODE_LOSSLESS", CompressionMode::LOSSLESS), crate::meta::inspect::EnumConstant::new("LOSSY", "COMPRESSION_MODE_LOSSY", CompressionMode::LOSSY), crate::meta::inspect::EnumConstant::new("BASIS_UNIVERSAL", "COMPRESSION_MODE_BASIS_UNIVERSAL", CompressionMode::BASIS_UNIVERSAL), crate::meta::inspect::EnumConstant::new("S3TC", "COMPRESSION_MODE_S3TC", CompressionMode::S3TC), crate::meta::inspect::EnumConstant::new("ETC2", "COMPRESSION_MODE_ETC2", CompressionMode::ETC2), crate::meta::inspect::EnumConstant::new("BPTC", "COMPRESSION_MODE_BPTC", CompressionMode::BPTC), crate::meta::inspect::EnumConstant::new("ASTC", "COMPRESSION_MODE_ASTC", CompressionMode::ASTC)]
        }
    }
}
impl crate::meta::GodotConvert for CompressionMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CompressionMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CompressionMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PortableCompressedTexture2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for PortableCompressedTexture2D {
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