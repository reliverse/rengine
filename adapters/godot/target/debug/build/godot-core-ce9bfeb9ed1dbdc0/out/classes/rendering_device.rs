#![doc = "Sidecar module for class [`RenderingDevice`][crate::classes::RenderingDevice].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RenderingDevice` enums](https://docs.godotengine.org/en/stable/classes/class_renderingdevice.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RenderingDevice.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`rendering_device`][crate::classes::rendering_device]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `RenderingDevice`](https://docs.godotengine.org/en/stable/classes/class_renderingdevice.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<RenderingDevice>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RenderingDevice {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl RenderingDevice {
        pub(crate) fn texture_create_full(&mut self, format: CowArg < Option < Gd < crate::classes::RdTextureFormat > > >, view: CowArg < Option < Gd < crate::classes::RdTextureView > > >, data: RefArg < Array < PackedByteArray > >,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::RdTextureFormat > > >, CowArg < 'a1, Option < Gd < crate::classes::RdTextureView > > >, RefArg < 'a2, Array < PackedByteArray > >,);
            let args = (format, view, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7346usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::texture_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn texture_create(&mut self, format: impl AsArg < Option < Gd < crate::classes::RdTextureFormat >> >, view: impl AsArg < Option < Gd < crate::classes::RdTextureView >> >,) -> Rid {
            self.texture_create_ex(format, view,) . done()
        }
        #[inline]
        pub fn texture_create_ex < 'a > (&'a mut self, format: impl AsArg < Option < Gd < crate::classes::RdTextureFormat >> > + 'a, view: impl AsArg < Option < Gd < crate::classes::RdTextureView >> > + 'a,) -> ExTextureCreate < 'a > {
            ExTextureCreate::new(self, format, view,)
        }
        pub fn texture_create_shared(&mut self, view: impl AsArg < Option < Gd < crate::classes::RdTextureView >> >, with_texture: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::RdTextureView > > >, Rid,);
            let args = (view.into_arg(), with_texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7347usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_create_shared", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn texture_create_shared_from_slice_full(&mut self, view: CowArg < Option < Gd < crate::classes::RdTextureView > > >, with_texture: Rid, layer: u32, mipmap: u32, mipmaps: u32, slice_type: crate::classes::rendering_device::TextureSliceType,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::RdTextureView > > >, Rid, u32, u32, u32, crate::classes::rendering_device::TextureSliceType,);
            let args = (view, with_texture, layer, mipmap, mipmaps, slice_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7348usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_create_shared_from_slice", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::texture_create_shared_from_slice_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn texture_create_shared_from_slice(&mut self, view: impl AsArg < Option < Gd < crate::classes::RdTextureView >> >, with_texture: Rid, layer: u32, mipmap: u32,) -> Rid {
            self.texture_create_shared_from_slice_ex(view, with_texture, layer, mipmap,) . done()
        }
        #[inline]
        pub fn texture_create_shared_from_slice_ex < 'a > (&'a mut self, view: impl AsArg < Option < Gd < crate::classes::RdTextureView >> > + 'a, with_texture: Rid, layer: u32, mipmap: u32,) -> ExTextureCreateSharedFromSlice < 'a > {
            ExTextureCreateSharedFromSlice::new(self, view, with_texture, layer, mipmap,)
        }
        pub(crate) fn texture_create_from_extension_full(&mut self, type_: crate::classes::rendering_device::TextureType, format: crate::classes::rendering_device::DataFormat, samples: crate::classes::rendering_device::TextureSamples, usage_flags: crate::classes::rendering_device::TextureUsageBits, image: u64, width: u64, height: u64, depth: u64, layers: u64, mipmaps: u64,) -> Rid {
            type CallRet = Rid;
            type CallParams = (crate::classes::rendering_device::TextureType, crate::classes::rendering_device::DataFormat, crate::classes::rendering_device::TextureSamples, crate::classes::rendering_device::TextureUsageBits, u64, u64, u64, u64, u64, u64,);
            let args = (type_, format, samples, usage_flags, image, width, height, depth, layers, mipmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7349usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_create_from_extension", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::texture_create_from_extension_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn texture_create_from_extension(&mut self, type_: crate::classes::rendering_device::TextureType, format: crate::classes::rendering_device::DataFormat, samples: crate::classes::rendering_device::TextureSamples, usage_flags: crate::classes::rendering_device::TextureUsageBits, image: u64, width: u64, height: u64, depth: u64, layers: u64,) -> Rid {
            self.texture_create_from_extension_ex(type_, format, samples, usage_flags, image, width, height, depth, layers,) . done()
        }
        #[inline]
        pub fn texture_create_from_extension_ex < 'a > (&'a mut self, type_: crate::classes::rendering_device::TextureType, format: crate::classes::rendering_device::DataFormat, samples: crate::classes::rendering_device::TextureSamples, usage_flags: crate::classes::rendering_device::TextureUsageBits, image: u64, width: u64, height: u64, depth: u64, layers: u64,) -> ExTextureCreateFromExtension < 'a > {
            ExTextureCreateFromExtension::new(self, type_, format, samples, usage_flags, image, width, height, depth, layers,)
        }
        pub fn texture_update(&mut self, texture: Rid, layer: u32, data: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (Rid, u32, RefArg < 'a0, PackedByteArray >,);
            let args = (texture, layer, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7350usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_update", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_get_data(&mut self, texture: Rid, layer: u32,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (Rid, u32,);
            let args = (texture, layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7351usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_get_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_get_data_async(&mut self, texture: Rid, layer: u32, callback: &Callable,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (Rid, u32, RefArg < 'a0, Callable >,);
            let args = (texture, layer, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7352usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_get_data_async", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_is_format_supported_for_usage(&self, format: crate::classes::rendering_device::DataFormat, usage_flags: crate::classes::rendering_device::TextureUsageBits,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::rendering_device::DataFormat, crate::classes::rendering_device::TextureUsageBits,);
            let args = (format, usage_flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7353usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_is_format_supported_for_usage", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_is_shared(&mut self, texture: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7354usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_is_shared", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_is_valid(&mut self, texture: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7355usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_is_valid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_set_discardable(&mut self, texture: Rid, discardable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (texture, discardable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7356usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_set_discardable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_is_discardable(&mut self, texture: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7357usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_is_discardable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_copy(&mut self, from_texture: Rid, to_texture: Rid, from_pos: Vector3, to_pos: Vector3, size: Vector3, src_mipmap: u32, dst_mipmap: u32, src_layer: u32, dst_layer: u32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (Rid, Rid, Vector3, Vector3, Vector3, u32, u32, u32, u32,);
            let args = (from_texture, to_texture, from_pos, to_pos, size, src_mipmap, dst_mipmap, src_layer, dst_layer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7358usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_copy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_clear(&mut self, texture: Rid, color: Color, base_mipmap: u32, mipmap_count: u32, base_layer: u32, layer_count: u32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (Rid, Color, u32, u32, u32, u32,);
            let args = (texture, color, base_mipmap, mipmap_count, base_layer, layer_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_resolve_multisample(&mut self, from_texture: Rid, to_texture: Rid,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (Rid, Rid,);
            let args = (from_texture, to_texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_resolve_multisample", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_get_format(&mut self, texture: Rid,) -> Option < Gd < crate::classes::RdTextureFormat > > {
            type CallRet = Option < Gd < crate::classes::RdTextureFormat > >;
            type CallParams = (Rid,);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7361usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_get_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_get_native_handle(&mut self, texture: Rid,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid,);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7362usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_get_native_handle", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn framebuffer_format_create_full(&mut self, attachments: RefArg < Array < Gd < crate::classes::RdAttachmentFormat > > >, view_count: u32,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::RdAttachmentFormat > > >, u32,);
            let args = (attachments, view_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7363usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_format_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::framebuffer_format_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn framebuffer_format_create(&mut self, attachments: &Array < Gd < crate::classes::RdAttachmentFormat > >,) -> i64 {
            self.framebuffer_format_create_ex(attachments,) . done()
        }
        #[inline]
        pub fn framebuffer_format_create_ex < 'a > (&'a mut self, attachments: &'a Array < Gd < crate::classes::RdAttachmentFormat > >,) -> ExFramebufferFormatCreate < 'a > {
            ExFramebufferFormatCreate::new(self, attachments,)
        }
        pub(crate) fn framebuffer_format_create_multipass_full(&mut self, attachments: RefArg < Array < Gd < crate::classes::RdAttachmentFormat > > >, passes: RefArg < Array < Gd < crate::classes::RdFramebufferPass > > >, view_count: u32,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Array < Gd < crate::classes::RdAttachmentFormat > > >, RefArg < 'a1, Array < Gd < crate::classes::RdFramebufferPass > > >, u32,);
            let args = (attachments, passes, view_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7364usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_format_create_multipass", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::framebuffer_format_create_multipass_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn framebuffer_format_create_multipass(&mut self, attachments: &Array < Gd < crate::classes::RdAttachmentFormat > >, passes: &Array < Gd < crate::classes::RdFramebufferPass > >,) -> i64 {
            self.framebuffer_format_create_multipass_ex(attachments, passes,) . done()
        }
        #[inline]
        pub fn framebuffer_format_create_multipass_ex < 'a > (&'a mut self, attachments: &'a Array < Gd < crate::classes::RdAttachmentFormat > >, passes: &'a Array < Gd < crate::classes::RdFramebufferPass > >,) -> ExFramebufferFormatCreateMultipass < 'a > {
            ExFramebufferFormatCreateMultipass::new(self, attachments, passes,)
        }
        pub(crate) fn framebuffer_format_create_empty_full(&mut self, samples: crate::classes::rendering_device::TextureSamples,) -> i64 {
            type CallRet = i64;
            type CallParams = (crate::classes::rendering_device::TextureSamples,);
            let args = (samples,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7365usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_format_create_empty", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::framebuffer_format_create_empty_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn framebuffer_format_create_empty(&mut self,) -> i64 {
            self.framebuffer_format_create_empty_ex() . done()
        }
        #[inline]
        pub fn framebuffer_format_create_empty_ex < 'a > (&'a mut self,) -> ExFramebufferFormatCreateEmpty < 'a > {
            ExFramebufferFormatCreateEmpty::new(self,)
        }
        pub(crate) fn framebuffer_format_get_texture_samples_full(&mut self, format: i64, render_pass: u32,) -> crate::classes::rendering_device::TextureSamples {
            type CallRet = crate::classes::rendering_device::TextureSamples;
            type CallParams = (i64, u32,);
            let args = (format, render_pass,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7366usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_format_get_texture_samples", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::framebuffer_format_get_texture_samples_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn framebuffer_format_get_texture_samples(&mut self, format: i64,) -> crate::classes::rendering_device::TextureSamples {
            self.framebuffer_format_get_texture_samples_ex(format,) . done()
        }
        #[inline]
        pub fn framebuffer_format_get_texture_samples_ex < 'a > (&'a mut self, format: i64,) -> ExFramebufferFormatGetTextureSamples < 'a > {
            ExFramebufferFormatGetTextureSamples::new(self, format,)
        }
        pub(crate) fn framebuffer_create_full(&mut self, textures: RefArg < Array < Rid > >, validate_with_format: i64, view_count: u32,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Rid > >, i64, u32,);
            let args = (textures, validate_with_format, view_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7367usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::framebuffer_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn framebuffer_create(&mut self, textures: &Array < Rid >,) -> Rid {
            self.framebuffer_create_ex(textures,) . done()
        }
        #[inline]
        pub fn framebuffer_create_ex < 'a > (&'a mut self, textures: &'a Array < Rid >,) -> ExFramebufferCreate < 'a > {
            ExFramebufferCreate::new(self, textures,)
        }
        pub(crate) fn framebuffer_create_multipass_full(&mut self, textures: RefArg < Array < Rid > >, passes: RefArg < Array < Gd < crate::classes::RdFramebufferPass > > >, validate_with_format: i64, view_count: u32,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Array < Rid > >, RefArg < 'a1, Array < Gd < crate::classes::RdFramebufferPass > > >, i64, u32,);
            let args = (textures, passes, validate_with_format, view_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7368usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_create_multipass", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::framebuffer_create_multipass_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn framebuffer_create_multipass(&mut self, textures: &Array < Rid >, passes: &Array < Gd < crate::classes::RdFramebufferPass > >,) -> Rid {
            self.framebuffer_create_multipass_ex(textures, passes,) . done()
        }
        #[inline]
        pub fn framebuffer_create_multipass_ex < 'a > (&'a mut self, textures: &'a Array < Rid >, passes: &'a Array < Gd < crate::classes::RdFramebufferPass > >,) -> ExFramebufferCreateMultipass < 'a > {
            ExFramebufferCreateMultipass::new(self, textures, passes,)
        }
        pub(crate) fn framebuffer_create_empty_full(&mut self, size: Vector2i, samples: crate::classes::rendering_device::TextureSamples, validate_with_format: i64,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Vector2i, crate::classes::rendering_device::TextureSamples, i64,);
            let args = (size, samples, validate_with_format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7369usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_create_empty", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::framebuffer_create_empty_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn framebuffer_create_empty(&mut self, size: Vector2i,) -> Rid {
            self.framebuffer_create_empty_ex(size,) . done()
        }
        #[inline]
        pub fn framebuffer_create_empty_ex < 'a > (&'a mut self, size: Vector2i,) -> ExFramebufferCreateEmpty < 'a > {
            ExFramebufferCreateEmpty::new(self, size,)
        }
        pub fn framebuffer_get_format(&mut self, framebuffer: Rid,) -> i64 {
            type CallRet = i64;
            type CallParams = (Rid,);
            let args = (framebuffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7370usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_get_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn framebuffer_is_valid(&self, framebuffer: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (framebuffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7371usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "framebuffer_is_valid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn sampler_create(&mut self, state: impl AsArg < Option < Gd < crate::classes::RdSamplerState >> >,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::RdSamplerState > > >,);
            let args = (state.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7372usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "sampler_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn sampler_is_format_supported_for_filter(&self, format: crate::classes::rendering_device::DataFormat, sampler_filter: crate::classes::rendering_device::SamplerFilter,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::rendering_device::DataFormat, crate::classes::rendering_device::SamplerFilter,);
            let args = (format, sampler_filter,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7373usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "sampler_is_format_supported_for_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn vertex_buffer_create_full(&mut self, size_bytes: u32, data: RefArg < PackedByteArray >, creation_bits: crate::classes::rendering_device::BufferCreationBits,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (u32, RefArg < 'a0, PackedByteArray >, crate::classes::rendering_device::BufferCreationBits,);
            let args = (size_bytes, data, creation_bits,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7374usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "vertex_buffer_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::vertex_buffer_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn vertex_buffer_create(&mut self, size_bytes: u32,) -> Rid {
            self.vertex_buffer_create_ex(size_bytes,) . done()
        }
        #[inline]
        pub fn vertex_buffer_create_ex < 'a > (&'a mut self, size_bytes: u32,) -> ExVertexBufferCreate < 'a > {
            ExVertexBufferCreate::new(self, size_bytes,)
        }
        pub fn vertex_format_create(&mut self, vertex_descriptions: &Array < Gd < crate::classes::RdVertexAttribute > >,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::RdVertexAttribute > > >,);
            let args = (RefArg::new(vertex_descriptions),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7375usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "vertex_format_create", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn vertex_array_create_full(&mut self, vertex_count: u32, vertex_format: i64, src_buffers: RefArg < Array < Rid > >, offsets: RefArg < PackedInt64Array >,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, 'a1, > = (u32, i64, RefArg < 'a0, Array < Rid > >, RefArg < 'a1, PackedInt64Array >,);
            let args = (vertex_count, vertex_format, src_buffers, offsets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7376usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "vertex_array_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::vertex_array_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn vertex_array_create(&mut self, vertex_count: u32, vertex_format: i64, src_buffers: &Array < Rid >,) -> Rid {
            self.vertex_array_create_ex(vertex_count, vertex_format, src_buffers,) . done()
        }
        #[inline]
        pub fn vertex_array_create_ex < 'a > (&'a mut self, vertex_count: u32, vertex_format: i64, src_buffers: &'a Array < Rid >,) -> ExVertexArrayCreate < 'a > {
            ExVertexArrayCreate::new(self, vertex_count, vertex_format, src_buffers,)
        }
        pub(crate) fn index_buffer_create_full(&mut self, size_indices: u32, format: crate::classes::rendering_device::IndexBufferFormat, data: RefArg < PackedByteArray >, use_restart_indices: bool, creation_bits: crate::classes::rendering_device::BufferCreationBits,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (u32, crate::classes::rendering_device::IndexBufferFormat, RefArg < 'a0, PackedByteArray >, bool, crate::classes::rendering_device::BufferCreationBits,);
            let args = (size_indices, format, data, use_restart_indices, creation_bits,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7377usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "index_buffer_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::index_buffer_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn index_buffer_create(&mut self, size_indices: u32, format: crate::classes::rendering_device::IndexBufferFormat,) -> Rid {
            self.index_buffer_create_ex(size_indices, format,) . done()
        }
        #[inline]
        pub fn index_buffer_create_ex < 'a > (&'a mut self, size_indices: u32, format: crate::classes::rendering_device::IndexBufferFormat,) -> ExIndexBufferCreate < 'a > {
            ExIndexBufferCreate::new(self, size_indices, format,)
        }
        pub fn index_array_create(&mut self, index_buffer: Rid, index_offset: u32, index_count: u32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, u32, u32,);
            let args = (index_buffer, index_offset, index_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7378usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "index_array_create", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn shader_compile_spirv_from_source_full(&mut self, shader_source: CowArg < Option < Gd < crate::classes::RdShaderSource > > >, allow_cache: bool,) -> Option < Gd < crate::classes::RdShaderSpirv > > {
            type CallRet = Option < Gd < crate::classes::RdShaderSpirv > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::RdShaderSource > > >, bool,);
            let args = (shader_source, allow_cache,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7379usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "shader_compile_spirv_from_source", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shader_compile_spirv_from_source_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shader_compile_spirv_from_source(&mut self, shader_source: impl AsArg < Option < Gd < crate::classes::RdShaderSource >> >,) -> Option < Gd < crate::classes::RdShaderSpirv > > {
            self.shader_compile_spirv_from_source_ex(shader_source,) . done()
        }
        #[inline]
        pub fn shader_compile_spirv_from_source_ex < 'a > (&'a mut self, shader_source: impl AsArg < Option < Gd < crate::classes::RdShaderSource >> > + 'a,) -> ExShaderCompileSpirvFromSource < 'a > {
            ExShaderCompileSpirvFromSource::new(self, shader_source,)
        }
        pub(crate) fn shader_compile_binary_from_spirv_full(&mut self, spirv_data: CowArg < Option < Gd < crate::classes::RdShaderSpirv > > >, name: CowArg < GString >,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::RdShaderSpirv > > >, CowArg < 'a1, GString >,);
            let args = (spirv_data, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7380usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "shader_compile_binary_from_spirv", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shader_compile_binary_from_spirv_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shader_compile_binary_from_spirv(&mut self, spirv_data: impl AsArg < Option < Gd < crate::classes::RdShaderSpirv >> >,) -> PackedByteArray {
            self.shader_compile_binary_from_spirv_ex(spirv_data,) . done()
        }
        #[inline]
        pub fn shader_compile_binary_from_spirv_ex < 'a > (&'a mut self, spirv_data: impl AsArg < Option < Gd < crate::classes::RdShaderSpirv >> > + 'a,) -> ExShaderCompileBinaryFromSpirv < 'a > {
            ExShaderCompileBinaryFromSpirv::new(self, spirv_data,)
        }
        pub(crate) fn shader_create_from_spirv_full(&mut self, spirv_data: CowArg < Option < Gd < crate::classes::RdShaderSpirv > > >, name: CowArg < GString >,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::RdShaderSpirv > > >, CowArg < 'a1, GString >,);
            let args = (spirv_data, name,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7381usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "shader_create_from_spirv", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shader_create_from_spirv_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shader_create_from_spirv(&mut self, spirv_data: impl AsArg < Option < Gd < crate::classes::RdShaderSpirv >> >,) -> Rid {
            self.shader_create_from_spirv_ex(spirv_data,) . done()
        }
        #[inline]
        pub fn shader_create_from_spirv_ex < 'a > (&'a mut self, spirv_data: impl AsArg < Option < Gd < crate::classes::RdShaderSpirv >> > + 'a,) -> ExShaderCreateFromSpirv < 'a > {
            ExShaderCreateFromSpirv::new(self, spirv_data,)
        }
        pub(crate) fn shader_create_from_bytecode_full(&mut self, binary_data: RefArg < PackedByteArray >, placeholder_rid: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >, Rid,);
            let args = (binary_data, placeholder_rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7382usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "shader_create_from_bytecode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shader_create_from_bytecode_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shader_create_from_bytecode(&mut self, binary_data: &PackedByteArray,) -> Rid {
            self.shader_create_from_bytecode_ex(binary_data,) . done()
        }
        #[inline]
        pub fn shader_create_from_bytecode_ex < 'a > (&'a mut self, binary_data: &'a PackedByteArray,) -> ExShaderCreateFromBytecode < 'a > {
            ExShaderCreateFromBytecode::new(self, binary_data,)
        }
        pub fn shader_create_placeholder(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7383usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "shader_create_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shader_get_vertex_input_attribute_mask(&mut self, shader: Rid,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid,);
            let args = (shader,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7384usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "shader_get_vertex_input_attribute_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn uniform_buffer_create_full(&mut self, size_bytes: u32, data: RefArg < PackedByteArray >, creation_bits: crate::classes::rendering_device::BufferCreationBits,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (u32, RefArg < 'a0, PackedByteArray >, crate::classes::rendering_device::BufferCreationBits,);
            let args = (size_bytes, data, creation_bits,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7385usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "uniform_buffer_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::uniform_buffer_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn uniform_buffer_create(&mut self, size_bytes: u32,) -> Rid {
            self.uniform_buffer_create_ex(size_bytes,) . done()
        }
        #[inline]
        pub fn uniform_buffer_create_ex < 'a > (&'a mut self, size_bytes: u32,) -> ExUniformBufferCreate < 'a > {
            ExUniformBufferCreate::new(self, size_bytes,)
        }
        pub(crate) fn storage_buffer_create_full(&mut self, size_bytes: u32, data: RefArg < PackedByteArray >, usage: crate::classes::rendering_device::StorageBufferUsage, creation_bits: crate::classes::rendering_device::BufferCreationBits,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (u32, RefArg < 'a0, PackedByteArray >, crate::classes::rendering_device::StorageBufferUsage, crate::classes::rendering_device::BufferCreationBits,);
            let args = (size_bytes, data, usage, creation_bits,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7386usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "storage_buffer_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::storage_buffer_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn storage_buffer_create(&mut self, size_bytes: u32,) -> Rid {
            self.storage_buffer_create_ex(size_bytes,) . done()
        }
        #[inline]
        pub fn storage_buffer_create_ex < 'a > (&'a mut self, size_bytes: u32,) -> ExStorageBufferCreate < 'a > {
            ExStorageBufferCreate::new(self, size_bytes,)
        }
        pub(crate) fn texture_buffer_create_full(&mut self, size_bytes: u32, format: crate::classes::rendering_device::DataFormat, data: RefArg < PackedByteArray >,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (u32, crate::classes::rendering_device::DataFormat, RefArg < 'a0, PackedByteArray >,);
            let args = (size_bytes, format, data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7387usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "texture_buffer_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::texture_buffer_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn texture_buffer_create(&mut self, size_bytes: u32, format: crate::classes::rendering_device::DataFormat,) -> Rid {
            self.texture_buffer_create_ex(size_bytes, format,) . done()
        }
        #[inline]
        pub fn texture_buffer_create_ex < 'a > (&'a mut self, size_bytes: u32, format: crate::classes::rendering_device::DataFormat,) -> ExTextureBufferCreate < 'a > {
            ExTextureBufferCreate::new(self, size_bytes, format,)
        }
        pub fn uniform_set_create(&mut self, uniforms: &Array < Gd < crate::classes::RdUniform > >, shader: Rid, shader_set: u32,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::RdUniform > > >, Rid, u32,);
            let args = (RefArg::new(uniforms), shader, shader_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7388usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "uniform_set_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn uniform_set_is_valid(&mut self, uniform_set: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (uniform_set,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7389usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "uniform_set_is_valid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn buffer_copy(&mut self, src_buffer: Rid, dst_buffer: Rid, src_offset: u32, dst_offset: u32, size: u32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (Rid, Rid, u32, u32, u32,);
            let args = (src_buffer, dst_buffer, src_offset, dst_offset, size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7390usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "buffer_copy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn buffer_update(&mut self, buffer: Rid, offset: u32, size_bytes: u32, data: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (Rid, u32, u32, RefArg < 'a0, PackedByteArray >,);
            let args = (buffer, offset, size_bytes, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7391usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "buffer_update", Some(self.__validated_obj()), args,)
            }
        }
        pub fn buffer_clear(&mut self, buffer: Rid, offset: u32, size_bytes: u32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (Rid, u32, u32,);
            let args = (buffer, offset, size_bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7392usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "buffer_clear", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn buffer_get_data_full(&mut self, buffer: Rid, offset_bytes: u32, size_bytes: u32,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (Rid, u32, u32,);
            let args = (buffer, offset_bytes, size_bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7393usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "buffer_get_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::buffer_get_data_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn buffer_get_data(&mut self, buffer: Rid,) -> PackedByteArray {
            self.buffer_get_data_ex(buffer,) . done()
        }
        #[inline]
        pub fn buffer_get_data_ex < 'a > (&'a mut self, buffer: Rid,) -> ExBufferGetData < 'a > {
            ExBufferGetData::new(self, buffer,)
        }
        pub(crate) fn buffer_get_data_async_full(&mut self, buffer: Rid, callback: RefArg < Callable >, offset_bytes: u32, size_bytes: u32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Callable >, u32, u32,);
            let args = (buffer, callback, offset_bytes, size_bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7394usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "buffer_get_data_async", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::buffer_get_data_async_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn buffer_get_data_async(&mut self, buffer: Rid, callback: &Callable,) -> crate::global::Error {
            self.buffer_get_data_async_ex(buffer, callback,) . done()
        }
        #[inline]
        pub fn buffer_get_data_async_ex < 'a > (&'a mut self, buffer: Rid, callback: &'a Callable,) -> ExBufferGetDataAsync < 'a > {
            ExBufferGetDataAsync::new(self, buffer, callback,)
        }
        pub fn buffer_get_device_address(&mut self, buffer: Rid,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid,);
            let args = (buffer,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7395usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "buffer_get_device_address", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn render_pipeline_create_full(&mut self, shader: Rid, framebuffer_format: i64, vertex_format: i64, primitive: crate::classes::rendering_device::RenderPrimitive, rasterization_state: CowArg < Option < Gd < crate::classes::RdPipelineRasterizationState > > >, multisample_state: CowArg < Option < Gd < crate::classes::RdPipelineMultisampleState > > >, stencil_state: CowArg < Option < Gd < crate::classes::RdPipelineDepthStencilState > > >, color_blend_state: CowArg < Option < Gd < crate::classes::RdPipelineColorBlendState > > >, dynamic_state_flags: crate::classes::rendering_device::PipelineDynamicStateFlags, for_render_pass: u32, specialization_constants: RefArg < Array < Gd < crate::classes::RdPipelineSpecializationConstant > > >,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, > = (Rid, i64, i64, crate::classes::rendering_device::RenderPrimitive, CowArg < 'a0, Option < Gd < crate::classes::RdPipelineRasterizationState > > >, CowArg < 'a1, Option < Gd < crate::classes::RdPipelineMultisampleState > > >, CowArg < 'a2, Option < Gd < crate::classes::RdPipelineDepthStencilState > > >, CowArg < 'a3, Option < Gd < crate::classes::RdPipelineColorBlendState > > >, crate::classes::rendering_device::PipelineDynamicStateFlags, u32, RefArg < 'a4, Array < Gd < crate::classes::RdPipelineSpecializationConstant > > >,);
            let args = (shader, framebuffer_format, vertex_format, primitive, rasterization_state, multisample_state, stencil_state, color_blend_state, dynamic_state_flags, for_render_pass, specialization_constants,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7396usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "render_pipeline_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::render_pipeline_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn render_pipeline_create(&mut self, shader: Rid, framebuffer_format: i64, vertex_format: i64, primitive: crate::classes::rendering_device::RenderPrimitive, rasterization_state: impl AsArg < Option < Gd < crate::classes::RdPipelineRasterizationState >> >, multisample_state: impl AsArg < Option < Gd < crate::classes::RdPipelineMultisampleState >> >, stencil_state: impl AsArg < Option < Gd < crate::classes::RdPipelineDepthStencilState >> >, color_blend_state: impl AsArg < Option < Gd < crate::classes::RdPipelineColorBlendState >> >,) -> Rid {
            self.render_pipeline_create_ex(shader, framebuffer_format, vertex_format, primitive, rasterization_state, multisample_state, stencil_state, color_blend_state,) . done()
        }
        #[inline]
        pub fn render_pipeline_create_ex < 'a > (&'a mut self, shader: Rid, framebuffer_format: i64, vertex_format: i64, primitive: crate::classes::rendering_device::RenderPrimitive, rasterization_state: impl AsArg < Option < Gd < crate::classes::RdPipelineRasterizationState >> > + 'a, multisample_state: impl AsArg < Option < Gd < crate::classes::RdPipelineMultisampleState >> > + 'a, stencil_state: impl AsArg < Option < Gd < crate::classes::RdPipelineDepthStencilState >> > + 'a, color_blend_state: impl AsArg < Option < Gd < crate::classes::RdPipelineColorBlendState >> > + 'a,) -> ExRenderPipelineCreate < 'a > {
            ExRenderPipelineCreate::new(self, shader, framebuffer_format, vertex_format, primitive, rasterization_state, multisample_state, stencil_state, color_blend_state,)
        }
        pub fn render_pipeline_is_valid(&mut self, render_pipeline: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (render_pipeline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7397usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "render_pipeline_is_valid", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn compute_pipeline_create_full(&mut self, shader: Rid, specialization_constants: RefArg < Array < Gd < crate::classes::RdPipelineSpecializationConstant > > >,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Array < Gd < crate::classes::RdPipelineSpecializationConstant > > >,);
            let args = (shader, specialization_constants,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7398usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_pipeline_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::compute_pipeline_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn compute_pipeline_create(&mut self, shader: Rid,) -> Rid {
            self.compute_pipeline_create_ex(shader,) . done()
        }
        #[inline]
        pub fn compute_pipeline_create_ex < 'a > (&'a mut self, shader: Rid,) -> ExComputePipelineCreate < 'a > {
            ExComputePipelineCreate::new(self, shader,)
        }
        pub fn compute_pipeline_is_valid(&mut self, compute_pipeline: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (compute_pipeline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7399usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_pipeline_is_valid", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn screen_get_width_full(&self, screen: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7400usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "screen_get_width", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_width_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_width(&self,) -> i32 {
            self.screen_get_width_ex() . done()
        }
        #[inline]
        pub fn screen_get_width_ex < 'a > (&'a self,) -> ExScreenGetWidth < 'a > {
            ExScreenGetWidth::new(self,)
        }
        pub(crate) fn screen_get_height_full(&self, screen: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7401usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "screen_get_height", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_height_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_height(&self,) -> i32 {
            self.screen_get_height_ex() . done()
        }
        #[inline]
        pub fn screen_get_height_ex < 'a > (&'a self,) -> ExScreenGetHeight < 'a > {
            ExScreenGetHeight::new(self,)
        }
        pub(crate) fn screen_get_framebuffer_format_full(&self, screen: i32,) -> i64 {
            type CallRet = i64;
            type CallParams = (i32,);
            let args = (screen,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7402usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "screen_get_framebuffer_format", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::screen_get_framebuffer_format_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn screen_get_framebuffer_format(&self,) -> i64 {
            self.screen_get_framebuffer_format_ex() . done()
        }
        #[inline]
        pub fn screen_get_framebuffer_format_ex < 'a > (&'a self,) -> ExScreenGetFramebufferFormat < 'a > {
            ExScreenGetFramebufferFormat::new(self,)
        }
        pub(crate) fn draw_list_begin_for_screen_full(&mut self, screen: i32, clear_color: Color,) -> i64 {
            type CallRet = i64;
            type CallParams = (i32, Color,);
            let args = (screen, clear_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7403usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_begin_for_screen", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_list_begin_for_screen_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_list_begin_for_screen(&mut self,) -> i64 {
            self.draw_list_begin_for_screen_ex() . done()
        }
        #[inline]
        pub fn draw_list_begin_for_screen_ex < 'a > (&'a mut self,) -> ExDrawListBeginForScreen < 'a > {
            ExDrawListBeginForScreen::new(self,)
        }
        pub(crate) fn draw_list_begin_full(&mut self, framebuffer: Rid, draw_flags: crate::classes::rendering_device::DrawFlags, clear_color_values: RefArg < PackedColorArray >, clear_depth_value: f32, clear_stencil_value: u32, region: Rect2, breadcrumb: u32,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, > = (Rid, crate::classes::rendering_device::DrawFlags, RefArg < 'a0, PackedColorArray >, f32, u32, Rect2, u32,);
            let args = (framebuffer, draw_flags, clear_color_values, clear_depth_value, clear_stencil_value, region, breadcrumb,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_begin", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_list_begin_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_list_begin(&mut self, framebuffer: Rid,) -> i64 {
            self.draw_list_begin_ex(framebuffer,) . done()
        }
        #[inline]
        pub fn draw_list_begin_ex < 'a > (&'a mut self, framebuffer: Rid,) -> ExDrawListBegin < 'a > {
            ExDrawListBegin::new(self, framebuffer,)
        }
        pub(crate) fn draw_list_begin_split_full(&mut self, framebuffer: Rid, splits: u32, initial_color_action: crate::classes::rendering_device::InitialAction, final_color_action: crate::classes::rendering_device::FinalAction, initial_depth_action: crate::classes::rendering_device::InitialAction, final_depth_action: crate::classes::rendering_device::FinalAction, clear_color_values: RefArg < PackedColorArray >, clear_depth: f32, clear_stencil: u32, region: Rect2, storage_textures: RefArg < Array < Rid > >,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams < 'a0, 'a1, > = (Rid, u32, crate::classes::rendering_device::InitialAction, crate::classes::rendering_device::FinalAction, crate::classes::rendering_device::InitialAction, crate::classes::rendering_device::FinalAction, RefArg < 'a0, PackedColorArray >, f32, u32, Rect2, RefArg < 'a1, Array < Rid > >,);
            let args = (framebuffer, splits, initial_color_action, final_color_action, initial_depth_action, final_depth_action, clear_color_values, clear_depth, clear_stencil, region, storage_textures,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_begin_split", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_list_begin_split_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_list_begin_split(&mut self, framebuffer: Rid, splits: u32, initial_color_action: crate::classes::rendering_device::InitialAction, final_color_action: crate::classes::rendering_device::FinalAction, initial_depth_action: crate::classes::rendering_device::InitialAction, final_depth_action: crate::classes::rendering_device::FinalAction,) -> PackedInt64Array {
            self.draw_list_begin_split_ex(framebuffer, splits, initial_color_action, final_color_action, initial_depth_action, final_depth_action,) . done()
        }
        #[inline]
        pub fn draw_list_begin_split_ex < 'a > (&'a mut self, framebuffer: Rid, splits: u32, initial_color_action: crate::classes::rendering_device::InitialAction, final_color_action: crate::classes::rendering_device::FinalAction, initial_depth_action: crate::classes::rendering_device::InitialAction, final_depth_action: crate::classes::rendering_device::FinalAction,) -> ExDrawListBeginSplit < 'a > {
            ExDrawListBeginSplit::new(self, framebuffer, splits, initial_color_action, final_color_action, initial_depth_action, final_depth_action,)
        }
        pub fn draw_list_set_blend_constants(&mut self, draw_list: i64, color: Color,) {
            type CallRet = ();
            type CallParams = (i64, Color,);
            let args = (draw_list, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_set_blend_constants", Some(self.__validated_obj()), args,)
            }
        }
        pub fn draw_list_bind_render_pipeline(&mut self, draw_list: i64, render_pipeline: Rid,) {
            type CallRet = ();
            type CallParams = (i64, Rid,);
            let args = (draw_list, render_pipeline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_bind_render_pipeline", Some(self.__validated_obj()), args,)
            }
        }
        pub fn draw_list_bind_uniform_set(&mut self, draw_list: i64, uniform_set: Rid, set_index: u32,) {
            type CallRet = ();
            type CallParams = (i64, Rid, u32,);
            let args = (draw_list, uniform_set, set_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_bind_uniform_set", Some(self.__validated_obj()), args,)
            }
        }
        pub fn draw_list_bind_vertex_array(&mut self, draw_list: i64, vertex_array: Rid,) {
            type CallRet = ();
            type CallParams = (i64, Rid,);
            let args = (draw_list, vertex_array,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_bind_vertex_array", Some(self.__validated_obj()), args,)
            }
        }
        pub fn draw_list_bind_index_array(&mut self, draw_list: i64, index_array: Rid,) {
            type CallRet = ();
            type CallParams = (i64, Rid,);
            let args = (draw_list, index_array,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_bind_index_array", Some(self.__validated_obj()), args,)
            }
        }
        pub fn draw_list_set_push_constant(&mut self, draw_list: i64, buffer: &PackedByteArray, size_bytes: u32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i64, RefArg < 'a0, PackedByteArray >, u32,);
            let args = (draw_list, RefArg::new(buffer), size_bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_set_push_constant", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn draw_list_draw_full(&mut self, draw_list: i64, use_indices: bool, instances: u32, procedural_vertex_count: u32,) {
            type CallRet = ();
            type CallParams = (i64, bool, u32, u32,);
            let args = (draw_list, use_indices, instances, procedural_vertex_count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_draw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_list_draw_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_list_draw(&mut self, draw_list: i64, use_indices: bool, instances: u32,) {
            self.draw_list_draw_ex(draw_list, use_indices, instances,) . done()
        }
        #[inline]
        pub fn draw_list_draw_ex < 'a > (&'a mut self, draw_list: i64, use_indices: bool, instances: u32,) -> ExDrawListDraw < 'a > {
            ExDrawListDraw::new(self, draw_list, use_indices, instances,)
        }
        pub(crate) fn draw_list_draw_indirect_full(&mut self, draw_list: i64, use_indices: bool, buffer: Rid, offset: u32, draw_count: u32, stride: u32,) {
            type CallRet = ();
            type CallParams = (i64, bool, Rid, u32, u32, u32,);
            let args = (draw_list, use_indices, buffer, offset, draw_count, stride,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_draw_indirect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_list_draw_indirect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_list_draw_indirect(&mut self, draw_list: i64, use_indices: bool, buffer: Rid,) {
            self.draw_list_draw_indirect_ex(draw_list, use_indices, buffer,) . done()
        }
        #[inline]
        pub fn draw_list_draw_indirect_ex < 'a > (&'a mut self, draw_list: i64, use_indices: bool, buffer: Rid,) -> ExDrawListDrawIndirect < 'a > {
            ExDrawListDrawIndirect::new(self, draw_list, use_indices, buffer,)
        }
        pub(crate) fn draw_list_enable_scissor_full(&mut self, draw_list: i64, rect: Rect2,) {
            type CallRet = ();
            type CallParams = (i64, Rect2,);
            let args = (draw_list, rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_enable_scissor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::draw_list_enable_scissor_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn draw_list_enable_scissor(&mut self, draw_list: i64,) {
            self.draw_list_enable_scissor_ex(draw_list,) . done()
        }
        #[inline]
        pub fn draw_list_enable_scissor_ex < 'a > (&'a mut self, draw_list: i64,) -> ExDrawListEnableScissor < 'a > {
            ExDrawListEnableScissor::new(self, draw_list,)
        }
        pub fn draw_list_disable_scissor(&mut self, draw_list: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (draw_list,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_disable_scissor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn draw_list_switch_to_next_pass(&mut self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_switch_to_next_pass", Some(self.__validated_obj()), args,)
            }
        }
        pub fn draw_list_switch_to_next_pass_split(&mut self, splits: u32,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams = (u32,);
            let args = (splits,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_switch_to_next_pass_split", Some(self.__validated_obj()), args,)
            }
        }
        pub fn draw_list_end(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_list_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn compute_list_begin(&mut self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_list_begin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn compute_list_bind_compute_pipeline(&mut self, compute_list: i64, compute_pipeline: Rid,) {
            type CallRet = ();
            type CallParams = (i64, Rid,);
            let args = (compute_list, compute_pipeline,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7420usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_list_bind_compute_pipeline", Some(self.__validated_obj()), args,)
            }
        }
        pub fn compute_list_set_push_constant(&mut self, compute_list: i64, buffer: &PackedByteArray, size_bytes: u32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i64, RefArg < 'a0, PackedByteArray >, u32,);
            let args = (compute_list, RefArg::new(buffer), size_bytes,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7421usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_list_set_push_constant", Some(self.__validated_obj()), args,)
            }
        }
        pub fn compute_list_bind_uniform_set(&mut self, compute_list: i64, uniform_set: Rid, set_index: u32,) {
            type CallRet = ();
            type CallParams = (i64, Rid, u32,);
            let args = (compute_list, uniform_set, set_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7422usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_list_bind_uniform_set", Some(self.__validated_obj()), args,)
            }
        }
        pub fn compute_list_dispatch(&mut self, compute_list: i64, x_groups: u32, y_groups: u32, z_groups: u32,) {
            type CallRet = ();
            type CallParams = (i64, u32, u32, u32,);
            let args = (compute_list, x_groups, y_groups, z_groups,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7423usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_list_dispatch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn compute_list_dispatch_indirect(&mut self, compute_list: i64, buffer: Rid, offset: u32,) {
            type CallRet = ();
            type CallParams = (i64, Rid, u32,);
            let args = (compute_list, buffer, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7424usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_list_dispatch_indirect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn compute_list_add_barrier(&mut self, compute_list: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (compute_list,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7425usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_list_add_barrier", Some(self.__validated_obj()), args,)
            }
        }
        pub fn compute_list_end(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7426usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "compute_list_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn free_rid(&mut self, rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7427usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "free_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn capture_timestamp(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7428usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "capture_timestamp", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_captured_timestamps_count(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7429usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_captured_timestamps_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_captured_timestamps_frame(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7430usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_captured_timestamps_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_captured_timestamp_gpu_time(&self, index: u32,) -> u64 {
            type CallRet = u64;
            type CallParams = (u32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7431usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_captured_timestamp_gpu_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_captured_timestamp_cpu_time(&self, index: u32,) -> u64 {
            type CallRet = u64;
            type CallParams = (u32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7432usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_captured_timestamp_cpu_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_captured_timestamp_name(&self, index: u32,) -> GString {
            type CallRet = GString;
            type CallParams = (u32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7433usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_captured_timestamp_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_feature(&self, feature: crate::classes::rendering_device::Features,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::rendering_device::Features,);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7434usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "has_feature", Some(self.__validated_obj()), args,)
            }
        }
        pub fn limit_get(&self, limit: crate::classes::rendering_device::Limit,) -> u64 {
            type CallRet = u64;
            type CallParams = (crate::classes::rendering_device::Limit,);
            let args = (limit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7435usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "limit_get", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frame_delay(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7436usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_frame_delay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn submit(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7437usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "submit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn sync(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7438usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "sync", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn barrier_full(&mut self, from: crate::classes::rendering_device::BarrierMask, to: crate::classes::rendering_device::BarrierMask,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_device::BarrierMask, crate::classes::rendering_device::BarrierMask,);
            let args = (from, to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7439usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "barrier", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::barrier_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn barrier(&mut self,) {
            self.barrier_ex() . done()
        }
        #[inline]
        pub fn barrier_ex < 'a > (&'a mut self,) -> ExBarrier < 'a > {
            ExBarrier::new(self,)
        }
        pub fn full_barrier(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7440usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "full_barrier", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_local_device(&mut self,) -> Option < Gd < crate::classes::RenderingDevice > > {
            type CallRet = Option < Gd < crate::classes::RenderingDevice > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7441usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "create_local_device", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_resource_name(&mut self, id: Rid, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (id, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7442usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "set_resource_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn draw_command_begin_label(&mut self, name: impl AsArg < GString >, color: Color,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, Color,);
            let args = (name.into_arg(), color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7443usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_command_begin_label", Some(self.__validated_obj()), args,)
            }
        }
        pub fn draw_command_insert_label(&mut self, name: impl AsArg < GString >, color: Color,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, Color,);
            let args = (name.into_arg(), color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7444usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_command_insert_label", Some(self.__validated_obj()), args,)
            }
        }
        pub fn draw_command_end_label(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7445usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "draw_command_end_label", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_device_vendor_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7446usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_device_vendor_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_device_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7447usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_device_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_device_pipeline_cache_uuid(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7448usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_device_pipeline_cache_uuid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_memory_usage(&self, type_: crate::classes::rendering_device::MemoryType,) -> u64 {
            type CallRet = u64;
            type CallParams = (crate::classes::rendering_device::MemoryType,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7449usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_memory_usage", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_driver_resource(&mut self, resource: crate::classes::rendering_device::DriverResource, rid: Rid, index: u64,) -> u64 {
            type CallRet = u64;
            type CallParams = (crate::classes::rendering_device::DriverResource, Rid, u64,);
            let args = (resource, rid, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7450usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_driver_resource", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_perf_report(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7451usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_perf_report", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_driver_and_device_memory_report(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7452usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_driver_and_device_memory_report", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tracked_object_name(&self, type_index: u32,) -> GString {
            type CallRet = GString;
            type CallParams = (u32,);
            let args = (type_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7453usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_tracked_object_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tracked_object_type_count(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7454usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_tracked_object_type_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_driver_total_memory(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7455usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_driver_total_memory", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_driver_allocation_count(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7456usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_driver_allocation_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_driver_memory_by_object_type(&self, type_: u32,) -> u64 {
            type CallRet = u64;
            type CallParams = (u32,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7457usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_driver_memory_by_object_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_driver_allocs_by_object_type(&self, type_: u32,) -> u64 {
            type CallRet = u64;
            type CallParams = (u32,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7458usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_driver_allocs_by_object_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_device_total_memory(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7459usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_device_total_memory", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_device_allocation_count(&self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7460usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_device_allocation_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_device_memory_by_object_type(&self, type_: u32,) -> u64 {
            type CallRet = u64;
            type CallParams = (u32,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7461usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_device_memory_by_object_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_device_allocs_by_object_type(&self, type_: u32,) -> u64 {
            type CallRet = u64;
            type CallParams = (u32,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7462usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingDevice", "get_device_allocs_by_object_type", Some(self.__validated_obj()), args,)
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
        pub const INVALID_ID: i32 = - 1i32;
        pub const INVALID_FORMAT_ID: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for RenderingDevice {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("RenderingDevice"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RenderingDevice {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RenderingDevice {
        
    }
    impl std::ops::Deref for RenderingDevice {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RenderingDevice {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RenderingDevice__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `RenderingDevice` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`RenderingDevice::texture_create_ex`][super::RenderingDevice::texture_create_ex]."]
#[must_use]
pub struct ExTextureCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, format: CowArg < 'a, Option < Gd < crate::classes::RdTextureFormat > > >, view: CowArg < 'a, Option < Gd < crate::classes::RdTextureView > > >, data: CowArg < 'a, Array < PackedByteArray > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, format: impl AsArg < Option < Gd < crate::classes::RdTextureFormat >> > + 'a, view: impl AsArg < Option < Gd < crate::classes::RdTextureView >> > + 'a,) -> Self {
        let data = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, format: format.into_arg(), view: view.into_arg(), data: CowArg::Owned(data),
        }
    }
    #[inline]
    pub fn data(self, data: &'a Array < PackedByteArray >) -> Self {
        Self {
            data: CowArg::Borrowed(data), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, format, view, data,
        }
        = self;
        re_export::RenderingDevice::texture_create_full(surround_object, format, view, data.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::texture_create_shared_from_slice_ex`][super::RenderingDevice::texture_create_shared_from_slice_ex]."]
#[must_use]
pub struct ExTextureCreateSharedFromSlice < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, view: CowArg < 'a, Option < Gd < crate::classes::RdTextureView > > >, with_texture: Rid, layer: u32, mipmap: u32, mipmaps: u32, slice_type: crate::classes::rendering_device::TextureSliceType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureCreateSharedFromSlice < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, view: impl AsArg < Option < Gd < crate::classes::RdTextureView >> > + 'a, with_texture: Rid, layer: u32, mipmap: u32,) -> Self {
        let mipmaps = 1u32;
        let slice_type = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, view: view.into_arg(), with_texture: with_texture, layer: layer, mipmap: mipmap, mipmaps: mipmaps, slice_type: slice_type,
        }
    }
    #[inline]
    pub fn mipmaps(self, mipmaps: u32) -> Self {
        Self {
            mipmaps: mipmaps, .. self
        }
    }
    #[inline]
    pub fn slice_type(self, slice_type: crate::classes::rendering_device::TextureSliceType) -> Self {
        Self {
            slice_type: slice_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, view, with_texture, layer, mipmap, mipmaps, slice_type,
        }
        = self;
        re_export::RenderingDevice::texture_create_shared_from_slice_full(surround_object, view, with_texture, layer, mipmap, mipmaps, slice_type,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::texture_create_from_extension_ex`][super::RenderingDevice::texture_create_from_extension_ex]."]
#[must_use]
pub struct ExTextureCreateFromExtension < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, type_: crate::classes::rendering_device::TextureType, format: crate::classes::rendering_device::DataFormat, samples: crate::classes::rendering_device::TextureSamples, usage_flags: crate::classes::rendering_device::TextureUsageBits, image: u64, width: u64, height: u64, depth: u64, layers: u64, mipmaps: u64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureCreateFromExtension < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, type_: crate::classes::rendering_device::TextureType, format: crate::classes::rendering_device::DataFormat, samples: crate::classes::rendering_device::TextureSamples, usage_flags: crate::classes::rendering_device::TextureUsageBits, image: u64, width: u64, height: u64, depth: u64, layers: u64,) -> Self {
        let mipmaps = 1u64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, type_: type_, format: format, samples: samples, usage_flags: usage_flags, image: image, width: width, height: height, depth: depth, layers: layers, mipmaps: mipmaps,
        }
    }
    #[inline]
    pub fn mipmaps(self, mipmaps: u64) -> Self {
        Self {
            mipmaps: mipmaps, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, type_, format, samples, usage_flags, image, width, height, depth, layers, mipmaps,
        }
        = self;
        re_export::RenderingDevice::texture_create_from_extension_full(surround_object, type_, format, samples, usage_flags, image, width, height, depth, layers, mipmaps,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_format_create_ex`][super::RenderingDevice::framebuffer_format_create_ex]."]
#[must_use]
pub struct ExFramebufferFormatCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, attachments: CowArg < 'a, Array < Gd < crate::classes::RdAttachmentFormat > > >, view_count: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferFormatCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, attachments: &'a Array < Gd < crate::classes::RdAttachmentFormat > >,) -> Self {
        let view_count = 1u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, attachments: CowArg::Borrowed(attachments), view_count: view_count,
        }
    }
    #[inline]
    pub fn view_count(self, view_count: u32) -> Self {
        Self {
            view_count: view_count, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, attachments, view_count,
        }
        = self;
        re_export::RenderingDevice::framebuffer_format_create_full(surround_object, attachments.cow_as_arg(), view_count,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_format_create_multipass_ex`][super::RenderingDevice::framebuffer_format_create_multipass_ex]."]
#[must_use]
pub struct ExFramebufferFormatCreateMultipass < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, attachments: CowArg < 'a, Array < Gd < crate::classes::RdAttachmentFormat > > >, passes: CowArg < 'a, Array < Gd < crate::classes::RdFramebufferPass > > >, view_count: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferFormatCreateMultipass < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, attachments: &'a Array < Gd < crate::classes::RdAttachmentFormat > >, passes: &'a Array < Gd < crate::classes::RdFramebufferPass > >,) -> Self {
        let view_count = 1u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, attachments: CowArg::Borrowed(attachments), passes: CowArg::Borrowed(passes), view_count: view_count,
        }
    }
    #[inline]
    pub fn view_count(self, view_count: u32) -> Self {
        Self {
            view_count: view_count, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, attachments, passes, view_count,
        }
        = self;
        re_export::RenderingDevice::framebuffer_format_create_multipass_full(surround_object, attachments.cow_as_arg(), passes.cow_as_arg(), view_count,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_format_create_empty_ex`][super::RenderingDevice::framebuffer_format_create_empty_ex]."]
#[must_use]
pub struct ExFramebufferFormatCreateEmpty < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, samples: crate::classes::rendering_device::TextureSamples,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferFormatCreateEmpty < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice,) -> Self {
        let samples = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, samples: samples,
        }
    }
    #[inline]
    pub fn samples(self, samples: crate::classes::rendering_device::TextureSamples) -> Self {
        Self {
            samples: samples, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, samples,
        }
        = self;
        re_export::RenderingDevice::framebuffer_format_create_empty_full(surround_object, samples,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_format_get_texture_samples_ex`][super::RenderingDevice::framebuffer_format_get_texture_samples_ex]."]
#[must_use]
pub struct ExFramebufferFormatGetTextureSamples < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, format: i64, render_pass: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferFormatGetTextureSamples < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, format: i64,) -> Self {
        let render_pass = 0u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, format: format, render_pass: render_pass,
        }
    }
    #[inline]
    pub fn render_pass(self, render_pass: u32) -> Self {
        Self {
            render_pass: render_pass, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::classes::rendering_device::TextureSamples {
        let Self {
            _phantom, surround_object, format, render_pass,
        }
        = self;
        re_export::RenderingDevice::framebuffer_format_get_texture_samples_full(surround_object, format, render_pass,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_create_ex`][super::RenderingDevice::framebuffer_create_ex]."]
#[must_use]
pub struct ExFramebufferCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, textures: CowArg < 'a, Array < Rid > >, validate_with_format: i64, view_count: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, textures: &'a Array < Rid >,) -> Self {
        let validate_with_format = - 1i64;
        let view_count = 1u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, textures: CowArg::Borrowed(textures), validate_with_format: validate_with_format, view_count: view_count,
        }
    }
    #[inline]
    pub fn validate_with_format(self, validate_with_format: i64) -> Self {
        Self {
            validate_with_format: validate_with_format, .. self
        }
    }
    #[inline]
    pub fn view_count(self, view_count: u32) -> Self {
        Self {
            view_count: view_count, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, textures, validate_with_format, view_count,
        }
        = self;
        re_export::RenderingDevice::framebuffer_create_full(surround_object, textures.cow_as_arg(), validate_with_format, view_count,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_create_multipass_ex`][super::RenderingDevice::framebuffer_create_multipass_ex]."]
#[must_use]
pub struct ExFramebufferCreateMultipass < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, textures: CowArg < 'a, Array < Rid > >, passes: CowArg < 'a, Array < Gd < crate::classes::RdFramebufferPass > > >, validate_with_format: i64, view_count: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferCreateMultipass < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, textures: &'a Array < Rid >, passes: &'a Array < Gd < crate::classes::RdFramebufferPass > >,) -> Self {
        let validate_with_format = - 1i64;
        let view_count = 1u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, textures: CowArg::Borrowed(textures), passes: CowArg::Borrowed(passes), validate_with_format: validate_with_format, view_count: view_count,
        }
    }
    #[inline]
    pub fn validate_with_format(self, validate_with_format: i64) -> Self {
        Self {
            validate_with_format: validate_with_format, .. self
        }
    }
    #[inline]
    pub fn view_count(self, view_count: u32) -> Self {
        Self {
            view_count: view_count, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, textures, passes, validate_with_format, view_count,
        }
        = self;
        re_export::RenderingDevice::framebuffer_create_multipass_full(surround_object, textures.cow_as_arg(), passes.cow_as_arg(), validate_with_format, view_count,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::framebuffer_create_empty_ex`][super::RenderingDevice::framebuffer_create_empty_ex]."]
#[must_use]
pub struct ExFramebufferCreateEmpty < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, size: Vector2i, samples: crate::classes::rendering_device::TextureSamples, validate_with_format: i64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExFramebufferCreateEmpty < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size: Vector2i,) -> Self {
        let samples = crate::obj::EngineEnum::from_ord(0);
        let validate_with_format = - 1i64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size: size, samples: samples, validate_with_format: validate_with_format,
        }
    }
    #[inline]
    pub fn samples(self, samples: crate::classes::rendering_device::TextureSamples) -> Self {
        Self {
            samples: samples, .. self
        }
    }
    #[inline]
    pub fn validate_with_format(self, validate_with_format: i64) -> Self {
        Self {
            validate_with_format: validate_with_format, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, size, samples, validate_with_format,
        }
        = self;
        re_export::RenderingDevice::framebuffer_create_empty_full(surround_object, size, samples, validate_with_format,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::vertex_buffer_create_ex`][super::RenderingDevice::vertex_buffer_create_ex]."]
#[must_use]
pub struct ExVertexBufferCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32, data: CowArg < 'a, PackedByteArray >, creation_bits: crate::classes::rendering_device::BufferCreationBits,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExVertexBufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32,) -> Self {
        let data = PackedByteArray::new();
        let creation_bits = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size_bytes: size_bytes, data: CowArg::Owned(data), creation_bits: creation_bits,
        }
    }
    #[inline]
    pub fn data(self, data: &'a PackedByteArray) -> Self {
        Self {
            data: CowArg::Borrowed(data), .. self
        }
    }
    #[inline]
    pub fn creation_bits(self, creation_bits: crate::classes::rendering_device::BufferCreationBits) -> Self {
        Self {
            creation_bits: creation_bits, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, size_bytes, data, creation_bits,
        }
        = self;
        re_export::RenderingDevice::vertex_buffer_create_full(surround_object, size_bytes, data.cow_as_arg(), creation_bits,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::vertex_array_create_ex`][super::RenderingDevice::vertex_array_create_ex]."]
#[must_use]
pub struct ExVertexArrayCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, vertex_count: u32, vertex_format: i64, src_buffers: CowArg < 'a, Array < Rid > >, offsets: CowArg < 'a, PackedInt64Array >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExVertexArrayCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, vertex_count: u32, vertex_format: i64, src_buffers: &'a Array < Rid >,) -> Self {
        let offsets = PackedInt64Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, vertex_count: vertex_count, vertex_format: vertex_format, src_buffers: CowArg::Borrowed(src_buffers), offsets: CowArg::Owned(offsets),
        }
    }
    #[inline]
    pub fn offsets(self, offsets: &'a PackedInt64Array) -> Self {
        Self {
            offsets: CowArg::Borrowed(offsets), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, vertex_count, vertex_format, src_buffers, offsets,
        }
        = self;
        re_export::RenderingDevice::vertex_array_create_full(surround_object, vertex_count, vertex_format, src_buffers.cow_as_arg(), offsets.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::index_buffer_create_ex`][super::RenderingDevice::index_buffer_create_ex]."]
#[must_use]
pub struct ExIndexBufferCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, size_indices: u32, format: crate::classes::rendering_device::IndexBufferFormat, data: CowArg < 'a, PackedByteArray >, use_restart_indices: bool, creation_bits: crate::classes::rendering_device::BufferCreationBits,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExIndexBufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size_indices: u32, format: crate::classes::rendering_device::IndexBufferFormat,) -> Self {
        let data = PackedByteArray::new();
        let use_restart_indices = false;
        let creation_bits = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size_indices: size_indices, format: format, data: CowArg::Owned(data), use_restart_indices: use_restart_indices, creation_bits: creation_bits,
        }
    }
    #[inline]
    pub fn data(self, data: &'a PackedByteArray) -> Self {
        Self {
            data: CowArg::Borrowed(data), .. self
        }
    }
    #[inline]
    pub fn use_restart_indices(self, use_restart_indices: bool) -> Self {
        Self {
            use_restart_indices: use_restart_indices, .. self
        }
    }
    #[inline]
    pub fn creation_bits(self, creation_bits: crate::classes::rendering_device::BufferCreationBits) -> Self {
        Self {
            creation_bits: creation_bits, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, size_indices, format, data, use_restart_indices, creation_bits,
        }
        = self;
        re_export::RenderingDevice::index_buffer_create_full(surround_object, size_indices, format, data.cow_as_arg(), use_restart_indices, creation_bits,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::shader_compile_spirv_from_source_ex`][super::RenderingDevice::shader_compile_spirv_from_source_ex]."]
#[must_use]
pub struct ExShaderCompileSpirvFromSource < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, shader_source: CowArg < 'a, Option < Gd < crate::classes::RdShaderSource > > >, allow_cache: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShaderCompileSpirvFromSource < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, shader_source: impl AsArg < Option < Gd < crate::classes::RdShaderSource >> > + 'a,) -> Self {
        let allow_cache = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shader_source: shader_source.into_arg(), allow_cache: allow_cache,
        }
    }
    #[inline]
    pub fn allow_cache(self, allow_cache: bool) -> Self {
        Self {
            allow_cache: allow_cache, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::RdShaderSpirv > > {
        let Self {
            _phantom, surround_object, shader_source, allow_cache,
        }
        = self;
        re_export::RenderingDevice::shader_compile_spirv_from_source_full(surround_object, shader_source, allow_cache,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::shader_compile_binary_from_spirv_ex`][super::RenderingDevice::shader_compile_binary_from_spirv_ex]."]
#[must_use]
pub struct ExShaderCompileBinaryFromSpirv < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, spirv_data: CowArg < 'a, Option < Gd < crate::classes::RdShaderSpirv > > >, name: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShaderCompileBinaryFromSpirv < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, spirv_data: impl AsArg < Option < Gd < crate::classes::RdShaderSpirv >> > + 'a,) -> Self {
        let name = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, spirv_data: spirv_data.into_arg(), name: CowArg::Owned(name),
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < GString > + 'a) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        let Self {
            _phantom, surround_object, spirv_data, name,
        }
        = self;
        re_export::RenderingDevice::shader_compile_binary_from_spirv_full(surround_object, spirv_data, name,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::shader_create_from_spirv_ex`][super::RenderingDevice::shader_create_from_spirv_ex]."]
#[must_use]
pub struct ExShaderCreateFromSpirv < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, spirv_data: CowArg < 'a, Option < Gd < crate::classes::RdShaderSpirv > > >, name: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShaderCreateFromSpirv < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, spirv_data: impl AsArg < Option < Gd < crate::classes::RdShaderSpirv >> > + 'a,) -> Self {
        let name = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, spirv_data: spirv_data.into_arg(), name: CowArg::Owned(name),
        }
    }
    #[inline]
    pub fn name(self, name: impl AsArg < GString > + 'a) -> Self {
        Self {
            name: name.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, spirv_data, name,
        }
        = self;
        re_export::RenderingDevice::shader_create_from_spirv_full(surround_object, spirv_data, name,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::shader_create_from_bytecode_ex`][super::RenderingDevice::shader_create_from_bytecode_ex]."]
#[must_use]
pub struct ExShaderCreateFromBytecode < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, binary_data: CowArg < 'a, PackedByteArray >, placeholder_rid: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShaderCreateFromBytecode < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, binary_data: &'a PackedByteArray,) -> Self {
        let placeholder_rid = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, binary_data: CowArg::Borrowed(binary_data), placeholder_rid: placeholder_rid,
        }
    }
    #[inline]
    pub fn placeholder_rid(self, placeholder_rid: Rid) -> Self {
        Self {
            placeholder_rid: placeholder_rid, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, binary_data, placeholder_rid,
        }
        = self;
        re_export::RenderingDevice::shader_create_from_bytecode_full(surround_object, binary_data.cow_as_arg(), placeholder_rid,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::uniform_buffer_create_ex`][super::RenderingDevice::uniform_buffer_create_ex]."]
#[must_use]
pub struct ExUniformBufferCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32, data: CowArg < 'a, PackedByteArray >, creation_bits: crate::classes::rendering_device::BufferCreationBits,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExUniformBufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32,) -> Self {
        let data = PackedByteArray::new();
        let creation_bits = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size_bytes: size_bytes, data: CowArg::Owned(data), creation_bits: creation_bits,
        }
    }
    #[inline]
    pub fn data(self, data: &'a PackedByteArray) -> Self {
        Self {
            data: CowArg::Borrowed(data), .. self
        }
    }
    #[inline]
    pub fn creation_bits(self, creation_bits: crate::classes::rendering_device::BufferCreationBits) -> Self {
        Self {
            creation_bits: creation_bits, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, size_bytes, data, creation_bits,
        }
        = self;
        re_export::RenderingDevice::uniform_buffer_create_full(surround_object, size_bytes, data.cow_as_arg(), creation_bits,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::storage_buffer_create_ex`][super::RenderingDevice::storage_buffer_create_ex]."]
#[must_use]
pub struct ExStorageBufferCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32, data: CowArg < 'a, PackedByteArray >, usage: crate::classes::rendering_device::StorageBufferUsage, creation_bits: crate::classes::rendering_device::BufferCreationBits,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExStorageBufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32,) -> Self {
        let data = PackedByteArray::new();
        let usage = crate::obj::EngineBitfield::from_ord(0);
        let creation_bits = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size_bytes: size_bytes, data: CowArg::Owned(data), usage: usage, creation_bits: creation_bits,
        }
    }
    #[inline]
    pub fn data(self, data: &'a PackedByteArray) -> Self {
        Self {
            data: CowArg::Borrowed(data), .. self
        }
    }
    #[inline]
    pub fn usage(self, usage: crate::classes::rendering_device::StorageBufferUsage) -> Self {
        Self {
            usage: usage, .. self
        }
    }
    #[inline]
    pub fn creation_bits(self, creation_bits: crate::classes::rendering_device::BufferCreationBits) -> Self {
        Self {
            creation_bits: creation_bits, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, size_bytes, data, usage, creation_bits,
        }
        = self;
        re_export::RenderingDevice::storage_buffer_create_full(surround_object, size_bytes, data.cow_as_arg(), usage, creation_bits,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::texture_buffer_create_ex`][super::RenderingDevice::texture_buffer_create_ex]."]
#[must_use]
pub struct ExTextureBufferCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32, format: crate::classes::rendering_device::DataFormat, data: CowArg < 'a, PackedByteArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureBufferCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, size_bytes: u32, format: crate::classes::rendering_device::DataFormat,) -> Self {
        let data = PackedByteArray::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, size_bytes: size_bytes, format: format, data: CowArg::Owned(data),
        }
    }
    #[inline]
    pub fn data(self, data: &'a PackedByteArray) -> Self {
        Self {
            data: CowArg::Borrowed(data), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, size_bytes, format, data,
        }
        = self;
        re_export::RenderingDevice::texture_buffer_create_full(surround_object, size_bytes, format, data.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::buffer_get_data_ex`][super::RenderingDevice::buffer_get_data_ex]."]
#[must_use]
pub struct ExBufferGetData < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, buffer: Rid, offset_bytes: u32, size_bytes: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBufferGetData < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, buffer: Rid,) -> Self {
        let offset_bytes = 0u32;
        let size_bytes = 0u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, buffer: buffer, offset_bytes: offset_bytes, size_bytes: size_bytes,
        }
    }
    #[inline]
    pub fn offset_bytes(self, offset_bytes: u32) -> Self {
        Self {
            offset_bytes: offset_bytes, .. self
        }
    }
    #[inline]
    pub fn size_bytes(self, size_bytes: u32) -> Self {
        Self {
            size_bytes: size_bytes, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        let Self {
            _phantom, surround_object, buffer, offset_bytes, size_bytes,
        }
        = self;
        re_export::RenderingDevice::buffer_get_data_full(surround_object, buffer, offset_bytes, size_bytes,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::buffer_get_data_async_ex`][super::RenderingDevice::buffer_get_data_async_ex]."]
#[must_use]
pub struct ExBufferGetDataAsync < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, buffer: Rid, callback: CowArg < 'a, Callable >, offset_bytes: u32, size_bytes: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBufferGetDataAsync < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, buffer: Rid, callback: &'a Callable,) -> Self {
        let offset_bytes = 0u32;
        let size_bytes = 0u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, buffer: buffer, callback: CowArg::Borrowed(callback), offset_bytes: offset_bytes, size_bytes: size_bytes,
        }
    }
    #[inline]
    pub fn offset_bytes(self, offset_bytes: u32) -> Self {
        Self {
            offset_bytes: offset_bytes, .. self
        }
    }
    #[inline]
    pub fn size_bytes(self, size_bytes: u32) -> Self {
        Self {
            size_bytes: size_bytes, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, buffer, callback, offset_bytes, size_bytes,
        }
        = self;
        re_export::RenderingDevice::buffer_get_data_async_full(surround_object, buffer, callback.cow_as_arg(), offset_bytes, size_bytes,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::render_pipeline_create_ex`][super::RenderingDevice::render_pipeline_create_ex]."]
#[must_use]
pub struct ExRenderPipelineCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, shader: Rid, framebuffer_format: i64, vertex_format: i64, primitive: crate::classes::rendering_device::RenderPrimitive, rasterization_state: CowArg < 'a, Option < Gd < crate::classes::RdPipelineRasterizationState > > >, multisample_state: CowArg < 'a, Option < Gd < crate::classes::RdPipelineMultisampleState > > >, stencil_state: CowArg < 'a, Option < Gd < crate::classes::RdPipelineDepthStencilState > > >, color_blend_state: CowArg < 'a, Option < Gd < crate::classes::RdPipelineColorBlendState > > >, dynamic_state_flags: crate::classes::rendering_device::PipelineDynamicStateFlags, for_render_pass: u32, specialization_constants: CowArg < 'a, Array < Gd < crate::classes::RdPipelineSpecializationConstant > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRenderPipelineCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, shader: Rid, framebuffer_format: i64, vertex_format: i64, primitive: crate::classes::rendering_device::RenderPrimitive, rasterization_state: impl AsArg < Option < Gd < crate::classes::RdPipelineRasterizationState >> > + 'a, multisample_state: impl AsArg < Option < Gd < crate::classes::RdPipelineMultisampleState >> > + 'a, stencil_state: impl AsArg < Option < Gd < crate::classes::RdPipelineDepthStencilState >> > + 'a, color_blend_state: impl AsArg < Option < Gd < crate::classes::RdPipelineColorBlendState >> > + 'a,) -> Self {
        let dynamic_state_flags = crate::obj::EngineBitfield::from_ord(0);
        let for_render_pass = 0u32;
        let specialization_constants = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shader: shader, framebuffer_format: framebuffer_format, vertex_format: vertex_format, primitive: primitive, rasterization_state: rasterization_state.into_arg(), multisample_state: multisample_state.into_arg(), stencil_state: stencil_state.into_arg(), color_blend_state: color_blend_state.into_arg(), dynamic_state_flags: dynamic_state_flags, for_render_pass: for_render_pass, specialization_constants: CowArg::Owned(specialization_constants),
        }
    }
    #[inline]
    pub fn dynamic_state_flags(self, dynamic_state_flags: crate::classes::rendering_device::PipelineDynamicStateFlags) -> Self {
        Self {
            dynamic_state_flags: dynamic_state_flags, .. self
        }
    }
    #[inline]
    pub fn for_render_pass(self, for_render_pass: u32) -> Self {
        Self {
            for_render_pass: for_render_pass, .. self
        }
    }
    #[inline]
    pub fn specialization_constants(self, specialization_constants: &'a Array < Gd < crate::classes::RdPipelineSpecializationConstant > >) -> Self {
        Self {
            specialization_constants: CowArg::Borrowed(specialization_constants), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, shader, framebuffer_format, vertex_format, primitive, rasterization_state, multisample_state, stencil_state, color_blend_state, dynamic_state_flags, for_render_pass, specialization_constants,
        }
        = self;
        re_export::RenderingDevice::render_pipeline_create_full(surround_object, shader, framebuffer_format, vertex_format, primitive, rasterization_state, multisample_state, stencil_state, color_blend_state, dynamic_state_flags, for_render_pass, specialization_constants.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::compute_pipeline_create_ex`][super::RenderingDevice::compute_pipeline_create_ex]."]
#[must_use]
pub struct ExComputePipelineCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, shader: Rid, specialization_constants: CowArg < 'a, Array < Gd < crate::classes::RdPipelineSpecializationConstant > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExComputePipelineCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, shader: Rid,) -> Self {
        let specialization_constants = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shader: shader, specialization_constants: CowArg::Owned(specialization_constants),
        }
    }
    #[inline]
    pub fn specialization_constants(self, specialization_constants: &'a Array < Gd < crate::classes::RdPipelineSpecializationConstant > >) -> Self {
        Self {
            specialization_constants: CowArg::Borrowed(specialization_constants), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, shader, specialization_constants,
        }
        = self;
        re_export::RenderingDevice::compute_pipeline_create_full(surround_object, shader, specialization_constants.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::screen_get_width_ex`][super::RenderingDevice::screen_get_width_ex]."]
#[must_use]
pub struct ExScreenGetWidth < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RenderingDevice, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetWidth < 'a > {
    fn new(surround_object: &'a re_export::RenderingDevice,) -> Self {
        let screen = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::RenderingDevice::screen_get_width_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::screen_get_height_ex`][super::RenderingDevice::screen_get_height_ex]."]
#[must_use]
pub struct ExScreenGetHeight < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RenderingDevice, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetHeight < 'a > {
    fn new(surround_object: &'a re_export::RenderingDevice,) -> Self {
        let screen = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::RenderingDevice::screen_get_height_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::screen_get_framebuffer_format_ex`][super::RenderingDevice::screen_get_framebuffer_format_ex]."]
#[must_use]
pub struct ExScreenGetFramebufferFormat < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RenderingDevice, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScreenGetFramebufferFormat < 'a > {
    fn new(surround_object: &'a re_export::RenderingDevice,) -> Self {
        let screen = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, screen,
        }
        = self;
        re_export::RenderingDevice::screen_get_framebuffer_format_full(surround_object, screen,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_begin_for_screen_ex`][super::RenderingDevice::draw_list_begin_for_screen_ex]."]
#[must_use]
pub struct ExDrawListBeginForScreen < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, screen: i32, clear_color: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListBeginForScreen < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice,) -> Self {
        let screen = 0i32;
        let clear_color = Color::from_rgba(0 as _, 0 as _, 0 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, screen: screen, clear_color: clear_color,
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn clear_color(self, clear_color: Color) -> Self {
        Self {
            clear_color: clear_color, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, screen, clear_color,
        }
        = self;
        re_export::RenderingDevice::draw_list_begin_for_screen_full(surround_object, screen, clear_color,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_begin_ex`][super::RenderingDevice::draw_list_begin_ex]."]
#[must_use]
pub struct ExDrawListBegin < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, framebuffer: Rid, draw_flags: crate::classes::rendering_device::DrawFlags, clear_color_values: CowArg < 'a, PackedColorArray >, clear_depth_value: f32, clear_stencil_value: u32, region: Rect2, breadcrumb: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListBegin < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, framebuffer: Rid,) -> Self {
        let draw_flags = crate::obj::EngineBitfield::from_ord(0);
        let clear_color_values = PackedColorArray::new();
        let clear_depth_value = 1f32;
        let clear_stencil_value = 0u32;
        let region = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let breadcrumb = 0u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, framebuffer: framebuffer, draw_flags: draw_flags, clear_color_values: CowArg::Owned(clear_color_values), clear_depth_value: clear_depth_value, clear_stencil_value: clear_stencil_value, region: region, breadcrumb: breadcrumb,
        }
    }
    #[inline]
    pub fn draw_flags(self, draw_flags: crate::classes::rendering_device::DrawFlags) -> Self {
        Self {
            draw_flags: draw_flags, .. self
        }
    }
    #[inline]
    pub fn clear_color_values(self, clear_color_values: &'a PackedColorArray) -> Self {
        Self {
            clear_color_values: CowArg::Borrowed(clear_color_values), .. self
        }
    }
    #[inline]
    pub fn clear_depth_value(self, clear_depth_value: f32) -> Self {
        Self {
            clear_depth_value: clear_depth_value, .. self
        }
    }
    #[inline]
    pub fn clear_stencil_value(self, clear_stencil_value: u32) -> Self {
        Self {
            clear_stencil_value: clear_stencil_value, .. self
        }
    }
    #[inline]
    pub fn region(self, region: Rect2) -> Self {
        Self {
            region: region, .. self
        }
    }
    #[inline]
    pub fn breadcrumb(self, breadcrumb: u32) -> Self {
        Self {
            breadcrumb: breadcrumb, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, framebuffer, draw_flags, clear_color_values, clear_depth_value, clear_stencil_value, region, breadcrumb,
        }
        = self;
        re_export::RenderingDevice::draw_list_begin_full(surround_object, framebuffer, draw_flags, clear_color_values.cow_as_arg(), clear_depth_value, clear_stencil_value, region, breadcrumb,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_begin_split_ex`][super::RenderingDevice::draw_list_begin_split_ex]."]
#[must_use]
pub struct ExDrawListBeginSplit < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, framebuffer: Rid, splits: u32, initial_color_action: crate::classes::rendering_device::InitialAction, final_color_action: crate::classes::rendering_device::FinalAction, initial_depth_action: crate::classes::rendering_device::InitialAction, final_depth_action: crate::classes::rendering_device::FinalAction, clear_color_values: CowArg < 'a, PackedColorArray >, clear_depth: f32, clear_stencil: u32, region: Rect2, storage_textures: CowArg < 'a, Array < Rid > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListBeginSplit < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, framebuffer: Rid, splits: u32, initial_color_action: crate::classes::rendering_device::InitialAction, final_color_action: crate::classes::rendering_device::FinalAction, initial_depth_action: crate::classes::rendering_device::InitialAction, final_depth_action: crate::classes::rendering_device::FinalAction,) -> Self {
        let clear_color_values = PackedColorArray::new();
        let clear_depth = 1f32;
        let clear_stencil = 0u32;
        let region = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let storage_textures = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, framebuffer: framebuffer, splits: splits, initial_color_action: initial_color_action, final_color_action: final_color_action, initial_depth_action: initial_depth_action, final_depth_action: final_depth_action, clear_color_values: CowArg::Owned(clear_color_values), clear_depth: clear_depth, clear_stencil: clear_stencil, region: region, storage_textures: CowArg::Owned(storage_textures),
        }
    }
    #[inline]
    pub fn clear_color_values(self, clear_color_values: &'a PackedColorArray) -> Self {
        Self {
            clear_color_values: CowArg::Borrowed(clear_color_values), .. self
        }
    }
    #[inline]
    pub fn clear_depth(self, clear_depth: f32) -> Self {
        Self {
            clear_depth: clear_depth, .. self
        }
    }
    #[inline]
    pub fn clear_stencil(self, clear_stencil: u32) -> Self {
        Self {
            clear_stencil: clear_stencil, .. self
        }
    }
    #[inline]
    pub fn region(self, region: Rect2) -> Self {
        Self {
            region: region, .. self
        }
    }
    #[inline]
    pub fn storage_textures(self, storage_textures: &'a Array < Rid >) -> Self {
        Self {
            storage_textures: CowArg::Borrowed(storage_textures), .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt64Array {
        let Self {
            _phantom, surround_object, framebuffer, splits, initial_color_action, final_color_action, initial_depth_action, final_depth_action, clear_color_values, clear_depth, clear_stencil, region, storage_textures,
        }
        = self;
        re_export::RenderingDevice::draw_list_begin_split_full(surround_object, framebuffer, splits, initial_color_action, final_color_action, initial_depth_action, final_depth_action, clear_color_values.cow_as_arg(), clear_depth, clear_stencil, region, storage_textures.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_draw_ex`][super::RenderingDevice::draw_list_draw_ex]."]
#[must_use]
pub struct ExDrawListDraw < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, draw_list: i64, use_indices: bool, instances: u32, procedural_vertex_count: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListDraw < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, draw_list: i64, use_indices: bool, instances: u32,) -> Self {
        let procedural_vertex_count = 0u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, draw_list: draw_list, use_indices: use_indices, instances: instances, procedural_vertex_count: procedural_vertex_count,
        }
    }
    #[inline]
    pub fn procedural_vertex_count(self, procedural_vertex_count: u32) -> Self {
        Self {
            procedural_vertex_count: procedural_vertex_count, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, draw_list, use_indices, instances, procedural_vertex_count,
        }
        = self;
        re_export::RenderingDevice::draw_list_draw_full(surround_object, draw_list, use_indices, instances, procedural_vertex_count,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_draw_indirect_ex`][super::RenderingDevice::draw_list_draw_indirect_ex]."]
#[must_use]
pub struct ExDrawListDrawIndirect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, draw_list: i64, use_indices: bool, buffer: Rid, offset: u32, draw_count: u32, stride: u32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListDrawIndirect < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, draw_list: i64, use_indices: bool, buffer: Rid,) -> Self {
        let offset = 0u32;
        let draw_count = 1u32;
        let stride = 0u32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, draw_list: draw_list, use_indices: use_indices, buffer: buffer, offset: offset, draw_count: draw_count, stride: stride,
        }
    }
    #[inline]
    pub fn offset(self, offset: u32) -> Self {
        Self {
            offset: offset, .. self
        }
    }
    #[inline]
    pub fn draw_count(self, draw_count: u32) -> Self {
        Self {
            draw_count: draw_count, .. self
        }
    }
    #[inline]
    pub fn stride(self, stride: u32) -> Self {
        Self {
            stride: stride, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, draw_list, use_indices, buffer, offset, draw_count, stride,
        }
        = self;
        re_export::RenderingDevice::draw_list_draw_indirect_full(surround_object, draw_list, use_indices, buffer, offset, draw_count, stride,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::draw_list_enable_scissor_ex`][super::RenderingDevice::draw_list_enable_scissor_ex]."]
#[must_use]
pub struct ExDrawListEnableScissor < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, draw_list: i64, rect: Rect2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDrawListEnableScissor < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice, draw_list: i64,) -> Self {
        let rect = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, draw_list: draw_list, rect: rect,
        }
    }
    #[inline]
    pub fn rect(self, rect: Rect2) -> Self {
        Self {
            rect: rect, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, draw_list, rect,
        }
        = self;
        re_export::RenderingDevice::draw_list_enable_scissor_full(surround_object, draw_list, rect,)
    }
}
#[doc = "Default-param extender for [`RenderingDevice::barrier_ex`][super::RenderingDevice::barrier_ex]."]
#[must_use]
pub struct ExBarrier < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingDevice, from: crate::classes::rendering_device::BarrierMask, to: crate::classes::rendering_device::BarrierMask,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBarrier < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingDevice,) -> Self {
        let from = crate::obj::EngineBitfield::from_ord(32767);
        let to = crate::obj::EngineBitfield::from_ord(32767);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from, to: to,
        }
    }
    #[inline]
    pub fn from(self, from: crate::classes::rendering_device::BarrierMask) -> Self {
        Self {
            from: from, .. self
        }
    }
    #[inline]
    pub fn to(self, to: crate::classes::rendering_device::BarrierMask) -> Self {
        Self {
            to: to, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from, to,
        }
        = self;
        re_export::RenderingDevice::barrier_full(surround_object, from, to,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DeviceType {
    ord: i32
}
impl DeviceType {
    #[doc(alias = "DEVICE_TYPE_OTHER")]
    #[doc = "Godot enumerator name: `DEVICE_TYPE_OTHER`"]
    pub const OTHER: DeviceType = DeviceType {
        ord: 0i32
    };
    #[doc(alias = "DEVICE_TYPE_INTEGRATED_GPU")]
    #[doc = "Godot enumerator name: `DEVICE_TYPE_INTEGRATED_GPU`"]
    pub const INTEGRATED_GPU: DeviceType = DeviceType {
        ord: 1i32
    };
    #[doc(alias = "DEVICE_TYPE_DISCRETE_GPU")]
    #[doc = "Godot enumerator name: `DEVICE_TYPE_DISCRETE_GPU`"]
    pub const DISCRETE_GPU: DeviceType = DeviceType {
        ord: 2i32
    };
    #[doc(alias = "DEVICE_TYPE_VIRTUAL_GPU")]
    #[doc = "Godot enumerator name: `DEVICE_TYPE_VIRTUAL_GPU`"]
    pub const VIRTUAL_GPU: DeviceType = DeviceType {
        ord: 3i32
    };
    #[doc(alias = "DEVICE_TYPE_CPU")]
    #[doc = "Godot enumerator name: `DEVICE_TYPE_CPU`"]
    pub const CPU: DeviceType = DeviceType {
        ord: 4i32
    };
    #[doc(alias = "DEVICE_TYPE_MAX")]
    #[doc = "Godot enumerator name: `DEVICE_TYPE_MAX`"]
    pub const MAX: DeviceType = DeviceType {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DeviceType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DeviceType {
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
            Self::OTHER => "OTHER", Self::INTEGRATED_GPU => "INTEGRATED_GPU", Self::DISCRETE_GPU => "DISCRETE_GPU", Self::VIRTUAL_GPU => "VIRTUAL_GPU", Self::CPU => "CPU", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DeviceType::OTHER, DeviceType::INTEGRATED_GPU, DeviceType::DISCRETE_GPU, DeviceType::VIRTUAL_GPU, DeviceType::CPU]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DeviceType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OTHER", "DEVICE_TYPE_OTHER", DeviceType::OTHER), crate::meta::inspect::EnumConstant::new("INTEGRATED_GPU", "DEVICE_TYPE_INTEGRATED_GPU", DeviceType::INTEGRATED_GPU), crate::meta::inspect::EnumConstant::new("DISCRETE_GPU", "DEVICE_TYPE_DISCRETE_GPU", DeviceType::DISCRETE_GPU), crate::meta::inspect::EnumConstant::new("VIRTUAL_GPU", "DEVICE_TYPE_VIRTUAL_GPU", DeviceType::VIRTUAL_GPU), crate::meta::inspect::EnumConstant::new("CPU", "DEVICE_TYPE_CPU", DeviceType::CPU), crate::meta::inspect::EnumConstant::new("MAX", "DEVICE_TYPE_MAX", DeviceType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for DeviceType {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for DeviceType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DeviceType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DeviceType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DriverResource {
    ord: i32
}
impl DriverResource {
    #[doc(alias = "DRIVER_RESOURCE_LOGICAL_DEVICE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_LOGICAL_DEVICE`"]
    pub const LOGICAL_DEVICE: DriverResource = DriverResource {
        ord: 0i32
    };
    #[doc(alias = "DRIVER_RESOURCE_PHYSICAL_DEVICE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_PHYSICAL_DEVICE`"]
    pub const PHYSICAL_DEVICE: DriverResource = DriverResource {
        ord: 1i32
    };
    #[doc(alias = "DRIVER_RESOURCE_TOPMOST_OBJECT")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_TOPMOST_OBJECT`"]
    pub const TOPMOST_OBJECT: DriverResource = DriverResource {
        ord: 2i32
    };
    #[doc(alias = "DRIVER_RESOURCE_COMMAND_QUEUE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_COMMAND_QUEUE`"]
    pub const COMMAND_QUEUE: DriverResource = DriverResource {
        ord: 3i32
    };
    #[doc(alias = "DRIVER_RESOURCE_QUEUE_FAMILY")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_QUEUE_FAMILY`"]
    pub const QUEUE_FAMILY: DriverResource = DriverResource {
        ord: 4i32
    };
    #[doc(alias = "DRIVER_RESOURCE_TEXTURE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_TEXTURE`"]
    pub const TEXTURE: DriverResource = DriverResource {
        ord: 5i32
    };
    #[doc(alias = "DRIVER_RESOURCE_TEXTURE_VIEW")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_TEXTURE_VIEW`"]
    pub const TEXTURE_VIEW: DriverResource = DriverResource {
        ord: 6i32
    };
    #[doc(alias = "DRIVER_RESOURCE_TEXTURE_DATA_FORMAT")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_TEXTURE_DATA_FORMAT`"]
    pub const TEXTURE_DATA_FORMAT: DriverResource = DriverResource {
        ord: 7i32
    };
    #[doc(alias = "DRIVER_RESOURCE_SAMPLER")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_SAMPLER`"]
    pub const SAMPLER: DriverResource = DriverResource {
        ord: 8i32
    };
    #[doc(alias = "DRIVER_RESOURCE_UNIFORM_SET")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_UNIFORM_SET`"]
    pub const UNIFORM_SET: DriverResource = DriverResource {
        ord: 9i32
    };
    #[doc(alias = "DRIVER_RESOURCE_BUFFER")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_BUFFER`"]
    pub const BUFFER: DriverResource = DriverResource {
        ord: 10i32
    };
    #[doc(alias = "DRIVER_RESOURCE_COMPUTE_PIPELINE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_COMPUTE_PIPELINE`"]
    pub const COMPUTE_PIPELINE: DriverResource = DriverResource {
        ord: 11i32
    };
    #[doc(alias = "DRIVER_RESOURCE_RENDER_PIPELINE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_RENDER_PIPELINE`"]
    pub const RENDER_PIPELINE: DriverResource = DriverResource {
        ord: 12i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_DEVICE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_DEVICE`"]
    pub const VULKAN_DEVICE: DriverResource = DriverResource {
        ord: 0i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_PHYSICAL_DEVICE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_PHYSICAL_DEVICE`"]
    pub const VULKAN_PHYSICAL_DEVICE: DriverResource = DriverResource {
        ord: 1i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_INSTANCE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_INSTANCE`"]
    pub const VULKAN_INSTANCE: DriverResource = DriverResource {
        ord: 2i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_QUEUE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_QUEUE`"]
    pub const VULKAN_QUEUE: DriverResource = DriverResource {
        ord: 3i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_QUEUE_FAMILY_INDEX")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_QUEUE_FAMILY_INDEX`"]
    pub const VULKAN_QUEUE_FAMILY_INDEX: DriverResource = DriverResource {
        ord: 4i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_IMAGE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_IMAGE`"]
    pub const VULKAN_IMAGE: DriverResource = DriverResource {
        ord: 5i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_IMAGE_VIEW")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_IMAGE_VIEW`"]
    pub const VULKAN_IMAGE_VIEW: DriverResource = DriverResource {
        ord: 6i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT`"]
    pub const VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT: DriverResource = DriverResource {
        ord: 7i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_SAMPLER")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_SAMPLER`"]
    pub const VULKAN_SAMPLER: DriverResource = DriverResource {
        ord: 8i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_DESCRIPTOR_SET")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_DESCRIPTOR_SET`"]
    pub const VULKAN_DESCRIPTOR_SET: DriverResource = DriverResource {
        ord: 9i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_BUFFER")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_BUFFER`"]
    pub const VULKAN_BUFFER: DriverResource = DriverResource {
        ord: 10i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_COMPUTE_PIPELINE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_COMPUTE_PIPELINE`"]
    pub const VULKAN_COMPUTE_PIPELINE: DriverResource = DriverResource {
        ord: 11i32
    };
    #[doc(alias = "DRIVER_RESOURCE_VULKAN_RENDER_PIPELINE")]
    #[doc = "Godot enumerator name: `DRIVER_RESOURCE_VULKAN_RENDER_PIPELINE`"]
    pub const VULKAN_RENDER_PIPELINE: DriverResource = DriverResource {
        ord: 12i32
    };
    
}
impl std::fmt::Debug for DriverResource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DriverResource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DriverResource {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 => Some(Self {
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
            Self::LOGICAL_DEVICE => "LOGICAL_DEVICE", Self::PHYSICAL_DEVICE => "PHYSICAL_DEVICE", Self::TOPMOST_OBJECT => "TOPMOST_OBJECT", Self::COMMAND_QUEUE => "COMMAND_QUEUE", Self::QUEUE_FAMILY => "QUEUE_FAMILY", Self::TEXTURE => "TEXTURE", Self::TEXTURE_VIEW => "TEXTURE_VIEW", Self::TEXTURE_DATA_FORMAT => "TEXTURE_DATA_FORMAT", Self::SAMPLER => "SAMPLER", Self::UNIFORM_SET => "UNIFORM_SET", Self::BUFFER => "BUFFER", Self::COMPUTE_PIPELINE => "COMPUTE_PIPELINE", Self::RENDER_PIPELINE => "RENDER_PIPELINE", Self::VULKAN_DEVICE => "VULKAN_DEVICE", Self::VULKAN_PHYSICAL_DEVICE => "VULKAN_PHYSICAL_DEVICE", Self::VULKAN_INSTANCE => "VULKAN_INSTANCE", Self::VULKAN_QUEUE => "VULKAN_QUEUE", Self::VULKAN_QUEUE_FAMILY_INDEX => "VULKAN_QUEUE_FAMILY_INDEX", Self::VULKAN_IMAGE => "VULKAN_IMAGE", Self::VULKAN_IMAGE_VIEW => "VULKAN_IMAGE_VIEW", Self::VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT => "VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT", Self::VULKAN_SAMPLER => "VULKAN_SAMPLER", Self::VULKAN_DESCRIPTOR_SET => "VULKAN_DESCRIPTOR_SET", Self::VULKAN_BUFFER => "VULKAN_BUFFER", Self::VULKAN_COMPUTE_PIPELINE => "VULKAN_COMPUTE_PIPELINE", Self::VULKAN_RENDER_PIPELINE => "VULKAN_RENDER_PIPELINE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DriverResource::LOGICAL_DEVICE, DriverResource::PHYSICAL_DEVICE, DriverResource::TOPMOST_OBJECT, DriverResource::COMMAND_QUEUE, DriverResource::QUEUE_FAMILY, DriverResource::TEXTURE, DriverResource::TEXTURE_VIEW, DriverResource::TEXTURE_DATA_FORMAT, DriverResource::SAMPLER, DriverResource::UNIFORM_SET, DriverResource::BUFFER, DriverResource::COMPUTE_PIPELINE, DriverResource::RENDER_PIPELINE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DriverResource >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LOGICAL_DEVICE", "DRIVER_RESOURCE_LOGICAL_DEVICE", DriverResource::LOGICAL_DEVICE), crate::meta::inspect::EnumConstant::new("PHYSICAL_DEVICE", "DRIVER_RESOURCE_PHYSICAL_DEVICE", DriverResource::PHYSICAL_DEVICE), crate::meta::inspect::EnumConstant::new("TOPMOST_OBJECT", "DRIVER_RESOURCE_TOPMOST_OBJECT", DriverResource::TOPMOST_OBJECT), crate::meta::inspect::EnumConstant::new("COMMAND_QUEUE", "DRIVER_RESOURCE_COMMAND_QUEUE", DriverResource::COMMAND_QUEUE), crate::meta::inspect::EnumConstant::new("QUEUE_FAMILY", "DRIVER_RESOURCE_QUEUE_FAMILY", DriverResource::QUEUE_FAMILY), crate::meta::inspect::EnumConstant::new("TEXTURE", "DRIVER_RESOURCE_TEXTURE", DriverResource::TEXTURE), crate::meta::inspect::EnumConstant::new("TEXTURE_VIEW", "DRIVER_RESOURCE_TEXTURE_VIEW", DriverResource::TEXTURE_VIEW), crate::meta::inspect::EnumConstant::new("TEXTURE_DATA_FORMAT", "DRIVER_RESOURCE_TEXTURE_DATA_FORMAT", DriverResource::TEXTURE_DATA_FORMAT), crate::meta::inspect::EnumConstant::new("SAMPLER", "DRIVER_RESOURCE_SAMPLER", DriverResource::SAMPLER), crate::meta::inspect::EnumConstant::new("UNIFORM_SET", "DRIVER_RESOURCE_UNIFORM_SET", DriverResource::UNIFORM_SET), crate::meta::inspect::EnumConstant::new("BUFFER", "DRIVER_RESOURCE_BUFFER", DriverResource::BUFFER), crate::meta::inspect::EnumConstant::new("COMPUTE_PIPELINE", "DRIVER_RESOURCE_COMPUTE_PIPELINE", DriverResource::COMPUTE_PIPELINE), crate::meta::inspect::EnumConstant::new("RENDER_PIPELINE", "DRIVER_RESOURCE_RENDER_PIPELINE", DriverResource::RENDER_PIPELINE), crate::meta::inspect::EnumConstant::new("VULKAN_DEVICE", "DRIVER_RESOURCE_VULKAN_DEVICE", DriverResource::VULKAN_DEVICE), crate::meta::inspect::EnumConstant::new("VULKAN_PHYSICAL_DEVICE", "DRIVER_RESOURCE_VULKAN_PHYSICAL_DEVICE", DriverResource::VULKAN_PHYSICAL_DEVICE), crate::meta::inspect::EnumConstant::new("VULKAN_INSTANCE", "DRIVER_RESOURCE_VULKAN_INSTANCE", DriverResource::VULKAN_INSTANCE), crate::meta::inspect::EnumConstant::new("VULKAN_QUEUE", "DRIVER_RESOURCE_VULKAN_QUEUE", DriverResource::VULKAN_QUEUE), crate::meta::inspect::EnumConstant::new("VULKAN_QUEUE_FAMILY_INDEX", "DRIVER_RESOURCE_VULKAN_QUEUE_FAMILY_INDEX", DriverResource::VULKAN_QUEUE_FAMILY_INDEX), crate::meta::inspect::EnumConstant::new("VULKAN_IMAGE", "DRIVER_RESOURCE_VULKAN_IMAGE", DriverResource::VULKAN_IMAGE), crate::meta::inspect::EnumConstant::new("VULKAN_IMAGE_VIEW", "DRIVER_RESOURCE_VULKAN_IMAGE_VIEW", DriverResource::VULKAN_IMAGE_VIEW), crate::meta::inspect::EnumConstant::new("VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT", "DRIVER_RESOURCE_VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT", DriverResource::VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT), crate::meta::inspect::EnumConstant::new("VULKAN_SAMPLER", "DRIVER_RESOURCE_VULKAN_SAMPLER", DriverResource::VULKAN_SAMPLER), crate::meta::inspect::EnumConstant::new("VULKAN_DESCRIPTOR_SET", "DRIVER_RESOURCE_VULKAN_DESCRIPTOR_SET", DriverResource::VULKAN_DESCRIPTOR_SET), crate::meta::inspect::EnumConstant::new("VULKAN_BUFFER", "DRIVER_RESOURCE_VULKAN_BUFFER", DriverResource::VULKAN_BUFFER), crate::meta::inspect::EnumConstant::new("VULKAN_COMPUTE_PIPELINE", "DRIVER_RESOURCE_VULKAN_COMPUTE_PIPELINE", DriverResource::VULKAN_COMPUTE_PIPELINE), crate::meta::inspect::EnumConstant::new("VULKAN_RENDER_PIPELINE", "DRIVER_RESOURCE_VULKAN_RENDER_PIPELINE", DriverResource::VULKAN_RENDER_PIPELINE)]
        }
    }
}
impl crate::meta::GodotConvert for DriverResource {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DriverResource {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DriverResource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DataFormat {
    ord: i32
}
impl DataFormat {
    #[doc(alias = "DATA_FORMAT_R4G4_UNORM_PACK8")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R4G4_UNORM_PACK8`"]
    pub const R4G4_UNORM_PACK8: DataFormat = DataFormat {
        ord: 0i32
    };
    #[doc(alias = "DATA_FORMAT_R4G4B4A4_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R4G4B4A4_UNORM_PACK16`"]
    pub const R4G4B4A4_UNORM_PACK16: DataFormat = DataFormat {
        ord: 1i32
    };
    #[doc(alias = "DATA_FORMAT_B4G4R4A4_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B4G4R4A4_UNORM_PACK16`"]
    pub const B4G4R4A4_UNORM_PACK16: DataFormat = DataFormat {
        ord: 2i32
    };
    #[doc(alias = "DATA_FORMAT_R5G6B5_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R5G6B5_UNORM_PACK16`"]
    pub const R5G6B5_UNORM_PACK16: DataFormat = DataFormat {
        ord: 3i32
    };
    #[doc(alias = "DATA_FORMAT_B5G6R5_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B5G6R5_UNORM_PACK16`"]
    pub const B5G6R5_UNORM_PACK16: DataFormat = DataFormat {
        ord: 4i32
    };
    #[doc(alias = "DATA_FORMAT_R5G5B5A1_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R5G5B5A1_UNORM_PACK16`"]
    pub const R5G5B5A1_UNORM_PACK16: DataFormat = DataFormat {
        ord: 5i32
    };
    #[doc(alias = "DATA_FORMAT_B5G5R5A1_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B5G5R5A1_UNORM_PACK16`"]
    pub const B5G5R5A1_UNORM_PACK16: DataFormat = DataFormat {
        ord: 6i32
    };
    #[doc(alias = "DATA_FORMAT_A1R5G5B5_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A1R5G5B5_UNORM_PACK16`"]
    pub const A1R5G5B5_UNORM_PACK16: DataFormat = DataFormat {
        ord: 7i32
    };
    #[doc(alias = "DATA_FORMAT_R8_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8_UNORM`"]
    pub const R8_UNORM: DataFormat = DataFormat {
        ord: 8i32
    };
    #[doc(alias = "DATA_FORMAT_R8_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8_SNORM`"]
    pub const R8_SNORM: DataFormat = DataFormat {
        ord: 9i32
    };
    #[doc(alias = "DATA_FORMAT_R8_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8_USCALED`"]
    pub const R8_USCALED: DataFormat = DataFormat {
        ord: 10i32
    };
    #[doc(alias = "DATA_FORMAT_R8_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8_SSCALED`"]
    pub const R8_SSCALED: DataFormat = DataFormat {
        ord: 11i32
    };
    #[doc(alias = "DATA_FORMAT_R8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8_UINT`"]
    pub const R8_UINT: DataFormat = DataFormat {
        ord: 12i32
    };
    #[doc(alias = "DATA_FORMAT_R8_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8_SINT`"]
    pub const R8_SINT: DataFormat = DataFormat {
        ord: 13i32
    };
    #[doc(alias = "DATA_FORMAT_R8_SRGB")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8_SRGB`"]
    pub const R8_SRGB: DataFormat = DataFormat {
        ord: 14i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8_UNORM`"]
    pub const R8G8_UNORM: DataFormat = DataFormat {
        ord: 15i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8_SNORM`"]
    pub const R8G8_SNORM: DataFormat = DataFormat {
        ord: 16i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8_USCALED`"]
    pub const R8G8_USCALED: DataFormat = DataFormat {
        ord: 17i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8_SSCALED`"]
    pub const R8G8_SSCALED: DataFormat = DataFormat {
        ord: 18i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8_UINT`"]
    pub const R8G8_UINT: DataFormat = DataFormat {
        ord: 19i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8_SINT`"]
    pub const R8G8_SINT: DataFormat = DataFormat {
        ord: 20i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8_SRGB")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8_SRGB`"]
    pub const R8G8_SRGB: DataFormat = DataFormat {
        ord: 21i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8_UNORM`"]
    pub const R8G8B8_UNORM: DataFormat = DataFormat {
        ord: 22i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8_SNORM`"]
    pub const R8G8B8_SNORM: DataFormat = DataFormat {
        ord: 23i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8_USCALED`"]
    pub const R8G8B8_USCALED: DataFormat = DataFormat {
        ord: 24i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8_SSCALED`"]
    pub const R8G8B8_SSCALED: DataFormat = DataFormat {
        ord: 25i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8_UINT`"]
    pub const R8G8B8_UINT: DataFormat = DataFormat {
        ord: 26i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8_SINT`"]
    pub const R8G8B8_SINT: DataFormat = DataFormat {
        ord: 27i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8_SRGB")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8_SRGB`"]
    pub const R8G8B8_SRGB: DataFormat = DataFormat {
        ord: 28i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8_UNORM`"]
    pub const B8G8R8_UNORM: DataFormat = DataFormat {
        ord: 29i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8_SNORM`"]
    pub const B8G8R8_SNORM: DataFormat = DataFormat {
        ord: 30i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8_USCALED`"]
    pub const B8G8R8_USCALED: DataFormat = DataFormat {
        ord: 31i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8_SSCALED`"]
    pub const B8G8R8_SSCALED: DataFormat = DataFormat {
        ord: 32i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8_UINT`"]
    pub const B8G8R8_UINT: DataFormat = DataFormat {
        ord: 33i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8_SINT`"]
    pub const B8G8R8_SINT: DataFormat = DataFormat {
        ord: 34i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8_SRGB")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8_SRGB`"]
    pub const B8G8R8_SRGB: DataFormat = DataFormat {
        ord: 35i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8A8_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8A8_UNORM`"]
    pub const R8G8B8A8_UNORM: DataFormat = DataFormat {
        ord: 36i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8A8_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8A8_SNORM`"]
    pub const R8G8B8A8_SNORM: DataFormat = DataFormat {
        ord: 37i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8A8_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8A8_USCALED`"]
    pub const R8G8B8A8_USCALED: DataFormat = DataFormat {
        ord: 38i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8A8_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8A8_SSCALED`"]
    pub const R8G8B8A8_SSCALED: DataFormat = DataFormat {
        ord: 39i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8A8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8A8_UINT`"]
    pub const R8G8B8A8_UINT: DataFormat = DataFormat {
        ord: 40i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8A8_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8A8_SINT`"]
    pub const R8G8B8A8_SINT: DataFormat = DataFormat {
        ord: 41i32
    };
    #[doc(alias = "DATA_FORMAT_R8G8B8A8_SRGB")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R8G8B8A8_SRGB`"]
    pub const R8G8B8A8_SRGB: DataFormat = DataFormat {
        ord: 42i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8A8_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8A8_UNORM`"]
    pub const B8G8R8A8_UNORM: DataFormat = DataFormat {
        ord: 43i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8A8_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8A8_SNORM`"]
    pub const B8G8R8A8_SNORM: DataFormat = DataFormat {
        ord: 44i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8A8_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8A8_USCALED`"]
    pub const B8G8R8A8_USCALED: DataFormat = DataFormat {
        ord: 45i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8A8_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8A8_SSCALED`"]
    pub const B8G8R8A8_SSCALED: DataFormat = DataFormat {
        ord: 46i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8A8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8A8_UINT`"]
    pub const B8G8R8A8_UINT: DataFormat = DataFormat {
        ord: 47i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8A8_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8A8_SINT`"]
    pub const B8G8R8A8_SINT: DataFormat = DataFormat {
        ord: 48i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8A8_SRGB")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8A8_SRGB`"]
    pub const B8G8R8A8_SRGB: DataFormat = DataFormat {
        ord: 49i32
    };
    #[doc(alias = "DATA_FORMAT_A8B8G8R8_UNORM_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A8B8G8R8_UNORM_PACK32`"]
    pub const A8B8G8R8_UNORM_PACK32: DataFormat = DataFormat {
        ord: 50i32
    };
    #[doc(alias = "DATA_FORMAT_A8B8G8R8_SNORM_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A8B8G8R8_SNORM_PACK32`"]
    pub const A8B8G8R8_SNORM_PACK32: DataFormat = DataFormat {
        ord: 51i32
    };
    #[doc(alias = "DATA_FORMAT_A8B8G8R8_USCALED_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A8B8G8R8_USCALED_PACK32`"]
    pub const A8B8G8R8_USCALED_PACK32: DataFormat = DataFormat {
        ord: 52i32
    };
    #[doc(alias = "DATA_FORMAT_A8B8G8R8_SSCALED_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A8B8G8R8_SSCALED_PACK32`"]
    pub const A8B8G8R8_SSCALED_PACK32: DataFormat = DataFormat {
        ord: 53i32
    };
    #[doc(alias = "DATA_FORMAT_A8B8G8R8_UINT_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A8B8G8R8_UINT_PACK32`"]
    pub const A8B8G8R8_UINT_PACK32: DataFormat = DataFormat {
        ord: 54i32
    };
    #[doc(alias = "DATA_FORMAT_A8B8G8R8_SINT_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A8B8G8R8_SINT_PACK32`"]
    pub const A8B8G8R8_SINT_PACK32: DataFormat = DataFormat {
        ord: 55i32
    };
    #[doc(alias = "DATA_FORMAT_A8B8G8R8_SRGB_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A8B8G8R8_SRGB_PACK32`"]
    pub const A8B8G8R8_SRGB_PACK32: DataFormat = DataFormat {
        ord: 56i32
    };
    #[doc(alias = "DATA_FORMAT_A2R10G10B10_UNORM_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2R10G10B10_UNORM_PACK32`"]
    pub const A2R10G10B10_UNORM_PACK32: DataFormat = DataFormat {
        ord: 57i32
    };
    #[doc(alias = "DATA_FORMAT_A2R10G10B10_SNORM_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2R10G10B10_SNORM_PACK32`"]
    pub const A2R10G10B10_SNORM_PACK32: DataFormat = DataFormat {
        ord: 58i32
    };
    #[doc(alias = "DATA_FORMAT_A2R10G10B10_USCALED_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2R10G10B10_USCALED_PACK32`"]
    pub const A2R10G10B10_USCALED_PACK32: DataFormat = DataFormat {
        ord: 59i32
    };
    #[doc(alias = "DATA_FORMAT_A2R10G10B10_SSCALED_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2R10G10B10_SSCALED_PACK32`"]
    pub const A2R10G10B10_SSCALED_PACK32: DataFormat = DataFormat {
        ord: 60i32
    };
    #[doc(alias = "DATA_FORMAT_A2R10G10B10_UINT_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2R10G10B10_UINT_PACK32`"]
    pub const A2R10G10B10_UINT_PACK32: DataFormat = DataFormat {
        ord: 61i32
    };
    #[doc(alias = "DATA_FORMAT_A2R10G10B10_SINT_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2R10G10B10_SINT_PACK32`"]
    pub const A2R10G10B10_SINT_PACK32: DataFormat = DataFormat {
        ord: 62i32
    };
    #[doc(alias = "DATA_FORMAT_A2B10G10R10_UNORM_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2B10G10R10_UNORM_PACK32`"]
    pub const A2B10G10R10_UNORM_PACK32: DataFormat = DataFormat {
        ord: 63i32
    };
    #[doc(alias = "DATA_FORMAT_A2B10G10R10_SNORM_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2B10G10R10_SNORM_PACK32`"]
    pub const A2B10G10R10_SNORM_PACK32: DataFormat = DataFormat {
        ord: 64i32
    };
    #[doc(alias = "DATA_FORMAT_A2B10G10R10_USCALED_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2B10G10R10_USCALED_PACK32`"]
    pub const A2B10G10R10_USCALED_PACK32: DataFormat = DataFormat {
        ord: 65i32
    };
    #[doc(alias = "DATA_FORMAT_A2B10G10R10_SSCALED_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2B10G10R10_SSCALED_PACK32`"]
    pub const A2B10G10R10_SSCALED_PACK32: DataFormat = DataFormat {
        ord: 66i32
    };
    #[doc(alias = "DATA_FORMAT_A2B10G10R10_UINT_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2B10G10R10_UINT_PACK32`"]
    pub const A2B10G10R10_UINT_PACK32: DataFormat = DataFormat {
        ord: 67i32
    };
    #[doc(alias = "DATA_FORMAT_A2B10G10R10_SINT_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_A2B10G10R10_SINT_PACK32`"]
    pub const A2B10G10R10_SINT_PACK32: DataFormat = DataFormat {
        ord: 68i32
    };
    #[doc(alias = "DATA_FORMAT_R16_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16_UNORM`"]
    pub const R16_UNORM: DataFormat = DataFormat {
        ord: 69i32
    };
    #[doc(alias = "DATA_FORMAT_R16_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16_SNORM`"]
    pub const R16_SNORM: DataFormat = DataFormat {
        ord: 70i32
    };
    #[doc(alias = "DATA_FORMAT_R16_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16_USCALED`"]
    pub const R16_USCALED: DataFormat = DataFormat {
        ord: 71i32
    };
    #[doc(alias = "DATA_FORMAT_R16_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16_SSCALED`"]
    pub const R16_SSCALED: DataFormat = DataFormat {
        ord: 72i32
    };
    #[doc(alias = "DATA_FORMAT_R16_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16_UINT`"]
    pub const R16_UINT: DataFormat = DataFormat {
        ord: 73i32
    };
    #[doc(alias = "DATA_FORMAT_R16_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16_SINT`"]
    pub const R16_SINT: DataFormat = DataFormat {
        ord: 74i32
    };
    #[doc(alias = "DATA_FORMAT_R16_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16_SFLOAT`"]
    pub const R16_SFLOAT: DataFormat = DataFormat {
        ord: 75i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16_UNORM`"]
    pub const R16G16_UNORM: DataFormat = DataFormat {
        ord: 76i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16_SNORM`"]
    pub const R16G16_SNORM: DataFormat = DataFormat {
        ord: 77i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16_USCALED`"]
    pub const R16G16_USCALED: DataFormat = DataFormat {
        ord: 78i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16_SSCALED`"]
    pub const R16G16_SSCALED: DataFormat = DataFormat {
        ord: 79i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16_UINT`"]
    pub const R16G16_UINT: DataFormat = DataFormat {
        ord: 80i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16_SINT`"]
    pub const R16G16_SINT: DataFormat = DataFormat {
        ord: 81i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16_SFLOAT`"]
    pub const R16G16_SFLOAT: DataFormat = DataFormat {
        ord: 82i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16_UNORM`"]
    pub const R16G16B16_UNORM: DataFormat = DataFormat {
        ord: 83i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16_SNORM`"]
    pub const R16G16B16_SNORM: DataFormat = DataFormat {
        ord: 84i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16_USCALED`"]
    pub const R16G16B16_USCALED: DataFormat = DataFormat {
        ord: 85i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16_SSCALED`"]
    pub const R16G16B16_SSCALED: DataFormat = DataFormat {
        ord: 86i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16_UINT`"]
    pub const R16G16B16_UINT: DataFormat = DataFormat {
        ord: 87i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16_SINT`"]
    pub const R16G16B16_SINT: DataFormat = DataFormat {
        ord: 88i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16_SFLOAT`"]
    pub const R16G16B16_SFLOAT: DataFormat = DataFormat {
        ord: 89i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16A16_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16A16_UNORM`"]
    pub const R16G16B16A16_UNORM: DataFormat = DataFormat {
        ord: 90i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16A16_SNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16A16_SNORM`"]
    pub const R16G16B16A16_SNORM: DataFormat = DataFormat {
        ord: 91i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16A16_USCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16A16_USCALED`"]
    pub const R16G16B16A16_USCALED: DataFormat = DataFormat {
        ord: 92i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16A16_SSCALED")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16A16_SSCALED`"]
    pub const R16G16B16A16_SSCALED: DataFormat = DataFormat {
        ord: 93i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16A16_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16A16_UINT`"]
    pub const R16G16B16A16_UINT: DataFormat = DataFormat {
        ord: 94i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16A16_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16A16_SINT`"]
    pub const R16G16B16A16_SINT: DataFormat = DataFormat {
        ord: 95i32
    };
    #[doc(alias = "DATA_FORMAT_R16G16B16A16_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R16G16B16A16_SFLOAT`"]
    pub const R16G16B16A16_SFLOAT: DataFormat = DataFormat {
        ord: 96i32
    };
    #[doc(alias = "DATA_FORMAT_R32_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32_UINT`"]
    pub const R32_UINT: DataFormat = DataFormat {
        ord: 97i32
    };
    #[doc(alias = "DATA_FORMAT_R32_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32_SINT`"]
    pub const R32_SINT: DataFormat = DataFormat {
        ord: 98i32
    };
    #[doc(alias = "DATA_FORMAT_R32_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32_SFLOAT`"]
    pub const R32_SFLOAT: DataFormat = DataFormat {
        ord: 99i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32_UINT`"]
    pub const R32G32_UINT: DataFormat = DataFormat {
        ord: 100i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32_SINT`"]
    pub const R32G32_SINT: DataFormat = DataFormat {
        ord: 101i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32_SFLOAT`"]
    pub const R32G32_SFLOAT: DataFormat = DataFormat {
        ord: 102i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32B32_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32B32_UINT`"]
    pub const R32G32B32_UINT: DataFormat = DataFormat {
        ord: 103i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32B32_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32B32_SINT`"]
    pub const R32G32B32_SINT: DataFormat = DataFormat {
        ord: 104i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32B32_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32B32_SFLOAT`"]
    pub const R32G32B32_SFLOAT: DataFormat = DataFormat {
        ord: 105i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32B32A32_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32B32A32_UINT`"]
    pub const R32G32B32A32_UINT: DataFormat = DataFormat {
        ord: 106i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32B32A32_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32B32A32_SINT`"]
    pub const R32G32B32A32_SINT: DataFormat = DataFormat {
        ord: 107i32
    };
    #[doc(alias = "DATA_FORMAT_R32G32B32A32_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R32G32B32A32_SFLOAT`"]
    pub const R32G32B32A32_SFLOAT: DataFormat = DataFormat {
        ord: 108i32
    };
    #[doc(alias = "DATA_FORMAT_R64_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64_UINT`"]
    pub const R64_UINT: DataFormat = DataFormat {
        ord: 109i32
    };
    #[doc(alias = "DATA_FORMAT_R64_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64_SINT`"]
    pub const R64_SINT: DataFormat = DataFormat {
        ord: 110i32
    };
    #[doc(alias = "DATA_FORMAT_R64_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64_SFLOAT`"]
    pub const R64_SFLOAT: DataFormat = DataFormat {
        ord: 111i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64_UINT`"]
    pub const R64G64_UINT: DataFormat = DataFormat {
        ord: 112i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64_SINT`"]
    pub const R64G64_SINT: DataFormat = DataFormat {
        ord: 113i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64_SFLOAT`"]
    pub const R64G64_SFLOAT: DataFormat = DataFormat {
        ord: 114i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64B64_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64B64_UINT`"]
    pub const R64G64B64_UINT: DataFormat = DataFormat {
        ord: 115i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64B64_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64B64_SINT`"]
    pub const R64G64B64_SINT: DataFormat = DataFormat {
        ord: 116i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64B64_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64B64_SFLOAT`"]
    pub const R64G64B64_SFLOAT: DataFormat = DataFormat {
        ord: 117i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64B64A64_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64B64A64_UINT`"]
    pub const R64G64B64A64_UINT: DataFormat = DataFormat {
        ord: 118i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64B64A64_SINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64B64A64_SINT`"]
    pub const R64G64B64A64_SINT: DataFormat = DataFormat {
        ord: 119i32
    };
    #[doc(alias = "DATA_FORMAT_R64G64B64A64_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R64G64B64A64_SFLOAT`"]
    pub const R64G64B64A64_SFLOAT: DataFormat = DataFormat {
        ord: 120i32
    };
    #[doc(alias = "DATA_FORMAT_B10G11R11_UFLOAT_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B10G11R11_UFLOAT_PACK32`"]
    pub const B10G11R11_UFLOAT_PACK32: DataFormat = DataFormat {
        ord: 121i32
    };
    #[doc(alias = "DATA_FORMAT_E5B9G9R9_UFLOAT_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_E5B9G9R9_UFLOAT_PACK32`"]
    pub const E5B9G9R9_UFLOAT_PACK32: DataFormat = DataFormat {
        ord: 122i32
    };
    #[doc(alias = "DATA_FORMAT_D16_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_D16_UNORM`"]
    pub const D16_UNORM: DataFormat = DataFormat {
        ord: 123i32
    };
    #[doc(alias = "DATA_FORMAT_X8_D24_UNORM_PACK32")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_X8_D24_UNORM_PACK32`"]
    pub const X8_D24_UNORM_PACK32: DataFormat = DataFormat {
        ord: 124i32
    };
    #[doc(alias = "DATA_FORMAT_D32_SFLOAT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_D32_SFLOAT`"]
    pub const D32_SFLOAT: DataFormat = DataFormat {
        ord: 125i32
    };
    #[doc(alias = "DATA_FORMAT_S8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_S8_UINT`"]
    pub const S8_UINT: DataFormat = DataFormat {
        ord: 126i32
    };
    #[doc(alias = "DATA_FORMAT_D16_UNORM_S8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_D16_UNORM_S8_UINT`"]
    pub const D16_UNORM_S8_UINT: DataFormat = DataFormat {
        ord: 127i32
    };
    #[doc(alias = "DATA_FORMAT_D24_UNORM_S8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_D24_UNORM_S8_UINT`"]
    pub const D24_UNORM_S8_UINT: DataFormat = DataFormat {
        ord: 128i32
    };
    #[doc(alias = "DATA_FORMAT_D32_SFLOAT_S8_UINT")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_D32_SFLOAT_S8_UINT`"]
    pub const D32_SFLOAT_S8_UINT: DataFormat = DataFormat {
        ord: 129i32
    };
    #[doc(alias = "DATA_FORMAT_BC1_RGB_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC1_RGB_UNORM_BLOCK`"]
    pub const BC1_RGB_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 130i32
    };
    #[doc(alias = "DATA_FORMAT_BC1_RGB_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC1_RGB_SRGB_BLOCK`"]
    pub const BC1_RGB_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 131i32
    };
    #[doc(alias = "DATA_FORMAT_BC1_RGBA_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC1_RGBA_UNORM_BLOCK`"]
    pub const BC1_RGBA_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 132i32
    };
    #[doc(alias = "DATA_FORMAT_BC1_RGBA_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC1_RGBA_SRGB_BLOCK`"]
    pub const BC1_RGBA_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 133i32
    };
    #[doc(alias = "DATA_FORMAT_BC2_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC2_UNORM_BLOCK`"]
    pub const BC2_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 134i32
    };
    #[doc(alias = "DATA_FORMAT_BC2_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC2_SRGB_BLOCK`"]
    pub const BC2_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 135i32
    };
    #[doc(alias = "DATA_FORMAT_BC3_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC3_UNORM_BLOCK`"]
    pub const BC3_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 136i32
    };
    #[doc(alias = "DATA_FORMAT_BC3_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC3_SRGB_BLOCK`"]
    pub const BC3_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 137i32
    };
    #[doc(alias = "DATA_FORMAT_BC4_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC4_UNORM_BLOCK`"]
    pub const BC4_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 138i32
    };
    #[doc(alias = "DATA_FORMAT_BC4_SNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC4_SNORM_BLOCK`"]
    pub const BC4_SNORM_BLOCK: DataFormat = DataFormat {
        ord: 139i32
    };
    #[doc(alias = "DATA_FORMAT_BC5_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC5_UNORM_BLOCK`"]
    pub const BC5_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 140i32
    };
    #[doc(alias = "DATA_FORMAT_BC5_SNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC5_SNORM_BLOCK`"]
    pub const BC5_SNORM_BLOCK: DataFormat = DataFormat {
        ord: 141i32
    };
    #[doc(alias = "DATA_FORMAT_BC6H_UFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC6H_UFLOAT_BLOCK`"]
    pub const BC6H_UFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 142i32
    };
    #[doc(alias = "DATA_FORMAT_BC6H_SFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC6H_SFLOAT_BLOCK`"]
    pub const BC6H_SFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 143i32
    };
    #[doc(alias = "DATA_FORMAT_BC7_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC7_UNORM_BLOCK`"]
    pub const BC7_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 144i32
    };
    #[doc(alias = "DATA_FORMAT_BC7_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_BC7_SRGB_BLOCK`"]
    pub const BC7_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 145i32
    };
    #[doc(alias = "DATA_FORMAT_ETC2_R8G8B8_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ETC2_R8G8B8_UNORM_BLOCK`"]
    pub const ETC2_R8G8B8_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 146i32
    };
    #[doc(alias = "DATA_FORMAT_ETC2_R8G8B8_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ETC2_R8G8B8_SRGB_BLOCK`"]
    pub const ETC2_R8G8B8_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 147i32
    };
    #[doc(alias = "DATA_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK`"]
    pub const ETC2_R8G8B8A1_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 148i32
    };
    #[doc(alias = "DATA_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK`"]
    pub const ETC2_R8G8B8A1_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 149i32
    };
    #[doc(alias = "DATA_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK`"]
    pub const ETC2_R8G8B8A8_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 150i32
    };
    #[doc(alias = "DATA_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK`"]
    pub const ETC2_R8G8B8A8_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 151i32
    };
    #[doc(alias = "DATA_FORMAT_EAC_R11_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_EAC_R11_UNORM_BLOCK`"]
    pub const EAC_R11_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 152i32
    };
    #[doc(alias = "DATA_FORMAT_EAC_R11_SNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_EAC_R11_SNORM_BLOCK`"]
    pub const EAC_R11_SNORM_BLOCK: DataFormat = DataFormat {
        ord: 153i32
    };
    #[doc(alias = "DATA_FORMAT_EAC_R11G11_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_EAC_R11G11_UNORM_BLOCK`"]
    pub const EAC_R11G11_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 154i32
    };
    #[doc(alias = "DATA_FORMAT_EAC_R11G11_SNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_EAC_R11G11_SNORM_BLOCK`"]
    pub const EAC_R11G11_SNORM_BLOCK: DataFormat = DataFormat {
        ord: 155i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_4x4_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_4x4_UNORM_BLOCK`"]
    pub const ASTC_4x4_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 156i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_4x4_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_4x4_SRGB_BLOCK`"]
    pub const ASTC_4x4_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 157i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_5x4_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_5x4_UNORM_BLOCK`"]
    pub const ASTC_5x4_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 158i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_5x4_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_5x4_SRGB_BLOCK`"]
    pub const ASTC_5x4_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 159i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_5x5_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_5x5_UNORM_BLOCK`"]
    pub const ASTC_5x5_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 160i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_5x5_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_5x5_SRGB_BLOCK`"]
    pub const ASTC_5x5_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 161i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_6x5_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_6x5_UNORM_BLOCK`"]
    pub const ASTC_6x5_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 162i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_6x5_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_6x5_SRGB_BLOCK`"]
    pub const ASTC_6x5_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 163i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_6x6_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_6x6_UNORM_BLOCK`"]
    pub const ASTC_6x6_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 164i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_6x6_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_6x6_SRGB_BLOCK`"]
    pub const ASTC_6x6_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 165i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_8x5_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_8x5_UNORM_BLOCK`"]
    pub const ASTC_8x5_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 166i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_8x5_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_8x5_SRGB_BLOCK`"]
    pub const ASTC_8x5_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 167i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_8x6_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_8x6_UNORM_BLOCK`"]
    pub const ASTC_8x6_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 168i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_8x6_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_8x6_SRGB_BLOCK`"]
    pub const ASTC_8x6_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 169i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_8x8_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_8x8_UNORM_BLOCK`"]
    pub const ASTC_8x8_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 170i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_8x8_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_8x8_SRGB_BLOCK`"]
    pub const ASTC_8x8_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 171i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x5_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x5_UNORM_BLOCK`"]
    pub const ASTC_10x5_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 172i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x5_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x5_SRGB_BLOCK`"]
    pub const ASTC_10x5_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 173i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x6_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x6_UNORM_BLOCK`"]
    pub const ASTC_10x6_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 174i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x6_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x6_SRGB_BLOCK`"]
    pub const ASTC_10x6_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 175i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x8_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x8_UNORM_BLOCK`"]
    pub const ASTC_10x8_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 176i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x8_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x8_SRGB_BLOCK`"]
    pub const ASTC_10x8_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 177i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x10_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x10_UNORM_BLOCK`"]
    pub const ASTC_10x10_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 178i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x10_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x10_SRGB_BLOCK`"]
    pub const ASTC_10x10_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 179i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_12x10_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_12x10_UNORM_BLOCK`"]
    pub const ASTC_12x10_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 180i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_12x10_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_12x10_SRGB_BLOCK`"]
    pub const ASTC_12x10_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 181i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_12x12_UNORM_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_12x12_UNORM_BLOCK`"]
    pub const ASTC_12x12_UNORM_BLOCK: DataFormat = DataFormat {
        ord: 182i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_12x12_SRGB_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_12x12_SRGB_BLOCK`"]
    pub const ASTC_12x12_SRGB_BLOCK: DataFormat = DataFormat {
        ord: 183i32
    };
    #[doc(alias = "DATA_FORMAT_G8B8G8R8_422_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G8B8G8R8_422_UNORM`"]
    pub const G8B8G8R8_422_UNORM: DataFormat = DataFormat {
        ord: 184i32
    };
    #[doc(alias = "DATA_FORMAT_B8G8R8G8_422_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B8G8R8G8_422_UNORM`"]
    pub const B8G8R8G8_422_UNORM: DataFormat = DataFormat {
        ord: 185i32
    };
    #[doc(alias = "DATA_FORMAT_G8_B8_R8_3PLANE_420_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G8_B8_R8_3PLANE_420_UNORM`"]
    pub const G8_B8_R8_3PLANE_420_UNORM: DataFormat = DataFormat {
        ord: 186i32
    };
    #[doc(alias = "DATA_FORMAT_G8_B8R8_2PLANE_420_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G8_B8R8_2PLANE_420_UNORM`"]
    pub const G8_B8R8_2PLANE_420_UNORM: DataFormat = DataFormat {
        ord: 187i32
    };
    #[doc(alias = "DATA_FORMAT_G8_B8_R8_3PLANE_422_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G8_B8_R8_3PLANE_422_UNORM`"]
    pub const G8_B8_R8_3PLANE_422_UNORM: DataFormat = DataFormat {
        ord: 188i32
    };
    #[doc(alias = "DATA_FORMAT_G8_B8R8_2PLANE_422_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G8_B8R8_2PLANE_422_UNORM`"]
    pub const G8_B8R8_2PLANE_422_UNORM: DataFormat = DataFormat {
        ord: 189i32
    };
    #[doc(alias = "DATA_FORMAT_G8_B8_R8_3PLANE_444_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G8_B8_R8_3PLANE_444_UNORM`"]
    pub const G8_B8_R8_3PLANE_444_UNORM: DataFormat = DataFormat {
        ord: 190i32
    };
    #[doc(alias = "DATA_FORMAT_R10X6_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R10X6_UNORM_PACK16`"]
    pub const R10X6_UNORM_PACK16: DataFormat = DataFormat {
        ord: 191i32
    };
    #[doc(alias = "DATA_FORMAT_R10X6G10X6_UNORM_2PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R10X6G10X6_UNORM_2PACK16`"]
    pub const R10X6G10X6_UNORM_2PACK16: DataFormat = DataFormat {
        ord: 192i32
    };
    #[doc(alias = "DATA_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16`"]
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16: DataFormat = DataFormat {
        ord: 193i32
    };
    #[doc(alias = "DATA_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16`"]
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: DataFormat = DataFormat {
        ord: 194i32
    };
    #[doc(alias = "DATA_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16`"]
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: DataFormat = DataFormat {
        ord: 195i32
    };
    #[doc(alias = "DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16`"]
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 196i32
    };
    #[doc(alias = "DATA_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16`"]
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 197i32
    };
    #[doc(alias = "DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16`"]
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 198i32
    };
    #[doc(alias = "DATA_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16`"]
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 199i32
    };
    #[doc(alias = "DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16`"]
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 200i32
    };
    #[doc(alias = "DATA_FORMAT_R12X4_UNORM_PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R12X4_UNORM_PACK16`"]
    pub const R12X4_UNORM_PACK16: DataFormat = DataFormat {
        ord: 201i32
    };
    #[doc(alias = "DATA_FORMAT_R12X4G12X4_UNORM_2PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R12X4G12X4_UNORM_2PACK16`"]
    pub const R12X4G12X4_UNORM_2PACK16: DataFormat = DataFormat {
        ord: 202i32
    };
    #[doc(alias = "DATA_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16`"]
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16: DataFormat = DataFormat {
        ord: 203i32
    };
    #[doc(alias = "DATA_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16`"]
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: DataFormat = DataFormat {
        ord: 204i32
    };
    #[doc(alias = "DATA_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16`"]
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: DataFormat = DataFormat {
        ord: 205i32
    };
    #[doc(alias = "DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16`"]
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 206i32
    };
    #[doc(alias = "DATA_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16`"]
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 207i32
    };
    #[doc(alias = "DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16`"]
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 208i32
    };
    #[doc(alias = "DATA_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16`"]
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 209i32
    };
    #[doc(alias = "DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16`"]
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: DataFormat = DataFormat {
        ord: 210i32
    };
    #[doc(alias = "DATA_FORMAT_G16B16G16R16_422_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G16B16G16R16_422_UNORM`"]
    pub const G16B16G16R16_422_UNORM: DataFormat = DataFormat {
        ord: 211i32
    };
    #[doc(alias = "DATA_FORMAT_B16G16R16G16_422_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_B16G16R16G16_422_UNORM`"]
    pub const B16G16R16G16_422_UNORM: DataFormat = DataFormat {
        ord: 212i32
    };
    #[doc(alias = "DATA_FORMAT_G16_B16_R16_3PLANE_420_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G16_B16_R16_3PLANE_420_UNORM`"]
    pub const G16_B16_R16_3PLANE_420_UNORM: DataFormat = DataFormat {
        ord: 213i32
    };
    #[doc(alias = "DATA_FORMAT_G16_B16R16_2PLANE_420_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G16_B16R16_2PLANE_420_UNORM`"]
    pub const G16_B16R16_2PLANE_420_UNORM: DataFormat = DataFormat {
        ord: 214i32
    };
    #[doc(alias = "DATA_FORMAT_G16_B16_R16_3PLANE_422_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G16_B16_R16_3PLANE_422_UNORM`"]
    pub const G16_B16_R16_3PLANE_422_UNORM: DataFormat = DataFormat {
        ord: 215i32
    };
    #[doc(alias = "DATA_FORMAT_G16_B16R16_2PLANE_422_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G16_B16R16_2PLANE_422_UNORM`"]
    pub const G16_B16R16_2PLANE_422_UNORM: DataFormat = DataFormat {
        ord: 216i32
    };
    #[doc(alias = "DATA_FORMAT_G16_B16_R16_3PLANE_444_UNORM")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_G16_B16_R16_3PLANE_444_UNORM`"]
    pub const G16_B16_R16_3PLANE_444_UNORM: DataFormat = DataFormat {
        ord: 217i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_4x4_SFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_4x4_SFLOAT_BLOCK`"]
    pub const ASTC_4x4_SFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 218i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_5x4_SFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_5x4_SFLOAT_BLOCK`"]
    pub const ASTC_5x4_SFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 219i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_5x5_SFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_5x5_SFLOAT_BLOCK`"]
    pub const ASTC_5x5_SFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 220i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_6x5_SFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_6x5_SFLOAT_BLOCK`"]
    pub const ASTC_6x5_SFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 221i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_6x6_SFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_6x6_SFLOAT_BLOCK`"]
    pub const ASTC_6x6_SFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 222i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_8x5_SFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_8x5_SFLOAT_BLOCK`"]
    pub const ASTC_8x5_SFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 223i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_8x6_SFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_8x6_SFLOAT_BLOCK`"]
    pub const ASTC_8x6_SFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 224i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_8x8_SFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_8x8_SFLOAT_BLOCK`"]
    pub const ASTC_8x8_SFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 225i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x5_SFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x5_SFLOAT_BLOCK`"]
    pub const ASTC_10x5_SFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 226i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x6_SFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x6_SFLOAT_BLOCK`"]
    pub const ASTC_10x6_SFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 227i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x8_SFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x8_SFLOAT_BLOCK`"]
    pub const ASTC_10x8_SFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 228i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_10x10_SFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_10x10_SFLOAT_BLOCK`"]
    pub const ASTC_10x10_SFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 229i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_12x10_SFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_12x10_SFLOAT_BLOCK`"]
    pub const ASTC_12x10_SFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 230i32
    };
    #[doc(alias = "DATA_FORMAT_ASTC_12x12_SFLOAT_BLOCK")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_ASTC_12x12_SFLOAT_BLOCK`"]
    pub const ASTC_12x12_SFLOAT_BLOCK: DataFormat = DataFormat {
        ord: 231i32
    };
    #[doc(alias = "DATA_FORMAT_MAX")]
    #[doc = "Godot enumerator name: `DATA_FORMAT_MAX`"]
    pub const MAX: DataFormat = DataFormat {
        ord: 232i32
    };
    
}
impl std::fmt::Debug for DataFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DataFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DataFormat {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 37i32 | ord @ 38i32 | ord @ 39i32 | ord @ 40i32 | ord @ 41i32 | ord @ 42i32 | ord @ 43i32 | ord @ 44i32 | ord @ 45i32 | ord @ 46i32 | ord @ 47i32 | ord @ 48i32 | ord @ 49i32 | ord @ 50i32 | ord @ 51i32 | ord @ 52i32 | ord @ 53i32 | ord @ 54i32 | ord @ 55i32 | ord @ 56i32 | ord @ 57i32 | ord @ 58i32 | ord @ 59i32 | ord @ 60i32 | ord @ 61i32 | ord @ 62i32 | ord @ 63i32 | ord @ 64i32 | ord @ 65i32 | ord @ 66i32 | ord @ 67i32 | ord @ 68i32 | ord @ 69i32 | ord @ 70i32 | ord @ 71i32 | ord @ 72i32 | ord @ 73i32 | ord @ 74i32 | ord @ 75i32 | ord @ 76i32 | ord @ 77i32 | ord @ 78i32 | ord @ 79i32 | ord @ 80i32 | ord @ 81i32 | ord @ 82i32 | ord @ 83i32 | ord @ 84i32 | ord @ 85i32 | ord @ 86i32 | ord @ 87i32 | ord @ 88i32 | ord @ 89i32 | ord @ 90i32 | ord @ 91i32 | ord @ 92i32 | ord @ 93i32 | ord @ 94i32 | ord @ 95i32 | ord @ 96i32 | ord @ 97i32 | ord @ 98i32 | ord @ 99i32 | ord @ 100i32 | ord @ 101i32 | ord @ 102i32 | ord @ 103i32 | ord @ 104i32 | ord @ 105i32 | ord @ 106i32 | ord @ 107i32 | ord @ 108i32 | ord @ 109i32 | ord @ 110i32 | ord @ 111i32 | ord @ 112i32 | ord @ 113i32 | ord @ 114i32 | ord @ 115i32 | ord @ 116i32 | ord @ 117i32 | ord @ 118i32 | ord @ 119i32 | ord @ 120i32 | ord @ 121i32 | ord @ 122i32 | ord @ 123i32 | ord @ 124i32 | ord @ 125i32 | ord @ 126i32 | ord @ 127i32 | ord @ 128i32 | ord @ 129i32 | ord @ 130i32 | ord @ 131i32 | ord @ 132i32 | ord @ 133i32 | ord @ 134i32 | ord @ 135i32 | ord @ 136i32 | ord @ 137i32 | ord @ 138i32 | ord @ 139i32 | ord @ 140i32 | ord @ 141i32 | ord @ 142i32 | ord @ 143i32 | ord @ 144i32 | ord @ 145i32 | ord @ 146i32 | ord @ 147i32 | ord @ 148i32 | ord @ 149i32 | ord @ 150i32 | ord @ 151i32 | ord @ 152i32 | ord @ 153i32 | ord @ 154i32 | ord @ 155i32 | ord @ 156i32 | ord @ 157i32 | ord @ 158i32 | ord @ 159i32 | ord @ 160i32 | ord @ 161i32 | ord @ 162i32 | ord @ 163i32 | ord @ 164i32 | ord @ 165i32 | ord @ 166i32 | ord @ 167i32 | ord @ 168i32 | ord @ 169i32 | ord @ 170i32 | ord @ 171i32 | ord @ 172i32 | ord @ 173i32 | ord @ 174i32 | ord @ 175i32 | ord @ 176i32 | ord @ 177i32 | ord @ 178i32 | ord @ 179i32 | ord @ 180i32 | ord @ 181i32 | ord @ 182i32 | ord @ 183i32 | ord @ 184i32 | ord @ 185i32 | ord @ 186i32 | ord @ 187i32 | ord @ 188i32 | ord @ 189i32 | ord @ 190i32 | ord @ 191i32 | ord @ 192i32 | ord @ 193i32 | ord @ 194i32 | ord @ 195i32 | ord @ 196i32 | ord @ 197i32 | ord @ 198i32 | ord @ 199i32 | ord @ 200i32 | ord @ 201i32 | ord @ 202i32 | ord @ 203i32 | ord @ 204i32 | ord @ 205i32 | ord @ 206i32 | ord @ 207i32 | ord @ 208i32 | ord @ 209i32 | ord @ 210i32 | ord @ 211i32 | ord @ 212i32 | ord @ 213i32 | ord @ 214i32 | ord @ 215i32 | ord @ 216i32 | ord @ 217i32 | ord @ 218i32 | ord @ 219i32 | ord @ 220i32 | ord @ 221i32 | ord @ 222i32 | ord @ 223i32 | ord @ 224i32 | ord @ 225i32 | ord @ 226i32 | ord @ 227i32 | ord @ 228i32 | ord @ 229i32 | ord @ 230i32 | ord @ 231i32 | ord @ 232i32 => Some(Self {
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
            Self::R4G4_UNORM_PACK8 => "R4G4_UNORM_PACK8", Self::R4G4B4A4_UNORM_PACK16 => "R4G4B4A4_UNORM_PACK16", Self::B4G4R4A4_UNORM_PACK16 => "B4G4R4A4_UNORM_PACK16", Self::R5G6B5_UNORM_PACK16 => "R5G6B5_UNORM_PACK16", Self::B5G6R5_UNORM_PACK16 => "B5G6R5_UNORM_PACK16", Self::R5G5B5A1_UNORM_PACK16 => "R5G5B5A1_UNORM_PACK16", Self::B5G5R5A1_UNORM_PACK16 => "B5G5R5A1_UNORM_PACK16", Self::A1R5G5B5_UNORM_PACK16 => "A1R5G5B5_UNORM_PACK16", Self::R8_UNORM => "R8_UNORM", Self::R8_SNORM => "R8_SNORM", Self::R8_USCALED => "R8_USCALED", Self::R8_SSCALED => "R8_SSCALED", Self::R8_UINT => "R8_UINT", Self::R8_SINT => "R8_SINT", Self::R8_SRGB => "R8_SRGB", Self::R8G8_UNORM => "R8G8_UNORM", Self::R8G8_SNORM => "R8G8_SNORM", Self::R8G8_USCALED => "R8G8_USCALED", Self::R8G8_SSCALED => "R8G8_SSCALED", Self::R8G8_UINT => "R8G8_UINT", Self::R8G8_SINT => "R8G8_SINT", Self::R8G8_SRGB => "R8G8_SRGB", Self::R8G8B8_UNORM => "R8G8B8_UNORM", Self::R8G8B8_SNORM => "R8G8B8_SNORM", Self::R8G8B8_USCALED => "R8G8B8_USCALED", Self::R8G8B8_SSCALED => "R8G8B8_SSCALED", Self::R8G8B8_UINT => "R8G8B8_UINT", Self::R8G8B8_SINT => "R8G8B8_SINT", Self::R8G8B8_SRGB => "R8G8B8_SRGB", Self::B8G8R8_UNORM => "B8G8R8_UNORM", Self::B8G8R8_SNORM => "B8G8R8_SNORM", Self::B8G8R8_USCALED => "B8G8R8_USCALED", Self::B8G8R8_SSCALED => "B8G8R8_SSCALED", Self::B8G8R8_UINT => "B8G8R8_UINT", Self::B8G8R8_SINT => "B8G8R8_SINT", Self::B8G8R8_SRGB => "B8G8R8_SRGB", Self::R8G8B8A8_UNORM => "R8G8B8A8_UNORM", Self::R8G8B8A8_SNORM => "R8G8B8A8_SNORM", Self::R8G8B8A8_USCALED => "R8G8B8A8_USCALED", Self::R8G8B8A8_SSCALED => "R8G8B8A8_SSCALED", Self::R8G8B8A8_UINT => "R8G8B8A8_UINT", Self::R8G8B8A8_SINT => "R8G8B8A8_SINT", Self::R8G8B8A8_SRGB => "R8G8B8A8_SRGB", Self::B8G8R8A8_UNORM => "B8G8R8A8_UNORM", Self::B8G8R8A8_SNORM => "B8G8R8A8_SNORM", Self::B8G8R8A8_USCALED => "B8G8R8A8_USCALED", Self::B8G8R8A8_SSCALED => "B8G8R8A8_SSCALED", Self::B8G8R8A8_UINT => "B8G8R8A8_UINT", Self::B8G8R8A8_SINT => "B8G8R8A8_SINT", Self::B8G8R8A8_SRGB => "B8G8R8A8_SRGB", Self::A8B8G8R8_UNORM_PACK32 => "A8B8G8R8_UNORM_PACK32", Self::A8B8G8R8_SNORM_PACK32 => "A8B8G8R8_SNORM_PACK32", Self::A8B8G8R8_USCALED_PACK32 => "A8B8G8R8_USCALED_PACK32", Self::A8B8G8R8_SSCALED_PACK32 => "A8B8G8R8_SSCALED_PACK32", Self::A8B8G8R8_UINT_PACK32 => "A8B8G8R8_UINT_PACK32", Self::A8B8G8R8_SINT_PACK32 => "A8B8G8R8_SINT_PACK32", Self::A8B8G8R8_SRGB_PACK32 => "A8B8G8R8_SRGB_PACK32", Self::A2R10G10B10_UNORM_PACK32 => "A2R10G10B10_UNORM_PACK32", Self::A2R10G10B10_SNORM_PACK32 => "A2R10G10B10_SNORM_PACK32", Self::A2R10G10B10_USCALED_PACK32 => "A2R10G10B10_USCALED_PACK32", Self::A2R10G10B10_SSCALED_PACK32 => "A2R10G10B10_SSCALED_PACK32", Self::A2R10G10B10_UINT_PACK32 => "A2R10G10B10_UINT_PACK32", Self::A2R10G10B10_SINT_PACK32 => "A2R10G10B10_SINT_PACK32", Self::A2B10G10R10_UNORM_PACK32 => "A2B10G10R10_UNORM_PACK32", Self::A2B10G10R10_SNORM_PACK32 => "A2B10G10R10_SNORM_PACK32", Self::A2B10G10R10_USCALED_PACK32 => "A2B10G10R10_USCALED_PACK32", Self::A2B10G10R10_SSCALED_PACK32 => "A2B10G10R10_SSCALED_PACK32", Self::A2B10G10R10_UINT_PACK32 => "A2B10G10R10_UINT_PACK32", Self::A2B10G10R10_SINT_PACK32 => "A2B10G10R10_SINT_PACK32", Self::R16_UNORM => "R16_UNORM", Self::R16_SNORM => "R16_SNORM", Self::R16_USCALED => "R16_USCALED", Self::R16_SSCALED => "R16_SSCALED", Self::R16_UINT => "R16_UINT", Self::R16_SINT => "R16_SINT", Self::R16_SFLOAT => "R16_SFLOAT", Self::R16G16_UNORM => "R16G16_UNORM", Self::R16G16_SNORM => "R16G16_SNORM", Self::R16G16_USCALED => "R16G16_USCALED", Self::R16G16_SSCALED => "R16G16_SSCALED", Self::R16G16_UINT => "R16G16_UINT", Self::R16G16_SINT => "R16G16_SINT", Self::R16G16_SFLOAT => "R16G16_SFLOAT", Self::R16G16B16_UNORM => "R16G16B16_UNORM", Self::R16G16B16_SNORM => "R16G16B16_SNORM", Self::R16G16B16_USCALED => "R16G16B16_USCALED", Self::R16G16B16_SSCALED => "R16G16B16_SSCALED", Self::R16G16B16_UINT => "R16G16B16_UINT", Self::R16G16B16_SINT => "R16G16B16_SINT", Self::R16G16B16_SFLOAT => "R16G16B16_SFLOAT", Self::R16G16B16A16_UNORM => "R16G16B16A16_UNORM", Self::R16G16B16A16_SNORM => "R16G16B16A16_SNORM", Self::R16G16B16A16_USCALED => "R16G16B16A16_USCALED", Self::R16G16B16A16_SSCALED => "R16G16B16A16_SSCALED", Self::R16G16B16A16_UINT => "R16G16B16A16_UINT", Self::R16G16B16A16_SINT => "R16G16B16A16_SINT", Self::R16G16B16A16_SFLOAT => "R16G16B16A16_SFLOAT", Self::R32_UINT => "R32_UINT", Self::R32_SINT => "R32_SINT", Self::R32_SFLOAT => "R32_SFLOAT", Self::R32G32_UINT => "R32G32_UINT", Self::R32G32_SINT => "R32G32_SINT", Self::R32G32_SFLOAT => "R32G32_SFLOAT", Self::R32G32B32_UINT => "R32G32B32_UINT", Self::R32G32B32_SINT => "R32G32B32_SINT", Self::R32G32B32_SFLOAT => "R32G32B32_SFLOAT", Self::R32G32B32A32_UINT => "R32G32B32A32_UINT", Self::R32G32B32A32_SINT => "R32G32B32A32_SINT", Self::R32G32B32A32_SFLOAT => "R32G32B32A32_SFLOAT", Self::R64_UINT => "R64_UINT", Self::R64_SINT => "R64_SINT", Self::R64_SFLOAT => "R64_SFLOAT", Self::R64G64_UINT => "R64G64_UINT", Self::R64G64_SINT => "R64G64_SINT", Self::R64G64_SFLOAT => "R64G64_SFLOAT", Self::R64G64B64_UINT => "R64G64B64_UINT", Self::R64G64B64_SINT => "R64G64B64_SINT", Self::R64G64B64_SFLOAT => "R64G64B64_SFLOAT", Self::R64G64B64A64_UINT => "R64G64B64A64_UINT", Self::R64G64B64A64_SINT => "R64G64B64A64_SINT", Self::R64G64B64A64_SFLOAT => "R64G64B64A64_SFLOAT", Self::B10G11R11_UFLOAT_PACK32 => "B10G11R11_UFLOAT_PACK32", Self::E5B9G9R9_UFLOAT_PACK32 => "E5B9G9R9_UFLOAT_PACK32", Self::D16_UNORM => "D16_UNORM", Self::X8_D24_UNORM_PACK32 => "X8_D24_UNORM_PACK32", Self::D32_SFLOAT => "D32_SFLOAT", Self::S8_UINT => "S8_UINT", Self::D16_UNORM_S8_UINT => "D16_UNORM_S8_UINT", Self::D24_UNORM_S8_UINT => "D24_UNORM_S8_UINT", Self::D32_SFLOAT_S8_UINT => "D32_SFLOAT_S8_UINT", Self::BC1_RGB_UNORM_BLOCK => "BC1_RGB_UNORM_BLOCK", Self::BC1_RGB_SRGB_BLOCK => "BC1_RGB_SRGB_BLOCK", Self::BC1_RGBA_UNORM_BLOCK => "BC1_RGBA_UNORM_BLOCK", Self::BC1_RGBA_SRGB_BLOCK => "BC1_RGBA_SRGB_BLOCK", Self::BC2_UNORM_BLOCK => "BC2_UNORM_BLOCK", Self::BC2_SRGB_BLOCK => "BC2_SRGB_BLOCK", Self::BC3_UNORM_BLOCK => "BC3_UNORM_BLOCK", Self::BC3_SRGB_BLOCK => "BC3_SRGB_BLOCK", Self::BC4_UNORM_BLOCK => "BC4_UNORM_BLOCK", Self::BC4_SNORM_BLOCK => "BC4_SNORM_BLOCK", Self::BC5_UNORM_BLOCK => "BC5_UNORM_BLOCK", Self::BC5_SNORM_BLOCK => "BC5_SNORM_BLOCK", Self::BC6H_UFLOAT_BLOCK => "BC6H_UFLOAT_BLOCK", Self::BC6H_SFLOAT_BLOCK => "BC6H_SFLOAT_BLOCK", Self::BC7_UNORM_BLOCK => "BC7_UNORM_BLOCK", Self::BC7_SRGB_BLOCK => "BC7_SRGB_BLOCK", Self::ETC2_R8G8B8_UNORM_BLOCK => "ETC2_R8G8B8_UNORM_BLOCK", Self::ETC2_R8G8B8_SRGB_BLOCK => "ETC2_R8G8B8_SRGB_BLOCK", Self::ETC2_R8G8B8A1_UNORM_BLOCK => "ETC2_R8G8B8A1_UNORM_BLOCK", Self::ETC2_R8G8B8A1_SRGB_BLOCK => "ETC2_R8G8B8A1_SRGB_BLOCK", Self::ETC2_R8G8B8A8_UNORM_BLOCK => "ETC2_R8G8B8A8_UNORM_BLOCK", Self::ETC2_R8G8B8A8_SRGB_BLOCK => "ETC2_R8G8B8A8_SRGB_BLOCK", Self::EAC_R11_UNORM_BLOCK => "EAC_R11_UNORM_BLOCK", Self::EAC_R11_SNORM_BLOCK => "EAC_R11_SNORM_BLOCK", Self::EAC_R11G11_UNORM_BLOCK => "EAC_R11G11_UNORM_BLOCK", Self::EAC_R11G11_SNORM_BLOCK => "EAC_R11G11_SNORM_BLOCK", Self::ASTC_4x4_UNORM_BLOCK => "ASTC_4x4_UNORM_BLOCK", Self::ASTC_4x4_SRGB_BLOCK => "ASTC_4x4_SRGB_BLOCK", Self::ASTC_5x4_UNORM_BLOCK => "ASTC_5x4_UNORM_BLOCK", Self::ASTC_5x4_SRGB_BLOCK => "ASTC_5x4_SRGB_BLOCK", Self::ASTC_5x5_UNORM_BLOCK => "ASTC_5x5_UNORM_BLOCK", Self::ASTC_5x5_SRGB_BLOCK => "ASTC_5x5_SRGB_BLOCK", Self::ASTC_6x5_UNORM_BLOCK => "ASTC_6x5_UNORM_BLOCK", Self::ASTC_6x5_SRGB_BLOCK => "ASTC_6x5_SRGB_BLOCK", Self::ASTC_6x6_UNORM_BLOCK => "ASTC_6x6_UNORM_BLOCK", Self::ASTC_6x6_SRGB_BLOCK => "ASTC_6x6_SRGB_BLOCK", Self::ASTC_8x5_UNORM_BLOCK => "ASTC_8x5_UNORM_BLOCK", Self::ASTC_8x5_SRGB_BLOCK => "ASTC_8x5_SRGB_BLOCK", Self::ASTC_8x6_UNORM_BLOCK => "ASTC_8x6_UNORM_BLOCK", Self::ASTC_8x6_SRGB_BLOCK => "ASTC_8x6_SRGB_BLOCK", Self::ASTC_8x8_UNORM_BLOCK => "ASTC_8x8_UNORM_BLOCK", Self::ASTC_8x8_SRGB_BLOCK => "ASTC_8x8_SRGB_BLOCK", Self::ASTC_10x5_UNORM_BLOCK => "ASTC_10x5_UNORM_BLOCK", Self::ASTC_10x5_SRGB_BLOCK => "ASTC_10x5_SRGB_BLOCK", Self::ASTC_10x6_UNORM_BLOCK => "ASTC_10x6_UNORM_BLOCK", Self::ASTC_10x6_SRGB_BLOCK => "ASTC_10x6_SRGB_BLOCK", Self::ASTC_10x8_UNORM_BLOCK => "ASTC_10x8_UNORM_BLOCK", Self::ASTC_10x8_SRGB_BLOCK => "ASTC_10x8_SRGB_BLOCK", Self::ASTC_10x10_UNORM_BLOCK => "ASTC_10x10_UNORM_BLOCK", Self::ASTC_10x10_SRGB_BLOCK => "ASTC_10x10_SRGB_BLOCK", Self::ASTC_12x10_UNORM_BLOCK => "ASTC_12x10_UNORM_BLOCK", Self::ASTC_12x10_SRGB_BLOCK => "ASTC_12x10_SRGB_BLOCK", Self::ASTC_12x12_UNORM_BLOCK => "ASTC_12x12_UNORM_BLOCK", Self::ASTC_12x12_SRGB_BLOCK => "ASTC_12x12_SRGB_BLOCK", Self::G8B8G8R8_422_UNORM => "G8B8G8R8_422_UNORM", Self::B8G8R8G8_422_UNORM => "B8G8R8G8_422_UNORM", Self::G8_B8_R8_3PLANE_420_UNORM => "G8_B8_R8_3PLANE_420_UNORM", Self::G8_B8R8_2PLANE_420_UNORM => "G8_B8R8_2PLANE_420_UNORM", Self::G8_B8_R8_3PLANE_422_UNORM => "G8_B8_R8_3PLANE_422_UNORM", Self::G8_B8R8_2PLANE_422_UNORM => "G8_B8R8_2PLANE_422_UNORM", Self::G8_B8_R8_3PLANE_444_UNORM => "G8_B8_R8_3PLANE_444_UNORM", Self::R10X6_UNORM_PACK16 => "R10X6_UNORM_PACK16", Self::R10X6G10X6_UNORM_2PACK16 => "R10X6G10X6_UNORM_2PACK16", Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => "R10X6G10X6B10X6A10X6_UNORM_4PACK16", Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => "G10X6B10X6G10X6R10X6_422_UNORM_4PACK16", Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => "B10X6G10X6R10X6G10X6_422_UNORM_4PACK16", Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => "G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16", Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => "G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16", Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => "G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16", Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => "G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16", Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => "G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16", Self::R12X4_UNORM_PACK16 => "R12X4_UNORM_PACK16", Self::R12X4G12X4_UNORM_2PACK16 => "R12X4G12X4_UNORM_2PACK16", Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => "R12X4G12X4B12X4A12X4_UNORM_4PACK16", Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => "G12X4B12X4G12X4R12X4_422_UNORM_4PACK16", Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => "B12X4G12X4R12X4G12X4_422_UNORM_4PACK16", Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => "G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16", Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => "G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16", Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => "G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16", Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => "G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16", Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => "G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16", Self::G16B16G16R16_422_UNORM => "G16B16G16R16_422_UNORM", Self::B16G16R16G16_422_UNORM => "B16G16R16G16_422_UNORM", Self::G16_B16_R16_3PLANE_420_UNORM => "G16_B16_R16_3PLANE_420_UNORM", Self::G16_B16R16_2PLANE_420_UNORM => "G16_B16R16_2PLANE_420_UNORM", Self::G16_B16_R16_3PLANE_422_UNORM => "G16_B16_R16_3PLANE_422_UNORM", Self::G16_B16R16_2PLANE_422_UNORM => "G16_B16R16_2PLANE_422_UNORM", Self::G16_B16_R16_3PLANE_444_UNORM => "G16_B16_R16_3PLANE_444_UNORM", Self::ASTC_4x4_SFLOAT_BLOCK => "ASTC_4x4_SFLOAT_BLOCK", Self::ASTC_5x4_SFLOAT_BLOCK => "ASTC_5x4_SFLOAT_BLOCK", Self::ASTC_5x5_SFLOAT_BLOCK => "ASTC_5x5_SFLOAT_BLOCK", Self::ASTC_6x5_SFLOAT_BLOCK => "ASTC_6x5_SFLOAT_BLOCK", Self::ASTC_6x6_SFLOAT_BLOCK => "ASTC_6x6_SFLOAT_BLOCK", Self::ASTC_8x5_SFLOAT_BLOCK => "ASTC_8x5_SFLOAT_BLOCK", Self::ASTC_8x6_SFLOAT_BLOCK => "ASTC_8x6_SFLOAT_BLOCK", Self::ASTC_8x8_SFLOAT_BLOCK => "ASTC_8x8_SFLOAT_BLOCK", Self::ASTC_10x5_SFLOAT_BLOCK => "ASTC_10x5_SFLOAT_BLOCK", Self::ASTC_10x6_SFLOAT_BLOCK => "ASTC_10x6_SFLOAT_BLOCK", Self::ASTC_10x8_SFLOAT_BLOCK => "ASTC_10x8_SFLOAT_BLOCK", Self::ASTC_10x10_SFLOAT_BLOCK => "ASTC_10x10_SFLOAT_BLOCK", Self::ASTC_12x10_SFLOAT_BLOCK => "ASTC_12x10_SFLOAT_BLOCK", Self::ASTC_12x12_SFLOAT_BLOCK => "ASTC_12x12_SFLOAT_BLOCK", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DataFormat::R4G4_UNORM_PACK8, DataFormat::R4G4B4A4_UNORM_PACK16, DataFormat::B4G4R4A4_UNORM_PACK16, DataFormat::R5G6B5_UNORM_PACK16, DataFormat::B5G6R5_UNORM_PACK16, DataFormat::R5G5B5A1_UNORM_PACK16, DataFormat::B5G5R5A1_UNORM_PACK16, DataFormat::A1R5G5B5_UNORM_PACK16, DataFormat::R8_UNORM, DataFormat::R8_SNORM, DataFormat::R8_USCALED, DataFormat::R8_SSCALED, DataFormat::R8_UINT, DataFormat::R8_SINT, DataFormat::R8_SRGB, DataFormat::R8G8_UNORM, DataFormat::R8G8_SNORM, DataFormat::R8G8_USCALED, DataFormat::R8G8_SSCALED, DataFormat::R8G8_UINT, DataFormat::R8G8_SINT, DataFormat::R8G8_SRGB, DataFormat::R8G8B8_UNORM, DataFormat::R8G8B8_SNORM, DataFormat::R8G8B8_USCALED, DataFormat::R8G8B8_SSCALED, DataFormat::R8G8B8_UINT, DataFormat::R8G8B8_SINT, DataFormat::R8G8B8_SRGB, DataFormat::B8G8R8_UNORM, DataFormat::B8G8R8_SNORM, DataFormat::B8G8R8_USCALED, DataFormat::B8G8R8_SSCALED, DataFormat::B8G8R8_UINT, DataFormat::B8G8R8_SINT, DataFormat::B8G8R8_SRGB, DataFormat::R8G8B8A8_UNORM, DataFormat::R8G8B8A8_SNORM, DataFormat::R8G8B8A8_USCALED, DataFormat::R8G8B8A8_SSCALED, DataFormat::R8G8B8A8_UINT, DataFormat::R8G8B8A8_SINT, DataFormat::R8G8B8A8_SRGB, DataFormat::B8G8R8A8_UNORM, DataFormat::B8G8R8A8_SNORM, DataFormat::B8G8R8A8_USCALED, DataFormat::B8G8R8A8_SSCALED, DataFormat::B8G8R8A8_UINT, DataFormat::B8G8R8A8_SINT, DataFormat::B8G8R8A8_SRGB, DataFormat::A8B8G8R8_UNORM_PACK32, DataFormat::A8B8G8R8_SNORM_PACK32, DataFormat::A8B8G8R8_USCALED_PACK32, DataFormat::A8B8G8R8_SSCALED_PACK32, DataFormat::A8B8G8R8_UINT_PACK32, DataFormat::A8B8G8R8_SINT_PACK32, DataFormat::A8B8G8R8_SRGB_PACK32, DataFormat::A2R10G10B10_UNORM_PACK32, DataFormat::A2R10G10B10_SNORM_PACK32, DataFormat::A2R10G10B10_USCALED_PACK32, DataFormat::A2R10G10B10_SSCALED_PACK32, DataFormat::A2R10G10B10_UINT_PACK32, DataFormat::A2R10G10B10_SINT_PACK32, DataFormat::A2B10G10R10_UNORM_PACK32, DataFormat::A2B10G10R10_SNORM_PACK32, DataFormat::A2B10G10R10_USCALED_PACK32, DataFormat::A2B10G10R10_SSCALED_PACK32, DataFormat::A2B10G10R10_UINT_PACK32, DataFormat::A2B10G10R10_SINT_PACK32, DataFormat::R16_UNORM, DataFormat::R16_SNORM, DataFormat::R16_USCALED, DataFormat::R16_SSCALED, DataFormat::R16_UINT, DataFormat::R16_SINT, DataFormat::R16_SFLOAT, DataFormat::R16G16_UNORM, DataFormat::R16G16_SNORM, DataFormat::R16G16_USCALED, DataFormat::R16G16_SSCALED, DataFormat::R16G16_UINT, DataFormat::R16G16_SINT, DataFormat::R16G16_SFLOAT, DataFormat::R16G16B16_UNORM, DataFormat::R16G16B16_SNORM, DataFormat::R16G16B16_USCALED, DataFormat::R16G16B16_SSCALED, DataFormat::R16G16B16_UINT, DataFormat::R16G16B16_SINT, DataFormat::R16G16B16_SFLOAT, DataFormat::R16G16B16A16_UNORM, DataFormat::R16G16B16A16_SNORM, DataFormat::R16G16B16A16_USCALED, DataFormat::R16G16B16A16_SSCALED, DataFormat::R16G16B16A16_UINT, DataFormat::R16G16B16A16_SINT, DataFormat::R16G16B16A16_SFLOAT, DataFormat::R32_UINT, DataFormat::R32_SINT, DataFormat::R32_SFLOAT, DataFormat::R32G32_UINT, DataFormat::R32G32_SINT, DataFormat::R32G32_SFLOAT, DataFormat::R32G32B32_UINT, DataFormat::R32G32B32_SINT, DataFormat::R32G32B32_SFLOAT, DataFormat::R32G32B32A32_UINT, DataFormat::R32G32B32A32_SINT, DataFormat::R32G32B32A32_SFLOAT, DataFormat::R64_UINT, DataFormat::R64_SINT, DataFormat::R64_SFLOAT, DataFormat::R64G64_UINT, DataFormat::R64G64_SINT, DataFormat::R64G64_SFLOAT, DataFormat::R64G64B64_UINT, DataFormat::R64G64B64_SINT, DataFormat::R64G64B64_SFLOAT, DataFormat::R64G64B64A64_UINT, DataFormat::R64G64B64A64_SINT, DataFormat::R64G64B64A64_SFLOAT, DataFormat::B10G11R11_UFLOAT_PACK32, DataFormat::E5B9G9R9_UFLOAT_PACK32, DataFormat::D16_UNORM, DataFormat::X8_D24_UNORM_PACK32, DataFormat::D32_SFLOAT, DataFormat::S8_UINT, DataFormat::D16_UNORM_S8_UINT, DataFormat::D24_UNORM_S8_UINT, DataFormat::D32_SFLOAT_S8_UINT, DataFormat::BC1_RGB_UNORM_BLOCK, DataFormat::BC1_RGB_SRGB_BLOCK, DataFormat::BC1_RGBA_UNORM_BLOCK, DataFormat::BC1_RGBA_SRGB_BLOCK, DataFormat::BC2_UNORM_BLOCK, DataFormat::BC2_SRGB_BLOCK, DataFormat::BC3_UNORM_BLOCK, DataFormat::BC3_SRGB_BLOCK, DataFormat::BC4_UNORM_BLOCK, DataFormat::BC4_SNORM_BLOCK, DataFormat::BC5_UNORM_BLOCK, DataFormat::BC5_SNORM_BLOCK, DataFormat::BC6H_UFLOAT_BLOCK, DataFormat::BC6H_SFLOAT_BLOCK, DataFormat::BC7_UNORM_BLOCK, DataFormat::BC7_SRGB_BLOCK, DataFormat::ETC2_R8G8B8_UNORM_BLOCK, DataFormat::ETC2_R8G8B8_SRGB_BLOCK, DataFormat::ETC2_R8G8B8A1_UNORM_BLOCK, DataFormat::ETC2_R8G8B8A1_SRGB_BLOCK, DataFormat::ETC2_R8G8B8A8_UNORM_BLOCK, DataFormat::ETC2_R8G8B8A8_SRGB_BLOCK, DataFormat::EAC_R11_UNORM_BLOCK, DataFormat::EAC_R11_SNORM_BLOCK, DataFormat::EAC_R11G11_UNORM_BLOCK, DataFormat::EAC_R11G11_SNORM_BLOCK, DataFormat::ASTC_4x4_UNORM_BLOCK, DataFormat::ASTC_4x4_SRGB_BLOCK, DataFormat::ASTC_5x4_UNORM_BLOCK, DataFormat::ASTC_5x4_SRGB_BLOCK, DataFormat::ASTC_5x5_UNORM_BLOCK, DataFormat::ASTC_5x5_SRGB_BLOCK, DataFormat::ASTC_6x5_UNORM_BLOCK, DataFormat::ASTC_6x5_SRGB_BLOCK, DataFormat::ASTC_6x6_UNORM_BLOCK, DataFormat::ASTC_6x6_SRGB_BLOCK, DataFormat::ASTC_8x5_UNORM_BLOCK, DataFormat::ASTC_8x5_SRGB_BLOCK, DataFormat::ASTC_8x6_UNORM_BLOCK, DataFormat::ASTC_8x6_SRGB_BLOCK, DataFormat::ASTC_8x8_UNORM_BLOCK, DataFormat::ASTC_8x8_SRGB_BLOCK, DataFormat::ASTC_10x5_UNORM_BLOCK, DataFormat::ASTC_10x5_SRGB_BLOCK, DataFormat::ASTC_10x6_UNORM_BLOCK, DataFormat::ASTC_10x6_SRGB_BLOCK, DataFormat::ASTC_10x8_UNORM_BLOCK, DataFormat::ASTC_10x8_SRGB_BLOCK, DataFormat::ASTC_10x10_UNORM_BLOCK, DataFormat::ASTC_10x10_SRGB_BLOCK, DataFormat::ASTC_12x10_UNORM_BLOCK, DataFormat::ASTC_12x10_SRGB_BLOCK, DataFormat::ASTC_12x12_UNORM_BLOCK, DataFormat::ASTC_12x12_SRGB_BLOCK, DataFormat::G8B8G8R8_422_UNORM, DataFormat::B8G8R8G8_422_UNORM, DataFormat::G8_B8_R8_3PLANE_420_UNORM, DataFormat::G8_B8R8_2PLANE_420_UNORM, DataFormat::G8_B8_R8_3PLANE_422_UNORM, DataFormat::G8_B8R8_2PLANE_422_UNORM, DataFormat::G8_B8_R8_3PLANE_444_UNORM, DataFormat::R10X6_UNORM_PACK16, DataFormat::R10X6G10X6_UNORM_2PACK16, DataFormat::R10X6G10X6B10X6A10X6_UNORM_4PACK16, DataFormat::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16, DataFormat::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16, DataFormat::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16, DataFormat::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16, DataFormat::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16, DataFormat::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16, DataFormat::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16, DataFormat::R12X4_UNORM_PACK16, DataFormat::R12X4G12X4_UNORM_2PACK16, DataFormat::R12X4G12X4B12X4A12X4_UNORM_4PACK16, DataFormat::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16, DataFormat::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16, DataFormat::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16, DataFormat::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16, DataFormat::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16, DataFormat::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16, DataFormat::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16, DataFormat::G16B16G16R16_422_UNORM, DataFormat::B16G16R16G16_422_UNORM, DataFormat::G16_B16_R16_3PLANE_420_UNORM, DataFormat::G16_B16R16_2PLANE_420_UNORM, DataFormat::G16_B16_R16_3PLANE_422_UNORM, DataFormat::G16_B16R16_2PLANE_422_UNORM, DataFormat::G16_B16_R16_3PLANE_444_UNORM, DataFormat::ASTC_4x4_SFLOAT_BLOCK, DataFormat::ASTC_5x4_SFLOAT_BLOCK, DataFormat::ASTC_5x5_SFLOAT_BLOCK, DataFormat::ASTC_6x5_SFLOAT_BLOCK, DataFormat::ASTC_6x6_SFLOAT_BLOCK, DataFormat::ASTC_8x5_SFLOAT_BLOCK, DataFormat::ASTC_8x6_SFLOAT_BLOCK, DataFormat::ASTC_8x8_SFLOAT_BLOCK, DataFormat::ASTC_10x5_SFLOAT_BLOCK, DataFormat::ASTC_10x6_SFLOAT_BLOCK, DataFormat::ASTC_10x8_SFLOAT_BLOCK, DataFormat::ASTC_10x10_SFLOAT_BLOCK, DataFormat::ASTC_12x10_SFLOAT_BLOCK, DataFormat::ASTC_12x12_SFLOAT_BLOCK]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DataFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("R4G4_UNORM_PACK8", "DATA_FORMAT_R4G4_UNORM_PACK8", DataFormat::R4G4_UNORM_PACK8), crate::meta::inspect::EnumConstant::new("R4G4B4A4_UNORM_PACK16", "DATA_FORMAT_R4G4B4A4_UNORM_PACK16", DataFormat::R4G4B4A4_UNORM_PACK16), crate::meta::inspect::EnumConstant::new("B4G4R4A4_UNORM_PACK16", "DATA_FORMAT_B4G4R4A4_UNORM_PACK16", DataFormat::B4G4R4A4_UNORM_PACK16), crate::meta::inspect::EnumConstant::new("R5G6B5_UNORM_PACK16", "DATA_FORMAT_R5G6B5_UNORM_PACK16", DataFormat::R5G6B5_UNORM_PACK16), crate::meta::inspect::EnumConstant::new("B5G6R5_UNORM_PACK16", "DATA_FORMAT_B5G6R5_UNORM_PACK16", DataFormat::B5G6R5_UNORM_PACK16), crate::meta::inspect::EnumConstant::new("R5G5B5A1_UNORM_PACK16", "DATA_FORMAT_R5G5B5A1_UNORM_PACK16", DataFormat::R5G5B5A1_UNORM_PACK16), crate::meta::inspect::EnumConstant::new("B5G5R5A1_UNORM_PACK16", "DATA_FORMAT_B5G5R5A1_UNORM_PACK16", DataFormat::B5G5R5A1_UNORM_PACK16), crate::meta::inspect::EnumConstant::new("A1R5G5B5_UNORM_PACK16", "DATA_FORMAT_A1R5G5B5_UNORM_PACK16", DataFormat::A1R5G5B5_UNORM_PACK16), crate::meta::inspect::EnumConstant::new("R8_UNORM", "DATA_FORMAT_R8_UNORM", DataFormat::R8_UNORM), crate::meta::inspect::EnumConstant::new("R8_SNORM", "DATA_FORMAT_R8_SNORM", DataFormat::R8_SNORM), crate::meta::inspect::EnumConstant::new("R8_USCALED", "DATA_FORMAT_R8_USCALED", DataFormat::R8_USCALED), crate::meta::inspect::EnumConstant::new("R8_SSCALED", "DATA_FORMAT_R8_SSCALED", DataFormat::R8_SSCALED), crate::meta::inspect::EnumConstant::new("R8_UINT", "DATA_FORMAT_R8_UINT", DataFormat::R8_UINT), crate::meta::inspect::EnumConstant::new("R8_SINT", "DATA_FORMAT_R8_SINT", DataFormat::R8_SINT), crate::meta::inspect::EnumConstant::new("R8_SRGB", "DATA_FORMAT_R8_SRGB", DataFormat::R8_SRGB), crate::meta::inspect::EnumConstant::new("R8G8_UNORM", "DATA_FORMAT_R8G8_UNORM", DataFormat::R8G8_UNORM), crate::meta::inspect::EnumConstant::new("R8G8_SNORM", "DATA_FORMAT_R8G8_SNORM", DataFormat::R8G8_SNORM), crate::meta::inspect::EnumConstant::new("R8G8_USCALED", "DATA_FORMAT_R8G8_USCALED", DataFormat::R8G8_USCALED), crate::meta::inspect::EnumConstant::new("R8G8_SSCALED", "DATA_FORMAT_R8G8_SSCALED", DataFormat::R8G8_SSCALED), crate::meta::inspect::EnumConstant::new("R8G8_UINT", "DATA_FORMAT_R8G8_UINT", DataFormat::R8G8_UINT), crate::meta::inspect::EnumConstant::new("R8G8_SINT", "DATA_FORMAT_R8G8_SINT", DataFormat::R8G8_SINT), crate::meta::inspect::EnumConstant::new("R8G8_SRGB", "DATA_FORMAT_R8G8_SRGB", DataFormat::R8G8_SRGB), crate::meta::inspect::EnumConstant::new("R8G8B8_UNORM", "DATA_FORMAT_R8G8B8_UNORM", DataFormat::R8G8B8_UNORM), crate::meta::inspect::EnumConstant::new("R8G8B8_SNORM", "DATA_FORMAT_R8G8B8_SNORM", DataFormat::R8G8B8_SNORM), crate::meta::inspect::EnumConstant::new("R8G8B8_USCALED", "DATA_FORMAT_R8G8B8_USCALED", DataFormat::R8G8B8_USCALED), crate::meta::inspect::EnumConstant::new("R8G8B8_SSCALED", "DATA_FORMAT_R8G8B8_SSCALED", DataFormat::R8G8B8_SSCALED), crate::meta::inspect::EnumConstant::new("R8G8B8_UINT", "DATA_FORMAT_R8G8B8_UINT", DataFormat::R8G8B8_UINT), crate::meta::inspect::EnumConstant::new("R8G8B8_SINT", "DATA_FORMAT_R8G8B8_SINT", DataFormat::R8G8B8_SINT), crate::meta::inspect::EnumConstant::new("R8G8B8_SRGB", "DATA_FORMAT_R8G8B8_SRGB", DataFormat::R8G8B8_SRGB), crate::meta::inspect::EnumConstant::new("B8G8R8_UNORM", "DATA_FORMAT_B8G8R8_UNORM", DataFormat::B8G8R8_UNORM), crate::meta::inspect::EnumConstant::new("B8G8R8_SNORM", "DATA_FORMAT_B8G8R8_SNORM", DataFormat::B8G8R8_SNORM), crate::meta::inspect::EnumConstant::new("B8G8R8_USCALED", "DATA_FORMAT_B8G8R8_USCALED", DataFormat::B8G8R8_USCALED), crate::meta::inspect::EnumConstant::new("B8G8R8_SSCALED", "DATA_FORMAT_B8G8R8_SSCALED", DataFormat::B8G8R8_SSCALED), crate::meta::inspect::EnumConstant::new("B8G8R8_UINT", "DATA_FORMAT_B8G8R8_UINT", DataFormat::B8G8R8_UINT), crate::meta::inspect::EnumConstant::new("B8G8R8_SINT", "DATA_FORMAT_B8G8R8_SINT", DataFormat::B8G8R8_SINT), crate::meta::inspect::EnumConstant::new("B8G8R8_SRGB", "DATA_FORMAT_B8G8R8_SRGB", DataFormat::B8G8R8_SRGB), crate::meta::inspect::EnumConstant::new("R8G8B8A8_UNORM", "DATA_FORMAT_R8G8B8A8_UNORM", DataFormat::R8G8B8A8_UNORM), crate::meta::inspect::EnumConstant::new("R8G8B8A8_SNORM", "DATA_FORMAT_R8G8B8A8_SNORM", DataFormat::R8G8B8A8_SNORM), crate::meta::inspect::EnumConstant::new("R8G8B8A8_USCALED", "DATA_FORMAT_R8G8B8A8_USCALED", DataFormat::R8G8B8A8_USCALED), crate::meta::inspect::EnumConstant::new("R8G8B8A8_SSCALED", "DATA_FORMAT_R8G8B8A8_SSCALED", DataFormat::R8G8B8A8_SSCALED), crate::meta::inspect::EnumConstant::new("R8G8B8A8_UINT", "DATA_FORMAT_R8G8B8A8_UINT", DataFormat::R8G8B8A8_UINT), crate::meta::inspect::EnumConstant::new("R8G8B8A8_SINT", "DATA_FORMAT_R8G8B8A8_SINT", DataFormat::R8G8B8A8_SINT), crate::meta::inspect::EnumConstant::new("R8G8B8A8_SRGB", "DATA_FORMAT_R8G8B8A8_SRGB", DataFormat::R8G8B8A8_SRGB), crate::meta::inspect::EnumConstant::new("B8G8R8A8_UNORM", "DATA_FORMAT_B8G8R8A8_UNORM", DataFormat::B8G8R8A8_UNORM), crate::meta::inspect::EnumConstant::new("B8G8R8A8_SNORM", "DATA_FORMAT_B8G8R8A8_SNORM", DataFormat::B8G8R8A8_SNORM), crate::meta::inspect::EnumConstant::new("B8G8R8A8_USCALED", "DATA_FORMAT_B8G8R8A8_USCALED", DataFormat::B8G8R8A8_USCALED), crate::meta::inspect::EnumConstant::new("B8G8R8A8_SSCALED", "DATA_FORMAT_B8G8R8A8_SSCALED", DataFormat::B8G8R8A8_SSCALED), crate::meta::inspect::EnumConstant::new("B8G8R8A8_UINT", "DATA_FORMAT_B8G8R8A8_UINT", DataFormat::B8G8R8A8_UINT), crate::meta::inspect::EnumConstant::new("B8G8R8A8_SINT", "DATA_FORMAT_B8G8R8A8_SINT", DataFormat::B8G8R8A8_SINT), crate::meta::inspect::EnumConstant::new("B8G8R8A8_SRGB", "DATA_FORMAT_B8G8R8A8_SRGB", DataFormat::B8G8R8A8_SRGB), crate::meta::inspect::EnumConstant::new("A8B8G8R8_UNORM_PACK32", "DATA_FORMAT_A8B8G8R8_UNORM_PACK32", DataFormat::A8B8G8R8_UNORM_PACK32), crate::meta::inspect::EnumConstant::new("A8B8G8R8_SNORM_PACK32", "DATA_FORMAT_A8B8G8R8_SNORM_PACK32", DataFormat::A8B8G8R8_SNORM_PACK32), crate::meta::inspect::EnumConstant::new("A8B8G8R8_USCALED_PACK32", "DATA_FORMAT_A8B8G8R8_USCALED_PACK32", DataFormat::A8B8G8R8_USCALED_PACK32), crate::meta::inspect::EnumConstant::new("A8B8G8R8_SSCALED_PACK32", "DATA_FORMAT_A8B8G8R8_SSCALED_PACK32", DataFormat::A8B8G8R8_SSCALED_PACK32), crate::meta::inspect::EnumConstant::new("A8B8G8R8_UINT_PACK32", "DATA_FORMAT_A8B8G8R8_UINT_PACK32", DataFormat::A8B8G8R8_UINT_PACK32), crate::meta::inspect::EnumConstant::new("A8B8G8R8_SINT_PACK32", "DATA_FORMAT_A8B8G8R8_SINT_PACK32", DataFormat::A8B8G8R8_SINT_PACK32), crate::meta::inspect::EnumConstant::new("A8B8G8R8_SRGB_PACK32", "DATA_FORMAT_A8B8G8R8_SRGB_PACK32", DataFormat::A8B8G8R8_SRGB_PACK32), crate::meta::inspect::EnumConstant::new("A2R10G10B10_UNORM_PACK32", "DATA_FORMAT_A2R10G10B10_UNORM_PACK32", DataFormat::A2R10G10B10_UNORM_PACK32), crate::meta::inspect::EnumConstant::new("A2R10G10B10_SNORM_PACK32", "DATA_FORMAT_A2R10G10B10_SNORM_PACK32", DataFormat::A2R10G10B10_SNORM_PACK32), crate::meta::inspect::EnumConstant::new("A2R10G10B10_USCALED_PACK32", "DATA_FORMAT_A2R10G10B10_USCALED_PACK32", DataFormat::A2R10G10B10_USCALED_PACK32), crate::meta::inspect::EnumConstant::new("A2R10G10B10_SSCALED_PACK32", "DATA_FORMAT_A2R10G10B10_SSCALED_PACK32", DataFormat::A2R10G10B10_SSCALED_PACK32), crate::meta::inspect::EnumConstant::new("A2R10G10B10_UINT_PACK32", "DATA_FORMAT_A2R10G10B10_UINT_PACK32", DataFormat::A2R10G10B10_UINT_PACK32), crate::meta::inspect::EnumConstant::new("A2R10G10B10_SINT_PACK32", "DATA_FORMAT_A2R10G10B10_SINT_PACK32", DataFormat::A2R10G10B10_SINT_PACK32), crate::meta::inspect::EnumConstant::new("A2B10G10R10_UNORM_PACK32", "DATA_FORMAT_A2B10G10R10_UNORM_PACK32", DataFormat::A2B10G10R10_UNORM_PACK32), crate::meta::inspect::EnumConstant::new("A2B10G10R10_SNORM_PACK32", "DATA_FORMAT_A2B10G10R10_SNORM_PACK32", DataFormat::A2B10G10R10_SNORM_PACK32), crate::meta::inspect::EnumConstant::new("A2B10G10R10_USCALED_PACK32", "DATA_FORMAT_A2B10G10R10_USCALED_PACK32", DataFormat::A2B10G10R10_USCALED_PACK32), crate::meta::inspect::EnumConstant::new("A2B10G10R10_SSCALED_PACK32", "DATA_FORMAT_A2B10G10R10_SSCALED_PACK32", DataFormat::A2B10G10R10_SSCALED_PACK32), crate::meta::inspect::EnumConstant::new("A2B10G10R10_UINT_PACK32", "DATA_FORMAT_A2B10G10R10_UINT_PACK32", DataFormat::A2B10G10R10_UINT_PACK32), crate::meta::inspect::EnumConstant::new("A2B10G10R10_SINT_PACK32", "DATA_FORMAT_A2B10G10R10_SINT_PACK32", DataFormat::A2B10G10R10_SINT_PACK32), crate::meta::inspect::EnumConstant::new("R16_UNORM", "DATA_FORMAT_R16_UNORM", DataFormat::R16_UNORM), crate::meta::inspect::EnumConstant::new("R16_SNORM", "DATA_FORMAT_R16_SNORM", DataFormat::R16_SNORM), crate::meta::inspect::EnumConstant::new("R16_USCALED", "DATA_FORMAT_R16_USCALED", DataFormat::R16_USCALED), crate::meta::inspect::EnumConstant::new("R16_SSCALED", "DATA_FORMAT_R16_SSCALED", DataFormat::R16_SSCALED), crate::meta::inspect::EnumConstant::new("R16_UINT", "DATA_FORMAT_R16_UINT", DataFormat::R16_UINT), crate::meta::inspect::EnumConstant::new("R16_SINT", "DATA_FORMAT_R16_SINT", DataFormat::R16_SINT), crate::meta::inspect::EnumConstant::new("R16_SFLOAT", "DATA_FORMAT_R16_SFLOAT", DataFormat::R16_SFLOAT), crate::meta::inspect::EnumConstant::new("R16G16_UNORM", "DATA_FORMAT_R16G16_UNORM", DataFormat::R16G16_UNORM), crate::meta::inspect::EnumConstant::new("R16G16_SNORM", "DATA_FORMAT_R16G16_SNORM", DataFormat::R16G16_SNORM), crate::meta::inspect::EnumConstant::new("R16G16_USCALED", "DATA_FORMAT_R16G16_USCALED", DataFormat::R16G16_USCALED), crate::meta::inspect::EnumConstant::new("R16G16_SSCALED", "DATA_FORMAT_R16G16_SSCALED", DataFormat::R16G16_SSCALED), crate::meta::inspect::EnumConstant::new("R16G16_UINT", "DATA_FORMAT_R16G16_UINT", DataFormat::R16G16_UINT), crate::meta::inspect::EnumConstant::new("R16G16_SINT", "DATA_FORMAT_R16G16_SINT", DataFormat::R16G16_SINT), crate::meta::inspect::EnumConstant::new("R16G16_SFLOAT", "DATA_FORMAT_R16G16_SFLOAT", DataFormat::R16G16_SFLOAT), crate::meta::inspect::EnumConstant::new("R16G16B16_UNORM", "DATA_FORMAT_R16G16B16_UNORM", DataFormat::R16G16B16_UNORM), crate::meta::inspect::EnumConstant::new("R16G16B16_SNORM", "DATA_FORMAT_R16G16B16_SNORM", DataFormat::R16G16B16_SNORM), crate::meta::inspect::EnumConstant::new("R16G16B16_USCALED", "DATA_FORMAT_R16G16B16_USCALED", DataFormat::R16G16B16_USCALED), crate::meta::inspect::EnumConstant::new("R16G16B16_SSCALED", "DATA_FORMAT_R16G16B16_SSCALED", DataFormat::R16G16B16_SSCALED), crate::meta::inspect::EnumConstant::new("R16G16B16_UINT", "DATA_FORMAT_R16G16B16_UINT", DataFormat::R16G16B16_UINT), crate::meta::inspect::EnumConstant::new("R16G16B16_SINT", "DATA_FORMAT_R16G16B16_SINT", DataFormat::R16G16B16_SINT), crate::meta::inspect::EnumConstant::new("R16G16B16_SFLOAT", "DATA_FORMAT_R16G16B16_SFLOAT", DataFormat::R16G16B16_SFLOAT), crate::meta::inspect::EnumConstant::new("R16G16B16A16_UNORM", "DATA_FORMAT_R16G16B16A16_UNORM", DataFormat::R16G16B16A16_UNORM), crate::meta::inspect::EnumConstant::new("R16G16B16A16_SNORM", "DATA_FORMAT_R16G16B16A16_SNORM", DataFormat::R16G16B16A16_SNORM), crate::meta::inspect::EnumConstant::new("R16G16B16A16_USCALED", "DATA_FORMAT_R16G16B16A16_USCALED", DataFormat::R16G16B16A16_USCALED), crate::meta::inspect::EnumConstant::new("R16G16B16A16_SSCALED", "DATA_FORMAT_R16G16B16A16_SSCALED", DataFormat::R16G16B16A16_SSCALED), crate::meta::inspect::EnumConstant::new("R16G16B16A16_UINT", "DATA_FORMAT_R16G16B16A16_UINT", DataFormat::R16G16B16A16_UINT), crate::meta::inspect::EnumConstant::new("R16G16B16A16_SINT", "DATA_FORMAT_R16G16B16A16_SINT", DataFormat::R16G16B16A16_SINT), crate::meta::inspect::EnumConstant::new("R16G16B16A16_SFLOAT", "DATA_FORMAT_R16G16B16A16_SFLOAT", DataFormat::R16G16B16A16_SFLOAT), crate::meta::inspect::EnumConstant::new("R32_UINT", "DATA_FORMAT_R32_UINT", DataFormat::R32_UINT), crate::meta::inspect::EnumConstant::new("R32_SINT", "DATA_FORMAT_R32_SINT", DataFormat::R32_SINT), crate::meta::inspect::EnumConstant::new("R32_SFLOAT", "DATA_FORMAT_R32_SFLOAT", DataFormat::R32_SFLOAT), crate::meta::inspect::EnumConstant::new("R32G32_UINT", "DATA_FORMAT_R32G32_UINT", DataFormat::R32G32_UINT), crate::meta::inspect::EnumConstant::new("R32G32_SINT", "DATA_FORMAT_R32G32_SINT", DataFormat::R32G32_SINT), crate::meta::inspect::EnumConstant::new("R32G32_SFLOAT", "DATA_FORMAT_R32G32_SFLOAT", DataFormat::R32G32_SFLOAT), crate::meta::inspect::EnumConstant::new("R32G32B32_UINT", "DATA_FORMAT_R32G32B32_UINT", DataFormat::R32G32B32_UINT), crate::meta::inspect::EnumConstant::new("R32G32B32_SINT", "DATA_FORMAT_R32G32B32_SINT", DataFormat::R32G32B32_SINT), crate::meta::inspect::EnumConstant::new("R32G32B32_SFLOAT", "DATA_FORMAT_R32G32B32_SFLOAT", DataFormat::R32G32B32_SFLOAT), crate::meta::inspect::EnumConstant::new("R32G32B32A32_UINT", "DATA_FORMAT_R32G32B32A32_UINT", DataFormat::R32G32B32A32_UINT), crate::meta::inspect::EnumConstant::new("R32G32B32A32_SINT", "DATA_FORMAT_R32G32B32A32_SINT", DataFormat::R32G32B32A32_SINT), crate::meta::inspect::EnumConstant::new("R32G32B32A32_SFLOAT", "DATA_FORMAT_R32G32B32A32_SFLOAT", DataFormat::R32G32B32A32_SFLOAT), crate::meta::inspect::EnumConstant::new("R64_UINT", "DATA_FORMAT_R64_UINT", DataFormat::R64_UINT), crate::meta::inspect::EnumConstant::new("R64_SINT", "DATA_FORMAT_R64_SINT", DataFormat::R64_SINT), crate::meta::inspect::EnumConstant::new("R64_SFLOAT", "DATA_FORMAT_R64_SFLOAT", DataFormat::R64_SFLOAT), crate::meta::inspect::EnumConstant::new("R64G64_UINT", "DATA_FORMAT_R64G64_UINT", DataFormat::R64G64_UINT), crate::meta::inspect::EnumConstant::new("R64G64_SINT", "DATA_FORMAT_R64G64_SINT", DataFormat::R64G64_SINT), crate::meta::inspect::EnumConstant::new("R64G64_SFLOAT", "DATA_FORMAT_R64G64_SFLOAT", DataFormat::R64G64_SFLOAT), crate::meta::inspect::EnumConstant::new("R64G64B64_UINT", "DATA_FORMAT_R64G64B64_UINT", DataFormat::R64G64B64_UINT), crate::meta::inspect::EnumConstant::new("R64G64B64_SINT", "DATA_FORMAT_R64G64B64_SINT", DataFormat::R64G64B64_SINT), crate::meta::inspect::EnumConstant::new("R64G64B64_SFLOAT", "DATA_FORMAT_R64G64B64_SFLOAT", DataFormat::R64G64B64_SFLOAT), crate::meta::inspect::EnumConstant::new("R64G64B64A64_UINT", "DATA_FORMAT_R64G64B64A64_UINT", DataFormat::R64G64B64A64_UINT), crate::meta::inspect::EnumConstant::new("R64G64B64A64_SINT", "DATA_FORMAT_R64G64B64A64_SINT", DataFormat::R64G64B64A64_SINT), crate::meta::inspect::EnumConstant::new("R64G64B64A64_SFLOAT", "DATA_FORMAT_R64G64B64A64_SFLOAT", DataFormat::R64G64B64A64_SFLOAT), crate::meta::inspect::EnumConstant::new("B10G11R11_UFLOAT_PACK32", "DATA_FORMAT_B10G11R11_UFLOAT_PACK32", DataFormat::B10G11R11_UFLOAT_PACK32), crate::meta::inspect::EnumConstant::new("E5B9G9R9_UFLOAT_PACK32", "DATA_FORMAT_E5B9G9R9_UFLOAT_PACK32", DataFormat::E5B9G9R9_UFLOAT_PACK32), crate::meta::inspect::EnumConstant::new("D16_UNORM", "DATA_FORMAT_D16_UNORM", DataFormat::D16_UNORM), crate::meta::inspect::EnumConstant::new("X8_D24_UNORM_PACK32", "DATA_FORMAT_X8_D24_UNORM_PACK32", DataFormat::X8_D24_UNORM_PACK32), crate::meta::inspect::EnumConstant::new("D32_SFLOAT", "DATA_FORMAT_D32_SFLOAT", DataFormat::D32_SFLOAT), crate::meta::inspect::EnumConstant::new("S8_UINT", "DATA_FORMAT_S8_UINT", DataFormat::S8_UINT), crate::meta::inspect::EnumConstant::new("D16_UNORM_S8_UINT", "DATA_FORMAT_D16_UNORM_S8_UINT", DataFormat::D16_UNORM_S8_UINT), crate::meta::inspect::EnumConstant::new("D24_UNORM_S8_UINT", "DATA_FORMAT_D24_UNORM_S8_UINT", DataFormat::D24_UNORM_S8_UINT), crate::meta::inspect::EnumConstant::new("D32_SFLOAT_S8_UINT", "DATA_FORMAT_D32_SFLOAT_S8_UINT", DataFormat::D32_SFLOAT_S8_UINT), crate::meta::inspect::EnumConstant::new("BC1_RGB_UNORM_BLOCK", "DATA_FORMAT_BC1_RGB_UNORM_BLOCK", DataFormat::BC1_RGB_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("BC1_RGB_SRGB_BLOCK", "DATA_FORMAT_BC1_RGB_SRGB_BLOCK", DataFormat::BC1_RGB_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("BC1_RGBA_UNORM_BLOCK", "DATA_FORMAT_BC1_RGBA_UNORM_BLOCK", DataFormat::BC1_RGBA_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("BC1_RGBA_SRGB_BLOCK", "DATA_FORMAT_BC1_RGBA_SRGB_BLOCK", DataFormat::BC1_RGBA_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("BC2_UNORM_BLOCK", "DATA_FORMAT_BC2_UNORM_BLOCK", DataFormat::BC2_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("BC2_SRGB_BLOCK", "DATA_FORMAT_BC2_SRGB_BLOCK", DataFormat::BC2_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("BC3_UNORM_BLOCK", "DATA_FORMAT_BC3_UNORM_BLOCK", DataFormat::BC3_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("BC3_SRGB_BLOCK", "DATA_FORMAT_BC3_SRGB_BLOCK", DataFormat::BC3_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("BC4_UNORM_BLOCK", "DATA_FORMAT_BC4_UNORM_BLOCK", DataFormat::BC4_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("BC4_SNORM_BLOCK", "DATA_FORMAT_BC4_SNORM_BLOCK", DataFormat::BC4_SNORM_BLOCK), crate::meta::inspect::EnumConstant::new("BC5_UNORM_BLOCK", "DATA_FORMAT_BC5_UNORM_BLOCK", DataFormat::BC5_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("BC5_SNORM_BLOCK", "DATA_FORMAT_BC5_SNORM_BLOCK", DataFormat::BC5_SNORM_BLOCK), crate::meta::inspect::EnumConstant::new("BC6H_UFLOAT_BLOCK", "DATA_FORMAT_BC6H_UFLOAT_BLOCK", DataFormat::BC6H_UFLOAT_BLOCK), crate::meta::inspect::EnumConstant::new("BC6H_SFLOAT_BLOCK", "DATA_FORMAT_BC6H_SFLOAT_BLOCK", DataFormat::BC6H_SFLOAT_BLOCK), crate::meta::inspect::EnumConstant::new("BC7_UNORM_BLOCK", "DATA_FORMAT_BC7_UNORM_BLOCK", DataFormat::BC7_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("BC7_SRGB_BLOCK", "DATA_FORMAT_BC7_SRGB_BLOCK", DataFormat::BC7_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("ETC2_R8G8B8_UNORM_BLOCK", "DATA_FORMAT_ETC2_R8G8B8_UNORM_BLOCK", DataFormat::ETC2_R8G8B8_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ETC2_R8G8B8_SRGB_BLOCK", "DATA_FORMAT_ETC2_R8G8B8_SRGB_BLOCK", DataFormat::ETC2_R8G8B8_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("ETC2_R8G8B8A1_UNORM_BLOCK", "DATA_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK", DataFormat::ETC2_R8G8B8A1_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ETC2_R8G8B8A1_SRGB_BLOCK", "DATA_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK", DataFormat::ETC2_R8G8B8A1_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("ETC2_R8G8B8A8_UNORM_BLOCK", "DATA_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK", DataFormat::ETC2_R8G8B8A8_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ETC2_R8G8B8A8_SRGB_BLOCK", "DATA_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK", DataFormat::ETC2_R8G8B8A8_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("EAC_R11_UNORM_BLOCK", "DATA_FORMAT_EAC_R11_UNORM_BLOCK", DataFormat::EAC_R11_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("EAC_R11_SNORM_BLOCK", "DATA_FORMAT_EAC_R11_SNORM_BLOCK", DataFormat::EAC_R11_SNORM_BLOCK), crate::meta::inspect::EnumConstant::new("EAC_R11G11_UNORM_BLOCK", "DATA_FORMAT_EAC_R11G11_UNORM_BLOCK", DataFormat::EAC_R11G11_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("EAC_R11G11_SNORM_BLOCK", "DATA_FORMAT_EAC_R11G11_SNORM_BLOCK", DataFormat::EAC_R11G11_SNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_4x4_UNORM_BLOCK", "DATA_FORMAT_ASTC_4x4_UNORM_BLOCK", DataFormat::ASTC_4x4_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_4x4_SRGB_BLOCK", "DATA_FORMAT_ASTC_4x4_SRGB_BLOCK", DataFormat::ASTC_4x4_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_5x4_UNORM_BLOCK", "DATA_FORMAT_ASTC_5x4_UNORM_BLOCK", DataFormat::ASTC_5x4_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_5x4_SRGB_BLOCK", "DATA_FORMAT_ASTC_5x4_SRGB_BLOCK", DataFormat::ASTC_5x4_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_5x5_UNORM_BLOCK", "DATA_FORMAT_ASTC_5x5_UNORM_BLOCK", DataFormat::ASTC_5x5_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_5x5_SRGB_BLOCK", "DATA_FORMAT_ASTC_5x5_SRGB_BLOCK", DataFormat::ASTC_5x5_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_6x5_UNORM_BLOCK", "DATA_FORMAT_ASTC_6x5_UNORM_BLOCK", DataFormat::ASTC_6x5_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_6x5_SRGB_BLOCK", "DATA_FORMAT_ASTC_6x5_SRGB_BLOCK", DataFormat::ASTC_6x5_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_6x6_UNORM_BLOCK", "DATA_FORMAT_ASTC_6x6_UNORM_BLOCK", DataFormat::ASTC_6x6_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_6x6_SRGB_BLOCK", "DATA_FORMAT_ASTC_6x6_SRGB_BLOCK", DataFormat::ASTC_6x6_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_8x5_UNORM_BLOCK", "DATA_FORMAT_ASTC_8x5_UNORM_BLOCK", DataFormat::ASTC_8x5_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_8x5_SRGB_BLOCK", "DATA_FORMAT_ASTC_8x5_SRGB_BLOCK", DataFormat::ASTC_8x5_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_8x6_UNORM_BLOCK", "DATA_FORMAT_ASTC_8x6_UNORM_BLOCK", DataFormat::ASTC_8x6_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_8x6_SRGB_BLOCK", "DATA_FORMAT_ASTC_8x6_SRGB_BLOCK", DataFormat::ASTC_8x6_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_8x8_UNORM_BLOCK", "DATA_FORMAT_ASTC_8x8_UNORM_BLOCK", DataFormat::ASTC_8x8_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_8x8_SRGB_BLOCK", "DATA_FORMAT_ASTC_8x8_SRGB_BLOCK", DataFormat::ASTC_8x8_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_10x5_UNORM_BLOCK", "DATA_FORMAT_ASTC_10x5_UNORM_BLOCK", DataFormat::ASTC_10x5_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_10x5_SRGB_BLOCK", "DATA_FORMAT_ASTC_10x5_SRGB_BLOCK", DataFormat::ASTC_10x5_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_10x6_UNORM_BLOCK", "DATA_FORMAT_ASTC_10x6_UNORM_BLOCK", DataFormat::ASTC_10x6_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_10x6_SRGB_BLOCK", "DATA_FORMAT_ASTC_10x6_SRGB_BLOCK", DataFormat::ASTC_10x6_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_10x8_UNORM_BLOCK", "DATA_FORMAT_ASTC_10x8_UNORM_BLOCK", DataFormat::ASTC_10x8_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_10x8_SRGB_BLOCK", "DATA_FORMAT_ASTC_10x8_SRGB_BLOCK", DataFormat::ASTC_10x8_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_10x10_UNORM_BLOCK", "DATA_FORMAT_ASTC_10x10_UNORM_BLOCK", DataFormat::ASTC_10x10_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_10x10_SRGB_BLOCK", "DATA_FORMAT_ASTC_10x10_SRGB_BLOCK", DataFormat::ASTC_10x10_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_12x10_UNORM_BLOCK", "DATA_FORMAT_ASTC_12x10_UNORM_BLOCK", DataFormat::ASTC_12x10_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_12x10_SRGB_BLOCK", "DATA_FORMAT_ASTC_12x10_SRGB_BLOCK", DataFormat::ASTC_12x10_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_12x12_UNORM_BLOCK", "DATA_FORMAT_ASTC_12x12_UNORM_BLOCK", DataFormat::ASTC_12x12_UNORM_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_12x12_SRGB_BLOCK", "DATA_FORMAT_ASTC_12x12_SRGB_BLOCK", DataFormat::ASTC_12x12_SRGB_BLOCK), crate::meta::inspect::EnumConstant::new("G8B8G8R8_422_UNORM", "DATA_FORMAT_G8B8G8R8_422_UNORM", DataFormat::G8B8G8R8_422_UNORM), crate::meta::inspect::EnumConstant::new("B8G8R8G8_422_UNORM", "DATA_FORMAT_B8G8R8G8_422_UNORM", DataFormat::B8G8R8G8_422_UNORM), crate::meta::inspect::EnumConstant::new("G8_B8_R8_3PLANE_420_UNORM", "DATA_FORMAT_G8_B8_R8_3PLANE_420_UNORM", DataFormat::G8_B8_R8_3PLANE_420_UNORM), crate::meta::inspect::EnumConstant::new("G8_B8R8_2PLANE_420_UNORM", "DATA_FORMAT_G8_B8R8_2PLANE_420_UNORM", DataFormat::G8_B8R8_2PLANE_420_UNORM), crate::meta::inspect::EnumConstant::new("G8_B8_R8_3PLANE_422_UNORM", "DATA_FORMAT_G8_B8_R8_3PLANE_422_UNORM", DataFormat::G8_B8_R8_3PLANE_422_UNORM), crate::meta::inspect::EnumConstant::new("G8_B8R8_2PLANE_422_UNORM", "DATA_FORMAT_G8_B8R8_2PLANE_422_UNORM", DataFormat::G8_B8R8_2PLANE_422_UNORM), crate::meta::inspect::EnumConstant::new("G8_B8_R8_3PLANE_444_UNORM", "DATA_FORMAT_G8_B8_R8_3PLANE_444_UNORM", DataFormat::G8_B8_R8_3PLANE_444_UNORM), crate::meta::inspect::EnumConstant::new("R10X6_UNORM_PACK16", "DATA_FORMAT_R10X6_UNORM_PACK16", DataFormat::R10X6_UNORM_PACK16), crate::meta::inspect::EnumConstant::new("R10X6G10X6_UNORM_2PACK16", "DATA_FORMAT_R10X6G10X6_UNORM_2PACK16", DataFormat::R10X6G10X6_UNORM_2PACK16), crate::meta::inspect::EnumConstant::new("R10X6G10X6B10X6A10X6_UNORM_4PACK16", "DATA_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16", DataFormat::R10X6G10X6B10X6A10X6_UNORM_4PACK16), crate::meta::inspect::EnumConstant::new("G10X6B10X6G10X6R10X6_422_UNORM_4PACK16", "DATA_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16", DataFormat::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16), crate::meta::inspect::EnumConstant::new("B10X6G10X6R10X6G10X6_422_UNORM_4PACK16", "DATA_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16", DataFormat::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16), crate::meta::inspect::EnumConstant::new("G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16", "DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16", DataFormat::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16), crate::meta::inspect::EnumConstant::new("G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16", "DATA_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16", DataFormat::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16), crate::meta::inspect::EnumConstant::new("G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16", "DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16", DataFormat::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16), crate::meta::inspect::EnumConstant::new("G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16", "DATA_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16", DataFormat::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16), crate::meta::inspect::EnumConstant::new("G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16", "DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16", DataFormat::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16), crate::meta::inspect::EnumConstant::new("R12X4_UNORM_PACK16", "DATA_FORMAT_R12X4_UNORM_PACK16", DataFormat::R12X4_UNORM_PACK16), crate::meta::inspect::EnumConstant::new("R12X4G12X4_UNORM_2PACK16", "DATA_FORMAT_R12X4G12X4_UNORM_2PACK16", DataFormat::R12X4G12X4_UNORM_2PACK16), crate::meta::inspect::EnumConstant::new("R12X4G12X4B12X4A12X4_UNORM_4PACK16", "DATA_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16", DataFormat::R12X4G12X4B12X4A12X4_UNORM_4PACK16), crate::meta::inspect::EnumConstant::new("G12X4B12X4G12X4R12X4_422_UNORM_4PACK16", "DATA_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16", DataFormat::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16), crate::meta::inspect::EnumConstant::new("B12X4G12X4R12X4G12X4_422_UNORM_4PACK16", "DATA_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16", DataFormat::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16), crate::meta::inspect::EnumConstant::new("G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16", "DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16", DataFormat::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16), crate::meta::inspect::EnumConstant::new("G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16", "DATA_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16", DataFormat::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16), crate::meta::inspect::EnumConstant::new("G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16", "DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16", DataFormat::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16), crate::meta::inspect::EnumConstant::new("G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16", "DATA_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16", DataFormat::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16), crate::meta::inspect::EnumConstant::new("G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16", "DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16", DataFormat::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16), crate::meta::inspect::EnumConstant::new("G16B16G16R16_422_UNORM", "DATA_FORMAT_G16B16G16R16_422_UNORM", DataFormat::G16B16G16R16_422_UNORM), crate::meta::inspect::EnumConstant::new("B16G16R16G16_422_UNORM", "DATA_FORMAT_B16G16R16G16_422_UNORM", DataFormat::B16G16R16G16_422_UNORM), crate::meta::inspect::EnumConstant::new("G16_B16_R16_3PLANE_420_UNORM", "DATA_FORMAT_G16_B16_R16_3PLANE_420_UNORM", DataFormat::G16_B16_R16_3PLANE_420_UNORM), crate::meta::inspect::EnumConstant::new("G16_B16R16_2PLANE_420_UNORM", "DATA_FORMAT_G16_B16R16_2PLANE_420_UNORM", DataFormat::G16_B16R16_2PLANE_420_UNORM), crate::meta::inspect::EnumConstant::new("G16_B16_R16_3PLANE_422_UNORM", "DATA_FORMAT_G16_B16_R16_3PLANE_422_UNORM", DataFormat::G16_B16_R16_3PLANE_422_UNORM), crate::meta::inspect::EnumConstant::new("G16_B16R16_2PLANE_422_UNORM", "DATA_FORMAT_G16_B16R16_2PLANE_422_UNORM", DataFormat::G16_B16R16_2PLANE_422_UNORM), crate::meta::inspect::EnumConstant::new("G16_B16_R16_3PLANE_444_UNORM", "DATA_FORMAT_G16_B16_R16_3PLANE_444_UNORM", DataFormat::G16_B16_R16_3PLANE_444_UNORM), crate::meta::inspect::EnumConstant::new("ASTC_4x4_SFLOAT_BLOCK", "DATA_FORMAT_ASTC_4x4_SFLOAT_BLOCK", DataFormat::ASTC_4x4_SFLOAT_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_5x4_SFLOAT_BLOCK", "DATA_FORMAT_ASTC_5x4_SFLOAT_BLOCK", DataFormat::ASTC_5x4_SFLOAT_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_5x5_SFLOAT_BLOCK", "DATA_FORMAT_ASTC_5x5_SFLOAT_BLOCK", DataFormat::ASTC_5x5_SFLOAT_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_6x5_SFLOAT_BLOCK", "DATA_FORMAT_ASTC_6x5_SFLOAT_BLOCK", DataFormat::ASTC_6x5_SFLOAT_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_6x6_SFLOAT_BLOCK", "DATA_FORMAT_ASTC_6x6_SFLOAT_BLOCK", DataFormat::ASTC_6x6_SFLOAT_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_8x5_SFLOAT_BLOCK", "DATA_FORMAT_ASTC_8x5_SFLOAT_BLOCK", DataFormat::ASTC_8x5_SFLOAT_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_8x6_SFLOAT_BLOCK", "DATA_FORMAT_ASTC_8x6_SFLOAT_BLOCK", DataFormat::ASTC_8x6_SFLOAT_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_8x8_SFLOAT_BLOCK", "DATA_FORMAT_ASTC_8x8_SFLOAT_BLOCK", DataFormat::ASTC_8x8_SFLOAT_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_10x5_SFLOAT_BLOCK", "DATA_FORMAT_ASTC_10x5_SFLOAT_BLOCK", DataFormat::ASTC_10x5_SFLOAT_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_10x6_SFLOAT_BLOCK", "DATA_FORMAT_ASTC_10x6_SFLOAT_BLOCK", DataFormat::ASTC_10x6_SFLOAT_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_10x8_SFLOAT_BLOCK", "DATA_FORMAT_ASTC_10x8_SFLOAT_BLOCK", DataFormat::ASTC_10x8_SFLOAT_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_10x10_SFLOAT_BLOCK", "DATA_FORMAT_ASTC_10x10_SFLOAT_BLOCK", DataFormat::ASTC_10x10_SFLOAT_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_12x10_SFLOAT_BLOCK", "DATA_FORMAT_ASTC_12x10_SFLOAT_BLOCK", DataFormat::ASTC_12x10_SFLOAT_BLOCK), crate::meta::inspect::EnumConstant::new("ASTC_12x12_SFLOAT_BLOCK", "DATA_FORMAT_ASTC_12x12_SFLOAT_BLOCK", DataFormat::ASTC_12x12_SFLOAT_BLOCK), crate::meta::inspect::EnumConstant::new("MAX", "DATA_FORMAT_MAX", DataFormat::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for DataFormat {
    const ENUMERATOR_COUNT: usize = 232usize;
    
}
impl crate::meta::GodotConvert for DataFormat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DataFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DataFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct BarrierMask {
    ord: u64
}
impl BarrierMask {
    #[doc(alias = "BARRIER_MASK_VERTEX")]
    #[doc = "Godot enumerator name: `BARRIER_MASK_VERTEX`"]
    pub const VERTEX: BarrierMask = BarrierMask {
        ord: 1u64
    };
    #[doc(alias = "BARRIER_MASK_FRAGMENT")]
    #[doc = "Godot enumerator name: `BARRIER_MASK_FRAGMENT`"]
    pub const FRAGMENT: BarrierMask = BarrierMask {
        ord: 8u64
    };
    #[doc(alias = "BARRIER_MASK_COMPUTE")]
    #[doc = "Godot enumerator name: `BARRIER_MASK_COMPUTE`"]
    pub const COMPUTE: BarrierMask = BarrierMask {
        ord: 2u64
    };
    #[doc(alias = "BARRIER_MASK_TRANSFER")]
    #[doc = "Godot enumerator name: `BARRIER_MASK_TRANSFER`"]
    pub const TRANSFER: BarrierMask = BarrierMask {
        ord: 4u64
    };
    #[doc(alias = "BARRIER_MASK_RASTER")]
    #[doc = "Godot enumerator name: `BARRIER_MASK_RASTER`"]
    pub const RASTER: BarrierMask = BarrierMask {
        ord: 9u64
    };
    #[doc(alias = "BARRIER_MASK_ALL_BARRIERS")]
    #[doc = "Godot enumerator name: `BARRIER_MASK_ALL_BARRIERS`"]
    pub const ALL_BARRIERS: BarrierMask = BarrierMask {
        ord: 32767u64
    };
    #[doc(alias = "BARRIER_MASK_NO_BARRIER")]
    #[doc = "Godot enumerator name: `BARRIER_MASK_NO_BARRIER`"]
    pub const NO_BARRIER: BarrierMask = BarrierMask {
        ord: 32768u64
    };
    
}
impl std::fmt::Debug for BarrierMask {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::VERTEX => "VERTEX", Self::FRAGMENT => "FRAGMENT", Self::COMPUTE => "COMPUTE", Self::TRANSFER => "TRANSFER", Self::RASTER => "RASTER", Self::ALL_BARRIERS => "ALL_BARRIERS", Self::NO_BARRIER => "NO_BARRIER", _ => {
                f.debug_struct("BarrierMask") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for BarrierMask {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BarrierMask >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERTEX", "BARRIER_MASK_VERTEX", BarrierMask::VERTEX), crate::meta::inspect::EnumConstant::new("FRAGMENT", "BARRIER_MASK_FRAGMENT", BarrierMask::FRAGMENT), crate::meta::inspect::EnumConstant::new("COMPUTE", "BARRIER_MASK_COMPUTE", BarrierMask::COMPUTE), crate::meta::inspect::EnumConstant::new("TRANSFER", "BARRIER_MASK_TRANSFER", BarrierMask::TRANSFER), crate::meta::inspect::EnumConstant::new("RASTER", "BARRIER_MASK_RASTER", BarrierMask::RASTER), crate::meta::inspect::EnumConstant::new("ALL_BARRIERS", "BARRIER_MASK_ALL_BARRIERS", BarrierMask::ALL_BARRIERS), crate::meta::inspect::EnumConstant::new("NO_BARRIER", "BARRIER_MASK_NO_BARRIER", BarrierMask::NO_BARRIER)]
        }
    }
}
impl std::ops::BitOr for BarrierMask {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for BarrierMask {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for BarrierMask {
    type Via = u64;
    
}
impl crate::meta::ToGodot for BarrierMask {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BarrierMask {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureType {
    ord: i32
}
impl TextureType {
    #[doc(alias = "TEXTURE_TYPE_1D")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_1D`"]
    pub const TYPE_1D: TextureType = TextureType {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_TYPE_2D")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_2D`"]
    pub const TYPE_2D: TextureType = TextureType {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_TYPE_3D")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_3D`"]
    pub const TYPE_3D: TextureType = TextureType {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_TYPE_CUBE")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_CUBE`"]
    pub const CUBE: TextureType = TextureType {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_TYPE_1D_ARRAY")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_1D_ARRAY`"]
    pub const TYPE_1D_ARRAY: TextureType = TextureType {
        ord: 4i32
    };
    #[doc(alias = "TEXTURE_TYPE_2D_ARRAY")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_2D_ARRAY`"]
    pub const TYPE_2D_ARRAY: TextureType = TextureType {
        ord: 5i32
    };
    #[doc(alias = "TEXTURE_TYPE_CUBE_ARRAY")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_CUBE_ARRAY`"]
    pub const CUBE_ARRAY: TextureType = TextureType {
        ord: 6i32
    };
    #[doc(alias = "TEXTURE_TYPE_MAX")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_MAX`"]
    pub const MAX: TextureType = TextureType {
        ord: 7i32
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
            Self::TYPE_1D => "TYPE_1D", Self::TYPE_2D => "TYPE_2D", Self::TYPE_3D => "TYPE_3D", Self::CUBE => "CUBE", Self::TYPE_1D_ARRAY => "TYPE_1D_ARRAY", Self::TYPE_2D_ARRAY => "TYPE_2D_ARRAY", Self::CUBE_ARRAY => "CUBE_ARRAY", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureType::TYPE_1D, TextureType::TYPE_2D, TextureType::TYPE_3D, TextureType::CUBE, TextureType::TYPE_1D_ARRAY, TextureType::TYPE_2D_ARRAY, TextureType::CUBE_ARRAY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TYPE_1D", "TEXTURE_TYPE_1D", TextureType::TYPE_1D), crate::meta::inspect::EnumConstant::new("TYPE_2D", "TEXTURE_TYPE_2D", TextureType::TYPE_2D), crate::meta::inspect::EnumConstant::new("TYPE_3D", "TEXTURE_TYPE_3D", TextureType::TYPE_3D), crate::meta::inspect::EnumConstant::new("CUBE", "TEXTURE_TYPE_CUBE", TextureType::CUBE), crate::meta::inspect::EnumConstant::new("TYPE_1D_ARRAY", "TEXTURE_TYPE_1D_ARRAY", TextureType::TYPE_1D_ARRAY), crate::meta::inspect::EnumConstant::new("TYPE_2D_ARRAY", "TEXTURE_TYPE_2D_ARRAY", TextureType::TYPE_2D_ARRAY), crate::meta::inspect::EnumConstant::new("CUBE_ARRAY", "TEXTURE_TYPE_CUBE_ARRAY", TextureType::CUBE_ARRAY), crate::meta::inspect::EnumConstant::new("MAX", "TEXTURE_TYPE_MAX", TextureType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for TextureType {
    const ENUMERATOR_COUNT: usize = 7usize;
    
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
pub struct TextureSamples {
    ord: i32
}
impl TextureSamples {
    #[doc(alias = "TEXTURE_SAMPLES_1")]
    #[doc = "Godot enumerator name: `TEXTURE_SAMPLES_1`"]
    pub const SAMPLES_1: TextureSamples = TextureSamples {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_SAMPLES_2")]
    #[doc = "Godot enumerator name: `TEXTURE_SAMPLES_2`"]
    pub const SAMPLES_2: TextureSamples = TextureSamples {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_SAMPLES_4")]
    #[doc = "Godot enumerator name: `TEXTURE_SAMPLES_4`"]
    pub const SAMPLES_4: TextureSamples = TextureSamples {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_SAMPLES_8")]
    #[doc = "Godot enumerator name: `TEXTURE_SAMPLES_8`"]
    pub const SAMPLES_8: TextureSamples = TextureSamples {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_SAMPLES_16")]
    #[doc = "Godot enumerator name: `TEXTURE_SAMPLES_16`"]
    pub const SAMPLES_16: TextureSamples = TextureSamples {
        ord: 4i32
    };
    #[doc(alias = "TEXTURE_SAMPLES_32")]
    #[doc = "Godot enumerator name: `TEXTURE_SAMPLES_32`"]
    pub const SAMPLES_32: TextureSamples = TextureSamples {
        ord: 5i32
    };
    #[doc(alias = "TEXTURE_SAMPLES_64")]
    #[doc = "Godot enumerator name: `TEXTURE_SAMPLES_64`"]
    pub const SAMPLES_64: TextureSamples = TextureSamples {
        ord: 6i32
    };
    #[doc(alias = "TEXTURE_SAMPLES_MAX")]
    #[doc = "Godot enumerator name: `TEXTURE_SAMPLES_MAX`"]
    pub const MAX: TextureSamples = TextureSamples {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for TextureSamples {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureSamples") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureSamples {
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
            Self::SAMPLES_1 => "SAMPLES_1", Self::SAMPLES_2 => "SAMPLES_2", Self::SAMPLES_4 => "SAMPLES_4", Self::SAMPLES_8 => "SAMPLES_8", Self::SAMPLES_16 => "SAMPLES_16", Self::SAMPLES_32 => "SAMPLES_32", Self::SAMPLES_64 => "SAMPLES_64", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureSamples::SAMPLES_1, TextureSamples::SAMPLES_2, TextureSamples::SAMPLES_4, TextureSamples::SAMPLES_8, TextureSamples::SAMPLES_16, TextureSamples::SAMPLES_32, TextureSamples::SAMPLES_64]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureSamples >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SAMPLES_1", "TEXTURE_SAMPLES_1", TextureSamples::SAMPLES_1), crate::meta::inspect::EnumConstant::new("SAMPLES_2", "TEXTURE_SAMPLES_2", TextureSamples::SAMPLES_2), crate::meta::inspect::EnumConstant::new("SAMPLES_4", "TEXTURE_SAMPLES_4", TextureSamples::SAMPLES_4), crate::meta::inspect::EnumConstant::new("SAMPLES_8", "TEXTURE_SAMPLES_8", TextureSamples::SAMPLES_8), crate::meta::inspect::EnumConstant::new("SAMPLES_16", "TEXTURE_SAMPLES_16", TextureSamples::SAMPLES_16), crate::meta::inspect::EnumConstant::new("SAMPLES_32", "TEXTURE_SAMPLES_32", TextureSamples::SAMPLES_32), crate::meta::inspect::EnumConstant::new("SAMPLES_64", "TEXTURE_SAMPLES_64", TextureSamples::SAMPLES_64), crate::meta::inspect::EnumConstant::new("MAX", "TEXTURE_SAMPLES_MAX", TextureSamples::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for TextureSamples {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for TextureSamples {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureSamples {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureSamples {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct TextureUsageBits {
    ord: u64
}
impl TextureUsageBits {
    #[doc(alias = "TEXTURE_USAGE_SAMPLING_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_SAMPLING_BIT`"]
    pub const SAMPLING_BIT: TextureUsageBits = TextureUsageBits {
        ord: 1u64
    };
    #[doc(alias = "TEXTURE_USAGE_COLOR_ATTACHMENT_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_COLOR_ATTACHMENT_BIT`"]
    pub const COLOR_ATTACHMENT_BIT: TextureUsageBits = TextureUsageBits {
        ord: 2u64
    };
    #[doc(alias = "TEXTURE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`"]
    pub const DEPTH_STENCIL_ATTACHMENT_BIT: TextureUsageBits = TextureUsageBits {
        ord: 4u64
    };
    #[doc(alias = "TEXTURE_USAGE_STORAGE_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_STORAGE_BIT`"]
    pub const STORAGE_BIT: TextureUsageBits = TextureUsageBits {
        ord: 8u64
    };
    #[doc(alias = "TEXTURE_USAGE_STORAGE_ATOMIC_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_STORAGE_ATOMIC_BIT`"]
    pub const STORAGE_ATOMIC_BIT: TextureUsageBits = TextureUsageBits {
        ord: 16u64
    };
    #[doc(alias = "TEXTURE_USAGE_CPU_READ_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_CPU_READ_BIT`"]
    pub const CPU_READ_BIT: TextureUsageBits = TextureUsageBits {
        ord: 32u64
    };
    #[doc(alias = "TEXTURE_USAGE_CAN_UPDATE_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_CAN_UPDATE_BIT`"]
    pub const CAN_UPDATE_BIT: TextureUsageBits = TextureUsageBits {
        ord: 64u64
    };
    #[doc(alias = "TEXTURE_USAGE_CAN_COPY_FROM_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_CAN_COPY_FROM_BIT`"]
    pub const CAN_COPY_FROM_BIT: TextureUsageBits = TextureUsageBits {
        ord: 128u64
    };
    #[doc(alias = "TEXTURE_USAGE_CAN_COPY_TO_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_CAN_COPY_TO_BIT`"]
    pub const CAN_COPY_TO_BIT: TextureUsageBits = TextureUsageBits {
        ord: 256u64
    };
    #[doc(alias = "TEXTURE_USAGE_INPUT_ATTACHMENT_BIT")]
    #[doc = "Godot enumerator name: `TEXTURE_USAGE_INPUT_ATTACHMENT_BIT`"]
    pub const INPUT_ATTACHMENT_BIT: TextureUsageBits = TextureUsageBits {
        ord: 512u64
    };
    
}
impl std::fmt::Debug for TextureUsageBits {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::SAMPLING_BIT => "SAMPLING_BIT", Self::COLOR_ATTACHMENT_BIT => "COLOR_ATTACHMENT_BIT", Self::DEPTH_STENCIL_ATTACHMENT_BIT => "DEPTH_STENCIL_ATTACHMENT_BIT", Self::STORAGE_BIT => "STORAGE_BIT", Self::STORAGE_ATOMIC_BIT => "STORAGE_ATOMIC_BIT", Self::CPU_READ_BIT => "CPU_READ_BIT", Self::CAN_UPDATE_BIT => "CAN_UPDATE_BIT", Self::CAN_COPY_FROM_BIT => "CAN_COPY_FROM_BIT", Self::CAN_COPY_TO_BIT => "CAN_COPY_TO_BIT", Self::INPUT_ATTACHMENT_BIT => "INPUT_ATTACHMENT_BIT", _ => {
                f.debug_struct("TextureUsageBits") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for TextureUsageBits {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureUsageBits >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SAMPLING_BIT", "TEXTURE_USAGE_SAMPLING_BIT", TextureUsageBits::SAMPLING_BIT), crate::meta::inspect::EnumConstant::new("COLOR_ATTACHMENT_BIT", "TEXTURE_USAGE_COLOR_ATTACHMENT_BIT", TextureUsageBits::COLOR_ATTACHMENT_BIT), crate::meta::inspect::EnumConstant::new("DEPTH_STENCIL_ATTACHMENT_BIT", "TEXTURE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT", TextureUsageBits::DEPTH_STENCIL_ATTACHMENT_BIT), crate::meta::inspect::EnumConstant::new("STORAGE_BIT", "TEXTURE_USAGE_STORAGE_BIT", TextureUsageBits::STORAGE_BIT), crate::meta::inspect::EnumConstant::new("STORAGE_ATOMIC_BIT", "TEXTURE_USAGE_STORAGE_ATOMIC_BIT", TextureUsageBits::STORAGE_ATOMIC_BIT), crate::meta::inspect::EnumConstant::new("CPU_READ_BIT", "TEXTURE_USAGE_CPU_READ_BIT", TextureUsageBits::CPU_READ_BIT), crate::meta::inspect::EnumConstant::new("CAN_UPDATE_BIT", "TEXTURE_USAGE_CAN_UPDATE_BIT", TextureUsageBits::CAN_UPDATE_BIT), crate::meta::inspect::EnumConstant::new("CAN_COPY_FROM_BIT", "TEXTURE_USAGE_CAN_COPY_FROM_BIT", TextureUsageBits::CAN_COPY_FROM_BIT), crate::meta::inspect::EnumConstant::new("CAN_COPY_TO_BIT", "TEXTURE_USAGE_CAN_COPY_TO_BIT", TextureUsageBits::CAN_COPY_TO_BIT), crate::meta::inspect::EnumConstant::new("INPUT_ATTACHMENT_BIT", "TEXTURE_USAGE_INPUT_ATTACHMENT_BIT", TextureUsageBits::INPUT_ATTACHMENT_BIT)]
        }
    }
}
impl std::ops::BitOr for TextureUsageBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for TextureUsageBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for TextureUsageBits {
    type Via = u64;
    
}
impl crate::meta::ToGodot for TextureUsageBits {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureUsageBits {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureSwizzle {
    ord: i32
}
impl TextureSwizzle {
    #[doc(alias = "TEXTURE_SWIZZLE_IDENTITY")]
    #[doc = "Godot enumerator name: `TEXTURE_SWIZZLE_IDENTITY`"]
    pub const IDENTITY: TextureSwizzle = TextureSwizzle {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_SWIZZLE_ZERO")]
    #[doc = "Godot enumerator name: `TEXTURE_SWIZZLE_ZERO`"]
    pub const ZERO: TextureSwizzle = TextureSwizzle {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_SWIZZLE_ONE")]
    #[doc = "Godot enumerator name: `TEXTURE_SWIZZLE_ONE`"]
    pub const ONE: TextureSwizzle = TextureSwizzle {
        ord: 2i32
    };
    #[doc(alias = "TEXTURE_SWIZZLE_R")]
    #[doc = "Godot enumerator name: `TEXTURE_SWIZZLE_R`"]
    pub const R: TextureSwizzle = TextureSwizzle {
        ord: 3i32
    };
    #[doc(alias = "TEXTURE_SWIZZLE_G")]
    #[doc = "Godot enumerator name: `TEXTURE_SWIZZLE_G`"]
    pub const G: TextureSwizzle = TextureSwizzle {
        ord: 4i32
    };
    #[doc(alias = "TEXTURE_SWIZZLE_B")]
    #[doc = "Godot enumerator name: `TEXTURE_SWIZZLE_B`"]
    pub const B: TextureSwizzle = TextureSwizzle {
        ord: 5i32
    };
    #[doc(alias = "TEXTURE_SWIZZLE_A")]
    #[doc = "Godot enumerator name: `TEXTURE_SWIZZLE_A`"]
    pub const A: TextureSwizzle = TextureSwizzle {
        ord: 6i32
    };
    #[doc(alias = "TEXTURE_SWIZZLE_MAX")]
    #[doc = "Godot enumerator name: `TEXTURE_SWIZZLE_MAX`"]
    pub const MAX: TextureSwizzle = TextureSwizzle {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for TextureSwizzle {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureSwizzle") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureSwizzle {
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
            Self::IDENTITY => "IDENTITY", Self::ZERO => "ZERO", Self::ONE => "ONE", Self::R => "R", Self::G => "G", Self::B => "B", Self::A => "A", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureSwizzle::IDENTITY, TextureSwizzle::ZERO, TextureSwizzle::ONE, TextureSwizzle::R, TextureSwizzle::G, TextureSwizzle::B, TextureSwizzle::A]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureSwizzle >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("IDENTITY", "TEXTURE_SWIZZLE_IDENTITY", TextureSwizzle::IDENTITY), crate::meta::inspect::EnumConstant::new("ZERO", "TEXTURE_SWIZZLE_ZERO", TextureSwizzle::ZERO), crate::meta::inspect::EnumConstant::new("ONE", "TEXTURE_SWIZZLE_ONE", TextureSwizzle::ONE), crate::meta::inspect::EnumConstant::new("R", "TEXTURE_SWIZZLE_R", TextureSwizzle::R), crate::meta::inspect::EnumConstant::new("G", "TEXTURE_SWIZZLE_G", TextureSwizzle::G), crate::meta::inspect::EnumConstant::new("B", "TEXTURE_SWIZZLE_B", TextureSwizzle::B), crate::meta::inspect::EnumConstant::new("A", "TEXTURE_SWIZZLE_A", TextureSwizzle::A), crate::meta::inspect::EnumConstant::new("MAX", "TEXTURE_SWIZZLE_MAX", TextureSwizzle::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for TextureSwizzle {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for TextureSwizzle {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureSwizzle {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureSwizzle {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureSliceType {
    ord: i32
}
impl TextureSliceType {
    #[doc(alias = "TEXTURE_SLICE_2D")]
    #[doc = "Godot enumerator name: `TEXTURE_SLICE_2D`"]
    pub const SLICE_2D: TextureSliceType = TextureSliceType {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_SLICE_CUBEMAP")]
    #[doc = "Godot enumerator name: `TEXTURE_SLICE_CUBEMAP`"]
    pub const CUBEMAP: TextureSliceType = TextureSliceType {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_SLICE_3D")]
    #[doc = "Godot enumerator name: `TEXTURE_SLICE_3D`"]
    pub const SLICE_3D: TextureSliceType = TextureSliceType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TextureSliceType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureSliceType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureSliceType {
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
            Self::SLICE_2D => "SLICE_2D", Self::CUBEMAP => "CUBEMAP", Self::SLICE_3D => "SLICE_3D", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureSliceType::SLICE_2D, TextureSliceType::CUBEMAP, TextureSliceType::SLICE_3D]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureSliceType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SLICE_2D", "TEXTURE_SLICE_2D", TextureSliceType::SLICE_2D), crate::meta::inspect::EnumConstant::new("CUBEMAP", "TEXTURE_SLICE_CUBEMAP", TextureSliceType::CUBEMAP), crate::meta::inspect::EnumConstant::new("SLICE_3D", "TEXTURE_SLICE_3D", TextureSliceType::SLICE_3D)]
        }
    }
}
impl crate::meta::GodotConvert for TextureSliceType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureSliceType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureSliceType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SamplerFilter {
    ord: i32
}
impl SamplerFilter {
    #[doc(alias = "SAMPLER_FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `SAMPLER_FILTER_NEAREST`"]
    pub const NEAREST: SamplerFilter = SamplerFilter {
        ord: 0i32
    };
    #[doc(alias = "SAMPLER_FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `SAMPLER_FILTER_LINEAR`"]
    pub const LINEAR: SamplerFilter = SamplerFilter {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for SamplerFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SamplerFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SamplerFilter {
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
            Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SamplerFilter::NEAREST, SamplerFilter::LINEAR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SamplerFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEAREST", "SAMPLER_FILTER_NEAREST", SamplerFilter::NEAREST), crate::meta::inspect::EnumConstant::new("LINEAR", "SAMPLER_FILTER_LINEAR", SamplerFilter::LINEAR)]
        }
    }
}
impl crate::meta::GodotConvert for SamplerFilter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SamplerFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SamplerFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SamplerRepeatMode {
    ord: i32
}
impl SamplerRepeatMode {
    #[doc(alias = "SAMPLER_REPEAT_MODE_REPEAT")]
    #[doc = "Godot enumerator name: `SAMPLER_REPEAT_MODE_REPEAT`"]
    pub const REPEAT: SamplerRepeatMode = SamplerRepeatMode {
        ord: 0i32
    };
    #[doc(alias = "SAMPLER_REPEAT_MODE_MIRRORED_REPEAT")]
    #[doc = "Godot enumerator name: `SAMPLER_REPEAT_MODE_MIRRORED_REPEAT`"]
    pub const MIRRORED_REPEAT: SamplerRepeatMode = SamplerRepeatMode {
        ord: 1i32
    };
    #[doc(alias = "SAMPLER_REPEAT_MODE_CLAMP_TO_EDGE")]
    #[doc = "Godot enumerator name: `SAMPLER_REPEAT_MODE_CLAMP_TO_EDGE`"]
    pub const CLAMP_TO_EDGE: SamplerRepeatMode = SamplerRepeatMode {
        ord: 2i32
    };
    #[doc(alias = "SAMPLER_REPEAT_MODE_CLAMP_TO_BORDER")]
    #[doc = "Godot enumerator name: `SAMPLER_REPEAT_MODE_CLAMP_TO_BORDER`"]
    pub const CLAMP_TO_BORDER: SamplerRepeatMode = SamplerRepeatMode {
        ord: 3i32
    };
    #[doc(alias = "SAMPLER_REPEAT_MODE_MIRROR_CLAMP_TO_EDGE")]
    #[doc = "Godot enumerator name: `SAMPLER_REPEAT_MODE_MIRROR_CLAMP_TO_EDGE`"]
    pub const MIRROR_CLAMP_TO_EDGE: SamplerRepeatMode = SamplerRepeatMode {
        ord: 4i32
    };
    #[doc(alias = "SAMPLER_REPEAT_MODE_MAX")]
    #[doc = "Godot enumerator name: `SAMPLER_REPEAT_MODE_MAX`"]
    pub const MAX: SamplerRepeatMode = SamplerRepeatMode {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for SamplerRepeatMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SamplerRepeatMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SamplerRepeatMode {
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
            Self::REPEAT => "REPEAT", Self::MIRRORED_REPEAT => "MIRRORED_REPEAT", Self::CLAMP_TO_EDGE => "CLAMP_TO_EDGE", Self::CLAMP_TO_BORDER => "CLAMP_TO_BORDER", Self::MIRROR_CLAMP_TO_EDGE => "MIRROR_CLAMP_TO_EDGE", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SamplerRepeatMode::REPEAT, SamplerRepeatMode::MIRRORED_REPEAT, SamplerRepeatMode::CLAMP_TO_EDGE, SamplerRepeatMode::CLAMP_TO_BORDER, SamplerRepeatMode::MIRROR_CLAMP_TO_EDGE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SamplerRepeatMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("REPEAT", "SAMPLER_REPEAT_MODE_REPEAT", SamplerRepeatMode::REPEAT), crate::meta::inspect::EnumConstant::new("MIRRORED_REPEAT", "SAMPLER_REPEAT_MODE_MIRRORED_REPEAT", SamplerRepeatMode::MIRRORED_REPEAT), crate::meta::inspect::EnumConstant::new("CLAMP_TO_EDGE", "SAMPLER_REPEAT_MODE_CLAMP_TO_EDGE", SamplerRepeatMode::CLAMP_TO_EDGE), crate::meta::inspect::EnumConstant::new("CLAMP_TO_BORDER", "SAMPLER_REPEAT_MODE_CLAMP_TO_BORDER", SamplerRepeatMode::CLAMP_TO_BORDER), crate::meta::inspect::EnumConstant::new("MIRROR_CLAMP_TO_EDGE", "SAMPLER_REPEAT_MODE_MIRROR_CLAMP_TO_EDGE", SamplerRepeatMode::MIRROR_CLAMP_TO_EDGE), crate::meta::inspect::EnumConstant::new("MAX", "SAMPLER_REPEAT_MODE_MAX", SamplerRepeatMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for SamplerRepeatMode {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for SamplerRepeatMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SamplerRepeatMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SamplerRepeatMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SamplerBorderColor {
    ord: i32
}
impl SamplerBorderColor {
    #[doc(alias = "SAMPLER_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK")]
    #[doc = "Godot enumerator name: `SAMPLER_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK`"]
    pub const FLOAT_TRANSPARENT_BLACK: SamplerBorderColor = SamplerBorderColor {
        ord: 0i32
    };
    #[doc(alias = "SAMPLER_BORDER_COLOR_INT_TRANSPARENT_BLACK")]
    #[doc = "Godot enumerator name: `SAMPLER_BORDER_COLOR_INT_TRANSPARENT_BLACK`"]
    pub const INT_TRANSPARENT_BLACK: SamplerBorderColor = SamplerBorderColor {
        ord: 1i32
    };
    #[doc(alias = "SAMPLER_BORDER_COLOR_FLOAT_OPAQUE_BLACK")]
    #[doc = "Godot enumerator name: `SAMPLER_BORDER_COLOR_FLOAT_OPAQUE_BLACK`"]
    pub const FLOAT_OPAQUE_BLACK: SamplerBorderColor = SamplerBorderColor {
        ord: 2i32
    };
    #[doc(alias = "SAMPLER_BORDER_COLOR_INT_OPAQUE_BLACK")]
    #[doc = "Godot enumerator name: `SAMPLER_BORDER_COLOR_INT_OPAQUE_BLACK`"]
    pub const INT_OPAQUE_BLACK: SamplerBorderColor = SamplerBorderColor {
        ord: 3i32
    };
    #[doc(alias = "SAMPLER_BORDER_COLOR_FLOAT_OPAQUE_WHITE")]
    #[doc = "Godot enumerator name: `SAMPLER_BORDER_COLOR_FLOAT_OPAQUE_WHITE`"]
    pub const FLOAT_OPAQUE_WHITE: SamplerBorderColor = SamplerBorderColor {
        ord: 4i32
    };
    #[doc(alias = "SAMPLER_BORDER_COLOR_INT_OPAQUE_WHITE")]
    #[doc = "Godot enumerator name: `SAMPLER_BORDER_COLOR_INT_OPAQUE_WHITE`"]
    pub const INT_OPAQUE_WHITE: SamplerBorderColor = SamplerBorderColor {
        ord: 5i32
    };
    #[doc(alias = "SAMPLER_BORDER_COLOR_MAX")]
    #[doc = "Godot enumerator name: `SAMPLER_BORDER_COLOR_MAX`"]
    pub const MAX: SamplerBorderColor = SamplerBorderColor {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for SamplerBorderColor {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SamplerBorderColor") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SamplerBorderColor {
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
            Self::FLOAT_TRANSPARENT_BLACK => "FLOAT_TRANSPARENT_BLACK", Self::INT_TRANSPARENT_BLACK => "INT_TRANSPARENT_BLACK", Self::FLOAT_OPAQUE_BLACK => "FLOAT_OPAQUE_BLACK", Self::INT_OPAQUE_BLACK => "INT_OPAQUE_BLACK", Self::FLOAT_OPAQUE_WHITE => "FLOAT_OPAQUE_WHITE", Self::INT_OPAQUE_WHITE => "INT_OPAQUE_WHITE", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SamplerBorderColor::FLOAT_TRANSPARENT_BLACK, SamplerBorderColor::INT_TRANSPARENT_BLACK, SamplerBorderColor::FLOAT_OPAQUE_BLACK, SamplerBorderColor::INT_OPAQUE_BLACK, SamplerBorderColor::FLOAT_OPAQUE_WHITE, SamplerBorderColor::INT_OPAQUE_WHITE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SamplerBorderColor >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FLOAT_TRANSPARENT_BLACK", "SAMPLER_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK", SamplerBorderColor::FLOAT_TRANSPARENT_BLACK), crate::meta::inspect::EnumConstant::new("INT_TRANSPARENT_BLACK", "SAMPLER_BORDER_COLOR_INT_TRANSPARENT_BLACK", SamplerBorderColor::INT_TRANSPARENT_BLACK), crate::meta::inspect::EnumConstant::new("FLOAT_OPAQUE_BLACK", "SAMPLER_BORDER_COLOR_FLOAT_OPAQUE_BLACK", SamplerBorderColor::FLOAT_OPAQUE_BLACK), crate::meta::inspect::EnumConstant::new("INT_OPAQUE_BLACK", "SAMPLER_BORDER_COLOR_INT_OPAQUE_BLACK", SamplerBorderColor::INT_OPAQUE_BLACK), crate::meta::inspect::EnumConstant::new("FLOAT_OPAQUE_WHITE", "SAMPLER_BORDER_COLOR_FLOAT_OPAQUE_WHITE", SamplerBorderColor::FLOAT_OPAQUE_WHITE), crate::meta::inspect::EnumConstant::new("INT_OPAQUE_WHITE", "SAMPLER_BORDER_COLOR_INT_OPAQUE_WHITE", SamplerBorderColor::INT_OPAQUE_WHITE), crate::meta::inspect::EnumConstant::new("MAX", "SAMPLER_BORDER_COLOR_MAX", SamplerBorderColor::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for SamplerBorderColor {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for SamplerBorderColor {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SamplerBorderColor {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SamplerBorderColor {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VertexFrequency {
    ord: i32
}
impl VertexFrequency {
    #[doc(alias = "VERTEX_FREQUENCY_VERTEX")]
    #[doc = "Godot enumerator name: `VERTEX_FREQUENCY_VERTEX`"]
    pub const VERTEX: VertexFrequency = VertexFrequency {
        ord: 0i32
    };
    #[doc(alias = "VERTEX_FREQUENCY_INSTANCE")]
    #[doc = "Godot enumerator name: `VERTEX_FREQUENCY_INSTANCE`"]
    pub const INSTANCE: VertexFrequency = VertexFrequency {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for VertexFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VertexFrequency") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VertexFrequency {
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
            Self::VERTEX => "VERTEX", Self::INSTANCE => "INSTANCE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VertexFrequency::VERTEX, VertexFrequency::INSTANCE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VertexFrequency >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERTEX", "VERTEX_FREQUENCY_VERTEX", VertexFrequency::VERTEX), crate::meta::inspect::EnumConstant::new("INSTANCE", "VERTEX_FREQUENCY_INSTANCE", VertexFrequency::INSTANCE)]
        }
    }
}
impl crate::meta::GodotConvert for VertexFrequency {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VertexFrequency {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VertexFrequency {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct IndexBufferFormat {
    ord: i32
}
impl IndexBufferFormat {
    #[doc(alias = "INDEX_BUFFER_FORMAT_UINT16")]
    #[doc = "Godot enumerator name: `INDEX_BUFFER_FORMAT_UINT16`"]
    pub const UINT16: IndexBufferFormat = IndexBufferFormat {
        ord: 0i32
    };
    #[doc(alias = "INDEX_BUFFER_FORMAT_UINT32")]
    #[doc = "Godot enumerator name: `INDEX_BUFFER_FORMAT_UINT32`"]
    pub const UINT32: IndexBufferFormat = IndexBufferFormat {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for IndexBufferFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("IndexBufferFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for IndexBufferFormat {
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
            Self::UINT16 => "UINT16", Self::UINT32 => "UINT32", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[IndexBufferFormat::UINT16, IndexBufferFormat::UINT32]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < IndexBufferFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UINT16", "INDEX_BUFFER_FORMAT_UINT16", IndexBufferFormat::UINT16), crate::meta::inspect::EnumConstant::new("UINT32", "INDEX_BUFFER_FORMAT_UINT32", IndexBufferFormat::UINT32)]
        }
    }
}
impl crate::meta::GodotConvert for IndexBufferFormat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for IndexBufferFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for IndexBufferFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct StorageBufferUsage {
    ord: u64
}
impl StorageBufferUsage {
    #[doc(alias = "STORAGE_BUFFER_USAGE_DISPATCH_INDIRECT")]
    #[doc = "Godot enumerator name: `STORAGE_BUFFER_USAGE_DISPATCH_INDIRECT`"]
    pub const DISPATCH_INDIRECT: StorageBufferUsage = StorageBufferUsage {
        ord: 1u64
    };
    
}
impl std::fmt::Debug for StorageBufferUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::DISPATCH_INDIRECT => "DISPATCH_INDIRECT", _ => {
                f.debug_struct("StorageBufferUsage") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for StorageBufferUsage {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < StorageBufferUsage >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISPATCH_INDIRECT", "STORAGE_BUFFER_USAGE_DISPATCH_INDIRECT", StorageBufferUsage::DISPATCH_INDIRECT)]
        }
    }
}
impl std::ops::BitOr for StorageBufferUsage {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for StorageBufferUsage {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for StorageBufferUsage {
    type Via = u64;
    
}
impl crate::meta::ToGodot for StorageBufferUsage {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for StorageBufferUsage {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct BufferCreationBits {
    ord: u64
}
impl BufferCreationBits {
    #[doc(alias = "BUFFER_CREATION_DEVICE_ADDRESS_BIT")]
    #[doc = "Godot enumerator name: `BUFFER_CREATION_DEVICE_ADDRESS_BIT`"]
    pub const DEVICE_ADDRESS_BIT: BufferCreationBits = BufferCreationBits {
        ord: 1u64
    };
    #[doc(alias = "BUFFER_CREATION_AS_STORAGE_BIT")]
    #[doc = "Godot enumerator name: `BUFFER_CREATION_AS_STORAGE_BIT`"]
    pub const AS_STORAGE_BIT: BufferCreationBits = BufferCreationBits {
        ord: 2u64
    };
    
}
impl std::fmt::Debug for BufferCreationBits {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::DEVICE_ADDRESS_BIT => "DEVICE_ADDRESS_BIT", Self::AS_STORAGE_BIT => "AS_STORAGE_BIT", _ => {
                f.debug_struct("BufferCreationBits") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for BufferCreationBits {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BufferCreationBits >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEVICE_ADDRESS_BIT", "BUFFER_CREATION_DEVICE_ADDRESS_BIT", BufferCreationBits::DEVICE_ADDRESS_BIT), crate::meta::inspect::EnumConstant::new("AS_STORAGE_BIT", "BUFFER_CREATION_AS_STORAGE_BIT", BufferCreationBits::AS_STORAGE_BIT)]
        }
    }
}
impl std::ops::BitOr for BufferCreationBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for BufferCreationBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for BufferCreationBits {
    type Via = u64;
    
}
impl crate::meta::ToGodot for BufferCreationBits {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BufferCreationBits {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct UniformType {
    ord: i32
}
impl UniformType {
    #[doc(alias = "UNIFORM_TYPE_SAMPLER")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_SAMPLER`"]
    pub const SAMPLER: UniformType = UniformType {
        ord: 0i32
    };
    #[doc(alias = "UNIFORM_TYPE_SAMPLER_WITH_TEXTURE")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_SAMPLER_WITH_TEXTURE`"]
    pub const SAMPLER_WITH_TEXTURE: UniformType = UniformType {
        ord: 1i32
    };
    #[doc(alias = "UNIFORM_TYPE_TEXTURE")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_TEXTURE`"]
    pub const TEXTURE: UniformType = UniformType {
        ord: 2i32
    };
    #[doc(alias = "UNIFORM_TYPE_IMAGE")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_IMAGE`"]
    pub const IMAGE: UniformType = UniformType {
        ord: 3i32
    };
    #[doc(alias = "UNIFORM_TYPE_TEXTURE_BUFFER")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_TEXTURE_BUFFER`"]
    pub const TEXTURE_BUFFER: UniformType = UniformType {
        ord: 4i32
    };
    #[doc(alias = "UNIFORM_TYPE_SAMPLER_WITH_TEXTURE_BUFFER")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_SAMPLER_WITH_TEXTURE_BUFFER`"]
    pub const SAMPLER_WITH_TEXTURE_BUFFER: UniformType = UniformType {
        ord: 5i32
    };
    #[doc(alias = "UNIFORM_TYPE_IMAGE_BUFFER")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_IMAGE_BUFFER`"]
    pub const IMAGE_BUFFER: UniformType = UniformType {
        ord: 6i32
    };
    #[doc(alias = "UNIFORM_TYPE_UNIFORM_BUFFER")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_UNIFORM_BUFFER`"]
    pub const UNIFORM_BUFFER: UniformType = UniformType {
        ord: 7i32
    };
    #[doc(alias = "UNIFORM_TYPE_STORAGE_BUFFER")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_STORAGE_BUFFER`"]
    pub const STORAGE_BUFFER: UniformType = UniformType {
        ord: 8i32
    };
    #[doc(alias = "UNIFORM_TYPE_INPUT_ATTACHMENT")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_INPUT_ATTACHMENT`"]
    pub const INPUT_ATTACHMENT: UniformType = UniformType {
        ord: 9i32
    };
    #[doc(alias = "UNIFORM_TYPE_MAX")]
    #[doc = "Godot enumerator name: `UNIFORM_TYPE_MAX`"]
    pub const MAX: UniformType = UniformType {
        ord: 10i32
    };
    
}
impl std::fmt::Debug for UniformType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("UniformType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for UniformType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 => Some(Self {
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
            Self::SAMPLER => "SAMPLER", Self::SAMPLER_WITH_TEXTURE => "SAMPLER_WITH_TEXTURE", Self::TEXTURE => "TEXTURE", Self::IMAGE => "IMAGE", Self::TEXTURE_BUFFER => "TEXTURE_BUFFER", Self::SAMPLER_WITH_TEXTURE_BUFFER => "SAMPLER_WITH_TEXTURE_BUFFER", Self::IMAGE_BUFFER => "IMAGE_BUFFER", Self::UNIFORM_BUFFER => "UNIFORM_BUFFER", Self::STORAGE_BUFFER => "STORAGE_BUFFER", Self::INPUT_ATTACHMENT => "INPUT_ATTACHMENT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[UniformType::SAMPLER, UniformType::SAMPLER_WITH_TEXTURE, UniformType::TEXTURE, UniformType::IMAGE, UniformType::TEXTURE_BUFFER, UniformType::SAMPLER_WITH_TEXTURE_BUFFER, UniformType::IMAGE_BUFFER, UniformType::UNIFORM_BUFFER, UniformType::STORAGE_BUFFER, UniformType::INPUT_ATTACHMENT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < UniformType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SAMPLER", "UNIFORM_TYPE_SAMPLER", UniformType::SAMPLER), crate::meta::inspect::EnumConstant::new("SAMPLER_WITH_TEXTURE", "UNIFORM_TYPE_SAMPLER_WITH_TEXTURE", UniformType::SAMPLER_WITH_TEXTURE), crate::meta::inspect::EnumConstant::new("TEXTURE", "UNIFORM_TYPE_TEXTURE", UniformType::TEXTURE), crate::meta::inspect::EnumConstant::new("IMAGE", "UNIFORM_TYPE_IMAGE", UniformType::IMAGE), crate::meta::inspect::EnumConstant::new("TEXTURE_BUFFER", "UNIFORM_TYPE_TEXTURE_BUFFER", UniformType::TEXTURE_BUFFER), crate::meta::inspect::EnumConstant::new("SAMPLER_WITH_TEXTURE_BUFFER", "UNIFORM_TYPE_SAMPLER_WITH_TEXTURE_BUFFER", UniformType::SAMPLER_WITH_TEXTURE_BUFFER), crate::meta::inspect::EnumConstant::new("IMAGE_BUFFER", "UNIFORM_TYPE_IMAGE_BUFFER", UniformType::IMAGE_BUFFER), crate::meta::inspect::EnumConstant::new("UNIFORM_BUFFER", "UNIFORM_TYPE_UNIFORM_BUFFER", UniformType::UNIFORM_BUFFER), crate::meta::inspect::EnumConstant::new("STORAGE_BUFFER", "UNIFORM_TYPE_STORAGE_BUFFER", UniformType::STORAGE_BUFFER), crate::meta::inspect::EnumConstant::new("INPUT_ATTACHMENT", "UNIFORM_TYPE_INPUT_ATTACHMENT", UniformType::INPUT_ATTACHMENT), crate::meta::inspect::EnumConstant::new("MAX", "UNIFORM_TYPE_MAX", UniformType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for UniformType {
    const ENUMERATOR_COUNT: usize = 10usize;
    
}
impl crate::meta::GodotConvert for UniformType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for UniformType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for UniformType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RenderPrimitive {
    ord: i32
}
impl RenderPrimitive {
    #[doc(alias = "RENDER_PRIMITIVE_POINTS")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_POINTS`"]
    pub const POINTS: RenderPrimitive = RenderPrimitive {
        ord: 0i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_LINES")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_LINES`"]
    pub const LINES: RenderPrimitive = RenderPrimitive {
        ord: 1i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_LINES_WITH_ADJACENCY")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_LINES_WITH_ADJACENCY`"]
    pub const LINES_WITH_ADJACENCY: RenderPrimitive = RenderPrimitive {
        ord: 2i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_LINESTRIPS")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_LINESTRIPS`"]
    pub const LINESTRIPS: RenderPrimitive = RenderPrimitive {
        ord: 3i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_LINESTRIPS_WITH_ADJACENCY")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_LINESTRIPS_WITH_ADJACENCY`"]
    pub const LINESTRIPS_WITH_ADJACENCY: RenderPrimitive = RenderPrimitive {
        ord: 4i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_TRIANGLES")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_TRIANGLES`"]
    pub const TRIANGLES: RenderPrimitive = RenderPrimitive {
        ord: 5i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_TRIANGLES_WITH_ADJACENCY")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_TRIANGLES_WITH_ADJACENCY`"]
    pub const TRIANGLES_WITH_ADJACENCY: RenderPrimitive = RenderPrimitive {
        ord: 6i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_TRIANGLE_STRIPS")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_TRIANGLE_STRIPS`"]
    pub const TRIANGLE_STRIPS: RenderPrimitive = RenderPrimitive {
        ord: 7i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_TRIANGLE_STRIPS_WITH_AJACENCY")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_TRIANGLE_STRIPS_WITH_AJACENCY`"]
    pub const TRIANGLE_STRIPS_WITH_AJACENCY: RenderPrimitive = RenderPrimitive {
        ord: 8i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_TRIANGLE_STRIPS_WITH_RESTART_INDEX")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_TRIANGLE_STRIPS_WITH_RESTART_INDEX`"]
    pub const TRIANGLE_STRIPS_WITH_RESTART_INDEX: RenderPrimitive = RenderPrimitive {
        ord: 9i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_TESSELATION_PATCH")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_TESSELATION_PATCH`"]
    pub const TESSELATION_PATCH: RenderPrimitive = RenderPrimitive {
        ord: 10i32
    };
    #[doc(alias = "RENDER_PRIMITIVE_MAX")]
    #[doc = "Godot enumerator name: `RENDER_PRIMITIVE_MAX`"]
    pub const MAX: RenderPrimitive = RenderPrimitive {
        ord: 11i32
    };
    
}
impl std::fmt::Debug for RenderPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RenderPrimitive") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RenderPrimitive {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 => Some(Self {
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
            Self::POINTS => "POINTS", Self::LINES => "LINES", Self::LINES_WITH_ADJACENCY => "LINES_WITH_ADJACENCY", Self::LINESTRIPS => "LINESTRIPS", Self::LINESTRIPS_WITH_ADJACENCY => "LINESTRIPS_WITH_ADJACENCY", Self::TRIANGLES => "TRIANGLES", Self::TRIANGLES_WITH_ADJACENCY => "TRIANGLES_WITH_ADJACENCY", Self::TRIANGLE_STRIPS => "TRIANGLE_STRIPS", Self::TRIANGLE_STRIPS_WITH_AJACENCY => "TRIANGLE_STRIPS_WITH_AJACENCY", Self::TRIANGLE_STRIPS_WITH_RESTART_INDEX => "TRIANGLE_STRIPS_WITH_RESTART_INDEX", Self::TESSELATION_PATCH => "TESSELATION_PATCH", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[RenderPrimitive::POINTS, RenderPrimitive::LINES, RenderPrimitive::LINES_WITH_ADJACENCY, RenderPrimitive::LINESTRIPS, RenderPrimitive::LINESTRIPS_WITH_ADJACENCY, RenderPrimitive::TRIANGLES, RenderPrimitive::TRIANGLES_WITH_ADJACENCY, RenderPrimitive::TRIANGLE_STRIPS, RenderPrimitive::TRIANGLE_STRIPS_WITH_AJACENCY, RenderPrimitive::TRIANGLE_STRIPS_WITH_RESTART_INDEX, RenderPrimitive::TESSELATION_PATCH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < RenderPrimitive >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("POINTS", "RENDER_PRIMITIVE_POINTS", RenderPrimitive::POINTS), crate::meta::inspect::EnumConstant::new("LINES", "RENDER_PRIMITIVE_LINES", RenderPrimitive::LINES), crate::meta::inspect::EnumConstant::new("LINES_WITH_ADJACENCY", "RENDER_PRIMITIVE_LINES_WITH_ADJACENCY", RenderPrimitive::LINES_WITH_ADJACENCY), crate::meta::inspect::EnumConstant::new("LINESTRIPS", "RENDER_PRIMITIVE_LINESTRIPS", RenderPrimitive::LINESTRIPS), crate::meta::inspect::EnumConstant::new("LINESTRIPS_WITH_ADJACENCY", "RENDER_PRIMITIVE_LINESTRIPS_WITH_ADJACENCY", RenderPrimitive::LINESTRIPS_WITH_ADJACENCY), crate::meta::inspect::EnumConstant::new("TRIANGLES", "RENDER_PRIMITIVE_TRIANGLES", RenderPrimitive::TRIANGLES), crate::meta::inspect::EnumConstant::new("TRIANGLES_WITH_ADJACENCY", "RENDER_PRIMITIVE_TRIANGLES_WITH_ADJACENCY", RenderPrimitive::TRIANGLES_WITH_ADJACENCY), crate::meta::inspect::EnumConstant::new("TRIANGLE_STRIPS", "RENDER_PRIMITIVE_TRIANGLE_STRIPS", RenderPrimitive::TRIANGLE_STRIPS), crate::meta::inspect::EnumConstant::new("TRIANGLE_STRIPS_WITH_AJACENCY", "RENDER_PRIMITIVE_TRIANGLE_STRIPS_WITH_AJACENCY", RenderPrimitive::TRIANGLE_STRIPS_WITH_AJACENCY), crate::meta::inspect::EnumConstant::new("TRIANGLE_STRIPS_WITH_RESTART_INDEX", "RENDER_PRIMITIVE_TRIANGLE_STRIPS_WITH_RESTART_INDEX", RenderPrimitive::TRIANGLE_STRIPS_WITH_RESTART_INDEX), crate::meta::inspect::EnumConstant::new("TESSELATION_PATCH", "RENDER_PRIMITIVE_TESSELATION_PATCH", RenderPrimitive::TESSELATION_PATCH), crate::meta::inspect::EnumConstant::new("MAX", "RENDER_PRIMITIVE_MAX", RenderPrimitive::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for RenderPrimitive {
    const ENUMERATOR_COUNT: usize = 11usize;
    
}
impl crate::meta::GodotConvert for RenderPrimitive {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RenderPrimitive {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RenderPrimitive {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PolygonCullMode {
    ord: i32
}
impl PolygonCullMode {
    #[doc(alias = "POLYGON_CULL_DISABLED")]
    #[doc = "Godot enumerator name: `POLYGON_CULL_DISABLED`"]
    pub const DISABLED: PolygonCullMode = PolygonCullMode {
        ord: 0i32
    };
    #[doc(alias = "POLYGON_CULL_FRONT")]
    #[doc = "Godot enumerator name: `POLYGON_CULL_FRONT`"]
    pub const FRONT: PolygonCullMode = PolygonCullMode {
        ord: 1i32
    };
    #[doc(alias = "POLYGON_CULL_BACK")]
    #[doc = "Godot enumerator name: `POLYGON_CULL_BACK`"]
    pub const BACK: PolygonCullMode = PolygonCullMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PolygonCullMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PolygonCullMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PolygonCullMode {
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
            Self::DISABLED => "DISABLED", Self::FRONT => "FRONT", Self::BACK => "BACK", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PolygonCullMode::DISABLED, PolygonCullMode::FRONT, PolygonCullMode::BACK]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PolygonCullMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "POLYGON_CULL_DISABLED", PolygonCullMode::DISABLED), crate::meta::inspect::EnumConstant::new("FRONT", "POLYGON_CULL_FRONT", PolygonCullMode::FRONT), crate::meta::inspect::EnumConstant::new("BACK", "POLYGON_CULL_BACK", PolygonCullMode::BACK)]
        }
    }
}
impl crate::meta::GodotConvert for PolygonCullMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PolygonCullMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PolygonCullMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PolygonFrontFace {
    ord: i32
}
impl PolygonFrontFace {
    #[doc(alias = "POLYGON_FRONT_FACE_CLOCKWISE")]
    #[doc = "Godot enumerator name: `POLYGON_FRONT_FACE_CLOCKWISE`"]
    pub const CLOCKWISE: PolygonFrontFace = PolygonFrontFace {
        ord: 0i32
    };
    #[doc(alias = "POLYGON_FRONT_FACE_COUNTER_CLOCKWISE")]
    #[doc = "Godot enumerator name: `POLYGON_FRONT_FACE_COUNTER_CLOCKWISE`"]
    pub const COUNTER_CLOCKWISE: PolygonFrontFace = PolygonFrontFace {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for PolygonFrontFace {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PolygonFrontFace") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PolygonFrontFace {
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
            Self::CLOCKWISE => "CLOCKWISE", Self::COUNTER_CLOCKWISE => "COUNTER_CLOCKWISE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PolygonFrontFace::CLOCKWISE, PolygonFrontFace::COUNTER_CLOCKWISE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PolygonFrontFace >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CLOCKWISE", "POLYGON_FRONT_FACE_CLOCKWISE", PolygonFrontFace::CLOCKWISE), crate::meta::inspect::EnumConstant::new("COUNTER_CLOCKWISE", "POLYGON_FRONT_FACE_COUNTER_CLOCKWISE", PolygonFrontFace::COUNTER_CLOCKWISE)]
        }
    }
}
impl crate::meta::GodotConvert for PolygonFrontFace {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PolygonFrontFace {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PolygonFrontFace {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct StencilOperation {
    ord: i32
}
impl StencilOperation {
    #[doc(alias = "STENCIL_OP_KEEP")]
    #[doc = "Godot enumerator name: `STENCIL_OP_KEEP`"]
    pub const KEEP: StencilOperation = StencilOperation {
        ord: 0i32
    };
    #[doc(alias = "STENCIL_OP_ZERO")]
    #[doc = "Godot enumerator name: `STENCIL_OP_ZERO`"]
    pub const ZERO: StencilOperation = StencilOperation {
        ord: 1i32
    };
    #[doc(alias = "STENCIL_OP_REPLACE")]
    #[doc = "Godot enumerator name: `STENCIL_OP_REPLACE`"]
    pub const REPLACE: StencilOperation = StencilOperation {
        ord: 2i32
    };
    #[doc(alias = "STENCIL_OP_INCREMENT_AND_CLAMP")]
    #[doc = "Godot enumerator name: `STENCIL_OP_INCREMENT_AND_CLAMP`"]
    pub const INCREMENT_AND_CLAMP: StencilOperation = StencilOperation {
        ord: 3i32
    };
    #[doc(alias = "STENCIL_OP_DECREMENT_AND_CLAMP")]
    #[doc = "Godot enumerator name: `STENCIL_OP_DECREMENT_AND_CLAMP`"]
    pub const DECREMENT_AND_CLAMP: StencilOperation = StencilOperation {
        ord: 4i32
    };
    #[doc(alias = "STENCIL_OP_INVERT")]
    #[doc = "Godot enumerator name: `STENCIL_OP_INVERT`"]
    pub const INVERT: StencilOperation = StencilOperation {
        ord: 5i32
    };
    #[doc(alias = "STENCIL_OP_INCREMENT_AND_WRAP")]
    #[doc = "Godot enumerator name: `STENCIL_OP_INCREMENT_AND_WRAP`"]
    pub const INCREMENT_AND_WRAP: StencilOperation = StencilOperation {
        ord: 6i32
    };
    #[doc(alias = "STENCIL_OP_DECREMENT_AND_WRAP")]
    #[doc = "Godot enumerator name: `STENCIL_OP_DECREMENT_AND_WRAP`"]
    pub const DECREMENT_AND_WRAP: StencilOperation = StencilOperation {
        ord: 7i32
    };
    #[doc(alias = "STENCIL_OP_MAX")]
    #[doc = "Godot enumerator name: `STENCIL_OP_MAX`"]
    pub const MAX: StencilOperation = StencilOperation {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for StencilOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("StencilOperation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for StencilOperation {
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
            Self::KEEP => "KEEP", Self::ZERO => "ZERO", Self::REPLACE => "REPLACE", Self::INCREMENT_AND_CLAMP => "INCREMENT_AND_CLAMP", Self::DECREMENT_AND_CLAMP => "DECREMENT_AND_CLAMP", Self::INVERT => "INVERT", Self::INCREMENT_AND_WRAP => "INCREMENT_AND_WRAP", Self::DECREMENT_AND_WRAP => "DECREMENT_AND_WRAP", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[StencilOperation::KEEP, StencilOperation::ZERO, StencilOperation::REPLACE, StencilOperation::INCREMENT_AND_CLAMP, StencilOperation::DECREMENT_AND_CLAMP, StencilOperation::INVERT, StencilOperation::INCREMENT_AND_WRAP, StencilOperation::DECREMENT_AND_WRAP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < StencilOperation >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("KEEP", "STENCIL_OP_KEEP", StencilOperation::KEEP), crate::meta::inspect::EnumConstant::new("ZERO", "STENCIL_OP_ZERO", StencilOperation::ZERO), crate::meta::inspect::EnumConstant::new("REPLACE", "STENCIL_OP_REPLACE", StencilOperation::REPLACE), crate::meta::inspect::EnumConstant::new("INCREMENT_AND_CLAMP", "STENCIL_OP_INCREMENT_AND_CLAMP", StencilOperation::INCREMENT_AND_CLAMP), crate::meta::inspect::EnumConstant::new("DECREMENT_AND_CLAMP", "STENCIL_OP_DECREMENT_AND_CLAMP", StencilOperation::DECREMENT_AND_CLAMP), crate::meta::inspect::EnumConstant::new("INVERT", "STENCIL_OP_INVERT", StencilOperation::INVERT), crate::meta::inspect::EnumConstant::new("INCREMENT_AND_WRAP", "STENCIL_OP_INCREMENT_AND_WRAP", StencilOperation::INCREMENT_AND_WRAP), crate::meta::inspect::EnumConstant::new("DECREMENT_AND_WRAP", "STENCIL_OP_DECREMENT_AND_WRAP", StencilOperation::DECREMENT_AND_WRAP), crate::meta::inspect::EnumConstant::new("MAX", "STENCIL_OP_MAX", StencilOperation::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for StencilOperation {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for StencilOperation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for StencilOperation {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for StencilOperation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompareOperator {
    ord: i32
}
impl CompareOperator {
    #[doc(alias = "COMPARE_OP_NEVER")]
    #[doc = "Godot enumerator name: `COMPARE_OP_NEVER`"]
    pub const NEVER: CompareOperator = CompareOperator {
        ord: 0i32
    };
    #[doc(alias = "COMPARE_OP_LESS")]
    #[doc = "Godot enumerator name: `COMPARE_OP_LESS`"]
    pub const LESS: CompareOperator = CompareOperator {
        ord: 1i32
    };
    #[doc(alias = "COMPARE_OP_EQUAL")]
    #[doc = "Godot enumerator name: `COMPARE_OP_EQUAL`"]
    pub const EQUAL: CompareOperator = CompareOperator {
        ord: 2i32
    };
    #[doc(alias = "COMPARE_OP_LESS_OR_EQUAL")]
    #[doc = "Godot enumerator name: `COMPARE_OP_LESS_OR_EQUAL`"]
    pub const LESS_OR_EQUAL: CompareOperator = CompareOperator {
        ord: 3i32
    };
    #[doc(alias = "COMPARE_OP_GREATER")]
    #[doc = "Godot enumerator name: `COMPARE_OP_GREATER`"]
    pub const GREATER: CompareOperator = CompareOperator {
        ord: 4i32
    };
    #[doc(alias = "COMPARE_OP_NOT_EQUAL")]
    #[doc = "Godot enumerator name: `COMPARE_OP_NOT_EQUAL`"]
    pub const NOT_EQUAL: CompareOperator = CompareOperator {
        ord: 5i32
    };
    #[doc(alias = "COMPARE_OP_GREATER_OR_EQUAL")]
    #[doc = "Godot enumerator name: `COMPARE_OP_GREATER_OR_EQUAL`"]
    pub const GREATER_OR_EQUAL: CompareOperator = CompareOperator {
        ord: 6i32
    };
    #[doc(alias = "COMPARE_OP_ALWAYS")]
    #[doc = "Godot enumerator name: `COMPARE_OP_ALWAYS`"]
    pub const ALWAYS: CompareOperator = CompareOperator {
        ord: 7i32
    };
    #[doc(alias = "COMPARE_OP_MAX")]
    #[doc = "Godot enumerator name: `COMPARE_OP_MAX`"]
    pub const MAX: CompareOperator = CompareOperator {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for CompareOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CompareOperator") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CompareOperator {
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
            Self::NEVER => "NEVER", Self::LESS => "LESS", Self::EQUAL => "EQUAL", Self::LESS_OR_EQUAL => "LESS_OR_EQUAL", Self::GREATER => "GREATER", Self::NOT_EQUAL => "NOT_EQUAL", Self::GREATER_OR_EQUAL => "GREATER_OR_EQUAL", Self::ALWAYS => "ALWAYS", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CompareOperator::NEVER, CompareOperator::LESS, CompareOperator::EQUAL, CompareOperator::LESS_OR_EQUAL, CompareOperator::GREATER, CompareOperator::NOT_EQUAL, CompareOperator::GREATER_OR_EQUAL, CompareOperator::ALWAYS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CompareOperator >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEVER", "COMPARE_OP_NEVER", CompareOperator::NEVER), crate::meta::inspect::EnumConstant::new("LESS", "COMPARE_OP_LESS", CompareOperator::LESS), crate::meta::inspect::EnumConstant::new("EQUAL", "COMPARE_OP_EQUAL", CompareOperator::EQUAL), crate::meta::inspect::EnumConstant::new("LESS_OR_EQUAL", "COMPARE_OP_LESS_OR_EQUAL", CompareOperator::LESS_OR_EQUAL), crate::meta::inspect::EnumConstant::new("GREATER", "COMPARE_OP_GREATER", CompareOperator::GREATER), crate::meta::inspect::EnumConstant::new("NOT_EQUAL", "COMPARE_OP_NOT_EQUAL", CompareOperator::NOT_EQUAL), crate::meta::inspect::EnumConstant::new("GREATER_OR_EQUAL", "COMPARE_OP_GREATER_OR_EQUAL", CompareOperator::GREATER_OR_EQUAL), crate::meta::inspect::EnumConstant::new("ALWAYS", "COMPARE_OP_ALWAYS", CompareOperator::ALWAYS), crate::meta::inspect::EnumConstant::new("MAX", "COMPARE_OP_MAX", CompareOperator::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CompareOperator {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for CompareOperator {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CompareOperator {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CompareOperator {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LogicOperation {
    ord: i32
}
impl LogicOperation {
    #[doc(alias = "LOGIC_OP_CLEAR")]
    #[doc = "Godot enumerator name: `LOGIC_OP_CLEAR`"]
    pub const CLEAR: LogicOperation = LogicOperation {
        ord: 0i32
    };
    #[doc(alias = "LOGIC_OP_AND")]
    #[doc = "Godot enumerator name: `LOGIC_OP_AND`"]
    pub const AND: LogicOperation = LogicOperation {
        ord: 1i32
    };
    #[doc(alias = "LOGIC_OP_AND_REVERSE")]
    #[doc = "Godot enumerator name: `LOGIC_OP_AND_REVERSE`"]
    pub const AND_REVERSE: LogicOperation = LogicOperation {
        ord: 2i32
    };
    #[doc(alias = "LOGIC_OP_COPY")]
    #[doc = "Godot enumerator name: `LOGIC_OP_COPY`"]
    pub const COPY: LogicOperation = LogicOperation {
        ord: 3i32
    };
    #[doc(alias = "LOGIC_OP_AND_INVERTED")]
    #[doc = "Godot enumerator name: `LOGIC_OP_AND_INVERTED`"]
    pub const AND_INVERTED: LogicOperation = LogicOperation {
        ord: 4i32
    };
    #[doc(alias = "LOGIC_OP_NO_OP")]
    #[doc = "Godot enumerator name: `LOGIC_OP_NO_OP`"]
    pub const NO_OP: LogicOperation = LogicOperation {
        ord: 5i32
    };
    #[doc(alias = "LOGIC_OP_XOR")]
    #[doc = "Godot enumerator name: `LOGIC_OP_XOR`"]
    pub const XOR: LogicOperation = LogicOperation {
        ord: 6i32
    };
    #[doc(alias = "LOGIC_OP_OR")]
    #[doc = "Godot enumerator name: `LOGIC_OP_OR`"]
    pub const OR: LogicOperation = LogicOperation {
        ord: 7i32
    };
    #[doc(alias = "LOGIC_OP_NOR")]
    #[doc = "Godot enumerator name: `LOGIC_OP_NOR`"]
    pub const NOR: LogicOperation = LogicOperation {
        ord: 8i32
    };
    #[doc(alias = "LOGIC_OP_EQUIVALENT")]
    #[doc = "Godot enumerator name: `LOGIC_OP_EQUIVALENT`"]
    pub const EQUIVALENT: LogicOperation = LogicOperation {
        ord: 9i32
    };
    #[doc(alias = "LOGIC_OP_INVERT")]
    #[doc = "Godot enumerator name: `LOGIC_OP_INVERT`"]
    pub const INVERT: LogicOperation = LogicOperation {
        ord: 10i32
    };
    #[doc(alias = "LOGIC_OP_OR_REVERSE")]
    #[doc = "Godot enumerator name: `LOGIC_OP_OR_REVERSE`"]
    pub const OR_REVERSE: LogicOperation = LogicOperation {
        ord: 11i32
    };
    #[doc(alias = "LOGIC_OP_COPY_INVERTED")]
    #[doc = "Godot enumerator name: `LOGIC_OP_COPY_INVERTED`"]
    pub const COPY_INVERTED: LogicOperation = LogicOperation {
        ord: 12i32
    };
    #[doc(alias = "LOGIC_OP_OR_INVERTED")]
    #[doc = "Godot enumerator name: `LOGIC_OP_OR_INVERTED`"]
    pub const OR_INVERTED: LogicOperation = LogicOperation {
        ord: 13i32
    };
    #[doc(alias = "LOGIC_OP_NAND")]
    #[doc = "Godot enumerator name: `LOGIC_OP_NAND`"]
    pub const NAND: LogicOperation = LogicOperation {
        ord: 14i32
    };
    #[doc(alias = "LOGIC_OP_SET")]
    #[doc = "Godot enumerator name: `LOGIC_OP_SET`"]
    pub const SET: LogicOperation = LogicOperation {
        ord: 15i32
    };
    #[doc(alias = "LOGIC_OP_MAX")]
    #[doc = "Godot enumerator name: `LOGIC_OP_MAX`"]
    pub const MAX: LogicOperation = LogicOperation {
        ord: 16i32
    };
    
}
impl std::fmt::Debug for LogicOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LogicOperation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LogicOperation {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 => Some(Self {
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
            Self::CLEAR => "CLEAR", Self::AND => "AND", Self::AND_REVERSE => "AND_REVERSE", Self::COPY => "COPY", Self::AND_INVERTED => "AND_INVERTED", Self::NO_OP => "NO_OP", Self::XOR => "XOR", Self::OR => "OR", Self::NOR => "NOR", Self::EQUIVALENT => "EQUIVALENT", Self::INVERT => "INVERT", Self::OR_REVERSE => "OR_REVERSE", Self::COPY_INVERTED => "COPY_INVERTED", Self::OR_INVERTED => "OR_INVERTED", Self::NAND => "NAND", Self::SET => "SET", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LogicOperation::CLEAR, LogicOperation::AND, LogicOperation::AND_REVERSE, LogicOperation::COPY, LogicOperation::AND_INVERTED, LogicOperation::NO_OP, LogicOperation::XOR, LogicOperation::OR, LogicOperation::NOR, LogicOperation::EQUIVALENT, LogicOperation::INVERT, LogicOperation::OR_REVERSE, LogicOperation::COPY_INVERTED, LogicOperation::OR_INVERTED, LogicOperation::NAND, LogicOperation::SET]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LogicOperation >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CLEAR", "LOGIC_OP_CLEAR", LogicOperation::CLEAR), crate::meta::inspect::EnumConstant::new("AND", "LOGIC_OP_AND", LogicOperation::AND), crate::meta::inspect::EnumConstant::new("AND_REVERSE", "LOGIC_OP_AND_REVERSE", LogicOperation::AND_REVERSE), crate::meta::inspect::EnumConstant::new("COPY", "LOGIC_OP_COPY", LogicOperation::COPY), crate::meta::inspect::EnumConstant::new("AND_INVERTED", "LOGIC_OP_AND_INVERTED", LogicOperation::AND_INVERTED), crate::meta::inspect::EnumConstant::new("NO_OP", "LOGIC_OP_NO_OP", LogicOperation::NO_OP), crate::meta::inspect::EnumConstant::new("XOR", "LOGIC_OP_XOR", LogicOperation::XOR), crate::meta::inspect::EnumConstant::new("OR", "LOGIC_OP_OR", LogicOperation::OR), crate::meta::inspect::EnumConstant::new("NOR", "LOGIC_OP_NOR", LogicOperation::NOR), crate::meta::inspect::EnumConstant::new("EQUIVALENT", "LOGIC_OP_EQUIVALENT", LogicOperation::EQUIVALENT), crate::meta::inspect::EnumConstant::new("INVERT", "LOGIC_OP_INVERT", LogicOperation::INVERT), crate::meta::inspect::EnumConstant::new("OR_REVERSE", "LOGIC_OP_OR_REVERSE", LogicOperation::OR_REVERSE), crate::meta::inspect::EnumConstant::new("COPY_INVERTED", "LOGIC_OP_COPY_INVERTED", LogicOperation::COPY_INVERTED), crate::meta::inspect::EnumConstant::new("OR_INVERTED", "LOGIC_OP_OR_INVERTED", LogicOperation::OR_INVERTED), crate::meta::inspect::EnumConstant::new("NAND", "LOGIC_OP_NAND", LogicOperation::NAND), crate::meta::inspect::EnumConstant::new("SET", "LOGIC_OP_SET", LogicOperation::SET), crate::meta::inspect::EnumConstant::new("MAX", "LOGIC_OP_MAX", LogicOperation::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for LogicOperation {
    const ENUMERATOR_COUNT: usize = 16usize;
    
}
impl crate::meta::GodotConvert for LogicOperation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LogicOperation {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LogicOperation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlendFactor {
    ord: i32
}
impl BlendFactor {
    #[doc(alias = "BLEND_FACTOR_ZERO")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ZERO`"]
    pub const ZERO: BlendFactor = BlendFactor {
        ord: 0i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE`"]
    pub const ONE: BlendFactor = BlendFactor {
        ord: 1i32
    };
    #[doc(alias = "BLEND_FACTOR_SRC_COLOR")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_SRC_COLOR`"]
    pub const SRC_COLOR: BlendFactor = BlendFactor {
        ord: 2i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE_MINUS_SRC_COLOR")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE_MINUS_SRC_COLOR`"]
    pub const ONE_MINUS_SRC_COLOR: BlendFactor = BlendFactor {
        ord: 3i32
    };
    #[doc(alias = "BLEND_FACTOR_DST_COLOR")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_DST_COLOR`"]
    pub const DST_COLOR: BlendFactor = BlendFactor {
        ord: 4i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE_MINUS_DST_COLOR")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE_MINUS_DST_COLOR`"]
    pub const ONE_MINUS_DST_COLOR: BlendFactor = BlendFactor {
        ord: 5i32
    };
    #[doc(alias = "BLEND_FACTOR_SRC_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_SRC_ALPHA`"]
    pub const SRC_ALPHA: BlendFactor = BlendFactor {
        ord: 6i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE_MINUS_SRC_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE_MINUS_SRC_ALPHA`"]
    pub const ONE_MINUS_SRC_ALPHA: BlendFactor = BlendFactor {
        ord: 7i32
    };
    #[doc(alias = "BLEND_FACTOR_DST_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_DST_ALPHA`"]
    pub const DST_ALPHA: BlendFactor = BlendFactor {
        ord: 8i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE_MINUS_DST_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE_MINUS_DST_ALPHA`"]
    pub const ONE_MINUS_DST_ALPHA: BlendFactor = BlendFactor {
        ord: 9i32
    };
    #[doc(alias = "BLEND_FACTOR_CONSTANT_COLOR")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_CONSTANT_COLOR`"]
    pub const CONSTANT_COLOR: BlendFactor = BlendFactor {
        ord: 10i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR`"]
    pub const ONE_MINUS_CONSTANT_COLOR: BlendFactor = BlendFactor {
        ord: 11i32
    };
    #[doc(alias = "BLEND_FACTOR_CONSTANT_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_CONSTANT_ALPHA`"]
    pub const CONSTANT_ALPHA: BlendFactor = BlendFactor {
        ord: 12i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA`"]
    pub const ONE_MINUS_CONSTANT_ALPHA: BlendFactor = BlendFactor {
        ord: 13i32
    };
    #[doc(alias = "BLEND_FACTOR_SRC_ALPHA_SATURATE")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_SRC_ALPHA_SATURATE`"]
    pub const SRC_ALPHA_SATURATE: BlendFactor = BlendFactor {
        ord: 14i32
    };
    #[doc(alias = "BLEND_FACTOR_SRC1_COLOR")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_SRC1_COLOR`"]
    pub const SRC1_COLOR: BlendFactor = BlendFactor {
        ord: 15i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE_MINUS_SRC1_COLOR")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE_MINUS_SRC1_COLOR`"]
    pub const ONE_MINUS_SRC1_COLOR: BlendFactor = BlendFactor {
        ord: 16i32
    };
    #[doc(alias = "BLEND_FACTOR_SRC1_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_SRC1_ALPHA`"]
    pub const SRC1_ALPHA: BlendFactor = BlendFactor {
        ord: 17i32
    };
    #[doc(alias = "BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA`"]
    pub const ONE_MINUS_SRC1_ALPHA: BlendFactor = BlendFactor {
        ord: 18i32
    };
    #[doc(alias = "BLEND_FACTOR_MAX")]
    #[doc = "Godot enumerator name: `BLEND_FACTOR_MAX`"]
    pub const MAX: BlendFactor = BlendFactor {
        ord: 19i32
    };
    
}
impl std::fmt::Debug for BlendFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BlendFactor") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BlendFactor {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 => Some(Self {
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
            Self::ZERO => "ZERO", Self::ONE => "ONE", Self::SRC_COLOR => "SRC_COLOR", Self::ONE_MINUS_SRC_COLOR => "ONE_MINUS_SRC_COLOR", Self::DST_COLOR => "DST_COLOR", Self::ONE_MINUS_DST_COLOR => "ONE_MINUS_DST_COLOR", Self::SRC_ALPHA => "SRC_ALPHA", Self::ONE_MINUS_SRC_ALPHA => "ONE_MINUS_SRC_ALPHA", Self::DST_ALPHA => "DST_ALPHA", Self::ONE_MINUS_DST_ALPHA => "ONE_MINUS_DST_ALPHA", Self::CONSTANT_COLOR => "CONSTANT_COLOR", Self::ONE_MINUS_CONSTANT_COLOR => "ONE_MINUS_CONSTANT_COLOR", Self::CONSTANT_ALPHA => "CONSTANT_ALPHA", Self::ONE_MINUS_CONSTANT_ALPHA => "ONE_MINUS_CONSTANT_ALPHA", Self::SRC_ALPHA_SATURATE => "SRC_ALPHA_SATURATE", Self::SRC1_COLOR => "SRC1_COLOR", Self::ONE_MINUS_SRC1_COLOR => "ONE_MINUS_SRC1_COLOR", Self::SRC1_ALPHA => "SRC1_ALPHA", Self::ONE_MINUS_SRC1_ALPHA => "ONE_MINUS_SRC1_ALPHA", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BlendFactor::ZERO, BlendFactor::ONE, BlendFactor::SRC_COLOR, BlendFactor::ONE_MINUS_SRC_COLOR, BlendFactor::DST_COLOR, BlendFactor::ONE_MINUS_DST_COLOR, BlendFactor::SRC_ALPHA, BlendFactor::ONE_MINUS_SRC_ALPHA, BlendFactor::DST_ALPHA, BlendFactor::ONE_MINUS_DST_ALPHA, BlendFactor::CONSTANT_COLOR, BlendFactor::ONE_MINUS_CONSTANT_COLOR, BlendFactor::CONSTANT_ALPHA, BlendFactor::ONE_MINUS_CONSTANT_ALPHA, BlendFactor::SRC_ALPHA_SATURATE, BlendFactor::SRC1_COLOR, BlendFactor::ONE_MINUS_SRC1_COLOR, BlendFactor::SRC1_ALPHA, BlendFactor::ONE_MINUS_SRC1_ALPHA]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BlendFactor >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ZERO", "BLEND_FACTOR_ZERO", BlendFactor::ZERO), crate::meta::inspect::EnumConstant::new("ONE", "BLEND_FACTOR_ONE", BlendFactor::ONE), crate::meta::inspect::EnumConstant::new("SRC_COLOR", "BLEND_FACTOR_SRC_COLOR", BlendFactor::SRC_COLOR), crate::meta::inspect::EnumConstant::new("ONE_MINUS_SRC_COLOR", "BLEND_FACTOR_ONE_MINUS_SRC_COLOR", BlendFactor::ONE_MINUS_SRC_COLOR), crate::meta::inspect::EnumConstant::new("DST_COLOR", "BLEND_FACTOR_DST_COLOR", BlendFactor::DST_COLOR), crate::meta::inspect::EnumConstant::new("ONE_MINUS_DST_COLOR", "BLEND_FACTOR_ONE_MINUS_DST_COLOR", BlendFactor::ONE_MINUS_DST_COLOR), crate::meta::inspect::EnumConstant::new("SRC_ALPHA", "BLEND_FACTOR_SRC_ALPHA", BlendFactor::SRC_ALPHA), crate::meta::inspect::EnumConstant::new("ONE_MINUS_SRC_ALPHA", "BLEND_FACTOR_ONE_MINUS_SRC_ALPHA", BlendFactor::ONE_MINUS_SRC_ALPHA), crate::meta::inspect::EnumConstant::new("DST_ALPHA", "BLEND_FACTOR_DST_ALPHA", BlendFactor::DST_ALPHA), crate::meta::inspect::EnumConstant::new("ONE_MINUS_DST_ALPHA", "BLEND_FACTOR_ONE_MINUS_DST_ALPHA", BlendFactor::ONE_MINUS_DST_ALPHA), crate::meta::inspect::EnumConstant::new("CONSTANT_COLOR", "BLEND_FACTOR_CONSTANT_COLOR", BlendFactor::CONSTANT_COLOR), crate::meta::inspect::EnumConstant::new("ONE_MINUS_CONSTANT_COLOR", "BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR", BlendFactor::ONE_MINUS_CONSTANT_COLOR), crate::meta::inspect::EnumConstant::new("CONSTANT_ALPHA", "BLEND_FACTOR_CONSTANT_ALPHA", BlendFactor::CONSTANT_ALPHA), crate::meta::inspect::EnumConstant::new("ONE_MINUS_CONSTANT_ALPHA", "BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA", BlendFactor::ONE_MINUS_CONSTANT_ALPHA), crate::meta::inspect::EnumConstant::new("SRC_ALPHA_SATURATE", "BLEND_FACTOR_SRC_ALPHA_SATURATE", BlendFactor::SRC_ALPHA_SATURATE), crate::meta::inspect::EnumConstant::new("SRC1_COLOR", "BLEND_FACTOR_SRC1_COLOR", BlendFactor::SRC1_COLOR), crate::meta::inspect::EnumConstant::new("ONE_MINUS_SRC1_COLOR", "BLEND_FACTOR_ONE_MINUS_SRC1_COLOR", BlendFactor::ONE_MINUS_SRC1_COLOR), crate::meta::inspect::EnumConstant::new("SRC1_ALPHA", "BLEND_FACTOR_SRC1_ALPHA", BlendFactor::SRC1_ALPHA), crate::meta::inspect::EnumConstant::new("ONE_MINUS_SRC1_ALPHA", "BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA", BlendFactor::ONE_MINUS_SRC1_ALPHA), crate::meta::inspect::EnumConstant::new("MAX", "BLEND_FACTOR_MAX", BlendFactor::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for BlendFactor {
    const ENUMERATOR_COUNT: usize = 19usize;
    
}
impl crate::meta::GodotConvert for BlendFactor {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BlendFactor {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BlendFactor {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlendOperation {
    ord: i32
}
impl BlendOperation {
    #[doc(alias = "BLEND_OP_ADD")]
    #[doc = "Godot enumerator name: `BLEND_OP_ADD`"]
    pub const ADD: BlendOperation = BlendOperation {
        ord: 0i32
    };
    #[doc(alias = "BLEND_OP_SUBTRACT")]
    #[doc = "Godot enumerator name: `BLEND_OP_SUBTRACT`"]
    pub const SUBTRACT: BlendOperation = BlendOperation {
        ord: 1i32
    };
    #[doc(alias = "BLEND_OP_REVERSE_SUBTRACT")]
    #[doc = "Godot enumerator name: `BLEND_OP_REVERSE_SUBTRACT`"]
    pub const REVERSE_SUBTRACT: BlendOperation = BlendOperation {
        ord: 2i32
    };
    #[doc(alias = "BLEND_OP_MINIMUM")]
    #[doc = "Godot enumerator name: `BLEND_OP_MINIMUM`"]
    pub const MINIMUM: BlendOperation = BlendOperation {
        ord: 3i32
    };
    #[doc(alias = "BLEND_OP_MAXIMUM")]
    #[doc = "Godot enumerator name: `BLEND_OP_MAXIMUM`"]
    pub const MAXIMUM: BlendOperation = BlendOperation {
        ord: 4i32
    };
    #[doc(alias = "BLEND_OP_MAX")]
    #[doc = "Godot enumerator name: `BLEND_OP_MAX`"]
    pub const MAX: BlendOperation = BlendOperation {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for BlendOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BlendOperation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BlendOperation {
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
            Self::ADD => "ADD", Self::SUBTRACT => "SUBTRACT", Self::REVERSE_SUBTRACT => "REVERSE_SUBTRACT", Self::MINIMUM => "MINIMUM", Self::MAXIMUM => "MAXIMUM", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BlendOperation::ADD, BlendOperation::SUBTRACT, BlendOperation::REVERSE_SUBTRACT, BlendOperation::MINIMUM, BlendOperation::MAXIMUM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BlendOperation >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ADD", "BLEND_OP_ADD", BlendOperation::ADD), crate::meta::inspect::EnumConstant::new("SUBTRACT", "BLEND_OP_SUBTRACT", BlendOperation::SUBTRACT), crate::meta::inspect::EnumConstant::new("REVERSE_SUBTRACT", "BLEND_OP_REVERSE_SUBTRACT", BlendOperation::REVERSE_SUBTRACT), crate::meta::inspect::EnumConstant::new("MINIMUM", "BLEND_OP_MINIMUM", BlendOperation::MINIMUM), crate::meta::inspect::EnumConstant::new("MAXIMUM", "BLEND_OP_MAXIMUM", BlendOperation::MAXIMUM), crate::meta::inspect::EnumConstant::new("MAX", "BLEND_OP_MAX", BlendOperation::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for BlendOperation {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for BlendOperation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BlendOperation {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BlendOperation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct PipelineDynamicStateFlags {
    ord: u64
}
impl PipelineDynamicStateFlags {
    #[doc(alias = "DYNAMIC_STATE_LINE_WIDTH")]
    #[doc = "Godot enumerator name: `DYNAMIC_STATE_LINE_WIDTH`"]
    pub const LINE_WIDTH: PipelineDynamicStateFlags = PipelineDynamicStateFlags {
        ord: 1u64
    };
    #[doc(alias = "DYNAMIC_STATE_DEPTH_BIAS")]
    #[doc = "Godot enumerator name: `DYNAMIC_STATE_DEPTH_BIAS`"]
    pub const DEPTH_BIAS: PipelineDynamicStateFlags = PipelineDynamicStateFlags {
        ord: 2u64
    };
    #[doc(alias = "DYNAMIC_STATE_BLEND_CONSTANTS")]
    #[doc = "Godot enumerator name: `DYNAMIC_STATE_BLEND_CONSTANTS`"]
    pub const BLEND_CONSTANTS: PipelineDynamicStateFlags = PipelineDynamicStateFlags {
        ord: 4u64
    };
    #[doc(alias = "DYNAMIC_STATE_DEPTH_BOUNDS")]
    #[doc = "Godot enumerator name: `DYNAMIC_STATE_DEPTH_BOUNDS`"]
    pub const DEPTH_BOUNDS: PipelineDynamicStateFlags = PipelineDynamicStateFlags {
        ord: 8u64
    };
    #[doc(alias = "DYNAMIC_STATE_STENCIL_COMPARE_MASK")]
    #[doc = "Godot enumerator name: `DYNAMIC_STATE_STENCIL_COMPARE_MASK`"]
    pub const STENCIL_COMPARE_MASK: PipelineDynamicStateFlags = PipelineDynamicStateFlags {
        ord: 16u64
    };
    #[doc(alias = "DYNAMIC_STATE_STENCIL_WRITE_MASK")]
    #[doc = "Godot enumerator name: `DYNAMIC_STATE_STENCIL_WRITE_MASK`"]
    pub const STENCIL_WRITE_MASK: PipelineDynamicStateFlags = PipelineDynamicStateFlags {
        ord: 32u64
    };
    #[doc(alias = "DYNAMIC_STATE_STENCIL_REFERENCE")]
    #[doc = "Godot enumerator name: `DYNAMIC_STATE_STENCIL_REFERENCE`"]
    pub const STENCIL_REFERENCE: PipelineDynamicStateFlags = PipelineDynamicStateFlags {
        ord: 64u64
    };
    
}
impl std::fmt::Debug for PipelineDynamicStateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::LINE_WIDTH => "LINE_WIDTH", Self::DEPTH_BIAS => "DEPTH_BIAS", Self::BLEND_CONSTANTS => "BLEND_CONSTANTS", Self::DEPTH_BOUNDS => "DEPTH_BOUNDS", Self::STENCIL_COMPARE_MASK => "STENCIL_COMPARE_MASK", Self::STENCIL_WRITE_MASK => "STENCIL_WRITE_MASK", Self::STENCIL_REFERENCE => "STENCIL_REFERENCE", _ => {
                f.debug_struct("PipelineDynamicStateFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for PipelineDynamicStateFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PipelineDynamicStateFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LINE_WIDTH", "DYNAMIC_STATE_LINE_WIDTH", PipelineDynamicStateFlags::LINE_WIDTH), crate::meta::inspect::EnumConstant::new("DEPTH_BIAS", "DYNAMIC_STATE_DEPTH_BIAS", PipelineDynamicStateFlags::DEPTH_BIAS), crate::meta::inspect::EnumConstant::new("BLEND_CONSTANTS", "DYNAMIC_STATE_BLEND_CONSTANTS", PipelineDynamicStateFlags::BLEND_CONSTANTS), crate::meta::inspect::EnumConstant::new("DEPTH_BOUNDS", "DYNAMIC_STATE_DEPTH_BOUNDS", PipelineDynamicStateFlags::DEPTH_BOUNDS), crate::meta::inspect::EnumConstant::new("STENCIL_COMPARE_MASK", "DYNAMIC_STATE_STENCIL_COMPARE_MASK", PipelineDynamicStateFlags::STENCIL_COMPARE_MASK), crate::meta::inspect::EnumConstant::new("STENCIL_WRITE_MASK", "DYNAMIC_STATE_STENCIL_WRITE_MASK", PipelineDynamicStateFlags::STENCIL_WRITE_MASK), crate::meta::inspect::EnumConstant::new("STENCIL_REFERENCE", "DYNAMIC_STATE_STENCIL_REFERENCE", PipelineDynamicStateFlags::STENCIL_REFERENCE)]
        }
    }
}
impl std::ops::BitOr for PipelineDynamicStateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for PipelineDynamicStateFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for PipelineDynamicStateFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for PipelineDynamicStateFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PipelineDynamicStateFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct InitialAction {
    ord: i32
}
impl InitialAction {
    #[doc(alias = "INITIAL_ACTION_LOAD")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_LOAD`"]
    pub const LOAD: InitialAction = InitialAction {
        ord: 0i32
    };
    #[doc(alias = "INITIAL_ACTION_CLEAR")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_CLEAR`"]
    pub const CLEAR: InitialAction = InitialAction {
        ord: 1i32
    };
    #[doc(alias = "INITIAL_ACTION_DISCARD")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_DISCARD`"]
    pub const DISCARD: InitialAction = InitialAction {
        ord: 2i32
    };
    #[doc(alias = "INITIAL_ACTION_MAX")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_MAX`"]
    pub const MAX: InitialAction = InitialAction {
        ord: 3i32
    };
    #[doc(alias = "INITIAL_ACTION_CLEAR_REGION")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_CLEAR_REGION`"]
    pub const CLEAR_REGION: InitialAction = InitialAction {
        ord: 1i32
    };
    #[doc(alias = "INITIAL_ACTION_CLEAR_REGION_CONTINUE")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_CLEAR_REGION_CONTINUE`"]
    pub const CLEAR_REGION_CONTINUE: InitialAction = InitialAction {
        ord: 1i32
    };
    #[doc(alias = "INITIAL_ACTION_KEEP")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_KEEP`"]
    pub const KEEP: InitialAction = InitialAction {
        ord: 0i32
    };
    #[doc(alias = "INITIAL_ACTION_DROP")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_DROP`"]
    pub const DROP: InitialAction = InitialAction {
        ord: 2i32
    };
    #[doc(alias = "INITIAL_ACTION_CONTINUE")]
    #[doc = "Godot enumerator name: `INITIAL_ACTION_CONTINUE`"]
    pub const CONTINUE: InitialAction = InitialAction {
        ord: 0i32
    };
    
}
impl std::fmt::Debug for InitialAction {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("InitialAction") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for InitialAction {
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
            Self::LOAD => "LOAD", Self::CLEAR => "CLEAR", Self::DISCARD => "DISCARD", Self::MAX => "MAX", Self::CLEAR_REGION => "CLEAR_REGION", Self::CLEAR_REGION_CONTINUE => "CLEAR_REGION_CONTINUE", Self::KEEP => "KEEP", Self::DROP => "DROP", Self::CONTINUE => "CONTINUE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[InitialAction::LOAD, InitialAction::CLEAR, InitialAction::DISCARD]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < InitialAction >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LOAD", "INITIAL_ACTION_LOAD", InitialAction::LOAD), crate::meta::inspect::EnumConstant::new("CLEAR", "INITIAL_ACTION_CLEAR", InitialAction::CLEAR), crate::meta::inspect::EnumConstant::new("DISCARD", "INITIAL_ACTION_DISCARD", InitialAction::DISCARD), crate::meta::inspect::EnumConstant::new("MAX", "INITIAL_ACTION_MAX", InitialAction::MAX), crate::meta::inspect::EnumConstant::new("CLEAR_REGION", "INITIAL_ACTION_CLEAR_REGION", InitialAction::CLEAR_REGION), crate::meta::inspect::EnumConstant::new("CLEAR_REGION_CONTINUE", "INITIAL_ACTION_CLEAR_REGION_CONTINUE", InitialAction::CLEAR_REGION_CONTINUE), crate::meta::inspect::EnumConstant::new("KEEP", "INITIAL_ACTION_KEEP", InitialAction::KEEP), crate::meta::inspect::EnumConstant::new("DROP", "INITIAL_ACTION_DROP", InitialAction::DROP), crate::meta::inspect::EnumConstant::new("CONTINUE", "INITIAL_ACTION_CONTINUE", InitialAction::CONTINUE)]
        }
    }
}
impl crate::obj::IndexEnum for InitialAction {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for InitialAction {
    type Via = i32;
    
}
impl crate::meta::ToGodot for InitialAction {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for InitialAction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FinalAction {
    ord: i32
}
impl FinalAction {
    #[doc(alias = "FINAL_ACTION_STORE")]
    #[doc = "Godot enumerator name: `FINAL_ACTION_STORE`"]
    pub const STORE: FinalAction = FinalAction {
        ord: 0i32
    };
    #[doc(alias = "FINAL_ACTION_DISCARD")]
    #[doc = "Godot enumerator name: `FINAL_ACTION_DISCARD`"]
    pub const DISCARD: FinalAction = FinalAction {
        ord: 1i32
    };
    #[doc(alias = "FINAL_ACTION_MAX")]
    #[doc = "Godot enumerator name: `FINAL_ACTION_MAX`"]
    pub const MAX: FinalAction = FinalAction {
        ord: 2i32
    };
    #[doc(alias = "FINAL_ACTION_READ")]
    #[doc = "Godot enumerator name: `FINAL_ACTION_READ`"]
    pub const READ: FinalAction = FinalAction {
        ord: 0i32
    };
    #[doc(alias = "FINAL_ACTION_CONTINUE")]
    #[doc = "Godot enumerator name: `FINAL_ACTION_CONTINUE`"]
    pub const CONTINUE: FinalAction = FinalAction {
        ord: 0i32
    };
    
}
impl std::fmt::Debug for FinalAction {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FinalAction") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FinalAction {
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
            Self::STORE => "STORE", Self::DISCARD => "DISCARD", Self::MAX => "MAX", Self::READ => "READ", Self::CONTINUE => "CONTINUE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FinalAction::STORE, FinalAction::DISCARD]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FinalAction >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STORE", "FINAL_ACTION_STORE", FinalAction::STORE), crate::meta::inspect::EnumConstant::new("DISCARD", "FINAL_ACTION_DISCARD", FinalAction::DISCARD), crate::meta::inspect::EnumConstant::new("MAX", "FINAL_ACTION_MAX", FinalAction::MAX), crate::meta::inspect::EnumConstant::new("READ", "FINAL_ACTION_READ", FinalAction::READ), crate::meta::inspect::EnumConstant::new("CONTINUE", "FINAL_ACTION_CONTINUE", FinalAction::CONTINUE)]
        }
    }
}
impl crate::obj::IndexEnum for FinalAction {
    const ENUMERATOR_COUNT: usize = 2usize;
    
}
impl crate::meta::GodotConvert for FinalAction {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FinalAction {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FinalAction {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShaderStage {
    ord: i32
}
impl ShaderStage {
    #[doc(alias = "SHADER_STAGE_VERTEX")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_VERTEX`"]
    pub const VERTEX: ShaderStage = ShaderStage {
        ord: 0i32
    };
    #[doc(alias = "SHADER_STAGE_FRAGMENT")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_FRAGMENT`"]
    pub const FRAGMENT: ShaderStage = ShaderStage {
        ord: 1i32
    };
    #[doc(alias = "SHADER_STAGE_TESSELATION_CONTROL")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_TESSELATION_CONTROL`"]
    pub const TESSELATION_CONTROL: ShaderStage = ShaderStage {
        ord: 2i32
    };
    #[doc(alias = "SHADER_STAGE_TESSELATION_EVALUATION")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_TESSELATION_EVALUATION`"]
    pub const TESSELATION_EVALUATION: ShaderStage = ShaderStage {
        ord: 3i32
    };
    #[doc(alias = "SHADER_STAGE_COMPUTE")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_COMPUTE`"]
    pub const COMPUTE: ShaderStage = ShaderStage {
        ord: 4i32
    };
    #[doc(alias = "SHADER_STAGE_MAX")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_MAX`"]
    pub const MAX: ShaderStage = ShaderStage {
        ord: 5i32
    };
    #[doc(alias = "SHADER_STAGE_VERTEX_BIT")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_VERTEX_BIT`"]
    pub const VERTEX_BIT: ShaderStage = ShaderStage {
        ord: 1i32
    };
    #[doc(alias = "SHADER_STAGE_FRAGMENT_BIT")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_FRAGMENT_BIT`"]
    pub const FRAGMENT_BIT: ShaderStage = ShaderStage {
        ord: 2i32
    };
    #[doc(alias = "SHADER_STAGE_TESSELATION_CONTROL_BIT")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_TESSELATION_CONTROL_BIT`"]
    pub const TESSELATION_CONTROL_BIT: ShaderStage = ShaderStage {
        ord: 4i32
    };
    #[doc(alias = "SHADER_STAGE_TESSELATION_EVALUATION_BIT")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_TESSELATION_EVALUATION_BIT`"]
    pub const TESSELATION_EVALUATION_BIT: ShaderStage = ShaderStage {
        ord: 8i32
    };
    #[doc(alias = "SHADER_STAGE_COMPUTE_BIT")]
    #[doc = "Godot enumerator name: `SHADER_STAGE_COMPUTE_BIT`"]
    pub const COMPUTE_BIT: ShaderStage = ShaderStage {
        ord: 16i32
    };
    
}
impl std::fmt::Debug for ShaderStage {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShaderStage") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShaderStage {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 8i32 | ord @ 16i32 => Some(Self {
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
            Self::VERTEX => "VERTEX", Self::FRAGMENT => "FRAGMENT", Self::TESSELATION_CONTROL => "TESSELATION_CONTROL", Self::TESSELATION_EVALUATION => "TESSELATION_EVALUATION", Self::COMPUTE => "COMPUTE", Self::MAX => "MAX", Self::VERTEX_BIT => "VERTEX_BIT", Self::FRAGMENT_BIT => "FRAGMENT_BIT", Self::TESSELATION_CONTROL_BIT => "TESSELATION_CONTROL_BIT", Self::TESSELATION_EVALUATION_BIT => "TESSELATION_EVALUATION_BIT", Self::COMPUTE_BIT => "COMPUTE_BIT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ShaderStage::VERTEX, ShaderStage::FRAGMENT, ShaderStage::TESSELATION_CONTROL, ShaderStage::TESSELATION_EVALUATION, ShaderStage::COMPUTE, ShaderStage::MAX, ShaderStage::TESSELATION_EVALUATION_BIT, ShaderStage::COMPUTE_BIT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ShaderStage >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERTEX", "SHADER_STAGE_VERTEX", ShaderStage::VERTEX), crate::meta::inspect::EnumConstant::new("FRAGMENT", "SHADER_STAGE_FRAGMENT", ShaderStage::FRAGMENT), crate::meta::inspect::EnumConstant::new("TESSELATION_CONTROL", "SHADER_STAGE_TESSELATION_CONTROL", ShaderStage::TESSELATION_CONTROL), crate::meta::inspect::EnumConstant::new("TESSELATION_EVALUATION", "SHADER_STAGE_TESSELATION_EVALUATION", ShaderStage::TESSELATION_EVALUATION), crate::meta::inspect::EnumConstant::new("COMPUTE", "SHADER_STAGE_COMPUTE", ShaderStage::COMPUTE), crate::meta::inspect::EnumConstant::new("MAX", "SHADER_STAGE_MAX", ShaderStage::MAX), crate::meta::inspect::EnumConstant::new("VERTEX_BIT", "SHADER_STAGE_VERTEX_BIT", ShaderStage::VERTEX_BIT), crate::meta::inspect::EnumConstant::new("FRAGMENT_BIT", "SHADER_STAGE_FRAGMENT_BIT", ShaderStage::FRAGMENT_BIT), crate::meta::inspect::EnumConstant::new("TESSELATION_CONTROL_BIT", "SHADER_STAGE_TESSELATION_CONTROL_BIT", ShaderStage::TESSELATION_CONTROL_BIT), crate::meta::inspect::EnumConstant::new("TESSELATION_EVALUATION_BIT", "SHADER_STAGE_TESSELATION_EVALUATION_BIT", ShaderStage::TESSELATION_EVALUATION_BIT), crate::meta::inspect::EnumConstant::new("COMPUTE_BIT", "SHADER_STAGE_COMPUTE_BIT", ShaderStage::COMPUTE_BIT)]
        }
    }
}
impl crate::meta::GodotConvert for ShaderStage {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ShaderStage {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShaderStage {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShaderLanguage {
    ord: i32
}
impl ShaderLanguage {
    #[doc(alias = "SHADER_LANGUAGE_GLSL")]
    #[doc = "Godot enumerator name: `SHADER_LANGUAGE_GLSL`"]
    pub const GLSL: ShaderLanguage = ShaderLanguage {
        ord: 0i32
    };
    #[doc(alias = "SHADER_LANGUAGE_HLSL")]
    #[doc = "Godot enumerator name: `SHADER_LANGUAGE_HLSL`"]
    pub const HLSL: ShaderLanguage = ShaderLanguage {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for ShaderLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShaderLanguage") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShaderLanguage {
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
            Self::GLSL => "GLSL", Self::HLSL => "HLSL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ShaderLanguage::GLSL, ShaderLanguage::HLSL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ShaderLanguage >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("GLSL", "SHADER_LANGUAGE_GLSL", ShaderLanguage::GLSL), crate::meta::inspect::EnumConstant::new("HLSL", "SHADER_LANGUAGE_HLSL", ShaderLanguage::HLSL)]
        }
    }
}
impl crate::meta::GodotConvert for ShaderLanguage {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ShaderLanguage {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShaderLanguage {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PipelineSpecializationConstantType {
    ord: i32
}
impl PipelineSpecializationConstantType {
    #[doc(alias = "PIPELINE_SPECIALIZATION_CONSTANT_TYPE_BOOL")]
    #[doc = "Godot enumerator name: `PIPELINE_SPECIALIZATION_CONSTANT_TYPE_BOOL`"]
    pub const BOOL: PipelineSpecializationConstantType = PipelineSpecializationConstantType {
        ord: 0i32
    };
    #[doc(alias = "PIPELINE_SPECIALIZATION_CONSTANT_TYPE_INT")]
    #[doc = "Godot enumerator name: `PIPELINE_SPECIALIZATION_CONSTANT_TYPE_INT`"]
    pub const INT: PipelineSpecializationConstantType = PipelineSpecializationConstantType {
        ord: 1i32
    };
    #[doc(alias = "PIPELINE_SPECIALIZATION_CONSTANT_TYPE_FLOAT")]
    #[doc = "Godot enumerator name: `PIPELINE_SPECIALIZATION_CONSTANT_TYPE_FLOAT`"]
    pub const FLOAT: PipelineSpecializationConstantType = PipelineSpecializationConstantType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PipelineSpecializationConstantType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PipelineSpecializationConstantType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PipelineSpecializationConstantType {
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
            Self::BOOL => "BOOL", Self::INT => "INT", Self::FLOAT => "FLOAT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PipelineSpecializationConstantType::BOOL, PipelineSpecializationConstantType::INT, PipelineSpecializationConstantType::FLOAT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PipelineSpecializationConstantType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BOOL", "PIPELINE_SPECIALIZATION_CONSTANT_TYPE_BOOL", PipelineSpecializationConstantType::BOOL), crate::meta::inspect::EnumConstant::new("INT", "PIPELINE_SPECIALIZATION_CONSTANT_TYPE_INT", PipelineSpecializationConstantType::INT), crate::meta::inspect::EnumConstant::new("FLOAT", "PIPELINE_SPECIALIZATION_CONSTANT_TYPE_FLOAT", PipelineSpecializationConstantType::FLOAT)]
        }
    }
}
impl crate::meta::GodotConvert for PipelineSpecializationConstantType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PipelineSpecializationConstantType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PipelineSpecializationConstantType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Features {
    ord: i32
}
impl Features {
    #[doc(alias = "SUPPORTS_METALFX_SPATIAL")]
    #[doc = "Godot enumerator name: `SUPPORTS_METALFX_SPATIAL`"]
    pub const METALFX_SPATIAL: Features = Features {
        ord: 3i32
    };
    #[doc(alias = "SUPPORTS_METALFX_TEMPORAL")]
    #[doc = "Godot enumerator name: `SUPPORTS_METALFX_TEMPORAL`"]
    pub const METALFX_TEMPORAL: Features = Features {
        ord: 4i32
    };
    #[doc(alias = "SUPPORTS_BUFFER_DEVICE_ADDRESS")]
    #[doc = "Godot enumerator name: `SUPPORTS_BUFFER_DEVICE_ADDRESS`"]
    pub const BUFFER_DEVICE_ADDRESS: Features = Features {
        ord: 6i32
    };
    #[doc(alias = "SUPPORTS_IMAGE_ATOMIC_32_BIT")]
    #[doc = "Godot enumerator name: `SUPPORTS_IMAGE_ATOMIC_32_BIT`"]
    pub const IMAGE_ATOMIC_32_BIT: Features = Features {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for Features {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Features") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Features {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 3i32 | ord @ 4i32 | ord @ 6i32 | ord @ 7i32 => Some(Self {
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
            Self::METALFX_SPATIAL => "METALFX_SPATIAL", Self::METALFX_TEMPORAL => "METALFX_TEMPORAL", Self::BUFFER_DEVICE_ADDRESS => "BUFFER_DEVICE_ADDRESS", Self::IMAGE_ATOMIC_32_BIT => "IMAGE_ATOMIC_32_BIT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Features::METALFX_SPATIAL, Features::METALFX_TEMPORAL, Features::BUFFER_DEVICE_ADDRESS, Features::IMAGE_ATOMIC_32_BIT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Features >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("METALFX_SPATIAL", "SUPPORTS_METALFX_SPATIAL", Features::METALFX_SPATIAL), crate::meta::inspect::EnumConstant::new("METALFX_TEMPORAL", "SUPPORTS_METALFX_TEMPORAL", Features::METALFX_TEMPORAL), crate::meta::inspect::EnumConstant::new("BUFFER_DEVICE_ADDRESS", "SUPPORTS_BUFFER_DEVICE_ADDRESS", Features::BUFFER_DEVICE_ADDRESS), crate::meta::inspect::EnumConstant::new("IMAGE_ATOMIC_32_BIT", "SUPPORTS_IMAGE_ATOMIC_32_BIT", Features::IMAGE_ATOMIC_32_BIT)]
        }
    }
}
impl crate::meta::GodotConvert for Features {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Features {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Features {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Limit {
    ord: i32
}
impl Limit {
    #[doc(alias = "LIMIT_MAX_BOUND_UNIFORM_SETS")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_BOUND_UNIFORM_SETS`"]
    pub const MAX_BOUND_UNIFORM_SETS: Limit = Limit {
        ord: 0i32
    };
    #[doc(alias = "LIMIT_MAX_FRAMEBUFFER_COLOR_ATTACHMENTS")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_FRAMEBUFFER_COLOR_ATTACHMENTS`"]
    pub const MAX_FRAMEBUFFER_COLOR_ATTACHMENTS: Limit = Limit {
        ord: 1i32
    };
    #[doc(alias = "LIMIT_MAX_TEXTURES_PER_UNIFORM_SET")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_TEXTURES_PER_UNIFORM_SET`"]
    pub const MAX_TEXTURES_PER_UNIFORM_SET: Limit = Limit {
        ord: 2i32
    };
    #[doc(alias = "LIMIT_MAX_SAMPLERS_PER_UNIFORM_SET")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_SAMPLERS_PER_UNIFORM_SET`"]
    pub const MAX_SAMPLERS_PER_UNIFORM_SET: Limit = Limit {
        ord: 3i32
    };
    #[doc(alias = "LIMIT_MAX_STORAGE_BUFFERS_PER_UNIFORM_SET")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_STORAGE_BUFFERS_PER_UNIFORM_SET`"]
    pub const MAX_STORAGE_BUFFERS_PER_UNIFORM_SET: Limit = Limit {
        ord: 4i32
    };
    #[doc(alias = "LIMIT_MAX_STORAGE_IMAGES_PER_UNIFORM_SET")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_STORAGE_IMAGES_PER_UNIFORM_SET`"]
    pub const MAX_STORAGE_IMAGES_PER_UNIFORM_SET: Limit = Limit {
        ord: 5i32
    };
    #[doc(alias = "LIMIT_MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET`"]
    pub const MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET: Limit = Limit {
        ord: 6i32
    };
    #[doc(alias = "LIMIT_MAX_DRAW_INDEXED_INDEX")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_DRAW_INDEXED_INDEX`"]
    pub const MAX_DRAW_INDEXED_INDEX: Limit = Limit {
        ord: 7i32
    };
    #[doc(alias = "LIMIT_MAX_FRAMEBUFFER_HEIGHT")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_FRAMEBUFFER_HEIGHT`"]
    pub const MAX_FRAMEBUFFER_HEIGHT: Limit = Limit {
        ord: 8i32
    };
    #[doc(alias = "LIMIT_MAX_FRAMEBUFFER_WIDTH")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_FRAMEBUFFER_WIDTH`"]
    pub const MAX_FRAMEBUFFER_WIDTH: Limit = Limit {
        ord: 9i32
    };
    #[doc(alias = "LIMIT_MAX_TEXTURE_ARRAY_LAYERS")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_TEXTURE_ARRAY_LAYERS`"]
    pub const MAX_TEXTURE_ARRAY_LAYERS: Limit = Limit {
        ord: 10i32
    };
    #[doc(alias = "LIMIT_MAX_TEXTURE_SIZE_1D")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_TEXTURE_SIZE_1D`"]
    pub const MAX_TEXTURE_SIZE_1D: Limit = Limit {
        ord: 11i32
    };
    #[doc(alias = "LIMIT_MAX_TEXTURE_SIZE_2D")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_TEXTURE_SIZE_2D`"]
    pub const MAX_TEXTURE_SIZE_2D: Limit = Limit {
        ord: 12i32
    };
    #[doc(alias = "LIMIT_MAX_TEXTURE_SIZE_3D")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_TEXTURE_SIZE_3D`"]
    pub const MAX_TEXTURE_SIZE_3D: Limit = Limit {
        ord: 13i32
    };
    #[doc(alias = "LIMIT_MAX_TEXTURE_SIZE_CUBE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_TEXTURE_SIZE_CUBE`"]
    pub const MAX_TEXTURE_SIZE_CUBE: Limit = Limit {
        ord: 14i32
    };
    #[doc(alias = "LIMIT_MAX_TEXTURES_PER_SHADER_STAGE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_TEXTURES_PER_SHADER_STAGE`"]
    pub const MAX_TEXTURES_PER_SHADER_STAGE: Limit = Limit {
        ord: 15i32
    };
    #[doc(alias = "LIMIT_MAX_SAMPLERS_PER_SHADER_STAGE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_SAMPLERS_PER_SHADER_STAGE`"]
    pub const MAX_SAMPLERS_PER_SHADER_STAGE: Limit = Limit {
        ord: 16i32
    };
    #[doc(alias = "LIMIT_MAX_STORAGE_BUFFERS_PER_SHADER_STAGE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_STORAGE_BUFFERS_PER_SHADER_STAGE`"]
    pub const MAX_STORAGE_BUFFERS_PER_SHADER_STAGE: Limit = Limit {
        ord: 17i32
    };
    #[doc(alias = "LIMIT_MAX_STORAGE_IMAGES_PER_SHADER_STAGE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_STORAGE_IMAGES_PER_SHADER_STAGE`"]
    pub const MAX_STORAGE_IMAGES_PER_SHADER_STAGE: Limit = Limit {
        ord: 18i32
    };
    #[doc(alias = "LIMIT_MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE`"]
    pub const MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE: Limit = Limit {
        ord: 19i32
    };
    #[doc(alias = "LIMIT_MAX_PUSH_CONSTANT_SIZE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_PUSH_CONSTANT_SIZE`"]
    pub const MAX_PUSH_CONSTANT_SIZE: Limit = Limit {
        ord: 20i32
    };
    #[doc(alias = "LIMIT_MAX_UNIFORM_BUFFER_SIZE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_UNIFORM_BUFFER_SIZE`"]
    pub const MAX_UNIFORM_BUFFER_SIZE: Limit = Limit {
        ord: 21i32
    };
    #[doc(alias = "LIMIT_MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET`"]
    pub const MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET: Limit = Limit {
        ord: 22i32
    };
    #[doc(alias = "LIMIT_MAX_VERTEX_INPUT_ATTRIBUTES")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_VERTEX_INPUT_ATTRIBUTES`"]
    pub const MAX_VERTEX_INPUT_ATTRIBUTES: Limit = Limit {
        ord: 23i32
    };
    #[doc(alias = "LIMIT_MAX_VERTEX_INPUT_BINDINGS")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_VERTEX_INPUT_BINDINGS`"]
    pub const MAX_VERTEX_INPUT_BINDINGS: Limit = Limit {
        ord: 24i32
    };
    #[doc(alias = "LIMIT_MAX_VERTEX_INPUT_BINDING_STRIDE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_VERTEX_INPUT_BINDING_STRIDE`"]
    pub const MAX_VERTEX_INPUT_BINDING_STRIDE: Limit = Limit {
        ord: 25i32
    };
    #[doc(alias = "LIMIT_MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT")]
    #[doc = "Godot enumerator name: `LIMIT_MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT`"]
    pub const MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT: Limit = Limit {
        ord: 26i32
    };
    #[doc(alias = "LIMIT_MAX_COMPUTE_SHARED_MEMORY_SIZE")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_COMPUTE_SHARED_MEMORY_SIZE`"]
    pub const MAX_COMPUTE_SHARED_MEMORY_SIZE: Limit = Limit {
        ord: 27i32
    };
    #[doc(alias = "LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_X")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_X`"]
    pub const MAX_COMPUTE_WORKGROUP_COUNT_X: Limit = Limit {
        ord: 28i32
    };
    #[doc(alias = "LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_Y")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_Y`"]
    pub const MAX_COMPUTE_WORKGROUP_COUNT_Y: Limit = Limit {
        ord: 29i32
    };
    #[doc(alias = "LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_Z")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_Z`"]
    pub const MAX_COMPUTE_WORKGROUP_COUNT_Z: Limit = Limit {
        ord: 30i32
    };
    #[doc(alias = "LIMIT_MAX_COMPUTE_WORKGROUP_INVOCATIONS")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_COMPUTE_WORKGROUP_INVOCATIONS`"]
    pub const MAX_COMPUTE_WORKGROUP_INVOCATIONS: Limit = Limit {
        ord: 31i32
    };
    #[doc(alias = "LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_X")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_X`"]
    pub const MAX_COMPUTE_WORKGROUP_SIZE_X: Limit = Limit {
        ord: 32i32
    };
    #[doc(alias = "LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_Y")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_Y`"]
    pub const MAX_COMPUTE_WORKGROUP_SIZE_Y: Limit = Limit {
        ord: 33i32
    };
    #[doc(alias = "LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_Z")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_Z`"]
    pub const MAX_COMPUTE_WORKGROUP_SIZE_Z: Limit = Limit {
        ord: 34i32
    };
    #[doc(alias = "LIMIT_MAX_VIEWPORT_DIMENSIONS_X")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_VIEWPORT_DIMENSIONS_X`"]
    pub const MAX_VIEWPORT_DIMENSIONS_X: Limit = Limit {
        ord: 35i32
    };
    #[doc(alias = "LIMIT_MAX_VIEWPORT_DIMENSIONS_Y")]
    #[doc = "Godot enumerator name: `LIMIT_MAX_VIEWPORT_DIMENSIONS_Y`"]
    pub const MAX_VIEWPORT_DIMENSIONS_Y: Limit = Limit {
        ord: 36i32
    };
    #[doc(alias = "LIMIT_METALFX_TEMPORAL_SCALER_MIN_SCALE")]
    #[doc = "Godot enumerator name: `LIMIT_METALFX_TEMPORAL_SCALER_MIN_SCALE`"]
    pub const METALFX_TEMPORAL_SCALER_MIN_SCALE: Limit = Limit {
        ord: 46i32
    };
    #[doc(alias = "LIMIT_METALFX_TEMPORAL_SCALER_MAX_SCALE")]
    #[doc = "Godot enumerator name: `LIMIT_METALFX_TEMPORAL_SCALER_MAX_SCALE`"]
    pub const METALFX_TEMPORAL_SCALER_MAX_SCALE: Limit = Limit {
        ord: 47i32
    };
    
}
impl std::fmt::Debug for Limit {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Limit") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Limit {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 46i32 | ord @ 47i32 => Some(Self {
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
            Self::MAX_BOUND_UNIFORM_SETS => "MAX_BOUND_UNIFORM_SETS", Self::MAX_FRAMEBUFFER_COLOR_ATTACHMENTS => "MAX_FRAMEBUFFER_COLOR_ATTACHMENTS", Self::MAX_TEXTURES_PER_UNIFORM_SET => "MAX_TEXTURES_PER_UNIFORM_SET", Self::MAX_SAMPLERS_PER_UNIFORM_SET => "MAX_SAMPLERS_PER_UNIFORM_SET", Self::MAX_STORAGE_BUFFERS_PER_UNIFORM_SET => "MAX_STORAGE_BUFFERS_PER_UNIFORM_SET", Self::MAX_STORAGE_IMAGES_PER_UNIFORM_SET => "MAX_STORAGE_IMAGES_PER_UNIFORM_SET", Self::MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET => "MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET", Self::MAX_DRAW_INDEXED_INDEX => "MAX_DRAW_INDEXED_INDEX", Self::MAX_FRAMEBUFFER_HEIGHT => "MAX_FRAMEBUFFER_HEIGHT", Self::MAX_FRAMEBUFFER_WIDTH => "MAX_FRAMEBUFFER_WIDTH", Self::MAX_TEXTURE_ARRAY_LAYERS => "MAX_TEXTURE_ARRAY_LAYERS", Self::MAX_TEXTURE_SIZE_1D => "MAX_TEXTURE_SIZE_1D", Self::MAX_TEXTURE_SIZE_2D => "MAX_TEXTURE_SIZE_2D", Self::MAX_TEXTURE_SIZE_3D => "MAX_TEXTURE_SIZE_3D", Self::MAX_TEXTURE_SIZE_CUBE => "MAX_TEXTURE_SIZE_CUBE", Self::MAX_TEXTURES_PER_SHADER_STAGE => "MAX_TEXTURES_PER_SHADER_STAGE", Self::MAX_SAMPLERS_PER_SHADER_STAGE => "MAX_SAMPLERS_PER_SHADER_STAGE", Self::MAX_STORAGE_BUFFERS_PER_SHADER_STAGE => "MAX_STORAGE_BUFFERS_PER_SHADER_STAGE", Self::MAX_STORAGE_IMAGES_PER_SHADER_STAGE => "MAX_STORAGE_IMAGES_PER_SHADER_STAGE", Self::MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE => "MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE", Self::MAX_PUSH_CONSTANT_SIZE => "MAX_PUSH_CONSTANT_SIZE", Self::MAX_UNIFORM_BUFFER_SIZE => "MAX_UNIFORM_BUFFER_SIZE", Self::MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET => "MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET", Self::MAX_VERTEX_INPUT_ATTRIBUTES => "MAX_VERTEX_INPUT_ATTRIBUTES", Self::MAX_VERTEX_INPUT_BINDINGS => "MAX_VERTEX_INPUT_BINDINGS", Self::MAX_VERTEX_INPUT_BINDING_STRIDE => "MAX_VERTEX_INPUT_BINDING_STRIDE", Self::MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT => "MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT", Self::MAX_COMPUTE_SHARED_MEMORY_SIZE => "MAX_COMPUTE_SHARED_MEMORY_SIZE", Self::MAX_COMPUTE_WORKGROUP_COUNT_X => "MAX_COMPUTE_WORKGROUP_COUNT_X", Self::MAX_COMPUTE_WORKGROUP_COUNT_Y => "MAX_COMPUTE_WORKGROUP_COUNT_Y", Self::MAX_COMPUTE_WORKGROUP_COUNT_Z => "MAX_COMPUTE_WORKGROUP_COUNT_Z", Self::MAX_COMPUTE_WORKGROUP_INVOCATIONS => "MAX_COMPUTE_WORKGROUP_INVOCATIONS", Self::MAX_COMPUTE_WORKGROUP_SIZE_X => "MAX_COMPUTE_WORKGROUP_SIZE_X", Self::MAX_COMPUTE_WORKGROUP_SIZE_Y => "MAX_COMPUTE_WORKGROUP_SIZE_Y", Self::MAX_COMPUTE_WORKGROUP_SIZE_Z => "MAX_COMPUTE_WORKGROUP_SIZE_Z", Self::MAX_VIEWPORT_DIMENSIONS_X => "MAX_VIEWPORT_DIMENSIONS_X", Self::MAX_VIEWPORT_DIMENSIONS_Y => "MAX_VIEWPORT_DIMENSIONS_Y", Self::METALFX_TEMPORAL_SCALER_MIN_SCALE => "METALFX_TEMPORAL_SCALER_MIN_SCALE", Self::METALFX_TEMPORAL_SCALER_MAX_SCALE => "METALFX_TEMPORAL_SCALER_MAX_SCALE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Limit::MAX_BOUND_UNIFORM_SETS, Limit::MAX_FRAMEBUFFER_COLOR_ATTACHMENTS, Limit::MAX_TEXTURES_PER_UNIFORM_SET, Limit::MAX_SAMPLERS_PER_UNIFORM_SET, Limit::MAX_STORAGE_BUFFERS_PER_UNIFORM_SET, Limit::MAX_STORAGE_IMAGES_PER_UNIFORM_SET, Limit::MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET, Limit::MAX_DRAW_INDEXED_INDEX, Limit::MAX_FRAMEBUFFER_HEIGHT, Limit::MAX_FRAMEBUFFER_WIDTH, Limit::MAX_TEXTURE_ARRAY_LAYERS, Limit::MAX_TEXTURE_SIZE_1D, Limit::MAX_TEXTURE_SIZE_2D, Limit::MAX_TEXTURE_SIZE_3D, Limit::MAX_TEXTURE_SIZE_CUBE, Limit::MAX_TEXTURES_PER_SHADER_STAGE, Limit::MAX_SAMPLERS_PER_SHADER_STAGE, Limit::MAX_STORAGE_BUFFERS_PER_SHADER_STAGE, Limit::MAX_STORAGE_IMAGES_PER_SHADER_STAGE, Limit::MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE, Limit::MAX_PUSH_CONSTANT_SIZE, Limit::MAX_UNIFORM_BUFFER_SIZE, Limit::MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET, Limit::MAX_VERTEX_INPUT_ATTRIBUTES, Limit::MAX_VERTEX_INPUT_BINDINGS, Limit::MAX_VERTEX_INPUT_BINDING_STRIDE, Limit::MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT, Limit::MAX_COMPUTE_SHARED_MEMORY_SIZE, Limit::MAX_COMPUTE_WORKGROUP_COUNT_X, Limit::MAX_COMPUTE_WORKGROUP_COUNT_Y, Limit::MAX_COMPUTE_WORKGROUP_COUNT_Z, Limit::MAX_COMPUTE_WORKGROUP_INVOCATIONS, Limit::MAX_COMPUTE_WORKGROUP_SIZE_X, Limit::MAX_COMPUTE_WORKGROUP_SIZE_Y, Limit::MAX_COMPUTE_WORKGROUP_SIZE_Z, Limit::MAX_VIEWPORT_DIMENSIONS_X, Limit::MAX_VIEWPORT_DIMENSIONS_Y, Limit::METALFX_TEMPORAL_SCALER_MIN_SCALE, Limit::METALFX_TEMPORAL_SCALER_MAX_SCALE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Limit >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("MAX_BOUND_UNIFORM_SETS", "LIMIT_MAX_BOUND_UNIFORM_SETS", Limit::MAX_BOUND_UNIFORM_SETS), crate::meta::inspect::EnumConstant::new("MAX_FRAMEBUFFER_COLOR_ATTACHMENTS", "LIMIT_MAX_FRAMEBUFFER_COLOR_ATTACHMENTS", Limit::MAX_FRAMEBUFFER_COLOR_ATTACHMENTS), crate::meta::inspect::EnumConstant::new("MAX_TEXTURES_PER_UNIFORM_SET", "LIMIT_MAX_TEXTURES_PER_UNIFORM_SET", Limit::MAX_TEXTURES_PER_UNIFORM_SET), crate::meta::inspect::EnumConstant::new("MAX_SAMPLERS_PER_UNIFORM_SET", "LIMIT_MAX_SAMPLERS_PER_UNIFORM_SET", Limit::MAX_SAMPLERS_PER_UNIFORM_SET), crate::meta::inspect::EnumConstant::new("MAX_STORAGE_BUFFERS_PER_UNIFORM_SET", "LIMIT_MAX_STORAGE_BUFFERS_PER_UNIFORM_SET", Limit::MAX_STORAGE_BUFFERS_PER_UNIFORM_SET), crate::meta::inspect::EnumConstant::new("MAX_STORAGE_IMAGES_PER_UNIFORM_SET", "LIMIT_MAX_STORAGE_IMAGES_PER_UNIFORM_SET", Limit::MAX_STORAGE_IMAGES_PER_UNIFORM_SET), crate::meta::inspect::EnumConstant::new("MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET", "LIMIT_MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET", Limit::MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET), crate::meta::inspect::EnumConstant::new("MAX_DRAW_INDEXED_INDEX", "LIMIT_MAX_DRAW_INDEXED_INDEX", Limit::MAX_DRAW_INDEXED_INDEX), crate::meta::inspect::EnumConstant::new("MAX_FRAMEBUFFER_HEIGHT", "LIMIT_MAX_FRAMEBUFFER_HEIGHT", Limit::MAX_FRAMEBUFFER_HEIGHT), crate::meta::inspect::EnumConstant::new("MAX_FRAMEBUFFER_WIDTH", "LIMIT_MAX_FRAMEBUFFER_WIDTH", Limit::MAX_FRAMEBUFFER_WIDTH), crate::meta::inspect::EnumConstant::new("MAX_TEXTURE_ARRAY_LAYERS", "LIMIT_MAX_TEXTURE_ARRAY_LAYERS", Limit::MAX_TEXTURE_ARRAY_LAYERS), crate::meta::inspect::EnumConstant::new("MAX_TEXTURE_SIZE_1D", "LIMIT_MAX_TEXTURE_SIZE_1D", Limit::MAX_TEXTURE_SIZE_1D), crate::meta::inspect::EnumConstant::new("MAX_TEXTURE_SIZE_2D", "LIMIT_MAX_TEXTURE_SIZE_2D", Limit::MAX_TEXTURE_SIZE_2D), crate::meta::inspect::EnumConstant::new("MAX_TEXTURE_SIZE_3D", "LIMIT_MAX_TEXTURE_SIZE_3D", Limit::MAX_TEXTURE_SIZE_3D), crate::meta::inspect::EnumConstant::new("MAX_TEXTURE_SIZE_CUBE", "LIMIT_MAX_TEXTURE_SIZE_CUBE", Limit::MAX_TEXTURE_SIZE_CUBE), crate::meta::inspect::EnumConstant::new("MAX_TEXTURES_PER_SHADER_STAGE", "LIMIT_MAX_TEXTURES_PER_SHADER_STAGE", Limit::MAX_TEXTURES_PER_SHADER_STAGE), crate::meta::inspect::EnumConstant::new("MAX_SAMPLERS_PER_SHADER_STAGE", "LIMIT_MAX_SAMPLERS_PER_SHADER_STAGE", Limit::MAX_SAMPLERS_PER_SHADER_STAGE), crate::meta::inspect::EnumConstant::new("MAX_STORAGE_BUFFERS_PER_SHADER_STAGE", "LIMIT_MAX_STORAGE_BUFFERS_PER_SHADER_STAGE", Limit::MAX_STORAGE_BUFFERS_PER_SHADER_STAGE), crate::meta::inspect::EnumConstant::new("MAX_STORAGE_IMAGES_PER_SHADER_STAGE", "LIMIT_MAX_STORAGE_IMAGES_PER_SHADER_STAGE", Limit::MAX_STORAGE_IMAGES_PER_SHADER_STAGE), crate::meta::inspect::EnumConstant::new("MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE", "LIMIT_MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE", Limit::MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE), crate::meta::inspect::EnumConstant::new("MAX_PUSH_CONSTANT_SIZE", "LIMIT_MAX_PUSH_CONSTANT_SIZE", Limit::MAX_PUSH_CONSTANT_SIZE), crate::meta::inspect::EnumConstant::new("MAX_UNIFORM_BUFFER_SIZE", "LIMIT_MAX_UNIFORM_BUFFER_SIZE", Limit::MAX_UNIFORM_BUFFER_SIZE), crate::meta::inspect::EnumConstant::new("MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET", "LIMIT_MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET", Limit::MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET), crate::meta::inspect::EnumConstant::new("MAX_VERTEX_INPUT_ATTRIBUTES", "LIMIT_MAX_VERTEX_INPUT_ATTRIBUTES", Limit::MAX_VERTEX_INPUT_ATTRIBUTES), crate::meta::inspect::EnumConstant::new("MAX_VERTEX_INPUT_BINDINGS", "LIMIT_MAX_VERTEX_INPUT_BINDINGS", Limit::MAX_VERTEX_INPUT_BINDINGS), crate::meta::inspect::EnumConstant::new("MAX_VERTEX_INPUT_BINDING_STRIDE", "LIMIT_MAX_VERTEX_INPUT_BINDING_STRIDE", Limit::MAX_VERTEX_INPUT_BINDING_STRIDE), crate::meta::inspect::EnumConstant::new("MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT", "LIMIT_MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT", Limit::MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT), crate::meta::inspect::EnumConstant::new("MAX_COMPUTE_SHARED_MEMORY_SIZE", "LIMIT_MAX_COMPUTE_SHARED_MEMORY_SIZE", Limit::MAX_COMPUTE_SHARED_MEMORY_SIZE), crate::meta::inspect::EnumConstant::new("MAX_COMPUTE_WORKGROUP_COUNT_X", "LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_X", Limit::MAX_COMPUTE_WORKGROUP_COUNT_X), crate::meta::inspect::EnumConstant::new("MAX_COMPUTE_WORKGROUP_COUNT_Y", "LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_Y", Limit::MAX_COMPUTE_WORKGROUP_COUNT_Y), crate::meta::inspect::EnumConstant::new("MAX_COMPUTE_WORKGROUP_COUNT_Z", "LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_Z", Limit::MAX_COMPUTE_WORKGROUP_COUNT_Z), crate::meta::inspect::EnumConstant::new("MAX_COMPUTE_WORKGROUP_INVOCATIONS", "LIMIT_MAX_COMPUTE_WORKGROUP_INVOCATIONS", Limit::MAX_COMPUTE_WORKGROUP_INVOCATIONS), crate::meta::inspect::EnumConstant::new("MAX_COMPUTE_WORKGROUP_SIZE_X", "LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_X", Limit::MAX_COMPUTE_WORKGROUP_SIZE_X), crate::meta::inspect::EnumConstant::new("MAX_COMPUTE_WORKGROUP_SIZE_Y", "LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_Y", Limit::MAX_COMPUTE_WORKGROUP_SIZE_Y), crate::meta::inspect::EnumConstant::new("MAX_COMPUTE_WORKGROUP_SIZE_Z", "LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_Z", Limit::MAX_COMPUTE_WORKGROUP_SIZE_Z), crate::meta::inspect::EnumConstant::new("MAX_VIEWPORT_DIMENSIONS_X", "LIMIT_MAX_VIEWPORT_DIMENSIONS_X", Limit::MAX_VIEWPORT_DIMENSIONS_X), crate::meta::inspect::EnumConstant::new("MAX_VIEWPORT_DIMENSIONS_Y", "LIMIT_MAX_VIEWPORT_DIMENSIONS_Y", Limit::MAX_VIEWPORT_DIMENSIONS_Y), crate::meta::inspect::EnumConstant::new("METALFX_TEMPORAL_SCALER_MIN_SCALE", "LIMIT_METALFX_TEMPORAL_SCALER_MIN_SCALE", Limit::METALFX_TEMPORAL_SCALER_MIN_SCALE), crate::meta::inspect::EnumConstant::new("METALFX_TEMPORAL_SCALER_MAX_SCALE", "LIMIT_METALFX_TEMPORAL_SCALER_MAX_SCALE", Limit::METALFX_TEMPORAL_SCALER_MAX_SCALE)]
        }
    }
}
impl crate::meta::GodotConvert for Limit {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Limit {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Limit {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MemoryType {
    ord: i32
}
impl MemoryType {
    #[doc(alias = "MEMORY_TEXTURES")]
    #[doc = "Godot enumerator name: `MEMORY_TEXTURES`"]
    pub const TEXTURES: MemoryType = MemoryType {
        ord: 0i32
    };
    #[doc(alias = "MEMORY_BUFFERS")]
    #[doc = "Godot enumerator name: `MEMORY_BUFFERS`"]
    pub const BUFFERS: MemoryType = MemoryType {
        ord: 1i32
    };
    #[doc(alias = "MEMORY_TOTAL")]
    #[doc = "Godot enumerator name: `MEMORY_TOTAL`"]
    pub const TOTAL: MemoryType = MemoryType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for MemoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MemoryType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MemoryType {
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
            Self::TEXTURES => "TEXTURES", Self::BUFFERS => "BUFFERS", Self::TOTAL => "TOTAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MemoryType::TEXTURES, MemoryType::BUFFERS, MemoryType::TOTAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MemoryType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TEXTURES", "MEMORY_TEXTURES", MemoryType::TEXTURES), crate::meta::inspect::EnumConstant::new("BUFFERS", "MEMORY_BUFFERS", MemoryType::BUFFERS), crate::meta::inspect::EnumConstant::new("TOTAL", "MEMORY_TOTAL", MemoryType::TOTAL)]
        }
    }
}
impl crate::meta::GodotConvert for MemoryType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MemoryType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MemoryType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BreadcrumbMarker {
    ord: i32
}
impl BreadcrumbMarker {
    pub const NONE: BreadcrumbMarker = BreadcrumbMarker {
        ord: 0i32
    };
    pub const REFLECTION_PROBES: BreadcrumbMarker = BreadcrumbMarker {
        ord: 65536i32
    };
    pub const SKY_PASS: BreadcrumbMarker = BreadcrumbMarker {
        ord: 131072i32
    };
    pub const LIGHTMAPPER_PASS: BreadcrumbMarker = BreadcrumbMarker {
        ord: 196608i32
    };
    pub const SHADOW_PASS_DIRECTIONAL: BreadcrumbMarker = BreadcrumbMarker {
        ord: 262144i32
    };
    pub const SHADOW_PASS_CUBE: BreadcrumbMarker = BreadcrumbMarker {
        ord: 327680i32
    };
    pub const OPAQUE_PASS: BreadcrumbMarker = BreadcrumbMarker {
        ord: 393216i32
    };
    pub const ALPHA_PASS: BreadcrumbMarker = BreadcrumbMarker {
        ord: 458752i32
    };
    pub const TRANSPARENT_PASS: BreadcrumbMarker = BreadcrumbMarker {
        ord: 524288i32
    };
    pub const POST_PROCESSING_PASS: BreadcrumbMarker = BreadcrumbMarker {
        ord: 589824i32
    };
    pub const BLIT_PASS: BreadcrumbMarker = BreadcrumbMarker {
        ord: 655360i32
    };
    pub const UI_PASS: BreadcrumbMarker = BreadcrumbMarker {
        ord: 720896i32
    };
    pub const DEBUG_PASS: BreadcrumbMarker = BreadcrumbMarker {
        ord: 786432i32
    };
    
}
impl std::fmt::Debug for BreadcrumbMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BreadcrumbMarker") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BreadcrumbMarker {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 65536i32 | ord @ 131072i32 | ord @ 196608i32 | ord @ 262144i32 | ord @ 327680i32 | ord @ 393216i32 | ord @ 458752i32 | ord @ 524288i32 | ord @ 589824i32 | ord @ 655360i32 | ord @ 720896i32 | ord @ 786432i32 => Some(Self {
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
            Self::NONE => "NONE", Self::REFLECTION_PROBES => "REFLECTION_PROBES", Self::SKY_PASS => "SKY_PASS", Self::LIGHTMAPPER_PASS => "LIGHTMAPPER_PASS", Self::SHADOW_PASS_DIRECTIONAL => "SHADOW_PASS_DIRECTIONAL", Self::SHADOW_PASS_CUBE => "SHADOW_PASS_CUBE", Self::OPAQUE_PASS => "OPAQUE_PASS", Self::ALPHA_PASS => "ALPHA_PASS", Self::TRANSPARENT_PASS => "TRANSPARENT_PASS", Self::POST_PROCESSING_PASS => "POST_PROCESSING_PASS", Self::BLIT_PASS => "BLIT_PASS", Self::UI_PASS => "UI_PASS", Self::DEBUG_PASS => "DEBUG_PASS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BreadcrumbMarker::NONE, BreadcrumbMarker::REFLECTION_PROBES, BreadcrumbMarker::SKY_PASS, BreadcrumbMarker::LIGHTMAPPER_PASS, BreadcrumbMarker::SHADOW_PASS_DIRECTIONAL, BreadcrumbMarker::SHADOW_PASS_CUBE, BreadcrumbMarker::OPAQUE_PASS, BreadcrumbMarker::ALPHA_PASS, BreadcrumbMarker::TRANSPARENT_PASS, BreadcrumbMarker::POST_PROCESSING_PASS, BreadcrumbMarker::BLIT_PASS, BreadcrumbMarker::UI_PASS, BreadcrumbMarker::DEBUG_PASS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BreadcrumbMarker >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "NONE", BreadcrumbMarker::NONE), crate::meta::inspect::EnumConstant::new("REFLECTION_PROBES", "REFLECTION_PROBES", BreadcrumbMarker::REFLECTION_PROBES), crate::meta::inspect::EnumConstant::new("SKY_PASS", "SKY_PASS", BreadcrumbMarker::SKY_PASS), crate::meta::inspect::EnumConstant::new("LIGHTMAPPER_PASS", "LIGHTMAPPER_PASS", BreadcrumbMarker::LIGHTMAPPER_PASS), crate::meta::inspect::EnumConstant::new("SHADOW_PASS_DIRECTIONAL", "SHADOW_PASS_DIRECTIONAL", BreadcrumbMarker::SHADOW_PASS_DIRECTIONAL), crate::meta::inspect::EnumConstant::new("SHADOW_PASS_CUBE", "SHADOW_PASS_CUBE", BreadcrumbMarker::SHADOW_PASS_CUBE), crate::meta::inspect::EnumConstant::new("OPAQUE_PASS", "OPAQUE_PASS", BreadcrumbMarker::OPAQUE_PASS), crate::meta::inspect::EnumConstant::new("ALPHA_PASS", "ALPHA_PASS", BreadcrumbMarker::ALPHA_PASS), crate::meta::inspect::EnumConstant::new("TRANSPARENT_PASS", "TRANSPARENT_PASS", BreadcrumbMarker::TRANSPARENT_PASS), crate::meta::inspect::EnumConstant::new("POST_PROCESSING_PASS", "POST_PROCESSING_PASS", BreadcrumbMarker::POST_PROCESSING_PASS), crate::meta::inspect::EnumConstant::new("BLIT_PASS", "BLIT_PASS", BreadcrumbMarker::BLIT_PASS), crate::meta::inspect::EnumConstant::new("UI_PASS", "UI_PASS", BreadcrumbMarker::UI_PASS), crate::meta::inspect::EnumConstant::new("DEBUG_PASS", "DEBUG_PASS", BreadcrumbMarker::DEBUG_PASS)]
        }
    }
}
impl crate::meta::GodotConvert for BreadcrumbMarker {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BreadcrumbMarker {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BreadcrumbMarker {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct DrawFlags {
    ord: u64
}
impl DrawFlags {
    #[doc(alias = "DRAW_DEFAULT_ALL")]
    #[doc = "Godot enumerator name: `DRAW_DEFAULT_ALL`"]
    pub const DEFAULT_ALL: DrawFlags = DrawFlags {
        ord: 0u64
    };
    #[doc(alias = "DRAW_CLEAR_COLOR_0")]
    #[doc = "Godot enumerator name: `DRAW_CLEAR_COLOR_0`"]
    pub const CLEAR_COLOR_0: DrawFlags = DrawFlags {
        ord: 1u64
    };
    #[doc(alias = "DRAW_CLEAR_COLOR_1")]
    #[doc = "Godot enumerator name: `DRAW_CLEAR_COLOR_1`"]
    pub const CLEAR_COLOR_1: DrawFlags = DrawFlags {
        ord: 2u64
    };
    #[doc(alias = "DRAW_CLEAR_COLOR_2")]
    #[doc = "Godot enumerator name: `DRAW_CLEAR_COLOR_2`"]
    pub const CLEAR_COLOR_2: DrawFlags = DrawFlags {
        ord: 4u64
    };
    #[doc(alias = "DRAW_CLEAR_COLOR_3")]
    #[doc = "Godot enumerator name: `DRAW_CLEAR_COLOR_3`"]
    pub const CLEAR_COLOR_3: DrawFlags = DrawFlags {
        ord: 8u64
    };
    #[doc(alias = "DRAW_CLEAR_COLOR_4")]
    #[doc = "Godot enumerator name: `DRAW_CLEAR_COLOR_4`"]
    pub const CLEAR_COLOR_4: DrawFlags = DrawFlags {
        ord: 16u64
    };
    #[doc(alias = "DRAW_CLEAR_COLOR_5")]
    #[doc = "Godot enumerator name: `DRAW_CLEAR_COLOR_5`"]
    pub const CLEAR_COLOR_5: DrawFlags = DrawFlags {
        ord: 32u64
    };
    #[doc(alias = "DRAW_CLEAR_COLOR_6")]
    #[doc = "Godot enumerator name: `DRAW_CLEAR_COLOR_6`"]
    pub const CLEAR_COLOR_6: DrawFlags = DrawFlags {
        ord: 64u64
    };
    #[doc(alias = "DRAW_CLEAR_COLOR_7")]
    #[doc = "Godot enumerator name: `DRAW_CLEAR_COLOR_7`"]
    pub const CLEAR_COLOR_7: DrawFlags = DrawFlags {
        ord: 128u64
    };
    #[doc(alias = "DRAW_CLEAR_COLOR_MASK")]
    #[doc = "Godot enumerator name: `DRAW_CLEAR_COLOR_MASK`"]
    pub const CLEAR_COLOR_MASK: DrawFlags = DrawFlags {
        ord: 255u64
    };
    #[doc(alias = "DRAW_CLEAR_COLOR_ALL")]
    #[doc = "Godot enumerator name: `DRAW_CLEAR_COLOR_ALL`"]
    pub const CLEAR_COLOR_ALL: DrawFlags = DrawFlags {
        ord: 255u64
    };
    #[doc(alias = "DRAW_IGNORE_COLOR_0")]
    #[doc = "Godot enumerator name: `DRAW_IGNORE_COLOR_0`"]
    pub const IGNORE_COLOR_0: DrawFlags = DrawFlags {
        ord: 256u64
    };
    #[doc(alias = "DRAW_IGNORE_COLOR_1")]
    #[doc = "Godot enumerator name: `DRAW_IGNORE_COLOR_1`"]
    pub const IGNORE_COLOR_1: DrawFlags = DrawFlags {
        ord: 512u64
    };
    #[doc(alias = "DRAW_IGNORE_COLOR_2")]
    #[doc = "Godot enumerator name: `DRAW_IGNORE_COLOR_2`"]
    pub const IGNORE_COLOR_2: DrawFlags = DrawFlags {
        ord: 1024u64
    };
    #[doc(alias = "DRAW_IGNORE_COLOR_3")]
    #[doc = "Godot enumerator name: `DRAW_IGNORE_COLOR_3`"]
    pub const IGNORE_COLOR_3: DrawFlags = DrawFlags {
        ord: 2048u64
    };
    #[doc(alias = "DRAW_IGNORE_COLOR_4")]
    #[doc = "Godot enumerator name: `DRAW_IGNORE_COLOR_4`"]
    pub const IGNORE_COLOR_4: DrawFlags = DrawFlags {
        ord: 4096u64
    };
    #[doc(alias = "DRAW_IGNORE_COLOR_5")]
    #[doc = "Godot enumerator name: `DRAW_IGNORE_COLOR_5`"]
    pub const IGNORE_COLOR_5: DrawFlags = DrawFlags {
        ord: 8192u64
    };
    #[doc(alias = "DRAW_IGNORE_COLOR_6")]
    #[doc = "Godot enumerator name: `DRAW_IGNORE_COLOR_6`"]
    pub const IGNORE_COLOR_6: DrawFlags = DrawFlags {
        ord: 16384u64
    };
    #[doc(alias = "DRAW_IGNORE_COLOR_7")]
    #[doc = "Godot enumerator name: `DRAW_IGNORE_COLOR_7`"]
    pub const IGNORE_COLOR_7: DrawFlags = DrawFlags {
        ord: 32768u64
    };
    #[doc(alias = "DRAW_IGNORE_COLOR_MASK")]
    #[doc = "Godot enumerator name: `DRAW_IGNORE_COLOR_MASK`"]
    pub const IGNORE_COLOR_MASK: DrawFlags = DrawFlags {
        ord: 65280u64
    };
    #[doc(alias = "DRAW_IGNORE_COLOR_ALL")]
    #[doc = "Godot enumerator name: `DRAW_IGNORE_COLOR_ALL`"]
    pub const IGNORE_COLOR_ALL: DrawFlags = DrawFlags {
        ord: 65280u64
    };
    #[doc(alias = "DRAW_CLEAR_DEPTH")]
    #[doc = "Godot enumerator name: `DRAW_CLEAR_DEPTH`"]
    pub const CLEAR_DEPTH: DrawFlags = DrawFlags {
        ord: 65536u64
    };
    #[doc(alias = "DRAW_IGNORE_DEPTH")]
    #[doc = "Godot enumerator name: `DRAW_IGNORE_DEPTH`"]
    pub const IGNORE_DEPTH: DrawFlags = DrawFlags {
        ord: 131072u64
    };
    #[doc(alias = "DRAW_CLEAR_STENCIL")]
    #[doc = "Godot enumerator name: `DRAW_CLEAR_STENCIL`"]
    pub const CLEAR_STENCIL: DrawFlags = DrawFlags {
        ord: 262144u64
    };
    #[doc(alias = "DRAW_IGNORE_STENCIL")]
    #[doc = "Godot enumerator name: `DRAW_IGNORE_STENCIL`"]
    pub const IGNORE_STENCIL: DrawFlags = DrawFlags {
        ord: 524288u64
    };
    #[doc(alias = "DRAW_CLEAR_ALL")]
    #[doc = "Godot enumerator name: `DRAW_CLEAR_ALL`"]
    pub const CLEAR_ALL: DrawFlags = DrawFlags {
        ord: 327935u64
    };
    #[doc(alias = "DRAW_IGNORE_ALL")]
    #[doc = "Godot enumerator name: `DRAW_IGNORE_ALL`"]
    pub const IGNORE_ALL: DrawFlags = DrawFlags {
        ord: 720640u64
    };
    
}
impl std::fmt::Debug for DrawFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::DEFAULT_ALL => "DEFAULT_ALL", Self::CLEAR_COLOR_0 => "CLEAR_COLOR_0", Self::CLEAR_COLOR_1 => "CLEAR_COLOR_1", Self::CLEAR_COLOR_2 => "CLEAR_COLOR_2", Self::CLEAR_COLOR_3 => "CLEAR_COLOR_3", Self::CLEAR_COLOR_4 => "CLEAR_COLOR_4", Self::CLEAR_COLOR_5 => "CLEAR_COLOR_5", Self::CLEAR_COLOR_6 => "CLEAR_COLOR_6", Self::CLEAR_COLOR_7 => "CLEAR_COLOR_7", Self::CLEAR_COLOR_MASK => "CLEAR_COLOR_MASK", Self::CLEAR_COLOR_ALL => "CLEAR_COLOR_ALL", Self::IGNORE_COLOR_0 => "IGNORE_COLOR_0", Self::IGNORE_COLOR_1 => "IGNORE_COLOR_1", Self::IGNORE_COLOR_2 => "IGNORE_COLOR_2", Self::IGNORE_COLOR_3 => "IGNORE_COLOR_3", Self::IGNORE_COLOR_4 => "IGNORE_COLOR_4", Self::IGNORE_COLOR_5 => "IGNORE_COLOR_5", Self::IGNORE_COLOR_6 => "IGNORE_COLOR_6", Self::IGNORE_COLOR_7 => "IGNORE_COLOR_7", Self::IGNORE_COLOR_MASK => "IGNORE_COLOR_MASK", Self::IGNORE_COLOR_ALL => "IGNORE_COLOR_ALL", Self::CLEAR_DEPTH => "CLEAR_DEPTH", Self::IGNORE_DEPTH => "IGNORE_DEPTH", Self::CLEAR_STENCIL => "CLEAR_STENCIL", Self::IGNORE_STENCIL => "IGNORE_STENCIL", Self::CLEAR_ALL => "CLEAR_ALL", Self::IGNORE_ALL => "IGNORE_ALL", _ => {
                f.debug_struct("DrawFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for DrawFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DrawFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT_ALL", "DRAW_DEFAULT_ALL", DrawFlags::DEFAULT_ALL), crate::meta::inspect::EnumConstant::new("CLEAR_COLOR_0", "DRAW_CLEAR_COLOR_0", DrawFlags::CLEAR_COLOR_0), crate::meta::inspect::EnumConstant::new("CLEAR_COLOR_1", "DRAW_CLEAR_COLOR_1", DrawFlags::CLEAR_COLOR_1), crate::meta::inspect::EnumConstant::new("CLEAR_COLOR_2", "DRAW_CLEAR_COLOR_2", DrawFlags::CLEAR_COLOR_2), crate::meta::inspect::EnumConstant::new("CLEAR_COLOR_3", "DRAW_CLEAR_COLOR_3", DrawFlags::CLEAR_COLOR_3), crate::meta::inspect::EnumConstant::new("CLEAR_COLOR_4", "DRAW_CLEAR_COLOR_4", DrawFlags::CLEAR_COLOR_4), crate::meta::inspect::EnumConstant::new("CLEAR_COLOR_5", "DRAW_CLEAR_COLOR_5", DrawFlags::CLEAR_COLOR_5), crate::meta::inspect::EnumConstant::new("CLEAR_COLOR_6", "DRAW_CLEAR_COLOR_6", DrawFlags::CLEAR_COLOR_6), crate::meta::inspect::EnumConstant::new("CLEAR_COLOR_7", "DRAW_CLEAR_COLOR_7", DrawFlags::CLEAR_COLOR_7), crate::meta::inspect::EnumConstant::new("CLEAR_COLOR_MASK", "DRAW_CLEAR_COLOR_MASK", DrawFlags::CLEAR_COLOR_MASK), crate::meta::inspect::EnumConstant::new("CLEAR_COLOR_ALL", "DRAW_CLEAR_COLOR_ALL", DrawFlags::CLEAR_COLOR_ALL), crate::meta::inspect::EnumConstant::new("IGNORE_COLOR_0", "DRAW_IGNORE_COLOR_0", DrawFlags::IGNORE_COLOR_0), crate::meta::inspect::EnumConstant::new("IGNORE_COLOR_1", "DRAW_IGNORE_COLOR_1", DrawFlags::IGNORE_COLOR_1), crate::meta::inspect::EnumConstant::new("IGNORE_COLOR_2", "DRAW_IGNORE_COLOR_2", DrawFlags::IGNORE_COLOR_2), crate::meta::inspect::EnumConstant::new("IGNORE_COLOR_3", "DRAW_IGNORE_COLOR_3", DrawFlags::IGNORE_COLOR_3), crate::meta::inspect::EnumConstant::new("IGNORE_COLOR_4", "DRAW_IGNORE_COLOR_4", DrawFlags::IGNORE_COLOR_4), crate::meta::inspect::EnumConstant::new("IGNORE_COLOR_5", "DRAW_IGNORE_COLOR_5", DrawFlags::IGNORE_COLOR_5), crate::meta::inspect::EnumConstant::new("IGNORE_COLOR_6", "DRAW_IGNORE_COLOR_6", DrawFlags::IGNORE_COLOR_6), crate::meta::inspect::EnumConstant::new("IGNORE_COLOR_7", "DRAW_IGNORE_COLOR_7", DrawFlags::IGNORE_COLOR_7), crate::meta::inspect::EnumConstant::new("IGNORE_COLOR_MASK", "DRAW_IGNORE_COLOR_MASK", DrawFlags::IGNORE_COLOR_MASK), crate::meta::inspect::EnumConstant::new("IGNORE_COLOR_ALL", "DRAW_IGNORE_COLOR_ALL", DrawFlags::IGNORE_COLOR_ALL), crate::meta::inspect::EnumConstant::new("CLEAR_DEPTH", "DRAW_CLEAR_DEPTH", DrawFlags::CLEAR_DEPTH), crate::meta::inspect::EnumConstant::new("IGNORE_DEPTH", "DRAW_IGNORE_DEPTH", DrawFlags::IGNORE_DEPTH), crate::meta::inspect::EnumConstant::new("CLEAR_STENCIL", "DRAW_CLEAR_STENCIL", DrawFlags::CLEAR_STENCIL), crate::meta::inspect::EnumConstant::new("IGNORE_STENCIL", "DRAW_IGNORE_STENCIL", DrawFlags::IGNORE_STENCIL), crate::meta::inspect::EnumConstant::new("CLEAR_ALL", "DRAW_CLEAR_ALL", DrawFlags::CLEAR_ALL), crate::meta::inspect::EnumConstant::new("IGNORE_ALL", "DRAW_IGNORE_ALL", DrawFlags::IGNORE_ALL)]
        }
    }
}
impl std::ops::BitOr for DrawFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for DrawFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for DrawFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for DrawFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DrawFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::RenderingDevice;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for RenderingDevice {
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