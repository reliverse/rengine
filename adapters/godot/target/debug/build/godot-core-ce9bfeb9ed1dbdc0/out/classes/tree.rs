#![doc = "Sidecar module for class [`Tree`][crate::classes::Tree].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Tree` enums](https://docs.godotengine.org/en/stable/classes/class_tree.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Tree.`\n\nInherits [`Control`][crate::classes::Control].\n\nRelated symbols:\n\n* [`tree`][crate::classes::tree]: sidecar module with related enum/flag types\n* [`ITree`][crate::classes::ITree]: virtual methods\n* [`SignalsOfTree`][crate::classes::tree::SignalsOfTree]: signal collection\n\n\nSee also [Godot docs for `Tree`](https://docs.godotengine.org/en/stable/classes/class_tree.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Tree::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Tree {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Tree`][crate::classes::Tree].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Tree` methods](https://docs.godotengine.org/en/stable/classes/class_tree.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITree: crate::obj::GodotClass < Base = Tree > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Tree {
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10069usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "clear", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn create_item_full(&mut self, parent: CowArg < Option < Gd < crate::classes::TreeItem > > >, index: i32,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >, i32,);
            let args = (parent, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10070usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "create_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_item(&mut self,) -> Option < Gd < crate::classes::TreeItem > > {
            self.create_item_ex() . done()
        }
        #[inline]
        pub fn create_item_ex < 'a > (&'a mut self,) -> ExCreateItem < 'a > {
            ExCreateItem::new(self,)
        }
        pub fn get_root(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10071usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_root", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_column_custom_minimum_width(&mut self, column: i32, min_width: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (column, min_width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10072usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_custom_minimum_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_column_expand(&mut self, column: i32, expand: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, expand,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10073usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_expand", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_column_expand_ratio(&mut self, column: i32, ratio: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (column, ratio,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10074usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_expand_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_column_clip_content(&mut self, column: i32, enable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (column, enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10075usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_clip_content", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_column_expanding(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10076usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_column_expanding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_column_clipping_content(&self, column: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10077usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_column_clipping_content", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_column_expand_ratio(&self, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10078usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_column_expand_ratio", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_column_width(&self, column: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10079usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_column_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hide_root(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10080usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_hide_root", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_root_hidden(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10081usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_root_hidden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_next_selected(&mut self, from: impl AsArg < Option < Gd < crate::classes::TreeItem >> >,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >,);
            let args = (from.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10082usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_next_selected", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selected(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10083usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_selected", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_selected(&mut self, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> >, column: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >, i32,);
            let args = (item.into_arg(), column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10084usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_selected", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selected_column(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10085usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_selected_column", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pressed_button(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10086usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_pressed_button", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_select_mode(&mut self, mode: crate::classes::tree::SelectMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::tree::SelectMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10087usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_select_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_select_mode(&self,) -> crate::classes::tree::SelectMode {
            type CallRet = crate::classes::tree::SelectMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10088usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_select_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn deselect_all(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10089usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "deselect_all", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_columns(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10090usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_columns", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_columns(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10091usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_columns", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_edited(&self,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10092usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_edited", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_edited_column(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10093usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_edited_column", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn edit_selected_full(&mut self, force_edit: bool,) -> bool {
            type CallRet = bool;
            type CallParams = (bool,);
            let args = (force_edit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10094usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "edit_selected", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::edit_selected_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn edit_selected(&mut self,) -> bool {
            self.edit_selected_ex() . done()
        }
        #[inline]
        pub fn edit_selected_ex < 'a > (&'a mut self,) -> ExEditSelected < 'a > {
            ExEditSelected::new(self,)
        }
        pub fn get_custom_popup_rect(&self,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10095usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_custom_popup_rect", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_item_area_rect_full(&self, item: CowArg < Option < Gd < crate::classes::TreeItem > > >, column: i32, button_index: i32,) -> Rect2 {
            type CallRet = Rect2;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >, i32, i32,);
            let args = (item, column, button_index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10096usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_item_area_rect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_item_area_rect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_item_area_rect(&self, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> >,) -> Rect2 {
            self.get_item_area_rect_ex(item,) . done()
        }
        #[inline]
        pub fn get_item_area_rect_ex < 'a > (&'a self, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> > + 'a,) -> ExGetItemAreaRect < 'a > {
            ExGetItemAreaRect::new(self, item,)
        }
        pub fn get_item_at_position(&self, position: Vector2,) -> Option < Gd < crate::classes::TreeItem > > {
            type CallRet = Option < Gd < crate::classes::TreeItem > >;
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10097usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_item_at_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_column_at_position(&self, position: Vector2,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10098usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_column_at_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_drop_section_at_position(&self, position: Vector2,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10099usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_drop_section_at_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_button_id_at_position(&self, position: Vector2,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2,);
            let args = (position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_button_id_at_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn ensure_cursor_is_visible(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10101usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "ensure_cursor_is_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_column_titles_visible(&mut self, visible: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (visible,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10102usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_titles_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn are_column_titles_visible(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10103usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "are_column_titles_visible", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_column_title(&mut self, column: i32, title: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, title.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_title", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_column_title(&self, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_column_title", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_column_title_alignment(&mut self, column: i32, title_alignment: crate::global::HorizontalAlignment,) {
            type CallRet = ();
            type CallParams = (i32, crate::global::HorizontalAlignment,);
            let args = (column, title_alignment,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_title_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_column_title_alignment(&self, column: i32,) -> crate::global::HorizontalAlignment {
            type CallRet = crate::global::HorizontalAlignment;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_column_title_alignment", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_column_title_direction(&mut self, column: i32, direction: crate::classes::control::TextDirection,) {
            type CallRet = ();
            type CallParams = (i32, crate::classes::control::TextDirection,);
            let args = (column, direction,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_title_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_column_title_direction(&self, column: i32,) -> crate::classes::control::TextDirection {
            type CallRet = crate::classes::control::TextDirection;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_column_title_direction", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_column_title_language(&mut self, column: i32, language: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (column, language.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_column_title_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_column_title_language(&self, column: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (column,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_column_title_language", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scroll(&self,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_scroll", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn scroll_to_item_full(&mut self, item: CowArg < Option < Gd < crate::classes::TreeItem > > >, center_on_item: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TreeItem > > >, bool,);
            let args = (item, center_on_item,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "scroll_to_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::scroll_to_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn scroll_to_item(&mut self, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> >,) {
            self.scroll_to_item_ex(item,) . done()
        }
        #[inline]
        pub fn scroll_to_item_ex < 'a > (&'a mut self, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> > + 'a,) -> ExScrollToItem < 'a > {
            ExScrollToItem::new(self, item,)
        }
        pub fn set_h_scroll_enabled(&mut self, h_scroll: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (h_scroll,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_h_scroll_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_h_scroll_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_h_scroll_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_v_scroll_enabled(&mut self, h_scroll: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (h_scroll,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_v_scroll_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_v_scroll_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_v_scroll_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hide_folding(&mut self, hide: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (hide,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_hide_folding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_folding_hidden(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_folding_hidden", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_enable_recursive_folding(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_enable_recursive_folding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_recursive_folding_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_recursive_folding_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drop_mode_flags(&mut self, flags: crate::classes::tree::DropModeFlags,) {
            type CallRet = ();
            type CallParams = (crate::classes::tree::DropModeFlags,);
            let args = (flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_drop_mode_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_drop_mode_flags(&self,) -> crate::classes::tree::DropModeFlags {
            type CallRet = crate::classes::tree::DropModeFlags;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_drop_mode_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_rmb_select(&mut self, allow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_allow_rmb_select", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_allow_rmb_select(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_allow_rmb_select", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_reselect(&mut self, allow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_allow_reselect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_allow_reselect(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_allow_reselect", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_search(&mut self, allow: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (allow,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_allow_search", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_allow_search(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "get_allow_search", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_tooltip(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "set_auto_tooltip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_auto_tooltip_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tree", "is_auto_tooltip_enabled", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Tree {
        type Base = crate::classes::Control;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Tree"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Tree {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for Tree {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Tree {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Tree {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Tree {
        
    }
    impl crate::obj::cap::GodotDefault for Tree {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Tree {
        type Target = crate::classes::Control;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Tree {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Tree`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Tree__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Tree > for $Class {
                
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
#[doc = "Default-param extender for [`Tree::create_item_ex`][super::Tree::create_item_ex]."]
#[must_use]
pub struct ExCreateItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Tree, parent: CowArg < 'a, Option < Gd < crate::classes::TreeItem > > >, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateItem < 'a > {
    fn new(surround_object: &'a mut re_export::Tree,) -> Self {
        let parent = Gd::null_arg();
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parent: parent.into_arg(), index: index,
        }
    }
    #[inline]
    pub fn parent(self, parent: impl AsArg < Option < Gd < crate::classes::TreeItem >> > + 'a) -> Self {
        Self {
            parent: parent.into_arg(), .. self
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TreeItem > > {
        let Self {
            _phantom, surround_object, parent, index,
        }
        = self;
        re_export::Tree::create_item_full(surround_object, parent, index,)
    }
}
#[doc = "Default-param extender for [`Tree::edit_selected_ex`][super::Tree::edit_selected_ex]."]
#[must_use]
pub struct ExEditSelected < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Tree, force_edit: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExEditSelected < 'a > {
    fn new(surround_object: &'a mut re_export::Tree,) -> Self {
        let force_edit = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, force_edit: force_edit,
        }
    }
    #[inline]
    pub fn force_edit(self, force_edit: bool) -> Self {
        Self {
            force_edit: force_edit, .. self
        }
    }
    #[inline]
    pub fn done(self) -> bool {
        let Self {
            _phantom, surround_object, force_edit,
        }
        = self;
        re_export::Tree::edit_selected_full(surround_object, force_edit,)
    }
}
#[doc = "Default-param extender for [`Tree::get_item_area_rect_ex`][super::Tree::get_item_area_rect_ex]."]
#[must_use]
pub struct ExGetItemAreaRect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Tree, item: CowArg < 'a, Option < Gd < crate::classes::TreeItem > > >, column: i32, button_index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetItemAreaRect < 'a > {
    fn new(surround_object: &'a re_export::Tree, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> > + 'a,) -> Self {
        let column = - 1i32;
        let button_index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item.into_arg(), column: column, button_index: button_index,
        }
    }
    #[inline]
    pub fn column(self, column: i32) -> Self {
        Self {
            column: column, .. self
        }
    }
    #[inline]
    pub fn button_index(self, button_index: i32) -> Self {
        Self {
            button_index: button_index, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Rect2 {
        let Self {
            _phantom, surround_object, item, column, button_index,
        }
        = self;
        re_export::Tree::get_item_area_rect_full(surround_object, item, column, button_index,)
    }
}
#[doc = "Default-param extender for [`Tree::scroll_to_item_ex`][super::Tree::scroll_to_item_ex]."]
#[must_use]
pub struct ExScrollToItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Tree, item: CowArg < 'a, Option < Gd < crate::classes::TreeItem > > >, center_on_item: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExScrollToItem < 'a > {
    fn new(surround_object: &'a mut re_export::Tree, item: impl AsArg < Option < Gd < crate::classes::TreeItem >> > + 'a,) -> Self {
        let center_on_item = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, item: item.into_arg(), center_on_item: center_on_item,
        }
    }
    #[inline]
    pub fn center_on_item(self, center_on_item: bool) -> Self {
        Self {
            center_on_item: center_on_item, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, item, center_on_item,
        }
        = self;
        re_export::Tree::scroll_to_item_full(surround_object, item, center_on_item,)
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
    #[doc(alias = "SELECT_ROW")]
    #[doc = "Godot enumerator name: `SELECT_ROW`"]
    pub const ROW: SelectMode = SelectMode {
        ord: 1i32
    };
    #[doc(alias = "SELECT_MULTI")]
    #[doc = "Godot enumerator name: `SELECT_MULTI`"]
    pub const MULTI: SelectMode = SelectMode {
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
            Self::SINGLE => "SINGLE", Self::ROW => "ROW", Self::MULTI => "MULTI", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SelectMode::SINGLE, SelectMode::ROW, SelectMode::MULTI]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SelectMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SINGLE", "SELECT_SINGLE", SelectMode::SINGLE), crate::meta::inspect::EnumConstant::new("ROW", "SELECT_ROW", SelectMode::ROW), crate::meta::inspect::EnumConstant::new("MULTI", "SELECT_MULTI", SelectMode::MULTI)]
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DropModeFlags {
    ord: i32
}
impl DropModeFlags {
    #[doc(alias = "DROP_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `DROP_MODE_DISABLED`"]
    pub const DISABLED: DropModeFlags = DropModeFlags {
        ord: 0i32
    };
    #[doc(alias = "DROP_MODE_ON_ITEM")]
    #[doc = "Godot enumerator name: `DROP_MODE_ON_ITEM`"]
    pub const ON_ITEM: DropModeFlags = DropModeFlags {
        ord: 1i32
    };
    #[doc(alias = "DROP_MODE_INBETWEEN")]
    #[doc = "Godot enumerator name: `DROP_MODE_INBETWEEN`"]
    pub const INBETWEEN: DropModeFlags = DropModeFlags {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for DropModeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DropModeFlags") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DropModeFlags {
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
            Self::DISABLED => "DISABLED", Self::ON_ITEM => "ON_ITEM", Self::INBETWEEN => "INBETWEEN", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DropModeFlags::DISABLED, DropModeFlags::ON_ITEM, DropModeFlags::INBETWEEN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DropModeFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "DROP_MODE_DISABLED", DropModeFlags::DISABLED), crate::meta::inspect::EnumConstant::new("ON_ITEM", "DROP_MODE_ON_ITEM", DropModeFlags::ON_ITEM), crate::meta::inspect::EnumConstant::new("INBETWEEN", "DROP_MODE_INBETWEEN", DropModeFlags::INBETWEEN)]
        }
    }
}
impl crate::meta::GodotConvert for DropModeFlags {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DropModeFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DropModeFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Tree;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Tree`][crate::classes::Tree] class."]
    pub struct SignalsOfTree < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfTree < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn item_selected(&mut self) -> SigItemSelected < 'c, C > {
            SigItemSelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "item_selected")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn cell_selected(&mut self) -> SigCellSelected < 'c, C > {
            SigCellSelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "cell_selected")
            }
        }
        #[doc = "Signature: `(item: Gd<TreeItem>, column: i64, selected: bool)`"]
        pub fn multi_selected(&mut self) -> SigMultiSelected < 'c, C > {
            SigMultiSelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "multi_selected")
            }
        }
        #[doc = "Signature: `(mouse_position: Vector2, mouse_button_index: i64)`"]
        pub fn item_mouse_selected(&mut self) -> SigItemMouseSelected < 'c, C > {
            SigItemMouseSelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "item_mouse_selected")
            }
        }
        #[doc = "Signature: `(click_position: Vector2, mouse_button_index: i64)`"]
        pub fn empty_clicked(&mut self) -> SigEmptyClicked < 'c, C > {
            SigEmptyClicked {
                typed: TypedSignal::extract(&mut self.__internal_obj, "empty_clicked")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn item_edited(&mut self) -> SigItemEdited < 'c, C > {
            SigItemEdited {
                typed: TypedSignal::extract(&mut self.__internal_obj, "item_edited")
            }
        }
        #[doc = "Signature: `(mouse_button_index: i64)`"]
        pub fn custom_item_clicked(&mut self) -> SigCustomItemClicked < 'c, C > {
            SigCustomItemClicked {
                typed: TypedSignal::extract(&mut self.__internal_obj, "custom_item_clicked")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn item_icon_double_clicked(&mut self) -> SigItemIconDoubleClicked < 'c, C > {
            SigItemIconDoubleClicked {
                typed: TypedSignal::extract(&mut self.__internal_obj, "item_icon_double_clicked")
            }
        }
        #[doc = "Signature: `(item: Gd<TreeItem>)`"]
        pub fn item_collapsed(&mut self) -> SigItemCollapsed < 'c, C > {
            SigItemCollapsed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "item_collapsed")
            }
        }
        #[doc = "Signature: `(item: Gd<TreeItem>, column: i64)`"]
        pub fn check_propagated_to_item(&mut self) -> SigCheckPropagatedToItem < 'c, C > {
            SigCheckPropagatedToItem {
                typed: TypedSignal::extract(&mut self.__internal_obj, "check_propagated_to_item")
            }
        }
        #[doc = "Signature: `(item: Gd<TreeItem>, column: i64, id: i64, mouse_button_index: i64)`"]
        pub fn button_clicked(&mut self) -> SigButtonClicked < 'c, C > {
            SigButtonClicked {
                typed: TypedSignal::extract(&mut self.__internal_obj, "button_clicked")
            }
        }
        #[doc = "Signature: `(arrow_clicked: bool)`"]
        pub fn custom_popup_edited(&mut self) -> SigCustomPopupEdited < 'c, C > {
            SigCustomPopupEdited {
                typed: TypedSignal::extract(&mut self.__internal_obj, "custom_popup_edited")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn item_activated(&mut self) -> SigItemActivated < 'c, C > {
            SigItemActivated {
                typed: TypedSignal::extract(&mut self.__internal_obj, "item_activated")
            }
        }
        #[doc = "Signature: `(column: i64, mouse_button_index: i64)`"]
        pub fn column_title_clicked(&mut self) -> SigColumnTitleClicked < 'c, C > {
            SigColumnTitleClicked {
                typed: TypedSignal::extract(&mut self.__internal_obj, "column_title_clicked")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn nothing_selected(&mut self) -> SigNothingSelected < 'c, C > {
            SigNothingSelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "nothing_selected")
            }
        }
    }
    type TypedSigItemSelected < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigItemSelected < 'c, C: WithSignals > {
        typed: TypedSigItemSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemSelected < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
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
    type TypedSigCellSelected < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigCellSelected < 'c, C: WithSignals > {
        typed: TypedSigCellSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCellSelected < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCellSelected < 'c, C > {
        type Target = TypedSigCellSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCellSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigMultiSelected < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::TreeItem >, i64, bool,) >;
    pub struct SigMultiSelected < 'c, C: WithSignals > {
        typed: TypedSigMultiSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigMultiSelected < 'c, C > {
        pub fn emit(&mut self, item: Gd < crate::classes::TreeItem >, column: i64, selected: bool,) {
            self.typed.emit_tuple((item, column, selected,));
            
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
    type TypedSigItemMouseSelected < 'c, C > = TypedSignal < 'c, C, (Vector2, i64,) >;
    pub struct SigItemMouseSelected < 'c, C: WithSignals > {
        typed: TypedSigItemMouseSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemMouseSelected < 'c, C > {
        pub fn emit(&mut self, mouse_position: Vector2, mouse_button_index: i64,) {
            self.typed.emit_tuple((mouse_position, mouse_button_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemMouseSelected < 'c, C > {
        type Target = TypedSigItemMouseSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemMouseSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigEmptyClicked < 'c, C > = TypedSignal < 'c, C, (Vector2, i64,) >;
    pub struct SigEmptyClicked < 'c, C: WithSignals > {
        typed: TypedSigEmptyClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigEmptyClicked < 'c, C > {
        pub fn emit(&mut self, click_position: Vector2, mouse_button_index: i64,) {
            self.typed.emit_tuple((click_position, mouse_button_index,));
            
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
    type TypedSigItemEdited < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigItemEdited < 'c, C: WithSignals > {
        typed: TypedSigItemEdited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemEdited < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemEdited < 'c, C > {
        type Target = TypedSigItemEdited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemEdited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCustomItemClicked < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigCustomItemClicked < 'c, C: WithSignals > {
        typed: TypedSigCustomItemClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCustomItemClicked < 'c, C > {
        pub fn emit(&mut self, mouse_button_index: i64,) {
            self.typed.emit_tuple((mouse_button_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCustomItemClicked < 'c, C > {
        type Target = TypedSigCustomItemClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCustomItemClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigItemIconDoubleClicked < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigItemIconDoubleClicked < 'c, C: WithSignals > {
        typed: TypedSigItemIconDoubleClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemIconDoubleClicked < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemIconDoubleClicked < 'c, C > {
        type Target = TypedSigItemIconDoubleClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemIconDoubleClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigItemCollapsed < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::TreeItem >,) >;
    pub struct SigItemCollapsed < 'c, C: WithSignals > {
        typed: TypedSigItemCollapsed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemCollapsed < 'c, C > {
        pub fn emit(&mut self, item: Gd < crate::classes::TreeItem >,) {
            self.typed.emit_tuple((item,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigItemCollapsed < 'c, C > {
        type Target = TypedSigItemCollapsed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigItemCollapsed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCheckPropagatedToItem < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::TreeItem >, i64,) >;
    pub struct SigCheckPropagatedToItem < 'c, C: WithSignals > {
        typed: TypedSigCheckPropagatedToItem < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCheckPropagatedToItem < 'c, C > {
        pub fn emit(&mut self, item: Gd < crate::classes::TreeItem >, column: i64,) {
            self.typed.emit_tuple((item, column,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCheckPropagatedToItem < 'c, C > {
        type Target = TypedSigCheckPropagatedToItem < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCheckPropagatedToItem < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigButtonClicked < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::TreeItem >, i64, i64, i64,) >;
    pub struct SigButtonClicked < 'c, C: WithSignals > {
        typed: TypedSigButtonClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigButtonClicked < 'c, C > {
        pub fn emit(&mut self, item: Gd < crate::classes::TreeItem >, column: i64, id: i64, mouse_button_index: i64,) {
            self.typed.emit_tuple((item, column, id, mouse_button_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigButtonClicked < 'c, C > {
        type Target = TypedSigButtonClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigButtonClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCustomPopupEdited < 'c, C > = TypedSignal < 'c, C, (bool,) >;
    pub struct SigCustomPopupEdited < 'c, C: WithSignals > {
        typed: TypedSigCustomPopupEdited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCustomPopupEdited < 'c, C > {
        pub fn emit(&mut self, arrow_clicked: bool,) {
            self.typed.emit_tuple((arrow_clicked,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCustomPopupEdited < 'c, C > {
        type Target = TypedSigCustomPopupEdited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCustomPopupEdited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigItemActivated < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigItemActivated < 'c, C: WithSignals > {
        typed: TypedSigItemActivated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigItemActivated < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
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
    type TypedSigColumnTitleClicked < 'c, C > = TypedSignal < 'c, C, (i64, i64,) >;
    pub struct SigColumnTitleClicked < 'c, C: WithSignals > {
        typed: TypedSigColumnTitleClicked < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigColumnTitleClicked < 'c, C > {
        pub fn emit(&mut self, column: i64, mouse_button_index: i64,) {
            self.typed.emit_tuple((column, mouse_button_index,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigColumnTitleClicked < 'c, C > {
        type Target = TypedSigColumnTitleClicked < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigColumnTitleClicked < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigNothingSelected < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigNothingSelected < 'c, C: WithSignals > {
        typed: TypedSigNothingSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigNothingSelected < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigNothingSelected < 'c, C > {
        type Target = TypedSigNothingSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigNothingSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Tree {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfTree < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfTree < 'c, C > {
        type Target = < < Tree as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Tree;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfTree < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Tree;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}