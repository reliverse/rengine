#![doc = "Sidecar module for class [`OptionButton`][crate::classes::OptionButton].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `OptionButton` enums](https://docs.godotengine.org/en/stable/classes/class_optionbutton.html#enumerations).\n\n"]
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
    #[doc = "Godot class `OptionButton.`\n\nInherits [`Button`][crate::classes::Button].\n\nRelated symbols:\n\n* [`option_button`][crate::classes::option_button]: sidecar module with related enum/flag types\n* [`IOptionButton`][crate::classes::IOptionButton]: virtual methods\n* [`SignalsOfOptionButton`][crate::classes::option_button::SignalsOfOptionButton]: signal collection\n\n\nSee also [Godot docs for `OptionButton`](https://docs.godotengine.org/en/stable/classes/class_optionbutton.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`OptionButton::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct OptionButton {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`OptionButton`][crate::classes::OptionButton].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IButton`][crate::classes::IButton] > [`IBaseButton`][crate::classes::IBaseButton] > [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `OptionButton` methods](https://docs.godotengine.org/en/stable/classes/class_optionbutton.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IOptionButton: crate::obj::GodotClass < Base = OptionButton > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn pressed(&mut self,) {
            unimplemented !()
        }
        fn toggled(&mut self, toggled_on: bool,) {
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
    impl OptionButton {
        pub(crate) fn add_item_full(&mut self, label: CowArg < GString >, id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (label, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6163usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "add_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_item(&mut self, label: impl AsArg < GString >,) {
            self.add_item_ex(label,) . done()
        }
        #[inline]
        pub fn add_item_ex < 'a > (&'a mut self, label: impl AsArg < GString > + 'a,) -> ExAddItem < 'a > {
            ExAddItem::new(self, label,)
        }
        pub(crate) fn add_icon_item_full(&mut self, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, label: CowArg < GString >, id: i32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, CowArg < 'a1, GString >, i32,);
            let args = (texture, label, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6164usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "add_icon_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_icon_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_icon_item(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >, label: impl AsArg < GString >,) {
            self.add_icon_item_ex(texture, label,) . done()
        }
        #[inline]
        pub fn add_icon_item_ex < 'a > (&'a mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, label: impl AsArg < GString > + 'a,) -> ExAddIconItem < 'a > {
            ExAddIconItem::new(self, texture, label,)
        }
        pub fn set_item_text(&mut self, idx: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (idx, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_icon(&mut self, idx: i32, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (idx, texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_disabled(&mut self, idx: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_id(&mut self, idx: i32, id: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (idx, id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_metadata(&mut self, idx: i32, metadata: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (idx, RefArg::new(metadata),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_metadata", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_tooltip(&mut self, idx: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (idx, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6170usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_auto_translate_mode(&mut self, idx: i32, mode: crate::classes::node::AutoTranslateMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::node::AutoTranslateMode,);
            let args = (idx, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6171usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_text(&self, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6172usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_icon(&self, idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6173usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_id(&self, idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6174usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_index(&self, id: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6175usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_index", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_metadata(&self, idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6176usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_metadata", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_tooltip(&self, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6177usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_auto_translate_mode(&self, idx: i32,) -> crate::classes::node::AutoTranslateMode {
            type CallRet = crate::classes::node::AutoTranslateMode;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6178usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_item_disabled(&self, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6179usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "is_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_item_separator(&self, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6180usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "is_item_separator", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_separator_full(&mut self, text: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6181usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "add_separator", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_separator_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_separator(&mut self,) {
            self.add_separator_ex() . done()
        }
        #[inline]
        pub fn add_separator_ex < 'a > (&'a mut self,) -> ExAddSeparator < 'a > {
            ExAddSeparator::new(self,)
        }
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6182usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn select(&mut self, idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6183usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "select", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selected(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6184usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_selected", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selected_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6185usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_selected_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selected_metadata(&self,) -> Variant {
            type CallRet = Variant;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6186usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_selected_metadata", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_item(&mut self, idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6187usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "remove_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_popup(&self,) -> Option < Gd < crate::classes::PopupMenu > > {
            type CallRet = Option < Gd < crate::classes::PopupMenu > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6188usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_popup", Some(self.__validated_obj()), args,)
            }
        }
        pub fn show_popup(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6189usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "show_popup", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6190usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_item_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6191usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_item_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_selectable_items(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6192usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "has_selectable_items", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_selectable_item_full(&self, from_last: bool,) -> i32 {
            type CallRet = i32;
            type CallParams = (bool,);
            let args = (from_last,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6193usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_selectable_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_selectable_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_selectable_item(&self,) -> i32 {
            self.get_selectable_item_ex() . done()
        }
        #[inline]
        pub fn get_selectable_item_ex < 'a > (&'a self,) -> ExGetSelectableItem < 'a > {
            ExGetSelectableItem::new(self,)
        }
        pub fn set_fit_to_longest_item(&mut self, fit: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (fit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6194usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_fit_to_longest_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_fit_to_longest_item(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6195usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "is_fit_to_longest_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_reselect(&mut self, allow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6196usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_allow_reselect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_allow_reselect(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6197usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "get_allow_reselect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_disable_shortcuts(&mut self, disabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6198usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "OptionButton", "set_disable_shortcuts", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for OptionButton {
        type Base = crate::classes::Button;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("OptionButton"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for OptionButton {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Button > for OptionButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::BaseButton > for OptionButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for OptionButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for OptionButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for OptionButton {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for OptionButton {
        
    }
    impl crate::obj::cap::GodotDefault for OptionButton {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for OptionButton {
        type Target = crate::classes::Button;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for OptionButton {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`OptionButton`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_OptionButton__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::OptionButton > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Button > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::BaseButton > for $Class {
                
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
#[doc = "Default-param extender for [`OptionButton::add_item_ex`][super::OptionButton::add_item_ex]."]
#[must_use]
pub struct ExAddItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::OptionButton, label: CowArg < 'a, GString >, id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddItem < 'a > {
    fn new(surround_object: &'a mut re_export::OptionButton, label: impl AsArg < GString > + 'a,) -> Self {
        let id = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), id: id,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, label, id,
        }
        = self;
        re_export::OptionButton::add_item_full(surround_object, label, id,)
    }
}
#[doc = "Default-param extender for [`OptionButton::add_icon_item_ex`][super::OptionButton::add_icon_item_ex]."]
#[must_use]
pub struct ExAddIconItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::OptionButton, texture: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, label: CowArg < 'a, GString >, id: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconItem < 'a > {
    fn new(surround_object: &'a mut re_export::OptionButton, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a, label: impl AsArg < GString > + 'a,) -> Self {
        let id = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, texture: texture.into_arg(), label: label.into_arg(), id: id,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, texture, label, id,
        }
        = self;
        re_export::OptionButton::add_icon_item_full(surround_object, texture, label, id,)
    }
}
#[doc = "Default-param extender for [`OptionButton::add_separator_ex`][super::OptionButton::add_separator_ex]."]
#[must_use]
pub struct ExAddSeparator < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::OptionButton, text: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSeparator < 'a > {
    fn new(surround_object: &'a mut re_export::OptionButton,) -> Self {
        let text = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: CowArg::Owned(text),
        }
    }
    #[inline]
    pub fn text(self, text: impl AsArg < GString > + 'a) -> Self {
        Self {
            text: text.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, text,
        }
        = self;
        re_export::OptionButton::add_separator_full(surround_object, text,)
    }
}
#[doc = "Default-param extender for [`OptionButton::get_selectable_item_ex`][super::OptionButton::get_selectable_item_ex]."]
#[must_use]
pub struct ExGetSelectableItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::OptionButton, from_last: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetSelectableItem < 'a > {
    fn new(surround_object: &'a re_export::OptionButton,) -> Self {
        let from_last = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_last: from_last,
        }
    }
    #[inline]
    pub fn from_last(self, from_last: bool) -> Self {
        Self {
            from_last: from_last, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, from_last,
        }
        = self;
        re_export::OptionButton::get_selectable_item_full(surround_object, from_last,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::OptionButton;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`OptionButton`][crate::classes::OptionButton] class."]
    pub struct SignalsOfOptionButton < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfOptionButton < 'c, C > {
        #[doc = "Signature: `(index: i64)`"]
        pub fn item_selected(&mut self) -> SigItemSelected < 'c, C > {
            SigItemSelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "item_selected")
            }
        }
        #[doc = "Signature: `(index: i64)`"]
        pub fn item_focused(&mut self) -> SigItemFocused < 'c, C > {
            SigItemFocused {
                typed: TypedSignal::extract(&mut self.__internal_obj, "item_focused")
            }
        }
    }
    type TypedSigItemSelected < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigItemSelected < 'c, C: WithSignals > {
        typed: TypedSigItemSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemSelected < 'c, C > {
        pub fn emit(&mut self, index: i64,) {
            self.typed.emit_tuple((index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemSelected < 'c, C > {
        type Target = TypedSigItemSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigItemFocused < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigItemFocused < 'c, C: WithSignals > {
        typed: TypedSigItemFocused < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemFocused < 'c, C > {
        pub fn emit(&mut self, index: i64,) {
            self.typed.emit_tuple((index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemFocused < 'c, C > {
        type Target = TypedSigItemFocused < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemFocused < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for OptionButton {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfOptionButton < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfOptionButton < 'c, C > {
        type Target = < < OptionButton as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = OptionButton;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfOptionButton < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = OptionButton;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}