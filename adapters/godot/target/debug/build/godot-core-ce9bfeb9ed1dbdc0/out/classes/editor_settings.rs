#![doc = "Sidecar module for class [`EditorSettings`][crate::classes::EditorSettings].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorSettings` enums](https://docs.godotengine.org/en/stable/classes/class_editorsettings.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorSettings.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`editor_settings`][crate::classes::editor_settings]: sidecar module with related enum/flag types\n* [`IEditorSettings`][crate::classes::IEditorSettings]: virtual methods\n* [`SignalsOfEditorSettings`][crate::classes::editor_settings::SignalsOfEditorSettings]: signal collection\n* [`EditorSettingsNotification`][crate::classes::notify::EditorSettingsNotification]: notification type\n\n\nSee also [Godot docs for `EditorSettings`](https://docs.godotengine.org/en/stable/classes/class_editorsettings.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`EditorSettings::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorSettings {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorSettings`][crate::classes::EditorSettings].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `EditorSettings` methods](https://docs.godotengine.org/en/stable/classes/class_editorsettings.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorSettings: crate::obj::GodotClass < Base = EditorSettings > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: EditorSettingsNotification) {
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
    #[doc = "Notification type for class [`EditorSettings`][crate::classes::EditorSettings]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[doc = r""]
    #[doc = r" Contains the [`Unknown`][Self::Unknown] variant for forward compatibility."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    #[allow(non_camel_case_types)]
    pub enum EditorSettingsNotification {
        EDITOR_SETTINGS_CHANGED = 10000i32, POSTINITIALIZE = 0i32, PREDELETE = 1i32, EXTENSION_RELOADED = 2i32, #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object::notify()`."]
        #[doc = r""]
        #[doc = r" This is also necessary if you develop an extension on a Godot version and want to be forward-compatible with newer"]
        #[doc = r" versions. If Godot adds new notifications, they will be unknown to your extension, but you can still handle them."]
        Unknown(i32),
    }
    impl From < i32 > for EditorSettingsNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                10000i32 => Self::EDITOR_SETTINGS_CHANGED, 0i32 => Self::POSTINITIALIZE, 1i32 => Self::PREDELETE, 2i32 => Self::EXTENSION_RELOADED, other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From < EditorSettingsNotification > for i32 {
        fn from(notification: EditorSettingsNotification) -> i32 {
            match notification {
                EditorSettingsNotification::EDITOR_SETTINGS_CHANGED => 10000i32, EditorSettingsNotification::POSTINITIALIZE => 0i32, EditorSettingsNotification::PREDELETE => 1i32, EditorSettingsNotification::EXTENSION_RELOADED => 2i32, EditorSettingsNotification::Unknown(int) => int,
            }
        }
    }
    impl EditorSettings {
        pub fn has_setting(&self, name: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(376usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSettings", "has_setting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_setting(&mut self, name: impl AsArg < GString >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(377usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSettings", "set_setting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_setting(&self, name: impl AsArg < GString >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(378usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSettings", "get_setting", Some(self.__validated_obj()), args,)
            }
        }
        pub fn erase(&mut self, property: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (property.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(379usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSettings", "erase", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_initial_value(&mut self, name: impl AsArg < StringName >, value: &Variant, update_current: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Variant >, bool,);
            let args = (name.into_arg(), RefArg::new(value), update_current,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(380usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSettings", "set_initial_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_property_info(&mut self, info: &VarDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >,);
            let args = (RefArg::new(info),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(381usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSettings", "add_property_info", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_project_metadata(&mut self, section: impl AsArg < GString >, key: impl AsArg < GString >, data: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, Variant >,);
            let args = (section.into_arg(), key.into_arg(), RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(382usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSettings", "set_project_metadata", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn get_project_metadata_full(&self, section: CowArg < GString >, key: CowArg < GString >, default: RefArg < Variant >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, Variant >,);
            let args = (section, key, default,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(383usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSettings", "get_project_metadata", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_project_metadata_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_project_metadata(&self, section: impl AsArg < GString >, key: impl AsArg < GString >,) -> Variant {
            self.get_project_metadata_ex(section, key,) . done()
        }
        #[inline]
        pub fn get_project_metadata_ex < 'a > (&'a self, section: impl AsArg < GString > + 'a, key: impl AsArg < GString > + 'a,) -> ExGetProjectMetadata < 'a > {
            ExGetProjectMetadata::new(self, section, key,)
        }
        pub fn set_favorites(&mut self, dirs: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(dirs),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(384usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSettings", "set_favorites", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_favorites(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(385usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSettings", "get_favorites", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_recent_dirs(&mut self, dirs: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(dirs),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(386usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSettings", "set_recent_dirs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_recent_dirs(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(387usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSettings", "get_recent_dirs", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_builtin_action_override(&mut self, name: impl AsArg < GString >, actions_list: &Array < Gd < crate::classes::InputEvent > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, Array < Gd < crate::classes::InputEvent > > >,);
            let args = (name.into_arg(), RefArg::new(actions_list),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(388usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSettings", "set_builtin_action_override", Some(self.__validated_obj()), args,)
            }
        }
        pub fn check_changed_settings_in_group(&self, setting_prefix: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (setting_prefix.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(389usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSettings", "check_changed_settings_in_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_changed_settings(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(390usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSettings", "get_changed_settings", Some(self.__validated_obj()), args,)
            }
        }
        pub fn mark_setting_changed(&mut self, setting: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (setting.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(391usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSettings", "mark_setting_changed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: EditorSettingsNotification) {
            self.notification(i32::from(what), false);
            
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: EditorSettingsNotification) {
            self.notification(i32::from(what), true);
            
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
        pub(crate) const NOTIFICATION_EDITOR_SETTINGS_CHANGED: i32 = 10000i32;
        
    }
    impl crate::obj::GodotClass for EditorSettings {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorSettings"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorSettings {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for EditorSettings {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorSettings {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorSettings {
        
    }
    impl crate::obj::cap::GodotDefault for EditorSettings {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorSettings {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorSettings {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorSettings`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorSettings__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorSettings > for $Class {
                
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
#[doc = "Default-param extender for [`EditorSettings::get_project_metadata_ex`][super::EditorSettings::get_project_metadata_ex]."]
#[must_use]
pub struct ExGetProjectMetadata < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::EditorSettings, section: CowArg < 'a, GString >, key: CowArg < 'a, GString >, default: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetProjectMetadata < 'a > {
    fn new(surround_object: &'a re_export::EditorSettings, section: impl AsArg < GString > + 'a, key: impl AsArg < GString > + 'a,) -> Self {
        let default = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, section: section.into_arg(), key: key.into_arg(), default: CowArg::Owned(default),
        }
    }
    #[inline]
    pub fn default(self, default: &'a Variant) -> Self {
        Self {
            default: CowArg::Borrowed(default), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, section, key, default,
        }
        = self;
        re_export::EditorSettings::get_project_metadata_full(surround_object, section, key, default.cow_as_arg(),)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorSettings;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`EditorSettings`][crate::classes::EditorSettings] class."]
    pub struct SignalsOfEditorSettings < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfEditorSettings < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn settings_changed(&mut self) -> SigSettingsChanged < 'c, C > {
            SigSettingsChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "settings_changed")
            }
        }
    }
    type TypedSigSettingsChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSettingsChanged < 'c, C: WithSignals > {
        typed: TypedSigSettingsChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSettingsChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSettingsChanged < 'c, C > {
        type Target = TypedSigSettingsChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSettingsChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for EditorSettings {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfEditorSettings < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfEditorSettings < 'c, C > {
        type Target = < < EditorSettings as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = EditorSettings;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfEditorSettings < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = EditorSettings;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}