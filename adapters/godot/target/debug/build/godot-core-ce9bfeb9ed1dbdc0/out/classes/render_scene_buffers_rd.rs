#![doc = "Sidecar module for class [`RenderSceneBuffersRd`][crate::classes::RenderSceneBuffersRd].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RenderSceneBuffersRD` enums](https://docs.godotengine.org/en/stable/classes/class_renderscenebuffersrd.html#enumerations).\n\n"]
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
    #[doc = "Godot class `RenderSceneBuffersRD.`\n\nInherits [`RenderSceneBuffers`][crate::classes::RenderSceneBuffers].\n\nRelated symbols:\n\n* [`render_scene_buffers_rd`][crate::classes::render_scene_buffers_rd]: sidecar module with related enum/flag types\n* [`IRenderSceneBuffersRd`][crate::classes::IRenderSceneBuffersRd]: virtual methods\n\n\nSee also [Godot docs for `RenderSceneBuffersRD`](https://docs.godotengine.org/en/stable/classes/class_renderscenebuffersrd.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`RenderSceneBuffersRd::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct RenderSceneBuffersRd {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`RenderSceneBuffersRd`][crate::classes::RenderSceneBuffersRd].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IRenderSceneBuffers`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `RenderSceneBuffersRD` methods](https://docs.godotengine.org/en/stable/classes/class_renderscenebuffersrd.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IRenderSceneBuffersRd: crate::obj::GodotClass < Base = RenderSceneBuffersRd > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl RenderSceneBuffersRd {
        pub fn has_texture(&self, context: impl AsArg < StringName >, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (context.into_arg(), name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7319usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "has_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_texture(&mut self, context: impl AsArg < StringName >, name: impl AsArg < StringName >, data_format: crate::classes::rendering_device::DataFormat, usage_bits: u32, texture_samples: crate::classes::rendering_device::TextureSamples, size: Vector2i, layers: u32, mipmaps: u32, unique: bool, discardable: bool,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, crate::classes::rendering_device::DataFormat, u32, crate::classes::rendering_device::TextureSamples, Vector2i, u32, u32, bool, bool,);
            let args = (context.into_arg(), name.into_arg(), data_format, usage_bits, texture_samples, size, layers, mipmaps, unique, discardable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7320usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "create_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_texture_from_format(&mut self, context: impl AsArg < StringName >, name: impl AsArg < StringName >, format: impl AsArg < Option < Gd < crate::classes::RdTextureFormat >> >, view: impl AsArg < Option < Gd < crate::classes::RdTextureView >> >, unique: bool,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, Option < Gd < crate::classes::RdTextureFormat > > >, CowArg < 'a3, Option < Gd < crate::classes::RdTextureView > > >, bool,);
            let args = (context.into_arg(), name.into_arg(), format.into_arg(), view.into_arg(), unique,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7321usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "create_texture_from_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_texture_view(&mut self, context: impl AsArg < StringName >, name: impl AsArg < StringName >, view_name: impl AsArg < StringName >, view: impl AsArg < Option < Gd < crate::classes::RdTextureView >> >,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >, CowArg < 'a3, Option < Gd < crate::classes::RdTextureView > > >,);
            let args = (context.into_arg(), name.into_arg(), view_name.into_arg(), view.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "create_texture_view", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture(&self, context: impl AsArg < StringName >, name: impl AsArg < StringName >,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (context.into_arg(), name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7323usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_format(&self, context: impl AsArg < StringName >, name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::RdTextureFormat > > {
            type CallRet = Option < Gd < crate::classes::RdTextureFormat > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (context.into_arg(), name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_texture_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_slice(&mut self, context: impl AsArg < StringName >, name: impl AsArg < StringName >, layer: u32, mipmap: u32, layers: u32, mipmaps: u32,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, u32, u32, u32, u32,);
            let args = (context.into_arg(), name.into_arg(), layer, mipmap, layers, mipmaps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_texture_slice", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_slice_view(&mut self, context: impl AsArg < StringName >, name: impl AsArg < StringName >, layer: u32, mipmap: u32, layers: u32, mipmaps: u32, view: impl AsArg < Option < Gd < crate::classes::RdTextureView >> >,) -> Rid {
            type CallRet = Rid;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, u32, u32, u32, u32, CowArg < 'a2, Option < Gd < crate::classes::RdTextureView > > >,);
            let args = (context.into_arg(), name.into_arg(), layer, mipmap, layers, mipmaps, view.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_texture_slice_view", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_slice_size(&mut self, context: impl AsArg < StringName >, name: impl AsArg < StringName >, mipmap: u32,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, u32,);
            let args = (context.into_arg(), name.into_arg(), mipmap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_texture_slice_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_context(&mut self, context: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (context.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "clear_context", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_color_texture_full(&mut self, msaa: bool,) -> Rid {
            type CallRet = Rid;
            type CallParams = (bool,);
            let args = (msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_color_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_color_texture_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_color_texture(&mut self,) -> Rid {
            self.get_color_texture_ex() . done()
        }
        #[inline]
        pub fn get_color_texture_ex < 'a > (&'a mut self,) -> ExGetColorTexture < 'a > {
            ExGetColorTexture::new(self,)
        }
        pub(crate) fn get_color_layer_full(&mut self, layer: u32, msaa: bool,) -> Rid {
            type CallRet = Rid;
            type CallParams = (u32, bool,);
            let args = (layer, msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_color_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_color_layer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_color_layer(&mut self, layer: u32,) -> Rid {
            self.get_color_layer_ex(layer,) . done()
        }
        #[inline]
        pub fn get_color_layer_ex < 'a > (&'a mut self, layer: u32,) -> ExGetColorLayer < 'a > {
            ExGetColorLayer::new(self, layer,)
        }
        pub(crate) fn get_depth_texture_full(&mut self, msaa: bool,) -> Rid {
            type CallRet = Rid;
            type CallParams = (bool,);
            let args = (msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_depth_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_depth_texture_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_depth_texture(&mut self,) -> Rid {
            self.get_depth_texture_ex() . done()
        }
        #[inline]
        pub fn get_depth_texture_ex < 'a > (&'a mut self,) -> ExGetDepthTexture < 'a > {
            ExGetDepthTexture::new(self,)
        }
        pub(crate) fn get_depth_layer_full(&mut self, layer: u32, msaa: bool,) -> Rid {
            type CallRet = Rid;
            type CallParams = (u32, bool,);
            let args = (layer, msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_depth_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_depth_layer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_depth_layer(&mut self, layer: u32,) -> Rid {
            self.get_depth_layer_ex(layer,) . done()
        }
        #[inline]
        pub fn get_depth_layer_ex < 'a > (&'a mut self, layer: u32,) -> ExGetDepthLayer < 'a > {
            ExGetDepthLayer::new(self, layer,)
        }
        pub(crate) fn get_velocity_texture_full(&mut self, msaa: bool,) -> Rid {
            type CallRet = Rid;
            type CallParams = (bool,);
            let args = (msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_velocity_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_velocity_texture_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_velocity_texture(&mut self,) -> Rid {
            self.get_velocity_texture_ex() . done()
        }
        #[inline]
        pub fn get_velocity_texture_ex < 'a > (&'a mut self,) -> ExGetVelocityTexture < 'a > {
            ExGetVelocityTexture::new(self,)
        }
        pub(crate) fn get_velocity_layer_full(&mut self, layer: u32, msaa: bool,) -> Rid {
            type CallRet = Rid;
            type CallParams = (u32, bool,);
            let args = (layer, msaa,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_velocity_layer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_velocity_layer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_velocity_layer(&mut self, layer: u32,) -> Rid {
            self.get_velocity_layer_ex(layer,) . done()
        }
        #[inline]
        pub fn get_velocity_layer_ex < 'a > (&'a mut self, layer: u32,) -> ExGetVelocityLayer < 'a > {
            ExGetVelocityLayer::new(self, layer,)
        }
        pub fn get_render_target(&self,) -> Rid {
            type CallRet = Rid;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_render_target", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_view_count(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_view_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_internal_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_internal_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_target_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_target_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scaling_3d_mode(&self,) -> crate::classes::rendering_server::ViewportScaling3DMode {
            type CallRet = crate::classes::rendering_server::ViewportScaling3DMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_scaling_3d_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fsr_sharpness(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_fsr_sharpness", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_msaa_3d(&self,) -> crate::classes::rendering_server::ViewportMsaa {
            type CallRet = crate::classes::rendering_server::ViewportMsaa;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_msaa_3d", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_samples(&self,) -> crate::classes::rendering_device::TextureSamples {
            type CallRet = crate::classes::rendering_device::TextureSamples;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7342usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_texture_samples", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_screen_space_aa(&self,) -> crate::classes::rendering_server::ViewportScreenSpaceAa {
            type CallRet = crate::classes::rendering_server::ViewportScreenSpaceAa;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7343usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_screen_space_aa", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_use_taa(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7344usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_use_taa", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_use_debanding(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7345usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "RenderSceneBuffersRd", "get_use_debanding", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for RenderSceneBuffersRd {
        type Base = crate::classes::RenderSceneBuffers;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("RenderSceneBuffersRD"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for RenderSceneBuffersRd {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RenderSceneBuffers > for RenderSceneBuffersRd {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for RenderSceneBuffersRd {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for RenderSceneBuffersRd {
        
    }
    impl crate::obj::cap::GodotDefault for RenderSceneBuffersRd {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for RenderSceneBuffersRd {
        type Target = crate::classes::RenderSceneBuffers;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for RenderSceneBuffersRd {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`RenderSceneBuffersRd`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_RenderSceneBuffersRd__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::RenderSceneBuffersRd > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RenderSceneBuffers > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`RenderSceneBuffersRd::get_color_texture_ex`][super::RenderSceneBuffersRd::get_color_texture_ex]."]
#[must_use]
pub struct ExGetColorTexture < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderSceneBuffersRd, msaa: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetColorTexture < 'a > {
    fn new(surround_object: &'a mut re_export::RenderSceneBuffersRd,) -> Self {
        let msaa = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, msaa: msaa,
        }
    }
    #[inline]
    pub fn msaa(self, msaa: bool) -> Self {
        Self {
            msaa: msaa, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, msaa,
        }
        = self;
        re_export::RenderSceneBuffersRd::get_color_texture_full(surround_object, msaa,)
    }
}
#[doc = "Default-param extender for [`RenderSceneBuffersRd::get_color_layer_ex`][super::RenderSceneBuffersRd::get_color_layer_ex]."]
#[must_use]
pub struct ExGetColorLayer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderSceneBuffersRd, layer: u32, msaa: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetColorLayer < 'a > {
    fn new(surround_object: &'a mut re_export::RenderSceneBuffersRd, layer: u32,) -> Self {
        let msaa = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, msaa: msaa,
        }
    }
    #[inline]
    pub fn msaa(self, msaa: bool) -> Self {
        Self {
            msaa: msaa, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, layer, msaa,
        }
        = self;
        re_export::RenderSceneBuffersRd::get_color_layer_full(surround_object, layer, msaa,)
    }
}
#[doc = "Default-param extender for [`RenderSceneBuffersRd::get_depth_texture_ex`][super::RenderSceneBuffersRd::get_depth_texture_ex]."]
#[must_use]
pub struct ExGetDepthTexture < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderSceneBuffersRd, msaa: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDepthTexture < 'a > {
    fn new(surround_object: &'a mut re_export::RenderSceneBuffersRd,) -> Self {
        let msaa = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, msaa: msaa,
        }
    }
    #[inline]
    pub fn msaa(self, msaa: bool) -> Self {
        Self {
            msaa: msaa, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, msaa,
        }
        = self;
        re_export::RenderSceneBuffersRd::get_depth_texture_full(surround_object, msaa,)
    }
}
#[doc = "Default-param extender for [`RenderSceneBuffersRd::get_depth_layer_ex`][super::RenderSceneBuffersRd::get_depth_layer_ex]."]
#[must_use]
pub struct ExGetDepthLayer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderSceneBuffersRd, layer: u32, msaa: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetDepthLayer < 'a > {
    fn new(surround_object: &'a mut re_export::RenderSceneBuffersRd, layer: u32,) -> Self {
        let msaa = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, msaa: msaa,
        }
    }
    #[inline]
    pub fn msaa(self, msaa: bool) -> Self {
        Self {
            msaa: msaa, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, layer, msaa,
        }
        = self;
        re_export::RenderSceneBuffersRd::get_depth_layer_full(surround_object, layer, msaa,)
    }
}
#[doc = "Default-param extender for [`RenderSceneBuffersRd::get_velocity_texture_ex`][super::RenderSceneBuffersRd::get_velocity_texture_ex]."]
#[must_use]
pub struct ExGetVelocityTexture < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderSceneBuffersRd, msaa: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetVelocityTexture < 'a > {
    fn new(surround_object: &'a mut re_export::RenderSceneBuffersRd,) -> Self {
        let msaa = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, msaa: msaa,
        }
    }
    #[inline]
    pub fn msaa(self, msaa: bool) -> Self {
        Self {
            msaa: msaa, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, msaa,
        }
        = self;
        re_export::RenderSceneBuffersRd::get_velocity_texture_full(surround_object, msaa,)
    }
}
#[doc = "Default-param extender for [`RenderSceneBuffersRd::get_velocity_layer_ex`][super::RenderSceneBuffersRd::get_velocity_layer_ex]."]
#[must_use]
pub struct ExGetVelocityLayer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::RenderSceneBuffersRd, layer: u32, msaa: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetVelocityLayer < 'a > {
    fn new(surround_object: &'a mut re_export::RenderSceneBuffersRd, layer: u32,) -> Self {
        let msaa = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, layer: layer, msaa: msaa,
        }
    }
    #[inline]
    pub fn msaa(self, msaa: bool) -> Self {
        Self {
            msaa: msaa, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rid {
        let Self {
            _phantom, surround_object, layer, msaa,
        }
        = self;
        re_export::RenderSceneBuffersRd::get_velocity_layer_full(surround_object, layer, msaa,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::RenderSceneBuffersRd;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for RenderSceneBuffersRd {
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