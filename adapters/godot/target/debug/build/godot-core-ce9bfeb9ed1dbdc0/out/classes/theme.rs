#![doc = "Sidecar module for class [`Theme`][crate::classes::Theme].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Theme` enums](https://docs.godotengine.org/en/stable/classes/class_theme.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Theme.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`theme`][crate::classes::theme]: sidecar module with related enum/flag types\n* [`ITheme`][crate::classes::ITheme]: virtual methods\n\n\nSee also [Godot docs for `Theme`](https://docs.godotengine.org/en/stable/classes/class_theme.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Theme::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Theme {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Theme`][crate::classes::Theme].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Theme` methods](https://docs.godotengine.org/en/stable/classes/class_theme.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITheme: crate::obj::GodotClass < Base = Theme > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Theme {
        pub fn set_icon(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (name.into_arg(), theme_type.into_arg(), texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9581usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_icon(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9582usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_icon(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9583usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rename_icon(&mut self, old_name: impl AsArg < StringName >, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >,);
            let args = (old_name.into_arg(), name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9584usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "rename_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_icon(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9585usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_icon_list(&self, theme_type: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9586usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_icon_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_icon_type_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9587usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_icon_type_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stylebox(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >, texture: impl AsArg < Option < Gd < crate::classes::StyleBox >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, Option < Gd < crate::classes::StyleBox > > >,);
            let args = (name.into_arg(), theme_type.into_arg(), texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9588usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stylebox(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> Option < Gd < crate::classes::StyleBox > > {
            type CallRet = Option < Gd < crate::classes::StyleBox > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9589usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_stylebox(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9590usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rename_stylebox(&mut self, old_name: impl AsArg < StringName >, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >,);
            let args = (old_name.into_arg(), name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9591usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "rename_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_stylebox(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9592usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear_stylebox", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stylebox_list(&self, theme_type: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9593usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_stylebox_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_stylebox_type_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9594usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_stylebox_type_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >, font: impl AsArg < Option < Gd < crate::classes::Font >> >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, Option < Gd < crate::classes::Font > > >,);
            let args = (name.into_arg(), theme_type.into_arg(), font.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9595usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Font > > {
            type CallRet = Option < Gd < crate::classes::Font > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9596usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_font(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9597usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rename_font(&mut self, old_name: impl AsArg < StringName >, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >,);
            let args = (old_name.into_arg(), name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9598usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "rename_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_font(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9599usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font_list(&self, theme_type: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9600usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_font_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font_type_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9601usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_font_type_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_font_size(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >, font_size: i32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, i32,);
            let args = (name.into_arg(), theme_type.into_arg(), font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9602usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font_size(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9603usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_font_size(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9604usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rename_font_size(&mut self, old_name: impl AsArg < StringName >, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >,);
            let args = (old_name.into_arg(), name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9605usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "rename_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_font_size(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9606usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font_size_list(&self, theme_type: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9607usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_font_size_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_font_size_type_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9608usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_font_size_type_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >, color: Color,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, Color,);
            let args = (name.into_arg(), theme_type.into_arg(), color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9609usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> Color {
            type CallRet = Color;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9610usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_color(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9611usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rename_color(&mut self, old_name: impl AsArg < StringName >, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >,);
            let args = (old_name.into_arg(), name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9612usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "rename_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_color(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9613usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color_list(&self, theme_type: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9614usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_color_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color_type_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9615usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_color_type_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_constant(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >, constant: i32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, i32,);
            let args = (name.into_arg(), theme_type.into_arg(), constant,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9616usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_constant", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_constant(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9617usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_constant", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_constant(&self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9618usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_constant", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rename_constant(&mut self, old_name: impl AsArg < StringName >, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >,);
            let args = (old_name.into_arg(), name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9619usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "rename_constant", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_constant(&mut self, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9620usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear_constant", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_constant_list(&self, theme_type: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9621usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_constant_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_constant_type_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9622usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_constant_type_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_default_base_scale(&mut self, base_scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (base_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9623usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_default_base_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_default_base_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9624usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_default_base_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_default_base_scale(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9625usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_default_base_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_default_font(&mut self, font: impl AsArg < Option < Gd < crate::classes::Font >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Font > > >,);
            let args = (font.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9626usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_default_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_default_font(&self,) -> Option < Gd < crate::classes::Font > > {
            type CallRet = Option < Gd < crate::classes::Font > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9627usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_default_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_default_font(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9628usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_default_font", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_default_font_size(&mut self, font_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (font_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9629usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_default_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_default_font_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9630usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_default_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_default_font_size(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9631usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_default_font_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_theme_item(&mut self, data_type: crate::classes::theme::DataType, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (crate::classes::theme::DataType, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, RefArg < 'a2, Variant >,);
            let args = (data_type, name.into_arg(), theme_type.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9632usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_theme_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_theme_item(&self, data_type: crate::classes::theme::DataType, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, > = (crate::classes::theme::DataType, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (data_type, name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9633usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_theme_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_theme_item(&self, data_type: crate::classes::theme::DataType, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (crate::classes::theme::DataType, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (data_type, name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9634usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "has_theme_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rename_theme_item(&mut self, data_type: crate::classes::theme::DataType, old_name: impl AsArg < StringName >, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (crate::classes::theme::DataType, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >, CowArg < 'a2, StringName >,);
            let args = (data_type, old_name.into_arg(), name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9635usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "rename_theme_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_theme_item(&mut self, data_type: crate::classes::theme::DataType, name: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (crate::classes::theme::DataType, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (data_type, name.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9636usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear_theme_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_theme_item_list(&self, data_type: crate::classes::theme::DataType, theme_type: impl AsArg < GString >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (crate::classes::theme::DataType, CowArg < 'a0, GString >,);
            let args = (data_type, theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9637usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_theme_item_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_theme_item_type_list(&self, data_type: crate::classes::theme::DataType,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = (crate::classes::theme::DataType,);
            let args = (data_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9638usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_theme_item_type_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_type_variation(&mut self, theme_type: impl AsArg < StringName >, base_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (theme_type.into_arg(), base_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9639usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "set_type_variation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_type_variation(&self, theme_type: impl AsArg < StringName >, base_type: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (theme_type.into_arg(), base_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9640usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "is_type_variation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_type_variation(&mut self, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9641usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear_type_variation", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_type_variation_base(&self, theme_type: impl AsArg < StringName >,) -> StringName {
            type CallRet = StringName;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9642usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_type_variation_base", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_type_variation_list(&self, base_type: impl AsArg < StringName >,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (base_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9643usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_type_variation_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_type(&mut self, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9644usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "add_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_type(&mut self, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9645usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "remove_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn rename_type(&mut self, old_theme_type: impl AsArg < StringName >, theme_type: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (old_theme_type.into_arg(), theme_type.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9646usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "rename_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_type_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9647usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "get_type_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn merge_with(&mut self, other: impl AsArg < Option < Gd < crate::classes::Theme >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Theme > > >,);
            let args = (other.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9648usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "merge_with", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9649usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Theme", "clear", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Theme {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Theme"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Theme {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Theme {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Theme {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Theme {
        
    }
    impl crate::obj::cap::GodotDefault for Theme {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Theme {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Theme {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Theme`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Theme__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Theme > for $Class {
                
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
pub struct DataType {
    ord: i32
}
impl DataType {
    #[doc(alias = "DATA_TYPE_COLOR")]
    #[doc = "Godot enumerator name: `DATA_TYPE_COLOR`"]
    pub const COLOR: DataType = DataType {
        ord: 0i32
    };
    #[doc(alias = "DATA_TYPE_CONSTANT")]
    #[doc = "Godot enumerator name: `DATA_TYPE_CONSTANT`"]
    pub const CONSTANT: DataType = DataType {
        ord: 1i32
    };
    #[doc(alias = "DATA_TYPE_FONT")]
    #[doc = "Godot enumerator name: `DATA_TYPE_FONT`"]
    pub const FONT: DataType = DataType {
        ord: 2i32
    };
    #[doc(alias = "DATA_TYPE_FONT_SIZE")]
    #[doc = "Godot enumerator name: `DATA_TYPE_FONT_SIZE`"]
    pub const FONT_SIZE: DataType = DataType {
        ord: 3i32
    };
    #[doc(alias = "DATA_TYPE_ICON")]
    #[doc = "Godot enumerator name: `DATA_TYPE_ICON`"]
    pub const ICON: DataType = DataType {
        ord: 4i32
    };
    #[doc(alias = "DATA_TYPE_STYLEBOX")]
    #[doc = "Godot enumerator name: `DATA_TYPE_STYLEBOX`"]
    pub const STYLEBOX: DataType = DataType {
        ord: 5i32
    };
    #[doc(alias = "DATA_TYPE_MAX")]
    #[doc = "Godot enumerator name: `DATA_TYPE_MAX`"]
    pub const MAX: DataType = DataType {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DataType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DataType {
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
            Self::COLOR => "COLOR", Self::CONSTANT => "CONSTANT", Self::FONT => "FONT", Self::FONT_SIZE => "FONT_SIZE", Self::ICON => "ICON", Self::STYLEBOX => "STYLEBOX", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DataType::COLOR, DataType::CONSTANT, DataType::FONT, DataType::FONT_SIZE, DataType::ICON, DataType::STYLEBOX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DataType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("COLOR", "DATA_TYPE_COLOR", DataType::COLOR), crate::meta::inspect::EnumConstant::new("CONSTANT", "DATA_TYPE_CONSTANT", DataType::CONSTANT), crate::meta::inspect::EnumConstant::new("FONT", "DATA_TYPE_FONT", DataType::FONT), crate::meta::inspect::EnumConstant::new("FONT_SIZE", "DATA_TYPE_FONT_SIZE", DataType::FONT_SIZE), crate::meta::inspect::EnumConstant::new("ICON", "DATA_TYPE_ICON", DataType::ICON), crate::meta::inspect::EnumConstant::new("STYLEBOX", "DATA_TYPE_STYLEBOX", DataType::STYLEBOX), crate::meta::inspect::EnumConstant::new("MAX", "DATA_TYPE_MAX", DataType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for DataType {
    const ENUMERATOR_COUNT: usize = 6usize;
    
}
impl crate::meta::GodotConvert for DataType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DataType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DataType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Theme;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Theme {
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