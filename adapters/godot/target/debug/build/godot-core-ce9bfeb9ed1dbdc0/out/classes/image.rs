#![doc = "Sidecar module for class [`Image`][crate::classes::Image].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Image` enums](https://docs.godotengine.org/en/stable/classes/class_image.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Image.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`image`][crate::classes::image]: sidecar module with related enum/flag types\n* [`IImage`][crate::classes::IImage]: virtual methods\n\n\nSee also [Godot docs for `Image`](https://docs.godotengine.org/en/stable/classes/class_image.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Image::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Image {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Image`][crate::classes::Image].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Image` methods](https://docs.godotengine.org/en/stable/classes/class_image.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IImage: crate::obj::GodotClass < Base = Image > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Image {
        pub fn get_width(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_height(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_mipmaps(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "has_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_format(&self,) -> crate::classes::image::Format {
            type CallRet = crate::classes::image::Format;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_data(&self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_data_size(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_data_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn convert(&mut self, format: crate::classes::image::Format,) {
            type CallRet = ();
            type CallParams = (crate::classes::image::Format,);
            let args = (format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "convert", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mipmap_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_mipmap_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mipmap_offset(&self, mipmap: i32,) -> i64 {
            type CallRet = i64;
            type CallParams = (i32,);
            let args = (mipmap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_mipmap_offset", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn resize_to_po2_full(&mut self, square: bool, interpolation: crate::classes::image::Interpolation,) {
            type CallRet = ();
            type CallParams = (bool, crate::classes::image::Interpolation,);
            let args = (square, interpolation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "resize_to_po2", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::resize_to_po2_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn resize_to_po2(&mut self,) {
            self.resize_to_po2_ex() . done()
        }
        #[inline]
        pub fn resize_to_po2_ex < 'a > (&'a mut self,) -> ExResizeToPo2 < 'a > {
            ExResizeToPo2::new(self,)
        }
        pub(crate) fn resize_full(&mut self, width: i32, height: i32, interpolation: crate::classes::image::Interpolation,) {
            type CallRet = ();
            type CallParams = (i32, i32, crate::classes::image::Interpolation,);
            let args = (width, height, interpolation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "resize", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::resize_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn resize(&mut self, width: i32, height: i32,) {
            self.resize_ex(width, height,) . done()
        }
        #[inline]
        pub fn resize_ex < 'a > (&'a mut self, width: i32, height: i32,) -> ExResize < 'a > {
            ExResize::new(self, width, height,)
        }
        pub fn shrink_x2(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "shrink_x2", Some(self.__validated_obj()), args,)
            }
        }
        pub fn crop(&mut self, width: i32, height: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (width, height,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "crop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn flip_x(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "flip_x", Some(self.__validated_obj()), args,)
            }
        }
        pub fn flip_y(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "flip_y", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn generate_mipmaps_full(&mut self, renormalize: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (bool,);
            let args = (renormalize,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "generate_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::generate_mipmaps_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn generate_mipmaps(&mut self,) -> crate::global::Error {
            self.generate_mipmaps_ex() . done()
        }
        #[inline]
        pub fn generate_mipmaps_ex < 'a > (&'a mut self,) -> ExGenerateMipmaps < 'a > {
            ExGenerateMipmaps::new(self,)
        }
        pub fn clear_mipmaps(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "clear_mipmaps", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create(width: i32, height: i32, use_mipmaps: bool, format: crate::classes::image::Format,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (i32, i32, bool, crate::classes::image::Format,);
            let args = (width, height, use_mipmaps, format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "create", None, args,)
            }
        }
        pub fn create_empty(width: i32, height: i32, use_mipmaps: bool, format: crate::classes::image::Format,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (i32, i32, bool, crate::classes::image::Format,);
            let args = (width, height, use_mipmaps, format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "create_empty", None, args,)
            }
        }
        pub fn create_from_data(width: i32, height: i32, use_mipmaps: bool, format: crate::classes::image::Format, data: &PackedByteArray,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams < 'a0, > = (i32, i32, bool, crate::classes::image::Format, RefArg < 'a0, PackedByteArray >,);
            let args = (width, height, use_mipmaps, format, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "create_from_data", None, args,)
            }
        }
        pub fn set_data(&mut self, width: i32, height: i32, use_mipmaps: bool, format: crate::classes::image::Format, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, i32, bool, crate::classes::image::Format, RefArg < 'a0, PackedByteArray >,);
            let args = (width, height, use_mipmaps, format, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "set_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_empty(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "is_empty", Some(self.__validated_obj()), args,)
            }
        }
        pub fn load(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load", Some(self.__validated_obj()), args,)
            }
        }
        pub fn load_from_file(path: impl AsArg < GString >,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_from_file", None, args,)
            }
        }
        pub fn save_png(&self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_png", Some(self.__validated_obj()), args,)
            }
        }
        pub fn save_png_to_buffer(&self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_png_to_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn save_jpg_full(&self, path: CowArg < GString >, quality: f32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, f32,);
            let args = (path, quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_jpg", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::save_jpg_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn save_jpg(&self, path: impl AsArg < GString >,) -> crate::global::Error {
            self.save_jpg_ex(path,) . done()
        }
        #[inline]
        pub fn save_jpg_ex < 'a > (&'a self, path: impl AsArg < GString > + 'a,) -> ExSaveJpg < 'a > {
            ExSaveJpg::new(self, path,)
        }
        pub(crate) fn save_jpg_to_buffer_full(&self, quality: f32,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (f32,);
            let args = (quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_jpg_to_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::save_jpg_to_buffer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn save_jpg_to_buffer(&self,) -> PackedByteArray {
            self.save_jpg_to_buffer_ex() . done()
        }
        #[inline]
        pub fn save_jpg_to_buffer_ex < 'a > (&'a self,) -> ExSaveJpgToBuffer < 'a > {
            ExSaveJpgToBuffer::new(self,)
        }
        pub(crate) fn save_exr_full(&self, path: CowArg < GString >, grayscale: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool,);
            let args = (path, grayscale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_exr", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::save_exr_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn save_exr(&self, path: impl AsArg < GString >,) -> crate::global::Error {
            self.save_exr_ex(path,) . done()
        }
        #[inline]
        pub fn save_exr_ex < 'a > (&'a self, path: impl AsArg < GString > + 'a,) -> ExSaveExr < 'a > {
            ExSaveExr::new(self, path,)
        }
        pub(crate) fn save_exr_to_buffer_full(&self, grayscale: bool,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (bool,);
            let args = (grayscale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_exr_to_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::save_exr_to_buffer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn save_exr_to_buffer(&self,) -> PackedByteArray {
            self.save_exr_to_buffer_ex() . done()
        }
        #[inline]
        pub fn save_exr_to_buffer_ex < 'a > (&'a self,) -> ExSaveExrToBuffer < 'a > {
            ExSaveExrToBuffer::new(self,)
        }
        pub fn save_dds(&self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_dds", Some(self.__validated_obj()), args,)
            }
        }
        pub fn save_dds_to_buffer(&self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_dds_to_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn save_webp_full(&self, path: CowArg < GString >, lossy: bool, quality: f32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, bool, f32,);
            let args = (path, lossy, quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_webp", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::save_webp_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn save_webp(&self, path: impl AsArg < GString >,) -> crate::global::Error {
            self.save_webp_ex(path,) . done()
        }
        #[inline]
        pub fn save_webp_ex < 'a > (&'a self, path: impl AsArg < GString > + 'a,) -> ExSaveWebp < 'a > {
            ExSaveWebp::new(self, path,)
        }
        pub(crate) fn save_webp_to_buffer_full(&self, lossy: bool, quality: f32,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = (bool, f32,);
            let args = (lossy, quality,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4279usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "save_webp_to_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::save_webp_to_buffer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn save_webp_to_buffer(&self,) -> PackedByteArray {
            self.save_webp_to_buffer_ex() . done()
        }
        #[inline]
        pub fn save_webp_to_buffer_ex < 'a > (&'a self,) -> ExSaveWebpToBuffer < 'a > {
            ExSaveWebpToBuffer::new(self,)
        }
        pub fn detect_alpha(&self,) -> crate::classes::image::AlphaMode {
            type CallRet = crate::classes::image::AlphaMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4280usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "detect_alpha", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_invisible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4281usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "is_invisible", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn detect_used_channels_full(&self, source: crate::classes::image::CompressSource,) -> crate::classes::image::UsedChannels {
            type CallRet = crate::classes::image::UsedChannels;
            type CallParams = (crate::classes::image::CompressSource,);
            let args = (source,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4282usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "detect_used_channels", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::detect_used_channels_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn detect_used_channels(&self,) -> crate::classes::image::UsedChannels {
            self.detect_used_channels_ex() . done()
        }
        #[inline]
        pub fn detect_used_channels_ex < 'a > (&'a self,) -> ExDetectUsedChannels < 'a > {
            ExDetectUsedChannels::new(self,)
        }
        pub(crate) fn compress_full(&mut self, mode: crate::classes::image::CompressMode, source: crate::classes::image::CompressSource, astc_format: crate::classes::image::AstcFormat,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (crate::classes::image::CompressMode, crate::classes::image::CompressSource, crate::classes::image::AstcFormat,);
            let args = (mode, source, astc_format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4283usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "compress", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::compress_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn compress(&mut self, mode: crate::classes::image::CompressMode,) -> crate::global::Error {
            self.compress_ex(mode,) . done()
        }
        #[inline]
        pub fn compress_ex < 'a > (&'a mut self, mode: crate::classes::image::CompressMode,) -> ExCompress < 'a > {
            ExCompress::new(self, mode,)
        }
        pub(crate) fn compress_from_channels_full(&mut self, mode: crate::classes::image::CompressMode, channels: crate::classes::image::UsedChannels, astc_format: crate::classes::image::AstcFormat,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (crate::classes::image::CompressMode, crate::classes::image::UsedChannels, crate::classes::image::AstcFormat,);
            let args = (mode, channels, astc_format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4284usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "compress_from_channels", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::compress_from_channels_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn compress_from_channels(&mut self, mode: crate::classes::image::CompressMode, channels: crate::classes::image::UsedChannels,) -> crate::global::Error {
            self.compress_from_channels_ex(mode, channels,) . done()
        }
        #[inline]
        pub fn compress_from_channels_ex < 'a > (&'a mut self, mode: crate::classes::image::CompressMode, channels: crate::classes::image::UsedChannels,) -> ExCompressFromChannels < 'a > {
            ExCompressFromChannels::new(self, mode, channels,)
        }
        pub fn decompress(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4285usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "decompress", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_compressed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4286usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "is_compressed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rotate_90(&mut self, direction: crate::global::ClockDirection,) {
            type CallRet = ();
            type CallParams = (crate::global::ClockDirection,);
            let args = (direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4287usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "rotate_90", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rotate_180(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4288usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "rotate_180", Some(self.__validated_obj()), args,)
            }
        }
        pub fn fix_alpha_edges(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4289usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "fix_alpha_edges", Some(self.__validated_obj()), args,)
            }
        }
        pub fn premultiply_alpha(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4290usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "premultiply_alpha", Some(self.__validated_obj()), args,)
            }
        }
        pub fn srgb_to_linear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4291usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "srgb_to_linear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn linear_to_srgb(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4292usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "linear_to_srgb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn normal_map_to_xy(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4293usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "normal_map_to_xy", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rgbe_to_srgb(&mut self,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4294usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "rgbe_to_srgb", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn bump_map_to_normal_map_full(&mut self, bump_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (bump_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4295usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "bump_map_to_normal_map", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::bump_map_to_normal_map_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn bump_map_to_normal_map(&mut self,) {
            self.bump_map_to_normal_map_ex() . done()
        }
        #[inline]
        pub fn bump_map_to_normal_map_ex < 'a > (&'a mut self,) -> ExBumpMapToNormalMap < 'a > {
            ExBumpMapToNormalMap::new(self,)
        }
        pub fn compute_image_metrics(&mut self, compared_image: impl AsArg < Option < Gd < crate::classes::Image >> >, use_luma: bool,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >, bool,);
            let args = (compared_image.into_arg(), use_luma,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4296usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "compute_image_metrics", Some(self.__validated_obj()), args,)
            }
        }
        pub fn blit_rect(&mut self, src: impl AsArg < Option < Gd < crate::classes::Image >> >, src_rect: Rect2i, dst: Vector2i,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >, Rect2i, Vector2i,);
            let args = (src.into_arg(), src_rect, dst,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4297usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "blit_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn blit_rect_mask(&mut self, src: impl AsArg < Option < Gd < crate::classes::Image >> >, mask: impl AsArg < Option < Gd < crate::classes::Image >> >, src_rect: Rect2i, dst: Vector2i,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >, CowArg < 'a1, Option < Gd < crate::classes::Image > > >, Rect2i, Vector2i,);
            let args = (src.into_arg(), mask.into_arg(), src_rect, dst,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4298usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "blit_rect_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn blend_rect(&mut self, src: impl AsArg < Option < Gd < crate::classes::Image >> >, src_rect: Rect2i, dst: Vector2i,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >, Rect2i, Vector2i,);
            let args = (src.into_arg(), src_rect, dst,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4299usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "blend_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn blend_rect_mask(&mut self, src: impl AsArg < Option < Gd < crate::classes::Image >> >, mask: impl AsArg < Option < Gd < crate::classes::Image >> >, src_rect: Rect2i, dst: Vector2i,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >, CowArg < 'a1, Option < Gd < crate::classes::Image > > >, Rect2i, Vector2i,);
            let args = (src.into_arg(), mask.into_arg(), src_rect, dst,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4300usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "blend_rect_mask", Some(self.__validated_obj()), args,)
            }
        }
        pub fn fill(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4301usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "fill", Some(self.__validated_obj()), args,)
            }
        }
        pub fn fill_rect(&mut self, rect: Rect2i, color: Color,) {
            type CallRet = ();
            type CallParams = (Rect2i, Color,);
            let args = (rect, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4302usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "fill_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_used_rect(&self,) -> Rect2i {
            type CallRet = Rect2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4303usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_used_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_region(&self, region: Rect2i,) -> Option < Gd < crate::classes::Image > > {
            type CallRet = Option < Gd < crate::classes::Image > >;
            type CallParams = (Rect2i,);
            let args = (region,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4304usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn copy_from(&mut self, src: impl AsArg < Option < Gd < crate::classes::Image >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Image > > >,);
            let args = (src.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4305usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "copy_from", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pixelv(&self, point: Vector2i,) -> Color {
            type CallRet = Color;
            type CallParams = (Vector2i,);
            let args = (point,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_pixelv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pixel(&self, x: i32, y: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32, i32,);
            let args = (x, y,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "get_pixel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pixelv(&mut self, point: Vector2i, color: Color,) {
            type CallRet = ();
            type CallParams = (Vector2i, Color,);
            let args = (point, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4308usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "set_pixelv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pixel(&mut self, x: i32, y: i32, color: Color,) {
            type CallRet = ();
            type CallParams = (i32, i32, Color,);
            let args = (x, y, color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4309usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "set_pixel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn adjust_bcs(&mut self, brightness: f32, contrast: f32, saturation: f32,) {
            type CallRet = ();
            type CallParams = (f32, f32, f32,);
            let args = (brightness, contrast, saturation,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4310usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "adjust_bcs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn load_png_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4311usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_png_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn load_jpg_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4312usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_jpg_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn load_webp_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4313usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_webp_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn load_tga_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4314usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_tga_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn load_bmp_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4315usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_bmp_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn load_ktx_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4316usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_ktx_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn load_dds_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4317usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_dds_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn load_svg_from_buffer_full(&mut self, buffer: RefArg < PackedByteArray >, scale: f32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >, f32,);
            let args = (buffer, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4318usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_svg_from_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::load_svg_from_buffer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn load_svg_from_buffer(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            self.load_svg_from_buffer_ex(buffer,) . done()
        }
        #[inline]
        pub fn load_svg_from_buffer_ex < 'a > (&'a mut self, buffer: &'a PackedByteArray,) -> ExLoadSvgFromBuffer < 'a > {
            ExLoadSvgFromBuffer::new(self, buffer,)
        }
        pub(crate) fn load_svg_from_string_full(&mut self, svg_str: CowArg < GString >, scale: f32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, f32,);
            let args = (svg_str, scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4319usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Image", "load_svg_from_string", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::load_svg_from_string_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn load_svg_from_string(&mut self, svg_str: impl AsArg < GString >,) -> crate::global::Error {
            self.load_svg_from_string_ex(svg_str,) . done()
        }
        #[inline]
        pub fn load_svg_from_string_ex < 'a > (&'a mut self, svg_str: impl AsArg < GString > + 'a,) -> ExLoadSvgFromString < 'a > {
            ExLoadSvgFromString::new(self, svg_str,)
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
        pub const MAX_WIDTH: i32 = 16777216i32;
        pub const MAX_HEIGHT: i32 = 16777216i32;
        
    }
    impl crate::obj::GodotClass for Image {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Image"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Image {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Image {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Image {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Image {
        
    }
    impl crate::obj::cap::GodotDefault for Image {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Image {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Image {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Image`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Image__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Image > for $Class {
                
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
#[doc = "Default-param extender for [`Image::resize_to_po2_ex`][super::Image::resize_to_po2_ex]."]
#[must_use]
pub struct ExResizeToPo2 < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Image, square: bool, interpolation: crate::classes::image::Interpolation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExResizeToPo2 < 'a > {
    fn new(surround_object: &'a mut re_export::Image,) -> Self {
        let square = false;
        let interpolation = crate::obj::EngineEnum::from_ord(1);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, square: square, interpolation: interpolation,
        }
    }
    #[inline]
    pub fn square(self, square: bool) -> Self {
        Self {
            square: square, .. self
        }
    }
    #[inline]
    pub fn interpolation(self, interpolation: crate::classes::image::Interpolation) -> Self {
        Self {
            interpolation: interpolation, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, square, interpolation,
        }
        = self;
        re_export::Image::resize_to_po2_full(surround_object, square, interpolation,)
    }
}
#[doc = "Default-param extender for [`Image::resize_ex`][super::Image::resize_ex]."]
#[must_use]
pub struct ExResize < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Image, width: i32, height: i32, interpolation: crate::classes::image::Interpolation,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExResize < 'a > {
    fn new(surround_object: &'a mut re_export::Image, width: i32, height: i32,) -> Self {
        let interpolation = crate::obj::EngineEnum::from_ord(1);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, width: width, height: height, interpolation: interpolation,
        }
    }
    #[inline]
    pub fn interpolation(self, interpolation: crate::classes::image::Interpolation) -> Self {
        Self {
            interpolation: interpolation, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, width, height, interpolation,
        }
        = self;
        re_export::Image::resize_full(surround_object, width, height, interpolation,)
    }
}
#[doc = "Default-param extender for [`Image::generate_mipmaps_ex`][super::Image::generate_mipmaps_ex]."]
#[must_use]
pub struct ExGenerateMipmaps < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Image, renormalize: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGenerateMipmaps < 'a > {
    fn new(surround_object: &'a mut re_export::Image,) -> Self {
        let renormalize = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, renormalize: renormalize,
        }
    }
    #[inline]
    pub fn renormalize(self, renormalize: bool) -> Self {
        Self {
            renormalize: renormalize, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, renormalize,
        }
        = self;
        re_export::Image::generate_mipmaps_full(surround_object, renormalize,)
    }
}
#[doc = "Default-param extender for [`Image::save_jpg_ex`][super::Image::save_jpg_ex]."]
#[must_use]
pub struct ExSaveJpg < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Image, path: CowArg < 'a, GString >, quality: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveJpg < 'a > {
    fn new(surround_object: &'a re_export::Image, path: impl AsArg < GString > + 'a,) -> Self {
        let quality = 0.75f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), quality: quality,
        }
    }
    #[inline]
    pub fn quality(self, quality: f32) -> Self {
        Self {
            quality: quality, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, path, quality,
        }
        = self;
        re_export::Image::save_jpg_full(surround_object, path, quality,)
    }
}
#[doc = "Default-param extender for [`Image::save_jpg_to_buffer_ex`][super::Image::save_jpg_to_buffer_ex]."]
#[must_use]
pub struct ExSaveJpgToBuffer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Image, quality: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveJpgToBuffer < 'a > {
    fn new(surround_object: &'a re_export::Image,) -> Self {
        let quality = 0.75f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, quality: quality,
        }
    }
    #[inline]
    pub fn quality(self, quality: f32) -> Self {
        Self {
            quality: quality, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        let Self {
            _phantom, surround_object, quality,
        }
        = self;
        re_export::Image::save_jpg_to_buffer_full(surround_object, quality,)
    }
}
#[doc = "Default-param extender for [`Image::save_exr_ex`][super::Image::save_exr_ex]."]
#[must_use]
pub struct ExSaveExr < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Image, path: CowArg < 'a, GString >, grayscale: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveExr < 'a > {
    fn new(surround_object: &'a re_export::Image, path: impl AsArg < GString > + 'a,) -> Self {
        let grayscale = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), grayscale: grayscale,
        }
    }
    #[inline]
    pub fn grayscale(self, grayscale: bool) -> Self {
        Self {
            grayscale: grayscale, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, path, grayscale,
        }
        = self;
        re_export::Image::save_exr_full(surround_object, path, grayscale,)
    }
}
#[doc = "Default-param extender for [`Image::save_exr_to_buffer_ex`][super::Image::save_exr_to_buffer_ex]."]
#[must_use]
pub struct ExSaveExrToBuffer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Image, grayscale: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveExrToBuffer < 'a > {
    fn new(surround_object: &'a re_export::Image,) -> Self {
        let grayscale = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, grayscale: grayscale,
        }
    }
    #[inline]
    pub fn grayscale(self, grayscale: bool) -> Self {
        Self {
            grayscale: grayscale, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        let Self {
            _phantom, surround_object, grayscale,
        }
        = self;
        re_export::Image::save_exr_to_buffer_full(surround_object, grayscale,)
    }
}
#[doc = "Default-param extender for [`Image::save_webp_ex`][super::Image::save_webp_ex]."]
#[must_use]
pub struct ExSaveWebp < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Image, path: CowArg < 'a, GString >, lossy: bool, quality: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveWebp < 'a > {
    fn new(surround_object: &'a re_export::Image, path: impl AsArg < GString > + 'a,) -> Self {
        let lossy = false;
        let quality = 0.75f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), lossy: lossy, quality: quality,
        }
    }
    #[inline]
    pub fn lossy(self, lossy: bool) -> Self {
        Self {
            lossy: lossy, .. self
        }
    }
    #[inline]
    pub fn quality(self, quality: f32) -> Self {
        Self {
            quality: quality, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, path, lossy, quality,
        }
        = self;
        re_export::Image::save_webp_full(surround_object, path, lossy, quality,)
    }
}
#[doc = "Default-param extender for [`Image::save_webp_to_buffer_ex`][super::Image::save_webp_to_buffer_ex]."]
#[must_use]
pub struct ExSaveWebpToBuffer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Image, lossy: bool, quality: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSaveWebpToBuffer < 'a > {
    fn new(surround_object: &'a re_export::Image,) -> Self {
        let lossy = false;
        let quality = 0.75f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, lossy: lossy, quality: quality,
        }
    }
    #[inline]
    pub fn lossy(self, lossy: bool) -> Self {
        Self {
            lossy: lossy, .. self
        }
    }
    #[inline]
    pub fn quality(self, quality: f32) -> Self {
        Self {
            quality: quality, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedByteArray {
        let Self {
            _phantom, surround_object, lossy, quality,
        }
        = self;
        re_export::Image::save_webp_to_buffer_full(surround_object, lossy, quality,)
    }
}
#[doc = "Default-param extender for [`Image::detect_used_channels_ex`][super::Image::detect_used_channels_ex]."]
#[must_use]
pub struct ExDetectUsedChannels < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Image, source: crate::classes::image::CompressSource,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDetectUsedChannels < 'a > {
    fn new(surround_object: &'a re_export::Image,) -> Self {
        let source = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, source: source,
        }
    }
    #[inline]
    pub fn source(self, source: crate::classes::image::CompressSource) -> Self {
        Self {
            source: source, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::classes::image::UsedChannels {
        let Self {
            _phantom, surround_object, source,
        }
        = self;
        re_export::Image::detect_used_channels_full(surround_object, source,)
    }
}
#[doc = "Default-param extender for [`Image::compress_ex`][super::Image::compress_ex]."]
#[must_use]
pub struct ExCompress < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Image, mode: crate::classes::image::CompressMode, source: crate::classes::image::CompressSource, astc_format: crate::classes::image::AstcFormat,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCompress < 'a > {
    fn new(surround_object: &'a mut re_export::Image, mode: crate::classes::image::CompressMode,) -> Self {
        let source = crate::obj::EngineEnum::from_ord(0);
        let astc_format = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mode: mode, source: source, astc_format: astc_format,
        }
    }
    #[inline]
    pub fn source(self, source: crate::classes::image::CompressSource) -> Self {
        Self {
            source: source, .. self
        }
    }
    #[inline]
    pub fn astc_format(self, astc_format: crate::classes::image::AstcFormat) -> Self {
        Self {
            astc_format: astc_format, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, mode, source, astc_format,
        }
        = self;
        re_export::Image::compress_full(surround_object, mode, source, astc_format,)
    }
}
#[doc = "Default-param extender for [`Image::compress_from_channels_ex`][super::Image::compress_from_channels_ex]."]
#[must_use]
pub struct ExCompressFromChannels < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Image, mode: crate::classes::image::CompressMode, channels: crate::classes::image::UsedChannels, astc_format: crate::classes::image::AstcFormat,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCompressFromChannels < 'a > {
    fn new(surround_object: &'a mut re_export::Image, mode: crate::classes::image::CompressMode, channels: crate::classes::image::UsedChannels,) -> Self {
        let astc_format = crate::obj::EngineEnum::from_ord(0);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, mode: mode, channels: channels, astc_format: astc_format,
        }
    }
    #[inline]
    pub fn astc_format(self, astc_format: crate::classes::image::AstcFormat) -> Self {
        Self {
            astc_format: astc_format, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, mode, channels, astc_format,
        }
        = self;
        re_export::Image::compress_from_channels_full(surround_object, mode, channels, astc_format,)
    }
}
#[doc = "Default-param extender for [`Image::bump_map_to_normal_map_ex`][super::Image::bump_map_to_normal_map_ex]."]
#[must_use]
pub struct ExBumpMapToNormalMap < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Image, bump_scale: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBumpMapToNormalMap < 'a > {
    fn new(surround_object: &'a mut re_export::Image,) -> Self {
        let bump_scale = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bump_scale: bump_scale,
        }
    }
    #[inline]
    pub fn bump_scale(self, bump_scale: f32) -> Self {
        Self {
            bump_scale: bump_scale, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, bump_scale,
        }
        = self;
        re_export::Image::bump_map_to_normal_map_full(surround_object, bump_scale,)
    }
}
#[doc = "Default-param extender for [`Image::load_svg_from_buffer_ex`][super::Image::load_svg_from_buffer_ex]."]
#[must_use]
pub struct ExLoadSvgFromBuffer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Image, buffer: CowArg < 'a, PackedByteArray >, scale: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLoadSvgFromBuffer < 'a > {
    fn new(surround_object: &'a mut re_export::Image, buffer: &'a PackedByteArray,) -> Self {
        let scale = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, buffer: CowArg::Borrowed(buffer), scale: scale,
        }
    }
    #[inline]
    pub fn scale(self, scale: f32) -> Self {
        Self {
            scale: scale, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, buffer, scale,
        }
        = self;
        re_export::Image::load_svg_from_buffer_full(surround_object, buffer.cow_as_arg(), scale,)
    }
}
#[doc = "Default-param extender for [`Image::load_svg_from_string_ex`][super::Image::load_svg_from_string_ex]."]
#[must_use]
pub struct ExLoadSvgFromString < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Image, svg_str: CowArg < 'a, GString >, scale: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExLoadSvgFromString < 'a > {
    fn new(surround_object: &'a mut re_export::Image, svg_str: impl AsArg < GString > + 'a,) -> Self {
        let scale = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, svg_str: svg_str.into_arg(), scale: scale,
        }
    }
    #[inline]
    pub fn scale(self, scale: f32) -> Self {
        Self {
            scale: scale, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, svg_str, scale,
        }
        = self;
        re_export::Image::load_svg_from_string_full(surround_object, svg_str, scale,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Format {
    ord: i32
}
impl Format {
    #[doc(alias = "FORMAT_L8")]
    #[doc = "Godot enumerator name: `FORMAT_L8`"]
    pub const L8: Format = Format {
        ord: 0i32
    };
    #[doc(alias = "FORMAT_LA8")]
    #[doc = "Godot enumerator name: `FORMAT_LA8`"]
    pub const LA8: Format = Format {
        ord: 1i32
    };
    #[doc(alias = "FORMAT_R8")]
    #[doc = "Godot enumerator name: `FORMAT_R8`"]
    pub const R8: Format = Format {
        ord: 2i32
    };
    #[doc(alias = "FORMAT_RG8")]
    #[doc = "Godot enumerator name: `FORMAT_RG8`"]
    pub const RG8: Format = Format {
        ord: 3i32
    };
    #[doc(alias = "FORMAT_RGB8")]
    #[doc = "Godot enumerator name: `FORMAT_RGB8`"]
    pub const RGB8: Format = Format {
        ord: 4i32
    };
    #[doc(alias = "FORMAT_RGBA8")]
    #[doc = "Godot enumerator name: `FORMAT_RGBA8`"]
    pub const RGBA8: Format = Format {
        ord: 5i32
    };
    #[doc(alias = "FORMAT_RGBA4444")]
    #[doc = "Godot enumerator name: `FORMAT_RGBA4444`"]
    pub const RGBA4444: Format = Format {
        ord: 6i32
    };
    #[doc(alias = "FORMAT_RGB565")]
    #[doc = "Godot enumerator name: `FORMAT_RGB565`"]
    pub const RGB565: Format = Format {
        ord: 7i32
    };
    #[doc(alias = "FORMAT_RF")]
    #[doc = "Godot enumerator name: `FORMAT_RF`"]
    pub const RF: Format = Format {
        ord: 8i32
    };
    #[doc(alias = "FORMAT_RGF")]
    #[doc = "Godot enumerator name: `FORMAT_RGF`"]
    pub const RGF: Format = Format {
        ord: 9i32
    };
    #[doc(alias = "FORMAT_RGBF")]
    #[doc = "Godot enumerator name: `FORMAT_RGBF`"]
    pub const RGBF: Format = Format {
        ord: 10i32
    };
    #[doc(alias = "FORMAT_RGBAF")]
    #[doc = "Godot enumerator name: `FORMAT_RGBAF`"]
    pub const RGBAF: Format = Format {
        ord: 11i32
    };
    #[doc(alias = "FORMAT_RH")]
    #[doc = "Godot enumerator name: `FORMAT_RH`"]
    pub const RH: Format = Format {
        ord: 12i32
    };
    #[doc(alias = "FORMAT_RGH")]
    #[doc = "Godot enumerator name: `FORMAT_RGH`"]
    pub const RGH: Format = Format {
        ord: 13i32
    };
    #[doc(alias = "FORMAT_RGBH")]
    #[doc = "Godot enumerator name: `FORMAT_RGBH`"]
    pub const RGBH: Format = Format {
        ord: 14i32
    };
    #[doc(alias = "FORMAT_RGBAH")]
    #[doc = "Godot enumerator name: `FORMAT_RGBAH`"]
    pub const RGBAH: Format = Format {
        ord: 15i32
    };
    #[doc(alias = "FORMAT_RGBE9995")]
    #[doc = "Godot enumerator name: `FORMAT_RGBE9995`"]
    pub const RGBE9995: Format = Format {
        ord: 16i32
    };
    #[doc(alias = "FORMAT_DXT1")]
    #[doc = "Godot enumerator name: `FORMAT_DXT1`"]
    pub const DXT1: Format = Format {
        ord: 17i32
    };
    #[doc(alias = "FORMAT_DXT3")]
    #[doc = "Godot enumerator name: `FORMAT_DXT3`"]
    pub const DXT3: Format = Format {
        ord: 18i32
    };
    #[doc(alias = "FORMAT_DXT5")]
    #[doc = "Godot enumerator name: `FORMAT_DXT5`"]
    pub const DXT5: Format = Format {
        ord: 19i32
    };
    #[doc(alias = "FORMAT_RGTC_R")]
    #[doc = "Godot enumerator name: `FORMAT_RGTC_R`"]
    pub const RGTC_R: Format = Format {
        ord: 20i32
    };
    #[doc(alias = "FORMAT_RGTC_RG")]
    #[doc = "Godot enumerator name: `FORMAT_RGTC_RG`"]
    pub const RGTC_RG: Format = Format {
        ord: 21i32
    };
    #[doc(alias = "FORMAT_BPTC_RGBA")]
    #[doc = "Godot enumerator name: `FORMAT_BPTC_RGBA`"]
    pub const BPTC_RGBA: Format = Format {
        ord: 22i32
    };
    #[doc(alias = "FORMAT_BPTC_RGBF")]
    #[doc = "Godot enumerator name: `FORMAT_BPTC_RGBF`"]
    pub const BPTC_RGBF: Format = Format {
        ord: 23i32
    };
    #[doc(alias = "FORMAT_BPTC_RGBFU")]
    #[doc = "Godot enumerator name: `FORMAT_BPTC_RGBFU`"]
    pub const BPTC_RGBFU: Format = Format {
        ord: 24i32
    };
    #[doc(alias = "FORMAT_ETC")]
    #[doc = "Godot enumerator name: `FORMAT_ETC`"]
    pub const ETC: Format = Format {
        ord: 25i32
    };
    #[doc(alias = "FORMAT_ETC2_R11")]
    #[doc = "Godot enumerator name: `FORMAT_ETC2_R11`"]
    pub const ETC2_R11: Format = Format {
        ord: 26i32
    };
    #[doc(alias = "FORMAT_ETC2_R11S")]
    #[doc = "Godot enumerator name: `FORMAT_ETC2_R11S`"]
    pub const ETC2_R11S: Format = Format {
        ord: 27i32
    };
    #[doc(alias = "FORMAT_ETC2_RG11")]
    #[doc = "Godot enumerator name: `FORMAT_ETC2_RG11`"]
    pub const ETC2_RG11: Format = Format {
        ord: 28i32
    };
    #[doc(alias = "FORMAT_ETC2_RG11S")]
    #[doc = "Godot enumerator name: `FORMAT_ETC2_RG11S`"]
    pub const ETC2_RG11S: Format = Format {
        ord: 29i32
    };
    #[doc(alias = "FORMAT_ETC2_RGB8")]
    #[doc = "Godot enumerator name: `FORMAT_ETC2_RGB8`"]
    pub const ETC2_RGB8: Format = Format {
        ord: 30i32
    };
    #[doc(alias = "FORMAT_ETC2_RGBA8")]
    #[doc = "Godot enumerator name: `FORMAT_ETC2_RGBA8`"]
    pub const ETC2_RGBA8: Format = Format {
        ord: 31i32
    };
    #[doc(alias = "FORMAT_ETC2_RGB8A1")]
    #[doc = "Godot enumerator name: `FORMAT_ETC2_RGB8A1`"]
    pub const ETC2_RGB8A1: Format = Format {
        ord: 32i32
    };
    #[doc(alias = "FORMAT_ETC2_RA_AS_RG")]
    #[doc = "Godot enumerator name: `FORMAT_ETC2_RA_AS_RG`"]
    pub const ETC2_RA_AS_RG: Format = Format {
        ord: 33i32
    };
    #[doc(alias = "FORMAT_DXT5_RA_AS_RG")]
    #[doc = "Godot enumerator name: `FORMAT_DXT5_RA_AS_RG`"]
    pub const DXT5_RA_AS_RG: Format = Format {
        ord: 34i32
    };
    #[doc(alias = "FORMAT_ASTC_4x4")]
    #[doc = "Godot enumerator name: `FORMAT_ASTC_4x4`"]
    pub const ASTC_4x4: Format = Format {
        ord: 35i32
    };
    #[doc(alias = "FORMAT_ASTC_4x4_HDR")]
    #[doc = "Godot enumerator name: `FORMAT_ASTC_4x4_HDR`"]
    pub const ASTC_4x4_HDR: Format = Format {
        ord: 36i32
    };
    #[doc(alias = "FORMAT_ASTC_8x8")]
    #[doc = "Godot enumerator name: `FORMAT_ASTC_8x8`"]
    pub const ASTC_8x8: Format = Format {
        ord: 37i32
    };
    #[doc(alias = "FORMAT_ASTC_8x8_HDR")]
    #[doc = "Godot enumerator name: `FORMAT_ASTC_8x8_HDR`"]
    pub const ASTC_8x8_HDR: Format = Format {
        ord: 38i32
    };
    #[doc(alias = "FORMAT_MAX")]
    #[doc = "Godot enumerator name: `FORMAT_MAX`"]
    pub const MAX: Format = Format {
        ord: 39i32
    };
    
}
impl std::fmt::Debug for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Format") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Format {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 37i32 | ord @ 38i32 | ord @ 39i32 => Some(Self {
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
            Self::L8 => "L8", Self::LA8 => "LA8", Self::R8 => "R8", Self::RG8 => "RG8", Self::RGB8 => "RGB8", Self::RGBA8 => "RGBA8", Self::RGBA4444 => "RGBA4444", Self::RGB565 => "RGB565", Self::RF => "RF", Self::RGF => "RGF", Self::RGBF => "RGBF", Self::RGBAF => "RGBAF", Self::RH => "RH", Self::RGH => "RGH", Self::RGBH => "RGBH", Self::RGBAH => "RGBAH", Self::RGBE9995 => "RGBE9995", Self::DXT1 => "DXT1", Self::DXT3 => "DXT3", Self::DXT5 => "DXT5", Self::RGTC_R => "RGTC_R", Self::RGTC_RG => "RGTC_RG", Self::BPTC_RGBA => "BPTC_RGBA", Self::BPTC_RGBF => "BPTC_RGBF", Self::BPTC_RGBFU => "BPTC_RGBFU", Self::ETC => "ETC", Self::ETC2_R11 => "ETC2_R11", Self::ETC2_R11S => "ETC2_R11S", Self::ETC2_RG11 => "ETC2_RG11", Self::ETC2_RG11S => "ETC2_RG11S", Self::ETC2_RGB8 => "ETC2_RGB8", Self::ETC2_RGBA8 => "ETC2_RGBA8", Self::ETC2_RGB8A1 => "ETC2_RGB8A1", Self::ETC2_RA_AS_RG => "ETC2_RA_AS_RG", Self::DXT5_RA_AS_RG => "DXT5_RA_AS_RG", Self::ASTC_4x4 => "ASTC_4x4", Self::ASTC_4x4_HDR => "ASTC_4x4_HDR", Self::ASTC_8x8 => "ASTC_8x8", Self::ASTC_8x8_HDR => "ASTC_8x8_HDR", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Format::L8, Format::LA8, Format::R8, Format::RG8, Format::RGB8, Format::RGBA8, Format::RGBA4444, Format::RGB565, Format::RF, Format::RGF, Format::RGBF, Format::RGBAF, Format::RH, Format::RGH, Format::RGBH, Format::RGBAH, Format::RGBE9995, Format::DXT1, Format::DXT3, Format::DXT5, Format::RGTC_R, Format::RGTC_RG, Format::BPTC_RGBA, Format::BPTC_RGBF, Format::BPTC_RGBFU, Format::ETC, Format::ETC2_R11, Format::ETC2_R11S, Format::ETC2_RG11, Format::ETC2_RG11S, Format::ETC2_RGB8, Format::ETC2_RGBA8, Format::ETC2_RGB8A1, Format::ETC2_RA_AS_RG, Format::DXT5_RA_AS_RG, Format::ASTC_4x4, Format::ASTC_4x4_HDR, Format::ASTC_8x8, Format::ASTC_8x8_HDR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Format >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("L8", "FORMAT_L8", Format::L8), crate::meta::inspect::EnumConstant::new("LA8", "FORMAT_LA8", Format::LA8), crate::meta::inspect::EnumConstant::new("R8", "FORMAT_R8", Format::R8), crate::meta::inspect::EnumConstant::new("RG8", "FORMAT_RG8", Format::RG8), crate::meta::inspect::EnumConstant::new("RGB8", "FORMAT_RGB8", Format::RGB8), crate::meta::inspect::EnumConstant::new("RGBA8", "FORMAT_RGBA8", Format::RGBA8), crate::meta::inspect::EnumConstant::new("RGBA4444", "FORMAT_RGBA4444", Format::RGBA4444), crate::meta::inspect::EnumConstant::new("RGB565", "FORMAT_RGB565", Format::RGB565), crate::meta::inspect::EnumConstant::new("RF", "FORMAT_RF", Format::RF), crate::meta::inspect::EnumConstant::new("RGF", "FORMAT_RGF", Format::RGF), crate::meta::inspect::EnumConstant::new("RGBF", "FORMAT_RGBF", Format::RGBF), crate::meta::inspect::EnumConstant::new("RGBAF", "FORMAT_RGBAF", Format::RGBAF), crate::meta::inspect::EnumConstant::new("RH", "FORMAT_RH", Format::RH), crate::meta::inspect::EnumConstant::new("RGH", "FORMAT_RGH", Format::RGH), crate::meta::inspect::EnumConstant::new("RGBH", "FORMAT_RGBH", Format::RGBH), crate::meta::inspect::EnumConstant::new("RGBAH", "FORMAT_RGBAH", Format::RGBAH), crate::meta::inspect::EnumConstant::new("RGBE9995", "FORMAT_RGBE9995", Format::RGBE9995), crate::meta::inspect::EnumConstant::new("DXT1", "FORMAT_DXT1", Format::DXT1), crate::meta::inspect::EnumConstant::new("DXT3", "FORMAT_DXT3", Format::DXT3), crate::meta::inspect::EnumConstant::new("DXT5", "FORMAT_DXT5", Format::DXT5), crate::meta::inspect::EnumConstant::new("RGTC_R", "FORMAT_RGTC_R", Format::RGTC_R), crate::meta::inspect::EnumConstant::new("RGTC_RG", "FORMAT_RGTC_RG", Format::RGTC_RG), crate::meta::inspect::EnumConstant::new("BPTC_RGBA", "FORMAT_BPTC_RGBA", Format::BPTC_RGBA), crate::meta::inspect::EnumConstant::new("BPTC_RGBF", "FORMAT_BPTC_RGBF", Format::BPTC_RGBF), crate::meta::inspect::EnumConstant::new("BPTC_RGBFU", "FORMAT_BPTC_RGBFU", Format::BPTC_RGBFU), crate::meta::inspect::EnumConstant::new("ETC", "FORMAT_ETC", Format::ETC), crate::meta::inspect::EnumConstant::new("ETC2_R11", "FORMAT_ETC2_R11", Format::ETC2_R11), crate::meta::inspect::EnumConstant::new("ETC2_R11S", "FORMAT_ETC2_R11S", Format::ETC2_R11S), crate::meta::inspect::EnumConstant::new("ETC2_RG11", "FORMAT_ETC2_RG11", Format::ETC2_RG11), crate::meta::inspect::EnumConstant::new("ETC2_RG11S", "FORMAT_ETC2_RG11S", Format::ETC2_RG11S), crate::meta::inspect::EnumConstant::new("ETC2_RGB8", "FORMAT_ETC2_RGB8", Format::ETC2_RGB8), crate::meta::inspect::EnumConstant::new("ETC2_RGBA8", "FORMAT_ETC2_RGBA8", Format::ETC2_RGBA8), crate::meta::inspect::EnumConstant::new("ETC2_RGB8A1", "FORMAT_ETC2_RGB8A1", Format::ETC2_RGB8A1), crate::meta::inspect::EnumConstant::new("ETC2_RA_AS_RG", "FORMAT_ETC2_RA_AS_RG", Format::ETC2_RA_AS_RG), crate::meta::inspect::EnumConstant::new("DXT5_RA_AS_RG", "FORMAT_DXT5_RA_AS_RG", Format::DXT5_RA_AS_RG), crate::meta::inspect::EnumConstant::new("ASTC_4x4", "FORMAT_ASTC_4x4", Format::ASTC_4x4), crate::meta::inspect::EnumConstant::new("ASTC_4x4_HDR", "FORMAT_ASTC_4x4_HDR", Format::ASTC_4x4_HDR), crate::meta::inspect::EnumConstant::new("ASTC_8x8", "FORMAT_ASTC_8x8", Format::ASTC_8x8), crate::meta::inspect::EnumConstant::new("ASTC_8x8_HDR", "FORMAT_ASTC_8x8_HDR", Format::ASTC_8x8_HDR), crate::meta::inspect::EnumConstant::new("MAX", "FORMAT_MAX", Format::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Format {
    const ENUMERATOR_COUNT: usize = 39usize;
    
}
impl crate::meta::GodotConvert for Format {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Format {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Format {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Interpolation {
    ord: i32
}
impl Interpolation {
    #[doc(alias = "INTERPOLATE_NEAREST")]
    #[doc = "Godot enumerator name: `INTERPOLATE_NEAREST`"]
    pub const NEAREST: Interpolation = Interpolation {
        ord: 0i32
    };
    #[doc(alias = "INTERPOLATE_BILINEAR")]
    #[doc = "Godot enumerator name: `INTERPOLATE_BILINEAR`"]
    pub const BILINEAR: Interpolation = Interpolation {
        ord: 1i32
    };
    #[doc(alias = "INTERPOLATE_CUBIC")]
    #[doc = "Godot enumerator name: `INTERPOLATE_CUBIC`"]
    pub const CUBIC: Interpolation = Interpolation {
        ord: 2i32
    };
    #[doc(alias = "INTERPOLATE_TRILINEAR")]
    #[doc = "Godot enumerator name: `INTERPOLATE_TRILINEAR`"]
    pub const TRILINEAR: Interpolation = Interpolation {
        ord: 3i32
    };
    #[doc(alias = "INTERPOLATE_LANCZOS")]
    #[doc = "Godot enumerator name: `INTERPOLATE_LANCZOS`"]
    pub const LANCZOS: Interpolation = Interpolation {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for Interpolation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Interpolation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Interpolation {
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
            Self::NEAREST => "NEAREST", Self::BILINEAR => "BILINEAR", Self::CUBIC => "CUBIC", Self::TRILINEAR => "TRILINEAR", Self::LANCZOS => "LANCZOS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Interpolation::NEAREST, Interpolation::BILINEAR, Interpolation::CUBIC, Interpolation::TRILINEAR, Interpolation::LANCZOS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Interpolation >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEAREST", "INTERPOLATE_NEAREST", Interpolation::NEAREST), crate::meta::inspect::EnumConstant::new("BILINEAR", "INTERPOLATE_BILINEAR", Interpolation::BILINEAR), crate::meta::inspect::EnumConstant::new("CUBIC", "INTERPOLATE_CUBIC", Interpolation::CUBIC), crate::meta::inspect::EnumConstant::new("TRILINEAR", "INTERPOLATE_TRILINEAR", Interpolation::TRILINEAR), crate::meta::inspect::EnumConstant::new("LANCZOS", "INTERPOLATE_LANCZOS", Interpolation::LANCZOS)]
        }
    }
}
impl crate::meta::GodotConvert for Interpolation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Interpolation {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Interpolation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct AlphaMode {
    ord: i32
}
impl AlphaMode {
    #[doc(alias = "ALPHA_NONE")]
    #[doc = "Godot enumerator name: `ALPHA_NONE`"]
    pub const NONE: AlphaMode = AlphaMode {
        ord: 0i32
    };
    #[doc(alias = "ALPHA_BIT")]
    #[doc = "Godot enumerator name: `ALPHA_BIT`"]
    pub const BIT: AlphaMode = AlphaMode {
        ord: 1i32
    };
    #[doc(alias = "ALPHA_BLEND")]
    #[doc = "Godot enumerator name: `ALPHA_BLEND`"]
    pub const BLEND: AlphaMode = AlphaMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for AlphaMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AlphaMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AlphaMode {
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
            Self::NONE => "NONE", Self::BIT => "BIT", Self::BLEND => "BLEND", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AlphaMode::NONE, AlphaMode::BIT, AlphaMode::BLEND]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AlphaMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "ALPHA_NONE", AlphaMode::NONE), crate::meta::inspect::EnumConstant::new("BIT", "ALPHA_BIT", AlphaMode::BIT), crate::meta::inspect::EnumConstant::new("BLEND", "ALPHA_BLEND", AlphaMode::BLEND)]
        }
    }
}
impl crate::meta::GodotConvert for AlphaMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AlphaMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AlphaMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompressMode {
    ord: i32
}
impl CompressMode {
    #[doc(alias = "COMPRESS_S3TC")]
    #[doc = "Godot enumerator name: `COMPRESS_S3TC`"]
    pub const S3TC: CompressMode = CompressMode {
        ord: 0i32
    };
    #[doc(alias = "COMPRESS_ETC")]
    #[doc = "Godot enumerator name: `COMPRESS_ETC`"]
    pub const ETC: CompressMode = CompressMode {
        ord: 1i32
    };
    #[doc(alias = "COMPRESS_ETC2")]
    #[doc = "Godot enumerator name: `COMPRESS_ETC2`"]
    pub const ETC2: CompressMode = CompressMode {
        ord: 2i32
    };
    #[doc(alias = "COMPRESS_BPTC")]
    #[doc = "Godot enumerator name: `COMPRESS_BPTC`"]
    pub const BPTC: CompressMode = CompressMode {
        ord: 3i32
    };
    #[doc(alias = "COMPRESS_ASTC")]
    #[doc = "Godot enumerator name: `COMPRESS_ASTC`"]
    pub const ASTC: CompressMode = CompressMode {
        ord: 4i32
    };
    #[doc(alias = "COMPRESS_MAX")]
    #[doc = "Godot enumerator name: `COMPRESS_MAX`"]
    pub const MAX: CompressMode = CompressMode {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for CompressMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CompressMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CompressMode {
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
            Self::S3TC => "S3TC", Self::ETC => "ETC", Self::ETC2 => "ETC2", Self::BPTC => "BPTC", Self::ASTC => "ASTC", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CompressMode::S3TC, CompressMode::ETC, CompressMode::ETC2, CompressMode::BPTC, CompressMode::ASTC]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CompressMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("S3TC", "COMPRESS_S3TC", CompressMode::S3TC), crate::meta::inspect::EnumConstant::new("ETC", "COMPRESS_ETC", CompressMode::ETC), crate::meta::inspect::EnumConstant::new("ETC2", "COMPRESS_ETC2", CompressMode::ETC2), crate::meta::inspect::EnumConstant::new("BPTC", "COMPRESS_BPTC", CompressMode::BPTC), crate::meta::inspect::EnumConstant::new("ASTC", "COMPRESS_ASTC", CompressMode::ASTC), crate::meta::inspect::EnumConstant::new("MAX", "COMPRESS_MAX", CompressMode::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CompressMode {
    const ENUMERATOR_COUNT: usize = 5usize;
    
}
impl crate::meta::GodotConvert for CompressMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CompressMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CompressMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct UsedChannels {
    ord: i32
}
impl UsedChannels {
    #[doc(alias = "USED_CHANNELS_L")]
    #[doc = "Godot enumerator name: `USED_CHANNELS_L`"]
    pub const L: UsedChannels = UsedChannels {
        ord: 0i32
    };
    #[doc(alias = "USED_CHANNELS_LA")]
    #[doc = "Godot enumerator name: `USED_CHANNELS_LA`"]
    pub const LA: UsedChannels = UsedChannels {
        ord: 1i32
    };
    #[doc(alias = "USED_CHANNELS_R")]
    #[doc = "Godot enumerator name: `USED_CHANNELS_R`"]
    pub const R: UsedChannels = UsedChannels {
        ord: 2i32
    };
    #[doc(alias = "USED_CHANNELS_RG")]
    #[doc = "Godot enumerator name: `USED_CHANNELS_RG`"]
    pub const RG: UsedChannels = UsedChannels {
        ord: 3i32
    };
    #[doc(alias = "USED_CHANNELS_RGB")]
    #[doc = "Godot enumerator name: `USED_CHANNELS_RGB`"]
    pub const RGB: UsedChannels = UsedChannels {
        ord: 4i32
    };
    #[doc(alias = "USED_CHANNELS_RGBA")]
    #[doc = "Godot enumerator name: `USED_CHANNELS_RGBA`"]
    pub const RGBA: UsedChannels = UsedChannels {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for UsedChannels {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("UsedChannels") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for UsedChannels {
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
            Self::L => "L", Self::LA => "LA", Self::R => "R", Self::RG => "RG", Self::RGB => "RGB", Self::RGBA => "RGBA", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[UsedChannels::L, UsedChannels::LA, UsedChannels::R, UsedChannels::RG, UsedChannels::RGB, UsedChannels::RGBA]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < UsedChannels >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("L", "USED_CHANNELS_L", UsedChannels::L), crate::meta::inspect::EnumConstant::new("LA", "USED_CHANNELS_LA", UsedChannels::LA), crate::meta::inspect::EnumConstant::new("R", "USED_CHANNELS_R", UsedChannels::R), crate::meta::inspect::EnumConstant::new("RG", "USED_CHANNELS_RG", UsedChannels::RG), crate::meta::inspect::EnumConstant::new("RGB", "USED_CHANNELS_RGB", UsedChannels::RGB), crate::meta::inspect::EnumConstant::new("RGBA", "USED_CHANNELS_RGBA", UsedChannels::RGBA)]
        }
    }
}
impl crate::meta::GodotConvert for UsedChannels {
    type Via = i32;
    
}
impl crate::meta::ToGodot for UsedChannels {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for UsedChannels {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CompressSource {
    ord: i32
}
impl CompressSource {
    #[doc(alias = "COMPRESS_SOURCE_GENERIC")]
    #[doc = "Godot enumerator name: `COMPRESS_SOURCE_GENERIC`"]
    pub const GENERIC: CompressSource = CompressSource {
        ord: 0i32
    };
    #[doc(alias = "COMPRESS_SOURCE_SRGB")]
    #[doc = "Godot enumerator name: `COMPRESS_SOURCE_SRGB`"]
    pub const SRGB: CompressSource = CompressSource {
        ord: 1i32
    };
    #[doc(alias = "COMPRESS_SOURCE_NORMAL")]
    #[doc = "Godot enumerator name: `COMPRESS_SOURCE_NORMAL`"]
    pub const NORMAL: CompressSource = CompressSource {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for CompressSource {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CompressSource") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CompressSource {
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
            Self::GENERIC => "GENERIC", Self::SRGB => "SRGB", Self::NORMAL => "NORMAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CompressSource::GENERIC, CompressSource::SRGB, CompressSource::NORMAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CompressSource >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("GENERIC", "COMPRESS_SOURCE_GENERIC", CompressSource::GENERIC), crate::meta::inspect::EnumConstant::new("SRGB", "COMPRESS_SOURCE_SRGB", CompressSource::SRGB), crate::meta::inspect::EnumConstant::new("NORMAL", "COMPRESS_SOURCE_NORMAL", CompressSource::NORMAL)]
        }
    }
}
impl crate::meta::GodotConvert for CompressSource {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CompressSource {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CompressSource {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `ASTCFormat`."]
pub struct AstcFormat {
    ord: i32
}
impl AstcFormat {
    #[doc(alias = "ASTC_FORMAT_4x4")]
    #[doc = "Godot enumerator name: `ASTC_FORMAT_4x4`"]
    pub const FORMAT_4x4: AstcFormat = AstcFormat {
        ord: 0i32
    };
    #[doc(alias = "ASTC_FORMAT_8x8")]
    #[doc = "Godot enumerator name: `ASTC_FORMAT_8x8`"]
    pub const FORMAT_8x8: AstcFormat = AstcFormat {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for AstcFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("AstcFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for AstcFormat {
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
            Self::FORMAT_4x4 => "FORMAT_4x4", Self::FORMAT_8x8 => "FORMAT_8x8", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[AstcFormat::FORMAT_4x4, AstcFormat::FORMAT_8x8]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < AstcFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FORMAT_4x4", "ASTC_FORMAT_4x4", AstcFormat::FORMAT_4x4), crate::meta::inspect::EnumConstant::new("FORMAT_8x8", "ASTC_FORMAT_8x8", AstcFormat::FORMAT_8x8)]
        }
    }
}
impl crate::meta::GodotConvert for AstcFormat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for AstcFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for AstcFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Image;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Image {
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