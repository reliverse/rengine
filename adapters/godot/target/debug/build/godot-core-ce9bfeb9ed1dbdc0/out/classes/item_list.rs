#![doc = "Sidecar module for class [`ItemList`][crate::classes::ItemList].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ItemList` enums](https://docs.godotengine.org/en/stable/classes/class_itemlist.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ItemList.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`item_list`][crate::classes::item_list]: sidecar module with related enum/flag types\n* [`IItemList`][crate::classes::IItemList]: virtual methods\n* [`SignalsOfItemList`][crate::classes::item_list::SignalsOfItemList]: signal collection\n\n\nSee also [Godot docs for `ItemList`](https://docs.godotengine.org/en/stable/classes/class_itemlist.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`ItemList::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ItemList {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ItemList`][crate::classes::ItemList].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `ItemList` methods](https://docs.godotengine.org/en/stable/classes/class_itemlist.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IItemList: crate::obj::GodotClass < Base = ItemList > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ItemList {
        pub(crate) fn add_item_full(&mut self, text: CowArg < GString >, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >, selectable: bool,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >, bool,);
            let args = (text, icon, selectable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4599usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "add_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_item(&mut self, text: impl AsArg < GString >,) -> i32 {
            self.add_item_ex(text,) . done()
        }
        #[inline]
        pub fn add_item_ex < 'a > (&'a mut self, text: impl AsArg < GString > + 'a,) -> ExAddItem < 'a > {
            ExAddItem::new(self, text,)
        }
        pub(crate) fn add_icon_item_full(&mut self, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >, selectable: bool,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >, bool,);
            let args = (icon, selectable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4600usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "add_icon_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_icon_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_icon_item(&mut self, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) -> i32 {
            self.add_icon_item_ex(icon,) . done()
        }
        #[inline]
        pub fn add_icon_item_ex < 'a > (&'a mut self, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a,) -> ExAddIconItem < 'a > {
            ExAddIconItem::new(self, icon,)
        }
        pub fn set_item_text(&mut self, idx: i32, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (idx, text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4601usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_text(&self, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4602usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_icon(&mut self, idx: i32, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (idx, icon.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4603usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_icon(&self, idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4604usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_icon", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_text_direction(&mut self, idx: i32, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::control::TextDirection,);
            let args = (idx, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4605usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_text_direction(&self, idx: i32,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4606usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_text_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_language(&mut self, idx: i32, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (idx, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4607usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_language(&self, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4608usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_auto_translate_mode(&mut self, idx: i32, mode: crate::classes::node::AutoTranslateMode,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::node::AutoTranslateMode,);
            let args = (idx, mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4609usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_auto_translate_mode(&self, idx: i32,) -> crate::classes::node::AutoTranslateMode {
            type CallRet = crate::classes::node::AutoTranslateMode;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4610usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_auto_translate_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_icon_transposed(&mut self, idx: i32, transposed: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (idx, transposed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4611usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_icon_transposed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_item_icon_transposed(&self, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4612usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "is_item_icon_transposed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_icon_region(&mut self, idx: i32, rect: Rect2,) {
            type CallRet = ();
            type CallParams = (i32, Rect2,);
            let args = (idx, rect,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4613usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_icon_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_icon_region(&self, idx: i32,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4614usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_icon_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_icon_modulate(&mut self, idx: i32, modulate: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (idx, modulate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4615usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_icon_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_icon_modulate(&self, idx: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4616usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_icon_modulate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_selectable(&mut self, idx: i32, selectable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (idx, selectable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4617usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_selectable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_item_selectable(&self, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4618usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "is_item_selectable", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_disabled(&mut self, idx: i32, disabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (idx, disabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4619usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_item_disabled(&self, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4620usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "is_item_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_metadata(&mut self, idx: i32, metadata: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, Variant >,);
            let args = (idx, RefArg::new(metadata),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4621usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_metadata", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_metadata(&self, idx: i32,) -> Variant {
            type CallRet = Variant;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4622usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_metadata", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_custom_bg_color(&mut self, idx: i32, custom_bg_color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (idx, custom_bg_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4623usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_custom_bg_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_custom_bg_color(&self, idx: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4624usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_custom_bg_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_custom_fg_color(&mut self, idx: i32, custom_fg_color: Color,) {
            type CallRet = ();
            type CallParams = (i32, Color,);
            let args = (idx, custom_fg_color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4625usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_custom_fg_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_custom_fg_color(&self, idx: i32,) -> Color {
            type CallRet = Color;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4626usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_custom_fg_color", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_item_rect_full(&self, idx: i32, expand: bool,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = (i32, bool,);
            let args = (idx, expand,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4627usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_item_rect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_item_rect(&self, idx: i32,) -> Rect2 {
            self.get_item_rect_ex(idx,) . done()
        }
        #[inline]
        pub fn get_item_rect_ex < 'a > (&'a self, idx: i32,) -> ExGetItemRect < 'a > {
            ExGetItemRect::new(self, idx,)
        }
        pub fn set_item_tooltip_enabled(&mut self, idx: i32, enable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (idx, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4628usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_tooltip_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_item_tooltip_enabled(&self, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4629usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "is_item_tooltip_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_tooltip(&mut self, idx: i32, tooltip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (idx, tooltip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4630usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_tooltip(&self, idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4631usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn select_full(&mut self, idx: i32, single: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (idx, single,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4632usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "select", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::select_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn select(&mut self, idx: i32,) {
            self.select_ex(idx,) . done()
        }
        #[inline]
        pub fn select_ex < 'a > (&'a mut self, idx: i32,) -> ExSelect < 'a > {
            ExSelect::new(self, idx,)
        }
        pub fn deselect(&mut self, idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4633usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "deselect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn deselect_all(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4634usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "deselect_all", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_selected(&self, idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4635usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "is_selected", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selected_items(&mut self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4636usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_selected_items", Some(self.__validated_obj()), args,)
            }
        }
        pub fn move_item(&mut self, from_idx: i32, to_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (from_idx, to_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4637usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "move_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_item_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4638usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_item_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_item_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4639usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_item(&mut self, idx: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4640usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "remove_item", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4641usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub fn sort_items_by_text(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4642usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "sort_items_by_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fixed_column_width(&mut self, width: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4643usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_fixed_column_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fixed_column_width(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4644usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_fixed_column_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_same_column_width(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4645usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_same_column_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_same_column_width(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4646usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "is_same_column_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_text_lines(&mut self, lines: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (lines,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4647usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_max_text_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_text_lines(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4648usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_max_text_lines", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_columns(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4649usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_max_columns", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_columns(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4650usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_max_columns", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_select_mode(&mut self, mode: crate::classes::item_list::SelectMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::item_list::SelectMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4651usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_select_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_select_mode(&self,) -> crate::classes::item_list::SelectMode {
            type CallRet = crate::classes::item_list::SelectMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4652usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_select_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_icon_mode(&mut self, mode: crate::classes::item_list::IconMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::item_list::IconMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4653usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_icon_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_icon_mode(&self,) -> crate::classes::item_list::IconMode {
            type CallRet = crate::classes::item_list::IconMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4654usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_icon_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_fixed_icon_size(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4655usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_fixed_icon_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_fixed_icon_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4656usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_fixed_icon_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_icon_scale(&mut self, scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4657usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_icon_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_icon_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4658usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_icon_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_rmb_select(&mut self, allow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4659usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_allow_rmb_select", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_allow_rmb_select(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4660usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_allow_rmb_select", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_reselect(&mut self, allow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4661usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_allow_reselect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_allow_reselect(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4662usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_allow_reselect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_search(&mut self, allow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4663usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_allow_search", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_allow_search(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4664usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_allow_search", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_width(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4665usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_auto_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_auto_width(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4666usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "has_auto_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_height(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4667usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_auto_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_auto_height(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "has_auto_height", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_anything_selected(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4669usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "is_anything_selected", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_item_at_position_full(&self, position: Vector2, exact: bool,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2, bool,);
            let args = (position, exact,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_item_at_position", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_item_at_position_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_item_at_position(&self, position: Vector2,) -> i32 {
            self.get_item_at_position_ex(position,) . done()
        }
        #[inline]
        pub fn get_item_at_position_ex < 'a > (&'a self, position: Vector2,) -> ExGetItemAtPosition < 'a > {
            ExGetItemAtPosition::new(self, position,)
        }
        pub fn ensure_current_is_visible(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "ensure_current_is_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_v_scroll_bar(&mut self,) -> Option < Gd < crate::classes::VScrollBar > > {
            type CallRet = Option < Gd < crate::classes::VScrollBar > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_v_scroll_bar", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_h_scroll_bar(&mut self,) -> Option < Gd < crate::classes::HScrollBar > > {
            type CallRet = Option < Gd < crate::classes::HScrollBar > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4673usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_h_scroll_bar", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text_overrun_behavior(&mut self, overrun_behavior: crate::classes::text_server::OverrunBehavior,) {
            type CallRet = ();
            type CallParams = (crate::classes::text_server::OverrunBehavior,);
            let args = (overrun_behavior,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4674usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text_overrun_behavior(&self,) -> crate::classes::text_server::OverrunBehavior {
            type CallRet = crate::classes::text_server::OverrunBehavior;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4675usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "get_text_overrun_behavior", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_wraparound_items(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4676usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "set_wraparound_items", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_wraparound_items(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4677usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "has_wraparound_items", Some(self.__validated_obj()), args,)
            }
        }
        pub fn force_update_list_size(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4678usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ItemList", "force_update_list_size", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ItemList {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ItemList"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ItemList {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for ItemList {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for ItemList {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for ItemList {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ItemList {
        
    }
    impl crate::obj::cap::GodotDefault for ItemList {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ItemList {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ItemList {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ItemList`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ItemList__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ItemList > for $Class {
                
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
#[doc = "Default-param extender for [`ItemList::add_item_ex`][super::ItemList::add_item_ex]."]
#[must_use]
pub struct ExAddItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ItemList, text: CowArg < 'a, GString >, icon: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, selectable: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddItem < 'a > {
    fn new(surround_object: &'a mut re_export::ItemList, text: impl AsArg < GString > + 'a,) -> Self {
        let icon = Gd::null_arg();
        let selectable = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), icon: icon.into_arg(), selectable: selectable,
        }
    }
    #[inline]
    pub fn icon(self, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a) -> Self {
        Self {
            icon: icon.into_arg(), .. self
        }
    }
    #[inline]
    pub fn selectable(self, selectable: bool) -> Self {
        Self {
            selectable: selectable, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, text, icon, selectable,
        }
        = self;
        re_export::ItemList::add_item_full(surround_object, text, icon, selectable,)
    }
}
#[doc = "Default-param extender for [`ItemList::add_icon_item_ex`][super::ItemList::add_icon_item_ex]."]
#[must_use]
pub struct ExAddIconItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ItemList, icon: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >, selectable: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddIconItem < 'a > {
    fn new(surround_object: &'a mut re_export::ItemList, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a,) -> Self {
        let selectable = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, icon: icon.into_arg(), selectable: selectable,
        }
    }
    #[inline]
    pub fn selectable(self, selectable: bool) -> Self {
        Self {
            selectable: selectable, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, icon, selectable,
        }
        = self;
        re_export::ItemList::add_icon_item_full(surround_object, icon, selectable,)
    }
}
#[doc = "Default-param extender for [`ItemList::get_item_rect_ex`][super::ItemList::get_item_rect_ex]."]
#[must_use]
pub struct ExGetItemRect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ItemList, idx: i32, expand: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetItemRect < 'a > {
    fn new(surround_object: &'a re_export::ItemList, idx: i32,) -> Self {
        let expand = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, idx: idx, expand: expand,
        }
    }
    #[inline]
    pub fn expand(self, expand: bool) -> Self {
        Self {
            expand: expand, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rect2 {
        let Self {
            _phantom, surround_object, idx, expand,
        }
        = self;
        re_export::ItemList::get_item_rect_full(surround_object, idx, expand,)
    }
}
#[doc = "Default-param extender for [`ItemList::select_ex`][super::ItemList::select_ex]."]
#[must_use]
pub struct ExSelect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ItemList, idx: i32, single: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSelect < 'a > {
    fn new(surround_object: &'a mut re_export::ItemList, idx: i32,) -> Self {
        let single = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, idx: idx, single: single,
        }
    }
    #[inline]
    pub fn single(self, single: bool) -> Self {
        Self {
            single: single, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, idx, single,
        }
        = self;
        re_export::ItemList::select_full(surround_object, idx, single,)
    }
}
#[doc = "Default-param extender for [`ItemList::get_item_at_position_ex`][super::ItemList::get_item_at_position_ex]."]
#[must_use]
pub struct ExGetItemAtPosition < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::ItemList, position: Vector2, exact: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetItemAtPosition < 'a > {
    fn new(surround_object: &'a re_export::ItemList, position: Vector2,) -> Self {
        let exact = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, exact: exact,
        }
    }
    #[inline]
    pub fn exact(self, exact: bool) -> Self {
        Self {
            exact: exact, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, position, exact,
        }
        = self;
        re_export::ItemList::get_item_at_position_full(surround_object, position, exact,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct IconMode {
    ord: i32
}
impl IconMode {
    #[doc(alias = "ICON_MODE_TOP")]
    #[doc = "Godot enumerator name: `ICON_MODE_TOP`"]
    pub const TOP: IconMode = IconMode {
        ord: 0i32
    };
    #[doc(alias = "ICON_MODE_LEFT")]
    #[doc = "Godot enumerator name: `ICON_MODE_LEFT`"]
    pub const LEFT: IconMode = IconMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for IconMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("IconMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for IconMode {
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
            Self::TOP => "TOP", Self::LEFT => "LEFT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[IconMode::TOP, IconMode::LEFT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < IconMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TOP", "ICON_MODE_TOP", IconMode::TOP), crate::meta::inspect::EnumConstant::new("LEFT", "ICON_MODE_LEFT", IconMode::LEFT)]
        }
    }
}
impl crate::meta::GodotConvert for IconMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for IconMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for IconMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SelectMode {
    ord: i32
}
impl SelectMode {
    #[doc(alias = "SELECT_SINGLE")]
    #[doc = "Godot enumerator name: `SELECT_SINGLE`"]
    pub const SINGLE: SelectMode = SelectMode {
        ord: 0i32
    };
    #[doc(alias = "SELECT_MULTI")]
    #[doc = "Godot enumerator name: `SELECT_MULTI`"]
    pub const MULTI: SelectMode = SelectMode {
        ord: 1i32
    };
    #[doc(alias = "SELECT_TOGGLE")]
    #[doc = "Godot enumerator name: `SELECT_TOGGLE`"]
    pub const TOGGLE: SelectMode = SelectMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for SelectMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SelectMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SelectMode {
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
            Self::SINGLE => "SINGLE", Self::MULTI => "MULTI", Self::TOGGLE => "TOGGLE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SelectMode::SINGLE, SelectMode::MULTI, SelectMode::TOGGLE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SelectMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SINGLE", "SELECT_SINGLE", SelectMode::SINGLE), crate::meta::inspect::EnumConstant::new("MULTI", "SELECT_MULTI", SelectMode::MULTI), crate::meta::inspect::EnumConstant::new("TOGGLE", "SELECT_TOGGLE", SelectMode::TOGGLE)]
        }
    }
}
impl crate::meta::GodotConvert for SelectMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for SelectMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SelectMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ItemList;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`ItemList`][crate::classes::ItemList] class."]
    pub struct SignalsOfItemList < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfItemList < 'c, C > {
        #[doc = "Signature: `(index: i64)`"]
        pub fn item_selected(&mut self) -> SigItemSelected < 'c, C > {
            SigItemSelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "item_selected")
            }
        }
        #[doc = "Signature: `(at_position: Vector2, mouse_button_index: i64)`"]
        pub fn empty_clicked(&mut self) -> SigEmptyClicked < 'c, C > {
            SigEmptyClicked {
                typed: TypedSignal::extract(&mut self.__internal_obj, "empty_clicked")
            }
        }
        #[doc = "Signature: `(index: i64, at_position: Vector2, mouse_button_index: i64)`"]
        pub fn item_clicked(&mut self) -> SigItemClicked < 'c, C > {
            SigItemClicked {
                typed: TypedSignal::extract(&mut self.__internal_obj, "item_clicked")
            }
        }
        #[doc = "Signature: `(index: i64, selected: bool)`"]
        pub fn multi_selected(&mut self) -> SigMultiSelected < 'c, C > {
            SigMultiSelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "multi_selected")
            }
        }
        #[doc = "Signature: `(index: i64)`"]
        pub fn item_activated(&mut self) -> SigItemActivated < 'c, C > {
            SigItemActivated {
                typed: TypedSignal::extract(&mut self.__internal_obj, "item_activated")
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
    type TypedSigEmptyClicked < 'c, C > = TypedSignal < 'c, C, (Vector2, i64,) >;
    pub struct SigEmptyClicked < 'c, C: WithSignals > {
        typed: TypedSigEmptyClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigEmptyClicked < 'c, C > {
        pub fn emit(&mut self, at_position: Vector2, mouse_button_index: i64,) {
            self.typed.emit_tuple((at_position, mouse_button_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigEmptyClicked < 'c, C > {
        type Target = TypedSigEmptyClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigEmptyClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigItemClicked < 'c, C > = TypedSignal < 'c, C, (i64, Vector2, i64,) >;
    pub struct SigItemClicked < 'c, C: WithSignals > {
        typed: TypedSigItemClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemClicked < 'c, C > {
        pub fn emit(&mut self, index: i64, at_position: Vector2, mouse_button_index: i64,) {
            self.typed.emit_tuple((index, at_position, mouse_button_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemClicked < 'c, C > {
        type Target = TypedSigItemClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMultiSelected < 'c, C > = TypedSignal < 'c, C, (i64, bool,) >;
    pub struct SigMultiSelected < 'c, C: WithSignals > {
        typed: TypedSigMultiSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMultiSelected < 'c, C > {
        pub fn emit(&mut self, index: i64, selected: bool,) {
            self.typed.emit_tuple((index, selected,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigMultiSelected < 'c, C > {
        type Target = TypedSigMultiSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigMultiSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigItemActivated < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigItemActivated < 'c, C: WithSignals > {
        typed: TypedSigItemActivated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemActivated < 'c, C > {
        pub fn emit(&mut self, index: i64,) {
            self.typed.emit_tuple((index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemActivated < 'c, C > {
        type Target = TypedSigItemActivated < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemActivated < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for ItemList {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfItemList < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfItemList < 'c, C > {
        type Target = < < ItemList as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = ItemList;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfItemList < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = ItemList;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}