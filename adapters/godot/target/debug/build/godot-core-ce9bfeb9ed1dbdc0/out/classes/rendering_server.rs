#![doc = "Sidecar module for class [`RenderingServer`][crate::classes::RenderingServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RenderingServer` enums](https://docs.godotengine.org/en/stable/classes/class_renderingserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RenderingServer.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`rendering_server`][crate::classes::rendering_server]: sidecar module with related enum/flag types\n* [`SignalsOfRenderingServer`][crate::classes::rendering_server::SignalsOfRenderingServer]: signal collection\n\n\nSee also [Godot docs for `RenderingServer`](https://docs.godotengine.org/en/stable/classes/class_renderingserver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RenderingServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl RenderingServer {
        pub fn texture_2d_create(&mut self, image: impl AsArg < Option < Gd < crate::classes::Image >> >,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >,);
            let args = (image.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(755usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_2d_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_2d_layered_create(&mut self, layers: &Array < Gd < crate::classes::Image > >, layered_type: crate::classes::rendering_server::TextureLayeredType,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Gd < crate::classes::Image > > >, crate::classes::rendering_server::TextureLayeredType,);
            let args = (RefArg::new(layers), layered_type,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(756usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_2d_layered_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_3d_create(&mut self, format: crate::classes::image::Format, width: i32, height: i32, depth: i32, mipmaps: bool, data: &Array < Gd < crate::classes::Image > >,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (crate::classes::image::Format, i32, i32, i32, bool, RefArg < 'a0, Array < Gd < crate::classes::Image > > >,);
            let args = (format, width, height, depth, mipmaps, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(757usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_3d_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_proxy_create(&mut self, base: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (base,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(758usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_proxy_create", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn texture_create_from_native_handle_full(&mut self, type_: crate::classes::rendering_server::TextureType, format: crate::classes::image::Format, native_handle: u64, width: i32, height: i32, depth: i32, layers: i32, layered_type: crate::classes::rendering_server::TextureLayeredType,) -> Rid {
            type CallRet = Rid;
            type CallParams = (crate::classes::rendering_server::TextureType, crate::classes::image::Format, u64, i32, i32, i32, i32, crate::classes::rendering_server::TextureLayeredType,);
            let args = (type_, format, native_handle, width, height, depth, layers, layered_type,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(759usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_create_from_native_handle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::texture_create_from_native_handle_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn texture_create_from_native_handle(&mut self, type_: crate::classes::rendering_server::TextureType, format: crate::classes::image::Format, native_handle: u64, width: i32, height: i32, depth: i32,) -> Rid {
            self.texture_create_from_native_handle_ex(type_, format, native_handle, width, height, depth,) . done()
        }
        #[inline]
        pub fn texture_create_from_native_handle_ex < 'a > (&'a mut self, type_: crate::classes::rendering_server::TextureType, format: crate::classes::image::Format, native_handle: u64, width: i32, height: i32, depth: i32,) -> ExTextureCreateFromNativeHandle < 'a > {
            ExTextureCreateFromNativeHandle::new(self, type_, format, native_handle, width, height, depth,)
        }
        pub fn texture_2d_update(&mut self, texture: Rid, image: impl AsArg < Option < Gd < crate::classes::Image >> >, layer: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, Option < Gd < crate::classes::Image > > >, i32,);
            let args = (texture, image.into_arg(), layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(760usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_2d_update", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_3d_update(&mut self, texture: Rid, data: &Array < Gd < crate::classes::Image > >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Array < Gd < crate::classes::Image > > >,);
            let args = (texture, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(761usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_3d_update", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_proxy_update(&mut self, texture: Rid, proxy_to: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (texture, proxy_to,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(762usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_proxy_update", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_2d_placeholder_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(763usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_2d_placeholder_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_2d_layered_placeholder_create(&mut self, layered_type: crate::classes::rendering_server::TextureLayeredType,) -> Rid {
            type CallRet = Rid;
            type CallParams = (crate::classes::rendering_server::TextureLayeredType,);
            let args = (layered_type,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(764usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_2d_layered_placeholder_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_3d_placeholder_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(765usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_3d_placeholder_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_2d_get(&self, texture: Rid,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (Rid,);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(766usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_2d_get", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_2d_layer_get(&self, texture: Rid, layer: i32,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (Rid, i32,);
            let args = (texture, layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(767usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_2d_layer_get", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_3d_get(&self, texture: Rid,) -> Array < Gd < crate::classes::Image > > {
            type CallRet = Array < Gd < crate::classes::Image > >;
            type CallParams = (Rid,);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(768usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_3d_get", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_replace(&mut self, texture: Rid, by_texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (texture, by_texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(769usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_replace", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_set_size_override(&mut self, texture: Rid, width: i32, height: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (texture, width, height,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(770usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_set_size_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_set_path(&mut self, texture: Rid, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (texture, path.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(771usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_set_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_get_path(&self, texture: Rid,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid,);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(772usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_get_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_get_format(&self, texture: Rid,) -> crate::classes::image::Format {
            type CallRet = crate::classes::image::Format;
            type CallParams = (Rid,);
            let args = (texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(773usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_get_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn texture_set_force_redraw_if_visible(&mut self, texture: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (texture, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(774usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_set_force_redraw_if_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn texture_rd_create_full(&mut self, rd_texture: Rid, layer_type: crate::classes::rendering_server::TextureLayeredType,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, crate::classes::rendering_server::TextureLayeredType,);
            let args = (rd_texture, layer_type,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(775usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_rd_create", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::texture_rd_create_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn texture_rd_create(&mut self, rd_texture: Rid,) -> Rid {
            self.texture_rd_create_ex(rd_texture,) . done()
        }
        #[inline]
        pub fn texture_rd_create_ex < 'a > (&'a mut self, rd_texture: Rid,) -> ExTextureRdCreate < 'a > {
            ExTextureRdCreate::new(self, rd_texture,)
        }
        pub(crate) fn texture_get_rd_texture_full(&self, texture: Rid, srgb: bool,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, bool,);
            let args = (texture, srgb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(776usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_get_rd_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::texture_get_rd_texture_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn texture_get_rd_texture(&self, texture: Rid,) -> Rid {
            self.texture_get_rd_texture_ex(texture,) . done()
        }
        #[inline]
        pub fn texture_get_rd_texture_ex < 'a > (&'a self, texture: Rid,) -> ExTextureGetRdTexture < 'a > {
            ExTextureGetRdTexture::new(self, texture,)
        }
        pub(crate) fn texture_get_native_handle_full(&self, texture: Rid, srgb: bool,) -> u64 {
            type CallRet = u64;
            type CallParams = (Rid, bool,);
            let args = (texture, srgb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(777usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "texture_get_native_handle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::texture_get_native_handle_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn texture_get_native_handle(&self, texture: Rid,) -> u64 {
            self.texture_get_native_handle_ex(texture,) . done()
        }
        #[inline]
        pub fn texture_get_native_handle_ex < 'a > (&'a self, texture: Rid,) -> ExTextureGetNativeHandle < 'a > {
            ExTextureGetNativeHandle::new(self, texture,)
        }
        pub fn shader_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(778usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "shader_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shader_set_code(&mut self, shader: Rid, code: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (shader, code.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(779usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "shader_set_code", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shader_set_path_hint(&mut self, shader: Rid, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, GString >,);
            let args = (shader, path.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(780usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "shader_set_path_hint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shader_get_code(&self, shader: Rid,) -> GString {
            type CallRet = GString;
            type CallParams = (Rid,);
            let args = (shader,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(781usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "shader_get_code", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_shader_parameter_list(&self, shader: Rid,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = (Rid,);
            let args = (shader,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(782usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_shader_parameter_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn shader_get_parameter_default(&self, shader: Rid, name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, StringName >,);
            let args = (shader, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(783usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "shader_get_parameter_default", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn shader_set_default_texture_parameter_full(&mut self, shader: Rid, name: CowArg < StringName >, texture: Rid, index: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, StringName >, Rid, i32,);
            let args = (shader, name, texture, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(784usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "shader_set_default_texture_parameter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shader_set_default_texture_parameter_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shader_set_default_texture_parameter(&mut self, shader: Rid, name: impl AsArg < StringName >, texture: Rid,) {
            self.shader_set_default_texture_parameter_ex(shader, name, texture,) . done()
        }
        #[inline]
        pub fn shader_set_default_texture_parameter_ex < 'a > (&'a mut self, shader: Rid, name: impl AsArg < StringName > + 'a, texture: Rid,) -> ExShaderSetDefaultTextureParameter < 'a > {
            ExShaderSetDefaultTextureParameter::new(self, shader, name, texture,)
        }
        pub(crate) fn shader_get_default_texture_parameter_full(&self, shader: Rid, name: CowArg < StringName >, index: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, StringName >, i32,);
            let args = (shader, name, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(785usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "shader_get_default_texture_parameter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::shader_get_default_texture_parameter_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn shader_get_default_texture_parameter(&self, shader: Rid, name: impl AsArg < StringName >,) -> Rid {
            self.shader_get_default_texture_parameter_ex(shader, name,) . done()
        }
        #[inline]
        pub fn shader_get_default_texture_parameter_ex < 'a > (&'a self, shader: Rid, name: impl AsArg < StringName > + 'a,) -> ExShaderGetDefaultTextureParameter < 'a > {
            ExShaderGetDefaultTextureParameter::new(self, shader, name,)
        }
        pub fn material_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(786usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "material_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn material_set_shader(&mut self, shader_material: Rid, shader: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (shader_material, shader,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(787usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "material_set_shader", Some(self.__validated_obj()), args,)
            }
        }
        pub fn material_set_param(&mut self, material: Rid, parameter: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (material, parameter.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(788usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "material_set_param", Some(self.__validated_obj()), args,)
            }
        }
        pub fn material_get_param(&self, material: Rid, parameter: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, StringName >,);
            let args = (material, parameter.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(789usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "material_get_param", Some(self.__validated_obj()), args,)
            }
        }
        pub fn material_set_render_priority(&mut self, material: Rid, priority: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (material, priority,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(790usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "material_set_render_priority", Some(self.__validated_obj()), args,)
            }
        }
        pub fn material_set_next_pass(&mut self, material: Rid, next_material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (material, next_material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(791usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "material_set_next_pass", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn mesh_create_from_surfaces_full(&mut self, surfaces: RefArg < Array < VarDictionary > >, blend_shape_count: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, > = (RefArg < 'a0, Array < VarDictionary > >, i32,);
            let args = (surfaces, blend_shape_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(792usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_create_from_surfaces", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::mesh_create_from_surfaces_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn mesh_create_from_surfaces(&mut self, surfaces: &Array < VarDictionary >,) -> Rid {
            self.mesh_create_from_surfaces_ex(surfaces,) . done()
        }
        #[inline]
        pub fn mesh_create_from_surfaces_ex < 'a > (&'a mut self, surfaces: &'a Array < VarDictionary >,) -> ExMeshCreateFromSurfaces < 'a > {
            ExMeshCreateFromSurfaces::new(self, surfaces,)
        }
        pub fn mesh_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(793usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_get_format_offset(&self, format: crate::classes::rendering_server::ArrayFormat, vertex_count: i32, array_index: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (crate::classes::rendering_server::ArrayFormat, i32, i32,);
            let args = (format, vertex_count, array_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(794usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_format_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_get_format_vertex_stride(&self, format: crate::classes::rendering_server::ArrayFormat, vertex_count: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (crate::classes::rendering_server::ArrayFormat, i32,);
            let args = (format, vertex_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(795usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_format_vertex_stride", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_get_format_normal_tangent_stride(&self, format: crate::classes::rendering_server::ArrayFormat, vertex_count: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (crate::classes::rendering_server::ArrayFormat, i32,);
            let args = (format, vertex_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(796usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_format_normal_tangent_stride", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_get_format_attribute_stride(&self, format: crate::classes::rendering_server::ArrayFormat, vertex_count: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (crate::classes::rendering_server::ArrayFormat, i32,);
            let args = (format, vertex_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(797usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_format_attribute_stride", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_get_format_skin_stride(&self, format: crate::classes::rendering_server::ArrayFormat, vertex_count: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (crate::classes::rendering_server::ArrayFormat, i32,);
            let args = (format, vertex_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(798usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_format_skin_stride", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_get_format_index_stride(&self, format: crate::classes::rendering_server::ArrayFormat, vertex_count: i32,) -> u32 {
            type CallRet = u32;
            type CallParams = (crate::classes::rendering_server::ArrayFormat, i32,);
            let args = (format, vertex_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(799usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_format_index_stride", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_add_surface(&mut self, mesh: Rid, surface: &VarDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, VarDictionary >,);
            let args = (mesh, RefArg::new(surface),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(800usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_add_surface", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn mesh_add_surface_from_arrays_full(&mut self, mesh: Rid, primitive: crate::classes::rendering_server::PrimitiveType, arrays: RefArg < VarArray >, blend_shapes: RefArg < VarArray >, lods: RefArg < VarDictionary >, compress_format: crate::classes::rendering_server::ArrayFormat,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (Rid, crate::classes::rendering_server::PrimitiveType, RefArg < 'a0, VarArray >, RefArg < 'a1, VarArray >, RefArg < 'a2, VarDictionary >, crate::classes::rendering_server::ArrayFormat,);
            let args = (mesh, primitive, arrays, blend_shapes, lods, compress_format,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(801usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_add_surface_from_arrays", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::mesh_add_surface_from_arrays_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn mesh_add_surface_from_arrays(&mut self, mesh: Rid, primitive: crate::classes::rendering_server::PrimitiveType, arrays: &VarArray,) {
            self.mesh_add_surface_from_arrays_ex(mesh, primitive, arrays,) . done()
        }
        #[inline]
        pub fn mesh_add_surface_from_arrays_ex < 'a > (&'a mut self, mesh: Rid, primitive: crate::classes::rendering_server::PrimitiveType, arrays: &'a VarArray,) -> ExMeshAddSurfaceFromArrays < 'a > {
            ExMeshAddSurfaceFromArrays::new(self, mesh, primitive, arrays,)
        }
        pub fn mesh_get_blend_shape_count(&self, mesh: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(802usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_get_blend_shape_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_set_blend_shape_mode(&mut self, mesh: Rid, mode: crate::classes::rendering_server::BlendShapeMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::BlendShapeMode,);
            let args = (mesh, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(803usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_set_blend_shape_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_get_blend_shape_mode(&self, mesh: Rid,) -> crate::classes::rendering_server::BlendShapeMode {
            type CallRet = crate::classes::rendering_server::BlendShapeMode;
            type CallParams = (Rid,);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(804usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_get_blend_shape_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_set_material(&mut self, mesh: Rid, surface: i32, material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Rid,);
            let args = (mesh, surface, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(805usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_set_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_get_material(&self, mesh: Rid, surface: i32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, i32,);
            let args = (mesh, surface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(806usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_get_surface(&mut self, mesh: Rid, surface: i32,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (Rid, i32,);
            let args = (mesh, surface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(807usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_get_surface", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_get_arrays(&self, mesh: Rid, surface: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (Rid, i32,);
            let args = (mesh, surface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(808usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_arrays", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_get_blend_shape_arrays(&self, mesh: Rid, surface: i32,) -> Array < VarArray > {
            type CallRet = Array < VarArray >;
            type CallParams = (Rid, i32,);
            let args = (mesh, surface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(809usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_get_blend_shape_arrays", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_get_surface_count(&self, mesh: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(810usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_get_surface_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_set_custom_aabb(&mut self, mesh: Rid, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Rid, Aabb,);
            let args = (mesh, aabb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(811usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_set_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_get_custom_aabb(&self, mesh: Rid,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = (Rid,);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(812usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_get_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_remove(&mut self, mesh: Rid, surface: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (mesh, surface,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(813usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_remove", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_clear(&mut self, mesh: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(814usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_update_vertex_region(&mut self, mesh: Rid, surface: i32, offset: i32, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, i32, RefArg < 'a0, PackedByteArray >,);
            let args = (mesh, surface, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(815usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_update_vertex_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_update_attribute_region(&mut self, mesh: Rid, surface: i32, offset: i32, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, i32, RefArg < 'a0, PackedByteArray >,);
            let args = (mesh, surface, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(816usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_update_attribute_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_update_skin_region(&mut self, mesh: Rid, surface: i32, offset: i32, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, i32, RefArg < 'a0, PackedByteArray >,);
            let args = (mesh, surface, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(817usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_update_skin_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_surface_update_index_region(&mut self, mesh: Rid, surface: i32, offset: i32, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, i32, i32, RefArg < 'a0, PackedByteArray >,);
            let args = (mesh, surface, offset, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(818usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_surface_update_index_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mesh_set_shadow_mesh(&mut self, mesh: Rid, shadow_mesh: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (mesh, shadow_mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(819usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "mesh_set_shadow_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(820usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_create", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn multimesh_allocate_data_full(&mut self, multimesh: Rid, instances: i32, transform_format: crate::classes::rendering_server::MultimeshTransformFormat, color_format: bool, custom_data_format: bool, use_indirect: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, crate::classes::rendering_server::MultimeshTransformFormat, bool, bool, bool,);
            let args = (multimesh, instances, transform_format, color_format, custom_data_format, use_indirect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(821usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_allocate_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::multimesh_allocate_data_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn multimesh_allocate_data(&mut self, multimesh: Rid, instances: i32, transform_format: crate::classes::rendering_server::MultimeshTransformFormat,) {
            self.multimesh_allocate_data_ex(multimesh, instances, transform_format,) . done()
        }
        #[inline]
        pub fn multimesh_allocate_data_ex < 'a > (&'a mut self, multimesh: Rid, instances: i32, transform_format: crate::classes::rendering_server::MultimeshTransformFormat,) -> ExMultimeshAllocateData < 'a > {
            ExMultimeshAllocateData::new(self, multimesh, instances, transform_format,)
        }
        pub fn multimesh_get_instance_count(&self, multimesh: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(822usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_get_instance_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_set_mesh(&mut self, multimesh: Rid, mesh: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (multimesh, mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(823usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_set_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_instance_set_transform(&mut self, multimesh: Rid, index: i32, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Transform3D,);
            let args = (multimesh, index, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(824usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_instance_set_transform_2d(&mut self, multimesh: Rid, index: i32, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Transform2D,);
            let args = (multimesh, index, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(825usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_set_transform_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_instance_set_color(&mut self, multimesh: Rid, index: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Color,);
            let args = (multimesh, index, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(826usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_set_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_instance_set_custom_data(&mut self, multimesh: Rid, index: i32, custom_data: Color,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Color,);
            let args = (multimesh, index, custom_data,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(827usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_set_custom_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_get_mesh(&self, multimesh: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(828usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_get_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_get_aabb(&self, multimesh: Rid,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(829usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_get_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_set_custom_aabb(&mut self, multimesh: Rid, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Rid, Aabb,);
            let args = (multimesh, aabb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(830usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_set_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_get_custom_aabb(&self, multimesh: Rid,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(831usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_get_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_instance_get_transform(&self, multimesh: Rid, index: i32,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (Rid, i32,);
            let args = (multimesh, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(832usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_get_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_instance_get_transform_2d(&self, multimesh: Rid, index: i32,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = (Rid, i32,);
            let args = (multimesh, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(833usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_get_transform_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_instance_get_color(&self, multimesh: Rid, index: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (Rid, i32,);
            let args = (multimesh, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(834usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_get_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_instance_get_custom_data(&self, multimesh: Rid, index: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (Rid, i32,);
            let args = (multimesh, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(835usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_get_custom_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_set_visible_instances(&mut self, multimesh: Rid, visible: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (multimesh, visible,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(836usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_set_visible_instances", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_get_visible_instances(&self, multimesh: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(837usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_get_visible_instances", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_set_buffer(&mut self, multimesh: Rid, buffer: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, PackedFloat32Array >,);
            let args = (multimesh, RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(838usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_set_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_get_command_buffer_rd_rid(&self, multimesh: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(839usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_get_command_buffer_rd_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_get_buffer_rd_rid(&self, multimesh: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(840usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_get_buffer_rd_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_get_buffer(&self, multimesh: Rid,) -> PackedFloat32Array {
            type CallRet = PackedFloat32Array;
            type CallParams = (Rid,);
            let args = (multimesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(841usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_get_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_set_buffer_interpolated(&mut self, multimesh: Rid, buffer: &PackedFloat32Array, buffer_previous: &PackedFloat32Array,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, RefArg < 'a0, PackedFloat32Array >, RefArg < 'a1, PackedFloat32Array >,);
            let args = (multimesh, RefArg::new(buffer), RefArg::new(buffer_previous),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(842usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_set_buffer_interpolated", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_set_physics_interpolated(&mut self, multimesh: Rid, interpolated: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (multimesh, interpolated,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(843usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_set_physics_interpolated", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_set_physics_interpolation_quality(&mut self, multimesh: Rid, quality: crate::classes::rendering_server::MultimeshPhysicsInterpolationQuality,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::MultimeshPhysicsInterpolationQuality,);
            let args = (multimesh, quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(844usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_set_physics_interpolation_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn multimesh_instance_reset_physics_interpolation(&mut self, multimesh: Rid, index: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (multimesh, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(845usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "multimesh_instance_reset_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn skeleton_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(846usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "skeleton_create", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn skeleton_allocate_data_full(&mut self, skeleton: Rid, bones: i32, is_2d_skeleton: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (skeleton, bones, is_2d_skeleton,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(847usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "skeleton_allocate_data", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::skeleton_allocate_data_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn skeleton_allocate_data(&mut self, skeleton: Rid, bones: i32,) {
            self.skeleton_allocate_data_ex(skeleton, bones,) . done()
        }
        #[inline]
        pub fn skeleton_allocate_data_ex < 'a > (&'a mut self, skeleton: Rid, bones: i32,) -> ExSkeletonAllocateData < 'a > {
            ExSkeletonAllocateData::new(self, skeleton, bones,)
        }
        pub fn skeleton_get_bone_count(&self, skeleton: Rid,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid,);
            let args = (skeleton,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(848usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "skeleton_get_bone_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn skeleton_bone_set_transform(&mut self, skeleton: Rid, bone: i32, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Transform3D,);
            let args = (skeleton, bone, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(849usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "skeleton_bone_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn skeleton_bone_get_transform(&self, skeleton: Rid, bone: i32,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (Rid, i32,);
            let args = (skeleton, bone,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(850usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "skeleton_bone_get_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn skeleton_bone_set_transform_2d(&mut self, skeleton: Rid, bone: i32, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Transform2D,);
            let args = (skeleton, bone, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(851usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "skeleton_bone_set_transform_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn skeleton_bone_get_transform_2d(&self, skeleton: Rid, bone: i32,) -> Transform2D {
            type CallRet = Transform2D;
            type CallParams = (Rid, i32,);
            let args = (skeleton, bone,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(852usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "skeleton_bone_get_transform_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn skeleton_set_base_transform_2d(&mut self, skeleton: Rid, base_transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (skeleton, base_transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(853usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "skeleton_set_base_transform_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn directional_light_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(854usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "directional_light_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn omni_light_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(855usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "omni_light_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn spot_light_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(856usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "spot_light_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn light_set_color(&mut self, light: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (light, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(857usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn light_set_param(&mut self, light: Rid, param: crate::classes::rendering_server::LightParam, value: f32,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::LightParam, f32,);
            let args = (light, param, value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(858usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_param", Some(self.__validated_obj()), args,)
            }
        }
        pub fn light_set_shadow(&mut self, light: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (light, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_shadow", Some(self.__validated_obj()), args,)
            }
        }
        pub fn light_set_projector(&mut self, light: Rid, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (light, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_projector", Some(self.__validated_obj()), args,)
            }
        }
        pub fn light_set_negative(&mut self, light: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (light, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(861usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_negative", Some(self.__validated_obj()), args,)
            }
        }
        pub fn light_set_cull_mask(&mut self, light: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (light, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(862usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn light_set_distance_fade(&mut self, decal: Rid, enabled: bool, begin: f32, shadow: f32, length: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, f32, f32, f32,);
            let args = (decal, enabled, begin, shadow, length,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(863usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_distance_fade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn light_set_reverse_cull_face_mode(&mut self, light: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (light, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(864usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_reverse_cull_face_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn light_set_shadow_caster_mask(&mut self, light: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (light, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(865usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_shadow_caster_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn light_set_bake_mode(&mut self, light: Rid, bake_mode: crate::classes::rendering_server::LightBakeMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::LightBakeMode,);
            let args = (light, bake_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(866usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_bake_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn light_set_max_sdfgi_cascade(&mut self, light: Rid, cascade: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (light, cascade,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(867usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_set_max_sdfgi_cascade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn light_omni_set_shadow_mode(&mut self, light: Rid, mode: crate::classes::rendering_server::LightOmniShadowMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::LightOmniShadowMode,);
            let args = (light, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(868usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_omni_set_shadow_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn light_directional_set_shadow_mode(&mut self, light: Rid, mode: crate::classes::rendering_server::LightDirectionalShadowMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::LightDirectionalShadowMode,);
            let args = (light, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(869usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_directional_set_shadow_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn light_directional_set_blend_splits(&mut self, light: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (light, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(870usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_directional_set_blend_splits", Some(self.__validated_obj()), args,)
            }
        }
        pub fn light_directional_set_sky_mode(&mut self, light: Rid, mode: crate::classes::rendering_server::LightDirectionalSkyMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::LightDirectionalSkyMode,);
            let args = (light, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(871usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_directional_set_sky_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn light_projectors_set_filter(&mut self, filter: crate::classes::rendering_server::LightProjectorFilter,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::LightProjectorFilter,);
            let args = (filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(872usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "light_projectors_set_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmaps_set_bicubic_filter(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(873usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmaps_set_bicubic_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn positional_soft_shadow_filter_set_quality(&mut self, quality: crate::classes::rendering_server::ShadowQuality,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::ShadowQuality,);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(874usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "positional_soft_shadow_filter_set_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn directional_soft_shadow_filter_set_quality(&mut self, quality: crate::classes::rendering_server::ShadowQuality,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::ShadowQuality,);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(875usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "directional_soft_shadow_filter_set_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn directional_shadow_atlas_set_size(&mut self, size: i32, is_16bits: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (size, is_16bits,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(876usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "directional_shadow_atlas_set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(877usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_set_update_mode(&mut self, probe: Rid, mode: crate::classes::rendering_server::ReflectionProbeUpdateMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ReflectionProbeUpdateMode,);
            let args = (probe, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(878usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_set_intensity(&mut self, probe: Rid, intensity: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (probe, intensity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(879usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_intensity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_set_blend_distance(&mut self, probe: Rid, blend_distance: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (probe, blend_distance,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(880usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_blend_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_set_ambient_mode(&mut self, probe: Rid, mode: crate::classes::rendering_server::ReflectionProbeAmbientMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ReflectionProbeAmbientMode,);
            let args = (probe, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(881usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_ambient_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_set_ambient_color(&mut self, probe: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (probe, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(882usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_ambient_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_set_ambient_energy(&mut self, probe: Rid, energy: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (probe, energy,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(883usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_ambient_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_set_max_distance(&mut self, probe: Rid, distance: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (probe, distance,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(884usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_max_distance", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_set_size(&mut self, probe: Rid, size: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (probe, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(885usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_set_origin_offset(&mut self, probe: Rid, offset: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (probe, offset,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(886usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_origin_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_set_as_interior(&mut self, probe: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (probe, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(887usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_as_interior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_set_enable_box_projection(&mut self, probe: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (probe, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(888usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_enable_box_projection", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_set_enable_shadows(&mut self, probe: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (probe, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(889usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_enable_shadows", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_set_cull_mask(&mut self, probe: Rid, layers: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (probe, layers,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(890usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_set_reflection_mask(&mut self, probe: Rid, layers: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (probe, layers,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(891usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_reflection_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_set_resolution(&mut self, probe: Rid, resolution: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (probe, resolution,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(892usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_resolution", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reflection_probe_set_mesh_lod_threshold(&mut self, probe: Rid, pixels: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (probe, pixels,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(893usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "reflection_probe_set_mesh_lod_threshold", Some(self.__validated_obj()), args,)
            }
        }
        pub fn decal_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(894usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn decal_set_size(&mut self, decal: Rid, size: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (decal, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(895usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn decal_set_texture(&mut self, decal: Rid, type_: crate::classes::rendering_server::DecalTexture, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::DecalTexture, Rid,);
            let args = (decal, type_, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(896usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn decal_set_emission_energy(&mut self, decal: Rid, energy: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (decal, energy,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(897usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_emission_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn decal_set_albedo_mix(&mut self, decal: Rid, albedo_mix: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (decal, albedo_mix,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(898usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_albedo_mix", Some(self.__validated_obj()), args,)
            }
        }
        pub fn decal_set_modulate(&mut self, decal: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (decal, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(899usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn decal_set_cull_mask(&mut self, decal: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (decal, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(900usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn decal_set_distance_fade(&mut self, decal: Rid, enabled: bool, begin: f32, length: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, f32, f32,);
            let args = (decal, enabled, begin, length,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(901usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_distance_fade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn decal_set_fade(&mut self, decal: Rid, above: f32, below: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32, f32,);
            let args = (decal, above, below,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(902usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_fade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn decal_set_normal_fade(&mut self, decal: Rid, fade: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (decal, fade,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(903usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decal_set_normal_fade", Some(self.__validated_obj()), args,)
            }
        }
        pub fn decals_set_filter(&mut self, filter: crate::classes::rendering_server::DecalFilter,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::DecalFilter,);
            let args = (filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(904usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "decals_set_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn gi_set_use_half_resolution(&mut self, half_resolution: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (half_resolution,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(905usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "gi_set_use_half_resolution", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(906usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_allocate_data(&mut self, voxel_gi: Rid, to_cell_xform: Transform3D, aabb: Aabb, octree_size: Vector3i, octree_cells: &PackedByteArray, data_cells: &PackedByteArray, distance_field: &PackedByteArray, level_counts: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (Rid, Transform3D, Aabb, Vector3i, RefArg < 'a0, PackedByteArray >, RefArg < 'a1, PackedByteArray >, RefArg < 'a2, PackedByteArray >, RefArg < 'a3, PackedInt32Array >,);
            let args = (voxel_gi, to_cell_xform, aabb, octree_size, RefArg::new(octree_cells), RefArg::new(data_cells), RefArg::new(distance_field), RefArg::new(level_counts),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(907usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_allocate_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_get_octree_size(&self, voxel_gi: Rid,) -> Vector3i {
            type CallRet = Vector3i;
            type CallParams = (Rid,);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(908usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_get_octree_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_get_octree_cells(&self, voxel_gi: Rid,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (Rid,);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(909usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_get_octree_cells", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_get_data_cells(&self, voxel_gi: Rid,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (Rid,);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(910usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_get_data_cells", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_get_distance_field(&self, voxel_gi: Rid,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (Rid,);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(911usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_get_distance_field", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_get_level_counts(&self, voxel_gi: Rid,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid,);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(912usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_get_level_counts", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_get_to_cell_xform(&self, voxel_gi: Rid,) -> Transform3D {
            type CallRet = Transform3D;
            type CallParams = (Rid,);
            let args = (voxel_gi,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(913usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_get_to_cell_xform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_set_dynamic_range(&mut self, voxel_gi: Rid, range: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (voxel_gi, range,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(914usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_dynamic_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_set_propagation(&mut self, voxel_gi: Rid, amount: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (voxel_gi, amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(915usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_propagation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_set_energy(&mut self, voxel_gi: Rid, energy: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (voxel_gi, energy,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(916usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_set_baked_exposure_normalization(&mut self, voxel_gi: Rid, baked_exposure: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (voxel_gi, baked_exposure,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(917usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_baked_exposure_normalization", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_set_bias(&mut self, voxel_gi: Rid, bias: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (voxel_gi, bias,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(918usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_set_normal_bias(&mut self, voxel_gi: Rid, bias: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (voxel_gi, bias,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(919usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_normal_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_set_interior(&mut self, voxel_gi: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (voxel_gi, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(920usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_interior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_set_use_two_bounces(&mut self, voxel_gi: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (voxel_gi, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(921usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_use_two_bounces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn voxel_gi_set_quality(&mut self, quality: crate::classes::rendering_server::VoxelGiQuality,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::VoxelGiQuality,);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(922usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "voxel_gi_set_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(923usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_set_textures(&mut self, lightmap: Rid, light: Rid, uses_sh: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, bool,);
            let args = (lightmap, light, uses_sh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(924usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_set_textures", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_set_probe_bounds(&mut self, lightmap: Rid, bounds: Aabb,) {
            type CallRet = ();
            type CallParams = (Rid, Aabb,);
            let args = (lightmap, bounds,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(925usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_set_probe_bounds", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_set_probe_interior(&mut self, lightmap: Rid, interior: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (lightmap, interior,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(926usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_set_probe_interior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_set_probe_capture_data(&mut self, lightmap: Rid, points: &PackedVector3Array, point_sh: &PackedColorArray, tetrahedra: &PackedInt32Array, bsp_tree: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (Rid, RefArg < 'a0, PackedVector3Array >, RefArg < 'a1, PackedColorArray >, RefArg < 'a2, PackedInt32Array >, RefArg < 'a3, PackedInt32Array >,);
            let args = (lightmap, RefArg::new(points), RefArg::new(point_sh), RefArg::new(tetrahedra), RefArg::new(bsp_tree),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(927usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_set_probe_capture_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_get_probe_capture_points(&self, lightmap: Rid,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams = (Rid,);
            let args = (lightmap,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(928usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_get_probe_capture_points", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_get_probe_capture_sh(&self, lightmap: Rid,) -> PackedColorArray {
            type CallRet = PackedColorArray;
            type CallParams = (Rid,);
            let args = (lightmap,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(929usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_get_probe_capture_sh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_get_probe_capture_tetrahedra(&self, lightmap: Rid,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid,);
            let args = (lightmap,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(930usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_get_probe_capture_tetrahedra", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_get_probe_capture_bsp_tree(&self, lightmap: Rid,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = (Rid,);
            let args = (lightmap,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(931usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_get_probe_capture_bsp_tree", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_set_baked_exposure_normalization(&mut self, lightmap: Rid, baked_exposure: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (lightmap, baked_exposure,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(932usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_set_baked_exposure_normalization", Some(self.__validated_obj()), args,)
            }
        }
        pub fn lightmap_set_probe_capture_update_speed(&mut self, speed: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (speed,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(933usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "lightmap_set_probe_capture_update_speed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(934usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_mode(&mut self, particles: Rid, mode: crate::classes::rendering_server::ParticlesMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ParticlesMode,);
            let args = (particles, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(935usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_emitting(&mut self, particles: Rid, emitting: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (particles, emitting,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(936usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_emitting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_get_emitting(&mut self, particles: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(937usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_get_emitting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_amount(&mut self, particles: Rid, amount: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (particles, amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(938usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_amount", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_amount_ratio(&mut self, particles: Rid, ratio: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles, ratio,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(939usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_amount_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_lifetime(&mut self, particles: Rid, lifetime: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (particles, lifetime,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(940usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_lifetime", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_one_shot(&mut self, particles: Rid, one_shot: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (particles, one_shot,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(941usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_one_shot", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_pre_process_time(&mut self, particles: Rid, time: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (particles, time,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(942usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_pre_process_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_request_process_time(&mut self, particles: Rid, time: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles, time,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(943usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_request_process_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_explosiveness_ratio(&mut self, particles: Rid, ratio: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles, ratio,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(944usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_explosiveness_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_randomness_ratio(&mut self, particles: Rid, ratio: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles, ratio,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(945usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_randomness_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_interp_to_end(&mut self, particles: Rid, factor: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles, factor,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(946usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_interp_to_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_emitter_velocity(&mut self, particles: Rid, velocity: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (particles, velocity,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(947usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_emitter_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_custom_aabb(&mut self, particles: Rid, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Rid, Aabb,);
            let args = (particles, aabb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(948usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_speed_scale(&mut self, particles: Rid, scale: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64,);
            let args = (particles, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(949usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_use_local_coordinates(&mut self, particles: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (particles, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(950usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_use_local_coordinates", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_process_material(&mut self, particles: Rid, material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (particles, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(951usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_process_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_fixed_fps(&mut self, particles: Rid, fps: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (particles, fps,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(952usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_fixed_fps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_interpolate(&mut self, particles: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (particles, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(953usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_interpolate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_fractional_delta(&mut self, particles: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (particles, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(954usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_fractional_delta", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_collision_base_size(&mut self, particles: Rid, size: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(955usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_collision_base_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_transform_align(&mut self, particles: Rid, align: crate::classes::rendering_server::ParticlesTransformAlign,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ParticlesTransformAlign,);
            let args = (particles, align,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(956usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_transform_align", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_trails(&mut self, particles: Rid, enable: bool, length_sec: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, f32,);
            let args = (particles, enable, length_sec,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(957usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_trails", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_trail_bind_poses(&mut self, particles: Rid, bind_poses: &Array < Transform3D >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Array < Transform3D > >,);
            let args = (particles, RefArg::new(bind_poses),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(958usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_trail_bind_poses", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_is_inactive(&mut self, particles: Rid,) -> bool {
            type CallRet = bool;
            type CallParams = (Rid,);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(959usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_is_inactive", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_request_process(&mut self, particles: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(960usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_request_process", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_restart(&mut self, particles: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(961usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_restart", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_subemitter(&mut self, particles: Rid, subemitter_particles: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (particles, subemitter_particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(962usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_subemitter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_emit(&mut self, particles: Rid, transform: Transform3D, velocity: Vector3, color: Color, custom: Color, emit_flags: u32,) {
            type CallRet = ();
            type CallParams = (Rid, Transform3D, Vector3, Color, Color, u32,);
            let args = (particles, transform, velocity, color, custom, emit_flags,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(963usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_emit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_draw_order(&mut self, particles: Rid, order: crate::classes::rendering_server::ParticlesDrawOrder,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ParticlesDrawOrder,);
            let args = (particles, order,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(964usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_draw_order", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_draw_passes(&mut self, particles: Rid, count: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (particles, count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(965usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_draw_passes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_draw_pass_mesh(&mut self, particles: Rid, pass: i32, mesh: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Rid,);
            let args = (particles, pass, mesh,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(966usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_draw_pass_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_get_current_aabb(&mut self, particles: Rid,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = (Rid,);
            let args = (particles,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(967usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_get_current_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_set_emission_transform(&mut self, particles: Rid, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform3D,);
            let args = (particles, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(968usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_set_emission_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_collision_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(969usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_collision_set_collision_type(&mut self, particles_collision: Rid, type_: crate::classes::rendering_server::ParticlesCollisionType,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ParticlesCollisionType,);
            let args = (particles_collision, type_,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(970usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_collision_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_collision_set_cull_mask(&mut self, particles_collision: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (particles_collision, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(971usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_collision_set_sphere_radius(&mut self, particles_collision: Rid, radius: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles_collision, radius,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(972usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_sphere_radius", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_collision_set_box_extents(&mut self, particles_collision: Rid, extents: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (particles_collision, extents,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(973usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_box_extents", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_collision_set_attractor_strength(&mut self, particles_collision: Rid, strength: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles_collision, strength,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(974usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_attractor_strength", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_collision_set_attractor_directionality(&mut self, particles_collision: Rid, amount: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles_collision, amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(975usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_attractor_directionality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_collision_set_attractor_attenuation(&mut self, particles_collision: Rid, curve: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (particles_collision, curve,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(976usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_attractor_attenuation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_collision_set_field_texture(&mut self, particles_collision: Rid, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (particles_collision, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(977usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_field_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_collision_height_field_update(&mut self, particles_collision: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (particles_collision,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(978usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_height_field_update", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_collision_set_height_field_resolution(&mut self, particles_collision: Rid, resolution: crate::classes::rendering_server::ParticlesCollisionHeightfieldResolution,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ParticlesCollisionHeightfieldResolution,);
            let args = (particles_collision, resolution,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(979usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_height_field_resolution", Some(self.__validated_obj()), args,)
            }
        }
        pub fn particles_collision_set_height_field_mask(&mut self, particles_collision: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (particles_collision, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(980usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "particles_collision_set_height_field_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn fog_volume_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(981usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "fog_volume_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn fog_volume_set_shape(&mut self, fog_volume: Rid, shape: crate::classes::rendering_server::FogVolumeShape,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::FogVolumeShape,);
            let args = (fog_volume, shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(982usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "fog_volume_set_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn fog_volume_set_size(&mut self, fog_volume: Rid, size: Vector3,) {
            type CallRet = ();
            type CallParams = (Rid, Vector3,);
            let args = (fog_volume, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(983usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "fog_volume_set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn fog_volume_set_material(&mut self, fog_volume: Rid, material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (fog_volume, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(984usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "fog_volume_set_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn visibility_notifier_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(985usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "visibility_notifier_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn visibility_notifier_set_aabb(&mut self, notifier: Rid, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Rid, Aabb,);
            let args = (notifier, aabb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(986usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "visibility_notifier_set_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn visibility_notifier_set_callbacks(&mut self, notifier: Rid, enter_callable: &Callable, exit_callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, RefArg < 'a0, Callable >, RefArg < 'a1, Callable >,);
            let args = (notifier, RefArg::new(enter_callable), RefArg::new(exit_callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(987usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "visibility_notifier_set_callbacks", Some(self.__validated_obj()), args,)
            }
        }
        pub fn occluder_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(988usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "occluder_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn occluder_set_mesh(&mut self, occluder: Rid, vertices: &PackedVector3Array, indices: &PackedInt32Array,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, RefArg < 'a0, PackedVector3Array >, RefArg < 'a1, PackedInt32Array >,);
            let args = (occluder, RefArg::new(vertices), RefArg::new(indices),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(989usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "occluder_set_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn camera_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(990usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn camera_set_perspective(&mut self, camera: Rid, fovy_degrees: f32, z_near: f32, z_far: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32, f32, f32,);
            let args = (camera, fovy_degrees, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(991usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_perspective", Some(self.__validated_obj()), args,)
            }
        }
        pub fn camera_set_orthogonal(&mut self, camera: Rid, size: f32, z_near: f32, z_far: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32, f32, f32,);
            let args = (camera, size, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(992usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_orthogonal", Some(self.__validated_obj()), args,)
            }
        }
        pub fn camera_set_frustum(&mut self, camera: Rid, size: f32, offset: Vector2, z_near: f32, z_far: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32, Vector2, f32, f32,);
            let args = (camera, size, offset, z_near, z_far,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(993usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_frustum", Some(self.__validated_obj()), args,)
            }
        }
        pub fn camera_set_transform(&mut self, camera: Rid, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform3D,);
            let args = (camera, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(994usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn camera_set_cull_mask(&mut self, camera: Rid, layers: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (camera, layers,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(995usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn camera_set_environment(&mut self, camera: Rid, env: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (camera, env,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(996usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_environment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn camera_set_camera_attributes(&mut self, camera: Rid, effects: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (camera, effects,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(997usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_camera_attributes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn camera_set_compositor(&mut self, camera: Rid, compositor: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (camera, compositor,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(998usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_compositor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn camera_set_use_vertical_aspect(&mut self, camera: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (camera, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(999usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_set_use_vertical_aspect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1000usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_use_xr(&mut self, viewport: Rid, use_xr: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, use_xr,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1001usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_use_xr", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_size(&mut self, viewport: Rid, width: i32, height: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (viewport, width, height,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1002usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_active(&mut self, viewport: Rid, active: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, active,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1003usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_parent_viewport(&mut self, viewport: Rid, parent_viewport: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (viewport, parent_viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1004usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_parent_viewport", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn viewport_attach_to_screen_full(&mut self, viewport: Rid, rect: Rect2, screen: i32,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, i32,);
            let args = (viewport, rect, screen,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1005usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_attach_to_screen", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::viewport_attach_to_screen_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn viewport_attach_to_screen(&mut self, viewport: Rid,) {
            self.viewport_attach_to_screen_ex(viewport,) . done()
        }
        #[inline]
        pub fn viewport_attach_to_screen_ex < 'a > (&'a mut self, viewport: Rid,) -> ExViewportAttachToScreen < 'a > {
            ExViewportAttachToScreen::new(self, viewport,)
        }
        pub fn viewport_set_render_direct_to_screen(&mut self, viewport: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1006usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_render_direct_to_screen", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_canvas_cull_mask(&mut self, viewport: Rid, canvas_cull_mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (viewport, canvas_cull_mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1007usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_canvas_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_scaling_3d_mode(&mut self, viewport: Rid, scaling_3d_mode: crate::classes::rendering_server::ViewportScaling3DMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportScaling3DMode,);
            let args = (viewport, scaling_3d_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1008usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_scaling_3d_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_scaling_3d_scale(&mut self, viewport: Rid, scale: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (viewport, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1009usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_scaling_3d_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_fsr_sharpness(&mut self, viewport: Rid, sharpness: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (viewport, sharpness,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1010usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_fsr_sharpness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_texture_mipmap_bias(&mut self, viewport: Rid, mipmap_bias: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (viewport, mipmap_bias,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1011usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_texture_mipmap_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_anisotropic_filtering_level(&mut self, viewport: Rid, anisotropic_filtering_level: crate::classes::rendering_server::ViewportAnisotropicFiltering,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportAnisotropicFiltering,);
            let args = (viewport, anisotropic_filtering_level,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1012usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_anisotropic_filtering_level", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_update_mode(&mut self, viewport: Rid, update_mode: crate::classes::rendering_server::ViewportUpdateMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportUpdateMode,);
            let args = (viewport, update_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1013usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_get_update_mode(&self, viewport: Rid,) -> crate::classes::rendering_server::ViewportUpdateMode {
            type CallRet = crate::classes::rendering_server::ViewportUpdateMode;
            type CallParams = (Rid,);
            let args = (viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1014usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_get_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_clear_mode(&mut self, viewport: Rid, clear_mode: crate::classes::rendering_server::ViewportClearMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportClearMode,);
            let args = (viewport, clear_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1015usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_clear_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_get_render_target(&self, viewport: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1016usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_get_render_target", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_get_texture(&self, viewport: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid,);
            let args = (viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1017usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_get_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_disable_3d(&mut self, viewport: Rid, disable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, disable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1018usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_disable_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_disable_2d(&mut self, viewport: Rid, disable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, disable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1019usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_disable_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_environment_mode(&mut self, viewport: Rid, mode: crate::classes::rendering_server::ViewportEnvironmentMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportEnvironmentMode,);
            let args = (viewport, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1020usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_environment_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_attach_camera(&mut self, viewport: Rid, camera: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (viewport, camera,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1021usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_attach_camera", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_scenario(&mut self, viewport: Rid, scenario: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (viewport, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1022usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_scenario", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_attach_canvas(&mut self, viewport: Rid, canvas: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (viewport, canvas,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1023usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_attach_canvas", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_remove_canvas(&mut self, viewport: Rid, canvas: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (viewport, canvas,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1024usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_remove_canvas", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_snap_2d_transforms_to_pixel(&mut self, viewport: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1025usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_snap_2d_transforms_to_pixel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_snap_2d_vertices_to_pixel(&mut self, viewport: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1026usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_snap_2d_vertices_to_pixel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_default_canvas_item_texture_filter(&mut self, viewport: Rid, filter: crate::classes::rendering_server::CanvasItemTextureFilter,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasItemTextureFilter,);
            let args = (viewport, filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1027usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_default_canvas_item_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_default_canvas_item_texture_repeat(&mut self, viewport: Rid, repeat: crate::classes::rendering_server::CanvasItemTextureRepeat,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasItemTextureRepeat,);
            let args = (viewport, repeat,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1028usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_default_canvas_item_texture_repeat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_canvas_transform(&mut self, viewport: Rid, canvas: Rid, offset: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Transform2D,);
            let args = (viewport, canvas, offset,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1029usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_canvas_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_canvas_stacking(&mut self, viewport: Rid, canvas: Rid, layer: i32, sublayer: i32,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, i32, i32,);
            let args = (viewport, canvas, layer, sublayer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1030usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_canvas_stacking", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_transparent_background(&mut self, viewport: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1031usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_transparent_background", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_global_canvas_transform(&mut self, viewport: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (viewport, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1032usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_global_canvas_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_sdf_oversize_and_scale(&mut self, viewport: Rid, oversize: crate::classes::rendering_server::ViewportSdfOversize, scale: crate::classes::rendering_server::ViewportSdfScale,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportSdfOversize, crate::classes::rendering_server::ViewportSdfScale,);
            let args = (viewport, oversize, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1033usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_sdf_oversize_and_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn viewport_set_positional_shadow_atlas_size_full(&mut self, viewport: Rid, size: i32, use_16_bits: bool,) {
            type CallRet = ();
            type CallParams = (Rid, i32, bool,);
            let args = (viewport, size, use_16_bits,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1034usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_positional_shadow_atlas_size", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::viewport_set_positional_shadow_atlas_size_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn viewport_set_positional_shadow_atlas_size(&mut self, viewport: Rid, size: i32,) {
            self.viewport_set_positional_shadow_atlas_size_ex(viewport, size,) . done()
        }
        #[inline]
        pub fn viewport_set_positional_shadow_atlas_size_ex < 'a > (&'a mut self, viewport: Rid, size: i32,) -> ExViewportSetPositionalShadowAtlasSize < 'a > {
            ExViewportSetPositionalShadowAtlasSize::new(self, viewport, size,)
        }
        pub fn viewport_set_positional_shadow_atlas_quadrant_subdivision(&mut self, viewport: Rid, quadrant: i32, subdivision: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (viewport, quadrant, subdivision,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1035usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_positional_shadow_atlas_quadrant_subdivision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_msaa_3d(&mut self, viewport: Rid, msaa: crate::classes::rendering_server::ViewportMsaa,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportMsaa,);
            let args = (viewport, msaa,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1036usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_msaa_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_msaa_2d(&mut self, viewport: Rid, msaa: crate::classes::rendering_server::ViewportMsaa,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportMsaa,);
            let args = (viewport, msaa,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1037usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_msaa_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_use_hdr_2d(&mut self, viewport: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1038usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_use_hdr_2d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_screen_space_aa(&mut self, viewport: Rid, mode: crate::classes::rendering_server::ViewportScreenSpaceAa,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportScreenSpaceAa,);
            let args = (viewport, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1039usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_screen_space_aa", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_use_taa(&mut self, viewport: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1040usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_use_taa", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_use_debanding(&mut self, viewport: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1041usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_use_debanding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_use_occlusion_culling(&mut self, viewport: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1042usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_use_occlusion_culling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_occlusion_rays_per_thread(&mut self, rays_per_thread: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (rays_per_thread,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1043usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_occlusion_rays_per_thread", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_occlusion_culling_build_quality(&mut self, quality: crate::classes::rendering_server::ViewportOcclusionCullingBuildQuality,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::ViewportOcclusionCullingBuildQuality,);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1044usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_occlusion_culling_build_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_get_render_info(&mut self, viewport: Rid, type_: crate::classes::rendering_server::ViewportRenderInfoType, info: crate::classes::rendering_server::ViewportRenderInfo,) -> i32 {
            type CallRet = i32;
            type CallParams = (Rid, crate::classes::rendering_server::ViewportRenderInfoType, crate::classes::rendering_server::ViewportRenderInfo,);
            let args = (viewport, type_, info,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1045usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_get_render_info", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_debug_draw(&mut self, viewport: Rid, draw: crate::classes::rendering_server::ViewportDebugDraw,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportDebugDraw,);
            let args = (viewport, draw,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1046usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_debug_draw", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_measure_render_time(&mut self, viewport: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (viewport, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1047usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_measure_render_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_get_measured_render_time_cpu(&self, viewport: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1048usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_get_measured_render_time_cpu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_get_measured_render_time_gpu(&self, viewport: Rid,) -> f64 {
            type CallRet = f64;
            type CallParams = (Rid,);
            let args = (viewport,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1049usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_get_measured_render_time_gpu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_vrs_mode(&mut self, viewport: Rid, mode: crate::classes::rendering_server::ViewportVrsMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportVrsMode,);
            let args = (viewport, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1050usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_vrs_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_vrs_update_mode(&mut self, viewport: Rid, mode: crate::classes::rendering_server::ViewportVrsUpdateMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ViewportVrsUpdateMode,);
            let args = (viewport, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1051usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_vrs_update_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn viewport_set_vrs_texture(&mut self, viewport: Rid, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (viewport, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1052usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "viewport_set_vrs_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn sky_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1053usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "sky_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn sky_set_radiance_size(&mut self, sky: Rid, radiance_size: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (sky, radiance_size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1054usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "sky_set_radiance_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn sky_set_mode(&mut self, sky: Rid, mode: crate::classes::rendering_server::SkyMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::SkyMode,);
            let args = (sky, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1055usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "sky_set_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn sky_set_material(&mut self, sky: Rid, material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (sky, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1056usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "sky_set_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn sky_bake_panorama(&mut self, sky: Rid, energy: f32, bake_irradiance: bool, size: Vector2i,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (Rid, f32, bool, Vector2i,);
            let args = (sky, energy, bake_irradiance, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1057usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "sky_bake_panorama", Some(self.__validated_obj()), args,)
            }
        }
        pub fn compositor_effect_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1058usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "compositor_effect_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn compositor_effect_set_enabled(&mut self, effect: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (effect, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1059usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "compositor_effect_set_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn compositor_effect_set_callback(&mut self, effect: Rid, callback_type: crate::classes::rendering_server::CompositorEffectCallbackType, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, crate::classes::rendering_server::CompositorEffectCallbackType, RefArg < 'a0, Callable >,);
            let args = (effect, callback_type, RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1060usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "compositor_effect_set_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn compositor_effect_set_flag(&mut self, effect: Rid, flag: crate::classes::rendering_server::CompositorEffectFlags, set: bool,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CompositorEffectFlags, bool,);
            let args = (effect, flag, set,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1061usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "compositor_effect_set_flag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn compositor_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1062usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "compositor_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn compositor_set_compositor_effects(&mut self, compositor: Rid, effects: &Array < Rid >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Array < Rid > >,);
            let args = (compositor, RefArg::new(effects),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1063usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "compositor_set_compositor_effects", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1064usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_background(&mut self, env: Rid, bg: crate::classes::rendering_server::EnvironmentBg,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::EnvironmentBg,);
            let args = (env, bg,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1065usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_background", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_camera_id(&mut self, env: Rid, id: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (env, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1066usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_camera_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_sky(&mut self, env: Rid, sky: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (env, sky,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1067usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_sky", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_sky_custom_fov(&mut self, env: Rid, scale: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (env, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1068usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_sky_custom_fov", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_sky_orientation(&mut self, env: Rid, orientation: Basis,) {
            type CallRet = ();
            type CallParams = (Rid, Basis,);
            let args = (env, orientation,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1069usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_sky_orientation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_bg_color(&mut self, env: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (env, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1070usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_bg_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_bg_energy(&mut self, env: Rid, multiplier: f32, exposure_value: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32, f32,);
            let args = (env, multiplier, exposure_value,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1071usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_bg_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_canvas_max_layer(&mut self, env: Rid, max_layer: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (env, max_layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1072usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_canvas_max_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn environment_set_ambient_light_full(&mut self, env: Rid, color: Color, ambient: crate::classes::rendering_server::EnvironmentAmbientSource, energy: f32, sky_contribution: f32, reflection_source: crate::classes::rendering_server::EnvironmentReflectionSource,) {
            type CallRet = ();
            type CallParams = (Rid, Color, crate::classes::rendering_server::EnvironmentAmbientSource, f32, f32, crate::classes::rendering_server::EnvironmentReflectionSource,);
            let args = (env, color, ambient, energy, sky_contribution, reflection_source,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1073usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_ambient_light", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::environment_set_ambient_light_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn environment_set_ambient_light(&mut self, env: Rid, color: Color,) {
            self.environment_set_ambient_light_ex(env, color,) . done()
        }
        #[inline]
        pub fn environment_set_ambient_light_ex < 'a > (&'a mut self, env: Rid, color: Color,) -> ExEnvironmentSetAmbientLight < 'a > {
            ExEnvironmentSetAmbientLight::new(self, env, color,)
        }
        pub fn environment_set_glow(&mut self, env: Rid, enable: bool, levels: &PackedFloat32Array, intensity: f32, strength: f32, mix: f32, bloom_threshold: f32, blend_mode: crate::classes::rendering_server::EnvironmentGlowBlendMode, hdr_bleed_threshold: f32, hdr_bleed_scale: f32, hdr_luminance_cap: f32, glow_map_strength: f32, glow_map: Rid,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, bool, RefArg < 'a0, PackedFloat32Array >, f32, f32, f32, f32, crate::classes::rendering_server::EnvironmentGlowBlendMode, f32, f32, f32, f32, Rid,);
            let args = (env, enable, RefArg::new(levels), intensity, strength, mix, bloom_threshold, blend_mode, hdr_bleed_threshold, hdr_bleed_scale, hdr_luminance_cap, glow_map_strength, glow_map,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1074usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_glow", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_tonemap(&mut self, env: Rid, tone_mapper: crate::classes::rendering_server::EnvironmentToneMapper, exposure: f32, white: f32,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::EnvironmentToneMapper, f32, f32,);
            let args = (env, tone_mapper, exposure, white,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1075usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_tonemap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_adjustment(&mut self, env: Rid, enable: bool, brightness: f32, contrast: f32, saturation: f32, use_1d_color_correction: bool, color_correction: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, bool, f32, f32, f32, bool, Rid,);
            let args = (env, enable, brightness, contrast, saturation, use_1d_color_correction, color_correction,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1076usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_adjustment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_ssr(&mut self, env: Rid, enable: bool, max_steps: i32, fade_in: f32, fade_out: f32, depth_tolerance: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, i32, f32, f32, f32,);
            let args = (env, enable, max_steps, fade_in, fade_out, depth_tolerance,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1077usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_ssr", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_ssao(&mut self, env: Rid, enable: bool, radius: f32, intensity: f32, power: f32, detail: f32, horizon: f32, sharpness: f32, light_affect: f32, ao_channel_affect: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, f32, f32, f32, f32, f32, f32, f32, f32,);
            let args = (env, enable, radius, intensity, power, detail, horizon, sharpness, light_affect, ao_channel_affect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1078usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_ssao", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn environment_set_fog_full(&mut self, env: Rid, enable: bool, light_color: Color, light_energy: f32, sun_scatter: f32, density: f32, height: f32, height_density: f32, aerial_perspective: f32, sky_affect: f32, fog_mode: crate::classes::rendering_server::EnvironmentFogMode,) {
            type CallRet = ();
            type CallParams = (Rid, bool, Color, f32, f32, f32, f32, f32, f32, f32, crate::classes::rendering_server::EnvironmentFogMode,);
            let args = (env, enable, light_color, light_energy, sun_scatter, density, height, height_density, aerial_perspective, sky_affect, fog_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1079usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_fog", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::environment_set_fog_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn environment_set_fog(&mut self, env: Rid, enable: bool, light_color: Color, light_energy: f32, sun_scatter: f32, density: f32, height: f32, height_density: f32, aerial_perspective: f32, sky_affect: f32,) {
            self.environment_set_fog_ex(env, enable, light_color, light_energy, sun_scatter, density, height, height_density, aerial_perspective, sky_affect,) . done()
        }
        #[inline]
        pub fn environment_set_fog_ex < 'a > (&'a mut self, env: Rid, enable: bool, light_color: Color, light_energy: f32, sun_scatter: f32, density: f32, height: f32, height_density: f32, aerial_perspective: f32, sky_affect: f32,) -> ExEnvironmentSetFog < 'a > {
            ExEnvironmentSetFog::new(self, env, enable, light_color, light_energy, sun_scatter, density, height, height_density, aerial_perspective, sky_affect,)
        }
        pub fn environment_set_fog_depth(&mut self, env: Rid, curve: f32, begin: f32, end: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32, f32, f32,);
            let args = (env, curve, begin, end,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1080usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_fog_depth", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_sdfgi(&mut self, env: Rid, enable: bool, cascades: i32, min_cell_size: f32, y_scale: crate::classes::rendering_server::EnvironmentSdfgiYScale, use_occlusion: bool, bounce_feedback: f32, read_sky: bool, energy: f32, normal_bias: f32, probe_bias: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, i32, f32, crate::classes::rendering_server::EnvironmentSdfgiYScale, bool, f32, bool, f32, f32, f32,);
            let args = (env, enable, cascades, min_cell_size, y_scale, use_occlusion, bounce_feedback, read_sky, energy, normal_bias, probe_bias,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1081usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_sdfgi", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_volumetric_fog(&mut self, env: Rid, enable: bool, density: f32, albedo: Color, emission: Color, emission_energy: f32, anisotropy: f32, length: f32, p_detail_spread: f32, gi_inject: f32, temporal_reprojection: bool, temporal_reprojection_amount: f32, ambient_inject: f32, sky_affect: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, f32, Color, Color, f32, f32, f32, f32, f32, bool, f32, f32, f32,);
            let args = (env, enable, density, albedo, emission, emission_energy, anisotropy, length, p_detail_spread, gi_inject, temporal_reprojection, temporal_reprojection_amount, ambient_inject, sky_affect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1082usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_volumetric_fog", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_glow_set_use_bicubic_upscale(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1083usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_glow_set_use_bicubic_upscale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_ssr_roughness_quality(&mut self, quality: crate::classes::rendering_server::EnvironmentSsrRoughnessQuality,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::EnvironmentSsrRoughnessQuality,);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1084usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_ssr_roughness_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_ssao_quality(&mut self, quality: crate::classes::rendering_server::EnvironmentSsaoQuality, half_size: bool, adaptive_target: f32, blur_passes: i32, fadeout_from: f32, fadeout_to: f32,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::EnvironmentSsaoQuality, bool, f32, i32, f32, f32,);
            let args = (quality, half_size, adaptive_target, blur_passes, fadeout_from, fadeout_to,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1085usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_ssao_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_ssil_quality(&mut self, quality: crate::classes::rendering_server::EnvironmentSsilQuality, half_size: bool, adaptive_target: f32, blur_passes: i32, fadeout_from: f32, fadeout_to: f32,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::EnvironmentSsilQuality, bool, f32, i32, f32, f32,);
            let args = (quality, half_size, adaptive_target, blur_passes, fadeout_from, fadeout_to,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1086usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_ssil_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_sdfgi_ray_count(&mut self, ray_count: crate::classes::rendering_server::EnvironmentSdfgiRayCount,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::EnvironmentSdfgiRayCount,);
            let args = (ray_count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1087usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_sdfgi_ray_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_sdfgi_frames_to_converge(&mut self, frames: crate::classes::rendering_server::EnvironmentSdfgiFramesToConverge,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::EnvironmentSdfgiFramesToConverge,);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1088usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_sdfgi_frames_to_converge", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_sdfgi_frames_to_update_light(&mut self, frames: crate::classes::rendering_server::EnvironmentSdfgiFramesToUpdateLight,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::EnvironmentSdfgiFramesToUpdateLight,);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1089usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_sdfgi_frames_to_update_light", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_volumetric_fog_volume_size(&mut self, size: i32, depth: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (size, depth,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1090usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_volumetric_fog_volume_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_set_volumetric_fog_filter_active(&mut self, active: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1091usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_set_volumetric_fog_filter_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn environment_bake_panorama(&mut self, environment: Rid, bake_irradiance: bool, size: Vector2i,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (Rid, bool, Vector2i,);
            let args = (environment, bake_irradiance, size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1092usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "environment_bake_panorama", Some(self.__validated_obj()), args,)
            }
        }
        pub fn screen_space_roughness_limiter_set_active(&mut self, enable: bool, amount: f32, limit: f32,) {
            type CallRet = ();
            type CallParams = (bool, f32, f32,);
            let args = (enable, amount, limit,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1093usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "screen_space_roughness_limiter_set_active", Some(self.__validated_obj()), args,)
            }
        }
        pub fn sub_surface_scattering_set_quality(&mut self, quality: crate::classes::rendering_server::SubSurfaceScatteringQuality,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::SubSurfaceScatteringQuality,);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1094usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "sub_surface_scattering_set_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn sub_surface_scattering_set_scale(&mut self, scale: f32, depth_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32, f32,);
            let args = (scale, depth_scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1095usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "sub_surface_scattering_set_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn camera_attributes_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1096usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_attributes_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn camera_attributes_set_dof_blur_quality(&mut self, quality: crate::classes::rendering_server::DofBlurQuality, use_jitter: bool,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::DofBlurQuality, bool,);
            let args = (quality, use_jitter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1097usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_attributes_set_dof_blur_quality", Some(self.__validated_obj()), args,)
            }
        }
        pub fn camera_attributes_set_dof_blur_bokeh_shape(&mut self, shape: crate::classes::rendering_server::DofBokehShape,) {
            type CallRet = ();
            type CallParams = (crate::classes::rendering_server::DofBokehShape,);
            let args = (shape,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1098usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_attributes_set_dof_blur_bokeh_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn camera_attributes_set_dof_blur(&mut self, camera_attributes: Rid, far_enable: bool, far_distance: f32, far_transition: f32, near_enable: bool, near_distance: f32, near_transition: f32, amount: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, f32, f32, bool, f32, f32, f32,);
            let args = (camera_attributes, far_enable, far_distance, far_transition, near_enable, near_distance, near_transition, amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1099usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_attributes_set_dof_blur", Some(self.__validated_obj()), args,)
            }
        }
        pub fn camera_attributes_set_exposure(&mut self, camera_attributes: Rid, multiplier: f32, normalization: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32, f32,);
            let args = (camera_attributes, multiplier, normalization,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_attributes_set_exposure", Some(self.__validated_obj()), args,)
            }
        }
        pub fn camera_attributes_set_auto_exposure(&mut self, camera_attributes: Rid, enable: bool, min_sensitivity: f32, max_sensitivity: f32, speed: f32, scale: f32,) {
            type CallRet = ();
            type CallParams = (Rid, bool, f32, f32, f32, f32,);
            let args = (camera_attributes, enable, min_sensitivity, max_sensitivity, speed, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1101usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "camera_attributes_set_auto_exposure", Some(self.__validated_obj()), args,)
            }
        }
        pub fn scenario_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1102usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "scenario_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn scenario_set_environment(&mut self, scenario: Rid, environment: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (scenario, environment,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "scenario_set_environment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn scenario_set_fallback_environment(&mut self, scenario: Rid, environment: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (scenario, environment,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "scenario_set_fallback_environment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn scenario_set_camera_attributes(&mut self, scenario: Rid, effects: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (scenario, effects,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "scenario_set_camera_attributes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn scenario_set_compositor(&mut self, scenario: Rid, compositor: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (scenario, compositor,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "scenario_set_compositor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_create2(&mut self, base: Rid, scenario: Rid,) -> Rid {
            type CallRet = Rid;
            type CallParams = (Rid, Rid,);
            let args = (base, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_create2", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_set_base(&mut self, instance: Rid, base: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (instance, base,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_base", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_set_scenario(&mut self, instance: Rid, scenario: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (instance, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_scenario", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_set_layer_mask(&mut self, instance: Rid, mask: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (instance, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_layer_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_set_pivot_data(&mut self, instance: Rid, sorting_offset: f32, use_aabb_center: bool,) {
            type CallRet = ();
            type CallParams = (Rid, f32, bool,);
            let args = (instance, sorting_offset, use_aabb_center,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_pivot_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_set_transform(&mut self, instance: Rid, transform: Transform3D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform3D,);
            let args = (instance, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_attach_object_instance_id(&mut self, instance: Rid, id: u64,) {
            type CallRet = ();
            type CallParams = (Rid, u64,);
            let args = (instance, id,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_attach_object_instance_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_set_blend_shape_weight(&mut self, instance: Rid, shape: i32, weight: f32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, f32,);
            let args = (instance, shape, weight,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_blend_shape_weight", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_set_surface_override_material(&mut self, instance: Rid, surface: i32, material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, i32, Rid,);
            let args = (instance, surface, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_surface_override_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_set_visible(&mut self, instance: Rid, visible: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (instance, visible,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_geometry_set_transparency(&mut self, instance: Rid, transparency: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (instance, transparency,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_transparency", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_teleport(&mut self, instance: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_teleport", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_set_custom_aabb(&mut self, instance: Rid, aabb: Aabb,) {
            type CallRet = ();
            type CallParams = (Rid, Aabb,);
            let args = (instance, aabb,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_custom_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_attach_skeleton(&mut self, instance: Rid, skeleton: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (instance, skeleton,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_attach_skeleton", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_set_extra_visibility_margin(&mut self, instance: Rid, margin: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (instance, margin,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_extra_visibility_margin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_set_visibility_parent(&mut self, instance: Rid, parent: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (instance, parent,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_visibility_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_set_ignore_culling(&mut self, instance: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (instance, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_set_ignore_culling", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_geometry_set_flag(&mut self, instance: Rid, flag: crate::classes::rendering_server::InstanceFlags, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::InstanceFlags, bool,);
            let args = (instance, flag, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_flag", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_geometry_set_cast_shadows_setting(&mut self, instance: Rid, shadow_casting_setting: crate::classes::rendering_server::ShadowCastingSetting,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::ShadowCastingSetting,);
            let args = (instance, shadow_casting_setting,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_cast_shadows_setting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_geometry_set_material_override(&mut self, instance: Rid, material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (instance, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_material_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_geometry_set_material_overlay(&mut self, instance: Rid, material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (instance, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_material_overlay", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_geometry_set_visibility_range(&mut self, instance: Rid, min: f32, max: f32, min_margin: f32, max_margin: f32, fade_mode: crate::classes::rendering_server::VisibilityRangeFadeMode,) {
            type CallRet = ();
            type CallParams = (Rid, f32, f32, f32, f32, crate::classes::rendering_server::VisibilityRangeFadeMode,);
            let args = (instance, min, max, min_margin, max_margin, fade_mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_visibility_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_geometry_set_lightmap(&mut self, instance: Rid, lightmap: Rid, lightmap_uv_scale: Rect2, lightmap_slice: i32,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Rect2, i32,);
            let args = (instance, lightmap, lightmap_uv_scale, lightmap_slice,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_lightmap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_geometry_set_lod_bias(&mut self, instance: Rid, lod_bias: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (instance, lod_bias,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_lod_bias", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_geometry_set_shader_parameter(&mut self, instance: Rid, parameter: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (instance, parameter.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_set_shader_parameter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_geometry_get_shader_parameter(&self, instance: Rid, parameter: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, StringName >,);
            let args = (instance, parameter.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_get_shader_parameter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_geometry_get_shader_parameter_default_value(&self, instance: Rid, parameter: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, StringName >,);
            let args = (instance, parameter.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_get_shader_parameter_default_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn instance_geometry_get_shader_parameter_list(&self, instance: Rid,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = (Rid,);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instance_geometry_get_shader_parameter_list", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn instances_cull_aabb_full(&self, aabb: Aabb, scenario: Rid,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams = (Aabb, Rid,);
            let args = (aabb, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instances_cull_aabb", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::instances_cull_aabb_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn instances_cull_aabb(&self, aabb: Aabb,) -> PackedInt64Array {
            self.instances_cull_aabb_ex(aabb,) . done()
        }
        #[inline]
        pub fn instances_cull_aabb_ex < 'a > (&'a self, aabb: Aabb,) -> ExInstancesCullAabb < 'a > {
            ExInstancesCullAabb::new(self, aabb,)
        }
        pub(crate) fn instances_cull_ray_full(&self, from: Vector3, to: Vector3, scenario: Rid,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams = (Vector3, Vector3, Rid,);
            let args = (from, to, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instances_cull_ray", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::instances_cull_ray_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn instances_cull_ray(&self, from: Vector3, to: Vector3,) -> PackedInt64Array {
            self.instances_cull_ray_ex(from, to,) . done()
        }
        #[inline]
        pub fn instances_cull_ray_ex < 'a > (&'a self, from: Vector3, to: Vector3,) -> ExInstancesCullRay < 'a > {
            ExInstancesCullRay::new(self, from, to,)
        }
        pub(crate) fn instances_cull_convex_full(&self, convex: RefArg < Array < Plane > >, scenario: Rid,) -> PackedInt64Array {
            type CallRet = PackedInt64Array;
            type CallParams < 'a0, > = (RefArg < 'a0, Array < Plane > >, Rid,);
            let args = (convex, scenario,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "instances_cull_convex", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::instances_cull_convex_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn instances_cull_convex(&self, convex: &Array < Plane >,) -> PackedInt64Array {
            self.instances_cull_convex_ex(convex,) . done()
        }
        #[inline]
        pub fn instances_cull_convex_ex < 'a > (&'a self, convex: &'a Array < Plane >,) -> ExInstancesCullConvex < 'a > {
            ExInstancesCullConvex::new(self, convex,)
        }
        pub fn bake_render_uv2(&mut self, base: Rid, material_overrides: &Array < Rid >, image_size: Vector2i,) -> Array < Gd < crate::classes::Image > > {
            type CallRet = Array < Gd < crate::classes::Image > >;
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, Array < Rid > >, Vector2i,);
            let args = (base, RefArg::new(material_overrides), image_size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "bake_render_uv2", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_set_item_mirroring(&mut self, canvas: Rid, item: Rid, mirroring: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Vector2,);
            let args = (canvas, item, mirroring,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_set_item_mirroring", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_set_item_repeat(&mut self, item: Rid, repeat_size: Vector2, repeat_times: i32,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, i32,);
            let args = (item, repeat_size, repeat_times,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1142usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_set_item_repeat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_set_modulate(&mut self, canvas: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (canvas, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1143usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_set_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_set_disable_scale(&mut self, disable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1144usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_set_disable_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_texture_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1145usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_texture_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_texture_set_channel(&mut self, canvas_texture: Rid, channel: crate::classes::rendering_server::CanvasTextureChannel, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasTextureChannel, Rid,);
            let args = (canvas_texture, channel, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1146usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_texture_set_channel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_texture_set_shading_parameters(&mut self, canvas_texture: Rid, base_color: Color, shininess: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Color, f32,);
            let args = (canvas_texture, base_color, shininess,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1147usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_texture_set_shading_parameters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_texture_set_texture_filter(&mut self, canvas_texture: Rid, filter: crate::classes::rendering_server::CanvasItemTextureFilter,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasItemTextureFilter,);
            let args = (canvas_texture, filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1148usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_texture_set_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_texture_set_texture_repeat(&mut self, canvas_texture: Rid, repeat: crate::classes::rendering_server::CanvasItemTextureRepeat,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasItemTextureRepeat,);
            let args = (canvas_texture, repeat,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1149usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_texture_set_texture_repeat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1150usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_parent(&mut self, item: Rid, parent: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (item, parent,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1151usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_default_texture_filter(&mut self, item: Rid, filter: crate::classes::rendering_server::CanvasItemTextureFilter,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasItemTextureFilter,);
            let args = (item, filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1152usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_default_texture_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_default_texture_repeat(&mut self, item: Rid, repeat: crate::classes::rendering_server::CanvasItemTextureRepeat,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasItemTextureRepeat,);
            let args = (item, repeat,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1153usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_default_texture_repeat", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_visible(&mut self, item: Rid, visible: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, visible,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1154usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_light_mask(&mut self, item: Rid, mask: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (item, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1155usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_light_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_visibility_layer(&mut self, item: Rid, visibility_layer: u32,) {
            type CallRet = ();
            type CallParams = (Rid, u32,);
            let args = (item, visibility_layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1156usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_visibility_layer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_transform(&mut self, item: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (item, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1157usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_clip(&mut self, item: Rid, clip: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, clip,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1158usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_clip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_distance_field_mode(&mut self, item: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1159usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_distance_field_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn canvas_item_set_custom_rect_full(&mut self, item: Rid, use_custom_rect: bool, rect: Rect2,) {
            type CallRet = ();
            type CallParams = (Rid, bool, Rect2,);
            let args = (item, use_custom_rect, rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1160usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_custom_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::canvas_item_set_custom_rect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn canvas_item_set_custom_rect(&mut self, item: Rid, use_custom_rect: bool,) {
            self.canvas_item_set_custom_rect_ex(item, use_custom_rect,) . done()
        }
        #[inline]
        pub fn canvas_item_set_custom_rect_ex < 'a > (&'a mut self, item: Rid, use_custom_rect: bool,) -> ExCanvasItemSetCustomRect < 'a > {
            ExCanvasItemSetCustomRect::new(self, item, use_custom_rect,)
        }
        pub fn canvas_item_set_modulate(&mut self, item: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (item, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1161usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_self_modulate(&mut self, item: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (item, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1162usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_self_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_draw_behind_parent(&mut self, item: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_draw_behind_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_interpolated(&mut self, item: Rid, interpolated: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, interpolated,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_interpolated", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_reset_physics_interpolation(&mut self, item: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (item,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_reset_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_transform_physics_interpolation(&mut self, item: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (item, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_transform_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn canvas_item_add_line_full(&mut self, item: Rid, from: Vector2, to: Vector2, color: Color, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, Vector2, Color, f32, bool,);
            let args = (item, from, to, color, width, antialiased,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_line", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::canvas_item_add_line_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn canvas_item_add_line(&mut self, item: Rid, from: Vector2, to: Vector2, color: Color,) {
            self.canvas_item_add_line_ex(item, from, to, color,) . done()
        }
        #[inline]
        pub fn canvas_item_add_line_ex < 'a > (&'a mut self, item: Rid, from: Vector2, to: Vector2, color: Color,) -> ExCanvasItemAddLine < 'a > {
            ExCanvasItemAddLine::new(self, item, from, to, color,)
        }
        pub(crate) fn canvas_item_add_polyline_full(&mut self, item: Rid, points: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedColorArray >, f32, bool,);
            let args = (item, points, colors, width, antialiased,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_polyline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::canvas_item_add_polyline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn canvas_item_add_polyline(&mut self, item: Rid, points: &PackedVector2Array, colors: &PackedColorArray,) {
            self.canvas_item_add_polyline_ex(item, points, colors,) . done()
        }
        #[inline]
        pub fn canvas_item_add_polyline_ex < 'a > (&'a mut self, item: Rid, points: &'a PackedVector2Array, colors: &'a PackedColorArray,) -> ExCanvasItemAddPolyline < 'a > {
            ExCanvasItemAddPolyline::new(self, item, points, colors,)
        }
        pub(crate) fn canvas_item_add_multiline_full(&mut self, item: Rid, points: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, width: f32, antialiased: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedColorArray >, f32, bool,);
            let args = (item, points, colors, width, antialiased,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_multiline", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::canvas_item_add_multiline_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn canvas_item_add_multiline(&mut self, item: Rid, points: &PackedVector2Array, colors: &PackedColorArray,) {
            self.canvas_item_add_multiline_ex(item, points, colors,) . done()
        }
        #[inline]
        pub fn canvas_item_add_multiline_ex < 'a > (&'a mut self, item: Rid, points: &'a PackedVector2Array, colors: &'a PackedColorArray,) -> ExCanvasItemAddMultiline < 'a > {
            ExCanvasItemAddMultiline::new(self, item, points, colors,)
        }
        pub(crate) fn canvas_item_add_rect_full(&mut self, item: Rid, rect: Rect2, color: Color, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, Color, bool,);
            let args = (item, rect, color, antialiased,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::canvas_item_add_rect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn canvas_item_add_rect(&mut self, item: Rid, rect: Rect2, color: Color,) {
            self.canvas_item_add_rect_ex(item, rect, color,) . done()
        }
        #[inline]
        pub fn canvas_item_add_rect_ex < 'a > (&'a mut self, item: Rid, rect: Rect2, color: Color,) -> ExCanvasItemAddRect < 'a > {
            ExCanvasItemAddRect::new(self, item, rect, color,)
        }
        pub(crate) fn canvas_item_add_circle_full(&mut self, item: Rid, pos: Vector2, radius: f32, color: Color, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2, f32, Color, bool,);
            let args = (item, pos, radius, color, antialiased,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_circle", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::canvas_item_add_circle_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn canvas_item_add_circle(&mut self, item: Rid, pos: Vector2, radius: f32, color: Color,) {
            self.canvas_item_add_circle_ex(item, pos, radius, color,) . done()
        }
        #[inline]
        pub fn canvas_item_add_circle_ex < 'a > (&'a mut self, item: Rid, pos: Vector2, radius: f32, color: Color,) -> ExCanvasItemAddCircle < 'a > {
            ExCanvasItemAddCircle::new(self, item, pos, radius, color,)
        }
        pub(crate) fn canvas_item_add_texture_rect_full(&mut self, item: Rid, rect: Rect2, texture: Rid, tile: bool, modulate: Color, transpose: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, Rid, bool, Color, bool,);
            let args = (item, rect, texture, tile, modulate, transpose,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_texture_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::canvas_item_add_texture_rect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn canvas_item_add_texture_rect(&mut self, item: Rid, rect: Rect2, texture: Rid,) {
            self.canvas_item_add_texture_rect_ex(item, rect, texture,) . done()
        }
        #[inline]
        pub fn canvas_item_add_texture_rect_ex < 'a > (&'a mut self, item: Rid, rect: Rect2, texture: Rid,) -> ExCanvasItemAddTextureRect < 'a > {
            ExCanvasItemAddTextureRect::new(self, item, rect, texture,)
        }
        pub(crate) fn canvas_item_add_msdf_texture_rect_region_full(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2, modulate: Color, outline_size: i32, px_range: f32, scale: f32,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, Rid, Rect2, Color, i32, f32, f32,);
            let args = (item, rect, texture, src_rect, modulate, outline_size, px_range, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_msdf_texture_rect_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::canvas_item_add_msdf_texture_rect_region_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn canvas_item_add_msdf_texture_rect_region(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) {
            self.canvas_item_add_msdf_texture_rect_region_ex(item, rect, texture, src_rect,) . done()
        }
        #[inline]
        pub fn canvas_item_add_msdf_texture_rect_region_ex < 'a > (&'a mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) -> ExCanvasItemAddMsdfTextureRectRegion < 'a > {
            ExCanvasItemAddMsdfTextureRectRegion::new(self, item, rect, texture, src_rect,)
        }
        pub fn canvas_item_add_lcd_texture_rect_region(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2, modulate: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, Rid, Rect2, Color,);
            let args = (item, rect, texture, src_rect, modulate,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_lcd_texture_rect_region", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn canvas_item_add_texture_rect_region_full(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, Rid, Rect2, Color, bool, bool,);
            let args = (item, rect, texture, src_rect, modulate, transpose, clip_uv,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_texture_rect_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::canvas_item_add_texture_rect_region_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn canvas_item_add_texture_rect_region(&mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) {
            self.canvas_item_add_texture_rect_region_ex(item, rect, texture, src_rect,) . done()
        }
        #[inline]
        pub fn canvas_item_add_texture_rect_region_ex < 'a > (&'a mut self, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) -> ExCanvasItemAddTextureRectRegion < 'a > {
            ExCanvasItemAddTextureRectRegion::new(self, item, rect, texture, src_rect,)
        }
        pub(crate) fn canvas_item_add_nine_patch_full(&mut self, item: Rid, rect: Rect2, source: Rect2, texture: Rid, topleft: Vector2, bottomright: Vector2, x_axis_mode: crate::classes::rendering_server::NinePatchAxisMode, y_axis_mode: crate::classes::rendering_server::NinePatchAxisMode, draw_center: bool, modulate: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Rect2, Rect2, Rid, Vector2, Vector2, crate::classes::rendering_server::NinePatchAxisMode, crate::classes::rendering_server::NinePatchAxisMode, bool, Color,);
            let args = (item, rect, source, texture, topleft, bottomright, x_axis_mode, y_axis_mode, draw_center, modulate,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_nine_patch", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::canvas_item_add_nine_patch_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn canvas_item_add_nine_patch(&mut self, item: Rid, rect: Rect2, source: Rect2, texture: Rid, topleft: Vector2, bottomright: Vector2,) {
            self.canvas_item_add_nine_patch_ex(item, rect, source, texture, topleft, bottomright,) . done()
        }
        #[inline]
        pub fn canvas_item_add_nine_patch_ex < 'a > (&'a mut self, item: Rid, rect: Rect2, source: Rect2, texture: Rid, topleft: Vector2, bottomright: Vector2,) -> ExCanvasItemAddNinePatch < 'a > {
            ExCanvasItemAddNinePatch::new(self, item, rect, source, texture, topleft, bottomright,)
        }
        pub fn canvas_item_add_primitive(&mut self, item: Rid, points: &PackedVector2Array, colors: &PackedColorArray, uvs: &PackedVector2Array, texture: Rid,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (Rid, RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedColorArray >, RefArg < 'a2, PackedVector2Array >, Rid,);
            let args = (item, RefArg::new(points), RefArg::new(colors), RefArg::new(uvs), texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_primitive", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn canvas_item_add_polygon_full(&mut self, item: Rid, points: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, uvs: RefArg < PackedVector2Array >, texture: Rid,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (Rid, RefArg < 'a0, PackedVector2Array >, RefArg < 'a1, PackedColorArray >, RefArg < 'a2, PackedVector2Array >, Rid,);
            let args = (item, points, colors, uvs, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_polygon", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::canvas_item_add_polygon_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn canvas_item_add_polygon(&mut self, item: Rid, points: &PackedVector2Array, colors: &PackedColorArray,) {
            self.canvas_item_add_polygon_ex(item, points, colors,) . done()
        }
        #[inline]
        pub fn canvas_item_add_polygon_ex < 'a > (&'a mut self, item: Rid, points: &'a PackedVector2Array, colors: &'a PackedColorArray,) -> ExCanvasItemAddPolygon < 'a > {
            ExCanvasItemAddPolygon::new(self, item, points, colors,)
        }
        pub(crate) fn canvas_item_add_triangle_array_full(&mut self, item: Rid, indices: RefArg < PackedInt32Array >, points: RefArg < PackedVector2Array >, colors: RefArg < PackedColorArray >, uvs: RefArg < PackedVector2Array >, bones: RefArg < PackedInt32Array >, weights: RefArg < PackedFloat32Array >, texture: Rid, count: i32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, 'a4, 'a5, > = (Rid, RefArg < 'a0, PackedInt32Array >, RefArg < 'a1, PackedVector2Array >, RefArg < 'a2, PackedColorArray >, RefArg < 'a3, PackedVector2Array >, RefArg < 'a4, PackedInt32Array >, RefArg < 'a5, PackedFloat32Array >, Rid, i32,);
            let args = (item, indices, points, colors, uvs, bones, weights, texture, count,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_triangle_array", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::canvas_item_add_triangle_array_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn canvas_item_add_triangle_array(&mut self, item: Rid, indices: &PackedInt32Array, points: &PackedVector2Array, colors: &PackedColorArray,) {
            self.canvas_item_add_triangle_array_ex(item, indices, points, colors,) . done()
        }
        #[inline]
        pub fn canvas_item_add_triangle_array_ex < 'a > (&'a mut self, item: Rid, indices: &'a PackedInt32Array, points: &'a PackedVector2Array, colors: &'a PackedColorArray,) -> ExCanvasItemAddTriangleArray < 'a > {
            ExCanvasItemAddTriangleArray::new(self, item, indices, points, colors,)
        }
        pub(crate) fn canvas_item_add_mesh_full(&mut self, item: Rid, mesh: Rid, transform: Transform2D, modulate: Color, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Transform2D, Color, Rid,);
            let args = (item, mesh, transform, modulate, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::canvas_item_add_mesh_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn canvas_item_add_mesh(&mut self, item: Rid, mesh: Rid,) {
            self.canvas_item_add_mesh_ex(item, mesh,) . done()
        }
        #[inline]
        pub fn canvas_item_add_mesh_ex < 'a > (&'a mut self, item: Rid, mesh: Rid,) -> ExCanvasItemAddMesh < 'a > {
            ExCanvasItemAddMesh::new(self, item, mesh,)
        }
        pub(crate) fn canvas_item_add_multimesh_full(&mut self, item: Rid, mesh: Rid, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Rid,);
            let args = (item, mesh, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_multimesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::canvas_item_add_multimesh_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn canvas_item_add_multimesh(&mut self, item: Rid, mesh: Rid,) {
            self.canvas_item_add_multimesh_ex(item, mesh,) . done()
        }
        #[inline]
        pub fn canvas_item_add_multimesh_ex < 'a > (&'a mut self, item: Rid, mesh: Rid,) -> ExCanvasItemAddMultimesh < 'a > {
            ExCanvasItemAddMultimesh::new(self, item, mesh,)
        }
        pub fn canvas_item_add_particles(&mut self, item: Rid, particles: Rid, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid, Rid,);
            let args = (item, particles, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_particles", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_add_set_transform(&mut self, item: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (item, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_add_clip_ignore(&mut self, item: Rid, ignore: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, ignore,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_clip_ignore", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn canvas_item_add_animation_slice_full(&mut self, item: Rid, animation_length: f64, slice_begin: f64, slice_end: f64, offset: f64,) {
            type CallRet = ();
            type CallParams = (Rid, f64, f64, f64, f64,);
            let args = (item, animation_length, slice_begin, slice_end, offset,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_add_animation_slice", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::canvas_item_add_animation_slice_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn canvas_item_add_animation_slice(&mut self, item: Rid, animation_length: f64, slice_begin: f64, slice_end: f64,) {
            self.canvas_item_add_animation_slice_ex(item, animation_length, slice_begin, slice_end,) . done()
        }
        #[inline]
        pub fn canvas_item_add_animation_slice_ex < 'a > (&'a mut self, item: Rid, animation_length: f64, slice_begin: f64, slice_end: f64,) -> ExCanvasItemAddAnimationSlice < 'a > {
            ExCanvasItemAddAnimationSlice::new(self, item, animation_length, slice_begin, slice_end,)
        }
        pub fn canvas_item_set_sort_children_by_y(&mut self, item: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_sort_children_by_y", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_z_index(&mut self, item: Rid, z_index: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (item, z_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_z_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_z_as_relative_to_parent(&mut self, item: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_z_as_relative_to_parent", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_copy_to_backbuffer(&mut self, item: Rid, enabled: bool, rect: Rect2,) {
            type CallRet = ();
            type CallParams = (Rid, bool, Rect2,);
            let args = (item, enabled, rect,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_copy_to_backbuffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_attach_skeleton(&mut self, item: Rid, skeleton: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (item, skeleton,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_attach_skeleton", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_clear(&mut self, item: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (item,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_draw_index(&mut self, item: Rid, index: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (item, index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_draw_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_material(&mut self, item: Rid, material: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (item, material,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_use_parent_material(&mut self, item: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (item, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_use_parent_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_instance_shader_parameter(&mut self, instance: Rid, parameter: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (instance, parameter.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_instance_shader_parameter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_get_instance_shader_parameter(&self, instance: Rid, parameter: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, StringName >,);
            let args = (instance, parameter.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_get_instance_shader_parameter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_get_instance_shader_parameter_default_value(&self, instance: Rid, parameter: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (Rid, CowArg < 'a0, StringName >,);
            let args = (instance, parameter.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_get_instance_shader_parameter_default_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_get_instance_shader_parameter_list(&self, instance: Rid,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = (Rid,);
            let args = (instance,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_get_instance_shader_parameter_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_item_set_visibility_notifier(&mut self, item: Rid, enable: bool, area: Rect2, enter_callable: &Callable, exit_callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (Rid, bool, Rect2, RefArg < 'a0, Callable >, RefArg < 'a1, Callable >,);
            let args = (item, enable, area, RefArg::new(enter_callable), RefArg::new(exit_callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1199usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_visibility_notifier", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn canvas_item_set_canvas_group_mode_full(&mut self, item: Rid, mode: crate::classes::rendering_server::CanvasGroupMode, clear_margin: f32, fit_empty: bool, fit_margin: f32, blur_mipmaps: bool,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasGroupMode, f32, bool, f32, bool,);
            let args = (item, mode, clear_margin, fit_empty, fit_margin, blur_mipmaps,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1200usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_item_set_canvas_group_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::canvas_item_set_canvas_group_mode_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn canvas_item_set_canvas_group_mode(&mut self, item: Rid, mode: crate::classes::rendering_server::CanvasGroupMode,) {
            self.canvas_item_set_canvas_group_mode_ex(item, mode,) . done()
        }
        #[inline]
        pub fn canvas_item_set_canvas_group_mode_ex < 'a > (&'a mut self, item: Rid, mode: crate::classes::rendering_server::CanvasGroupMode,) -> ExCanvasItemSetCanvasGroupMode < 'a > {
            ExCanvasItemSetCanvasGroupMode::new(self, item, mode,)
        }
        pub fn debug_canvas_item_get_rect(&mut self, item: Rid,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (Rid,);
            let args = (item,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1201usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "debug_canvas_item_get_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1202usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_attach_to_canvas(&mut self, light: Rid, canvas: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (light, canvas,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1203usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_attach_to_canvas", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_enabled(&mut self, light: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (light, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1204usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_texture_scale(&mut self, light: Rid, scale: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (light, scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1205usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_texture_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_transform(&mut self, light: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (light, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1206usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_texture(&mut self, light: Rid, texture: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (light, texture,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1207usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_texture_offset(&mut self, light: Rid, offset: Vector2,) {
            type CallRet = ();
            type CallParams = (Rid, Vector2,);
            let args = (light, offset,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1208usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_texture_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_color(&mut self, light: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (light, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1209usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_height(&mut self, light: Rid, height: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (light, height,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_energy(&mut self, light: Rid, energy: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (light, energy,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_energy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_z_range(&mut self, light: Rid, min_z: i32, max_z: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (light, min_z, max_z,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_z_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_layer_range(&mut self, light: Rid, min_layer: i32, max_layer: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32, i32,);
            let args = (light, min_layer, max_layer,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_layer_range", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_item_cull_mask(&mut self, light: Rid, mask: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (light, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_item_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_item_shadow_cull_mask(&mut self, light: Rid, mask: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (light, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_item_shadow_cull_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_mode(&mut self, light: Rid, mode: crate::classes::rendering_server::CanvasLightMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasLightMode,);
            let args = (light, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_shadow_enabled(&mut self, light: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (light, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_shadow_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_shadow_filter(&mut self, light: Rid, filter: crate::classes::rendering_server::CanvasLightShadowFilter,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasLightShadowFilter,);
            let args = (light, filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_shadow_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_shadow_color(&mut self, light: Rid, color: Color,) {
            type CallRet = ();
            type CallParams = (Rid, Color,);
            let args = (light, color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_shadow_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_shadow_smooth(&mut self, light: Rid, smooth: f32,) {
            type CallRet = ();
            type CallParams = (Rid, f32,);
            let args = (light, smooth,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_shadow_smooth", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_blend_mode(&mut self, light: Rid, mode: crate::classes::rendering_server::CanvasLightBlendMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasLightBlendMode,);
            let args = (light, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_blend_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_set_interpolated(&mut self, light: Rid, interpolated: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (light, interpolated,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_set_interpolated", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_reset_physics_interpolation(&mut self, light: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (light,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_reset_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_transform_physics_interpolation(&mut self, light: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (light, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_transform_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_occluder_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_occluder_attach_to_canvas(&mut self, occluder: Rid, canvas: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (occluder, canvas,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_attach_to_canvas", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_occluder_set_enabled(&mut self, occluder: Rid, enabled: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (occluder, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_set_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_occluder_set_polygon(&mut self, occluder: Rid, polygon: Rid,) {
            type CallRet = ();
            type CallParams = (Rid, Rid,);
            let args = (occluder, polygon,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_set_polygon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_occluder_set_as_sdf_collision(&mut self, occluder: Rid, enable: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (occluder, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_set_as_sdf_collision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_occluder_set_transform(&mut self, occluder: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (occluder, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_set_transform", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_occluder_set_light_mask(&mut self, occluder: Rid, mask: i32,) {
            type CallRet = ();
            type CallParams = (Rid, i32,);
            let args = (occluder, mask,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_set_light_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_occluder_set_interpolated(&mut self, occluder: Rid, interpolated: bool,) {
            type CallRet = ();
            type CallParams = (Rid, bool,);
            let args = (occluder, interpolated,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_set_interpolated", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_occluder_reset_physics_interpolation(&mut self, occluder: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (occluder,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_reset_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_light_occluder_transform_physics_interpolation(&mut self, occluder: Rid, transform: Transform2D,) {
            type CallRet = ();
            type CallParams = (Rid, Transform2D,);
            let args = (occluder, transform,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_light_occluder_transform_physics_interpolation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_occluder_polygon_create(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_occluder_polygon_create", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_occluder_polygon_set_shape(&mut self, occluder_polygon: Rid, shape: &PackedVector2Array, closed: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (Rid, RefArg < 'a0, PackedVector2Array >, bool,);
            let args = (occluder_polygon, RefArg::new(shape), closed,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_occluder_polygon_set_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_occluder_polygon_set_cull_mode(&mut self, occluder_polygon: Rid, mode: crate::classes::rendering_server::CanvasOccluderPolygonCullMode,) {
            type CallRet = ();
            type CallParams = (Rid, crate::classes::rendering_server::CanvasOccluderPolygonCullMode,);
            let args = (occluder_polygon, mode,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_occluder_polygon_set_cull_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn canvas_set_shadow_texture_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "canvas_set_shadow_texture_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_shader_parameter_add(&mut self, name: impl AsArg < StringName >, type_: crate::classes::rendering_server::GlobalShaderParameterType, default_value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, crate::classes::rendering_server::GlobalShaderParameterType, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), type_, RefArg::new(default_value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "global_shader_parameter_add", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_shader_parameter_remove(&mut self, name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "global_shader_parameter_remove", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_shader_parameter_get_list(&self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "global_shader_parameter_get_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_shader_parameter_set(&mut self, name: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "global_shader_parameter_set", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_shader_parameter_set_override(&mut self, name: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "global_shader_parameter_set_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_shader_parameter_get(&self, name: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "global_shader_parameter_get", Some(self.__validated_obj()), args,)
            }
        }
        pub fn global_shader_parameter_get_type(&self, name: impl AsArg < StringName >,) -> crate::classes::rendering_server::GlobalShaderParameterType {
            type CallRet = crate::classes::rendering_server::GlobalShaderParameterType;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "global_shader_parameter_get_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn free_rid(&mut self, rid: Rid,) {
            type CallRet = ();
            type CallParams = (Rid,);
            let args = (rid,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "free_rid", Some(self.__validated_obj()), args,)
            }
        }
        pub fn request_frame_drawn_callback(&mut self, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "request_frame_drawn_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_changed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "has_changed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_rendering_info(&mut self, info: crate::classes::rendering_server::RenderingInfo,) -> u64 {
            type CallRet = u64;
            type CallParams = (crate::classes::rendering_server::RenderingInfo,);
            let args = (info,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_rendering_info", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_video_adapter_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_video_adapter_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_video_adapter_vendor(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_video_adapter_vendor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_video_adapter_type(&self,) -> crate::classes::rendering_device::DeviceType {
            type CallRet = crate::classes::rendering_device::DeviceType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_video_adapter_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_video_adapter_api_version(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_video_adapter_api_version", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_rendering_driver_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_current_rendering_driver_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_rendering_method(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_current_rendering_method", Some(self.__validated_obj()), args,)
            }
        }
        pub fn make_sphere_mesh(&mut self, latitudes: i32, longitudes: i32, radius: f32,) -> Rid {
            type CallRet = Rid;
            type CallParams = (i32, i32, f32,);
            let args = (latitudes, longitudes, radius,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "make_sphere_mesh", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_test_cube(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_test_cube", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_test_texture(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_test_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_white_texture(&mut self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_white_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn set_boot_image_full(&mut self, image: CowArg < Option < Gd < crate::classes::Image > > >, color: Color, scale: bool, use_filter: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >, Color, bool, bool,);
            let args = (image, color, scale, use_filter,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "set_boot_image", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_boot_image_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_boot_image(&mut self, image: impl AsArg < Option < Gd < crate::classes::Image >> >, color: Color, scale: bool,) {
            self.set_boot_image_ex(image, color, scale,) . done()
        }
        #[inline]
        pub fn set_boot_image_ex < 'a > (&'a mut self, image: impl AsArg < Option < Gd < crate::classes::Image >> > + 'a, color: Color, scale: bool,) -> ExSetBootImage < 'a > {
            ExSetBootImage::new(self, image, color, scale,)
        }
        pub fn get_default_clear_color(&mut self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_default_clear_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_default_clear_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "set_default_clear_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_os_feature(&self, feature: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (feature.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "has_os_feature", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_debug_generate_wireframes(&mut self, generate: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (generate,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "set_debug_generate_wireframes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_render_loop_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "is_render_loop_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_render_loop_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "set_render_loop_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frame_setup_time_cpu(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_frame_setup_time_cpu", Some(self.__validated_obj()), args,)
            }
        }
        pub fn force_sync(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "force_sync", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn force_draw_full(&mut self, swap_buffers: bool, frame_step: f64,) {
            type CallRet = ();
            type CallParams = (bool, f64,);
            let args = (swap_buffers, frame_step,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "force_draw", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::force_draw_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn force_draw(&mut self,) {
            self.force_draw_ex() . done()
        }
        #[inline]
        pub fn force_draw_ex < 'a > (&'a mut self,) -> ExForceDraw < 'a > {
            ExForceDraw::new(self,)
        }
        pub fn get_rendering_device(&self,) -> Option < Gd < crate::classes::RenderingDevice > > {
            type CallRet = Option < Gd < crate::classes::RenderingDevice > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "get_rendering_device", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_local_rendering_device(&self,) -> Option < Gd < crate::classes::RenderingDevice > > {
            type CallRet = Option < Gd < crate::classes::RenderingDevice > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "create_local_rendering_device", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_on_render_thread(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "is_on_render_thread", Some(self.__validated_obj()), args,)
            }
        }
        pub fn call_on_render_thread(&mut self, callable: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callable),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "call_on_render_thread", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_feature(&self, feature: crate::classes::rendering_server::Features,) -> bool {
            type CallRet = bool;
            type CallParams = (crate::classes::rendering_server::Features,);
            let args = (feature,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderingServer", "has_feature", Some(self.__validated_obj()), args,)
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
        pub const NO_INDEX_ARRAY: i32 = - 1i32;
        pub const ARRAY_WEIGHTS_SIZE: i32 = 4i32;
        pub const CANVAS_ITEM_Z_MIN: i32 = - 4096i32;
        pub const CANVAS_ITEM_Z_MAX: i32 = 4096i32;
        pub const CANVAS_LAYER_MIN: i32 = - 2147483648i32;
        pub const CANVAS_LAYER_MAX: i32 = 2147483647i32;
        pub const MAX_GLOW_LEVELS: i32 = 7i32;
        pub const MAX_CURSORS: i32 = 8i32;
        pub const MAX_2D_DIRECTIONAL_LIGHTS: i32 = 8i32;
        pub const MAX_MESH_SURFACES: i32 = 256i32;
        pub const MATERIAL_RENDER_PRIORITY_MIN: i32 = - 128i32;
        pub const MATERIAL_RENDER_PRIORITY_MAX: i32 = 127i32;
        pub const ARRAY_CUSTOM_COUNT: i32 = 4i32;
        pub const PARTICLES_EMIT_FLAG_POSITION: i32 = 1i32;
        pub const PARTICLES_EMIT_FLAG_ROTATION_SCALE: i32 = 2i32;
        pub const PARTICLES_EMIT_FLAG_VELOCITY: i32 = 4i32;
        pub const PARTICLES_EMIT_FLAG_COLOR: i32 = 8i32;
        pub const PARTICLES_EMIT_FLAG_CUSTOM: i32 = 16i32;
        
    }
    impl crate::obj::GodotClass for RenderingServer {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("RenderingServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for RenderingServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RenderingServer {
        
    }
    impl crate::obj::Singleton for RenderingServer {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"RenderingServer"))
            }
        }
    }
    impl std::ops::Deref for RenderingServer {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RenderingServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RenderingServer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `RenderingServer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`RenderingServer::texture_create_from_native_handle_ex`][super::RenderingServer::texture_create_from_native_handle_ex]."]
#[must_use]
pub struct ExTextureCreateFromNativeHandle < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, type_: crate::classes::rendering_server::TextureType, format: crate::classes::image::Format, native_handle: u64, width: i32, height: i32, depth: i32, layers: i32, layered_type: crate::classes::rendering_server::TextureLayeredType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureCreateFromNativeHandle < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, type_: crate::classes::rendering_server::TextureType, format: crate::classes::image::Format, native_handle: u64, width: i32, height: i32, depth: i32,) -> Self {
        let layers = 1i32;
        let layered_type = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, type_: type_, format: format, native_handle: native_handle, width: width, height: height, depth: depth, layers: layers, layered_type: layered_type,
        }
    }
    #[inline]
    pub fn layers(self, layers: i32) -> Self {
        Self {
            layers: layers, .. self
        }
    }
    #[inline]
    pub fn layered_type(self, layered_type: crate::classes::rendering_server::TextureLayeredType) -> Self {
        Self {
            layered_type: layered_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, type_, format, native_handle, width, height, depth, layers, layered_type,
        }
        = self;
        re_export::RenderingServer::texture_create_from_native_handle_full(surround_object, type_, format, native_handle, width, height, depth, layers, layered_type,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::texture_rd_create_ex`][super::RenderingServer::texture_rd_create_ex]."]
#[must_use]
pub struct ExTextureRdCreate < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, rd_texture: Rid, layer_type: crate::classes::rendering_server::TextureLayeredType,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureRdCreate < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, rd_texture: Rid,) -> Self {
        let layer_type = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, rd_texture: rd_texture, layer_type: layer_type,
        }
    }
    #[inline]
    pub fn layer_type(self, layer_type: crate::classes::rendering_server::TextureLayeredType) -> Self {
        Self {
            layer_type: layer_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, rd_texture, layer_type,
        }
        = self;
        re_export::RenderingServer::texture_rd_create_full(surround_object, rd_texture, layer_type,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::texture_get_rd_texture_ex`][super::RenderingServer::texture_get_rd_texture_ex]."]
#[must_use]
pub struct ExTextureGetRdTexture < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RenderingServer, texture: Rid, srgb: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureGetRdTexture < 'a > {
    fn new(surround_object: &'a re_export::RenderingServer, texture: Rid,) -> Self {
        let srgb = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture, srgb: srgb,
        }
    }
    #[inline]
    pub fn srgb(self, srgb: bool) -> Self {
        Self {
            srgb: srgb, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, texture, srgb,
        }
        = self;
        re_export::RenderingServer::texture_get_rd_texture_full(surround_object, texture, srgb,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::texture_get_native_handle_ex`][super::RenderingServer::texture_get_native_handle_ex]."]
#[must_use]
pub struct ExTextureGetNativeHandle < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RenderingServer, texture: Rid, srgb: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExTextureGetNativeHandle < 'a > {
    fn new(surround_object: &'a re_export::RenderingServer, texture: Rid,) -> Self {
        let srgb = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture, srgb: srgb,
        }
    }
    #[inline]
    pub fn srgb(self, srgb: bool) -> Self {
        Self {
            srgb: srgb, .. self
        }
    }
    #[inline]
    pub fn done(self) -> u64 {
        let Self {
            _phantom, surround_object, texture, srgb,
        }
        = self;
        re_export::RenderingServer::texture_get_native_handle_full(surround_object, texture, srgb,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::shader_set_default_texture_parameter_ex`][super::RenderingServer::shader_set_default_texture_parameter_ex]."]
#[must_use]
pub struct ExShaderSetDefaultTextureParameter < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, shader: Rid, name: CowArg < 'a, StringName >, texture: Rid, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShaderSetDefaultTextureParameter < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, shader: Rid, name: impl AsArg < StringName > + 'a, texture: Rid,) -> Self {
        let index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shader: shader, name: name.into_arg(), texture: texture, index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, shader, name, texture, index,
        }
        = self;
        re_export::RenderingServer::shader_set_default_texture_parameter_full(surround_object, shader, name, texture, index,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::shader_get_default_texture_parameter_ex`][super::RenderingServer::shader_get_default_texture_parameter_ex]."]
#[must_use]
pub struct ExShaderGetDefaultTextureParameter < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RenderingServer, shader: Rid, name: CowArg < 'a, StringName >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExShaderGetDefaultTextureParameter < 'a > {
    fn new(surround_object: &'a re_export::RenderingServer, shader: Rid, name: impl AsArg < StringName > + 'a,) -> Self {
        let index = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, shader: shader, name: name.into_arg(), index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, shader, name, index,
        }
        = self;
        re_export::RenderingServer::shader_get_default_texture_parameter_full(surround_object, shader, name, index,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::mesh_create_from_surfaces_ex`][super::RenderingServer::mesh_create_from_surfaces_ex]."]
#[must_use]
pub struct ExMeshCreateFromSurfaces < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, surfaces: CowArg < 'a, Array < VarDictionary > >, blend_shape_count: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMeshCreateFromSurfaces < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, surfaces: &'a Array < VarDictionary >,) -> Self {
        let blend_shape_count = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, surfaces: CowArg::Borrowed(surfaces), blend_shape_count: blend_shape_count,
        }
    }
    #[inline]
    pub fn blend_shape_count(self, blend_shape_count: i32) -> Self {
        Self {
            blend_shape_count: blend_shape_count, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, surfaces, blend_shape_count,
        }
        = self;
        re_export::RenderingServer::mesh_create_from_surfaces_full(surround_object, surfaces.cow_as_arg(), blend_shape_count,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::mesh_add_surface_from_arrays_ex`][super::RenderingServer::mesh_add_surface_from_arrays_ex]."]
#[must_use]
pub struct ExMeshAddSurfaceFromArrays < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, mesh: Rid, primitive: crate::classes::rendering_server::PrimitiveType, arrays: CowArg < 'a, VarArray >, blend_shapes: CowArg < 'a, VarArray >, lods: CowArg < 'a, VarDictionary >, compress_format: crate::classes::rendering_server::ArrayFormat,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMeshAddSurfaceFromArrays < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, mesh: Rid, primitive: crate::classes::rendering_server::PrimitiveType, arrays: &'a VarArray,) -> Self {
        let blend_shapes = Array::new();
        let lods = VarDictionary::new();
        let compress_format = crate::obj::EngineBitfield::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mesh: mesh, primitive: primitive, arrays: CowArg::Borrowed(arrays), blend_shapes: CowArg::Owned(blend_shapes), lods: CowArg::Owned(lods), compress_format: compress_format,
        }
    }
    #[inline]
    pub fn blend_shapes(self, blend_shapes: &'a VarArray) -> Self {
        Self {
            blend_shapes: CowArg::Borrowed(blend_shapes), .. self
        }
    }
    #[inline]
    pub fn lods(self, lods: &'a VarDictionary) -> Self {
        Self {
            lods: CowArg::Borrowed(lods), .. self
        }
    }
    #[inline]
    pub fn compress_format(self, compress_format: crate::classes::rendering_server::ArrayFormat) -> Self {
        Self {
            compress_format: compress_format, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, mesh, primitive, arrays, blend_shapes, lods, compress_format,
        }
        = self;
        re_export::RenderingServer::mesh_add_surface_from_arrays_full(surround_object, mesh, primitive, arrays.cow_as_arg(), blend_shapes.cow_as_arg(), lods.cow_as_arg(), compress_format,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::multimesh_allocate_data_ex`][super::RenderingServer::multimesh_allocate_data_ex]."]
#[must_use]
pub struct ExMultimeshAllocateData < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, multimesh: Rid, instances: i32, transform_format: crate::classes::rendering_server::MultimeshTransformFormat, color_format: bool, custom_data_format: bool, use_indirect: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExMultimeshAllocateData < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, multimesh: Rid, instances: i32, transform_format: crate::classes::rendering_server::MultimeshTransformFormat,) -> Self {
        let color_format = false;
        let custom_data_format = false;
        let use_indirect = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, multimesh: multimesh, instances: instances, transform_format: transform_format, color_format: color_format, custom_data_format: custom_data_format, use_indirect: use_indirect,
        }
    }
    #[inline]
    pub fn color_format(self, color_format: bool) -> Self {
        Self {
            color_format: color_format, .. self
        }
    }
    #[inline]
    pub fn custom_data_format(self, custom_data_format: bool) -> Self {
        Self {
            custom_data_format: custom_data_format, .. self
        }
    }
    #[inline]
    pub fn use_indirect(self, use_indirect: bool) -> Self {
        Self {
            use_indirect: use_indirect, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, multimesh, instances, transform_format, color_format, custom_data_format, use_indirect,
        }
        = self;
        re_export::RenderingServer::multimesh_allocate_data_full(surround_object, multimesh, instances, transform_format, color_format, custom_data_format, use_indirect,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::skeleton_allocate_data_ex`][super::RenderingServer::skeleton_allocate_data_ex]."]
#[must_use]
pub struct ExSkeletonAllocateData < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, skeleton: Rid, bones: i32, is_2d_skeleton: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSkeletonAllocateData < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, skeleton: Rid, bones: i32,) -> Self {
        let is_2d_skeleton = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, skeleton: skeleton, bones: bones, is_2d_skeleton: is_2d_skeleton,
        }
    }
    #[inline]
    pub fn is_2d_skeleton(self, is_2d_skeleton: bool) -> Self {
        Self {
            is_2d_skeleton: is_2d_skeleton, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, skeleton, bones, is_2d_skeleton,
        }
        = self;
        re_export::RenderingServer::skeleton_allocate_data_full(surround_object, skeleton, bones, is_2d_skeleton,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::viewport_attach_to_screen_ex`][super::RenderingServer::viewport_attach_to_screen_ex]."]
#[must_use]
pub struct ExViewportAttachToScreen < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, viewport: Rid, rect: Rect2, screen: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExViewportAttachToScreen < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, viewport: Rid,) -> Self {
        let rect = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        let screen = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, viewport: viewport, rect: rect, screen: screen,
        }
    }
    #[inline]
    pub fn rect(self, rect: Rect2) -> Self {
        Self {
            rect: rect, .. self
        }
    }
    #[inline]
    pub fn screen(self, screen: i32) -> Self {
        Self {
            screen: screen, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, viewport, rect, screen,
        }
        = self;
        re_export::RenderingServer::viewport_attach_to_screen_full(surround_object, viewport, rect, screen,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::viewport_set_positional_shadow_atlas_size_ex`][super::RenderingServer::viewport_set_positional_shadow_atlas_size_ex]."]
#[must_use]
pub struct ExViewportSetPositionalShadowAtlasSize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, viewport: Rid, size: i32, use_16_bits: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExViewportSetPositionalShadowAtlasSize < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, viewport: Rid, size: i32,) -> Self {
        let use_16_bits = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, viewport: viewport, size: size, use_16_bits: use_16_bits,
        }
    }
    #[inline]
    pub fn use_16_bits(self, use_16_bits: bool) -> Self {
        Self {
            use_16_bits: use_16_bits, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, viewport, size, use_16_bits,
        }
        = self;
        re_export::RenderingServer::viewport_set_positional_shadow_atlas_size_full(surround_object, viewport, size, use_16_bits,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::environment_set_ambient_light_ex`][super::RenderingServer::environment_set_ambient_light_ex]."]
#[must_use]
pub struct ExEnvironmentSetAmbientLight < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, env: Rid, color: Color, ambient: crate::classes::rendering_server::EnvironmentAmbientSource, energy: f32, sky_contribution: f32, reflection_source: crate::classes::rendering_server::EnvironmentReflectionSource,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExEnvironmentSetAmbientLight < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, env: Rid, color: Color,) -> Self {
        let ambient = crate::obj::EngineEnum::from_ord(0);
        let energy = 1f32;
        let sky_contribution = 0f32;
        let reflection_source = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, env: env, color: color, ambient: ambient, energy: energy, sky_contribution: sky_contribution, reflection_source: reflection_source,
        }
    }
    #[inline]
    pub fn ambient(self, ambient: crate::classes::rendering_server::EnvironmentAmbientSource) -> Self {
        Self {
            ambient: ambient, .. self
        }
    }
    #[inline]
    pub fn energy(self, energy: f32) -> Self {
        Self {
            energy: energy, .. self
        }
    }
    #[inline]
    pub fn sky_contribution(self, sky_contribution: f32) -> Self {
        Self {
            sky_contribution: sky_contribution, .. self
        }
    }
    #[inline]
    pub fn reflection_source(self, reflection_source: crate::classes::rendering_server::EnvironmentReflectionSource) -> Self {
        Self {
            reflection_source: reflection_source, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, env, color, ambient, energy, sky_contribution, reflection_source,
        }
        = self;
        re_export::RenderingServer::environment_set_ambient_light_full(surround_object, env, color, ambient, energy, sky_contribution, reflection_source,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::environment_set_fog_ex`][super::RenderingServer::environment_set_fog_ex]."]
#[must_use]
pub struct ExEnvironmentSetFog < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, env: Rid, enable: bool, light_color: Color, light_energy: f32, sun_scatter: f32, density: f32, height: f32, height_density: f32, aerial_perspective: f32, sky_affect: f32, fog_mode: crate::classes::rendering_server::EnvironmentFogMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExEnvironmentSetFog < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, env: Rid, enable: bool, light_color: Color, light_energy: f32, sun_scatter: f32, density: f32, height: f32, height_density: f32, aerial_perspective: f32, sky_affect: f32,) -> Self {
        let fog_mode = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, env: env, enable: enable, light_color: light_color, light_energy: light_energy, sun_scatter: sun_scatter, density: density, height: height, height_density: height_density, aerial_perspective: aerial_perspective, sky_affect: sky_affect, fog_mode: fog_mode,
        }
    }
    #[inline]
    pub fn fog_mode(self, fog_mode: crate::classes::rendering_server::EnvironmentFogMode) -> Self {
        Self {
            fog_mode: fog_mode, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, env, enable, light_color, light_energy, sun_scatter, density, height, height_density, aerial_perspective, sky_affect, fog_mode,
        }
        = self;
        re_export::RenderingServer::environment_set_fog_full(surround_object, env, enable, light_color, light_energy, sun_scatter, density, height, height_density, aerial_perspective, sky_affect, fog_mode,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::instances_cull_aabb_ex`][super::RenderingServer::instances_cull_aabb_ex]."]
#[must_use]
pub struct ExInstancesCullAabb < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RenderingServer, aabb: Aabb, scenario: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExInstancesCullAabb < 'a > {
    fn new(surround_object: &'a re_export::RenderingServer, aabb: Aabb,) -> Self {
        let scenario = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, aabb: aabb, scenario: scenario,
        }
    }
    #[inline]
    pub fn scenario(self, scenario: Rid) -> Self {
        Self {
            scenario: scenario, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt64Array {
        let Self {
            _phantom, surround_object, aabb, scenario,
        }
        = self;
        re_export::RenderingServer::instances_cull_aabb_full(surround_object, aabb, scenario,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::instances_cull_ray_ex`][super::RenderingServer::instances_cull_ray_ex]."]
#[must_use]
pub struct ExInstancesCullRay < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RenderingServer, from: Vector3, to: Vector3, scenario: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExInstancesCullRay < 'a > {
    fn new(surround_object: &'a re_export::RenderingServer, from: Vector3, to: Vector3,) -> Self {
        let scenario = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from: from, to: to, scenario: scenario,
        }
    }
    #[inline]
    pub fn scenario(self, scenario: Rid) -> Self {
        Self {
            scenario: scenario, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt64Array {
        let Self {
            _phantom, surround_object, from, to, scenario,
        }
        = self;
        re_export::RenderingServer::instances_cull_ray_full(surround_object, from, to, scenario,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::instances_cull_convex_ex`][super::RenderingServer::instances_cull_convex_ex]."]
#[must_use]
pub struct ExInstancesCullConvex < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::RenderingServer, convex: CowArg < 'a, Array < Plane > >, scenario: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExInstancesCullConvex < 'a > {
    fn new(surround_object: &'a re_export::RenderingServer, convex: &'a Array < Plane >,) -> Self {
        let scenario = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, convex: CowArg::Borrowed(convex), scenario: scenario,
        }
    }
    #[inline]
    pub fn scenario(self, scenario: Rid) -> Self {
        Self {
            scenario: scenario, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedInt64Array {
        let Self {
            _phantom, surround_object, convex, scenario,
        }
        = self;
        re_export::RenderingServer::instances_cull_convex_full(surround_object, convex.cow_as_arg(), scenario,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_set_custom_rect_ex`][super::RenderingServer::canvas_item_set_custom_rect_ex]."]
#[must_use]
pub struct ExCanvasItemSetCustomRect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, item: Rid, use_custom_rect: bool, rect: Rect2,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemSetCustomRect < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, use_custom_rect: bool,) -> Self {
        let rect = Rect2::from_components(0 as _, 0 as _, 0 as _, 0 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, use_custom_rect: use_custom_rect, rect: rect,
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
            _phantom, surround_object, item, use_custom_rect, rect,
        }
        = self;
        re_export::RenderingServer::canvas_item_set_custom_rect_full(surround_object, item, use_custom_rect, rect,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_line_ex`][super::RenderingServer::canvas_item_add_line_ex]."]
#[must_use]
pub struct ExCanvasItemAddLine < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, item: Rid, from: Vector2, to: Vector2, color: Color, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddLine < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, from: Vector2, to: Vector2, color: Color,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, from: from, to: to, color: color, width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, from, to, color, width, antialiased,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_line_full(surround_object, item, from, to, color, width, antialiased,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_polyline_ex`][super::RenderingServer::canvas_item_add_polyline_ex]."]
#[must_use]
pub struct ExCanvasItemAddPolyline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, item: Rid, points: CowArg < 'a, PackedVector2Array >, colors: CowArg < 'a, PackedColorArray >, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddPolyline < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, points: &'a PackedVector2Array, colors: &'a PackedColorArray,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, points: CowArg::Borrowed(points), colors: CowArg::Borrowed(colors), width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, points, colors, width, antialiased,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_polyline_full(surround_object, item, points.cow_as_arg(), colors.cow_as_arg(), width, antialiased,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_multiline_ex`][super::RenderingServer::canvas_item_add_multiline_ex]."]
#[must_use]
pub struct ExCanvasItemAddMultiline < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, item: Rid, points: CowArg < 'a, PackedVector2Array >, colors: CowArg < 'a, PackedColorArray >, width: f32, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddMultiline < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, points: &'a PackedVector2Array, colors: &'a PackedColorArray,) -> Self {
        let width = - 1f32;
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, points: CowArg::Borrowed(points), colors: CowArg::Borrowed(colors), width: width, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn width(self, width: f32) -> Self {
        Self {
            width: width, .. self
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, points, colors, width, antialiased,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_multiline_full(surround_object, item, points.cow_as_arg(), colors.cow_as_arg(), width, antialiased,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_rect_ex`][super::RenderingServer::canvas_item_add_rect_ex]."]
#[must_use]
pub struct ExCanvasItemAddRect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, color: Color, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddRect < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, color: Color,) -> Self {
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, rect: rect, color: color, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, rect, color, antialiased,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_rect_full(surround_object, item, rect, color, antialiased,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_circle_ex`][super::RenderingServer::canvas_item_add_circle_ex]."]
#[must_use]
pub struct ExCanvasItemAddCircle < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, item: Rid, pos: Vector2, radius: f32, color: Color, antialiased: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddCircle < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, pos: Vector2, radius: f32, color: Color,) -> Self {
        let antialiased = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, pos: pos, radius: radius, color: color, antialiased: antialiased,
        }
    }
    #[inline]
    pub fn antialiased(self, antialiased: bool) -> Self {
        Self {
            antialiased: antialiased, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, pos, radius, color, antialiased,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_circle_full(surround_object, item, pos, radius, color, antialiased,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_texture_rect_ex`][super::RenderingServer::canvas_item_add_texture_rect_ex]."]
#[must_use]
pub struct ExCanvasItemAddTextureRect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid, tile: bool, modulate: Color, transpose: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddTextureRect < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid,) -> Self {
        let tile = false;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let transpose = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, rect: rect, texture: texture, tile: tile, modulate: modulate, transpose: transpose,
        }
    }
    #[inline]
    pub fn tile(self, tile: bool) -> Self {
        Self {
            tile: tile, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn transpose(self, transpose: bool) -> Self {
        Self {
            transpose: transpose, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, rect, texture, tile, modulate, transpose,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_texture_rect_full(surround_object, item, rect, texture, tile, modulate, transpose,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_msdf_texture_rect_region_ex`][super::RenderingServer::canvas_item_add_msdf_texture_rect_region_ex]."]
#[must_use]
pub struct ExCanvasItemAddMsdfTextureRectRegion < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2, modulate: Color, outline_size: i32, px_range: f32, scale: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddMsdfTextureRectRegion < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let outline_size = 0i32;
        let px_range = 1f32;
        let scale = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, rect: rect, texture: texture, src_rect: src_rect, modulate: modulate, outline_size: outline_size, px_range: px_range, scale: scale,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn outline_size(self, outline_size: i32) -> Self {
        Self {
            outline_size: outline_size, .. self
        }
    }
    #[inline]
    pub fn px_range(self, px_range: f32) -> Self {
        Self {
            px_range: px_range, .. self
        }
    }
    #[inline]
    pub fn scale(self, scale: f32) -> Self {
        Self {
            scale: scale, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, rect, texture, src_rect, modulate, outline_size, px_range, scale,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_msdf_texture_rect_region_full(surround_object, item, rect, texture, src_rect, modulate, outline_size, px_range, scale,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_texture_rect_region_ex`][super::RenderingServer::canvas_item_add_texture_rect_region_ex]."]
#[must_use]
pub struct ExCanvasItemAddTextureRectRegion < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2, modulate: Color, transpose: bool, clip_uv: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddTextureRectRegion < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, texture: Rid, src_rect: Rect2,) -> Self {
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let transpose = false;
        let clip_uv = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, rect: rect, texture: texture, src_rect: src_rect, modulate: modulate, transpose: transpose, clip_uv: clip_uv,
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn transpose(self, transpose: bool) -> Self {
        Self {
            transpose: transpose, .. self
        }
    }
    #[inline]
    pub fn clip_uv(self, clip_uv: bool) -> Self {
        Self {
            clip_uv: clip_uv, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, rect, texture, src_rect, modulate, transpose, clip_uv,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_texture_rect_region_full(surround_object, item, rect, texture, src_rect, modulate, transpose, clip_uv,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_nine_patch_ex`][super::RenderingServer::canvas_item_add_nine_patch_ex]."]
#[must_use]
pub struct ExCanvasItemAddNinePatch < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, source: Rect2, texture: Rid, topleft: Vector2, bottomright: Vector2, x_axis_mode: crate::classes::rendering_server::NinePatchAxisMode, y_axis_mode: crate::classes::rendering_server::NinePatchAxisMode, draw_center: bool, modulate: Color,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddNinePatch < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, rect: Rect2, source: Rect2, texture: Rid, topleft: Vector2, bottomright: Vector2,) -> Self {
        let x_axis_mode = crate::obj::EngineEnum::from_ord(0);
        let y_axis_mode = crate::obj::EngineEnum::from_ord(0);
        let draw_center = true;
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, rect: rect, source: source, texture: texture, topleft: topleft, bottomright: bottomright, x_axis_mode: x_axis_mode, y_axis_mode: y_axis_mode, draw_center: draw_center, modulate: modulate,
        }
    }
    #[inline]
    pub fn x_axis_mode(self, x_axis_mode: crate::classes::rendering_server::NinePatchAxisMode) -> Self {
        Self {
            x_axis_mode: x_axis_mode, .. self
        }
    }
    #[inline]
    pub fn y_axis_mode(self, y_axis_mode: crate::classes::rendering_server::NinePatchAxisMode) -> Self {
        Self {
            y_axis_mode: y_axis_mode, .. self
        }
    }
    #[inline]
    pub fn draw_center(self, draw_center: bool) -> Self {
        Self {
            draw_center: draw_center, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, rect, source, texture, topleft, bottomright, x_axis_mode, y_axis_mode, draw_center, modulate,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_nine_patch_full(surround_object, item, rect, source, texture, topleft, bottomright, x_axis_mode, y_axis_mode, draw_center, modulate,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_polygon_ex`][super::RenderingServer::canvas_item_add_polygon_ex]."]
#[must_use]
pub struct ExCanvasItemAddPolygon < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, item: Rid, points: CowArg < 'a, PackedVector2Array >, colors: CowArg < 'a, PackedColorArray >, uvs: CowArg < 'a, PackedVector2Array >, texture: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddPolygon < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, points: &'a PackedVector2Array, colors: &'a PackedColorArray,) -> Self {
        let uvs = PackedVector2Array::new();
        let texture = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, points: CowArg::Borrowed(points), colors: CowArg::Borrowed(colors), uvs: CowArg::Owned(uvs), texture: texture,
        }
    }
    #[inline]
    pub fn uvs(self, uvs: &'a PackedVector2Array) -> Self {
        Self {
            uvs: CowArg::Borrowed(uvs), .. self
        }
    }
    #[inline]
    pub fn texture(self, texture: Rid) -> Self {
        Self {
            texture: texture, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, points, colors, uvs, texture,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_polygon_full(surround_object, item, points.cow_as_arg(), colors.cow_as_arg(), uvs.cow_as_arg(), texture,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_triangle_array_ex`][super::RenderingServer::canvas_item_add_triangle_array_ex]."]
#[must_use]
pub struct ExCanvasItemAddTriangleArray < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, item: Rid, indices: CowArg < 'a, PackedInt32Array >, points: CowArg < 'a, PackedVector2Array >, colors: CowArg < 'a, PackedColorArray >, uvs: CowArg < 'a, PackedVector2Array >, bones: CowArg < 'a, PackedInt32Array >, weights: CowArg < 'a, PackedFloat32Array >, texture: Rid, count: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddTriangleArray < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, indices: &'a PackedInt32Array, points: &'a PackedVector2Array, colors: &'a PackedColorArray,) -> Self {
        let uvs = PackedVector2Array::new();
        let bones = PackedInt32Array::new();
        let weights = PackedFloat32Array::new();
        let texture = Rid::Invalid;
        let count = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, indices: CowArg::Borrowed(indices), points: CowArg::Borrowed(points), colors: CowArg::Borrowed(colors), uvs: CowArg::Owned(uvs), bones: CowArg::Owned(bones), weights: CowArg::Owned(weights), texture: texture, count: count,
        }
    }
    #[inline]
    pub fn uvs(self, uvs: &'a PackedVector2Array) -> Self {
        Self {
            uvs: CowArg::Borrowed(uvs), .. self
        }
    }
    #[inline]
    pub fn bones(self, bones: &'a PackedInt32Array) -> Self {
        Self {
            bones: CowArg::Borrowed(bones), .. self
        }
    }
    #[inline]
    pub fn weights(self, weights: &'a PackedFloat32Array) -> Self {
        Self {
            weights: CowArg::Borrowed(weights), .. self
        }
    }
    #[inline]
    pub fn texture(self, texture: Rid) -> Self {
        Self {
            texture: texture, .. self
        }
    }
    #[inline]
    pub fn count(self, count: i32) -> Self {
        Self {
            count: count, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, indices, points, colors, uvs, bones, weights, texture, count,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_triangle_array_full(surround_object, item, indices.cow_as_arg(), points.cow_as_arg(), colors.cow_as_arg(), uvs.cow_as_arg(), bones.cow_as_arg(), weights.cow_as_arg(), texture, count,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_mesh_ex`][super::RenderingServer::canvas_item_add_mesh_ex]."]
#[must_use]
pub struct ExCanvasItemAddMesh < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, item: Rid, mesh: Rid, transform: Transform2D, modulate: Color, texture: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddMesh < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, mesh: Rid,) -> Self {
        let transform = Transform2D::__internal_codegen(1 as _, 0 as _, 0 as _, 1 as _, 0 as _, 0 as _);
        let modulate = Color::from_rgba(1 as _, 1 as _, 1 as _, 1 as _);
        let texture = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, mesh: mesh, transform: transform, modulate: modulate, texture: texture,
        }
    }
    #[inline]
    pub fn transform(self, transform: Transform2D) -> Self {
        Self {
            transform: transform, .. self
        }
    }
    #[inline]
    pub fn modulate(self, modulate: Color) -> Self {
        Self {
            modulate: modulate, .. self
        }
    }
    #[inline]
    pub fn texture(self, texture: Rid) -> Self {
        Self {
            texture: texture, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, mesh, transform, modulate, texture,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_mesh_full(surround_object, item, mesh, transform, modulate, texture,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_multimesh_ex`][super::RenderingServer::canvas_item_add_multimesh_ex]."]
#[must_use]
pub struct ExCanvasItemAddMultimesh < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, item: Rid, mesh: Rid, texture: Rid,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddMultimesh < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, mesh: Rid,) -> Self {
        let texture = Rid::Invalid;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, mesh: mesh, texture: texture,
        }
    }
    #[inline]
    pub fn texture(self, texture: Rid) -> Self {
        Self {
            texture: texture, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, mesh, texture,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_multimesh_full(surround_object, item, mesh, texture,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_add_animation_slice_ex`][super::RenderingServer::canvas_item_add_animation_slice_ex]."]
#[must_use]
pub struct ExCanvasItemAddAnimationSlice < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, item: Rid, animation_length: f64, slice_begin: f64, slice_end: f64, offset: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemAddAnimationSlice < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, animation_length: f64, slice_begin: f64, slice_end: f64,) -> Self {
        let offset = 0f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, animation_length: animation_length, slice_begin: slice_begin, slice_end: slice_end, offset: offset,
        }
    }
    #[inline]
    pub fn offset(self, offset: f64) -> Self {
        Self {
            offset: offset, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, animation_length, slice_begin, slice_end, offset,
        }
        = self;
        re_export::RenderingServer::canvas_item_add_animation_slice_full(surround_object, item, animation_length, slice_begin, slice_end, offset,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::canvas_item_set_canvas_group_mode_ex`][super::RenderingServer::canvas_item_set_canvas_group_mode_ex]."]
#[must_use]
pub struct ExCanvasItemSetCanvasGroupMode < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, item: Rid, mode: crate::classes::rendering_server::CanvasGroupMode, clear_margin: f32, fit_empty: bool, fit_margin: f32, blur_mipmaps: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCanvasItemSetCanvasGroupMode < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, item: Rid, mode: crate::classes::rendering_server::CanvasGroupMode,) -> Self {
        let clear_margin = 5f32;
        let fit_empty = false;
        let fit_margin = 0f32;
        let blur_mipmaps = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item, mode: mode, clear_margin: clear_margin, fit_empty: fit_empty, fit_margin: fit_margin, blur_mipmaps: blur_mipmaps,
        }
    }
    #[inline]
    pub fn clear_margin(self, clear_margin: f32) -> Self {
        Self {
            clear_margin: clear_margin, .. self
        }
    }
    #[inline]
    pub fn fit_empty(self, fit_empty: bool) -> Self {
        Self {
            fit_empty: fit_empty, .. self
        }
    }
    #[inline]
    pub fn fit_margin(self, fit_margin: f32) -> Self {
        Self {
            fit_margin: fit_margin, .. self
        }
    }
    #[inline]
    pub fn blur_mipmaps(self, blur_mipmaps: bool) -> Self {
        Self {
            blur_mipmaps: blur_mipmaps, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, mode, clear_margin, fit_empty, fit_margin, blur_mipmaps,
        }
        = self;
        re_export::RenderingServer::canvas_item_set_canvas_group_mode_full(surround_object, item, mode, clear_margin, fit_empty, fit_margin, blur_mipmaps,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::set_boot_image_ex`][super::RenderingServer::set_boot_image_ex]."]
#[must_use]
pub struct ExSetBootImage < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, image: CowArg < 'a, Option < Gd < crate::classes::Image > > >, color: Color, scale: bool, use_filter: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetBootImage < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer, image: impl AsArg < Option < Gd < crate::classes::Image >> > + 'a, color: Color, scale: bool,) -> Self {
        let use_filter = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, image: image.into_arg(), color: color, scale: scale, use_filter: use_filter,
        }
    }
    #[inline]
    pub fn use_filter(self, use_filter: bool) -> Self {
        Self {
            use_filter: use_filter, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, image, color, scale, use_filter,
        }
        = self;
        re_export::RenderingServer::set_boot_image_full(surround_object, image, color, scale, use_filter,)
    }
}
#[doc = "Default-param extender for [`RenderingServer::force_draw_ex`][super::RenderingServer::force_draw_ex]."]
#[must_use]
pub struct ExForceDraw < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderingServer, swap_buffers: bool, frame_step: f64,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExForceDraw < 'a > {
    fn new(surround_object: &'a mut re_export::RenderingServer,) -> Self {
        let swap_buffers = true;
        let frame_step = 0f64;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, swap_buffers: swap_buffers, frame_step: frame_step,
        }
    }
    #[inline]
    pub fn swap_buffers(self, swap_buffers: bool) -> Self {
        Self {
            swap_buffers: swap_buffers, .. self
        }
    }
    #[inline]
    pub fn frame_step(self, frame_step: f64) -> Self {
        Self {
            frame_step: frame_step, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, swap_buffers, frame_step,
        }
        = self;
        re_export::RenderingServer::force_draw_full(surround_object, swap_buffers, frame_step,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TextureType {
    ord: i32
}
impl TextureType {
    #[doc(alias = "TEXTURE_TYPE_2D")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_2D`"]
    pub const TYPE_2D: TextureType = TextureType {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_TYPE_LAYERED")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_LAYERED`"]
    pub const LAYERED: TextureType = TextureType {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_TYPE_3D")]
    #[doc = "Godot enumerator name: `TEXTURE_TYPE_3D`"]
    pub const TYPE_3D: TextureType = TextureType {
        ord: 2i32
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
            Self::TYPE_2D => "TYPE_2D", Self::LAYERED => "LAYERED", Self::TYPE_3D => "TYPE_3D", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureType::TYPE_2D, TextureType::LAYERED, TextureType::TYPE_3D]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TYPE_2D", "TEXTURE_TYPE_2D", TextureType::TYPE_2D), crate::meta::inspect::EnumConstant::new("LAYERED", "TEXTURE_TYPE_LAYERED", TextureType::LAYERED), crate::meta::inspect::EnumConstant::new("TYPE_3D", "TEXTURE_TYPE_3D", TextureType::TYPE_3D)]
        }
    }
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
pub struct TextureLayeredType {
    ord: i32
}
impl TextureLayeredType {
    #[doc(alias = "TEXTURE_LAYERED_2D_ARRAY")]
    #[doc = "Godot enumerator name: `TEXTURE_LAYERED_2D_ARRAY`"]
    pub const LAYERED_2D_ARRAY: TextureLayeredType = TextureLayeredType {
        ord: 0i32
    };
    #[doc(alias = "TEXTURE_LAYERED_CUBEMAP")]
    #[doc = "Godot enumerator name: `TEXTURE_LAYERED_CUBEMAP`"]
    pub const CUBEMAP: TextureLayeredType = TextureLayeredType {
        ord: 1i32
    };
    #[doc(alias = "TEXTURE_LAYERED_CUBEMAP_ARRAY")]
    #[doc = "Godot enumerator name: `TEXTURE_LAYERED_CUBEMAP_ARRAY`"]
    pub const CUBEMAP_ARRAY: TextureLayeredType = TextureLayeredType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TextureLayeredType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TextureLayeredType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TextureLayeredType {
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
            Self::LAYERED_2D_ARRAY => "LAYERED_2D_ARRAY", Self::CUBEMAP => "CUBEMAP", Self::CUBEMAP_ARRAY => "CUBEMAP_ARRAY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TextureLayeredType::LAYERED_2D_ARRAY, TextureLayeredType::CUBEMAP, TextureLayeredType::CUBEMAP_ARRAY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TextureLayeredType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LAYERED_2D_ARRAY", "TEXTURE_LAYERED_2D_ARRAY", TextureLayeredType::LAYERED_2D_ARRAY), crate::meta::inspect::EnumConstant::new("CUBEMAP", "TEXTURE_LAYERED_CUBEMAP", TextureLayeredType::CUBEMAP), crate::meta::inspect::EnumConstant::new("CUBEMAP_ARRAY", "TEXTURE_LAYERED_CUBEMAP_ARRAY", TextureLayeredType::CUBEMAP_ARRAY)]
        }
    }
}
impl crate::meta::GodotConvert for TextureLayeredType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TextureLayeredType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TextureLayeredType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CubeMapLayer {
    ord: i32
}
impl CubeMapLayer {
    #[doc(alias = "CUBEMAP_LAYER_LEFT")]
    #[doc = "Godot enumerator name: `CUBEMAP_LAYER_LEFT`"]
    pub const LEFT: CubeMapLayer = CubeMapLayer {
        ord: 0i32
    };
    #[doc(alias = "CUBEMAP_LAYER_RIGHT")]
    #[doc = "Godot enumerator name: `CUBEMAP_LAYER_RIGHT`"]
    pub const RIGHT: CubeMapLayer = CubeMapLayer {
        ord: 1i32
    };
    #[doc(alias = "CUBEMAP_LAYER_BOTTOM")]
    #[doc = "Godot enumerator name: `CUBEMAP_LAYER_BOTTOM`"]
    pub const BOTTOM: CubeMapLayer = CubeMapLayer {
        ord: 2i32
    };
    #[doc(alias = "CUBEMAP_LAYER_TOP")]
    #[doc = "Godot enumerator name: `CUBEMAP_LAYER_TOP`"]
    pub const TOP: CubeMapLayer = CubeMapLayer {
        ord: 3i32
    };
    #[doc(alias = "CUBEMAP_LAYER_FRONT")]
    #[doc = "Godot enumerator name: `CUBEMAP_LAYER_FRONT`"]
    pub const FRONT: CubeMapLayer = CubeMapLayer {
        ord: 4i32
    };
    #[doc(alias = "CUBEMAP_LAYER_BACK")]
    #[doc = "Godot enumerator name: `CUBEMAP_LAYER_BACK`"]
    pub const BACK: CubeMapLayer = CubeMapLayer {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for CubeMapLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CubeMapLayer") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CubeMapLayer {
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
            Self::LEFT => "LEFT", Self::RIGHT => "RIGHT", Self::BOTTOM => "BOTTOM", Self::TOP => "TOP", Self::FRONT => "FRONT", Self::BACK => "BACK", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CubeMapLayer::LEFT, CubeMapLayer::RIGHT, CubeMapLayer::BOTTOM, CubeMapLayer::TOP, CubeMapLayer::FRONT, CubeMapLayer::BACK]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CubeMapLayer >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LEFT", "CUBEMAP_LAYER_LEFT", CubeMapLayer::LEFT), crate::meta::inspect::EnumConstant::new("RIGHT", "CUBEMAP_LAYER_RIGHT", CubeMapLayer::RIGHT), crate::meta::inspect::EnumConstant::new("BOTTOM", "CUBEMAP_LAYER_BOTTOM", CubeMapLayer::BOTTOM), crate::meta::inspect::EnumConstant::new("TOP", "CUBEMAP_LAYER_TOP", CubeMapLayer::TOP), crate::meta::inspect::EnumConstant::new("FRONT", "CUBEMAP_LAYER_FRONT", CubeMapLayer::FRONT), crate::meta::inspect::EnumConstant::new("BACK", "CUBEMAP_LAYER_BACK", CubeMapLayer::BACK)]
        }
    }
}
impl crate::meta::GodotConvert for CubeMapLayer {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CubeMapLayer {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CubeMapLayer {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShaderMode {
    ord: i32
}
impl ShaderMode {
    #[doc(alias = "SHADER_SPATIAL")]
    #[doc = "Godot enumerator name: `SHADER_SPATIAL`"]
    pub const SPATIAL: ShaderMode = ShaderMode {
        ord: 0i32
    };
    #[doc(alias = "SHADER_CANVAS_ITEM")]
    #[doc = "Godot enumerator name: `SHADER_CANVAS_ITEM`"]
    pub const CANVAS_ITEM: ShaderMode = ShaderMode {
        ord: 1i32
    };
    #[doc(alias = "SHADER_PARTICLES")]
    #[doc = "Godot enumerator name: `SHADER_PARTICLES`"]
    pub const PARTICLES: ShaderMode = ShaderMode {
        ord: 2i32
    };
    #[doc(alias = "SHADER_SKY")]
    #[doc = "Godot enumerator name: `SHADER_SKY`"]
    pub const SKY: ShaderMode = ShaderMode {
        ord: 3i32
    };
    #[doc(alias = "SHADER_FOG")]
    #[doc = "Godot enumerator name: `SHADER_FOG`"]
    pub const FOG: ShaderMode = ShaderMode {
        ord: 4i32
    };
    #[doc(alias = "SHADER_MAX")]
    #[doc = "Godot enumerator name: `SHADER_MAX`"]
    pub const MAX: ShaderMode = ShaderMode {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for ShaderMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShaderMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShaderMode {
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
            Self::SPATIAL => "SPATIAL", Self::CANVAS_ITEM => "CANVAS_ITEM", Self::PARTICLES => "PARTICLES", Self::SKY => "SKY", Self::FOG => "FOG", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ShaderMode::SPATIAL, ShaderMode::CANVAS_ITEM, ShaderMode::PARTICLES, ShaderMode::SKY, ShaderMode::FOG]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ShaderMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SPATIAL", "SHADER_SPATIAL", ShaderMode::SPATIAL), crate::meta::inspect::EnumConstant::new("CANVAS_ITEM", "SHADER_CANVAS_ITEM", ShaderMode::CANVAS_ITEM), crate::meta::inspect::EnumConstant::new("PARTICLES", "SHADER_PARTICLES", ShaderMode::PARTICLES), crate::meta::inspect::EnumConstant::new("SKY", "SHADER_SKY", ShaderMode::SKY), crate::meta::inspect::EnumConstant::new("FOG", "SHADER_FOG", ShaderMode::FOG), crate::meta::inspect::EnumConstant::new("MAX", "SHADER_MAX", ShaderMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ShaderMode {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for ShaderMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ShaderMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShaderMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ArrayType {
    ord: i32
}
impl ArrayType {
    #[doc(alias = "ARRAY_VERTEX")]
    #[doc = "Godot enumerator name: `ARRAY_VERTEX`"]
    pub const VERTEX: ArrayType = ArrayType {
        ord: 0i32
    };
    #[doc(alias = "ARRAY_NORMAL")]
    #[doc = "Godot enumerator name: `ARRAY_NORMAL`"]
    pub const NORMAL: ArrayType = ArrayType {
        ord: 1i32
    };
    #[doc(alias = "ARRAY_TANGENT")]
    #[doc = "Godot enumerator name: `ARRAY_TANGENT`"]
    pub const TANGENT: ArrayType = ArrayType {
        ord: 2i32
    };
    #[doc(alias = "ARRAY_COLOR")]
    #[doc = "Godot enumerator name: `ARRAY_COLOR`"]
    pub const COLOR: ArrayType = ArrayType {
        ord: 3i32
    };
    #[doc(alias = "ARRAY_TEX_UV")]
    #[doc = "Godot enumerator name: `ARRAY_TEX_UV`"]
    pub const TEX_UV: ArrayType = ArrayType {
        ord: 4i32
    };
    #[doc(alias = "ARRAY_TEX_UV2")]
    #[doc = "Godot enumerator name: `ARRAY_TEX_UV2`"]
    pub const TEX_UV2: ArrayType = ArrayType {
        ord: 5i32
    };
    #[doc(alias = "ARRAY_CUSTOM0")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM0`"]
    pub const CUSTOM0: ArrayType = ArrayType {
        ord: 6i32
    };
    #[doc(alias = "ARRAY_CUSTOM1")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM1`"]
    pub const CUSTOM1: ArrayType = ArrayType {
        ord: 7i32
    };
    #[doc(alias = "ARRAY_CUSTOM2")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM2`"]
    pub const CUSTOM2: ArrayType = ArrayType {
        ord: 8i32
    };
    #[doc(alias = "ARRAY_CUSTOM3")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM3`"]
    pub const CUSTOM3: ArrayType = ArrayType {
        ord: 9i32
    };
    #[doc(alias = "ARRAY_BONES")]
    #[doc = "Godot enumerator name: `ARRAY_BONES`"]
    pub const BONES: ArrayType = ArrayType {
        ord: 10i32
    };
    #[doc(alias = "ARRAY_WEIGHTS")]
    #[doc = "Godot enumerator name: `ARRAY_WEIGHTS`"]
    pub const WEIGHTS: ArrayType = ArrayType {
        ord: 11i32
    };
    #[doc(alias = "ARRAY_INDEX")]
    #[doc = "Godot enumerator name: `ARRAY_INDEX`"]
    pub const INDEX: ArrayType = ArrayType {
        ord: 12i32
    };
    #[doc(alias = "ARRAY_MAX")]
    #[doc = "Godot enumerator name: `ARRAY_MAX`"]
    pub const MAX: ArrayType = ArrayType {
        ord: 13i32
    };
    
}
impl std::fmt::Debug for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ArrayType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ArrayType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
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
            Self::VERTEX => "VERTEX", Self::NORMAL => "NORMAL", Self::TANGENT => "TANGENT", Self::COLOR => "COLOR", Self::TEX_UV => "TEX_UV", Self::TEX_UV2 => "TEX_UV2", Self::CUSTOM0 => "CUSTOM0", Self::CUSTOM1 => "CUSTOM1", Self::CUSTOM2 => "CUSTOM2", Self::CUSTOM3 => "CUSTOM3", Self::BONES => "BONES", Self::WEIGHTS => "WEIGHTS", Self::INDEX => "INDEX", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ArrayType::VERTEX, ArrayType::NORMAL, ArrayType::TANGENT, ArrayType::COLOR, ArrayType::TEX_UV, ArrayType::TEX_UV2, ArrayType::CUSTOM0, ArrayType::CUSTOM1, ArrayType::CUSTOM2, ArrayType::CUSTOM3, ArrayType::BONES, ArrayType::WEIGHTS, ArrayType::INDEX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ArrayType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERTEX", "ARRAY_VERTEX", ArrayType::VERTEX), crate::meta::inspect::EnumConstant::new("NORMAL", "ARRAY_NORMAL", ArrayType::NORMAL), crate::meta::inspect::EnumConstant::new("TANGENT", "ARRAY_TANGENT", ArrayType::TANGENT), crate::meta::inspect::EnumConstant::new("COLOR", "ARRAY_COLOR", ArrayType::COLOR), crate::meta::inspect::EnumConstant::new("TEX_UV", "ARRAY_TEX_UV", ArrayType::TEX_UV), crate::meta::inspect::EnumConstant::new("TEX_UV2", "ARRAY_TEX_UV2", ArrayType::TEX_UV2), crate::meta::inspect::EnumConstant::new("CUSTOM0", "ARRAY_CUSTOM0", ArrayType::CUSTOM0), crate::meta::inspect::EnumConstant::new("CUSTOM1", "ARRAY_CUSTOM1", ArrayType::CUSTOM1), crate::meta::inspect::EnumConstant::new("CUSTOM2", "ARRAY_CUSTOM2", ArrayType::CUSTOM2), crate::meta::inspect::EnumConstant::new("CUSTOM3", "ARRAY_CUSTOM3", ArrayType::CUSTOM3), crate::meta::inspect::EnumConstant::new("BONES", "ARRAY_BONES", ArrayType::BONES), crate::meta::inspect::EnumConstant::new("WEIGHTS", "ARRAY_WEIGHTS", ArrayType::WEIGHTS), crate::meta::inspect::EnumConstant::new("INDEX", "ARRAY_INDEX", ArrayType::INDEX), crate::meta::inspect::EnumConstant::new("MAX", "ARRAY_MAX", ArrayType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ArrayType {
    const ENUMERATOR_COUNT: usize = 13usize;
    
}
impl crate::meta::GodotConvert for ArrayType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ArrayType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ArrayType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ArrayCustomFormat {
    ord: i32
}
impl ArrayCustomFormat {
    #[doc(alias = "ARRAY_CUSTOM_RGBA8_UNORM")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGBA8_UNORM`"]
    pub const RGBA8_UNORM: ArrayCustomFormat = ArrayCustomFormat {
        ord: 0i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RGBA8_SNORM")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGBA8_SNORM`"]
    pub const RGBA8_SNORM: ArrayCustomFormat = ArrayCustomFormat {
        ord: 1i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RG_HALF")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RG_HALF`"]
    pub const RG_HALF: ArrayCustomFormat = ArrayCustomFormat {
        ord: 2i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RGBA_HALF")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGBA_HALF`"]
    pub const RGBA_HALF: ArrayCustomFormat = ArrayCustomFormat {
        ord: 3i32
    };
    #[doc(alias = "ARRAY_CUSTOM_R_FLOAT")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_R_FLOAT`"]
    pub const R_FLOAT: ArrayCustomFormat = ArrayCustomFormat {
        ord: 4i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RG_FLOAT")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RG_FLOAT`"]
    pub const RG_FLOAT: ArrayCustomFormat = ArrayCustomFormat {
        ord: 5i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RGB_FLOAT")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGB_FLOAT`"]
    pub const RGB_FLOAT: ArrayCustomFormat = ArrayCustomFormat {
        ord: 6i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RGBA_FLOAT")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGBA_FLOAT`"]
    pub const RGBA_FLOAT: ArrayCustomFormat = ArrayCustomFormat {
        ord: 7i32
    };
    #[doc(alias = "ARRAY_CUSTOM_MAX")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_MAX`"]
    pub const MAX: ArrayCustomFormat = ArrayCustomFormat {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for ArrayCustomFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ArrayCustomFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ArrayCustomFormat {
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
            Self::RGBA8_UNORM => "RGBA8_UNORM", Self::RGBA8_SNORM => "RGBA8_SNORM", Self::RG_HALF => "RG_HALF", Self::RGBA_HALF => "RGBA_HALF", Self::R_FLOAT => "R_FLOAT", Self::RG_FLOAT => "RG_FLOAT", Self::RGB_FLOAT => "RGB_FLOAT", Self::RGBA_FLOAT => "RGBA_FLOAT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ArrayCustomFormat::RGBA8_UNORM, ArrayCustomFormat::RGBA8_SNORM, ArrayCustomFormat::RG_HALF, ArrayCustomFormat::RGBA_HALF, ArrayCustomFormat::R_FLOAT, ArrayCustomFormat::RG_FLOAT, ArrayCustomFormat::RGB_FLOAT, ArrayCustomFormat::RGBA_FLOAT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ArrayCustomFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RGBA8_UNORM", "ARRAY_CUSTOM_RGBA8_UNORM", ArrayCustomFormat::RGBA8_UNORM), crate::meta::inspect::EnumConstant::new("RGBA8_SNORM", "ARRAY_CUSTOM_RGBA8_SNORM", ArrayCustomFormat::RGBA8_SNORM), crate::meta::inspect::EnumConstant::new("RG_HALF", "ARRAY_CUSTOM_RG_HALF", ArrayCustomFormat::RG_HALF), crate::meta::inspect::EnumConstant::new("RGBA_HALF", "ARRAY_CUSTOM_RGBA_HALF", ArrayCustomFormat::RGBA_HALF), crate::meta::inspect::EnumConstant::new("R_FLOAT", "ARRAY_CUSTOM_R_FLOAT", ArrayCustomFormat::R_FLOAT), crate::meta::inspect::EnumConstant::new("RG_FLOAT", "ARRAY_CUSTOM_RG_FLOAT", ArrayCustomFormat::RG_FLOAT), crate::meta::inspect::EnumConstant::new("RGB_FLOAT", "ARRAY_CUSTOM_RGB_FLOAT", ArrayCustomFormat::RGB_FLOAT), crate::meta::inspect::EnumConstant::new("RGBA_FLOAT", "ARRAY_CUSTOM_RGBA_FLOAT", ArrayCustomFormat::RGBA_FLOAT), crate::meta::inspect::EnumConstant::new("MAX", "ARRAY_CUSTOM_MAX", ArrayCustomFormat::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ArrayCustomFormat {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for ArrayCustomFormat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ArrayCustomFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ArrayCustomFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ArrayFormat {
    ord: u64
}
impl ArrayFormat {
    #[doc(alias = "ARRAY_FORMAT_VERTEX")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_VERTEX`"]
    pub const VERTEX: ArrayFormat = ArrayFormat {
        ord: 1u64
    };
    #[doc(alias = "ARRAY_FORMAT_NORMAL")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_NORMAL`"]
    pub const NORMAL: ArrayFormat = ArrayFormat {
        ord: 2u64
    };
    #[doc(alias = "ARRAY_FORMAT_TANGENT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_TANGENT`"]
    pub const TANGENT: ArrayFormat = ArrayFormat {
        ord: 4u64
    };
    #[doc(alias = "ARRAY_FORMAT_COLOR")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_COLOR`"]
    pub const COLOR: ArrayFormat = ArrayFormat {
        ord: 8u64
    };
    #[doc(alias = "ARRAY_FORMAT_TEX_UV")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_TEX_UV`"]
    pub const TEX_UV: ArrayFormat = ArrayFormat {
        ord: 16u64
    };
    #[doc(alias = "ARRAY_FORMAT_TEX_UV2")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_TEX_UV2`"]
    pub const TEX_UV2: ArrayFormat = ArrayFormat {
        ord: 32u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM0")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM0`"]
    pub const CUSTOM0: ArrayFormat = ArrayFormat {
        ord: 64u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM1")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM1`"]
    pub const CUSTOM1: ArrayFormat = ArrayFormat {
        ord: 128u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM2")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM2`"]
    pub const CUSTOM2: ArrayFormat = ArrayFormat {
        ord: 256u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM3")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM3`"]
    pub const CUSTOM3: ArrayFormat = ArrayFormat {
        ord: 512u64
    };
    #[doc(alias = "ARRAY_FORMAT_BONES")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_BONES`"]
    pub const BONES: ArrayFormat = ArrayFormat {
        ord: 1024u64
    };
    #[doc(alias = "ARRAY_FORMAT_WEIGHTS")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_WEIGHTS`"]
    pub const WEIGHTS: ArrayFormat = ArrayFormat {
        ord: 2048u64
    };
    #[doc(alias = "ARRAY_FORMAT_INDEX")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_INDEX`"]
    pub const INDEX: ArrayFormat = ArrayFormat {
        ord: 4096u64
    };
    #[doc(alias = "ARRAY_FORMAT_BLEND_SHAPE_MASK")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_BLEND_SHAPE_MASK`"]
    pub const BLEND_SHAPE_MASK: ArrayFormat = ArrayFormat {
        ord: 7u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM_BASE")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM_BASE`"]
    pub const CUSTOM_BASE: ArrayFormat = ArrayFormat {
        ord: 13u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM_BITS")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM_BITS`"]
    pub const CUSTOM_BITS: ArrayFormat = ArrayFormat {
        ord: 3u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM0_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM0_SHIFT`"]
    pub const CUSTOM0_SHIFT: ArrayFormat = ArrayFormat {
        ord: 13u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM1_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM1_SHIFT`"]
    pub const CUSTOM1_SHIFT: ArrayFormat = ArrayFormat {
        ord: 16u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM2_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM2_SHIFT`"]
    pub const CUSTOM2_SHIFT: ArrayFormat = ArrayFormat {
        ord: 19u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM3_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM3_SHIFT`"]
    pub const CUSTOM3_SHIFT: ArrayFormat = ArrayFormat {
        ord: 22u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM_MASK")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM_MASK`"]
    pub const CUSTOM_MASK: ArrayFormat = ArrayFormat {
        ord: 7u64
    };
    #[doc(alias = "ARRAY_COMPRESS_FLAGS_BASE")]
    #[doc = "Godot enumerator name: `ARRAY_COMPRESS_FLAGS_BASE`"]
    pub const COMPRESS_FLAGS_BASE: ArrayFormat = ArrayFormat {
        ord: 25u64
    };
    #[doc(alias = "ARRAY_FLAG_USE_2D_VERTICES")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_USE_2D_VERTICES`"]
    pub const FLAG_USE_2D_VERTICES: ArrayFormat = ArrayFormat {
        ord: 33554432u64
    };
    #[doc(alias = "ARRAY_FLAG_USE_DYNAMIC_UPDATE")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_USE_DYNAMIC_UPDATE`"]
    pub const FLAG_USE_DYNAMIC_UPDATE: ArrayFormat = ArrayFormat {
        ord: 67108864u64
    };
    #[doc(alias = "ARRAY_FLAG_USE_8_BONE_WEIGHTS")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_USE_8_BONE_WEIGHTS`"]
    pub const FLAG_USE_8_BONE_WEIGHTS: ArrayFormat = ArrayFormat {
        ord: 134217728u64
    };
    #[doc(alias = "ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY`"]
    pub const FLAG_USES_EMPTY_VERTEX_ARRAY: ArrayFormat = ArrayFormat {
        ord: 268435456u64
    };
    #[doc(alias = "ARRAY_FLAG_COMPRESS_ATTRIBUTES")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_COMPRESS_ATTRIBUTES`"]
    pub const FLAG_COMPRESS_ATTRIBUTES: ArrayFormat = ArrayFormat {
        ord: 536870912u64
    };
    #[doc(alias = "ARRAY_FLAG_FORMAT_VERSION_BASE")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_FORMAT_VERSION_BASE`"]
    pub const FLAG_FORMAT_VERSION_BASE: ArrayFormat = ArrayFormat {
        ord: 35u64
    };
    #[doc(alias = "ARRAY_FLAG_FORMAT_VERSION_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_FORMAT_VERSION_SHIFT`"]
    pub const FLAG_FORMAT_VERSION_SHIFT: ArrayFormat = ArrayFormat {
        ord: 35u64
    };
    #[doc(alias = "ARRAY_FLAG_FORMAT_VERSION_1")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_FORMAT_VERSION_1`"]
    pub const FLAG_FORMAT_VERSION_1: ArrayFormat = ArrayFormat {
        ord: 0u64
    };
    #[doc(alias = "ARRAY_FLAG_FORMAT_VERSION_2")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_FORMAT_VERSION_2`"]
    pub const FLAG_FORMAT_VERSION_2: ArrayFormat = ArrayFormat {
        ord: 34359738368u64
    };
    #[doc(alias = "ARRAY_FLAG_FORMAT_CURRENT_VERSION")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_FORMAT_CURRENT_VERSION`"]
    pub const FLAG_FORMAT_CURRENT_VERSION: ArrayFormat = ArrayFormat {
        ord: 34359738368u64
    };
    #[doc(alias = "ARRAY_FLAG_FORMAT_VERSION_MASK")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_FORMAT_VERSION_MASK`"]
    pub const FLAG_FORMAT_VERSION_MASK: ArrayFormat = ArrayFormat {
        ord: 255u64
    };
    
}
impl std::fmt::Debug for ArrayFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::VERTEX => "VERTEX", Self::NORMAL => "NORMAL", Self::TANGENT => "TANGENT", Self::COLOR => "COLOR", Self::TEX_UV => "TEX_UV", Self::TEX_UV2 => "TEX_UV2", Self::CUSTOM0 => "CUSTOM0", Self::CUSTOM1 => "CUSTOM1", Self::CUSTOM2 => "CUSTOM2", Self::CUSTOM3 => "CUSTOM3", Self::BONES => "BONES", Self::WEIGHTS => "WEIGHTS", Self::INDEX => "INDEX", Self::BLEND_SHAPE_MASK => "BLEND_SHAPE_MASK", Self::CUSTOM_BASE => "CUSTOM_BASE", Self::CUSTOM_BITS => "CUSTOM_BITS", Self::CUSTOM0_SHIFT => "CUSTOM0_SHIFT", Self::CUSTOM1_SHIFT => "CUSTOM1_SHIFT", Self::CUSTOM2_SHIFT => "CUSTOM2_SHIFT", Self::CUSTOM3_SHIFT => "CUSTOM3_SHIFT", Self::CUSTOM_MASK => "CUSTOM_MASK", Self::COMPRESS_FLAGS_BASE => "COMPRESS_FLAGS_BASE", Self::FLAG_USE_2D_VERTICES => "FLAG_USE_2D_VERTICES", Self::FLAG_USE_DYNAMIC_UPDATE => "FLAG_USE_DYNAMIC_UPDATE", Self::FLAG_USE_8_BONE_WEIGHTS => "FLAG_USE_8_BONE_WEIGHTS", Self::FLAG_USES_EMPTY_VERTEX_ARRAY => "FLAG_USES_EMPTY_VERTEX_ARRAY", Self::FLAG_COMPRESS_ATTRIBUTES => "FLAG_COMPRESS_ATTRIBUTES", Self::FLAG_FORMAT_VERSION_BASE => "FLAG_FORMAT_VERSION_BASE", Self::FLAG_FORMAT_VERSION_SHIFT => "FLAG_FORMAT_VERSION_SHIFT", Self::FLAG_FORMAT_VERSION_1 => "FLAG_FORMAT_VERSION_1", Self::FLAG_FORMAT_VERSION_2 => "FLAG_FORMAT_VERSION_2", Self::FLAG_FORMAT_CURRENT_VERSION => "FLAG_FORMAT_CURRENT_VERSION", Self::FLAG_FORMAT_VERSION_MASK => "FLAG_FORMAT_VERSION_MASK", _ => {
                f.debug_struct("ArrayFormat") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for ArrayFormat {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ArrayFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERTEX", "ARRAY_FORMAT_VERTEX", ArrayFormat::VERTEX), crate::meta::inspect::EnumConstant::new("NORMAL", "ARRAY_FORMAT_NORMAL", ArrayFormat::NORMAL), crate::meta::inspect::EnumConstant::new("TANGENT", "ARRAY_FORMAT_TANGENT", ArrayFormat::TANGENT), crate::meta::inspect::EnumConstant::new("COLOR", "ARRAY_FORMAT_COLOR", ArrayFormat::COLOR), crate::meta::inspect::EnumConstant::new("TEX_UV", "ARRAY_FORMAT_TEX_UV", ArrayFormat::TEX_UV), crate::meta::inspect::EnumConstant::new("TEX_UV2", "ARRAY_FORMAT_TEX_UV2", ArrayFormat::TEX_UV2), crate::meta::inspect::EnumConstant::new("CUSTOM0", "ARRAY_FORMAT_CUSTOM0", ArrayFormat::CUSTOM0), crate::meta::inspect::EnumConstant::new("CUSTOM1", "ARRAY_FORMAT_CUSTOM1", ArrayFormat::CUSTOM1), crate::meta::inspect::EnumConstant::new("CUSTOM2", "ARRAY_FORMAT_CUSTOM2", ArrayFormat::CUSTOM2), crate::meta::inspect::EnumConstant::new("CUSTOM3", "ARRAY_FORMAT_CUSTOM3", ArrayFormat::CUSTOM3), crate::meta::inspect::EnumConstant::new("BONES", "ARRAY_FORMAT_BONES", ArrayFormat::BONES), crate::meta::inspect::EnumConstant::new("WEIGHTS", "ARRAY_FORMAT_WEIGHTS", ArrayFormat::WEIGHTS), crate::meta::inspect::EnumConstant::new("INDEX", "ARRAY_FORMAT_INDEX", ArrayFormat::INDEX), crate::meta::inspect::EnumConstant::new("BLEND_SHAPE_MASK", "ARRAY_FORMAT_BLEND_SHAPE_MASK", ArrayFormat::BLEND_SHAPE_MASK), crate::meta::inspect::EnumConstant::new("CUSTOM_BASE", "ARRAY_FORMAT_CUSTOM_BASE", ArrayFormat::CUSTOM_BASE), crate::meta::inspect::EnumConstant::new("CUSTOM_BITS", "ARRAY_FORMAT_CUSTOM_BITS", ArrayFormat::CUSTOM_BITS), crate::meta::inspect::EnumConstant::new("CUSTOM0_SHIFT", "ARRAY_FORMAT_CUSTOM0_SHIFT", ArrayFormat::CUSTOM0_SHIFT), crate::meta::inspect::EnumConstant::new("CUSTOM1_SHIFT", "ARRAY_FORMAT_CUSTOM1_SHIFT", ArrayFormat::CUSTOM1_SHIFT), crate::meta::inspect::EnumConstant::new("CUSTOM2_SHIFT", "ARRAY_FORMAT_CUSTOM2_SHIFT", ArrayFormat::CUSTOM2_SHIFT), crate::meta::inspect::EnumConstant::new("CUSTOM3_SHIFT", "ARRAY_FORMAT_CUSTOM3_SHIFT", ArrayFormat::CUSTOM3_SHIFT), crate::meta::inspect::EnumConstant::new("CUSTOM_MASK", "ARRAY_FORMAT_CUSTOM_MASK", ArrayFormat::CUSTOM_MASK), crate::meta::inspect::EnumConstant::new("COMPRESS_FLAGS_BASE", "ARRAY_COMPRESS_FLAGS_BASE", ArrayFormat::COMPRESS_FLAGS_BASE), crate::meta::inspect::EnumConstant::new("FLAG_USE_2D_VERTICES", "ARRAY_FLAG_USE_2D_VERTICES", ArrayFormat::FLAG_USE_2D_VERTICES), crate::meta::inspect::EnumConstant::new("FLAG_USE_DYNAMIC_UPDATE", "ARRAY_FLAG_USE_DYNAMIC_UPDATE", ArrayFormat::FLAG_USE_DYNAMIC_UPDATE), crate::meta::inspect::EnumConstant::new("FLAG_USE_8_BONE_WEIGHTS", "ARRAY_FLAG_USE_8_BONE_WEIGHTS", ArrayFormat::FLAG_USE_8_BONE_WEIGHTS), crate::meta::inspect::EnumConstant::new("FLAG_USES_EMPTY_VERTEX_ARRAY", "ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY", ArrayFormat::FLAG_USES_EMPTY_VERTEX_ARRAY), crate::meta::inspect::EnumConstant::new("FLAG_COMPRESS_ATTRIBUTES", "ARRAY_FLAG_COMPRESS_ATTRIBUTES", ArrayFormat::FLAG_COMPRESS_ATTRIBUTES), crate::meta::inspect::EnumConstant::new("FLAG_FORMAT_VERSION_BASE", "ARRAY_FLAG_FORMAT_VERSION_BASE", ArrayFormat::FLAG_FORMAT_VERSION_BASE), crate::meta::inspect::EnumConstant::new("FLAG_FORMAT_VERSION_SHIFT", "ARRAY_FLAG_FORMAT_VERSION_SHIFT", ArrayFormat::FLAG_FORMAT_VERSION_SHIFT), crate::meta::inspect::EnumConstant::new("FLAG_FORMAT_VERSION_1", "ARRAY_FLAG_FORMAT_VERSION_1", ArrayFormat::FLAG_FORMAT_VERSION_1), crate::meta::inspect::EnumConstant::new("FLAG_FORMAT_VERSION_2", "ARRAY_FLAG_FORMAT_VERSION_2", ArrayFormat::FLAG_FORMAT_VERSION_2), crate::meta::inspect::EnumConstant::new("FLAG_FORMAT_CURRENT_VERSION", "ARRAY_FLAG_FORMAT_CURRENT_VERSION", ArrayFormat::FLAG_FORMAT_CURRENT_VERSION), crate::meta::inspect::EnumConstant::new("FLAG_FORMAT_VERSION_MASK", "ARRAY_FLAG_FORMAT_VERSION_MASK", ArrayFormat::FLAG_FORMAT_VERSION_MASK)]
        }
    }
}
impl std::ops::BitOr for ArrayFormat {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for ArrayFormat {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for ArrayFormat {
    type Via = u64;
    
}
impl crate::meta::ToGodot for ArrayFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ArrayFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PrimitiveType {
    ord: i32
}
impl PrimitiveType {
    #[doc(alias = "PRIMITIVE_POINTS")]
    #[doc = "Godot enumerator name: `PRIMITIVE_POINTS`"]
    pub const POINTS: PrimitiveType = PrimitiveType {
        ord: 0i32
    };
    #[doc(alias = "PRIMITIVE_LINES")]
    #[doc = "Godot enumerator name: `PRIMITIVE_LINES`"]
    pub const LINES: PrimitiveType = PrimitiveType {
        ord: 1i32
    };
    #[doc(alias = "PRIMITIVE_LINE_STRIP")]
    #[doc = "Godot enumerator name: `PRIMITIVE_LINE_STRIP`"]
    pub const LINE_STRIP: PrimitiveType = PrimitiveType {
        ord: 2i32
    };
    #[doc(alias = "PRIMITIVE_TRIANGLES")]
    #[doc = "Godot enumerator name: `PRIMITIVE_TRIANGLES`"]
    pub const TRIANGLES: PrimitiveType = PrimitiveType {
        ord: 3i32
    };
    #[doc(alias = "PRIMITIVE_TRIANGLE_STRIP")]
    #[doc = "Godot enumerator name: `PRIMITIVE_TRIANGLE_STRIP`"]
    pub const TRIANGLE_STRIP: PrimitiveType = PrimitiveType {
        ord: 4i32
    };
    #[doc(alias = "PRIMITIVE_MAX")]
    #[doc = "Godot enumerator name: `PRIMITIVE_MAX`"]
    pub const MAX: PrimitiveType = PrimitiveType {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PrimitiveType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PrimitiveType {
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
            Self::POINTS => "POINTS", Self::LINES => "LINES", Self::LINE_STRIP => "LINE_STRIP", Self::TRIANGLES => "TRIANGLES", Self::TRIANGLE_STRIP => "TRIANGLE_STRIP", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PrimitiveType::POINTS, PrimitiveType::LINES, PrimitiveType::LINE_STRIP, PrimitiveType::TRIANGLES, PrimitiveType::TRIANGLE_STRIP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PrimitiveType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("POINTS", "PRIMITIVE_POINTS", PrimitiveType::POINTS), crate::meta::inspect::EnumConstant::new("LINES", "PRIMITIVE_LINES", PrimitiveType::LINES), crate::meta::inspect::EnumConstant::new("LINE_STRIP", "PRIMITIVE_LINE_STRIP", PrimitiveType::LINE_STRIP), crate::meta::inspect::EnumConstant::new("TRIANGLES", "PRIMITIVE_TRIANGLES", PrimitiveType::TRIANGLES), crate::meta::inspect::EnumConstant::new("TRIANGLE_STRIP", "PRIMITIVE_TRIANGLE_STRIP", PrimitiveType::TRIANGLE_STRIP), crate::meta::inspect::EnumConstant::new("MAX", "PRIMITIVE_MAX", PrimitiveType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for PrimitiveType {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for PrimitiveType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PrimitiveType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PrimitiveType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlendShapeMode {
    ord: i32
}
impl BlendShapeMode {
    #[doc(alias = "BLEND_SHAPE_MODE_NORMALIZED")]
    #[doc = "Godot enumerator name: `BLEND_SHAPE_MODE_NORMALIZED`"]
    pub const NORMALIZED: BlendShapeMode = BlendShapeMode {
        ord: 0i32
    };
    #[doc(alias = "BLEND_SHAPE_MODE_RELATIVE")]
    #[doc = "Godot enumerator name: `BLEND_SHAPE_MODE_RELATIVE`"]
    pub const RELATIVE: BlendShapeMode = BlendShapeMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for BlendShapeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BlendShapeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BlendShapeMode {
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
            Self::NORMALIZED => "NORMALIZED", Self::RELATIVE => "RELATIVE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BlendShapeMode::NORMALIZED, BlendShapeMode::RELATIVE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BlendShapeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NORMALIZED", "BLEND_SHAPE_MODE_NORMALIZED", BlendShapeMode::NORMALIZED), crate::meta::inspect::EnumConstant::new("RELATIVE", "BLEND_SHAPE_MODE_RELATIVE", BlendShapeMode::RELATIVE)]
        }
    }
}
impl crate::meta::GodotConvert for BlendShapeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BlendShapeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BlendShapeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MultimeshTransformFormat {
    ord: i32
}
impl MultimeshTransformFormat {
    #[doc(alias = "MULTIMESH_TRANSFORM_2D")]
    #[doc = "Godot enumerator name: `MULTIMESH_TRANSFORM_2D`"]
    pub const TRANSFORM_2D: MultimeshTransformFormat = MultimeshTransformFormat {
        ord: 0i32
    };
    #[doc(alias = "MULTIMESH_TRANSFORM_3D")]
    #[doc = "Godot enumerator name: `MULTIMESH_TRANSFORM_3D`"]
    pub const TRANSFORM_3D: MultimeshTransformFormat = MultimeshTransformFormat {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for MultimeshTransformFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MultimeshTransformFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MultimeshTransformFormat {
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
            Self::TRANSFORM_2D => "TRANSFORM_2D", Self::TRANSFORM_3D => "TRANSFORM_3D", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MultimeshTransformFormat::TRANSFORM_2D, MultimeshTransformFormat::TRANSFORM_3D]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MultimeshTransformFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TRANSFORM_2D", "MULTIMESH_TRANSFORM_2D", MultimeshTransformFormat::TRANSFORM_2D), crate::meta::inspect::EnumConstant::new("TRANSFORM_3D", "MULTIMESH_TRANSFORM_3D", MultimeshTransformFormat::TRANSFORM_3D)]
        }
    }
}
impl crate::meta::GodotConvert for MultimeshTransformFormat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MultimeshTransformFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MultimeshTransformFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct MultimeshPhysicsInterpolationQuality {
    ord: i32
}
impl MultimeshPhysicsInterpolationQuality {
    #[doc(alias = "MULTIMESH_INTERP_QUALITY_FAST")]
    #[doc = "Godot enumerator name: `MULTIMESH_INTERP_QUALITY_FAST`"]
    pub const FAST: MultimeshPhysicsInterpolationQuality = MultimeshPhysicsInterpolationQuality {
        ord: 0i32
    };
    #[doc(alias = "MULTIMESH_INTERP_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `MULTIMESH_INTERP_QUALITY_HIGH`"]
    pub const HIGH: MultimeshPhysicsInterpolationQuality = MultimeshPhysicsInterpolationQuality {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for MultimeshPhysicsInterpolationQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("MultimeshPhysicsInterpolationQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for MultimeshPhysicsInterpolationQuality {
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
            Self::FAST => "FAST", Self::HIGH => "HIGH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[MultimeshPhysicsInterpolationQuality::FAST, MultimeshPhysicsInterpolationQuality::HIGH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < MultimeshPhysicsInterpolationQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FAST", "MULTIMESH_INTERP_QUALITY_FAST", MultimeshPhysicsInterpolationQuality::FAST), crate::meta::inspect::EnumConstant::new("HIGH", "MULTIMESH_INTERP_QUALITY_HIGH", MultimeshPhysicsInterpolationQuality::HIGH)]
        }
    }
}
impl crate::meta::GodotConvert for MultimeshPhysicsInterpolationQuality {
    type Via = i32;
    
}
impl crate::meta::ToGodot for MultimeshPhysicsInterpolationQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for MultimeshPhysicsInterpolationQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightProjectorFilter {
    ord: i32
}
impl LightProjectorFilter {
    #[doc(alias = "LIGHT_PROJECTOR_FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `LIGHT_PROJECTOR_FILTER_NEAREST`"]
    pub const NEAREST: LightProjectorFilter = LightProjectorFilter {
        ord: 0i32
    };
    #[doc(alias = "LIGHT_PROJECTOR_FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `LIGHT_PROJECTOR_FILTER_LINEAR`"]
    pub const LINEAR: LightProjectorFilter = LightProjectorFilter {
        ord: 1i32
    };
    #[doc(alias = "LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS")]
    #[doc = "Godot enumerator name: `LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS`"]
    pub const NEAREST_MIPMAPS: LightProjectorFilter = LightProjectorFilter {
        ord: 2i32
    };
    #[doc(alias = "LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS")]
    #[doc = "Godot enumerator name: `LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS`"]
    pub const LINEAR_MIPMAPS: LightProjectorFilter = LightProjectorFilter {
        ord: 3i32
    };
    #[doc(alias = "LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS_ANISOTROPIC`"]
    pub const NEAREST_MIPMAPS_ANISOTROPIC: LightProjectorFilter = LightProjectorFilter {
        ord: 4i32
    };
    #[doc(alias = "LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS_ANISOTROPIC`"]
    pub const LINEAR_MIPMAPS_ANISOTROPIC: LightProjectorFilter = LightProjectorFilter {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for LightProjectorFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightProjectorFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightProjectorFilter {
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
            Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", Self::NEAREST_MIPMAPS => "NEAREST_MIPMAPS", Self::LINEAR_MIPMAPS => "LINEAR_MIPMAPS", Self::NEAREST_MIPMAPS_ANISOTROPIC => "NEAREST_MIPMAPS_ANISOTROPIC", Self::LINEAR_MIPMAPS_ANISOTROPIC => "LINEAR_MIPMAPS_ANISOTROPIC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightProjectorFilter::NEAREST, LightProjectorFilter::LINEAR, LightProjectorFilter::NEAREST_MIPMAPS, LightProjectorFilter::LINEAR_MIPMAPS, LightProjectorFilter::NEAREST_MIPMAPS_ANISOTROPIC, LightProjectorFilter::LINEAR_MIPMAPS_ANISOTROPIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightProjectorFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEAREST", "LIGHT_PROJECTOR_FILTER_NEAREST", LightProjectorFilter::NEAREST), crate::meta::inspect::EnumConstant::new("LINEAR", "LIGHT_PROJECTOR_FILTER_LINEAR", LightProjectorFilter::LINEAR), crate::meta::inspect::EnumConstant::new("NEAREST_MIPMAPS", "LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS", LightProjectorFilter::NEAREST_MIPMAPS), crate::meta::inspect::EnumConstant::new("LINEAR_MIPMAPS", "LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS", LightProjectorFilter::LINEAR_MIPMAPS), crate::meta::inspect::EnumConstant::new("NEAREST_MIPMAPS_ANISOTROPIC", "LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS_ANISOTROPIC", LightProjectorFilter::NEAREST_MIPMAPS_ANISOTROPIC), crate::meta::inspect::EnumConstant::new("LINEAR_MIPMAPS_ANISOTROPIC", "LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS_ANISOTROPIC", LightProjectorFilter::LINEAR_MIPMAPS_ANISOTROPIC)]
        }
    }
}
impl crate::meta::GodotConvert for LightProjectorFilter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LightProjectorFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightProjectorFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightType {
    ord: i32
}
impl LightType {
    #[doc(alias = "LIGHT_DIRECTIONAL")]
    #[doc = "Godot enumerator name: `LIGHT_DIRECTIONAL`"]
    pub const DIRECTIONAL: LightType = LightType {
        ord: 0i32
    };
    #[doc(alias = "LIGHT_OMNI")]
    #[doc = "Godot enumerator name: `LIGHT_OMNI`"]
    pub const OMNI: LightType = LightType {
        ord: 1i32
    };
    #[doc(alias = "LIGHT_SPOT")]
    #[doc = "Godot enumerator name: `LIGHT_SPOT`"]
    pub const SPOT: LightType = LightType {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for LightType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightType {
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
            Self::DIRECTIONAL => "DIRECTIONAL", Self::OMNI => "OMNI", Self::SPOT => "SPOT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightType::DIRECTIONAL, LightType::OMNI, LightType::SPOT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DIRECTIONAL", "LIGHT_DIRECTIONAL", LightType::DIRECTIONAL), crate::meta::inspect::EnumConstant::new("OMNI", "LIGHT_OMNI", LightType::OMNI), crate::meta::inspect::EnumConstant::new("SPOT", "LIGHT_SPOT", LightType::SPOT)]
        }
    }
}
impl crate::meta::GodotConvert for LightType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LightType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightParam {
    ord: i32
}
impl LightParam {
    #[doc(alias = "LIGHT_PARAM_ENERGY")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_ENERGY`"]
    pub const ENERGY: LightParam = LightParam {
        ord: 0i32
    };
    #[doc(alias = "LIGHT_PARAM_INDIRECT_ENERGY")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_INDIRECT_ENERGY`"]
    pub const INDIRECT_ENERGY: LightParam = LightParam {
        ord: 1i32
    };
    #[doc(alias = "LIGHT_PARAM_VOLUMETRIC_FOG_ENERGY")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_VOLUMETRIC_FOG_ENERGY`"]
    pub const VOLUMETRIC_FOG_ENERGY: LightParam = LightParam {
        ord: 2i32
    };
    #[doc(alias = "LIGHT_PARAM_SPECULAR")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SPECULAR`"]
    pub const SPECULAR: LightParam = LightParam {
        ord: 3i32
    };
    #[doc(alias = "LIGHT_PARAM_RANGE")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_RANGE`"]
    pub const RANGE: LightParam = LightParam {
        ord: 4i32
    };
    #[doc(alias = "LIGHT_PARAM_SIZE")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SIZE`"]
    pub const SIZE: LightParam = LightParam {
        ord: 5i32
    };
    #[doc(alias = "LIGHT_PARAM_ATTENUATION")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_ATTENUATION`"]
    pub const ATTENUATION: LightParam = LightParam {
        ord: 6i32
    };
    #[doc(alias = "LIGHT_PARAM_SPOT_ANGLE")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SPOT_ANGLE`"]
    pub const SPOT_ANGLE: LightParam = LightParam {
        ord: 7i32
    };
    #[doc(alias = "LIGHT_PARAM_SPOT_ATTENUATION")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SPOT_ATTENUATION`"]
    pub const SPOT_ATTENUATION: LightParam = LightParam {
        ord: 8i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_MAX_DISTANCE")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_MAX_DISTANCE`"]
    pub const SHADOW_MAX_DISTANCE: LightParam = LightParam {
        ord: 9i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_SPLIT_1_OFFSET")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_SPLIT_1_OFFSET`"]
    pub const SHADOW_SPLIT_1_OFFSET: LightParam = LightParam {
        ord: 10i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_SPLIT_2_OFFSET")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_SPLIT_2_OFFSET`"]
    pub const SHADOW_SPLIT_2_OFFSET: LightParam = LightParam {
        ord: 11i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_SPLIT_3_OFFSET")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_SPLIT_3_OFFSET`"]
    pub const SHADOW_SPLIT_3_OFFSET: LightParam = LightParam {
        ord: 12i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_FADE_START")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_FADE_START`"]
    pub const SHADOW_FADE_START: LightParam = LightParam {
        ord: 13i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_NORMAL_BIAS")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_NORMAL_BIAS`"]
    pub const SHADOW_NORMAL_BIAS: LightParam = LightParam {
        ord: 14i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_BIAS")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_BIAS`"]
    pub const SHADOW_BIAS: LightParam = LightParam {
        ord: 15i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_PANCAKE_SIZE")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_PANCAKE_SIZE`"]
    pub const SHADOW_PANCAKE_SIZE: LightParam = LightParam {
        ord: 16i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_OPACITY")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_OPACITY`"]
    pub const SHADOW_OPACITY: LightParam = LightParam {
        ord: 17i32
    };
    #[doc(alias = "LIGHT_PARAM_SHADOW_BLUR")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_SHADOW_BLUR`"]
    pub const SHADOW_BLUR: LightParam = LightParam {
        ord: 18i32
    };
    #[doc(alias = "LIGHT_PARAM_TRANSMITTANCE_BIAS")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_TRANSMITTANCE_BIAS`"]
    pub const TRANSMITTANCE_BIAS: LightParam = LightParam {
        ord: 19i32
    };
    #[doc(alias = "LIGHT_PARAM_INTENSITY")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_INTENSITY`"]
    pub const INTENSITY: LightParam = LightParam {
        ord: 20i32
    };
    #[doc(alias = "LIGHT_PARAM_MAX")]
    #[doc = "Godot enumerator name: `LIGHT_PARAM_MAX`"]
    pub const MAX: LightParam = LightParam {
        ord: 21i32
    };
    
}
impl std::fmt::Debug for LightParam {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightParam") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightParam {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 => Some(Self {
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
            Self::ENERGY => "ENERGY", Self::INDIRECT_ENERGY => "INDIRECT_ENERGY", Self::VOLUMETRIC_FOG_ENERGY => "VOLUMETRIC_FOG_ENERGY", Self::SPECULAR => "SPECULAR", Self::RANGE => "RANGE", Self::SIZE => "SIZE", Self::ATTENUATION => "ATTENUATION", Self::SPOT_ANGLE => "SPOT_ANGLE", Self::SPOT_ATTENUATION => "SPOT_ATTENUATION", Self::SHADOW_MAX_DISTANCE => "SHADOW_MAX_DISTANCE", Self::SHADOW_SPLIT_1_OFFSET => "SHADOW_SPLIT_1_OFFSET", Self::SHADOW_SPLIT_2_OFFSET => "SHADOW_SPLIT_2_OFFSET", Self::SHADOW_SPLIT_3_OFFSET => "SHADOW_SPLIT_3_OFFSET", Self::SHADOW_FADE_START => "SHADOW_FADE_START", Self::SHADOW_NORMAL_BIAS => "SHADOW_NORMAL_BIAS", Self::SHADOW_BIAS => "SHADOW_BIAS", Self::SHADOW_PANCAKE_SIZE => "SHADOW_PANCAKE_SIZE", Self::SHADOW_OPACITY => "SHADOW_OPACITY", Self::SHADOW_BLUR => "SHADOW_BLUR", Self::TRANSMITTANCE_BIAS => "TRANSMITTANCE_BIAS", Self::INTENSITY => "INTENSITY", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightParam::ENERGY, LightParam::INDIRECT_ENERGY, LightParam::VOLUMETRIC_FOG_ENERGY, LightParam::SPECULAR, LightParam::RANGE, LightParam::SIZE, LightParam::ATTENUATION, LightParam::SPOT_ANGLE, LightParam::SPOT_ATTENUATION, LightParam::SHADOW_MAX_DISTANCE, LightParam::SHADOW_SPLIT_1_OFFSET, LightParam::SHADOW_SPLIT_2_OFFSET, LightParam::SHADOW_SPLIT_3_OFFSET, LightParam::SHADOW_FADE_START, LightParam::SHADOW_NORMAL_BIAS, LightParam::SHADOW_BIAS, LightParam::SHADOW_PANCAKE_SIZE, LightParam::SHADOW_OPACITY, LightParam::SHADOW_BLUR, LightParam::TRANSMITTANCE_BIAS, LightParam::INTENSITY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightParam >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ENERGY", "LIGHT_PARAM_ENERGY", LightParam::ENERGY), crate::meta::inspect::EnumConstant::new("INDIRECT_ENERGY", "LIGHT_PARAM_INDIRECT_ENERGY", LightParam::INDIRECT_ENERGY), crate::meta::inspect::EnumConstant::new("VOLUMETRIC_FOG_ENERGY", "LIGHT_PARAM_VOLUMETRIC_FOG_ENERGY", LightParam::VOLUMETRIC_FOG_ENERGY), crate::meta::inspect::EnumConstant::new("SPECULAR", "LIGHT_PARAM_SPECULAR", LightParam::SPECULAR), crate::meta::inspect::EnumConstant::new("RANGE", "LIGHT_PARAM_RANGE", LightParam::RANGE), crate::meta::inspect::EnumConstant::new("SIZE", "LIGHT_PARAM_SIZE", LightParam::SIZE), crate::meta::inspect::EnumConstant::new("ATTENUATION", "LIGHT_PARAM_ATTENUATION", LightParam::ATTENUATION), crate::meta::inspect::EnumConstant::new("SPOT_ANGLE", "LIGHT_PARAM_SPOT_ANGLE", LightParam::SPOT_ANGLE), crate::meta::inspect::EnumConstant::new("SPOT_ATTENUATION", "LIGHT_PARAM_SPOT_ATTENUATION", LightParam::SPOT_ATTENUATION), crate::meta::inspect::EnumConstant::new("SHADOW_MAX_DISTANCE", "LIGHT_PARAM_SHADOW_MAX_DISTANCE", LightParam::SHADOW_MAX_DISTANCE), crate::meta::inspect::EnumConstant::new("SHADOW_SPLIT_1_OFFSET", "LIGHT_PARAM_SHADOW_SPLIT_1_OFFSET", LightParam::SHADOW_SPLIT_1_OFFSET), crate::meta::inspect::EnumConstant::new("SHADOW_SPLIT_2_OFFSET", "LIGHT_PARAM_SHADOW_SPLIT_2_OFFSET", LightParam::SHADOW_SPLIT_2_OFFSET), crate::meta::inspect::EnumConstant::new("SHADOW_SPLIT_3_OFFSET", "LIGHT_PARAM_SHADOW_SPLIT_3_OFFSET", LightParam::SHADOW_SPLIT_3_OFFSET), crate::meta::inspect::EnumConstant::new("SHADOW_FADE_START", "LIGHT_PARAM_SHADOW_FADE_START", LightParam::SHADOW_FADE_START), crate::meta::inspect::EnumConstant::new("SHADOW_NORMAL_BIAS", "LIGHT_PARAM_SHADOW_NORMAL_BIAS", LightParam::SHADOW_NORMAL_BIAS), crate::meta::inspect::EnumConstant::new("SHADOW_BIAS", "LIGHT_PARAM_SHADOW_BIAS", LightParam::SHADOW_BIAS), crate::meta::inspect::EnumConstant::new("SHADOW_PANCAKE_SIZE", "LIGHT_PARAM_SHADOW_PANCAKE_SIZE", LightParam::SHADOW_PANCAKE_SIZE), crate::meta::inspect::EnumConstant::new("SHADOW_OPACITY", "LIGHT_PARAM_SHADOW_OPACITY", LightParam::SHADOW_OPACITY), crate::meta::inspect::EnumConstant::new("SHADOW_BLUR", "LIGHT_PARAM_SHADOW_BLUR", LightParam::SHADOW_BLUR), crate::meta::inspect::EnumConstant::new("TRANSMITTANCE_BIAS", "LIGHT_PARAM_TRANSMITTANCE_BIAS", LightParam::TRANSMITTANCE_BIAS), crate::meta::inspect::EnumConstant::new("INTENSITY", "LIGHT_PARAM_INTENSITY", LightParam::INTENSITY), crate::meta::inspect::EnumConstant::new("MAX", "LIGHT_PARAM_MAX", LightParam::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for LightParam {
    const ENUMERATOR_COUNT: usize = 21usize;
    
}
impl crate::meta::GodotConvert for LightParam {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LightParam {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightParam {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightBakeMode {
    ord: i32
}
impl LightBakeMode {
    #[doc(alias = "LIGHT_BAKE_DISABLED")]
    #[doc = "Godot enumerator name: `LIGHT_BAKE_DISABLED`"]
    pub const DISABLED: LightBakeMode = LightBakeMode {
        ord: 0i32
    };
    #[doc(alias = "LIGHT_BAKE_STATIC")]
    #[doc = "Godot enumerator name: `LIGHT_BAKE_STATIC`"]
    pub const STATIC: LightBakeMode = LightBakeMode {
        ord: 1i32
    };
    #[doc(alias = "LIGHT_BAKE_DYNAMIC")]
    #[doc = "Godot enumerator name: `LIGHT_BAKE_DYNAMIC`"]
    pub const DYNAMIC: LightBakeMode = LightBakeMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for LightBakeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightBakeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightBakeMode {
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
            Self::DISABLED => "DISABLED", Self::STATIC => "STATIC", Self::DYNAMIC => "DYNAMIC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightBakeMode::DISABLED, LightBakeMode::STATIC, LightBakeMode::DYNAMIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightBakeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "LIGHT_BAKE_DISABLED", LightBakeMode::DISABLED), crate::meta::inspect::EnumConstant::new("STATIC", "LIGHT_BAKE_STATIC", LightBakeMode::STATIC), crate::meta::inspect::EnumConstant::new("DYNAMIC", "LIGHT_BAKE_DYNAMIC", LightBakeMode::DYNAMIC)]
        }
    }
}
impl crate::meta::GodotConvert for LightBakeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LightBakeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightBakeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightOmniShadowMode {
    ord: i32
}
impl LightOmniShadowMode {
    #[doc(alias = "LIGHT_OMNI_SHADOW_DUAL_PARABOLOID")]
    #[doc = "Godot enumerator name: `LIGHT_OMNI_SHADOW_DUAL_PARABOLOID`"]
    pub const DUAL_PARABOLOID: LightOmniShadowMode = LightOmniShadowMode {
        ord: 0i32
    };
    #[doc(alias = "LIGHT_OMNI_SHADOW_CUBE")]
    #[doc = "Godot enumerator name: `LIGHT_OMNI_SHADOW_CUBE`"]
    pub const CUBE: LightOmniShadowMode = LightOmniShadowMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for LightOmniShadowMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightOmniShadowMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightOmniShadowMode {
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
            Self::DUAL_PARABOLOID => "DUAL_PARABOLOID", Self::CUBE => "CUBE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightOmniShadowMode::DUAL_PARABOLOID, LightOmniShadowMode::CUBE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightOmniShadowMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DUAL_PARABOLOID", "LIGHT_OMNI_SHADOW_DUAL_PARABOLOID", LightOmniShadowMode::DUAL_PARABOLOID), crate::meta::inspect::EnumConstant::new("CUBE", "LIGHT_OMNI_SHADOW_CUBE", LightOmniShadowMode::CUBE)]
        }
    }
}
impl crate::meta::GodotConvert for LightOmniShadowMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LightOmniShadowMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightOmniShadowMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightDirectionalShadowMode {
    ord: i32
}
impl LightDirectionalShadowMode {
    #[doc(alias = "LIGHT_DIRECTIONAL_SHADOW_ORTHOGONAL")]
    #[doc = "Godot enumerator name: `LIGHT_DIRECTIONAL_SHADOW_ORTHOGONAL`"]
    pub const ORTHOGONAL: LightDirectionalShadowMode = LightDirectionalShadowMode {
        ord: 0i32
    };
    #[doc(alias = "LIGHT_DIRECTIONAL_SHADOW_PARALLEL_2_SPLITS")]
    #[doc = "Godot enumerator name: `LIGHT_DIRECTIONAL_SHADOW_PARALLEL_2_SPLITS`"]
    pub const PARALLEL_2_SPLITS: LightDirectionalShadowMode = LightDirectionalShadowMode {
        ord: 1i32
    };
    #[doc(alias = "LIGHT_DIRECTIONAL_SHADOW_PARALLEL_4_SPLITS")]
    #[doc = "Godot enumerator name: `LIGHT_DIRECTIONAL_SHADOW_PARALLEL_4_SPLITS`"]
    pub const PARALLEL_4_SPLITS: LightDirectionalShadowMode = LightDirectionalShadowMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for LightDirectionalShadowMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightDirectionalShadowMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightDirectionalShadowMode {
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
            Self::ORTHOGONAL => "ORTHOGONAL", Self::PARALLEL_2_SPLITS => "PARALLEL_2_SPLITS", Self::PARALLEL_4_SPLITS => "PARALLEL_4_SPLITS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightDirectionalShadowMode::ORTHOGONAL, LightDirectionalShadowMode::PARALLEL_2_SPLITS, LightDirectionalShadowMode::PARALLEL_4_SPLITS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightDirectionalShadowMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ORTHOGONAL", "LIGHT_DIRECTIONAL_SHADOW_ORTHOGONAL", LightDirectionalShadowMode::ORTHOGONAL), crate::meta::inspect::EnumConstant::new("PARALLEL_2_SPLITS", "LIGHT_DIRECTIONAL_SHADOW_PARALLEL_2_SPLITS", LightDirectionalShadowMode::PARALLEL_2_SPLITS), crate::meta::inspect::EnumConstant::new("PARALLEL_4_SPLITS", "LIGHT_DIRECTIONAL_SHADOW_PARALLEL_4_SPLITS", LightDirectionalShadowMode::PARALLEL_4_SPLITS)]
        }
    }
}
impl crate::meta::GodotConvert for LightDirectionalShadowMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LightDirectionalShadowMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightDirectionalShadowMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LightDirectionalSkyMode {
    ord: i32
}
impl LightDirectionalSkyMode {
    #[doc(alias = "LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_AND_SKY")]
    #[doc = "Godot enumerator name: `LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_AND_SKY`"]
    pub const LIGHT_AND_SKY: LightDirectionalSkyMode = LightDirectionalSkyMode {
        ord: 0i32
    };
    #[doc(alias = "LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_ONLY")]
    #[doc = "Godot enumerator name: `LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_ONLY`"]
    pub const LIGHT_ONLY: LightDirectionalSkyMode = LightDirectionalSkyMode {
        ord: 1i32
    };
    #[doc(alias = "LIGHT_DIRECTIONAL_SKY_MODE_SKY_ONLY")]
    #[doc = "Godot enumerator name: `LIGHT_DIRECTIONAL_SKY_MODE_SKY_ONLY`"]
    pub const SKY_ONLY: LightDirectionalSkyMode = LightDirectionalSkyMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for LightDirectionalSkyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LightDirectionalSkyMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LightDirectionalSkyMode {
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
            Self::LIGHT_AND_SKY => "LIGHT_AND_SKY", Self::LIGHT_ONLY => "LIGHT_ONLY", Self::SKY_ONLY => "SKY_ONLY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LightDirectionalSkyMode::LIGHT_AND_SKY, LightDirectionalSkyMode::LIGHT_ONLY, LightDirectionalSkyMode::SKY_ONLY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LightDirectionalSkyMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LIGHT_AND_SKY", "LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_AND_SKY", LightDirectionalSkyMode::LIGHT_AND_SKY), crate::meta::inspect::EnumConstant::new("LIGHT_ONLY", "LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_ONLY", LightDirectionalSkyMode::LIGHT_ONLY), crate::meta::inspect::EnumConstant::new("SKY_ONLY", "LIGHT_DIRECTIONAL_SKY_MODE_SKY_ONLY", LightDirectionalSkyMode::SKY_ONLY)]
        }
    }
}
impl crate::meta::GodotConvert for LightDirectionalSkyMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LightDirectionalSkyMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LightDirectionalSkyMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShadowQuality {
    ord: i32
}
impl ShadowQuality {
    #[doc(alias = "SHADOW_QUALITY_HARD")]
    #[doc = "Godot enumerator name: `SHADOW_QUALITY_HARD`"]
    pub const HARD: ShadowQuality = ShadowQuality {
        ord: 0i32
    };
    #[doc(alias = "SHADOW_QUALITY_SOFT_VERY_LOW")]
    #[doc = "Godot enumerator name: `SHADOW_QUALITY_SOFT_VERY_LOW`"]
    pub const SOFT_VERY_LOW: ShadowQuality = ShadowQuality {
        ord: 1i32
    };
    #[doc(alias = "SHADOW_QUALITY_SOFT_LOW")]
    #[doc = "Godot enumerator name: `SHADOW_QUALITY_SOFT_LOW`"]
    pub const SOFT_LOW: ShadowQuality = ShadowQuality {
        ord: 2i32
    };
    #[doc(alias = "SHADOW_QUALITY_SOFT_MEDIUM")]
    #[doc = "Godot enumerator name: `SHADOW_QUALITY_SOFT_MEDIUM`"]
    pub const SOFT_MEDIUM: ShadowQuality = ShadowQuality {
        ord: 3i32
    };
    #[doc(alias = "SHADOW_QUALITY_SOFT_HIGH")]
    #[doc = "Godot enumerator name: `SHADOW_QUALITY_SOFT_HIGH`"]
    pub const SOFT_HIGH: ShadowQuality = ShadowQuality {
        ord: 4i32
    };
    #[doc(alias = "SHADOW_QUALITY_SOFT_ULTRA")]
    #[doc = "Godot enumerator name: `SHADOW_QUALITY_SOFT_ULTRA`"]
    pub const SOFT_ULTRA: ShadowQuality = ShadowQuality {
        ord: 5i32
    };
    #[doc(alias = "SHADOW_QUALITY_MAX")]
    #[doc = "Godot enumerator name: `SHADOW_QUALITY_MAX`"]
    pub const MAX: ShadowQuality = ShadowQuality {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for ShadowQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShadowQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShadowQuality {
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
            Self::HARD => "HARD", Self::SOFT_VERY_LOW => "SOFT_VERY_LOW", Self::SOFT_LOW => "SOFT_LOW", Self::SOFT_MEDIUM => "SOFT_MEDIUM", Self::SOFT_HIGH => "SOFT_HIGH", Self::SOFT_ULTRA => "SOFT_ULTRA", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ShadowQuality::HARD, ShadowQuality::SOFT_VERY_LOW, ShadowQuality::SOFT_LOW, ShadowQuality::SOFT_MEDIUM, ShadowQuality::SOFT_HIGH, ShadowQuality::SOFT_ULTRA]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ShadowQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("HARD", "SHADOW_QUALITY_HARD", ShadowQuality::HARD), crate::meta::inspect::EnumConstant::new("SOFT_VERY_LOW", "SHADOW_QUALITY_SOFT_VERY_LOW", ShadowQuality::SOFT_VERY_LOW), crate::meta::inspect::EnumConstant::new("SOFT_LOW", "SHADOW_QUALITY_SOFT_LOW", ShadowQuality::SOFT_LOW), crate::meta::inspect::EnumConstant::new("SOFT_MEDIUM", "SHADOW_QUALITY_SOFT_MEDIUM", ShadowQuality::SOFT_MEDIUM), crate::meta::inspect::EnumConstant::new("SOFT_HIGH", "SHADOW_QUALITY_SOFT_HIGH", ShadowQuality::SOFT_HIGH), crate::meta::inspect::EnumConstant::new("SOFT_ULTRA", "SHADOW_QUALITY_SOFT_ULTRA", ShadowQuality::SOFT_ULTRA), crate::meta::inspect::EnumConstant::new("MAX", "SHADOW_QUALITY_MAX", ShadowQuality::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ShadowQuality {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for ShadowQuality {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ShadowQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShadowQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ReflectionProbeUpdateMode {
    ord: i32
}
impl ReflectionProbeUpdateMode {
    #[doc(alias = "REFLECTION_PROBE_UPDATE_ONCE")]
    #[doc = "Godot enumerator name: `REFLECTION_PROBE_UPDATE_ONCE`"]
    pub const ONCE: ReflectionProbeUpdateMode = ReflectionProbeUpdateMode {
        ord: 0i32
    };
    #[doc(alias = "REFLECTION_PROBE_UPDATE_ALWAYS")]
    #[doc = "Godot enumerator name: `REFLECTION_PROBE_UPDATE_ALWAYS`"]
    pub const ALWAYS: ReflectionProbeUpdateMode = ReflectionProbeUpdateMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for ReflectionProbeUpdateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ReflectionProbeUpdateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ReflectionProbeUpdateMode {
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
            Self::ONCE => "ONCE", Self::ALWAYS => "ALWAYS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ReflectionProbeUpdateMode::ONCE, ReflectionProbeUpdateMode::ALWAYS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ReflectionProbeUpdateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ONCE", "REFLECTION_PROBE_UPDATE_ONCE", ReflectionProbeUpdateMode::ONCE), crate::meta::inspect::EnumConstant::new("ALWAYS", "REFLECTION_PROBE_UPDATE_ALWAYS", ReflectionProbeUpdateMode::ALWAYS)]
        }
    }
}
impl crate::meta::GodotConvert for ReflectionProbeUpdateMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ReflectionProbeUpdateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ReflectionProbeUpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ReflectionProbeAmbientMode {
    ord: i32
}
impl ReflectionProbeAmbientMode {
    #[doc(alias = "REFLECTION_PROBE_AMBIENT_DISABLED")]
    #[doc = "Godot enumerator name: `REFLECTION_PROBE_AMBIENT_DISABLED`"]
    pub const DISABLED: ReflectionProbeAmbientMode = ReflectionProbeAmbientMode {
        ord: 0i32
    };
    #[doc(alias = "REFLECTION_PROBE_AMBIENT_ENVIRONMENT")]
    #[doc = "Godot enumerator name: `REFLECTION_PROBE_AMBIENT_ENVIRONMENT`"]
    pub const ENVIRONMENT: ReflectionProbeAmbientMode = ReflectionProbeAmbientMode {
        ord: 1i32
    };
    #[doc(alias = "REFLECTION_PROBE_AMBIENT_COLOR")]
    #[doc = "Godot enumerator name: `REFLECTION_PROBE_AMBIENT_COLOR`"]
    pub const COLOR: ReflectionProbeAmbientMode = ReflectionProbeAmbientMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ReflectionProbeAmbientMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ReflectionProbeAmbientMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ReflectionProbeAmbientMode {
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
            Self::DISABLED => "DISABLED", Self::ENVIRONMENT => "ENVIRONMENT", Self::COLOR => "COLOR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ReflectionProbeAmbientMode::DISABLED, ReflectionProbeAmbientMode::ENVIRONMENT, ReflectionProbeAmbientMode::COLOR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ReflectionProbeAmbientMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "REFLECTION_PROBE_AMBIENT_DISABLED", ReflectionProbeAmbientMode::DISABLED), crate::meta::inspect::EnumConstant::new("ENVIRONMENT", "REFLECTION_PROBE_AMBIENT_ENVIRONMENT", ReflectionProbeAmbientMode::ENVIRONMENT), crate::meta::inspect::EnumConstant::new("COLOR", "REFLECTION_PROBE_AMBIENT_COLOR", ReflectionProbeAmbientMode::COLOR)]
        }
    }
}
impl crate::meta::GodotConvert for ReflectionProbeAmbientMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ReflectionProbeAmbientMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ReflectionProbeAmbientMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DecalTexture {
    ord: i32
}
impl DecalTexture {
    #[doc(alias = "DECAL_TEXTURE_ALBEDO")]
    #[doc = "Godot enumerator name: `DECAL_TEXTURE_ALBEDO`"]
    pub const ALBEDO: DecalTexture = DecalTexture {
        ord: 0i32
    };
    #[doc(alias = "DECAL_TEXTURE_NORMAL")]
    #[doc = "Godot enumerator name: `DECAL_TEXTURE_NORMAL`"]
    pub const NORMAL: DecalTexture = DecalTexture {
        ord: 1i32
    };
    #[doc(alias = "DECAL_TEXTURE_ORM")]
    #[doc = "Godot enumerator name: `DECAL_TEXTURE_ORM`"]
    pub const ORM: DecalTexture = DecalTexture {
        ord: 2i32
    };
    #[doc(alias = "DECAL_TEXTURE_EMISSION")]
    #[doc = "Godot enumerator name: `DECAL_TEXTURE_EMISSION`"]
    pub const EMISSION: DecalTexture = DecalTexture {
        ord: 3i32
    };
    #[doc(alias = "DECAL_TEXTURE_MAX")]
    #[doc = "Godot enumerator name: `DECAL_TEXTURE_MAX`"]
    pub const MAX: DecalTexture = DecalTexture {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for DecalTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DecalTexture") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DecalTexture {
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
            Self::ALBEDO => "ALBEDO", Self::NORMAL => "NORMAL", Self::ORM => "ORM", Self::EMISSION => "EMISSION", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DecalTexture::ALBEDO, DecalTexture::NORMAL, DecalTexture::ORM, DecalTexture::EMISSION]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DecalTexture >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ALBEDO", "DECAL_TEXTURE_ALBEDO", DecalTexture::ALBEDO), crate::meta::inspect::EnumConstant::new("NORMAL", "DECAL_TEXTURE_NORMAL", DecalTexture::NORMAL), crate::meta::inspect::EnumConstant::new("ORM", "DECAL_TEXTURE_ORM", DecalTexture::ORM), crate::meta::inspect::EnumConstant::new("EMISSION", "DECAL_TEXTURE_EMISSION", DecalTexture::EMISSION), crate::meta::inspect::EnumConstant::new("MAX", "DECAL_TEXTURE_MAX", DecalTexture::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for DecalTexture {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for DecalTexture {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DecalTexture {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DecalTexture {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DecalFilter {
    ord: i32
}
impl DecalFilter {
    #[doc(alias = "DECAL_FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `DECAL_FILTER_NEAREST`"]
    pub const NEAREST: DecalFilter = DecalFilter {
        ord: 0i32
    };
    #[doc(alias = "DECAL_FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `DECAL_FILTER_LINEAR`"]
    pub const LINEAR: DecalFilter = DecalFilter {
        ord: 1i32
    };
    #[doc(alias = "DECAL_FILTER_NEAREST_MIPMAPS")]
    #[doc = "Godot enumerator name: `DECAL_FILTER_NEAREST_MIPMAPS`"]
    pub const NEAREST_MIPMAPS: DecalFilter = DecalFilter {
        ord: 2i32
    };
    #[doc(alias = "DECAL_FILTER_LINEAR_MIPMAPS")]
    #[doc = "Godot enumerator name: `DECAL_FILTER_LINEAR_MIPMAPS`"]
    pub const LINEAR_MIPMAPS: DecalFilter = DecalFilter {
        ord: 3i32
    };
    #[doc(alias = "DECAL_FILTER_NEAREST_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `DECAL_FILTER_NEAREST_MIPMAPS_ANISOTROPIC`"]
    pub const NEAREST_MIPMAPS_ANISOTROPIC: DecalFilter = DecalFilter {
        ord: 4i32
    };
    #[doc(alias = "DECAL_FILTER_LINEAR_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `DECAL_FILTER_LINEAR_MIPMAPS_ANISOTROPIC`"]
    pub const LINEAR_MIPMAPS_ANISOTROPIC: DecalFilter = DecalFilter {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for DecalFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DecalFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DecalFilter {
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
            Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", Self::NEAREST_MIPMAPS => "NEAREST_MIPMAPS", Self::LINEAR_MIPMAPS => "LINEAR_MIPMAPS", Self::NEAREST_MIPMAPS_ANISOTROPIC => "NEAREST_MIPMAPS_ANISOTROPIC", Self::LINEAR_MIPMAPS_ANISOTROPIC => "LINEAR_MIPMAPS_ANISOTROPIC", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DecalFilter::NEAREST, DecalFilter::LINEAR, DecalFilter::NEAREST_MIPMAPS, DecalFilter::LINEAR_MIPMAPS, DecalFilter::NEAREST_MIPMAPS_ANISOTROPIC, DecalFilter::LINEAR_MIPMAPS_ANISOTROPIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DecalFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEAREST", "DECAL_FILTER_NEAREST", DecalFilter::NEAREST), crate::meta::inspect::EnumConstant::new("LINEAR", "DECAL_FILTER_LINEAR", DecalFilter::LINEAR), crate::meta::inspect::EnumConstant::new("NEAREST_MIPMAPS", "DECAL_FILTER_NEAREST_MIPMAPS", DecalFilter::NEAREST_MIPMAPS), crate::meta::inspect::EnumConstant::new("LINEAR_MIPMAPS", "DECAL_FILTER_LINEAR_MIPMAPS", DecalFilter::LINEAR_MIPMAPS), crate::meta::inspect::EnumConstant::new("NEAREST_MIPMAPS_ANISOTROPIC", "DECAL_FILTER_NEAREST_MIPMAPS_ANISOTROPIC", DecalFilter::NEAREST_MIPMAPS_ANISOTROPIC), crate::meta::inspect::EnumConstant::new("LINEAR_MIPMAPS_ANISOTROPIC", "DECAL_FILTER_LINEAR_MIPMAPS_ANISOTROPIC", DecalFilter::LINEAR_MIPMAPS_ANISOTROPIC)]
        }
    }
}
impl crate::meta::GodotConvert for DecalFilter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DecalFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DecalFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `VoxelGIQuality`."]
pub struct VoxelGiQuality {
    ord: i32
}
impl VoxelGiQuality {
    #[doc(alias = "VOXEL_GI_QUALITY_LOW")]
    #[doc = "Godot enumerator name: `VOXEL_GI_QUALITY_LOW`"]
    pub const LOW: VoxelGiQuality = VoxelGiQuality {
        ord: 0i32
    };
    #[doc(alias = "VOXEL_GI_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `VOXEL_GI_QUALITY_HIGH`"]
    pub const HIGH: VoxelGiQuality = VoxelGiQuality {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for VoxelGiQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VoxelGiQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VoxelGiQuality {
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
            Self::LOW => "LOW", Self::HIGH => "HIGH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VoxelGiQuality::LOW, VoxelGiQuality::HIGH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VoxelGiQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LOW", "VOXEL_GI_QUALITY_LOW", VoxelGiQuality::LOW), crate::meta::inspect::EnumConstant::new("HIGH", "VOXEL_GI_QUALITY_HIGH", VoxelGiQuality::HIGH)]
        }
    }
}
impl crate::meta::GodotConvert for VoxelGiQuality {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VoxelGiQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VoxelGiQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ParticlesMode {
    ord: i32
}
impl ParticlesMode {
    #[doc(alias = "PARTICLES_MODE_2D")]
    #[doc = "Godot enumerator name: `PARTICLES_MODE_2D`"]
    pub const MODE_2D: ParticlesMode = ParticlesMode {
        ord: 0i32
    };
    #[doc(alias = "PARTICLES_MODE_3D")]
    #[doc = "Godot enumerator name: `PARTICLES_MODE_3D`"]
    pub const MODE_3D: ParticlesMode = ParticlesMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for ParticlesMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ParticlesMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ParticlesMode {
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
            Self::MODE_2D => "MODE_2D", Self::MODE_3D => "MODE_3D", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ParticlesMode::MODE_2D, ParticlesMode::MODE_3D]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ParticlesMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("MODE_2D", "PARTICLES_MODE_2D", ParticlesMode::MODE_2D), crate::meta::inspect::EnumConstant::new("MODE_3D", "PARTICLES_MODE_3D", ParticlesMode::MODE_3D)]
        }
    }
}
impl crate::meta::GodotConvert for ParticlesMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ParticlesMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ParticlesMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ParticlesTransformAlign {
    ord: i32
}
impl ParticlesTransformAlign {
    #[doc(alias = "PARTICLES_TRANSFORM_ALIGN_DISABLED")]
    #[doc = "Godot enumerator name: `PARTICLES_TRANSFORM_ALIGN_DISABLED`"]
    pub const DISABLED: ParticlesTransformAlign = ParticlesTransformAlign {
        ord: 0i32
    };
    #[doc(alias = "PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD")]
    #[doc = "Godot enumerator name: `PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD`"]
    pub const Z_BILLBOARD: ParticlesTransformAlign = ParticlesTransformAlign {
        ord: 1i32
    };
    #[doc(alias = "PARTICLES_TRANSFORM_ALIGN_Y_TO_VELOCITY")]
    #[doc = "Godot enumerator name: `PARTICLES_TRANSFORM_ALIGN_Y_TO_VELOCITY`"]
    pub const Y_TO_VELOCITY: ParticlesTransformAlign = ParticlesTransformAlign {
        ord: 2i32
    };
    #[doc(alias = "PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY")]
    #[doc = "Godot enumerator name: `PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY`"]
    pub const Z_BILLBOARD_Y_TO_VELOCITY: ParticlesTransformAlign = ParticlesTransformAlign {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ParticlesTransformAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ParticlesTransformAlign") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ParticlesTransformAlign {
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
            Self::DISABLED => "DISABLED", Self::Z_BILLBOARD => "Z_BILLBOARD", Self::Y_TO_VELOCITY => "Y_TO_VELOCITY", Self::Z_BILLBOARD_Y_TO_VELOCITY => "Z_BILLBOARD_Y_TO_VELOCITY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ParticlesTransformAlign::DISABLED, ParticlesTransformAlign::Z_BILLBOARD, ParticlesTransformAlign::Y_TO_VELOCITY, ParticlesTransformAlign::Z_BILLBOARD_Y_TO_VELOCITY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ParticlesTransformAlign >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "PARTICLES_TRANSFORM_ALIGN_DISABLED", ParticlesTransformAlign::DISABLED), crate::meta::inspect::EnumConstant::new("Z_BILLBOARD", "PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD", ParticlesTransformAlign::Z_BILLBOARD), crate::meta::inspect::EnumConstant::new("Y_TO_VELOCITY", "PARTICLES_TRANSFORM_ALIGN_Y_TO_VELOCITY", ParticlesTransformAlign::Y_TO_VELOCITY), crate::meta::inspect::EnumConstant::new("Z_BILLBOARD_Y_TO_VELOCITY", "PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY", ParticlesTransformAlign::Z_BILLBOARD_Y_TO_VELOCITY)]
        }
    }
}
impl crate::meta::GodotConvert for ParticlesTransformAlign {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ParticlesTransformAlign {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ParticlesTransformAlign {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ParticlesDrawOrder {
    ord: i32
}
impl ParticlesDrawOrder {
    #[doc(alias = "PARTICLES_DRAW_ORDER_INDEX")]
    #[doc = "Godot enumerator name: `PARTICLES_DRAW_ORDER_INDEX`"]
    pub const INDEX: ParticlesDrawOrder = ParticlesDrawOrder {
        ord: 0i32
    };
    #[doc(alias = "PARTICLES_DRAW_ORDER_LIFETIME")]
    #[doc = "Godot enumerator name: `PARTICLES_DRAW_ORDER_LIFETIME`"]
    pub const LIFETIME: ParticlesDrawOrder = ParticlesDrawOrder {
        ord: 1i32
    };
    #[doc(alias = "PARTICLES_DRAW_ORDER_REVERSE_LIFETIME")]
    #[doc = "Godot enumerator name: `PARTICLES_DRAW_ORDER_REVERSE_LIFETIME`"]
    pub const REVERSE_LIFETIME: ParticlesDrawOrder = ParticlesDrawOrder {
        ord: 2i32
    };
    #[doc(alias = "PARTICLES_DRAW_ORDER_VIEW_DEPTH")]
    #[doc = "Godot enumerator name: `PARTICLES_DRAW_ORDER_VIEW_DEPTH`"]
    pub const VIEW_DEPTH: ParticlesDrawOrder = ParticlesDrawOrder {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ParticlesDrawOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ParticlesDrawOrder") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ParticlesDrawOrder {
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
            Self::INDEX => "INDEX", Self::LIFETIME => "LIFETIME", Self::REVERSE_LIFETIME => "REVERSE_LIFETIME", Self::VIEW_DEPTH => "VIEW_DEPTH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ParticlesDrawOrder::INDEX, ParticlesDrawOrder::LIFETIME, ParticlesDrawOrder::REVERSE_LIFETIME, ParticlesDrawOrder::VIEW_DEPTH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ParticlesDrawOrder >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("INDEX", "PARTICLES_DRAW_ORDER_INDEX", ParticlesDrawOrder::INDEX), crate::meta::inspect::EnumConstant::new("LIFETIME", "PARTICLES_DRAW_ORDER_LIFETIME", ParticlesDrawOrder::LIFETIME), crate::meta::inspect::EnumConstant::new("REVERSE_LIFETIME", "PARTICLES_DRAW_ORDER_REVERSE_LIFETIME", ParticlesDrawOrder::REVERSE_LIFETIME), crate::meta::inspect::EnumConstant::new("VIEW_DEPTH", "PARTICLES_DRAW_ORDER_VIEW_DEPTH", ParticlesDrawOrder::VIEW_DEPTH)]
        }
    }
}
impl crate::meta::GodotConvert for ParticlesDrawOrder {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ParticlesDrawOrder {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ParticlesDrawOrder {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ParticlesCollisionType {
    ord: i32
}
impl ParticlesCollisionType {
    #[doc(alias = "PARTICLES_COLLISION_TYPE_SPHERE_ATTRACT")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_TYPE_SPHERE_ATTRACT`"]
    pub const SPHERE_ATTRACT: ParticlesCollisionType = ParticlesCollisionType {
        ord: 0i32
    };
    #[doc(alias = "PARTICLES_COLLISION_TYPE_BOX_ATTRACT")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_TYPE_BOX_ATTRACT`"]
    pub const BOX_ATTRACT: ParticlesCollisionType = ParticlesCollisionType {
        ord: 1i32
    };
    #[doc(alias = "PARTICLES_COLLISION_TYPE_VECTOR_FIELD_ATTRACT")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_TYPE_VECTOR_FIELD_ATTRACT`"]
    pub const VECTOR_FIELD_ATTRACT: ParticlesCollisionType = ParticlesCollisionType {
        ord: 2i32
    };
    #[doc(alias = "PARTICLES_COLLISION_TYPE_SPHERE_COLLIDE")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_TYPE_SPHERE_COLLIDE`"]
    pub const SPHERE_COLLIDE: ParticlesCollisionType = ParticlesCollisionType {
        ord: 3i32
    };
    #[doc(alias = "PARTICLES_COLLISION_TYPE_BOX_COLLIDE")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_TYPE_BOX_COLLIDE`"]
    pub const BOX_COLLIDE: ParticlesCollisionType = ParticlesCollisionType {
        ord: 4i32
    };
    #[doc(alias = "PARTICLES_COLLISION_TYPE_SDF_COLLIDE")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_TYPE_SDF_COLLIDE`"]
    pub const SDF_COLLIDE: ParticlesCollisionType = ParticlesCollisionType {
        ord: 5i32
    };
    #[doc(alias = "PARTICLES_COLLISION_TYPE_HEIGHTFIELD_COLLIDE")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_TYPE_HEIGHTFIELD_COLLIDE`"]
    pub const HEIGHTFIELD_COLLIDE: ParticlesCollisionType = ParticlesCollisionType {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for ParticlesCollisionType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ParticlesCollisionType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ParticlesCollisionType {
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
            Self::SPHERE_ATTRACT => "SPHERE_ATTRACT", Self::BOX_ATTRACT => "BOX_ATTRACT", Self::VECTOR_FIELD_ATTRACT => "VECTOR_FIELD_ATTRACT", Self::SPHERE_COLLIDE => "SPHERE_COLLIDE", Self::BOX_COLLIDE => "BOX_COLLIDE", Self::SDF_COLLIDE => "SDF_COLLIDE", Self::HEIGHTFIELD_COLLIDE => "HEIGHTFIELD_COLLIDE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ParticlesCollisionType::SPHERE_ATTRACT, ParticlesCollisionType::BOX_ATTRACT, ParticlesCollisionType::VECTOR_FIELD_ATTRACT, ParticlesCollisionType::SPHERE_COLLIDE, ParticlesCollisionType::BOX_COLLIDE, ParticlesCollisionType::SDF_COLLIDE, ParticlesCollisionType::HEIGHTFIELD_COLLIDE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ParticlesCollisionType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SPHERE_ATTRACT", "PARTICLES_COLLISION_TYPE_SPHERE_ATTRACT", ParticlesCollisionType::SPHERE_ATTRACT), crate::meta::inspect::EnumConstant::new("BOX_ATTRACT", "PARTICLES_COLLISION_TYPE_BOX_ATTRACT", ParticlesCollisionType::BOX_ATTRACT), crate::meta::inspect::EnumConstant::new("VECTOR_FIELD_ATTRACT", "PARTICLES_COLLISION_TYPE_VECTOR_FIELD_ATTRACT", ParticlesCollisionType::VECTOR_FIELD_ATTRACT), crate::meta::inspect::EnumConstant::new("SPHERE_COLLIDE", "PARTICLES_COLLISION_TYPE_SPHERE_COLLIDE", ParticlesCollisionType::SPHERE_COLLIDE), crate::meta::inspect::EnumConstant::new("BOX_COLLIDE", "PARTICLES_COLLISION_TYPE_BOX_COLLIDE", ParticlesCollisionType::BOX_COLLIDE), crate::meta::inspect::EnumConstant::new("SDF_COLLIDE", "PARTICLES_COLLISION_TYPE_SDF_COLLIDE", ParticlesCollisionType::SDF_COLLIDE), crate::meta::inspect::EnumConstant::new("HEIGHTFIELD_COLLIDE", "PARTICLES_COLLISION_TYPE_HEIGHTFIELD_COLLIDE", ParticlesCollisionType::HEIGHTFIELD_COLLIDE)]
        }
    }
}
impl crate::meta::GodotConvert for ParticlesCollisionType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ParticlesCollisionType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ParticlesCollisionType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ParticlesCollisionHeightfieldResolution {
    ord: i32
}
impl ParticlesCollisionHeightfieldResolution {
    #[doc(alias = "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_256")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_256`"]
    pub const RESOLUTION_256: ParticlesCollisionHeightfieldResolution = ParticlesCollisionHeightfieldResolution {
        ord: 0i32
    };
    #[doc(alias = "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_512")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_512`"]
    pub const RESOLUTION_512: ParticlesCollisionHeightfieldResolution = ParticlesCollisionHeightfieldResolution {
        ord: 1i32
    };
    #[doc(alias = "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_1024")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_1024`"]
    pub const RESOLUTION_1024: ParticlesCollisionHeightfieldResolution = ParticlesCollisionHeightfieldResolution {
        ord: 2i32
    };
    #[doc(alias = "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_2048")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_2048`"]
    pub const RESOLUTION_2048: ParticlesCollisionHeightfieldResolution = ParticlesCollisionHeightfieldResolution {
        ord: 3i32
    };
    #[doc(alias = "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_4096")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_4096`"]
    pub const RESOLUTION_4096: ParticlesCollisionHeightfieldResolution = ParticlesCollisionHeightfieldResolution {
        ord: 4i32
    };
    #[doc(alias = "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_8192")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_8192`"]
    pub const RESOLUTION_8192: ParticlesCollisionHeightfieldResolution = ParticlesCollisionHeightfieldResolution {
        ord: 5i32
    };
    #[doc(alias = "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_MAX")]
    #[doc = "Godot enumerator name: `PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_MAX`"]
    pub const MAX: ParticlesCollisionHeightfieldResolution = ParticlesCollisionHeightfieldResolution {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for ParticlesCollisionHeightfieldResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ParticlesCollisionHeightfieldResolution") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ParticlesCollisionHeightfieldResolution {
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
        &[ParticlesCollisionHeightfieldResolution::RESOLUTION_256, ParticlesCollisionHeightfieldResolution::RESOLUTION_512, ParticlesCollisionHeightfieldResolution::RESOLUTION_1024, ParticlesCollisionHeightfieldResolution::RESOLUTION_2048, ParticlesCollisionHeightfieldResolution::RESOLUTION_4096, ParticlesCollisionHeightfieldResolution::RESOLUTION_8192]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ParticlesCollisionHeightfieldResolution >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RESOLUTION_256", "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_256", ParticlesCollisionHeightfieldResolution::RESOLUTION_256), crate::meta::inspect::EnumConstant::new("RESOLUTION_512", "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_512", ParticlesCollisionHeightfieldResolution::RESOLUTION_512), crate::meta::inspect::EnumConstant::new("RESOLUTION_1024", "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_1024", ParticlesCollisionHeightfieldResolution::RESOLUTION_1024), crate::meta::inspect::EnumConstant::new("RESOLUTION_2048", "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_2048", ParticlesCollisionHeightfieldResolution::RESOLUTION_2048), crate::meta::inspect::EnumConstant::new("RESOLUTION_4096", "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_4096", ParticlesCollisionHeightfieldResolution::RESOLUTION_4096), crate::meta::inspect::EnumConstant::new("RESOLUTION_8192", "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_8192", ParticlesCollisionHeightfieldResolution::RESOLUTION_8192), crate::meta::inspect::EnumConstant::new("MAX", "PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_MAX", ParticlesCollisionHeightfieldResolution::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ParticlesCollisionHeightfieldResolution {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for ParticlesCollisionHeightfieldResolution {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ParticlesCollisionHeightfieldResolution {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ParticlesCollisionHeightfieldResolution {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FogVolumeShape {
    ord: i32
}
impl FogVolumeShape {
    #[doc(alias = "FOG_VOLUME_SHAPE_ELLIPSOID")]
    #[doc = "Godot enumerator name: `FOG_VOLUME_SHAPE_ELLIPSOID`"]
    pub const ELLIPSOID: FogVolumeShape = FogVolumeShape {
        ord: 0i32
    };
    #[doc(alias = "FOG_VOLUME_SHAPE_CONE")]
    #[doc = "Godot enumerator name: `FOG_VOLUME_SHAPE_CONE`"]
    pub const CONE: FogVolumeShape = FogVolumeShape {
        ord: 1i32
    };
    #[doc(alias = "FOG_VOLUME_SHAPE_CYLINDER")]
    #[doc = "Godot enumerator name: `FOG_VOLUME_SHAPE_CYLINDER`"]
    pub const CYLINDER: FogVolumeShape = FogVolumeShape {
        ord: 2i32
    };
    #[doc(alias = "FOG_VOLUME_SHAPE_BOX")]
    #[doc = "Godot enumerator name: `FOG_VOLUME_SHAPE_BOX`"]
    pub const BOX: FogVolumeShape = FogVolumeShape {
        ord: 3i32
    };
    #[doc(alias = "FOG_VOLUME_SHAPE_WORLD")]
    #[doc = "Godot enumerator name: `FOG_VOLUME_SHAPE_WORLD`"]
    pub const WORLD: FogVolumeShape = FogVolumeShape {
        ord: 4i32
    };
    #[doc(alias = "FOG_VOLUME_SHAPE_MAX")]
    #[doc = "Godot enumerator name: `FOG_VOLUME_SHAPE_MAX`"]
    pub const MAX: FogVolumeShape = FogVolumeShape {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for FogVolumeShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FogVolumeShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FogVolumeShape {
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
            Self::ELLIPSOID => "ELLIPSOID", Self::CONE => "CONE", Self::CYLINDER => "CYLINDER", Self::BOX => "BOX", Self::WORLD => "WORLD", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FogVolumeShape::ELLIPSOID, FogVolumeShape::CONE, FogVolumeShape::CYLINDER, FogVolumeShape::BOX, FogVolumeShape::WORLD]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FogVolumeShape >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ELLIPSOID", "FOG_VOLUME_SHAPE_ELLIPSOID", FogVolumeShape::ELLIPSOID), crate::meta::inspect::EnumConstant::new("CONE", "FOG_VOLUME_SHAPE_CONE", FogVolumeShape::CONE), crate::meta::inspect::EnumConstant::new("CYLINDER", "FOG_VOLUME_SHAPE_CYLINDER", FogVolumeShape::CYLINDER), crate::meta::inspect::EnumConstant::new("BOX", "FOG_VOLUME_SHAPE_BOX", FogVolumeShape::BOX), crate::meta::inspect::EnumConstant::new("WORLD", "FOG_VOLUME_SHAPE_WORLD", FogVolumeShape::WORLD), crate::meta::inspect::EnumConstant::new("MAX", "FOG_VOLUME_SHAPE_MAX", FogVolumeShape::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for FogVolumeShape {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for FogVolumeShape {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FogVolumeShape {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FogVolumeShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportScaling3DMode {
    ord: i32
}
impl ViewportScaling3DMode {
    #[doc(alias = "VIEWPORT_SCALING_3D_MODE_BILINEAR")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCALING_3D_MODE_BILINEAR`"]
    pub const BILINEAR: ViewportScaling3DMode = ViewportScaling3DMode {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_SCALING_3D_MODE_FSR")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCALING_3D_MODE_FSR`"]
    pub const FSR: ViewportScaling3DMode = ViewportScaling3DMode {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_SCALING_3D_MODE_FSR2")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCALING_3D_MODE_FSR2`"]
    pub const FSR2: ViewportScaling3DMode = ViewportScaling3DMode {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_SCALING_3D_MODE_METALFX_SPATIAL")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCALING_3D_MODE_METALFX_SPATIAL`"]
    pub const METALFX_SPATIAL: ViewportScaling3DMode = ViewportScaling3DMode {
        ord: 3i32
    };
    #[doc(alias = "VIEWPORT_SCALING_3D_MODE_METALFX_TEMPORAL")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCALING_3D_MODE_METALFX_TEMPORAL`"]
    pub const METALFX_TEMPORAL: ViewportScaling3DMode = ViewportScaling3DMode {
        ord: 4i32
    };
    #[doc(alias = "VIEWPORT_SCALING_3D_MODE_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCALING_3D_MODE_MAX`"]
    pub const MAX: ViewportScaling3DMode = ViewportScaling3DMode {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for ViewportScaling3DMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportScaling3DMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportScaling3DMode {
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
            Self::BILINEAR => "BILINEAR", Self::FSR => "FSR", Self::FSR2 => "FSR2", Self::METALFX_SPATIAL => "METALFX_SPATIAL", Self::METALFX_TEMPORAL => "METALFX_TEMPORAL", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportScaling3DMode::BILINEAR, ViewportScaling3DMode::FSR, ViewportScaling3DMode::FSR2, ViewportScaling3DMode::METALFX_SPATIAL, ViewportScaling3DMode::METALFX_TEMPORAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportScaling3DMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BILINEAR", "VIEWPORT_SCALING_3D_MODE_BILINEAR", ViewportScaling3DMode::BILINEAR), crate::meta::inspect::EnumConstant::new("FSR", "VIEWPORT_SCALING_3D_MODE_FSR", ViewportScaling3DMode::FSR), crate::meta::inspect::EnumConstant::new("FSR2", "VIEWPORT_SCALING_3D_MODE_FSR2", ViewportScaling3DMode::FSR2), crate::meta::inspect::EnumConstant::new("METALFX_SPATIAL", "VIEWPORT_SCALING_3D_MODE_METALFX_SPATIAL", ViewportScaling3DMode::METALFX_SPATIAL), crate::meta::inspect::EnumConstant::new("METALFX_TEMPORAL", "VIEWPORT_SCALING_3D_MODE_METALFX_TEMPORAL", ViewportScaling3DMode::METALFX_TEMPORAL), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_SCALING_3D_MODE_MAX", ViewportScaling3DMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportScaling3DMode {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for ViewportScaling3DMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ViewportScaling3DMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportScaling3DMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportUpdateMode {
    ord: i32
}
impl ViewportUpdateMode {
    #[doc(alias = "VIEWPORT_UPDATE_DISABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_UPDATE_DISABLED`"]
    pub const DISABLED: ViewportUpdateMode = ViewportUpdateMode {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_UPDATE_ONCE")]
    #[doc = "Godot enumerator name: `VIEWPORT_UPDATE_ONCE`"]
    pub const ONCE: ViewportUpdateMode = ViewportUpdateMode {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_UPDATE_WHEN_VISIBLE")]
    #[doc = "Godot enumerator name: `VIEWPORT_UPDATE_WHEN_VISIBLE`"]
    pub const WHEN_VISIBLE: ViewportUpdateMode = ViewportUpdateMode {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_UPDATE_WHEN_PARENT_VISIBLE")]
    #[doc = "Godot enumerator name: `VIEWPORT_UPDATE_WHEN_PARENT_VISIBLE`"]
    pub const WHEN_PARENT_VISIBLE: ViewportUpdateMode = ViewportUpdateMode {
        ord: 3i32
    };
    #[doc(alias = "VIEWPORT_UPDATE_ALWAYS")]
    #[doc = "Godot enumerator name: `VIEWPORT_UPDATE_ALWAYS`"]
    pub const ALWAYS: ViewportUpdateMode = ViewportUpdateMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ViewportUpdateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportUpdateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportUpdateMode {
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
            Self::DISABLED => "DISABLED", Self::ONCE => "ONCE", Self::WHEN_VISIBLE => "WHEN_VISIBLE", Self::WHEN_PARENT_VISIBLE => "WHEN_PARENT_VISIBLE", Self::ALWAYS => "ALWAYS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportUpdateMode::DISABLED, ViewportUpdateMode::ONCE, ViewportUpdateMode::WHEN_VISIBLE, ViewportUpdateMode::WHEN_PARENT_VISIBLE, ViewportUpdateMode::ALWAYS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportUpdateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VIEWPORT_UPDATE_DISABLED", ViewportUpdateMode::DISABLED), crate::meta::inspect::EnumConstant::new("ONCE", "VIEWPORT_UPDATE_ONCE", ViewportUpdateMode::ONCE), crate::meta::inspect::EnumConstant::new("WHEN_VISIBLE", "VIEWPORT_UPDATE_WHEN_VISIBLE", ViewportUpdateMode::WHEN_VISIBLE), crate::meta::inspect::EnumConstant::new("WHEN_PARENT_VISIBLE", "VIEWPORT_UPDATE_WHEN_PARENT_VISIBLE", ViewportUpdateMode::WHEN_PARENT_VISIBLE), crate::meta::inspect::EnumConstant::new("ALWAYS", "VIEWPORT_UPDATE_ALWAYS", ViewportUpdateMode::ALWAYS)]
        }
    }
}
impl crate::meta::GodotConvert for ViewportUpdateMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ViewportUpdateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportUpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportClearMode {
    ord: i32
}
impl ViewportClearMode {
    #[doc(alias = "VIEWPORT_CLEAR_ALWAYS")]
    #[doc = "Godot enumerator name: `VIEWPORT_CLEAR_ALWAYS`"]
    pub const ALWAYS: ViewportClearMode = ViewportClearMode {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_CLEAR_NEVER")]
    #[doc = "Godot enumerator name: `VIEWPORT_CLEAR_NEVER`"]
    pub const NEVER: ViewportClearMode = ViewportClearMode {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_CLEAR_ONLY_NEXT_FRAME")]
    #[doc = "Godot enumerator name: `VIEWPORT_CLEAR_ONLY_NEXT_FRAME`"]
    pub const ONLY_NEXT_FRAME: ViewportClearMode = ViewportClearMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ViewportClearMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportClearMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportClearMode {
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
            Self::ALWAYS => "ALWAYS", Self::NEVER => "NEVER", Self::ONLY_NEXT_FRAME => "ONLY_NEXT_FRAME", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportClearMode::ALWAYS, ViewportClearMode::NEVER, ViewportClearMode::ONLY_NEXT_FRAME]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportClearMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ALWAYS", "VIEWPORT_CLEAR_ALWAYS", ViewportClearMode::ALWAYS), crate::meta::inspect::EnumConstant::new("NEVER", "VIEWPORT_CLEAR_NEVER", ViewportClearMode::NEVER), crate::meta::inspect::EnumConstant::new("ONLY_NEXT_FRAME", "VIEWPORT_CLEAR_ONLY_NEXT_FRAME", ViewportClearMode::ONLY_NEXT_FRAME)]
        }
    }
}
impl crate::meta::GodotConvert for ViewportClearMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ViewportClearMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportClearMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportEnvironmentMode {
    ord: i32
}
impl ViewportEnvironmentMode {
    #[doc(alias = "VIEWPORT_ENVIRONMENT_DISABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_ENVIRONMENT_DISABLED`"]
    pub const DISABLED: ViewportEnvironmentMode = ViewportEnvironmentMode {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_ENVIRONMENT_ENABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_ENVIRONMENT_ENABLED`"]
    pub const ENABLED: ViewportEnvironmentMode = ViewportEnvironmentMode {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_ENVIRONMENT_INHERIT")]
    #[doc = "Godot enumerator name: `VIEWPORT_ENVIRONMENT_INHERIT`"]
    pub const INHERIT: ViewportEnvironmentMode = ViewportEnvironmentMode {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_ENVIRONMENT_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_ENVIRONMENT_MAX`"]
    pub const MAX: ViewportEnvironmentMode = ViewportEnvironmentMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ViewportEnvironmentMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportEnvironmentMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportEnvironmentMode {
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
            Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::INHERIT => "INHERIT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportEnvironmentMode::DISABLED, ViewportEnvironmentMode::ENABLED, ViewportEnvironmentMode::INHERIT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportEnvironmentMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VIEWPORT_ENVIRONMENT_DISABLED", ViewportEnvironmentMode::DISABLED), crate::meta::inspect::EnumConstant::new("ENABLED", "VIEWPORT_ENVIRONMENT_ENABLED", ViewportEnvironmentMode::ENABLED), crate::meta::inspect::EnumConstant::new("INHERIT", "VIEWPORT_ENVIRONMENT_INHERIT", ViewportEnvironmentMode::INHERIT), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_ENVIRONMENT_MAX", ViewportEnvironmentMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportEnvironmentMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ViewportEnvironmentMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ViewportEnvironmentMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportEnvironmentMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `ViewportSDFOversize`."]
pub struct ViewportSdfOversize {
    ord: i32
}
impl ViewportSdfOversize {
    #[doc(alias = "VIEWPORT_SDF_OVERSIZE_100_PERCENT")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_OVERSIZE_100_PERCENT`"]
    pub const OVERSIZE_100_PERCENT: ViewportSdfOversize = ViewportSdfOversize {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_SDF_OVERSIZE_120_PERCENT")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_OVERSIZE_120_PERCENT`"]
    pub const OVERSIZE_120_PERCENT: ViewportSdfOversize = ViewportSdfOversize {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_SDF_OVERSIZE_150_PERCENT")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_OVERSIZE_150_PERCENT`"]
    pub const OVERSIZE_150_PERCENT: ViewportSdfOversize = ViewportSdfOversize {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_SDF_OVERSIZE_200_PERCENT")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_OVERSIZE_200_PERCENT`"]
    pub const OVERSIZE_200_PERCENT: ViewportSdfOversize = ViewportSdfOversize {
        ord: 3i32
    };
    #[doc(alias = "VIEWPORT_SDF_OVERSIZE_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_OVERSIZE_MAX`"]
    pub const MAX: ViewportSdfOversize = ViewportSdfOversize {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ViewportSdfOversize {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportSdfOversize") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportSdfOversize {
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
            Self::OVERSIZE_100_PERCENT => "OVERSIZE_100_PERCENT", Self::OVERSIZE_120_PERCENT => "OVERSIZE_120_PERCENT", Self::OVERSIZE_150_PERCENT => "OVERSIZE_150_PERCENT", Self::OVERSIZE_200_PERCENT => "OVERSIZE_200_PERCENT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportSdfOversize::OVERSIZE_100_PERCENT, ViewportSdfOversize::OVERSIZE_120_PERCENT, ViewportSdfOversize::OVERSIZE_150_PERCENT, ViewportSdfOversize::OVERSIZE_200_PERCENT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportSdfOversize >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OVERSIZE_100_PERCENT", "VIEWPORT_SDF_OVERSIZE_100_PERCENT", ViewportSdfOversize::OVERSIZE_100_PERCENT), crate::meta::inspect::EnumConstant::new("OVERSIZE_120_PERCENT", "VIEWPORT_SDF_OVERSIZE_120_PERCENT", ViewportSdfOversize::OVERSIZE_120_PERCENT), crate::meta::inspect::EnumConstant::new("OVERSIZE_150_PERCENT", "VIEWPORT_SDF_OVERSIZE_150_PERCENT", ViewportSdfOversize::OVERSIZE_150_PERCENT), crate::meta::inspect::EnumConstant::new("OVERSIZE_200_PERCENT", "VIEWPORT_SDF_OVERSIZE_200_PERCENT", ViewportSdfOversize::OVERSIZE_200_PERCENT), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_SDF_OVERSIZE_MAX", ViewportSdfOversize::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportSdfOversize {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for ViewportSdfOversize {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ViewportSdfOversize {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportSdfOversize {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `ViewportSDFScale`."]
pub struct ViewportSdfScale {
    ord: i32
}
impl ViewportSdfScale {
    #[doc(alias = "VIEWPORT_SDF_SCALE_100_PERCENT")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_SCALE_100_PERCENT`"]
    pub const SCALE_100_PERCENT: ViewportSdfScale = ViewportSdfScale {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_SDF_SCALE_50_PERCENT")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_SCALE_50_PERCENT`"]
    pub const SCALE_50_PERCENT: ViewportSdfScale = ViewportSdfScale {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_SDF_SCALE_25_PERCENT")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_SCALE_25_PERCENT`"]
    pub const SCALE_25_PERCENT: ViewportSdfScale = ViewportSdfScale {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_SDF_SCALE_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_SDF_SCALE_MAX`"]
    pub const MAX: ViewportSdfScale = ViewportSdfScale {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ViewportSdfScale {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportSdfScale") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportSdfScale {
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
            Self::SCALE_100_PERCENT => "SCALE_100_PERCENT", Self::SCALE_50_PERCENT => "SCALE_50_PERCENT", Self::SCALE_25_PERCENT => "SCALE_25_PERCENT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportSdfScale::SCALE_100_PERCENT, ViewportSdfScale::SCALE_50_PERCENT, ViewportSdfScale::SCALE_25_PERCENT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportSdfScale >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SCALE_100_PERCENT", "VIEWPORT_SDF_SCALE_100_PERCENT", ViewportSdfScale::SCALE_100_PERCENT), crate::meta::inspect::EnumConstant::new("SCALE_50_PERCENT", "VIEWPORT_SDF_SCALE_50_PERCENT", ViewportSdfScale::SCALE_50_PERCENT), crate::meta::inspect::EnumConstant::new("SCALE_25_PERCENT", "VIEWPORT_SDF_SCALE_25_PERCENT", ViewportSdfScale::SCALE_25_PERCENT), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_SDF_SCALE_MAX", ViewportSdfScale::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportSdfScale {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ViewportSdfScale {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ViewportSdfScale {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportSdfScale {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `ViewportMSAA`."]
pub struct ViewportMsaa {
    ord: i32
}
impl ViewportMsaa {
    #[doc(alias = "VIEWPORT_MSAA_DISABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_MSAA_DISABLED`"]
    pub const DISABLED: ViewportMsaa = ViewportMsaa {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_MSAA_2X")]
    #[doc = "Godot enumerator name: `VIEWPORT_MSAA_2X`"]
    pub const MSAA_2X: ViewportMsaa = ViewportMsaa {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_MSAA_4X")]
    #[doc = "Godot enumerator name: `VIEWPORT_MSAA_4X`"]
    pub const MSAA_4X: ViewportMsaa = ViewportMsaa {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_MSAA_8X")]
    #[doc = "Godot enumerator name: `VIEWPORT_MSAA_8X`"]
    pub const MSAA_8X: ViewportMsaa = ViewportMsaa {
        ord: 3i32
    };
    #[doc(alias = "VIEWPORT_MSAA_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_MSAA_MAX`"]
    pub const MAX: ViewportMsaa = ViewportMsaa {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for ViewportMsaa {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportMsaa") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportMsaa {
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
            Self::DISABLED => "DISABLED", Self::MSAA_2X => "MSAA_2X", Self::MSAA_4X => "MSAA_4X", Self::MSAA_8X => "MSAA_8X", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportMsaa::DISABLED, ViewportMsaa::MSAA_2X, ViewportMsaa::MSAA_4X, ViewportMsaa::MSAA_8X]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportMsaa >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VIEWPORT_MSAA_DISABLED", ViewportMsaa::DISABLED), crate::meta::inspect::EnumConstant::new("MSAA_2X", "VIEWPORT_MSAA_2X", ViewportMsaa::MSAA_2X), crate::meta::inspect::EnumConstant::new("MSAA_4X", "VIEWPORT_MSAA_4X", ViewportMsaa::MSAA_4X), crate::meta::inspect::EnumConstant::new("MSAA_8X", "VIEWPORT_MSAA_8X", ViewportMsaa::MSAA_8X), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_MSAA_MAX", ViewportMsaa::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportMsaa {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for ViewportMsaa {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ViewportMsaa {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportMsaa {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportAnisotropicFiltering {
    ord: i32
}
impl ViewportAnisotropicFiltering {
    #[doc(alias = "VIEWPORT_ANISOTROPY_DISABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_ANISOTROPY_DISABLED`"]
    pub const DISABLED: ViewportAnisotropicFiltering = ViewportAnisotropicFiltering {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_ANISOTROPY_2X")]
    #[doc = "Godot enumerator name: `VIEWPORT_ANISOTROPY_2X`"]
    pub const ANISOTROPY_2X: ViewportAnisotropicFiltering = ViewportAnisotropicFiltering {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_ANISOTROPY_4X")]
    #[doc = "Godot enumerator name: `VIEWPORT_ANISOTROPY_4X`"]
    pub const ANISOTROPY_4X: ViewportAnisotropicFiltering = ViewportAnisotropicFiltering {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_ANISOTROPY_8X")]
    #[doc = "Godot enumerator name: `VIEWPORT_ANISOTROPY_8X`"]
    pub const ANISOTROPY_8X: ViewportAnisotropicFiltering = ViewportAnisotropicFiltering {
        ord: 3i32
    };
    #[doc(alias = "VIEWPORT_ANISOTROPY_16X")]
    #[doc = "Godot enumerator name: `VIEWPORT_ANISOTROPY_16X`"]
    pub const ANISOTROPY_16X: ViewportAnisotropicFiltering = ViewportAnisotropicFiltering {
        ord: 4i32
    };
    #[doc(alias = "VIEWPORT_ANISOTROPY_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_ANISOTROPY_MAX`"]
    pub const MAX: ViewportAnisotropicFiltering = ViewportAnisotropicFiltering {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for ViewportAnisotropicFiltering {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportAnisotropicFiltering") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportAnisotropicFiltering {
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
            Self::DISABLED => "DISABLED", Self::ANISOTROPY_2X => "ANISOTROPY_2X", Self::ANISOTROPY_4X => "ANISOTROPY_4X", Self::ANISOTROPY_8X => "ANISOTROPY_8X", Self::ANISOTROPY_16X => "ANISOTROPY_16X", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportAnisotropicFiltering::DISABLED, ViewportAnisotropicFiltering::ANISOTROPY_2X, ViewportAnisotropicFiltering::ANISOTROPY_4X, ViewportAnisotropicFiltering::ANISOTROPY_8X, ViewportAnisotropicFiltering::ANISOTROPY_16X]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportAnisotropicFiltering >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VIEWPORT_ANISOTROPY_DISABLED", ViewportAnisotropicFiltering::DISABLED), crate::meta::inspect::EnumConstant::new("ANISOTROPY_2X", "VIEWPORT_ANISOTROPY_2X", ViewportAnisotropicFiltering::ANISOTROPY_2X), crate::meta::inspect::EnumConstant::new("ANISOTROPY_4X", "VIEWPORT_ANISOTROPY_4X", ViewportAnisotropicFiltering::ANISOTROPY_4X), crate::meta::inspect::EnumConstant::new("ANISOTROPY_8X", "VIEWPORT_ANISOTROPY_8X", ViewportAnisotropicFiltering::ANISOTROPY_8X), crate::meta::inspect::EnumConstant::new("ANISOTROPY_16X", "VIEWPORT_ANISOTROPY_16X", ViewportAnisotropicFiltering::ANISOTROPY_16X), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_ANISOTROPY_MAX", ViewportAnisotropicFiltering::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportAnisotropicFiltering {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for ViewportAnisotropicFiltering {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ViewportAnisotropicFiltering {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportAnisotropicFiltering {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `ViewportScreenSpaceAA`."]
pub struct ViewportScreenSpaceAa {
    ord: i32
}
impl ViewportScreenSpaceAa {
    #[doc(alias = "VIEWPORT_SCREEN_SPACE_AA_DISABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCREEN_SPACE_AA_DISABLED`"]
    pub const DISABLED: ViewportScreenSpaceAa = ViewportScreenSpaceAa {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_SCREEN_SPACE_AA_FXAA")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCREEN_SPACE_AA_FXAA`"]
    pub const FXAA: ViewportScreenSpaceAa = ViewportScreenSpaceAa {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_SCREEN_SPACE_AA_SMAA")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCREEN_SPACE_AA_SMAA`"]
    pub const SMAA: ViewportScreenSpaceAa = ViewportScreenSpaceAa {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_SCREEN_SPACE_AA_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_SCREEN_SPACE_AA_MAX`"]
    pub const MAX: ViewportScreenSpaceAa = ViewportScreenSpaceAa {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ViewportScreenSpaceAa {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportScreenSpaceAa") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportScreenSpaceAa {
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
            Self::DISABLED => "DISABLED", Self::FXAA => "FXAA", Self::SMAA => "SMAA", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportScreenSpaceAa::DISABLED, ViewportScreenSpaceAa::FXAA, ViewportScreenSpaceAa::SMAA]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportScreenSpaceAa >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VIEWPORT_SCREEN_SPACE_AA_DISABLED", ViewportScreenSpaceAa::DISABLED), crate::meta::inspect::EnumConstant::new("FXAA", "VIEWPORT_SCREEN_SPACE_AA_FXAA", ViewportScreenSpaceAa::FXAA), crate::meta::inspect::EnumConstant::new("SMAA", "VIEWPORT_SCREEN_SPACE_AA_SMAA", ViewportScreenSpaceAa::SMAA), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_SCREEN_SPACE_AA_MAX", ViewportScreenSpaceAa::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportScreenSpaceAa {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ViewportScreenSpaceAa {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ViewportScreenSpaceAa {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportScreenSpaceAa {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportOcclusionCullingBuildQuality {
    ord: i32
}
impl ViewportOcclusionCullingBuildQuality {
    #[doc(alias = "VIEWPORT_OCCLUSION_BUILD_QUALITY_LOW")]
    #[doc = "Godot enumerator name: `VIEWPORT_OCCLUSION_BUILD_QUALITY_LOW`"]
    pub const LOW: ViewportOcclusionCullingBuildQuality = ViewportOcclusionCullingBuildQuality {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_OCCLUSION_BUILD_QUALITY_MEDIUM")]
    #[doc = "Godot enumerator name: `VIEWPORT_OCCLUSION_BUILD_QUALITY_MEDIUM`"]
    pub const MEDIUM: ViewportOcclusionCullingBuildQuality = ViewportOcclusionCullingBuildQuality {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_OCCLUSION_BUILD_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `VIEWPORT_OCCLUSION_BUILD_QUALITY_HIGH`"]
    pub const HIGH: ViewportOcclusionCullingBuildQuality = ViewportOcclusionCullingBuildQuality {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ViewportOcclusionCullingBuildQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportOcclusionCullingBuildQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportOcclusionCullingBuildQuality {
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
            Self::LOW => "LOW", Self::MEDIUM => "MEDIUM", Self::HIGH => "HIGH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportOcclusionCullingBuildQuality::LOW, ViewportOcclusionCullingBuildQuality::MEDIUM, ViewportOcclusionCullingBuildQuality::HIGH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportOcclusionCullingBuildQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LOW", "VIEWPORT_OCCLUSION_BUILD_QUALITY_LOW", ViewportOcclusionCullingBuildQuality::LOW), crate::meta::inspect::EnumConstant::new("MEDIUM", "VIEWPORT_OCCLUSION_BUILD_QUALITY_MEDIUM", ViewportOcclusionCullingBuildQuality::MEDIUM), crate::meta::inspect::EnumConstant::new("HIGH", "VIEWPORT_OCCLUSION_BUILD_QUALITY_HIGH", ViewportOcclusionCullingBuildQuality::HIGH)]
        }
    }
}
impl crate::meta::GodotConvert for ViewportOcclusionCullingBuildQuality {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ViewportOcclusionCullingBuildQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportOcclusionCullingBuildQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportRenderInfo {
    ord: i32
}
impl ViewportRenderInfo {
    #[doc(alias = "VIEWPORT_RENDER_INFO_OBJECTS_IN_FRAME")]
    #[doc = "Godot enumerator name: `VIEWPORT_RENDER_INFO_OBJECTS_IN_FRAME`"]
    pub const OBJECTS_IN_FRAME: ViewportRenderInfo = ViewportRenderInfo {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_RENDER_INFO_PRIMITIVES_IN_FRAME")]
    #[doc = "Godot enumerator name: `VIEWPORT_RENDER_INFO_PRIMITIVES_IN_FRAME`"]
    pub const PRIMITIVES_IN_FRAME: ViewportRenderInfo = ViewportRenderInfo {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_RENDER_INFO_DRAW_CALLS_IN_FRAME")]
    #[doc = "Godot enumerator name: `VIEWPORT_RENDER_INFO_DRAW_CALLS_IN_FRAME`"]
    pub const DRAW_CALLS_IN_FRAME: ViewportRenderInfo = ViewportRenderInfo {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_RENDER_INFO_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_RENDER_INFO_MAX`"]
    pub const MAX: ViewportRenderInfo = ViewportRenderInfo {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ViewportRenderInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportRenderInfo") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportRenderInfo {
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
            Self::OBJECTS_IN_FRAME => "OBJECTS_IN_FRAME", Self::PRIMITIVES_IN_FRAME => "PRIMITIVES_IN_FRAME", Self::DRAW_CALLS_IN_FRAME => "DRAW_CALLS_IN_FRAME", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportRenderInfo::OBJECTS_IN_FRAME, ViewportRenderInfo::PRIMITIVES_IN_FRAME, ViewportRenderInfo::DRAW_CALLS_IN_FRAME]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportRenderInfo >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OBJECTS_IN_FRAME", "VIEWPORT_RENDER_INFO_OBJECTS_IN_FRAME", ViewportRenderInfo::OBJECTS_IN_FRAME), crate::meta::inspect::EnumConstant::new("PRIMITIVES_IN_FRAME", "VIEWPORT_RENDER_INFO_PRIMITIVES_IN_FRAME", ViewportRenderInfo::PRIMITIVES_IN_FRAME), crate::meta::inspect::EnumConstant::new("DRAW_CALLS_IN_FRAME", "VIEWPORT_RENDER_INFO_DRAW_CALLS_IN_FRAME", ViewportRenderInfo::DRAW_CALLS_IN_FRAME), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_RENDER_INFO_MAX", ViewportRenderInfo::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportRenderInfo {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ViewportRenderInfo {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ViewportRenderInfo {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportRenderInfo {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportRenderInfoType {
    ord: i32
}
impl ViewportRenderInfoType {
    #[doc(alias = "VIEWPORT_RENDER_INFO_TYPE_VISIBLE")]
    #[doc = "Godot enumerator name: `VIEWPORT_RENDER_INFO_TYPE_VISIBLE`"]
    pub const VISIBLE: ViewportRenderInfoType = ViewportRenderInfoType {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_RENDER_INFO_TYPE_SHADOW")]
    #[doc = "Godot enumerator name: `VIEWPORT_RENDER_INFO_TYPE_SHADOW`"]
    pub const SHADOW: ViewportRenderInfoType = ViewportRenderInfoType {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_RENDER_INFO_TYPE_CANVAS")]
    #[doc = "Godot enumerator name: `VIEWPORT_RENDER_INFO_TYPE_CANVAS`"]
    pub const CANVAS: ViewportRenderInfoType = ViewportRenderInfoType {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_RENDER_INFO_TYPE_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_RENDER_INFO_TYPE_MAX`"]
    pub const MAX: ViewportRenderInfoType = ViewportRenderInfoType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ViewportRenderInfoType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportRenderInfoType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportRenderInfoType {
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
            Self::VISIBLE => "VISIBLE", Self::SHADOW => "SHADOW", Self::CANVAS => "CANVAS", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportRenderInfoType::VISIBLE, ViewportRenderInfoType::SHADOW, ViewportRenderInfoType::CANVAS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportRenderInfoType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VISIBLE", "VIEWPORT_RENDER_INFO_TYPE_VISIBLE", ViewportRenderInfoType::VISIBLE), crate::meta::inspect::EnumConstant::new("SHADOW", "VIEWPORT_RENDER_INFO_TYPE_SHADOW", ViewportRenderInfoType::SHADOW), crate::meta::inspect::EnumConstant::new("CANVAS", "VIEWPORT_RENDER_INFO_TYPE_CANVAS", ViewportRenderInfoType::CANVAS), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_RENDER_INFO_TYPE_MAX", ViewportRenderInfoType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportRenderInfoType {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ViewportRenderInfoType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ViewportRenderInfoType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportRenderInfoType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ViewportDebugDraw {
    ord: i32
}
impl ViewportDebugDraw {
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_DISABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_DISABLED`"]
    pub const DISABLED: ViewportDebugDraw = ViewportDebugDraw {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_UNSHADED")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_UNSHADED`"]
    pub const UNSHADED: ViewportDebugDraw = ViewportDebugDraw {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_LIGHTING")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_LIGHTING`"]
    pub const LIGHTING: ViewportDebugDraw = ViewportDebugDraw {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_OVERDRAW")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_OVERDRAW`"]
    pub const OVERDRAW: ViewportDebugDraw = ViewportDebugDraw {
        ord: 3i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_WIREFRAME")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_WIREFRAME`"]
    pub const WIREFRAME: ViewportDebugDraw = ViewportDebugDraw {
        ord: 4i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_NORMAL_BUFFER")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_NORMAL_BUFFER`"]
    pub const NORMAL_BUFFER: ViewportDebugDraw = ViewportDebugDraw {
        ord: 5i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_VOXEL_GI_ALBEDO")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_VOXEL_GI_ALBEDO`"]
    pub const VOXEL_GI_ALBEDO: ViewportDebugDraw = ViewportDebugDraw {
        ord: 6i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_VOXEL_GI_LIGHTING")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_VOXEL_GI_LIGHTING`"]
    pub const VOXEL_GI_LIGHTING: ViewportDebugDraw = ViewportDebugDraw {
        ord: 7i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_VOXEL_GI_EMISSION")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_VOXEL_GI_EMISSION`"]
    pub const VOXEL_GI_EMISSION: ViewportDebugDraw = ViewportDebugDraw {
        ord: 8i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_SHADOW_ATLAS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_SHADOW_ATLAS`"]
    pub const SHADOW_ATLAS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 9i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS`"]
    pub const DIRECTIONAL_SHADOW_ATLAS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 10i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_SCENE_LUMINANCE")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_SCENE_LUMINANCE`"]
    pub const SCENE_LUMINANCE: ViewportDebugDraw = ViewportDebugDraw {
        ord: 11i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_SSAO")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_SSAO`"]
    pub const SSAO: ViewportDebugDraw = ViewportDebugDraw {
        ord: 12i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_SSIL")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_SSIL`"]
    pub const SSIL: ViewportDebugDraw = ViewportDebugDraw {
        ord: 13i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_PSSM_SPLITS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_PSSM_SPLITS`"]
    pub const PSSM_SPLITS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 14i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_DECAL_ATLAS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_DECAL_ATLAS`"]
    pub const DECAL_ATLAS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 15i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_SDFGI")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_SDFGI`"]
    pub const SDFGI: ViewportDebugDraw = ViewportDebugDraw {
        ord: 16i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_SDFGI_PROBES")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_SDFGI_PROBES`"]
    pub const SDFGI_PROBES: ViewportDebugDraw = ViewportDebugDraw {
        ord: 17i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_GI_BUFFER")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_GI_BUFFER`"]
    pub const GI_BUFFER: ViewportDebugDraw = ViewportDebugDraw {
        ord: 18i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_DISABLE_LOD")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_DISABLE_LOD`"]
    pub const DISABLE_LOD: ViewportDebugDraw = ViewportDebugDraw {
        ord: 19i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_CLUSTER_OMNI_LIGHTS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_CLUSTER_OMNI_LIGHTS`"]
    pub const CLUSTER_OMNI_LIGHTS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 20i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_CLUSTER_SPOT_LIGHTS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_CLUSTER_SPOT_LIGHTS`"]
    pub const CLUSTER_SPOT_LIGHTS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 21i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_CLUSTER_DECALS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_CLUSTER_DECALS`"]
    pub const CLUSTER_DECALS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 22i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_CLUSTER_REFLECTION_PROBES")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_CLUSTER_REFLECTION_PROBES`"]
    pub const CLUSTER_REFLECTION_PROBES: ViewportDebugDraw = ViewportDebugDraw {
        ord: 23i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_OCCLUDERS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_OCCLUDERS`"]
    pub const OCCLUDERS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 24i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_MOTION_VECTORS")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_MOTION_VECTORS`"]
    pub const MOTION_VECTORS: ViewportDebugDraw = ViewportDebugDraw {
        ord: 25i32
    };
    #[doc(alias = "VIEWPORT_DEBUG_DRAW_INTERNAL_BUFFER")]
    #[doc = "Godot enumerator name: `VIEWPORT_DEBUG_DRAW_INTERNAL_BUFFER`"]
    pub const INTERNAL_BUFFER: ViewportDebugDraw = ViewportDebugDraw {
        ord: 26i32
    };
    
}
impl std::fmt::Debug for ViewportDebugDraw {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportDebugDraw") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportDebugDraw {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::UNSHADED => "UNSHADED", Self::LIGHTING => "LIGHTING", Self::OVERDRAW => "OVERDRAW", Self::WIREFRAME => "WIREFRAME", Self::NORMAL_BUFFER => "NORMAL_BUFFER", Self::VOXEL_GI_ALBEDO => "VOXEL_GI_ALBEDO", Self::VOXEL_GI_LIGHTING => "VOXEL_GI_LIGHTING", Self::VOXEL_GI_EMISSION => "VOXEL_GI_EMISSION", Self::SHADOW_ATLAS => "SHADOW_ATLAS", Self::DIRECTIONAL_SHADOW_ATLAS => "DIRECTIONAL_SHADOW_ATLAS", Self::SCENE_LUMINANCE => "SCENE_LUMINANCE", Self::SSAO => "SSAO", Self::SSIL => "SSIL", Self::PSSM_SPLITS => "PSSM_SPLITS", Self::DECAL_ATLAS => "DECAL_ATLAS", Self::SDFGI => "SDFGI", Self::SDFGI_PROBES => "SDFGI_PROBES", Self::GI_BUFFER => "GI_BUFFER", Self::DISABLE_LOD => "DISABLE_LOD", Self::CLUSTER_OMNI_LIGHTS => "CLUSTER_OMNI_LIGHTS", Self::CLUSTER_SPOT_LIGHTS => "CLUSTER_SPOT_LIGHTS", Self::CLUSTER_DECALS => "CLUSTER_DECALS", Self::CLUSTER_REFLECTION_PROBES => "CLUSTER_REFLECTION_PROBES", Self::OCCLUDERS => "OCCLUDERS", Self::MOTION_VECTORS => "MOTION_VECTORS", Self::INTERNAL_BUFFER => "INTERNAL_BUFFER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportDebugDraw::DISABLED, ViewportDebugDraw::UNSHADED, ViewportDebugDraw::LIGHTING, ViewportDebugDraw::OVERDRAW, ViewportDebugDraw::WIREFRAME, ViewportDebugDraw::NORMAL_BUFFER, ViewportDebugDraw::VOXEL_GI_ALBEDO, ViewportDebugDraw::VOXEL_GI_LIGHTING, ViewportDebugDraw::VOXEL_GI_EMISSION, ViewportDebugDraw::SHADOW_ATLAS, ViewportDebugDraw::DIRECTIONAL_SHADOW_ATLAS, ViewportDebugDraw::SCENE_LUMINANCE, ViewportDebugDraw::SSAO, ViewportDebugDraw::SSIL, ViewportDebugDraw::PSSM_SPLITS, ViewportDebugDraw::DECAL_ATLAS, ViewportDebugDraw::SDFGI, ViewportDebugDraw::SDFGI_PROBES, ViewportDebugDraw::GI_BUFFER, ViewportDebugDraw::DISABLE_LOD, ViewportDebugDraw::CLUSTER_OMNI_LIGHTS, ViewportDebugDraw::CLUSTER_SPOT_LIGHTS, ViewportDebugDraw::CLUSTER_DECALS, ViewportDebugDraw::CLUSTER_REFLECTION_PROBES, ViewportDebugDraw::OCCLUDERS, ViewportDebugDraw::MOTION_VECTORS, ViewportDebugDraw::INTERNAL_BUFFER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportDebugDraw >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VIEWPORT_DEBUG_DRAW_DISABLED", ViewportDebugDraw::DISABLED), crate::meta::inspect::EnumConstant::new("UNSHADED", "VIEWPORT_DEBUG_DRAW_UNSHADED", ViewportDebugDraw::UNSHADED), crate::meta::inspect::EnumConstant::new("LIGHTING", "VIEWPORT_DEBUG_DRAW_LIGHTING", ViewportDebugDraw::LIGHTING), crate::meta::inspect::EnumConstant::new("OVERDRAW", "VIEWPORT_DEBUG_DRAW_OVERDRAW", ViewportDebugDraw::OVERDRAW), crate::meta::inspect::EnumConstant::new("WIREFRAME", "VIEWPORT_DEBUG_DRAW_WIREFRAME", ViewportDebugDraw::WIREFRAME), crate::meta::inspect::EnumConstant::new("NORMAL_BUFFER", "VIEWPORT_DEBUG_DRAW_NORMAL_BUFFER", ViewportDebugDraw::NORMAL_BUFFER), crate::meta::inspect::EnumConstant::new("VOXEL_GI_ALBEDO", "VIEWPORT_DEBUG_DRAW_VOXEL_GI_ALBEDO", ViewportDebugDraw::VOXEL_GI_ALBEDO), crate::meta::inspect::EnumConstant::new("VOXEL_GI_LIGHTING", "VIEWPORT_DEBUG_DRAW_VOXEL_GI_LIGHTING", ViewportDebugDraw::VOXEL_GI_LIGHTING), crate::meta::inspect::EnumConstant::new("VOXEL_GI_EMISSION", "VIEWPORT_DEBUG_DRAW_VOXEL_GI_EMISSION", ViewportDebugDraw::VOXEL_GI_EMISSION), crate::meta::inspect::EnumConstant::new("SHADOW_ATLAS", "VIEWPORT_DEBUG_DRAW_SHADOW_ATLAS", ViewportDebugDraw::SHADOW_ATLAS), crate::meta::inspect::EnumConstant::new("DIRECTIONAL_SHADOW_ATLAS", "VIEWPORT_DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS", ViewportDebugDraw::DIRECTIONAL_SHADOW_ATLAS), crate::meta::inspect::EnumConstant::new("SCENE_LUMINANCE", "VIEWPORT_DEBUG_DRAW_SCENE_LUMINANCE", ViewportDebugDraw::SCENE_LUMINANCE), crate::meta::inspect::EnumConstant::new("SSAO", "VIEWPORT_DEBUG_DRAW_SSAO", ViewportDebugDraw::SSAO), crate::meta::inspect::EnumConstant::new("SSIL", "VIEWPORT_DEBUG_DRAW_SSIL", ViewportDebugDraw::SSIL), crate::meta::inspect::EnumConstant::new("PSSM_SPLITS", "VIEWPORT_DEBUG_DRAW_PSSM_SPLITS", ViewportDebugDraw::PSSM_SPLITS), crate::meta::inspect::EnumConstant::new("DECAL_ATLAS", "VIEWPORT_DEBUG_DRAW_DECAL_ATLAS", ViewportDebugDraw::DECAL_ATLAS), crate::meta::inspect::EnumConstant::new("SDFGI", "VIEWPORT_DEBUG_DRAW_SDFGI", ViewportDebugDraw::SDFGI), crate::meta::inspect::EnumConstant::new("SDFGI_PROBES", "VIEWPORT_DEBUG_DRAW_SDFGI_PROBES", ViewportDebugDraw::SDFGI_PROBES), crate::meta::inspect::EnumConstant::new("GI_BUFFER", "VIEWPORT_DEBUG_DRAW_GI_BUFFER", ViewportDebugDraw::GI_BUFFER), crate::meta::inspect::EnumConstant::new("DISABLE_LOD", "VIEWPORT_DEBUG_DRAW_DISABLE_LOD", ViewportDebugDraw::DISABLE_LOD), crate::meta::inspect::EnumConstant::new("CLUSTER_OMNI_LIGHTS", "VIEWPORT_DEBUG_DRAW_CLUSTER_OMNI_LIGHTS", ViewportDebugDraw::CLUSTER_OMNI_LIGHTS), crate::meta::inspect::EnumConstant::new("CLUSTER_SPOT_LIGHTS", "VIEWPORT_DEBUG_DRAW_CLUSTER_SPOT_LIGHTS", ViewportDebugDraw::CLUSTER_SPOT_LIGHTS), crate::meta::inspect::EnumConstant::new("CLUSTER_DECALS", "VIEWPORT_DEBUG_DRAW_CLUSTER_DECALS", ViewportDebugDraw::CLUSTER_DECALS), crate::meta::inspect::EnumConstant::new("CLUSTER_REFLECTION_PROBES", "VIEWPORT_DEBUG_DRAW_CLUSTER_REFLECTION_PROBES", ViewportDebugDraw::CLUSTER_REFLECTION_PROBES), crate::meta::inspect::EnumConstant::new("OCCLUDERS", "VIEWPORT_DEBUG_DRAW_OCCLUDERS", ViewportDebugDraw::OCCLUDERS), crate::meta::inspect::EnumConstant::new("MOTION_VECTORS", "VIEWPORT_DEBUG_DRAW_MOTION_VECTORS", ViewportDebugDraw::MOTION_VECTORS), crate::meta::inspect::EnumConstant::new("INTERNAL_BUFFER", "VIEWPORT_DEBUG_DRAW_INTERNAL_BUFFER", ViewportDebugDraw::INTERNAL_BUFFER)]
        }
    }
}
impl crate::meta::GodotConvert for ViewportDebugDraw {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ViewportDebugDraw {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportDebugDraw {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `ViewportVRSMode`."]
pub struct ViewportVrsMode {
    ord: i32
}
impl ViewportVrsMode {
    #[doc(alias = "VIEWPORT_VRS_DISABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_VRS_DISABLED`"]
    pub const DISABLED: ViewportVrsMode = ViewportVrsMode {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_VRS_TEXTURE")]
    #[doc = "Godot enumerator name: `VIEWPORT_VRS_TEXTURE`"]
    pub const TEXTURE: ViewportVrsMode = ViewportVrsMode {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_VRS_XR")]
    #[doc = "Godot enumerator name: `VIEWPORT_VRS_XR`"]
    pub const XR: ViewportVrsMode = ViewportVrsMode {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_VRS_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_VRS_MAX`"]
    pub const MAX: ViewportVrsMode = ViewportVrsMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ViewportVrsMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportVrsMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportVrsMode {
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
            Self::DISABLED => "DISABLED", Self::TEXTURE => "TEXTURE", Self::XR => "XR", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportVrsMode::DISABLED, ViewportVrsMode::TEXTURE, ViewportVrsMode::XR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportVrsMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VIEWPORT_VRS_DISABLED", ViewportVrsMode::DISABLED), crate::meta::inspect::EnumConstant::new("TEXTURE", "VIEWPORT_VRS_TEXTURE", ViewportVrsMode::TEXTURE), crate::meta::inspect::EnumConstant::new("XR", "VIEWPORT_VRS_XR", ViewportVrsMode::XR), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_VRS_MAX", ViewportVrsMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportVrsMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ViewportVrsMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ViewportVrsMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportVrsMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `ViewportVRSUpdateMode`."]
pub struct ViewportVrsUpdateMode {
    ord: i32
}
impl ViewportVrsUpdateMode {
    #[doc(alias = "VIEWPORT_VRS_UPDATE_DISABLED")]
    #[doc = "Godot enumerator name: `VIEWPORT_VRS_UPDATE_DISABLED`"]
    pub const DISABLED: ViewportVrsUpdateMode = ViewportVrsUpdateMode {
        ord: 0i32
    };
    #[doc(alias = "VIEWPORT_VRS_UPDATE_ONCE")]
    #[doc = "Godot enumerator name: `VIEWPORT_VRS_UPDATE_ONCE`"]
    pub const ONCE: ViewportVrsUpdateMode = ViewportVrsUpdateMode {
        ord: 1i32
    };
    #[doc(alias = "VIEWPORT_VRS_UPDATE_ALWAYS")]
    #[doc = "Godot enumerator name: `VIEWPORT_VRS_UPDATE_ALWAYS`"]
    pub const ALWAYS: ViewportVrsUpdateMode = ViewportVrsUpdateMode {
        ord: 2i32
    };
    #[doc(alias = "VIEWPORT_VRS_UPDATE_MAX")]
    #[doc = "Godot enumerator name: `VIEWPORT_VRS_UPDATE_MAX`"]
    pub const MAX: ViewportVrsUpdateMode = ViewportVrsUpdateMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ViewportVrsUpdateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ViewportVrsUpdateMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ViewportVrsUpdateMode {
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
            Self::DISABLED => "DISABLED", Self::ONCE => "ONCE", Self::ALWAYS => "ALWAYS", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ViewportVrsUpdateMode::DISABLED, ViewportVrsUpdateMode::ONCE, ViewportVrsUpdateMode::ALWAYS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ViewportVrsUpdateMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VIEWPORT_VRS_UPDATE_DISABLED", ViewportVrsUpdateMode::DISABLED), crate::meta::inspect::EnumConstant::new("ONCE", "VIEWPORT_VRS_UPDATE_ONCE", ViewportVrsUpdateMode::ONCE), crate::meta::inspect::EnumConstant::new("ALWAYS", "VIEWPORT_VRS_UPDATE_ALWAYS", ViewportVrsUpdateMode::ALWAYS), crate::meta::inspect::EnumConstant::new("MAX", "VIEWPORT_VRS_UPDATE_MAX", ViewportVrsUpdateMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ViewportVrsUpdateMode {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for ViewportVrsUpdateMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ViewportVrsUpdateMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ViewportVrsUpdateMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SkyMode {
    ord: i32
}
impl SkyMode {
    #[doc(alias = "SKY_MODE_AUTOMATIC")]
    #[doc = "Godot enumerator name: `SKY_MODE_AUTOMATIC`"]
    pub const AUTOMATIC: SkyMode = SkyMode {
        ord: 0i32
    };
    #[doc(alias = "SKY_MODE_QUALITY")]
    #[doc = "Godot enumerator name: `SKY_MODE_QUALITY`"]
    pub const QUALITY: SkyMode = SkyMode {
        ord: 1i32
    };
    #[doc(alias = "SKY_MODE_INCREMENTAL")]
    #[doc = "Godot enumerator name: `SKY_MODE_INCREMENTAL`"]
    pub const INCREMENTAL: SkyMode = SkyMode {
        ord: 2i32
    };
    #[doc(alias = "SKY_MODE_REALTIME")]
    #[doc = "Godot enumerator name: `SKY_MODE_REALTIME`"]
    pub const REALTIME: SkyMode = SkyMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for SkyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SkyMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SkyMode {
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
            Self::AUTOMATIC => "AUTOMATIC", Self::QUALITY => "QUALITY", Self::INCREMENTAL => "INCREMENTAL", Self::REALTIME => "REALTIME", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SkyMode::AUTOMATIC, SkyMode::QUALITY, SkyMode::INCREMENTAL, SkyMode::REALTIME]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SkyMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("AUTOMATIC", "SKY_MODE_AUTOMATIC", SkyMode::AUTOMATIC), crate::meta::inspect::EnumConstant::new("QUALITY", "SKY_MODE_QUALITY", SkyMode::QUALITY), crate::meta::inspect::EnumConstant::new("INCREMENTAL", "SKY_MODE_INCREMENTAL", SkyMode::INCREMENTAL), crate::meta::inspect::EnumConstant::new("REALTIME", "SKY_MODE_REALTIME", SkyMode::REALTIME)]
        }
    }
}
impl crate::meta::GodotConvert for SkyMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SkyMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SkyMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompositorEffectFlags {
    ord: i32
}
impl CompositorEffectFlags {
    #[doc(alias = "COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_COLOR")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_COLOR`"]
    pub const ACCESS_RESOLVED_COLOR: CompositorEffectFlags = CompositorEffectFlags {
        ord: 1i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_DEPTH")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_DEPTH`"]
    pub const ACCESS_RESOLVED_DEPTH: CompositorEffectFlags = CompositorEffectFlags {
        ord: 2i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_FLAG_NEEDS_MOTION_VECTORS")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_FLAG_NEEDS_MOTION_VECTORS`"]
    pub const NEEDS_MOTION_VECTORS: CompositorEffectFlags = CompositorEffectFlags {
        ord: 4i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_FLAG_NEEDS_ROUGHNESS")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_FLAG_NEEDS_ROUGHNESS`"]
    pub const NEEDS_ROUGHNESS: CompositorEffectFlags = CompositorEffectFlags {
        ord: 8i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_FLAG_NEEDS_SEPARATE_SPECULAR")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_FLAG_NEEDS_SEPARATE_SPECULAR`"]
    pub const NEEDS_SEPARATE_SPECULAR: CompositorEffectFlags = CompositorEffectFlags {
        ord: 16i32
    };
    
}
impl std::fmt::Debug for CompositorEffectFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CompositorEffectFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CompositorEffectFlags {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 => Some(Self {
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
            Self::ACCESS_RESOLVED_COLOR => "ACCESS_RESOLVED_COLOR", Self::ACCESS_RESOLVED_DEPTH => "ACCESS_RESOLVED_DEPTH", Self::NEEDS_MOTION_VECTORS => "NEEDS_MOTION_VECTORS", Self::NEEDS_ROUGHNESS => "NEEDS_ROUGHNESS", Self::NEEDS_SEPARATE_SPECULAR => "NEEDS_SEPARATE_SPECULAR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CompositorEffectFlags::ACCESS_RESOLVED_COLOR, CompositorEffectFlags::ACCESS_RESOLVED_DEPTH, CompositorEffectFlags::NEEDS_MOTION_VECTORS, CompositorEffectFlags::NEEDS_ROUGHNESS, CompositorEffectFlags::NEEDS_SEPARATE_SPECULAR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CompositorEffectFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ACCESS_RESOLVED_COLOR", "COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_COLOR", CompositorEffectFlags::ACCESS_RESOLVED_COLOR), crate::meta::inspect::EnumConstant::new("ACCESS_RESOLVED_DEPTH", "COMPOSITOR_EFFECT_FLAG_ACCESS_RESOLVED_DEPTH", CompositorEffectFlags::ACCESS_RESOLVED_DEPTH), crate::meta::inspect::EnumConstant::new("NEEDS_MOTION_VECTORS", "COMPOSITOR_EFFECT_FLAG_NEEDS_MOTION_VECTORS", CompositorEffectFlags::NEEDS_MOTION_VECTORS), crate::meta::inspect::EnumConstant::new("NEEDS_ROUGHNESS", "COMPOSITOR_EFFECT_FLAG_NEEDS_ROUGHNESS", CompositorEffectFlags::NEEDS_ROUGHNESS), crate::meta::inspect::EnumConstant::new("NEEDS_SEPARATE_SPECULAR", "COMPOSITOR_EFFECT_FLAG_NEEDS_SEPARATE_SPECULAR", CompositorEffectFlags::NEEDS_SEPARATE_SPECULAR)]
        }
    }
}
impl crate::meta::GodotConvert for CompositorEffectFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CompositorEffectFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CompositorEffectFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompositorEffectCallbackType {
    ord: i32
}
impl CompositorEffectCallbackType {
    #[doc(alias = "COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_OPAQUE")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_OPAQUE`"]
    pub const PRE_OPAQUE: CompositorEffectCallbackType = CompositorEffectCallbackType {
        ord: 0i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_OPAQUE")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_OPAQUE`"]
    pub const POST_OPAQUE: CompositorEffectCallbackType = CompositorEffectCallbackType {
        ord: 1i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_SKY")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_SKY`"]
    pub const POST_SKY: CompositorEffectCallbackType = CompositorEffectCallbackType {
        ord: 2i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_TRANSPARENT")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_TRANSPARENT`"]
    pub const PRE_TRANSPARENT: CompositorEffectCallbackType = CompositorEffectCallbackType {
        ord: 3i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_TRANSPARENT")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_TRANSPARENT`"]
    pub const POST_TRANSPARENT: CompositorEffectCallbackType = CompositorEffectCallbackType {
        ord: 4i32
    };
    #[doc(alias = "COMPOSITOR_EFFECT_CALLBACK_TYPE_ANY")]
    #[doc = "Godot enumerator name: `COMPOSITOR_EFFECT_CALLBACK_TYPE_ANY`"]
    pub const ANY: CompositorEffectCallbackType = CompositorEffectCallbackType {
        ord: - 1i32
    };
    
}
impl std::fmt::Debug for CompositorEffectCallbackType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CompositorEffectCallbackType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CompositorEffectCallbackType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ - 1i32 | ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::PRE_OPAQUE => "PRE_OPAQUE", Self::POST_OPAQUE => "POST_OPAQUE", Self::POST_SKY => "POST_SKY", Self::PRE_TRANSPARENT => "PRE_TRANSPARENT", Self::POST_TRANSPARENT => "POST_TRANSPARENT", Self::ANY => "ANY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CompositorEffectCallbackType::PRE_OPAQUE, CompositorEffectCallbackType::POST_OPAQUE, CompositorEffectCallbackType::POST_SKY, CompositorEffectCallbackType::PRE_TRANSPARENT, CompositorEffectCallbackType::POST_TRANSPARENT, CompositorEffectCallbackType::ANY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CompositorEffectCallbackType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PRE_OPAQUE", "COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_OPAQUE", CompositorEffectCallbackType::PRE_OPAQUE), crate::meta::inspect::EnumConstant::new("POST_OPAQUE", "COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_OPAQUE", CompositorEffectCallbackType::POST_OPAQUE), crate::meta::inspect::EnumConstant::new("POST_SKY", "COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_SKY", CompositorEffectCallbackType::POST_SKY), crate::meta::inspect::EnumConstant::new("PRE_TRANSPARENT", "COMPOSITOR_EFFECT_CALLBACK_TYPE_PRE_TRANSPARENT", CompositorEffectCallbackType::PRE_TRANSPARENT), crate::meta::inspect::EnumConstant::new("POST_TRANSPARENT", "COMPOSITOR_EFFECT_CALLBACK_TYPE_POST_TRANSPARENT", CompositorEffectCallbackType::POST_TRANSPARENT), crate::meta::inspect::EnumConstant::new("ANY", "COMPOSITOR_EFFECT_CALLBACK_TYPE_ANY", CompositorEffectCallbackType::ANY)]
        }
    }
}
impl crate::meta::GodotConvert for CompositorEffectCallbackType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CompositorEffectCallbackType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CompositorEffectCallbackType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `EnvironmentBG`."]
pub struct EnvironmentBg {
    ord: i32
}
impl EnvironmentBg {
    #[doc(alias = "ENV_BG_CLEAR_COLOR")]
    #[doc = "Godot enumerator name: `ENV_BG_CLEAR_COLOR`"]
    pub const CLEAR_COLOR: EnvironmentBg = EnvironmentBg {
        ord: 0i32
    };
    #[doc(alias = "ENV_BG_COLOR")]
    #[doc = "Godot enumerator name: `ENV_BG_COLOR`"]
    pub const COLOR: EnvironmentBg = EnvironmentBg {
        ord: 1i32
    };
    #[doc(alias = "ENV_BG_SKY")]
    #[doc = "Godot enumerator name: `ENV_BG_SKY`"]
    pub const SKY: EnvironmentBg = EnvironmentBg {
        ord: 2i32
    };
    #[doc(alias = "ENV_BG_CANVAS")]
    #[doc = "Godot enumerator name: `ENV_BG_CANVAS`"]
    pub const CANVAS: EnvironmentBg = EnvironmentBg {
        ord: 3i32
    };
    #[doc(alias = "ENV_BG_KEEP")]
    #[doc = "Godot enumerator name: `ENV_BG_KEEP`"]
    pub const KEEP: EnvironmentBg = EnvironmentBg {
        ord: 4i32
    };
    #[doc(alias = "ENV_BG_CAMERA_FEED")]
    #[doc = "Godot enumerator name: `ENV_BG_CAMERA_FEED`"]
    pub const CAMERA_FEED: EnvironmentBg = EnvironmentBg {
        ord: 5i32
    };
    #[doc(alias = "ENV_BG_MAX")]
    #[doc = "Godot enumerator name: `ENV_BG_MAX`"]
    pub const MAX: EnvironmentBg = EnvironmentBg {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for EnvironmentBg {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentBg") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentBg {
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
            Self::CLEAR_COLOR => "CLEAR_COLOR", Self::COLOR => "COLOR", Self::SKY => "SKY", Self::CANVAS => "CANVAS", Self::KEEP => "KEEP", Self::CAMERA_FEED => "CAMERA_FEED", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentBg::CLEAR_COLOR, EnvironmentBg::COLOR, EnvironmentBg::SKY, EnvironmentBg::CANVAS, EnvironmentBg::KEEP, EnvironmentBg::CAMERA_FEED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentBg >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CLEAR_COLOR", "ENV_BG_CLEAR_COLOR", EnvironmentBg::CLEAR_COLOR), crate::meta::inspect::EnumConstant::new("COLOR", "ENV_BG_COLOR", EnvironmentBg::COLOR), crate::meta::inspect::EnumConstant::new("SKY", "ENV_BG_SKY", EnvironmentBg::SKY), crate::meta::inspect::EnumConstant::new("CANVAS", "ENV_BG_CANVAS", EnvironmentBg::CANVAS), crate::meta::inspect::EnumConstant::new("KEEP", "ENV_BG_KEEP", EnvironmentBg::KEEP), crate::meta::inspect::EnumConstant::new("CAMERA_FEED", "ENV_BG_CAMERA_FEED", EnvironmentBg::CAMERA_FEED), crate::meta::inspect::EnumConstant::new("MAX", "ENV_BG_MAX", EnvironmentBg::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for EnvironmentBg {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for EnvironmentBg {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnvironmentBg {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentBg {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EnvironmentAmbientSource {
    ord: i32
}
impl EnvironmentAmbientSource {
    #[doc(alias = "ENV_AMBIENT_SOURCE_BG")]
    #[doc = "Godot enumerator name: `ENV_AMBIENT_SOURCE_BG`"]
    pub const BG: EnvironmentAmbientSource = EnvironmentAmbientSource {
        ord: 0i32
    };
    #[doc(alias = "ENV_AMBIENT_SOURCE_DISABLED")]
    #[doc = "Godot enumerator name: `ENV_AMBIENT_SOURCE_DISABLED`"]
    pub const DISABLED: EnvironmentAmbientSource = EnvironmentAmbientSource {
        ord: 1i32
    };
    #[doc(alias = "ENV_AMBIENT_SOURCE_COLOR")]
    #[doc = "Godot enumerator name: `ENV_AMBIENT_SOURCE_COLOR`"]
    pub const COLOR: EnvironmentAmbientSource = EnvironmentAmbientSource {
        ord: 2i32
    };
    #[doc(alias = "ENV_AMBIENT_SOURCE_SKY")]
    #[doc = "Godot enumerator name: `ENV_AMBIENT_SOURCE_SKY`"]
    pub const SKY: EnvironmentAmbientSource = EnvironmentAmbientSource {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for EnvironmentAmbientSource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentAmbientSource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentAmbientSource {
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
            Self::BG => "BG", Self::DISABLED => "DISABLED", Self::COLOR => "COLOR", Self::SKY => "SKY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentAmbientSource::BG, EnvironmentAmbientSource::DISABLED, EnvironmentAmbientSource::COLOR, EnvironmentAmbientSource::SKY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentAmbientSource >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BG", "ENV_AMBIENT_SOURCE_BG", EnvironmentAmbientSource::BG), crate::meta::inspect::EnumConstant::new("DISABLED", "ENV_AMBIENT_SOURCE_DISABLED", EnvironmentAmbientSource::DISABLED), crate::meta::inspect::EnumConstant::new("COLOR", "ENV_AMBIENT_SOURCE_COLOR", EnvironmentAmbientSource::COLOR), crate::meta::inspect::EnumConstant::new("SKY", "ENV_AMBIENT_SOURCE_SKY", EnvironmentAmbientSource::SKY)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentAmbientSource {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnvironmentAmbientSource {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentAmbientSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EnvironmentReflectionSource {
    ord: i32
}
impl EnvironmentReflectionSource {
    #[doc(alias = "ENV_REFLECTION_SOURCE_BG")]
    #[doc = "Godot enumerator name: `ENV_REFLECTION_SOURCE_BG`"]
    pub const BG: EnvironmentReflectionSource = EnvironmentReflectionSource {
        ord: 0i32
    };
    #[doc(alias = "ENV_REFLECTION_SOURCE_DISABLED")]
    #[doc = "Godot enumerator name: `ENV_REFLECTION_SOURCE_DISABLED`"]
    pub const DISABLED: EnvironmentReflectionSource = EnvironmentReflectionSource {
        ord: 1i32
    };
    #[doc(alias = "ENV_REFLECTION_SOURCE_SKY")]
    #[doc = "Godot enumerator name: `ENV_REFLECTION_SOURCE_SKY`"]
    pub const SKY: EnvironmentReflectionSource = EnvironmentReflectionSource {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for EnvironmentReflectionSource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentReflectionSource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentReflectionSource {
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
            Self::BG => "BG", Self::DISABLED => "DISABLED", Self::SKY => "SKY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentReflectionSource::BG, EnvironmentReflectionSource::DISABLED, EnvironmentReflectionSource::SKY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentReflectionSource >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BG", "ENV_REFLECTION_SOURCE_BG", EnvironmentReflectionSource::BG), crate::meta::inspect::EnumConstant::new("DISABLED", "ENV_REFLECTION_SOURCE_DISABLED", EnvironmentReflectionSource::DISABLED), crate::meta::inspect::EnumConstant::new("SKY", "ENV_REFLECTION_SOURCE_SKY", EnvironmentReflectionSource::SKY)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentReflectionSource {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnvironmentReflectionSource {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentReflectionSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EnvironmentGlowBlendMode {
    ord: i32
}
impl EnvironmentGlowBlendMode {
    #[doc(alias = "ENV_GLOW_BLEND_MODE_ADDITIVE")]
    #[doc = "Godot enumerator name: `ENV_GLOW_BLEND_MODE_ADDITIVE`"]
    pub const ADDITIVE: EnvironmentGlowBlendMode = EnvironmentGlowBlendMode {
        ord: 0i32
    };
    #[doc(alias = "ENV_GLOW_BLEND_MODE_SCREEN")]
    #[doc = "Godot enumerator name: `ENV_GLOW_BLEND_MODE_SCREEN`"]
    pub const SCREEN: EnvironmentGlowBlendMode = EnvironmentGlowBlendMode {
        ord: 1i32
    };
    #[doc(alias = "ENV_GLOW_BLEND_MODE_SOFTLIGHT")]
    #[doc = "Godot enumerator name: `ENV_GLOW_BLEND_MODE_SOFTLIGHT`"]
    pub const SOFTLIGHT: EnvironmentGlowBlendMode = EnvironmentGlowBlendMode {
        ord: 2i32
    };
    #[doc(alias = "ENV_GLOW_BLEND_MODE_REPLACE")]
    #[doc = "Godot enumerator name: `ENV_GLOW_BLEND_MODE_REPLACE`"]
    pub const REPLACE: EnvironmentGlowBlendMode = EnvironmentGlowBlendMode {
        ord: 3i32
    };
    #[doc(alias = "ENV_GLOW_BLEND_MODE_MIX")]
    #[doc = "Godot enumerator name: `ENV_GLOW_BLEND_MODE_MIX`"]
    pub const MIX: EnvironmentGlowBlendMode = EnvironmentGlowBlendMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for EnvironmentGlowBlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentGlowBlendMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentGlowBlendMode {
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
            Self::ADDITIVE => "ADDITIVE", Self::SCREEN => "SCREEN", Self::SOFTLIGHT => "SOFTLIGHT", Self::REPLACE => "REPLACE", Self::MIX => "MIX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentGlowBlendMode::ADDITIVE, EnvironmentGlowBlendMode::SCREEN, EnvironmentGlowBlendMode::SOFTLIGHT, EnvironmentGlowBlendMode::REPLACE, EnvironmentGlowBlendMode::MIX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentGlowBlendMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ADDITIVE", "ENV_GLOW_BLEND_MODE_ADDITIVE", EnvironmentGlowBlendMode::ADDITIVE), crate::meta::inspect::EnumConstant::new("SCREEN", "ENV_GLOW_BLEND_MODE_SCREEN", EnvironmentGlowBlendMode::SCREEN), crate::meta::inspect::EnumConstant::new("SOFTLIGHT", "ENV_GLOW_BLEND_MODE_SOFTLIGHT", EnvironmentGlowBlendMode::SOFTLIGHT), crate::meta::inspect::EnumConstant::new("REPLACE", "ENV_GLOW_BLEND_MODE_REPLACE", EnvironmentGlowBlendMode::REPLACE), crate::meta::inspect::EnumConstant::new("MIX", "ENV_GLOW_BLEND_MODE_MIX", EnvironmentGlowBlendMode::MIX)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentGlowBlendMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnvironmentGlowBlendMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentGlowBlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EnvironmentFogMode {
    ord: i32
}
impl EnvironmentFogMode {
    #[doc(alias = "ENV_FOG_MODE_EXPONENTIAL")]
    #[doc = "Godot enumerator name: `ENV_FOG_MODE_EXPONENTIAL`"]
    pub const EXPONENTIAL: EnvironmentFogMode = EnvironmentFogMode {
        ord: 0i32
    };
    #[doc(alias = "ENV_FOG_MODE_DEPTH")]
    #[doc = "Godot enumerator name: `ENV_FOG_MODE_DEPTH`"]
    pub const DEPTH: EnvironmentFogMode = EnvironmentFogMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for EnvironmentFogMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentFogMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentFogMode {
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
            Self::EXPONENTIAL => "EXPONENTIAL", Self::DEPTH => "DEPTH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentFogMode::EXPONENTIAL, EnvironmentFogMode::DEPTH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentFogMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("EXPONENTIAL", "ENV_FOG_MODE_EXPONENTIAL", EnvironmentFogMode::EXPONENTIAL), crate::meta::inspect::EnumConstant::new("DEPTH", "ENV_FOG_MODE_DEPTH", EnvironmentFogMode::DEPTH)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentFogMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnvironmentFogMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentFogMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EnvironmentToneMapper {
    ord: i32
}
impl EnvironmentToneMapper {
    #[doc(alias = "ENV_TONE_MAPPER_LINEAR")]
    #[doc = "Godot enumerator name: `ENV_TONE_MAPPER_LINEAR`"]
    pub const LINEAR: EnvironmentToneMapper = EnvironmentToneMapper {
        ord: 0i32
    };
    #[doc(alias = "ENV_TONE_MAPPER_REINHARD")]
    #[doc = "Godot enumerator name: `ENV_TONE_MAPPER_REINHARD`"]
    pub const REINHARD: EnvironmentToneMapper = EnvironmentToneMapper {
        ord: 1i32
    };
    #[doc(alias = "ENV_TONE_MAPPER_FILMIC")]
    #[doc = "Godot enumerator name: `ENV_TONE_MAPPER_FILMIC`"]
    pub const FILMIC: EnvironmentToneMapper = EnvironmentToneMapper {
        ord: 2i32
    };
    #[doc(alias = "ENV_TONE_MAPPER_ACES")]
    #[doc = "Godot enumerator name: `ENV_TONE_MAPPER_ACES`"]
    pub const ACES: EnvironmentToneMapper = EnvironmentToneMapper {
        ord: 3i32
    };
    #[doc(alias = "ENV_TONE_MAPPER_AGX")]
    #[doc = "Godot enumerator name: `ENV_TONE_MAPPER_AGX`"]
    pub const AGX: EnvironmentToneMapper = EnvironmentToneMapper {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for EnvironmentToneMapper {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentToneMapper") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentToneMapper {
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
            Self::LINEAR => "LINEAR", Self::REINHARD => "REINHARD", Self::FILMIC => "FILMIC", Self::ACES => "ACES", Self::AGX => "AGX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentToneMapper::LINEAR, EnvironmentToneMapper::REINHARD, EnvironmentToneMapper::FILMIC, EnvironmentToneMapper::ACES, EnvironmentToneMapper::AGX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentToneMapper >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LINEAR", "ENV_TONE_MAPPER_LINEAR", EnvironmentToneMapper::LINEAR), crate::meta::inspect::EnumConstant::new("REINHARD", "ENV_TONE_MAPPER_REINHARD", EnvironmentToneMapper::REINHARD), crate::meta::inspect::EnumConstant::new("FILMIC", "ENV_TONE_MAPPER_FILMIC", EnvironmentToneMapper::FILMIC), crate::meta::inspect::EnumConstant::new("ACES", "ENV_TONE_MAPPER_ACES", EnvironmentToneMapper::ACES), crate::meta::inspect::EnumConstant::new("AGX", "ENV_TONE_MAPPER_AGX", EnvironmentToneMapper::AGX)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentToneMapper {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnvironmentToneMapper {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentToneMapper {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `EnvironmentSSRRoughnessQuality`."]
pub struct EnvironmentSsrRoughnessQuality {
    ord: i32
}
impl EnvironmentSsrRoughnessQuality {
    #[doc(alias = "ENV_SSR_ROUGHNESS_QUALITY_DISABLED")]
    #[doc = "Godot enumerator name: `ENV_SSR_ROUGHNESS_QUALITY_DISABLED`"]
    pub const DISABLED: EnvironmentSsrRoughnessQuality = EnvironmentSsrRoughnessQuality {
        ord: 0i32
    };
    #[doc(alias = "ENV_SSR_ROUGHNESS_QUALITY_LOW")]
    #[doc = "Godot enumerator name: `ENV_SSR_ROUGHNESS_QUALITY_LOW`"]
    pub const LOW: EnvironmentSsrRoughnessQuality = EnvironmentSsrRoughnessQuality {
        ord: 1i32
    };
    #[doc(alias = "ENV_SSR_ROUGHNESS_QUALITY_MEDIUM")]
    #[doc = "Godot enumerator name: `ENV_SSR_ROUGHNESS_QUALITY_MEDIUM`"]
    pub const MEDIUM: EnvironmentSsrRoughnessQuality = EnvironmentSsrRoughnessQuality {
        ord: 2i32
    };
    #[doc(alias = "ENV_SSR_ROUGHNESS_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `ENV_SSR_ROUGHNESS_QUALITY_HIGH`"]
    pub const HIGH: EnvironmentSsrRoughnessQuality = EnvironmentSsrRoughnessQuality {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for EnvironmentSsrRoughnessQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentSsrRoughnessQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentSsrRoughnessQuality {
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
            Self::DISABLED => "DISABLED", Self::LOW => "LOW", Self::MEDIUM => "MEDIUM", Self::HIGH => "HIGH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentSsrRoughnessQuality::DISABLED, EnvironmentSsrRoughnessQuality::LOW, EnvironmentSsrRoughnessQuality::MEDIUM, EnvironmentSsrRoughnessQuality::HIGH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentSsrRoughnessQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "ENV_SSR_ROUGHNESS_QUALITY_DISABLED", EnvironmentSsrRoughnessQuality::DISABLED), crate::meta::inspect::EnumConstant::new("LOW", "ENV_SSR_ROUGHNESS_QUALITY_LOW", EnvironmentSsrRoughnessQuality::LOW), crate::meta::inspect::EnumConstant::new("MEDIUM", "ENV_SSR_ROUGHNESS_QUALITY_MEDIUM", EnvironmentSsrRoughnessQuality::MEDIUM), crate::meta::inspect::EnumConstant::new("HIGH", "ENV_SSR_ROUGHNESS_QUALITY_HIGH", EnvironmentSsrRoughnessQuality::HIGH)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentSsrRoughnessQuality {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnvironmentSsrRoughnessQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentSsrRoughnessQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `EnvironmentSSAOQuality`."]
pub struct EnvironmentSsaoQuality {
    ord: i32
}
impl EnvironmentSsaoQuality {
    #[doc(alias = "ENV_SSAO_QUALITY_VERY_LOW")]
    #[doc = "Godot enumerator name: `ENV_SSAO_QUALITY_VERY_LOW`"]
    pub const VERY_LOW: EnvironmentSsaoQuality = EnvironmentSsaoQuality {
        ord: 0i32
    };
    #[doc(alias = "ENV_SSAO_QUALITY_LOW")]
    #[doc = "Godot enumerator name: `ENV_SSAO_QUALITY_LOW`"]
    pub const LOW: EnvironmentSsaoQuality = EnvironmentSsaoQuality {
        ord: 1i32
    };
    #[doc(alias = "ENV_SSAO_QUALITY_MEDIUM")]
    #[doc = "Godot enumerator name: `ENV_SSAO_QUALITY_MEDIUM`"]
    pub const MEDIUM: EnvironmentSsaoQuality = EnvironmentSsaoQuality {
        ord: 2i32
    };
    #[doc(alias = "ENV_SSAO_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `ENV_SSAO_QUALITY_HIGH`"]
    pub const HIGH: EnvironmentSsaoQuality = EnvironmentSsaoQuality {
        ord: 3i32
    };
    #[doc(alias = "ENV_SSAO_QUALITY_ULTRA")]
    #[doc = "Godot enumerator name: `ENV_SSAO_QUALITY_ULTRA`"]
    pub const ULTRA: EnvironmentSsaoQuality = EnvironmentSsaoQuality {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for EnvironmentSsaoQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentSsaoQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentSsaoQuality {
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
            Self::VERY_LOW => "VERY_LOW", Self::LOW => "LOW", Self::MEDIUM => "MEDIUM", Self::HIGH => "HIGH", Self::ULTRA => "ULTRA", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentSsaoQuality::VERY_LOW, EnvironmentSsaoQuality::LOW, EnvironmentSsaoQuality::MEDIUM, EnvironmentSsaoQuality::HIGH, EnvironmentSsaoQuality::ULTRA]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentSsaoQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERY_LOW", "ENV_SSAO_QUALITY_VERY_LOW", EnvironmentSsaoQuality::VERY_LOW), crate::meta::inspect::EnumConstant::new("LOW", "ENV_SSAO_QUALITY_LOW", EnvironmentSsaoQuality::LOW), crate::meta::inspect::EnumConstant::new("MEDIUM", "ENV_SSAO_QUALITY_MEDIUM", EnvironmentSsaoQuality::MEDIUM), crate::meta::inspect::EnumConstant::new("HIGH", "ENV_SSAO_QUALITY_HIGH", EnvironmentSsaoQuality::HIGH), crate::meta::inspect::EnumConstant::new("ULTRA", "ENV_SSAO_QUALITY_ULTRA", EnvironmentSsaoQuality::ULTRA)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentSsaoQuality {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnvironmentSsaoQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentSsaoQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `EnvironmentSSILQuality`."]
pub struct EnvironmentSsilQuality {
    ord: i32
}
impl EnvironmentSsilQuality {
    #[doc(alias = "ENV_SSIL_QUALITY_VERY_LOW")]
    #[doc = "Godot enumerator name: `ENV_SSIL_QUALITY_VERY_LOW`"]
    pub const VERY_LOW: EnvironmentSsilQuality = EnvironmentSsilQuality {
        ord: 0i32
    };
    #[doc(alias = "ENV_SSIL_QUALITY_LOW")]
    #[doc = "Godot enumerator name: `ENV_SSIL_QUALITY_LOW`"]
    pub const LOW: EnvironmentSsilQuality = EnvironmentSsilQuality {
        ord: 1i32
    };
    #[doc(alias = "ENV_SSIL_QUALITY_MEDIUM")]
    #[doc = "Godot enumerator name: `ENV_SSIL_QUALITY_MEDIUM`"]
    pub const MEDIUM: EnvironmentSsilQuality = EnvironmentSsilQuality {
        ord: 2i32
    };
    #[doc(alias = "ENV_SSIL_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `ENV_SSIL_QUALITY_HIGH`"]
    pub const HIGH: EnvironmentSsilQuality = EnvironmentSsilQuality {
        ord: 3i32
    };
    #[doc(alias = "ENV_SSIL_QUALITY_ULTRA")]
    #[doc = "Godot enumerator name: `ENV_SSIL_QUALITY_ULTRA`"]
    pub const ULTRA: EnvironmentSsilQuality = EnvironmentSsilQuality {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for EnvironmentSsilQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentSsilQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentSsilQuality {
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
            Self::VERY_LOW => "VERY_LOW", Self::LOW => "LOW", Self::MEDIUM => "MEDIUM", Self::HIGH => "HIGH", Self::ULTRA => "ULTRA", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentSsilQuality::VERY_LOW, EnvironmentSsilQuality::LOW, EnvironmentSsilQuality::MEDIUM, EnvironmentSsilQuality::HIGH, EnvironmentSsilQuality::ULTRA]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentSsilQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERY_LOW", "ENV_SSIL_QUALITY_VERY_LOW", EnvironmentSsilQuality::VERY_LOW), crate::meta::inspect::EnumConstant::new("LOW", "ENV_SSIL_QUALITY_LOW", EnvironmentSsilQuality::LOW), crate::meta::inspect::EnumConstant::new("MEDIUM", "ENV_SSIL_QUALITY_MEDIUM", EnvironmentSsilQuality::MEDIUM), crate::meta::inspect::EnumConstant::new("HIGH", "ENV_SSIL_QUALITY_HIGH", EnvironmentSsilQuality::HIGH), crate::meta::inspect::EnumConstant::new("ULTRA", "ENV_SSIL_QUALITY_ULTRA", EnvironmentSsilQuality::ULTRA)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentSsilQuality {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnvironmentSsilQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentSsilQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `EnvironmentSDFGIYScale`."]
pub struct EnvironmentSdfgiYScale {
    ord: i32
}
impl EnvironmentSdfgiYScale {
    #[doc(alias = "ENV_SDFGI_Y_SCALE_50_PERCENT")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_Y_SCALE_50_PERCENT`"]
    pub const SCALE_50_PERCENT: EnvironmentSdfgiYScale = EnvironmentSdfgiYScale {
        ord: 0i32
    };
    #[doc(alias = "ENV_SDFGI_Y_SCALE_75_PERCENT")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_Y_SCALE_75_PERCENT`"]
    pub const SCALE_75_PERCENT: EnvironmentSdfgiYScale = EnvironmentSdfgiYScale {
        ord: 1i32
    };
    #[doc(alias = "ENV_SDFGI_Y_SCALE_100_PERCENT")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_Y_SCALE_100_PERCENT`"]
    pub const SCALE_100_PERCENT: EnvironmentSdfgiYScale = EnvironmentSdfgiYScale {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for EnvironmentSdfgiYScale {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentSdfgiYScale") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentSdfgiYScale {
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
            Self::SCALE_50_PERCENT => "SCALE_50_PERCENT", Self::SCALE_75_PERCENT => "SCALE_75_PERCENT", Self::SCALE_100_PERCENT => "SCALE_100_PERCENT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentSdfgiYScale::SCALE_50_PERCENT, EnvironmentSdfgiYScale::SCALE_75_PERCENT, EnvironmentSdfgiYScale::SCALE_100_PERCENT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentSdfgiYScale >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SCALE_50_PERCENT", "ENV_SDFGI_Y_SCALE_50_PERCENT", EnvironmentSdfgiYScale::SCALE_50_PERCENT), crate::meta::inspect::EnumConstant::new("SCALE_75_PERCENT", "ENV_SDFGI_Y_SCALE_75_PERCENT", EnvironmentSdfgiYScale::SCALE_75_PERCENT), crate::meta::inspect::EnumConstant::new("SCALE_100_PERCENT", "ENV_SDFGI_Y_SCALE_100_PERCENT", EnvironmentSdfgiYScale::SCALE_100_PERCENT)]
        }
    }
}
impl crate::meta::GodotConvert for EnvironmentSdfgiYScale {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnvironmentSdfgiYScale {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentSdfgiYScale {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `EnvironmentSDFGIRayCount`."]
pub struct EnvironmentSdfgiRayCount {
    ord: i32
}
impl EnvironmentSdfgiRayCount {
    #[doc(alias = "ENV_SDFGI_RAY_COUNT_4")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_RAY_COUNT_4`"]
    pub const COUNT_4: EnvironmentSdfgiRayCount = EnvironmentSdfgiRayCount {
        ord: 0i32
    };
    #[doc(alias = "ENV_SDFGI_RAY_COUNT_8")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_RAY_COUNT_8`"]
    pub const COUNT_8: EnvironmentSdfgiRayCount = EnvironmentSdfgiRayCount {
        ord: 1i32
    };
    #[doc(alias = "ENV_SDFGI_RAY_COUNT_16")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_RAY_COUNT_16`"]
    pub const COUNT_16: EnvironmentSdfgiRayCount = EnvironmentSdfgiRayCount {
        ord: 2i32
    };
    #[doc(alias = "ENV_SDFGI_RAY_COUNT_32")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_RAY_COUNT_32`"]
    pub const COUNT_32: EnvironmentSdfgiRayCount = EnvironmentSdfgiRayCount {
        ord: 3i32
    };
    #[doc(alias = "ENV_SDFGI_RAY_COUNT_64")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_RAY_COUNT_64`"]
    pub const COUNT_64: EnvironmentSdfgiRayCount = EnvironmentSdfgiRayCount {
        ord: 4i32
    };
    #[doc(alias = "ENV_SDFGI_RAY_COUNT_96")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_RAY_COUNT_96`"]
    pub const COUNT_96: EnvironmentSdfgiRayCount = EnvironmentSdfgiRayCount {
        ord: 5i32
    };
    #[doc(alias = "ENV_SDFGI_RAY_COUNT_128")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_RAY_COUNT_128`"]
    pub const COUNT_128: EnvironmentSdfgiRayCount = EnvironmentSdfgiRayCount {
        ord: 6i32
    };
    #[doc(alias = "ENV_SDFGI_RAY_COUNT_MAX")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_RAY_COUNT_MAX`"]
    pub const MAX: EnvironmentSdfgiRayCount = EnvironmentSdfgiRayCount {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for EnvironmentSdfgiRayCount {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentSdfgiRayCount") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentSdfgiRayCount {
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
            Self::COUNT_4 => "COUNT_4", Self::COUNT_8 => "COUNT_8", Self::COUNT_16 => "COUNT_16", Self::COUNT_32 => "COUNT_32", Self::COUNT_64 => "COUNT_64", Self::COUNT_96 => "COUNT_96", Self::COUNT_128 => "COUNT_128", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentSdfgiRayCount::COUNT_4, EnvironmentSdfgiRayCount::COUNT_8, EnvironmentSdfgiRayCount::COUNT_16, EnvironmentSdfgiRayCount::COUNT_32, EnvironmentSdfgiRayCount::COUNT_64, EnvironmentSdfgiRayCount::COUNT_96, EnvironmentSdfgiRayCount::COUNT_128]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentSdfgiRayCount >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("COUNT_4", "ENV_SDFGI_RAY_COUNT_4", EnvironmentSdfgiRayCount::COUNT_4), crate::meta::inspect::EnumConstant::new("COUNT_8", "ENV_SDFGI_RAY_COUNT_8", EnvironmentSdfgiRayCount::COUNT_8), crate::meta::inspect::EnumConstant::new("COUNT_16", "ENV_SDFGI_RAY_COUNT_16", EnvironmentSdfgiRayCount::COUNT_16), crate::meta::inspect::EnumConstant::new("COUNT_32", "ENV_SDFGI_RAY_COUNT_32", EnvironmentSdfgiRayCount::COUNT_32), crate::meta::inspect::EnumConstant::new("COUNT_64", "ENV_SDFGI_RAY_COUNT_64", EnvironmentSdfgiRayCount::COUNT_64), crate::meta::inspect::EnumConstant::new("COUNT_96", "ENV_SDFGI_RAY_COUNT_96", EnvironmentSdfgiRayCount::COUNT_96), crate::meta::inspect::EnumConstant::new("COUNT_128", "ENV_SDFGI_RAY_COUNT_128", EnvironmentSdfgiRayCount::COUNT_128), crate::meta::inspect::EnumConstant::new("MAX", "ENV_SDFGI_RAY_COUNT_MAX", EnvironmentSdfgiRayCount::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for EnvironmentSdfgiRayCount {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for EnvironmentSdfgiRayCount {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnvironmentSdfgiRayCount {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentSdfgiRayCount {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `EnvironmentSDFGIFramesToConverge`."]
pub struct EnvironmentSdfgiFramesToConverge {
    ord: i32
}
impl EnvironmentSdfgiFramesToConverge {
    #[doc(alias = "ENV_SDFGI_CONVERGE_IN_5_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_CONVERGE_IN_5_FRAMES`"]
    pub const IN_5_FRAMES: EnvironmentSdfgiFramesToConverge = EnvironmentSdfgiFramesToConverge {
        ord: 0i32
    };
    #[doc(alias = "ENV_SDFGI_CONVERGE_IN_10_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_CONVERGE_IN_10_FRAMES`"]
    pub const IN_10_FRAMES: EnvironmentSdfgiFramesToConverge = EnvironmentSdfgiFramesToConverge {
        ord: 1i32
    };
    #[doc(alias = "ENV_SDFGI_CONVERGE_IN_15_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_CONVERGE_IN_15_FRAMES`"]
    pub const IN_15_FRAMES: EnvironmentSdfgiFramesToConverge = EnvironmentSdfgiFramesToConverge {
        ord: 2i32
    };
    #[doc(alias = "ENV_SDFGI_CONVERGE_IN_20_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_CONVERGE_IN_20_FRAMES`"]
    pub const IN_20_FRAMES: EnvironmentSdfgiFramesToConverge = EnvironmentSdfgiFramesToConverge {
        ord: 3i32
    };
    #[doc(alias = "ENV_SDFGI_CONVERGE_IN_25_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_CONVERGE_IN_25_FRAMES`"]
    pub const IN_25_FRAMES: EnvironmentSdfgiFramesToConverge = EnvironmentSdfgiFramesToConverge {
        ord: 4i32
    };
    #[doc(alias = "ENV_SDFGI_CONVERGE_IN_30_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_CONVERGE_IN_30_FRAMES`"]
    pub const IN_30_FRAMES: EnvironmentSdfgiFramesToConverge = EnvironmentSdfgiFramesToConverge {
        ord: 5i32
    };
    #[doc(alias = "ENV_SDFGI_CONVERGE_MAX")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_CONVERGE_MAX`"]
    pub const MAX: EnvironmentSdfgiFramesToConverge = EnvironmentSdfgiFramesToConverge {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for EnvironmentSdfgiFramesToConverge {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentSdfgiFramesToConverge") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentSdfgiFramesToConverge {
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
            Self::IN_5_FRAMES => "IN_5_FRAMES", Self::IN_10_FRAMES => "IN_10_FRAMES", Self::IN_15_FRAMES => "IN_15_FRAMES", Self::IN_20_FRAMES => "IN_20_FRAMES", Self::IN_25_FRAMES => "IN_25_FRAMES", Self::IN_30_FRAMES => "IN_30_FRAMES", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentSdfgiFramesToConverge::IN_5_FRAMES, EnvironmentSdfgiFramesToConverge::IN_10_FRAMES, EnvironmentSdfgiFramesToConverge::IN_15_FRAMES, EnvironmentSdfgiFramesToConverge::IN_20_FRAMES, EnvironmentSdfgiFramesToConverge::IN_25_FRAMES, EnvironmentSdfgiFramesToConverge::IN_30_FRAMES]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentSdfgiFramesToConverge >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("IN_5_FRAMES", "ENV_SDFGI_CONVERGE_IN_5_FRAMES", EnvironmentSdfgiFramesToConverge::IN_5_FRAMES), crate::meta::inspect::EnumConstant::new("IN_10_FRAMES", "ENV_SDFGI_CONVERGE_IN_10_FRAMES", EnvironmentSdfgiFramesToConverge::IN_10_FRAMES), crate::meta::inspect::EnumConstant::new("IN_15_FRAMES", "ENV_SDFGI_CONVERGE_IN_15_FRAMES", EnvironmentSdfgiFramesToConverge::IN_15_FRAMES), crate::meta::inspect::EnumConstant::new("IN_20_FRAMES", "ENV_SDFGI_CONVERGE_IN_20_FRAMES", EnvironmentSdfgiFramesToConverge::IN_20_FRAMES), crate::meta::inspect::EnumConstant::new("IN_25_FRAMES", "ENV_SDFGI_CONVERGE_IN_25_FRAMES", EnvironmentSdfgiFramesToConverge::IN_25_FRAMES), crate::meta::inspect::EnumConstant::new("IN_30_FRAMES", "ENV_SDFGI_CONVERGE_IN_30_FRAMES", EnvironmentSdfgiFramesToConverge::IN_30_FRAMES), crate::meta::inspect::EnumConstant::new("MAX", "ENV_SDFGI_CONVERGE_MAX", EnvironmentSdfgiFramesToConverge::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for EnvironmentSdfgiFramesToConverge {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for EnvironmentSdfgiFramesToConverge {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnvironmentSdfgiFramesToConverge {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentSdfgiFramesToConverge {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `EnvironmentSDFGIFramesToUpdateLight`."]
pub struct EnvironmentSdfgiFramesToUpdateLight {
    ord: i32
}
impl EnvironmentSdfgiFramesToUpdateLight {
    #[doc(alias = "ENV_SDFGI_UPDATE_LIGHT_IN_1_FRAME")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_UPDATE_LIGHT_IN_1_FRAME`"]
    pub const IN_1_FRAME: EnvironmentSdfgiFramesToUpdateLight = EnvironmentSdfgiFramesToUpdateLight {
        ord: 0i32
    };
    #[doc(alias = "ENV_SDFGI_UPDATE_LIGHT_IN_2_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_UPDATE_LIGHT_IN_2_FRAMES`"]
    pub const IN_2_FRAMES: EnvironmentSdfgiFramesToUpdateLight = EnvironmentSdfgiFramesToUpdateLight {
        ord: 1i32
    };
    #[doc(alias = "ENV_SDFGI_UPDATE_LIGHT_IN_4_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_UPDATE_LIGHT_IN_4_FRAMES`"]
    pub const IN_4_FRAMES: EnvironmentSdfgiFramesToUpdateLight = EnvironmentSdfgiFramesToUpdateLight {
        ord: 2i32
    };
    #[doc(alias = "ENV_SDFGI_UPDATE_LIGHT_IN_8_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_UPDATE_LIGHT_IN_8_FRAMES`"]
    pub const IN_8_FRAMES: EnvironmentSdfgiFramesToUpdateLight = EnvironmentSdfgiFramesToUpdateLight {
        ord: 3i32
    };
    #[doc(alias = "ENV_SDFGI_UPDATE_LIGHT_IN_16_FRAMES")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_UPDATE_LIGHT_IN_16_FRAMES`"]
    pub const IN_16_FRAMES: EnvironmentSdfgiFramesToUpdateLight = EnvironmentSdfgiFramesToUpdateLight {
        ord: 4i32
    };
    #[doc(alias = "ENV_SDFGI_UPDATE_LIGHT_MAX")]
    #[doc = "Godot enumerator name: `ENV_SDFGI_UPDATE_LIGHT_MAX`"]
    pub const MAX: EnvironmentSdfgiFramesToUpdateLight = EnvironmentSdfgiFramesToUpdateLight {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for EnvironmentSdfgiFramesToUpdateLight {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EnvironmentSdfgiFramesToUpdateLight") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EnvironmentSdfgiFramesToUpdateLight {
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
            Self::IN_1_FRAME => "IN_1_FRAME", Self::IN_2_FRAMES => "IN_2_FRAMES", Self::IN_4_FRAMES => "IN_4_FRAMES", Self::IN_8_FRAMES => "IN_8_FRAMES", Self::IN_16_FRAMES => "IN_16_FRAMES", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EnvironmentSdfgiFramesToUpdateLight::IN_1_FRAME, EnvironmentSdfgiFramesToUpdateLight::IN_2_FRAMES, EnvironmentSdfgiFramesToUpdateLight::IN_4_FRAMES, EnvironmentSdfgiFramesToUpdateLight::IN_8_FRAMES, EnvironmentSdfgiFramesToUpdateLight::IN_16_FRAMES]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EnvironmentSdfgiFramesToUpdateLight >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("IN_1_FRAME", "ENV_SDFGI_UPDATE_LIGHT_IN_1_FRAME", EnvironmentSdfgiFramesToUpdateLight::IN_1_FRAME), crate::meta::inspect::EnumConstant::new("IN_2_FRAMES", "ENV_SDFGI_UPDATE_LIGHT_IN_2_FRAMES", EnvironmentSdfgiFramesToUpdateLight::IN_2_FRAMES), crate::meta::inspect::EnumConstant::new("IN_4_FRAMES", "ENV_SDFGI_UPDATE_LIGHT_IN_4_FRAMES", EnvironmentSdfgiFramesToUpdateLight::IN_4_FRAMES), crate::meta::inspect::EnumConstant::new("IN_8_FRAMES", "ENV_SDFGI_UPDATE_LIGHT_IN_8_FRAMES", EnvironmentSdfgiFramesToUpdateLight::IN_8_FRAMES), crate::meta::inspect::EnumConstant::new("IN_16_FRAMES", "ENV_SDFGI_UPDATE_LIGHT_IN_16_FRAMES", EnvironmentSdfgiFramesToUpdateLight::IN_16_FRAMES), crate::meta::inspect::EnumConstant::new("MAX", "ENV_SDFGI_UPDATE_LIGHT_MAX", EnvironmentSdfgiFramesToUpdateLight::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for EnvironmentSdfgiFramesToUpdateLight {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for EnvironmentSdfgiFramesToUpdateLight {
    type Via = i32;
    
}
impl crate::meta::ToGodot for EnvironmentSdfgiFramesToUpdateLight {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EnvironmentSdfgiFramesToUpdateLight {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SubSurfaceScatteringQuality {
    ord: i32
}
impl SubSurfaceScatteringQuality {
    #[doc(alias = "SUB_SURFACE_SCATTERING_QUALITY_DISABLED")]
    #[doc = "Godot enumerator name: `SUB_SURFACE_SCATTERING_QUALITY_DISABLED`"]
    pub const DISABLED: SubSurfaceScatteringQuality = SubSurfaceScatteringQuality {
        ord: 0i32
    };
    #[doc(alias = "SUB_SURFACE_SCATTERING_QUALITY_LOW")]
    #[doc = "Godot enumerator name: `SUB_SURFACE_SCATTERING_QUALITY_LOW`"]
    pub const LOW: SubSurfaceScatteringQuality = SubSurfaceScatteringQuality {
        ord: 1i32
    };
    #[doc(alias = "SUB_SURFACE_SCATTERING_QUALITY_MEDIUM")]
    #[doc = "Godot enumerator name: `SUB_SURFACE_SCATTERING_QUALITY_MEDIUM`"]
    pub const MEDIUM: SubSurfaceScatteringQuality = SubSurfaceScatteringQuality {
        ord: 2i32
    };
    #[doc(alias = "SUB_SURFACE_SCATTERING_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `SUB_SURFACE_SCATTERING_QUALITY_HIGH`"]
    pub const HIGH: SubSurfaceScatteringQuality = SubSurfaceScatteringQuality {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for SubSurfaceScatteringQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SubSurfaceScatteringQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SubSurfaceScatteringQuality {
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
            Self::DISABLED => "DISABLED", Self::LOW => "LOW", Self::MEDIUM => "MEDIUM", Self::HIGH => "HIGH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SubSurfaceScatteringQuality::DISABLED, SubSurfaceScatteringQuality::LOW, SubSurfaceScatteringQuality::MEDIUM, SubSurfaceScatteringQuality::HIGH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SubSurfaceScatteringQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "SUB_SURFACE_SCATTERING_QUALITY_DISABLED", SubSurfaceScatteringQuality::DISABLED), crate::meta::inspect::EnumConstant::new("LOW", "SUB_SURFACE_SCATTERING_QUALITY_LOW", SubSurfaceScatteringQuality::LOW), crate::meta::inspect::EnumConstant::new("MEDIUM", "SUB_SURFACE_SCATTERING_QUALITY_MEDIUM", SubSurfaceScatteringQuality::MEDIUM), crate::meta::inspect::EnumConstant::new("HIGH", "SUB_SURFACE_SCATTERING_QUALITY_HIGH", SubSurfaceScatteringQuality::HIGH)]
        }
    }
}
impl crate::meta::GodotConvert for SubSurfaceScatteringQuality {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SubSurfaceScatteringQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SubSurfaceScatteringQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `DOFBokehShape`."]
pub struct DofBokehShape {
    ord: i32
}
impl DofBokehShape {
    #[doc(alias = "DOF_BOKEH_BOX")]
    #[doc = "Godot enumerator name: `DOF_BOKEH_BOX`"]
    pub const BOX: DofBokehShape = DofBokehShape {
        ord: 0i32
    };
    #[doc(alias = "DOF_BOKEH_HEXAGON")]
    #[doc = "Godot enumerator name: `DOF_BOKEH_HEXAGON`"]
    pub const HEXAGON: DofBokehShape = DofBokehShape {
        ord: 1i32
    };
    #[doc(alias = "DOF_BOKEH_CIRCLE")]
    #[doc = "Godot enumerator name: `DOF_BOKEH_CIRCLE`"]
    pub const CIRCLE: DofBokehShape = DofBokehShape {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DofBokehShape {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DofBokehShape") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DofBokehShape {
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
            Self::BOX => "BOX", Self::HEXAGON => "HEXAGON", Self::CIRCLE => "CIRCLE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DofBokehShape::BOX, DofBokehShape::HEXAGON, DofBokehShape::CIRCLE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DofBokehShape >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BOX", "DOF_BOKEH_BOX", DofBokehShape::BOX), crate::meta::inspect::EnumConstant::new("HEXAGON", "DOF_BOKEH_HEXAGON", DofBokehShape::HEXAGON), crate::meta::inspect::EnumConstant::new("CIRCLE", "DOF_BOKEH_CIRCLE", DofBokehShape::CIRCLE)]
        }
    }
}
impl crate::meta::GodotConvert for DofBokehShape {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DofBokehShape {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DofBokehShape {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `DOFBlurQuality`."]
pub struct DofBlurQuality {
    ord: i32
}
impl DofBlurQuality {
    #[doc(alias = "DOF_BLUR_QUALITY_VERY_LOW")]
    #[doc = "Godot enumerator name: `DOF_BLUR_QUALITY_VERY_LOW`"]
    pub const VERY_LOW: DofBlurQuality = DofBlurQuality {
        ord: 0i32
    };
    #[doc(alias = "DOF_BLUR_QUALITY_LOW")]
    #[doc = "Godot enumerator name: `DOF_BLUR_QUALITY_LOW`"]
    pub const LOW: DofBlurQuality = DofBlurQuality {
        ord: 1i32
    };
    #[doc(alias = "DOF_BLUR_QUALITY_MEDIUM")]
    #[doc = "Godot enumerator name: `DOF_BLUR_QUALITY_MEDIUM`"]
    pub const MEDIUM: DofBlurQuality = DofBlurQuality {
        ord: 2i32
    };
    #[doc(alias = "DOF_BLUR_QUALITY_HIGH")]
    #[doc = "Godot enumerator name: `DOF_BLUR_QUALITY_HIGH`"]
    pub const HIGH: DofBlurQuality = DofBlurQuality {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for DofBlurQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DofBlurQuality") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DofBlurQuality {
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
            Self::VERY_LOW => "VERY_LOW", Self::LOW => "LOW", Self::MEDIUM => "MEDIUM", Self::HIGH => "HIGH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DofBlurQuality::VERY_LOW, DofBlurQuality::LOW, DofBlurQuality::MEDIUM, DofBlurQuality::HIGH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DofBlurQuality >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERY_LOW", "DOF_BLUR_QUALITY_VERY_LOW", DofBlurQuality::VERY_LOW), crate::meta::inspect::EnumConstant::new("LOW", "DOF_BLUR_QUALITY_LOW", DofBlurQuality::LOW), crate::meta::inspect::EnumConstant::new("MEDIUM", "DOF_BLUR_QUALITY_MEDIUM", DofBlurQuality::MEDIUM), crate::meta::inspect::EnumConstant::new("HIGH", "DOF_BLUR_QUALITY_HIGH", DofBlurQuality::HIGH)]
        }
    }
}
impl crate::meta::GodotConvert for DofBlurQuality {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DofBlurQuality {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DofBlurQuality {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct InstanceType {
    ord: i32
}
impl InstanceType {
    #[doc(alias = "INSTANCE_NONE")]
    #[doc = "Godot enumerator name: `INSTANCE_NONE`"]
    pub const NONE: InstanceType = InstanceType {
        ord: 0i32
    };
    #[doc(alias = "INSTANCE_MESH")]
    #[doc = "Godot enumerator name: `INSTANCE_MESH`"]
    pub const MESH: InstanceType = InstanceType {
        ord: 1i32
    };
    #[doc(alias = "INSTANCE_MULTIMESH")]
    #[doc = "Godot enumerator name: `INSTANCE_MULTIMESH`"]
    pub const MULTIMESH: InstanceType = InstanceType {
        ord: 2i32
    };
    #[doc(alias = "INSTANCE_PARTICLES")]
    #[doc = "Godot enumerator name: `INSTANCE_PARTICLES`"]
    pub const PARTICLES: InstanceType = InstanceType {
        ord: 3i32
    };
    #[doc(alias = "INSTANCE_PARTICLES_COLLISION")]
    #[doc = "Godot enumerator name: `INSTANCE_PARTICLES_COLLISION`"]
    pub const PARTICLES_COLLISION: InstanceType = InstanceType {
        ord: 4i32
    };
    #[doc(alias = "INSTANCE_LIGHT")]
    #[doc = "Godot enumerator name: `INSTANCE_LIGHT`"]
    pub const LIGHT: InstanceType = InstanceType {
        ord: 5i32
    };
    #[doc(alias = "INSTANCE_REFLECTION_PROBE")]
    #[doc = "Godot enumerator name: `INSTANCE_REFLECTION_PROBE`"]
    pub const REFLECTION_PROBE: InstanceType = InstanceType {
        ord: 6i32
    };
    #[doc(alias = "INSTANCE_DECAL")]
    #[doc = "Godot enumerator name: `INSTANCE_DECAL`"]
    pub const DECAL: InstanceType = InstanceType {
        ord: 7i32
    };
    #[doc(alias = "INSTANCE_VOXEL_GI")]
    #[doc = "Godot enumerator name: `INSTANCE_VOXEL_GI`"]
    pub const VOXEL_GI: InstanceType = InstanceType {
        ord: 8i32
    };
    #[doc(alias = "INSTANCE_LIGHTMAP")]
    #[doc = "Godot enumerator name: `INSTANCE_LIGHTMAP`"]
    pub const LIGHTMAP: InstanceType = InstanceType {
        ord: 9i32
    };
    #[doc(alias = "INSTANCE_OCCLUDER")]
    #[doc = "Godot enumerator name: `INSTANCE_OCCLUDER`"]
    pub const OCCLUDER: InstanceType = InstanceType {
        ord: 10i32
    };
    #[doc(alias = "INSTANCE_VISIBLITY_NOTIFIER")]
    #[doc = "Godot enumerator name: `INSTANCE_VISIBLITY_NOTIFIER`"]
    pub const VISIBLITY_NOTIFIER: InstanceType = InstanceType {
        ord: 11i32
    };
    #[doc(alias = "INSTANCE_FOG_VOLUME")]
    #[doc = "Godot enumerator name: `INSTANCE_FOG_VOLUME`"]
    pub const FOG_VOLUME: InstanceType = InstanceType {
        ord: 12i32
    };
    #[doc(alias = "INSTANCE_MAX")]
    #[doc = "Godot enumerator name: `INSTANCE_MAX`"]
    pub const MAX: InstanceType = InstanceType {
        ord: 13i32
    };
    #[doc(alias = "INSTANCE_GEOMETRY_MASK")]
    #[doc = "Godot enumerator name: `INSTANCE_GEOMETRY_MASK`"]
    pub const GEOMETRY_MASK: InstanceType = InstanceType {
        ord: 14i32
    };
    
}
impl std::fmt::Debug for InstanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("InstanceType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for InstanceType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 => Some(Self {
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
            Self::NONE => "NONE", Self::MESH => "MESH", Self::MULTIMESH => "MULTIMESH", Self::PARTICLES => "PARTICLES", Self::PARTICLES_COLLISION => "PARTICLES_COLLISION", Self::LIGHT => "LIGHT", Self::REFLECTION_PROBE => "REFLECTION_PROBE", Self::DECAL => "DECAL", Self::VOXEL_GI => "VOXEL_GI", Self::LIGHTMAP => "LIGHTMAP", Self::OCCLUDER => "OCCLUDER", Self::VISIBLITY_NOTIFIER => "VISIBLITY_NOTIFIER", Self::FOG_VOLUME => "FOG_VOLUME", Self::MAX => "MAX", Self::GEOMETRY_MASK => "GEOMETRY_MASK", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[InstanceType::NONE, InstanceType::MESH, InstanceType::MULTIMESH, InstanceType::PARTICLES, InstanceType::PARTICLES_COLLISION, InstanceType::LIGHT, InstanceType::REFLECTION_PROBE, InstanceType::DECAL, InstanceType::VOXEL_GI, InstanceType::LIGHTMAP, InstanceType::OCCLUDER, InstanceType::VISIBLITY_NOTIFIER, InstanceType::FOG_VOLUME, InstanceType::MAX, InstanceType::GEOMETRY_MASK]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < InstanceType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "INSTANCE_NONE", InstanceType::NONE), crate::meta::inspect::EnumConstant::new("MESH", "INSTANCE_MESH", InstanceType::MESH), crate::meta::inspect::EnumConstant::new("MULTIMESH", "INSTANCE_MULTIMESH", InstanceType::MULTIMESH), crate::meta::inspect::EnumConstant::new("PARTICLES", "INSTANCE_PARTICLES", InstanceType::PARTICLES), crate::meta::inspect::EnumConstant::new("PARTICLES_COLLISION", "INSTANCE_PARTICLES_COLLISION", InstanceType::PARTICLES_COLLISION), crate::meta::inspect::EnumConstant::new("LIGHT", "INSTANCE_LIGHT", InstanceType::LIGHT), crate::meta::inspect::EnumConstant::new("REFLECTION_PROBE", "INSTANCE_REFLECTION_PROBE", InstanceType::REFLECTION_PROBE), crate::meta::inspect::EnumConstant::new("DECAL", "INSTANCE_DECAL", InstanceType::DECAL), crate::meta::inspect::EnumConstant::new("VOXEL_GI", "INSTANCE_VOXEL_GI", InstanceType::VOXEL_GI), crate::meta::inspect::EnumConstant::new("LIGHTMAP", "INSTANCE_LIGHTMAP", InstanceType::LIGHTMAP), crate::meta::inspect::EnumConstant::new("OCCLUDER", "INSTANCE_OCCLUDER", InstanceType::OCCLUDER), crate::meta::inspect::EnumConstant::new("VISIBLITY_NOTIFIER", "INSTANCE_VISIBLITY_NOTIFIER", InstanceType::VISIBLITY_NOTIFIER), crate::meta::inspect::EnumConstant::new("FOG_VOLUME", "INSTANCE_FOG_VOLUME", InstanceType::FOG_VOLUME), crate::meta::inspect::EnumConstant::new("MAX", "INSTANCE_MAX", InstanceType::MAX), crate::meta::inspect::EnumConstant::new("GEOMETRY_MASK", "INSTANCE_GEOMETRY_MASK", InstanceType::GEOMETRY_MASK)]
        }
    }
}
impl crate::meta::GodotConvert for InstanceType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for InstanceType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for InstanceType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct InstanceFlags {
    ord: i32
}
impl InstanceFlags {
    #[doc(alias = "INSTANCE_FLAG_USE_BAKED_LIGHT")]
    #[doc = "Godot enumerator name: `INSTANCE_FLAG_USE_BAKED_LIGHT`"]
    pub const USE_BAKED_LIGHT: InstanceFlags = InstanceFlags {
        ord: 0i32
    };
    #[doc(alias = "INSTANCE_FLAG_USE_DYNAMIC_GI")]
    #[doc = "Godot enumerator name: `INSTANCE_FLAG_USE_DYNAMIC_GI`"]
    pub const USE_DYNAMIC_GI: InstanceFlags = InstanceFlags {
        ord: 1i32
    };
    #[doc(alias = "INSTANCE_FLAG_DRAW_NEXT_FRAME_IF_VISIBLE")]
    #[doc = "Godot enumerator name: `INSTANCE_FLAG_DRAW_NEXT_FRAME_IF_VISIBLE`"]
    pub const DRAW_NEXT_FRAME_IF_VISIBLE: InstanceFlags = InstanceFlags {
        ord: 2i32
    };
    #[doc(alias = "INSTANCE_FLAG_IGNORE_OCCLUSION_CULLING")]
    #[doc = "Godot enumerator name: `INSTANCE_FLAG_IGNORE_OCCLUSION_CULLING`"]
    pub const IGNORE_OCCLUSION_CULLING: InstanceFlags = InstanceFlags {
        ord: 3i32
    };
    #[doc(alias = "INSTANCE_FLAG_MAX")]
    #[doc = "Godot enumerator name: `INSTANCE_FLAG_MAX`"]
    pub const MAX: InstanceFlags = InstanceFlags {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for InstanceFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("InstanceFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for InstanceFlags {
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
            Self::USE_BAKED_LIGHT => "USE_BAKED_LIGHT", Self::USE_DYNAMIC_GI => "USE_DYNAMIC_GI", Self::DRAW_NEXT_FRAME_IF_VISIBLE => "DRAW_NEXT_FRAME_IF_VISIBLE", Self::IGNORE_OCCLUSION_CULLING => "IGNORE_OCCLUSION_CULLING", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[InstanceFlags::USE_BAKED_LIGHT, InstanceFlags::USE_DYNAMIC_GI, InstanceFlags::DRAW_NEXT_FRAME_IF_VISIBLE, InstanceFlags::IGNORE_OCCLUSION_CULLING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < InstanceFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("USE_BAKED_LIGHT", "INSTANCE_FLAG_USE_BAKED_LIGHT", InstanceFlags::USE_BAKED_LIGHT), crate::meta::inspect::EnumConstant::new("USE_DYNAMIC_GI", "INSTANCE_FLAG_USE_DYNAMIC_GI", InstanceFlags::USE_DYNAMIC_GI), crate::meta::inspect::EnumConstant::new("DRAW_NEXT_FRAME_IF_VISIBLE", "INSTANCE_FLAG_DRAW_NEXT_FRAME_IF_VISIBLE", InstanceFlags::DRAW_NEXT_FRAME_IF_VISIBLE), crate::meta::inspect::EnumConstant::new("IGNORE_OCCLUSION_CULLING", "INSTANCE_FLAG_IGNORE_OCCLUSION_CULLING", InstanceFlags::IGNORE_OCCLUSION_CULLING), crate::meta::inspect::EnumConstant::new("MAX", "INSTANCE_FLAG_MAX", InstanceFlags::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for InstanceFlags {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for InstanceFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for InstanceFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for InstanceFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ShadowCastingSetting {
    ord: i32
}
impl ShadowCastingSetting {
    #[doc(alias = "SHADOW_CASTING_SETTING_OFF")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_OFF`"]
    pub const OFF: ShadowCastingSetting = ShadowCastingSetting {
        ord: 0i32
    };
    #[doc(alias = "SHADOW_CASTING_SETTING_ON")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_ON`"]
    pub const ON: ShadowCastingSetting = ShadowCastingSetting {
        ord: 1i32
    };
    #[doc(alias = "SHADOW_CASTING_SETTING_DOUBLE_SIDED")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_DOUBLE_SIDED`"]
    pub const DOUBLE_SIDED: ShadowCastingSetting = ShadowCastingSetting {
        ord: 2i32
    };
    #[doc(alias = "SHADOW_CASTING_SETTING_SHADOWS_ONLY")]
    #[doc = "Godot enumerator name: `SHADOW_CASTING_SETTING_SHADOWS_ONLY`"]
    pub const SHADOWS_ONLY: ShadowCastingSetting = ShadowCastingSetting {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ShadowCastingSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ShadowCastingSetting") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ShadowCastingSetting {
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
            Self::OFF => "OFF", Self::ON => "ON", Self::DOUBLE_SIDED => "DOUBLE_SIDED", Self::SHADOWS_ONLY => "SHADOWS_ONLY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ShadowCastingSetting::OFF, ShadowCastingSetting::ON, ShadowCastingSetting::DOUBLE_SIDED, ShadowCastingSetting::SHADOWS_ONLY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ShadowCastingSetting >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OFF", "SHADOW_CASTING_SETTING_OFF", ShadowCastingSetting::OFF), crate::meta::inspect::EnumConstant::new("ON", "SHADOW_CASTING_SETTING_ON", ShadowCastingSetting::ON), crate::meta::inspect::EnumConstant::new("DOUBLE_SIDED", "SHADOW_CASTING_SETTING_DOUBLE_SIDED", ShadowCastingSetting::DOUBLE_SIDED), crate::meta::inspect::EnumConstant::new("SHADOWS_ONLY", "SHADOW_CASTING_SETTING_SHADOWS_ONLY", ShadowCastingSetting::SHADOWS_ONLY)]
        }
    }
}
impl crate::meta::GodotConvert for ShadowCastingSetting {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ShadowCastingSetting {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ShadowCastingSetting {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct VisibilityRangeFadeMode {
    ord: i32
}
impl VisibilityRangeFadeMode {
    #[doc(alias = "VISIBILITY_RANGE_FADE_DISABLED")]
    #[doc = "Godot enumerator name: `VISIBILITY_RANGE_FADE_DISABLED`"]
    pub const DISABLED: VisibilityRangeFadeMode = VisibilityRangeFadeMode {
        ord: 0i32
    };
    #[doc(alias = "VISIBILITY_RANGE_FADE_SELF")]
    #[doc = "Godot enumerator name: `VISIBILITY_RANGE_FADE_SELF`"]
    pub const SELF: VisibilityRangeFadeMode = VisibilityRangeFadeMode {
        ord: 1i32
    };
    #[doc(alias = "VISIBILITY_RANGE_FADE_DEPENDENCIES")]
    #[doc = "Godot enumerator name: `VISIBILITY_RANGE_FADE_DEPENDENCIES`"]
    pub const DEPENDENCIES: VisibilityRangeFadeMode = VisibilityRangeFadeMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for VisibilityRangeFadeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("VisibilityRangeFadeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for VisibilityRangeFadeMode {
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
            Self::DISABLED => "DISABLED", Self::SELF => "SELF", Self::DEPENDENCIES => "DEPENDENCIES", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[VisibilityRangeFadeMode::DISABLED, VisibilityRangeFadeMode::SELF, VisibilityRangeFadeMode::DEPENDENCIES]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < VisibilityRangeFadeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "VISIBILITY_RANGE_FADE_DISABLED", VisibilityRangeFadeMode::DISABLED), crate::meta::inspect::EnumConstant::new("SELF", "VISIBILITY_RANGE_FADE_SELF", VisibilityRangeFadeMode::SELF), crate::meta::inspect::EnumConstant::new("DEPENDENCIES", "VISIBILITY_RANGE_FADE_DEPENDENCIES", VisibilityRangeFadeMode::DEPENDENCIES)]
        }
    }
}
impl crate::meta::GodotConvert for VisibilityRangeFadeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for VisibilityRangeFadeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for VisibilityRangeFadeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BakeChannels {
    ord: i32
}
impl BakeChannels {
    #[doc(alias = "BAKE_CHANNEL_ALBEDO_ALPHA")]
    #[doc = "Godot enumerator name: `BAKE_CHANNEL_ALBEDO_ALPHA`"]
    pub const ALBEDO_ALPHA: BakeChannels = BakeChannels {
        ord: 0i32
    };
    #[doc(alias = "BAKE_CHANNEL_NORMAL")]
    #[doc = "Godot enumerator name: `BAKE_CHANNEL_NORMAL`"]
    pub const NORMAL: BakeChannels = BakeChannels {
        ord: 1i32
    };
    #[doc(alias = "BAKE_CHANNEL_ORM")]
    #[doc = "Godot enumerator name: `BAKE_CHANNEL_ORM`"]
    pub const ORM: BakeChannels = BakeChannels {
        ord: 2i32
    };
    #[doc(alias = "BAKE_CHANNEL_EMISSION")]
    #[doc = "Godot enumerator name: `BAKE_CHANNEL_EMISSION`"]
    pub const EMISSION: BakeChannels = BakeChannels {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for BakeChannels {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BakeChannels") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BakeChannels {
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
            Self::ALBEDO_ALPHA => "ALBEDO_ALPHA", Self::NORMAL => "NORMAL", Self::ORM => "ORM", Self::EMISSION => "EMISSION", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BakeChannels::ALBEDO_ALPHA, BakeChannels::NORMAL, BakeChannels::ORM, BakeChannels::EMISSION]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BakeChannels >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ALBEDO_ALPHA", "BAKE_CHANNEL_ALBEDO_ALPHA", BakeChannels::ALBEDO_ALPHA), crate::meta::inspect::EnumConstant::new("NORMAL", "BAKE_CHANNEL_NORMAL", BakeChannels::NORMAL), crate::meta::inspect::EnumConstant::new("ORM", "BAKE_CHANNEL_ORM", BakeChannels::ORM), crate::meta::inspect::EnumConstant::new("EMISSION", "BAKE_CHANNEL_EMISSION", BakeChannels::EMISSION)]
        }
    }
}
impl crate::meta::GodotConvert for BakeChannels {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BakeChannels {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BakeChannels {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CanvasTextureChannel {
    ord: i32
}
impl CanvasTextureChannel {
    #[doc(alias = "CANVAS_TEXTURE_CHANNEL_DIFFUSE")]
    #[doc = "Godot enumerator name: `CANVAS_TEXTURE_CHANNEL_DIFFUSE`"]
    pub const DIFFUSE: CanvasTextureChannel = CanvasTextureChannel {
        ord: 0i32
    };
    #[doc(alias = "CANVAS_TEXTURE_CHANNEL_NORMAL")]
    #[doc = "Godot enumerator name: `CANVAS_TEXTURE_CHANNEL_NORMAL`"]
    pub const NORMAL: CanvasTextureChannel = CanvasTextureChannel {
        ord: 1i32
    };
    #[doc(alias = "CANVAS_TEXTURE_CHANNEL_SPECULAR")]
    #[doc = "Godot enumerator name: `CANVAS_TEXTURE_CHANNEL_SPECULAR`"]
    pub const SPECULAR: CanvasTextureChannel = CanvasTextureChannel {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for CanvasTextureChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CanvasTextureChannel") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CanvasTextureChannel {
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
            Self::DIFFUSE => "DIFFUSE", Self::NORMAL => "NORMAL", Self::SPECULAR => "SPECULAR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CanvasTextureChannel::DIFFUSE, CanvasTextureChannel::NORMAL, CanvasTextureChannel::SPECULAR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CanvasTextureChannel >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DIFFUSE", "CANVAS_TEXTURE_CHANNEL_DIFFUSE", CanvasTextureChannel::DIFFUSE), crate::meta::inspect::EnumConstant::new("NORMAL", "CANVAS_TEXTURE_CHANNEL_NORMAL", CanvasTextureChannel::NORMAL), crate::meta::inspect::EnumConstant::new("SPECULAR", "CANVAS_TEXTURE_CHANNEL_SPECULAR", CanvasTextureChannel::SPECULAR)]
        }
    }
}
impl crate::meta::GodotConvert for CanvasTextureChannel {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CanvasTextureChannel {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CanvasTextureChannel {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct NinePatchAxisMode {
    ord: i32
}
impl NinePatchAxisMode {
    #[doc(alias = "NINE_PATCH_STRETCH")]
    #[doc = "Godot enumerator name: `NINE_PATCH_STRETCH`"]
    pub const STRETCH: NinePatchAxisMode = NinePatchAxisMode {
        ord: 0i32
    };
    #[doc(alias = "NINE_PATCH_TILE")]
    #[doc = "Godot enumerator name: `NINE_PATCH_TILE`"]
    pub const TILE: NinePatchAxisMode = NinePatchAxisMode {
        ord: 1i32
    };
    #[doc(alias = "NINE_PATCH_TILE_FIT")]
    #[doc = "Godot enumerator name: `NINE_PATCH_TILE_FIT`"]
    pub const TILE_FIT: NinePatchAxisMode = NinePatchAxisMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for NinePatchAxisMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("NinePatchAxisMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for NinePatchAxisMode {
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
            Self::STRETCH => "STRETCH", Self::TILE => "TILE", Self::TILE_FIT => "TILE_FIT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[NinePatchAxisMode::STRETCH, NinePatchAxisMode::TILE, NinePatchAxisMode::TILE_FIT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < NinePatchAxisMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STRETCH", "NINE_PATCH_STRETCH", NinePatchAxisMode::STRETCH), crate::meta::inspect::EnumConstant::new("TILE", "NINE_PATCH_TILE", NinePatchAxisMode::TILE), crate::meta::inspect::EnumConstant::new("TILE_FIT", "NINE_PATCH_TILE_FIT", NinePatchAxisMode::TILE_FIT)]
        }
    }
}
impl crate::meta::GodotConvert for NinePatchAxisMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for NinePatchAxisMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for NinePatchAxisMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CanvasItemTextureFilter {
    ord: i32
}
impl CanvasItemTextureFilter {
    #[doc(alias = "CANVAS_ITEM_TEXTURE_FILTER_DEFAULT")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_FILTER_DEFAULT`"]
    pub const DEFAULT: CanvasItemTextureFilter = CanvasItemTextureFilter {
        ord: 0i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_FILTER_NEAREST")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_FILTER_NEAREST`"]
    pub const NEAREST: CanvasItemTextureFilter = CanvasItemTextureFilter {
        ord: 1i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_FILTER_LINEAR")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_FILTER_LINEAR`"]
    pub const LINEAR: CanvasItemTextureFilter = CanvasItemTextureFilter {
        ord: 2i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS`"]
    pub const NEAREST_WITH_MIPMAPS: CanvasItemTextureFilter = CanvasItemTextureFilter {
        ord: 3i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS`"]
    pub const LINEAR_WITH_MIPMAPS: CanvasItemTextureFilter = CanvasItemTextureFilter {
        ord: 4i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC`"]
    pub const NEAREST_WITH_MIPMAPS_ANISOTROPIC: CanvasItemTextureFilter = CanvasItemTextureFilter {
        ord: 5i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC`"]
    pub const LINEAR_WITH_MIPMAPS_ANISOTROPIC: CanvasItemTextureFilter = CanvasItemTextureFilter {
        ord: 6i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_FILTER_MAX")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_FILTER_MAX`"]
    pub const MAX: CanvasItemTextureFilter = CanvasItemTextureFilter {
        ord: 7i32
    };
    
}
impl std::fmt::Debug for CanvasItemTextureFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CanvasItemTextureFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CanvasItemTextureFilter {
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
            Self::DEFAULT => "DEFAULT", Self::NEAREST => "NEAREST", Self::LINEAR => "LINEAR", Self::NEAREST_WITH_MIPMAPS => "NEAREST_WITH_MIPMAPS", Self::LINEAR_WITH_MIPMAPS => "LINEAR_WITH_MIPMAPS", Self::NEAREST_WITH_MIPMAPS_ANISOTROPIC => "NEAREST_WITH_MIPMAPS_ANISOTROPIC", Self::LINEAR_WITH_MIPMAPS_ANISOTROPIC => "LINEAR_WITH_MIPMAPS_ANISOTROPIC", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CanvasItemTextureFilter::DEFAULT, CanvasItemTextureFilter::NEAREST, CanvasItemTextureFilter::LINEAR, CanvasItemTextureFilter::NEAREST_WITH_MIPMAPS, CanvasItemTextureFilter::LINEAR_WITH_MIPMAPS, CanvasItemTextureFilter::NEAREST_WITH_MIPMAPS_ANISOTROPIC, CanvasItemTextureFilter::LINEAR_WITH_MIPMAPS_ANISOTROPIC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CanvasItemTextureFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "CANVAS_ITEM_TEXTURE_FILTER_DEFAULT", CanvasItemTextureFilter::DEFAULT), crate::meta::inspect::EnumConstant::new("NEAREST", "CANVAS_ITEM_TEXTURE_FILTER_NEAREST", CanvasItemTextureFilter::NEAREST), crate::meta::inspect::EnumConstant::new("LINEAR", "CANVAS_ITEM_TEXTURE_FILTER_LINEAR", CanvasItemTextureFilter::LINEAR), crate::meta::inspect::EnumConstant::new("NEAREST_WITH_MIPMAPS", "CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS", CanvasItemTextureFilter::NEAREST_WITH_MIPMAPS), crate::meta::inspect::EnumConstant::new("LINEAR_WITH_MIPMAPS", "CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS", CanvasItemTextureFilter::LINEAR_WITH_MIPMAPS), crate::meta::inspect::EnumConstant::new("NEAREST_WITH_MIPMAPS_ANISOTROPIC", "CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC", CanvasItemTextureFilter::NEAREST_WITH_MIPMAPS_ANISOTROPIC), crate::meta::inspect::EnumConstant::new("LINEAR_WITH_MIPMAPS_ANISOTROPIC", "CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC", CanvasItemTextureFilter::LINEAR_WITH_MIPMAPS_ANISOTROPIC), crate::meta::inspect::EnumConstant::new("MAX", "CANVAS_ITEM_TEXTURE_FILTER_MAX", CanvasItemTextureFilter::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CanvasItemTextureFilter {
    const ENUMERATOR_COUNT: usize = 7usize;
    
}
impl crate::meta::GodotConvert for CanvasItemTextureFilter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CanvasItemTextureFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CanvasItemTextureFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CanvasItemTextureRepeat {
    ord: i32
}
impl CanvasItemTextureRepeat {
    #[doc(alias = "CANVAS_ITEM_TEXTURE_REPEAT_DEFAULT")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_REPEAT_DEFAULT`"]
    pub const DEFAULT: CanvasItemTextureRepeat = CanvasItemTextureRepeat {
        ord: 0i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_REPEAT_DISABLED")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_REPEAT_DISABLED`"]
    pub const DISABLED: CanvasItemTextureRepeat = CanvasItemTextureRepeat {
        ord: 1i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_REPEAT_ENABLED")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_REPEAT_ENABLED`"]
    pub const ENABLED: CanvasItemTextureRepeat = CanvasItemTextureRepeat {
        ord: 2i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_REPEAT_MIRROR")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_REPEAT_MIRROR`"]
    pub const MIRROR: CanvasItemTextureRepeat = CanvasItemTextureRepeat {
        ord: 3i32
    };
    #[doc(alias = "CANVAS_ITEM_TEXTURE_REPEAT_MAX")]
    #[doc = "Godot enumerator name: `CANVAS_ITEM_TEXTURE_REPEAT_MAX`"]
    pub const MAX: CanvasItemTextureRepeat = CanvasItemTextureRepeat {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for CanvasItemTextureRepeat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CanvasItemTextureRepeat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CanvasItemTextureRepeat {
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
            Self::DEFAULT => "DEFAULT", Self::DISABLED => "DISABLED", Self::ENABLED => "ENABLED", Self::MIRROR => "MIRROR", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CanvasItemTextureRepeat::DEFAULT, CanvasItemTextureRepeat::DISABLED, CanvasItemTextureRepeat::ENABLED, CanvasItemTextureRepeat::MIRROR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CanvasItemTextureRepeat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "CANVAS_ITEM_TEXTURE_REPEAT_DEFAULT", CanvasItemTextureRepeat::DEFAULT), crate::meta::inspect::EnumConstant::new("DISABLED", "CANVAS_ITEM_TEXTURE_REPEAT_DISABLED", CanvasItemTextureRepeat::DISABLED), crate::meta::inspect::EnumConstant::new("ENABLED", "CANVAS_ITEM_TEXTURE_REPEAT_ENABLED", CanvasItemTextureRepeat::ENABLED), crate::meta::inspect::EnumConstant::new("MIRROR", "CANVAS_ITEM_TEXTURE_REPEAT_MIRROR", CanvasItemTextureRepeat::MIRROR), crate::meta::inspect::EnumConstant::new("MAX", "CANVAS_ITEM_TEXTURE_REPEAT_MAX", CanvasItemTextureRepeat::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CanvasItemTextureRepeat {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for CanvasItemTextureRepeat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CanvasItemTextureRepeat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CanvasItemTextureRepeat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CanvasGroupMode {
    ord: i32
}
impl CanvasGroupMode {
    #[doc(alias = "CANVAS_GROUP_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `CANVAS_GROUP_MODE_DISABLED`"]
    pub const DISABLED: CanvasGroupMode = CanvasGroupMode {
        ord: 0i32
    };
    #[doc(alias = "CANVAS_GROUP_MODE_CLIP_ONLY")]
    #[doc = "Godot enumerator name: `CANVAS_GROUP_MODE_CLIP_ONLY`"]
    pub const CLIP_ONLY: CanvasGroupMode = CanvasGroupMode {
        ord: 1i32
    };
    #[doc(alias = "CANVAS_GROUP_MODE_CLIP_AND_DRAW")]
    #[doc = "Godot enumerator name: `CANVAS_GROUP_MODE_CLIP_AND_DRAW`"]
    pub const CLIP_AND_DRAW: CanvasGroupMode = CanvasGroupMode {
        ord: 2i32
    };
    #[doc(alias = "CANVAS_GROUP_MODE_TRANSPARENT")]
    #[doc = "Godot enumerator name: `CANVAS_GROUP_MODE_TRANSPARENT`"]
    pub const TRANSPARENT: CanvasGroupMode = CanvasGroupMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for CanvasGroupMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CanvasGroupMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CanvasGroupMode {
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
            Self::DISABLED => "DISABLED", Self::CLIP_ONLY => "CLIP_ONLY", Self::CLIP_AND_DRAW => "CLIP_AND_DRAW", Self::TRANSPARENT => "TRANSPARENT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CanvasGroupMode::DISABLED, CanvasGroupMode::CLIP_ONLY, CanvasGroupMode::CLIP_AND_DRAW, CanvasGroupMode::TRANSPARENT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CanvasGroupMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "CANVAS_GROUP_MODE_DISABLED", CanvasGroupMode::DISABLED), crate::meta::inspect::EnumConstant::new("CLIP_ONLY", "CANVAS_GROUP_MODE_CLIP_ONLY", CanvasGroupMode::CLIP_ONLY), crate::meta::inspect::EnumConstant::new("CLIP_AND_DRAW", "CANVAS_GROUP_MODE_CLIP_AND_DRAW", CanvasGroupMode::CLIP_AND_DRAW), crate::meta::inspect::EnumConstant::new("TRANSPARENT", "CANVAS_GROUP_MODE_TRANSPARENT", CanvasGroupMode::TRANSPARENT)]
        }
    }
}
impl crate::meta::GodotConvert for CanvasGroupMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CanvasGroupMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CanvasGroupMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CanvasLightMode {
    ord: i32
}
impl CanvasLightMode {
    #[doc(alias = "CANVAS_LIGHT_MODE_POINT")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_MODE_POINT`"]
    pub const POINT: CanvasLightMode = CanvasLightMode {
        ord: 0i32
    };
    #[doc(alias = "CANVAS_LIGHT_MODE_DIRECTIONAL")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_MODE_DIRECTIONAL`"]
    pub const DIRECTIONAL: CanvasLightMode = CanvasLightMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for CanvasLightMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CanvasLightMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CanvasLightMode {
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
            Self::POINT => "POINT", Self::DIRECTIONAL => "DIRECTIONAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CanvasLightMode::POINT, CanvasLightMode::DIRECTIONAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CanvasLightMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("POINT", "CANVAS_LIGHT_MODE_POINT", CanvasLightMode::POINT), crate::meta::inspect::EnumConstant::new("DIRECTIONAL", "CANVAS_LIGHT_MODE_DIRECTIONAL", CanvasLightMode::DIRECTIONAL)]
        }
    }
}
impl crate::meta::GodotConvert for CanvasLightMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CanvasLightMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CanvasLightMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CanvasLightBlendMode {
    ord: i32
}
impl CanvasLightBlendMode {
    #[doc(alias = "CANVAS_LIGHT_BLEND_MODE_ADD")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_BLEND_MODE_ADD`"]
    pub const ADD: CanvasLightBlendMode = CanvasLightBlendMode {
        ord: 0i32
    };
    #[doc(alias = "CANVAS_LIGHT_BLEND_MODE_SUB")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_BLEND_MODE_SUB`"]
    pub const SUB: CanvasLightBlendMode = CanvasLightBlendMode {
        ord: 1i32
    };
    #[doc(alias = "CANVAS_LIGHT_BLEND_MODE_MIX")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_BLEND_MODE_MIX`"]
    pub const MIX: CanvasLightBlendMode = CanvasLightBlendMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for CanvasLightBlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CanvasLightBlendMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CanvasLightBlendMode {
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
            Self::ADD => "ADD", Self::SUB => "SUB", Self::MIX => "MIX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CanvasLightBlendMode::ADD, CanvasLightBlendMode::SUB, CanvasLightBlendMode::MIX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CanvasLightBlendMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("ADD", "CANVAS_LIGHT_BLEND_MODE_ADD", CanvasLightBlendMode::ADD), crate::meta::inspect::EnumConstant::new("SUB", "CANVAS_LIGHT_BLEND_MODE_SUB", CanvasLightBlendMode::SUB), crate::meta::inspect::EnumConstant::new("MIX", "CANVAS_LIGHT_BLEND_MODE_MIX", CanvasLightBlendMode::MIX)]
        }
    }
}
impl crate::meta::GodotConvert for CanvasLightBlendMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CanvasLightBlendMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CanvasLightBlendMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CanvasLightShadowFilter {
    ord: i32
}
impl CanvasLightShadowFilter {
    #[doc(alias = "CANVAS_LIGHT_FILTER_NONE")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_FILTER_NONE`"]
    pub const NONE: CanvasLightShadowFilter = CanvasLightShadowFilter {
        ord: 0i32
    };
    #[doc(alias = "CANVAS_LIGHT_FILTER_PCF5")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_FILTER_PCF5`"]
    pub const PCF5: CanvasLightShadowFilter = CanvasLightShadowFilter {
        ord: 1i32
    };
    #[doc(alias = "CANVAS_LIGHT_FILTER_PCF13")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_FILTER_PCF13`"]
    pub const PCF13: CanvasLightShadowFilter = CanvasLightShadowFilter {
        ord: 2i32
    };
    #[doc(alias = "CANVAS_LIGHT_FILTER_MAX")]
    #[doc = "Godot enumerator name: `CANVAS_LIGHT_FILTER_MAX`"]
    pub const MAX: CanvasLightShadowFilter = CanvasLightShadowFilter {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for CanvasLightShadowFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CanvasLightShadowFilter") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CanvasLightShadowFilter {
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
            Self::NONE => "NONE", Self::PCF5 => "PCF5", Self::PCF13 => "PCF13", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CanvasLightShadowFilter::NONE, CanvasLightShadowFilter::PCF5, CanvasLightShadowFilter::PCF13]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CanvasLightShadowFilter >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "CANVAS_LIGHT_FILTER_NONE", CanvasLightShadowFilter::NONE), crate::meta::inspect::EnumConstant::new("PCF5", "CANVAS_LIGHT_FILTER_PCF5", CanvasLightShadowFilter::PCF5), crate::meta::inspect::EnumConstant::new("PCF13", "CANVAS_LIGHT_FILTER_PCF13", CanvasLightShadowFilter::PCF13), crate::meta::inspect::EnumConstant::new("MAX", "CANVAS_LIGHT_FILTER_MAX", CanvasLightShadowFilter::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CanvasLightShadowFilter {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for CanvasLightShadowFilter {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CanvasLightShadowFilter {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CanvasLightShadowFilter {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CanvasOccluderPolygonCullMode {
    ord: i32
}
impl CanvasOccluderPolygonCullMode {
    #[doc(alias = "CANVAS_OCCLUDER_POLYGON_CULL_DISABLED")]
    #[doc = "Godot enumerator name: `CANVAS_OCCLUDER_POLYGON_CULL_DISABLED`"]
    pub const DISABLED: CanvasOccluderPolygonCullMode = CanvasOccluderPolygonCullMode {
        ord: 0i32
    };
    #[doc(alias = "CANVAS_OCCLUDER_POLYGON_CULL_CLOCKWISE")]
    #[doc = "Godot enumerator name: `CANVAS_OCCLUDER_POLYGON_CULL_CLOCKWISE`"]
    pub const CLOCKWISE: CanvasOccluderPolygonCullMode = CanvasOccluderPolygonCullMode {
        ord: 1i32
    };
    #[doc(alias = "CANVAS_OCCLUDER_POLYGON_CULL_COUNTER_CLOCKWISE")]
    #[doc = "Godot enumerator name: `CANVAS_OCCLUDER_POLYGON_CULL_COUNTER_CLOCKWISE`"]
    pub const COUNTER_CLOCKWISE: CanvasOccluderPolygonCullMode = CanvasOccluderPolygonCullMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for CanvasOccluderPolygonCullMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CanvasOccluderPolygonCullMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CanvasOccluderPolygonCullMode {
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
            Self::DISABLED => "DISABLED", Self::CLOCKWISE => "CLOCKWISE", Self::COUNTER_CLOCKWISE => "COUNTER_CLOCKWISE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CanvasOccluderPolygonCullMode::DISABLED, CanvasOccluderPolygonCullMode::CLOCKWISE, CanvasOccluderPolygonCullMode::COUNTER_CLOCKWISE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CanvasOccluderPolygonCullMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "CANVAS_OCCLUDER_POLYGON_CULL_DISABLED", CanvasOccluderPolygonCullMode::DISABLED), crate::meta::inspect::EnumConstant::new("CLOCKWISE", "CANVAS_OCCLUDER_POLYGON_CULL_CLOCKWISE", CanvasOccluderPolygonCullMode::CLOCKWISE), crate::meta::inspect::EnumConstant::new("COUNTER_CLOCKWISE", "CANVAS_OCCLUDER_POLYGON_CULL_COUNTER_CLOCKWISE", CanvasOccluderPolygonCullMode::COUNTER_CLOCKWISE)]
        }
    }
}
impl crate::meta::GodotConvert for CanvasOccluderPolygonCullMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CanvasOccluderPolygonCullMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CanvasOccluderPolygonCullMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GlobalShaderParameterType {
    ord: i32
}
impl GlobalShaderParameterType {
    #[doc(alias = "GLOBAL_VAR_TYPE_BOOL")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_BOOL`"]
    pub const BOOL: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 0i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_BVEC2")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_BVEC2`"]
    pub const BVEC2: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 1i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_BVEC3")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_BVEC3`"]
    pub const BVEC3: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 2i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_BVEC4")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_BVEC4`"]
    pub const BVEC4: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 3i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_INT")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_INT`"]
    pub const INT: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 4i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_IVEC2")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_IVEC2`"]
    pub const IVEC2: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 5i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_IVEC3")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_IVEC3`"]
    pub const IVEC3: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 6i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_IVEC4")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_IVEC4`"]
    pub const IVEC4: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 7i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_RECT2I")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_RECT2I`"]
    pub const RECT2I: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 8i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_UINT")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_UINT`"]
    pub const UINT: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 9i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_UVEC2")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_UVEC2`"]
    pub const UVEC2: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 10i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_UVEC3")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_UVEC3`"]
    pub const UVEC3: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 11i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_UVEC4")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_UVEC4`"]
    pub const UVEC4: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 12i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_FLOAT")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_FLOAT`"]
    pub const FLOAT: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 13i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_VEC2")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_VEC2`"]
    pub const VEC2: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 14i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_VEC3")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_VEC3`"]
    pub const VEC3: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 15i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_VEC4")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_VEC4`"]
    pub const VEC4: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 16i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_COLOR")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_COLOR`"]
    pub const COLOR: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 17i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_RECT2")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_RECT2`"]
    pub const RECT2: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 18i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_MAT2")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_MAT2`"]
    pub const MAT2: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 19i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_MAT3")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_MAT3`"]
    pub const MAT3: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 20i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_MAT4")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_MAT4`"]
    pub const MAT4: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 21i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_TRANSFORM_2D")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_TRANSFORM_2D`"]
    pub const TRANSFORM_2D: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 22i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_TRANSFORM")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_TRANSFORM`"]
    pub const TRANSFORM: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 23i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_SAMPLER2D")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_SAMPLER2D`"]
    pub const SAMPLER2D: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 24i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_SAMPLER2DARRAY")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_SAMPLER2DARRAY`"]
    pub const SAMPLER2DARRAY: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 25i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_SAMPLER3D")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_SAMPLER3D`"]
    pub const SAMPLER3D: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 26i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_SAMPLERCUBE")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_SAMPLERCUBE`"]
    pub const SAMPLERCUBE: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 27i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_SAMPLEREXT")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_SAMPLEREXT`"]
    pub const SAMPLEREXT: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 28i32
    };
    #[doc(alias = "GLOBAL_VAR_TYPE_MAX")]
    #[doc = "Godot enumerator name: `GLOBAL_VAR_TYPE_MAX`"]
    pub const MAX: GlobalShaderParameterType = GlobalShaderParameterType {
        ord: 29i32
    };
    
}
impl std::fmt::Debug for GlobalShaderParameterType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GlobalShaderParameterType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GlobalShaderParameterType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 => Some(Self {
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
            Self::BOOL => "BOOL", Self::BVEC2 => "BVEC2", Self::BVEC3 => "BVEC3", Self::BVEC4 => "BVEC4", Self::INT => "INT", Self::IVEC2 => "IVEC2", Self::IVEC3 => "IVEC3", Self::IVEC4 => "IVEC4", Self::RECT2I => "RECT2I", Self::UINT => "UINT", Self::UVEC2 => "UVEC2", Self::UVEC3 => "UVEC3", Self::UVEC4 => "UVEC4", Self::FLOAT => "FLOAT", Self::VEC2 => "VEC2", Self::VEC3 => "VEC3", Self::VEC4 => "VEC4", Self::COLOR => "COLOR", Self::RECT2 => "RECT2", Self::MAT2 => "MAT2", Self::MAT3 => "MAT3", Self::MAT4 => "MAT4", Self::TRANSFORM_2D => "TRANSFORM_2D", Self::TRANSFORM => "TRANSFORM", Self::SAMPLER2D => "SAMPLER2D", Self::SAMPLER2DARRAY => "SAMPLER2DARRAY", Self::SAMPLER3D => "SAMPLER3D", Self::SAMPLERCUBE => "SAMPLERCUBE", Self::SAMPLEREXT => "SAMPLEREXT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[GlobalShaderParameterType::BOOL, GlobalShaderParameterType::BVEC2, GlobalShaderParameterType::BVEC3, GlobalShaderParameterType::BVEC4, GlobalShaderParameterType::INT, GlobalShaderParameterType::IVEC2, GlobalShaderParameterType::IVEC3, GlobalShaderParameterType::IVEC4, GlobalShaderParameterType::RECT2I, GlobalShaderParameterType::UINT, GlobalShaderParameterType::UVEC2, GlobalShaderParameterType::UVEC3, GlobalShaderParameterType::UVEC4, GlobalShaderParameterType::FLOAT, GlobalShaderParameterType::VEC2, GlobalShaderParameterType::VEC3, GlobalShaderParameterType::VEC4, GlobalShaderParameterType::COLOR, GlobalShaderParameterType::RECT2, GlobalShaderParameterType::MAT2, GlobalShaderParameterType::MAT3, GlobalShaderParameterType::MAT4, GlobalShaderParameterType::TRANSFORM_2D, GlobalShaderParameterType::TRANSFORM, GlobalShaderParameterType::SAMPLER2D, GlobalShaderParameterType::SAMPLER2DARRAY, GlobalShaderParameterType::SAMPLER3D, GlobalShaderParameterType::SAMPLERCUBE, GlobalShaderParameterType::SAMPLEREXT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GlobalShaderParameterType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BOOL", "GLOBAL_VAR_TYPE_BOOL", GlobalShaderParameterType::BOOL), crate::meta::inspect::EnumConstant::new("BVEC2", "GLOBAL_VAR_TYPE_BVEC2", GlobalShaderParameterType::BVEC2), crate::meta::inspect::EnumConstant::new("BVEC3", "GLOBAL_VAR_TYPE_BVEC3", GlobalShaderParameterType::BVEC3), crate::meta::inspect::EnumConstant::new("BVEC4", "GLOBAL_VAR_TYPE_BVEC4", GlobalShaderParameterType::BVEC4), crate::meta::inspect::EnumConstant::new("INT", "GLOBAL_VAR_TYPE_INT", GlobalShaderParameterType::INT), crate::meta::inspect::EnumConstant::new("IVEC2", "GLOBAL_VAR_TYPE_IVEC2", GlobalShaderParameterType::IVEC2), crate::meta::inspect::EnumConstant::new("IVEC3", "GLOBAL_VAR_TYPE_IVEC3", GlobalShaderParameterType::IVEC3), crate::meta::inspect::EnumConstant::new("IVEC4", "GLOBAL_VAR_TYPE_IVEC4", GlobalShaderParameterType::IVEC4), crate::meta::inspect::EnumConstant::new("RECT2I", "GLOBAL_VAR_TYPE_RECT2I", GlobalShaderParameterType::RECT2I), crate::meta::inspect::EnumConstant::new("UINT", "GLOBAL_VAR_TYPE_UINT", GlobalShaderParameterType::UINT), crate::meta::inspect::EnumConstant::new("UVEC2", "GLOBAL_VAR_TYPE_UVEC2", GlobalShaderParameterType::UVEC2), crate::meta::inspect::EnumConstant::new("UVEC3", "GLOBAL_VAR_TYPE_UVEC3", GlobalShaderParameterType::UVEC3), crate::meta::inspect::EnumConstant::new("UVEC4", "GLOBAL_VAR_TYPE_UVEC4", GlobalShaderParameterType::UVEC4), crate::meta::inspect::EnumConstant::new("FLOAT", "GLOBAL_VAR_TYPE_FLOAT", GlobalShaderParameterType::FLOAT), crate::meta::inspect::EnumConstant::new("VEC2", "GLOBAL_VAR_TYPE_VEC2", GlobalShaderParameterType::VEC2), crate::meta::inspect::EnumConstant::new("VEC3", "GLOBAL_VAR_TYPE_VEC3", GlobalShaderParameterType::VEC3), crate::meta::inspect::EnumConstant::new("VEC4", "GLOBAL_VAR_TYPE_VEC4", GlobalShaderParameterType::VEC4), crate::meta::inspect::EnumConstant::new("COLOR", "GLOBAL_VAR_TYPE_COLOR", GlobalShaderParameterType::COLOR), crate::meta::inspect::EnumConstant::new("RECT2", "GLOBAL_VAR_TYPE_RECT2", GlobalShaderParameterType::RECT2), crate::meta::inspect::EnumConstant::new("MAT2", "GLOBAL_VAR_TYPE_MAT2", GlobalShaderParameterType::MAT2), crate::meta::inspect::EnumConstant::new("MAT3", "GLOBAL_VAR_TYPE_MAT3", GlobalShaderParameterType::MAT3), crate::meta::inspect::EnumConstant::new("MAT4", "GLOBAL_VAR_TYPE_MAT4", GlobalShaderParameterType::MAT4), crate::meta::inspect::EnumConstant::new("TRANSFORM_2D", "GLOBAL_VAR_TYPE_TRANSFORM_2D", GlobalShaderParameterType::TRANSFORM_2D), crate::meta::inspect::EnumConstant::new("TRANSFORM", "GLOBAL_VAR_TYPE_TRANSFORM", GlobalShaderParameterType::TRANSFORM), crate::meta::inspect::EnumConstant::new("SAMPLER2D", "GLOBAL_VAR_TYPE_SAMPLER2D", GlobalShaderParameterType::SAMPLER2D), crate::meta::inspect::EnumConstant::new("SAMPLER2DARRAY", "GLOBAL_VAR_TYPE_SAMPLER2DARRAY", GlobalShaderParameterType::SAMPLER2DARRAY), crate::meta::inspect::EnumConstant::new("SAMPLER3D", "GLOBAL_VAR_TYPE_SAMPLER3D", GlobalShaderParameterType::SAMPLER3D), crate::meta::inspect::EnumConstant::new("SAMPLERCUBE", "GLOBAL_VAR_TYPE_SAMPLERCUBE", GlobalShaderParameterType::SAMPLERCUBE), crate::meta::inspect::EnumConstant::new("SAMPLEREXT", "GLOBAL_VAR_TYPE_SAMPLEREXT", GlobalShaderParameterType::SAMPLEREXT), crate::meta::inspect::EnumConstant::new("MAX", "GLOBAL_VAR_TYPE_MAX", GlobalShaderParameterType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for GlobalShaderParameterType {
    const ENUMERATOR_COUNT: usize = 29usize;
    
}
impl crate::meta::GodotConvert for GlobalShaderParameterType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for GlobalShaderParameterType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GlobalShaderParameterType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RenderingInfo {
    ord: i32
}
impl RenderingInfo {
    #[doc(alias = "RENDERING_INFO_TOTAL_OBJECTS_IN_FRAME")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_TOTAL_OBJECTS_IN_FRAME`"]
    pub const TOTAL_OBJECTS_IN_FRAME: RenderingInfo = RenderingInfo {
        ord: 0i32
    };
    #[doc(alias = "RENDERING_INFO_TOTAL_PRIMITIVES_IN_FRAME")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_TOTAL_PRIMITIVES_IN_FRAME`"]
    pub const TOTAL_PRIMITIVES_IN_FRAME: RenderingInfo = RenderingInfo {
        ord: 1i32
    };
    #[doc(alias = "RENDERING_INFO_TOTAL_DRAW_CALLS_IN_FRAME")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_TOTAL_DRAW_CALLS_IN_FRAME`"]
    pub const TOTAL_DRAW_CALLS_IN_FRAME: RenderingInfo = RenderingInfo {
        ord: 2i32
    };
    #[doc(alias = "RENDERING_INFO_TEXTURE_MEM_USED")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_TEXTURE_MEM_USED`"]
    pub const TEXTURE_MEM_USED: RenderingInfo = RenderingInfo {
        ord: 3i32
    };
    #[doc(alias = "RENDERING_INFO_BUFFER_MEM_USED")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_BUFFER_MEM_USED`"]
    pub const BUFFER_MEM_USED: RenderingInfo = RenderingInfo {
        ord: 4i32
    };
    #[doc(alias = "RENDERING_INFO_VIDEO_MEM_USED")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_VIDEO_MEM_USED`"]
    pub const VIDEO_MEM_USED: RenderingInfo = RenderingInfo {
        ord: 5i32
    };
    #[doc(alias = "RENDERING_INFO_PIPELINE_COMPILATIONS_CANVAS")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_PIPELINE_COMPILATIONS_CANVAS`"]
    pub const PIPELINE_COMPILATIONS_CANVAS: RenderingInfo = RenderingInfo {
        ord: 6i32
    };
    #[doc(alias = "RENDERING_INFO_PIPELINE_COMPILATIONS_MESH")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_PIPELINE_COMPILATIONS_MESH`"]
    pub const PIPELINE_COMPILATIONS_MESH: RenderingInfo = RenderingInfo {
        ord: 7i32
    };
    #[doc(alias = "RENDERING_INFO_PIPELINE_COMPILATIONS_SURFACE")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_PIPELINE_COMPILATIONS_SURFACE`"]
    pub const PIPELINE_COMPILATIONS_SURFACE: RenderingInfo = RenderingInfo {
        ord: 8i32
    };
    #[doc(alias = "RENDERING_INFO_PIPELINE_COMPILATIONS_DRAW")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_PIPELINE_COMPILATIONS_DRAW`"]
    pub const PIPELINE_COMPILATIONS_DRAW: RenderingInfo = RenderingInfo {
        ord: 9i32
    };
    #[doc(alias = "RENDERING_INFO_PIPELINE_COMPILATIONS_SPECIALIZATION")]
    #[doc = "Godot enumerator name: `RENDERING_INFO_PIPELINE_COMPILATIONS_SPECIALIZATION`"]
    pub const PIPELINE_COMPILATIONS_SPECIALIZATION: RenderingInfo = RenderingInfo {
        ord: 10i32
    };
    
}
impl std::fmt::Debug for RenderingInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RenderingInfo") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RenderingInfo {
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
            Self::TOTAL_OBJECTS_IN_FRAME => "TOTAL_OBJECTS_IN_FRAME", Self::TOTAL_PRIMITIVES_IN_FRAME => "TOTAL_PRIMITIVES_IN_FRAME", Self::TOTAL_DRAW_CALLS_IN_FRAME => "TOTAL_DRAW_CALLS_IN_FRAME", Self::TEXTURE_MEM_USED => "TEXTURE_MEM_USED", Self::BUFFER_MEM_USED => "BUFFER_MEM_USED", Self::VIDEO_MEM_USED => "VIDEO_MEM_USED", Self::PIPELINE_COMPILATIONS_CANVAS => "PIPELINE_COMPILATIONS_CANVAS", Self::PIPELINE_COMPILATIONS_MESH => "PIPELINE_COMPILATIONS_MESH", Self::PIPELINE_COMPILATIONS_SURFACE => "PIPELINE_COMPILATIONS_SURFACE", Self::PIPELINE_COMPILATIONS_DRAW => "PIPELINE_COMPILATIONS_DRAW", Self::PIPELINE_COMPILATIONS_SPECIALIZATION => "PIPELINE_COMPILATIONS_SPECIALIZATION", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[RenderingInfo::TOTAL_OBJECTS_IN_FRAME, RenderingInfo::TOTAL_PRIMITIVES_IN_FRAME, RenderingInfo::TOTAL_DRAW_CALLS_IN_FRAME, RenderingInfo::TEXTURE_MEM_USED, RenderingInfo::BUFFER_MEM_USED, RenderingInfo::VIDEO_MEM_USED, RenderingInfo::PIPELINE_COMPILATIONS_CANVAS, RenderingInfo::PIPELINE_COMPILATIONS_MESH, RenderingInfo::PIPELINE_COMPILATIONS_SURFACE, RenderingInfo::PIPELINE_COMPILATIONS_DRAW, RenderingInfo::PIPELINE_COMPILATIONS_SPECIALIZATION]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < RenderingInfo >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TOTAL_OBJECTS_IN_FRAME", "RENDERING_INFO_TOTAL_OBJECTS_IN_FRAME", RenderingInfo::TOTAL_OBJECTS_IN_FRAME), crate::meta::inspect::EnumConstant::new("TOTAL_PRIMITIVES_IN_FRAME", "RENDERING_INFO_TOTAL_PRIMITIVES_IN_FRAME", RenderingInfo::TOTAL_PRIMITIVES_IN_FRAME), crate::meta::inspect::EnumConstant::new("TOTAL_DRAW_CALLS_IN_FRAME", "RENDERING_INFO_TOTAL_DRAW_CALLS_IN_FRAME", RenderingInfo::TOTAL_DRAW_CALLS_IN_FRAME), crate::meta::inspect::EnumConstant::new("TEXTURE_MEM_USED", "RENDERING_INFO_TEXTURE_MEM_USED", RenderingInfo::TEXTURE_MEM_USED), crate::meta::inspect::EnumConstant::new("BUFFER_MEM_USED", "RENDERING_INFO_BUFFER_MEM_USED", RenderingInfo::BUFFER_MEM_USED), crate::meta::inspect::EnumConstant::new("VIDEO_MEM_USED", "RENDERING_INFO_VIDEO_MEM_USED", RenderingInfo::VIDEO_MEM_USED), crate::meta::inspect::EnumConstant::new("PIPELINE_COMPILATIONS_CANVAS", "RENDERING_INFO_PIPELINE_COMPILATIONS_CANVAS", RenderingInfo::PIPELINE_COMPILATIONS_CANVAS), crate::meta::inspect::EnumConstant::new("PIPELINE_COMPILATIONS_MESH", "RENDERING_INFO_PIPELINE_COMPILATIONS_MESH", RenderingInfo::PIPELINE_COMPILATIONS_MESH), crate::meta::inspect::EnumConstant::new("PIPELINE_COMPILATIONS_SURFACE", "RENDERING_INFO_PIPELINE_COMPILATIONS_SURFACE", RenderingInfo::PIPELINE_COMPILATIONS_SURFACE), crate::meta::inspect::EnumConstant::new("PIPELINE_COMPILATIONS_DRAW", "RENDERING_INFO_PIPELINE_COMPILATIONS_DRAW", RenderingInfo::PIPELINE_COMPILATIONS_DRAW), crate::meta::inspect::EnumConstant::new("PIPELINE_COMPILATIONS_SPECIALIZATION", "RENDERING_INFO_PIPELINE_COMPILATIONS_SPECIALIZATION", RenderingInfo::PIPELINE_COMPILATIONS_SPECIALIZATION)]
        }
    }
}
impl crate::meta::GodotConvert for RenderingInfo {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RenderingInfo {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RenderingInfo {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PipelineSource {
    ord: i32
}
impl PipelineSource {
    #[doc(alias = "PIPELINE_SOURCE_CANVAS")]
    #[doc = "Godot enumerator name: `PIPELINE_SOURCE_CANVAS`"]
    pub const CANVAS: PipelineSource = PipelineSource {
        ord: 0i32
    };
    #[doc(alias = "PIPELINE_SOURCE_MESH")]
    #[doc = "Godot enumerator name: `PIPELINE_SOURCE_MESH`"]
    pub const MESH: PipelineSource = PipelineSource {
        ord: 1i32
    };
    #[doc(alias = "PIPELINE_SOURCE_SURFACE")]
    #[doc = "Godot enumerator name: `PIPELINE_SOURCE_SURFACE`"]
    pub const SURFACE: PipelineSource = PipelineSource {
        ord: 2i32
    };
    #[doc(alias = "PIPELINE_SOURCE_DRAW")]
    #[doc = "Godot enumerator name: `PIPELINE_SOURCE_DRAW`"]
    pub const DRAW: PipelineSource = PipelineSource {
        ord: 3i32
    };
    #[doc(alias = "PIPELINE_SOURCE_SPECIALIZATION")]
    #[doc = "Godot enumerator name: `PIPELINE_SOURCE_SPECIALIZATION`"]
    pub const SPECIALIZATION: PipelineSource = PipelineSource {
        ord: 4i32
    };
    #[doc(alias = "PIPELINE_SOURCE_MAX")]
    #[doc = "Godot enumerator name: `PIPELINE_SOURCE_MAX`"]
    pub const MAX: PipelineSource = PipelineSource {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for PipelineSource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PipelineSource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PipelineSource {
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
            Self::CANVAS => "CANVAS", Self::MESH => "MESH", Self::SURFACE => "SURFACE", Self::DRAW => "DRAW", Self::SPECIALIZATION => "SPECIALIZATION", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PipelineSource::CANVAS, PipelineSource::MESH, PipelineSource::SURFACE, PipelineSource::DRAW, PipelineSource::SPECIALIZATION]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PipelineSource >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CANVAS", "PIPELINE_SOURCE_CANVAS", PipelineSource::CANVAS), crate::meta::inspect::EnumConstant::new("MESH", "PIPELINE_SOURCE_MESH", PipelineSource::MESH), crate::meta::inspect::EnumConstant::new("SURFACE", "PIPELINE_SOURCE_SURFACE", PipelineSource::SURFACE), crate::meta::inspect::EnumConstant::new("DRAW", "PIPELINE_SOURCE_DRAW", PipelineSource::DRAW), crate::meta::inspect::EnumConstant::new("SPECIALIZATION", "PIPELINE_SOURCE_SPECIALIZATION", PipelineSource::SPECIALIZATION), crate::meta::inspect::EnumConstant::new("MAX", "PIPELINE_SOURCE_MAX", PipelineSource::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for PipelineSource {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for PipelineSource {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PipelineSource {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PipelineSource {
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
    #[doc(alias = "FEATURE_SHADERS")]
    #[doc = "Godot enumerator name: `FEATURE_SHADERS`"]
    pub const SHADERS: Features = Features {
        ord: 0i32
    };
    #[doc(alias = "FEATURE_MULTITHREADED")]
    #[doc = "Godot enumerator name: `FEATURE_MULTITHREADED`"]
    pub const MULTITHREADED: Features = Features {
        ord: 1i32
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
            Self::SHADERS => "SHADERS", Self::MULTITHREADED => "MULTITHREADED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Features::SHADERS, Features::MULTITHREADED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Features >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SHADERS", "FEATURE_SHADERS", Features::SHADERS), crate::meta::inspect::EnumConstant::new("MULTITHREADED", "FEATURE_MULTITHREADED", Features::MULTITHREADED)]
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::RenderingServer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`RenderingServer`][crate::classes::RenderingServer] class."]
    pub struct SignalsOfRenderingServer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfRenderingServer < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn frame_pre_draw(&mut self) -> SigFramePreDraw < 'c, C > {
            SigFramePreDraw {
                typed: TypedSignal::extract(&mut self.__internal_obj, "frame_pre_draw")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn frame_post_draw(&mut self) -> SigFramePostDraw < 'c, C > {
            SigFramePostDraw {
                typed: TypedSignal::extract(&mut self.__internal_obj, "frame_post_draw")
            }
        }
    }
    type TypedSigFramePreDraw < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFramePreDraw < 'c, C: WithSignals > {
        typed: TypedSigFramePreDraw < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFramePreDraw < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFramePreDraw < 'c, C > {
        type Target = TypedSigFramePreDraw < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFramePreDraw < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFramePostDraw < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFramePostDraw < 'c, C: WithSignals > {
        typed: TypedSigFramePostDraw < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFramePostDraw < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFramePostDraw < 'c, C > {
        type Target = TypedSigFramePostDraw < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFramePostDraw < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for RenderingServer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfRenderingServer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfRenderingServer < 'c, C > {
        type Target = < < RenderingServer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = RenderingServer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfRenderingServer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = RenderingServer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}