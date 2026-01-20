#![doc = "Sidecar module for class [`EditorImportPlugin`][crate::classes::EditorImportPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorImportPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorimportplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorImportPlugin.`\n\nInherits [`ResourceImporter`][crate::classes::ResourceImporter].\n\nRelated symbols:\n\n* [`editor_import_plugin`][crate::classes::editor_import_plugin]: sidecar module with related enum/flag types\n* [`IEditorImportPlugin`][crate::classes::IEditorImportPlugin]: virtual methods\n\n\nSee also [Godot docs for `EditorImportPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorimportplugin.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`EditorImportPlugin::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorImportPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorImportPlugin`][crate::classes::EditorImportPlugin].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IResourceImporter`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `EditorImportPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorimportplugin.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorImportPlugin: crate::obj::GodotClass < Base = EditorImportPlugin > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_importer_name(&self,) -> GString {
            unimplemented !()
        }
        fn get_visible_name(&self,) -> GString {
            unimplemented !()
        }
        fn get_preset_count(&self,) -> i32 {
            unimplemented !()
        }
        fn get_preset_name(&self, preset_index: i32,) -> GString {
            unimplemented !()
        }
        fn get_recognized_extensions(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_import_options(&self, path: GString, preset_index: i32,) -> Array < VarDictionary > {
            unimplemented !()
        }
        fn get_save_extension(&self,) -> GString {
            unimplemented !()
        }
        fn get_resource_type(&self,) -> GString {
            unimplemented !()
        }
        fn get_priority(&self,) -> f32 {
            unimplemented !()
        }
        fn get_import_order(&self,) -> i32 {
            unimplemented !()
        }
        fn get_format_version(&self,) -> i32 {
            unimplemented !()
        }
        fn get_option_visibility(&self, path: GString, option_name: StringName, options: VarDictionary,) -> bool {
            unimplemented !()
        }
        fn import(&self, source_file: GString, save_path: GString, options: VarDictionary, platform_variants: Array < GString >, gen_files: Array < GString >,) -> crate::global::Error {
            unimplemented !()
        }
        fn can_import_threaded(&self,) -> bool {
            unimplemented !()
        }
        fn get_build_dependencies(&self, path: GString,) -> PackedStringArray {
            unimplemented !()
        }
    }
    impl EditorImportPlugin {
        pub(crate) fn append_import_external_resource_full(&mut self, path: CowArg < GString >, custom_options: RefArg < VarDictionary >, custom_importer: CowArg < GString >, generator_parameters: RefArg < Variant >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, RefArg < 'a1, VarDictionary >, CowArg < 'a2, GString >, RefArg < 'a3, Variant >,);
            let args = (path, custom_options, custom_importer, generator_parameters,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(165usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorImportPlugin", "append_import_external_resource", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::append_import_external_resource_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn append_import_external_resource(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            self.append_import_external_resource_ex(path,) . done()
        }
        #[inline]
        pub fn append_import_external_resource_ex < 'a > (&'a mut self, path: impl AsArg < GString > + 'a,) -> ExAppendImportExternalResource < 'a > {
            ExAppendImportExternalResource::new(self, path,)
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
    impl crate::obj::GodotClass for EditorImportPlugin {
        type Base = crate::classes::ResourceImporter;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorImportPlugin"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorImportPlugin {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::ResourceImporter > for EditorImportPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorImportPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorImportPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorImportPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorImportPlugin {
        type Target = crate::classes::ResourceImporter;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorImportPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorImportPlugin`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorImportPlugin__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorImportPlugin > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::ResourceImporter > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorImportPlugin::append_import_external_resource_ex`][super::EditorImportPlugin::append_import_external_resource_ex]."]
#[must_use]
pub struct ExAppendImportExternalResource < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorImportPlugin, path: CowArg < 'a, GString >, custom_options: CowArg < 'a, VarDictionary >, custom_importer: CowArg < 'a, GString >, generator_parameters: CowArg < 'a, Variant >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAppendImportExternalResource < 'a > {
    fn new(surround_object: &'a mut re_export::EditorImportPlugin, path: impl AsArg < GString > + 'a,) -> Self {
        let custom_options = VarDictionary::new();
        let custom_importer = GString::from("");
        let generator_parameters = Variant::nil();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, path: path.into_arg(), custom_options: CowArg::Owned(custom_options), custom_importer: CowArg::Owned(custom_importer), generator_parameters: CowArg::Owned(generator_parameters),
        }
    }
    #[inline]
    pub fn custom_options(self, custom_options: &'a VarDictionary) -> Self {
        Self {
            custom_options: CowArg::Borrowed(custom_options), .. self
        }
    }
    #[inline]
    pub fn custom_importer(self, custom_importer: impl AsArg < GString > + 'a) -> Self {
        Self {
            custom_importer: custom_importer.into_arg(), .. self
        }
    }
    #[inline]
    pub fn generator_parameters(self, generator_parameters: &'a Variant) -> Self {
        Self {
            generator_parameters: CowArg::Borrowed(generator_parameters), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, path, custom_options, custom_importer, generator_parameters,
        }
        = self;
        re_export::EditorImportPlugin::append_import_external_resource_full(surround_object, path, custom_options.cow_as_arg(), custom_importer, generator_parameters.cow_as_arg(),)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorImportPlugin;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EditorImportPlugin {
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