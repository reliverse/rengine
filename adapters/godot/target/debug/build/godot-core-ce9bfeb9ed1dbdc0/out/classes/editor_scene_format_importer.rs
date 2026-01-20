#![doc = "Sidecar module for class [`EditorSceneFormatImporter`][crate::classes::EditorSceneFormatImporter].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorSceneFormatImporter` enums](https://docs.godotengine.org/en/stable/classes/class_editorsceneformatimporter.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorSceneFormatImporter.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`editor_scene_format_importer`][crate::classes::editor_scene_format_importer]: sidecar module with related enum/flag types\n* [`IEditorSceneFormatImporter`][crate::classes::IEditorSceneFormatImporter]: virtual methods\n\n\nSee also [Godot docs for `EditorSceneFormatImporter`](https://docs.godotengine.org/en/stable/classes/class_editorsceneformatimporter.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`EditorSceneFormatImporter::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorSceneFormatImporter {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorSceneFormatImporter`][crate::classes::EditorSceneFormatImporter].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `EditorSceneFormatImporter` methods](https://docs.godotengine.org/en/stable/classes/class_editorsceneformatimporter.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorSceneFormatImporter: crate::obj::GodotClass < Base = EditorSceneFormatImporter > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_extensions(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn import_scene(&mut self, path: GString, flags: u32, options: VarDictionary,) -> Option < Gd < crate::classes::Object > > {
            unimplemented !()
        }
        fn get_import_options(&mut self, path: GString,) {
            unimplemented !()
        }
        fn get_option_visibility(&self, path: GString, for_animation: bool, option: GString,) -> Variant {
            unimplemented !()
        }
    }
    impl EditorSceneFormatImporter {
        pub fn add_import_option(&mut self, name: impl AsArg < GString >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, Variant >,);
            let args = (name.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(359usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSceneFormatImporter", "add_import_option", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_import_option_advanced_full(&mut self, type_: VariantType, name: CowArg < GString >, default_value: RefArg < Variant >, hint: crate::global::PropertyHint, hint_string: CowArg < GString >, usage_flags: i32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (VariantType, CowArg < 'a0, GString >, RefArg < 'a1, Variant >, crate::global::PropertyHint, CowArg < 'a2, GString >, i32,);
            let args = (type_, name, default_value, hint, hint_string, usage_flags,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(360usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorSceneFormatImporter", "add_import_option_advanced", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_import_option_advanced_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_import_option_advanced(&mut self, type_: VariantType, name: impl AsArg < GString >, default_value: &Variant,) {
            self.add_import_option_advanced_ex(type_, name, default_value,) . done()
        }
        #[inline]
        pub fn add_import_option_advanced_ex < 'a > (&'a mut self, type_: VariantType, name: impl AsArg < GString > + 'a, default_value: &'a Variant,) -> ExAddImportOptionAdvanced < 'a > {
            ExAddImportOptionAdvanced::new(self, type_, name, default_value,)
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
        pub const IMPORT_SCENE: i32 = 1i32;
        pub const IMPORT_ANIMATION: i32 = 2i32;
        pub const IMPORT_FAIL_ON_MISSING_DEPENDENCIES: i32 = 4i32;
        pub const IMPORT_GENERATE_TANGENT_ARRAYS: i32 = 8i32;
        pub const IMPORT_USE_NAMED_SKIN_BINDS: i32 = 16i32;
        pub const IMPORT_DISCARD_MESHES_AND_MATERIALS: i32 = 32i32;
        pub const IMPORT_FORCE_DISABLE_MESH_COMPRESSION: i32 = 64i32;
        
    }
    impl crate::obj::GodotClass for EditorSceneFormatImporter {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorSceneFormatImporter"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorSceneFormatImporter {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorSceneFormatImporter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorSceneFormatImporter {
        
    }
    impl crate::obj::cap::GodotDefault for EditorSceneFormatImporter {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorSceneFormatImporter {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorSceneFormatImporter {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorSceneFormatImporter`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorSceneFormatImporter__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorSceneFormatImporter > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorSceneFormatImporter::add_import_option_advanced_ex`][super::EditorSceneFormatImporter::add_import_option_advanced_ex]."]
#[must_use]
pub struct ExAddImportOptionAdvanced < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorSceneFormatImporter, type_: VariantType, name: CowArg < 'a, GString >, default_value: CowArg < 'a, Variant >, hint: crate::global::PropertyHint, hint_string: CowArg < 'a, GString >, usage_flags: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddImportOptionAdvanced < 'a > {
    fn new(surround_object: &'a mut re_export::EditorSceneFormatImporter, type_: VariantType, name: impl AsArg < GString > + 'a, default_value: &'a Variant,) -> Self {
        let hint = crate::obj::EngineEnum::from_ord(0);
        let hint_string = GString::from("");
        let usage_flags = 6i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, type_: type_, name: name.into_arg(), default_value: CowArg::Borrowed(default_value), hint: hint, hint_string: CowArg::Owned(hint_string), usage_flags: usage_flags,
        }
    }
    #[inline]
    pub fn hint(self, hint: crate::global::PropertyHint) -> Self {
        Self {
            hint: hint, .. self
        }
    }
    #[inline]
    pub fn hint_string(self, hint_string: impl AsArg < GString > + 'a) -> Self {
        Self {
            hint_string: hint_string.into_arg(), .. self
        }
    }
    #[inline]
    pub fn usage_flags(self, usage_flags: i32) -> Self {
        Self {
            usage_flags: usage_flags, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, type_, name, default_value, hint, hint_string, usage_flags,
        }
        = self;
        re_export::EditorSceneFormatImporter::add_import_option_advanced_full(surround_object, type_, name, default_value.cow_as_arg(), hint, hint_string, usage_flags,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorSceneFormatImporter;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EditorSceneFormatImporter {
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