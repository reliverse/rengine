#![doc = "Sidecar module for class [`EditorFileDialog`][crate::classes::EditorFileDialog].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorFileDialog` enums](https://docs.godotengine.org/en/stable/classes/class_editorfiledialog.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorFileDialog.`\n\nInherits [`ConfirmationDialog`][crate::classes::ConfirmationDialog].\n\nRelated symbols:\n\n* [`editor_file_dialog`][crate::classes::editor_file_dialog]: sidecar module with related enum/flag types\n* [`IEditorFileDialog`][crate::classes::IEditorFileDialog]: virtual methods\n* [`SignalsOfEditorFileDialog`][crate::classes::editor_file_dialog::SignalsOfEditorFileDialog]: signal collection\n\n\nSee also [Godot docs for `EditorFileDialog`](https://docs.godotengine.org/en/stable/classes/class_editorfiledialog.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`EditorFileDialog::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorFileDialog {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorFileDialog`][crate::classes::EditorFileDialog].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IConfirmationDialog`][crate::classes::IConfirmationDialog] > [`IAcceptDialog`][crate::classes::IAcceptDialog] > [`IWindow`][crate::classes::IWindow] > ~~`IViewport`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `EditorFileDialog` methods](https://docs.godotengine.org/en/stable/classes/class_editorfiledialog.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorFileDialog: crate::obj::GodotClass < Base = EditorFileDialog > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: WindowNotification) {
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
        fn get_contents_minimum_size(&self,) -> Vector2 {
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
    impl EditorFileDialog {
        pub fn clear_filters(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(104usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "clear_filters", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_filter_full(&mut self, filter: CowArg < GString >, description: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (filter, description,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(105usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "add_filter", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_filter_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_filter(&mut self, filter: impl AsArg < GString >,) {
            self.add_filter_ex(filter,) . done()
        }
        #[inline]
        pub fn add_filter_ex < 'a > (&'a mut self, filter: impl AsArg < GString > + 'a,) -> ExAddFilter < 'a > {
            ExAddFilter::new(self, filter,)
        }
        pub fn set_filters(&mut self, filters: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(filters),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(106usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_filters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_filters(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(107usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_filters", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_option_name(&self, option: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(108usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_option_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_option_values(&self, option: i32,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = (i32,);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(109usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_option_values", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_option_default(&self, option: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (option,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(110usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_option_default", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_option_name(&mut self, option: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (option, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(111usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_option_name", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_option_values(&mut self, option: i32, values: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, RefArg < 'a0, PackedStringArray >,);
            let args = (option, RefArg::new(values),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(112usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_option_values", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_option_default(&mut self, option: i32, default_value_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (option, default_value_index,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_option_default", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_option_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_option_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_option_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_option_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_option(&mut self, name: impl AsArg < GString >, values: &PackedStringArray, default_value_index: i32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, PackedStringArray >, i32,);
            let args = (name.into_arg(), RefArg::new(values), default_value_index,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "add_option", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selected_options(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_selected_options", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_filename_filter(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "clear_filename_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_filename_filter(&mut self, filter: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (filter.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_filename_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_filename_filter(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_filename_filter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_dir(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_current_dir", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_file(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(122usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_current_file", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(123usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_current_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_current_dir(&mut self, dir: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (dir.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(124usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_current_dir", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_current_file(&mut self, file: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (file.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(125usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_current_file", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_current_path(&mut self, path: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(126usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_current_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_file_mode(&mut self, mode: crate::classes::editor_file_dialog::FileMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::editor_file_dialog::FileMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(127usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_file_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_file_mode(&self,) -> crate::classes::editor_file_dialog::FileMode {
            type CallRet = crate::classes::editor_file_dialog::FileMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(128usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_file_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_vbox(&mut self,) -> Option < Gd < crate::classes::VBoxContainer > > {
            type CallRet = Option < Gd < crate::classes::VBoxContainer > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(129usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_vbox", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_line_edit(&mut self,) -> Option < Gd < crate::classes::LineEdit > > {
            type CallRet = Option < Gd < crate::classes::LineEdit > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(130usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_line_edit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_access(&mut self, access: crate::classes::editor_file_dialog::Access,) {
            type CallRet = ();
            type CallParams = (crate::classes::editor_file_dialog::Access,);
            let args = (access,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(131usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_access", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_access(&self,) -> crate::classes::editor_file_dialog::Access {
            type CallRet = crate::classes::editor_file_dialog::Access;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(132usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_access", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_show_hidden_files(&mut self, show: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (show,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(133usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_show_hidden_files", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_showing_hidden_files(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(134usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "is_showing_hidden_files", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_display_mode(&mut self, mode: crate::classes::editor_file_dialog::DisplayMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::editor_file_dialog::DisplayMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(135usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_display_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_display_mode(&self,) -> crate::classes::editor_file_dialog::DisplayMode {
            type CallRet = crate::classes::editor_file_dialog::DisplayMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(136usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "get_display_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_disable_overwrite_warning(&mut self, disable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (disable,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(137usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "set_disable_overwrite_warning", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_overwrite_warning_disabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(138usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "is_overwrite_warning_disabled", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_side_menu_full(&mut self, menu: CowArg < Option < Gd < crate::classes::Control > > >, title: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Control > > >, CowArg < 'a1, GString >,);
            let args = (menu, title,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(139usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "add_side_menu", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_side_menu_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_side_menu(&mut self, menu: impl AsArg < Option < Gd < crate::classes::Control >> >,) {
            self.add_side_menu_ex(menu,) . done()
        }
        #[inline]
        pub fn add_side_menu_ex < 'a > (&'a mut self, menu: impl AsArg < Option < Gd < crate::classes::Control >> > + 'a,) -> ExAddSideMenu < 'a > {
            ExAddSideMenu::new(self, menu,)
        }
        pub fn popup_file_dialog(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(140usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "popup_file_dialog", Some(self.__validated_obj()), args,)
            }
        }
        pub fn invalidate(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(141usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorFileDialog", "invalidate", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EditorFileDialog {
        type Base = crate::classes::ConfirmationDialog;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorFileDialog"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorFileDialog {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::ConfirmationDialog > for EditorFileDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AcceptDialog > for EditorFileDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Window > for EditorFileDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Viewport > for EditorFileDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for EditorFileDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorFileDialog {
        
    }
    impl crate::obj::cap::GodotDefault for EditorFileDialog {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorFileDialog {
        type Target = crate::classes::ConfirmationDialog;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorFileDialog {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorFileDialog`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorFileDialog__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorFileDialog > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::ConfirmationDialog > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AcceptDialog > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Window > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Viewport > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorFileDialog::add_filter_ex`][super::EditorFileDialog::add_filter_ex]."]
#[must_use]
pub struct ExAddFilter < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorFileDialog, filter: CowArg < 'a, GString >, description: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddFilter < 'a > {
    fn new(surround_object: &'a mut re_export::EditorFileDialog, filter: impl AsArg < GString > + 'a,) -> Self {
        let description = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, filter: filter.into_arg(), description: CowArg::Owned(description),
        }
    }
    #[inline]
    pub fn description(self, description: impl AsArg < GString > + 'a) -> Self {
        Self {
            description: description.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, filter, description,
        }
        = self;
        re_export::EditorFileDialog::add_filter_full(surround_object, filter, description,)
    }
}
#[doc = "Default-param extender for [`EditorFileDialog::add_side_menu_ex`][super::EditorFileDialog::add_side_menu_ex]."]
#[must_use]
pub struct ExAddSideMenu < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorFileDialog, menu: CowArg < 'a, Option < Gd < crate::classes::Control > > >, title: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddSideMenu < 'a > {
    fn new(surround_object: &'a mut re_export::EditorFileDialog, menu: impl AsArg < Option < Gd < crate::classes::Control >> > + 'a,) -> Self {
        let title = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, menu: menu.into_arg(), title: CowArg::Owned(title),
        }
    }
    #[inline]
    pub fn title(self, title: impl AsArg < GString > + 'a) -> Self {
        Self {
            title: title.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, menu, title,
        }
        = self;
        re_export::EditorFileDialog::add_side_menu_full(surround_object, menu, title,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FileMode {
    ord: i32
}
impl FileMode {
    #[doc(alias = "FILE_MODE_OPEN_FILE")]
    #[doc = "Godot enumerator name: `FILE_MODE_OPEN_FILE`"]
    pub const OPEN_FILE: FileMode = FileMode {
        ord: 0i32
    };
    #[doc(alias = "FILE_MODE_OPEN_FILES")]
    #[doc = "Godot enumerator name: `FILE_MODE_OPEN_FILES`"]
    pub const OPEN_FILES: FileMode = FileMode {
        ord: 1i32
    };
    #[doc(alias = "FILE_MODE_OPEN_DIR")]
    #[doc = "Godot enumerator name: `FILE_MODE_OPEN_DIR`"]
    pub const OPEN_DIR: FileMode = FileMode {
        ord: 2i32
    };
    #[doc(alias = "FILE_MODE_OPEN_ANY")]
    #[doc = "Godot enumerator name: `FILE_MODE_OPEN_ANY`"]
    pub const OPEN_ANY: FileMode = FileMode {
        ord: 3i32
    };
    #[doc(alias = "FILE_MODE_SAVE_FILE")]
    #[doc = "Godot enumerator name: `FILE_MODE_SAVE_FILE`"]
    pub const SAVE_FILE: FileMode = FileMode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for FileMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("FileMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for FileMode {
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
            Self::OPEN_FILE => "OPEN_FILE", Self::OPEN_FILES => "OPEN_FILES", Self::OPEN_DIR => "OPEN_DIR", Self::OPEN_ANY => "OPEN_ANY", Self::SAVE_FILE => "SAVE_FILE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[FileMode::OPEN_FILE, FileMode::OPEN_FILES, FileMode::OPEN_DIR, FileMode::OPEN_ANY, FileMode::SAVE_FILE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < FileMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("OPEN_FILE", "FILE_MODE_OPEN_FILE", FileMode::OPEN_FILE), crate::meta::inspect::EnumConstant::new("OPEN_FILES", "FILE_MODE_OPEN_FILES", FileMode::OPEN_FILES), crate::meta::inspect::EnumConstant::new("OPEN_DIR", "FILE_MODE_OPEN_DIR", FileMode::OPEN_DIR), crate::meta::inspect::EnumConstant::new("OPEN_ANY", "FILE_MODE_OPEN_ANY", FileMode::OPEN_ANY), crate::meta::inspect::EnumConstant::new("SAVE_FILE", "FILE_MODE_SAVE_FILE", FileMode::SAVE_FILE)]
        }
    }
}
impl crate::meta::GodotConvert for FileMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for FileMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for FileMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Access {
    ord: i32
}
impl Access {
    #[doc(alias = "ACCESS_RESOURCES")]
    #[doc = "Godot enumerator name: `ACCESS_RESOURCES`"]
    pub const RESOURCES: Access = Access {
        ord: 0i32
    };
    #[doc(alias = "ACCESS_USERDATA")]
    #[doc = "Godot enumerator name: `ACCESS_USERDATA`"]
    pub const USERDATA: Access = Access {
        ord: 1i32
    };
    #[doc(alias = "ACCESS_FILESYSTEM")]
    #[doc = "Godot enumerator name: `ACCESS_FILESYSTEM`"]
    pub const FILESYSTEM: Access = Access {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for Access {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Access") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Access {
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
            Self::RESOURCES => "RESOURCES", Self::USERDATA => "USERDATA", Self::FILESYSTEM => "FILESYSTEM", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Access::RESOURCES, Access::USERDATA, Access::FILESYSTEM]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Access >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RESOURCES", "ACCESS_RESOURCES", Access::RESOURCES), crate::meta::inspect::EnumConstant::new("USERDATA", "ACCESS_USERDATA", Access::USERDATA), crate::meta::inspect::EnumConstant::new("FILESYSTEM", "ACCESS_FILESYSTEM", Access::FILESYSTEM)]
        }
    }
}
impl crate::meta::GodotConvert for Access {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Access {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Access {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DisplayMode {
    ord: i32
}
impl DisplayMode {
    #[doc(alias = "DISPLAY_THUMBNAILS")]
    #[doc = "Godot enumerator name: `DISPLAY_THUMBNAILS`"]
    pub const THUMBNAILS: DisplayMode = DisplayMode {
        ord: 0i32
    };
    #[doc(alias = "DISPLAY_LIST")]
    #[doc = "Godot enumerator name: `DISPLAY_LIST`"]
    pub const LIST: DisplayMode = DisplayMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for DisplayMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("DisplayMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for DisplayMode {
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
            Self::THUMBNAILS => "THUMBNAILS", Self::LIST => "LIST", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[DisplayMode::THUMBNAILS, DisplayMode::LIST]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < DisplayMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("THUMBNAILS", "DISPLAY_THUMBNAILS", DisplayMode::THUMBNAILS), crate::meta::inspect::EnumConstant::new("LIST", "DISPLAY_LIST", DisplayMode::LIST)]
        }
    }
}
impl crate::meta::GodotConvert for DisplayMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for DisplayMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for DisplayMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorFileDialog;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`EditorFileDialog`][crate::classes::EditorFileDialog] class."]
    pub struct SignalsOfEditorFileDialog < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfEditorFileDialog < 'c, C > {
        #[doc = "Signature: `(path: GString)`"]
        pub fn file_selected(&mut self) -> SigFileSelected < 'c, C > {
            SigFileSelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "file_selected")
            }
        }
        #[doc = "Signature: `(paths: PackedStringArray)`"]
        pub fn files_selected(&mut self) -> SigFilesSelected < 'c, C > {
            SigFilesSelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "files_selected")
            }
        }
        #[doc = "Signature: `(dir: GString)`"]
        pub fn dir_selected(&mut self) -> SigDirSelected < 'c, C > {
            SigDirSelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "dir_selected")
            }
        }
        #[doc = "Signature: `(filter: GString)`"]
        pub fn filename_filter_changed(&mut self) -> SigFilenameFilterChanged < 'c, C > {
            SigFilenameFilterChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "filename_filter_changed")
            }
        }
    }
    type TypedSigFileSelected < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigFileSelected < 'c, C: WithSignals > {
        typed: TypedSigFileSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFileSelected < 'c, C > {
        pub fn emit(&mut self, path: GString,) {
            self.typed.emit_tuple((path,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFileSelected < 'c, C > {
        type Target = TypedSigFileSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFileSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFilesSelected < 'c, C > = TypedSignal < 'c, C, (PackedStringArray,) >;
    pub struct SigFilesSelected < 'c, C: WithSignals > {
        typed: TypedSigFilesSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFilesSelected < 'c, C > {
        pub fn emit(&mut self, paths: PackedStringArray,) {
            self.typed.emit_tuple((paths,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFilesSelected < 'c, C > {
        type Target = TypedSigFilesSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFilesSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigDirSelected < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigDirSelected < 'c, C: WithSignals > {
        typed: TypedSigDirSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDirSelected < 'c, C > {
        pub fn emit(&mut self, dir: GString,) {
            self.typed.emit_tuple((dir,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDirSelected < 'c, C > {
        type Target = TypedSigDirSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDirSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFilenameFilterChanged < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigFilenameFilterChanged < 'c, C: WithSignals > {
        typed: TypedSigFilenameFilterChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFilenameFilterChanged < 'c, C > {
        pub fn emit(&mut self, filter: GString,) {
            self.typed.emit_tuple((filter,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFilenameFilterChanged < 'c, C > {
        type Target = TypedSigFilenameFilterChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFilenameFilterChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for EditorFileDialog {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfEditorFileDialog < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfEditorFileDialog < 'c, C > {
        type Target = < < EditorFileDialog as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = EditorFileDialog;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfEditorFileDialog < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = EditorFileDialog;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}