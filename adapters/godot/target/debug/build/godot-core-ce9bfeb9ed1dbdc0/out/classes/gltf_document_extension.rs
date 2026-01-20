#![doc = "Sidecar module for class [`GltfDocumentExtension`][crate::classes::GltfDocumentExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFDocumentExtension` enums](https://docs.godotengine.org/en/stable/classes/class_gltfdocumentextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFDocumentExtension.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`IGltfDocumentExtension`][crate::classes::IGltfDocumentExtension]: virtual methods\n\n\nSee also [Godot docs for `GLTFDocumentExtension`](https://docs.godotengine.org/en/stable/classes/class_gltfdocumentextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GltfDocumentExtension::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfDocumentExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GltfDocumentExtension`][crate::classes::GltfDocumentExtension].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GLTFDocumentExtension` methods](https://docs.godotengine.org/en/stable/classes/class_gltfdocumentextension.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfDocumentExtension: crate::obj::GodotClass < Base = GltfDocumentExtension > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn import_preflight(&mut self, state: Option < Gd < crate::classes::GltfState > >, extensions: PackedStringArray,) -> crate::global::Error {
            unimplemented !()
        }
        fn get_supported_extensions(&mut self,) -> PackedStringArray {
            unimplemented !()
        }
        fn parse_node_extensions(&mut self, state: Option < Gd < crate::classes::GltfState > >, gltf_node: Option < Gd < crate::classes::GltfNode > >, extensions: VarDictionary,) -> crate::global::Error {
            unimplemented !()
        }
        fn parse_image_data(&mut self, state: Option < Gd < crate::classes::GltfState > >, image_data: PackedByteArray, mime_type: GString, ret_image: Option < Gd < crate::classes::Image > >,) -> crate::global::Error {
            unimplemented !()
        }
        fn get_image_file_extension(&mut self,) -> GString {
            unimplemented !()
        }
        fn parse_texture_json(&mut self, state: Option < Gd < crate::classes::GltfState > >, texture_json: VarDictionary, ret_gltf_texture: Option < Gd < crate::classes::GltfTexture > >,) -> crate::global::Error {
            unimplemented !()
        }
        fn import_object_model_property(&mut self, state: Option < Gd < crate::classes::GltfState > >, split_json_pointer: PackedStringArray, partial_paths: Array < NodePath >,) -> Option < Gd < crate::classes::GltfObjectModelProperty > > {
            unimplemented !()
        }
        fn import_post_parse(&mut self, state: Option < Gd < crate::classes::GltfState > >,) -> crate::global::Error {
            unimplemented !()
        }
        fn import_pre_generate(&mut self, state: Option < Gd < crate::classes::GltfState > >,) -> crate::global::Error {
            unimplemented !()
        }
        fn generate_scene_node(&mut self, state: Option < Gd < crate::classes::GltfState > >, gltf_node: Option < Gd < crate::classes::GltfNode > >, scene_parent: Option < Gd < crate::classes::Node > >,) -> Option < Gd < crate::classes::Node3D > > {
            unimplemented !()
        }
        fn import_node(&mut self, state: Option < Gd < crate::classes::GltfState > >, gltf_node: Option < Gd < crate::classes::GltfNode > >, json: VarDictionary, node: Option < Gd < crate::classes::Node > >,) -> crate::global::Error {
            unimplemented !()
        }
        fn import_post(&mut self, state: Option < Gd < crate::classes::GltfState > >, root: Option < Gd < crate::classes::Node > >,) -> crate::global::Error {
            unimplemented !()
        }
        fn export_preflight(&mut self, state: Option < Gd < crate::classes::GltfState > >, root: Option < Gd < crate::classes::Node > >,) -> crate::global::Error {
            unimplemented !()
        }
        fn convert_scene_node(&mut self, state: Option < Gd < crate::classes::GltfState > >, gltf_node: Option < Gd < crate::classes::GltfNode > >, scene_node: Option < Gd < crate::classes::Node > >,) {
            unimplemented !()
        }
        fn export_post_convert(&mut self, state: Option < Gd < crate::classes::GltfState > >, root: Option < Gd < crate::classes::Node > >,) -> crate::global::Error {
            unimplemented !()
        }
        fn export_preserialize(&mut self, state: Option < Gd < crate::classes::GltfState > >,) -> crate::global::Error {
            unimplemented !()
        }
        fn export_object_model_property(&mut self, state: Option < Gd < crate::classes::GltfState > >, node_path: NodePath, godot_node: Option < Gd < crate::classes::Node > >, gltf_node_index: i32, target_object: Option < Gd < crate::classes::Object > >, target_depth: i32,) -> Option < Gd < crate::classes::GltfObjectModelProperty > > {
            unimplemented !()
        }
        fn get_saveable_image_formats(&mut self,) -> PackedStringArray {
            unimplemented !()
        }
        fn serialize_image_to_bytes(&mut self, state: Option < Gd < crate::classes::GltfState > >, image: Option < Gd < crate::classes::Image > >, image_dict: VarDictionary, image_format: GString, lossy_quality: f32,) -> PackedByteArray {
            unimplemented !()
        }
        fn save_image_at_path(&mut self, state: Option < Gd < crate::classes::GltfState > >, image: Option < Gd < crate::classes::Image > >, file_path: GString, image_format: GString, lossy_quality: f32,) -> crate::global::Error {
            unimplemented !()
        }
        fn serialize_texture_json(&mut self, state: Option < Gd < crate::classes::GltfState > >, texture_json: VarDictionary, gltf_texture: Option < Gd < crate::classes::GltfTexture > >, image_format: GString,) -> crate::global::Error {
            unimplemented !()
        }
        fn export_node(&mut self, state: Option < Gd < crate::classes::GltfState > >, gltf_node: Option < Gd < crate::classes::GltfNode > >, json: VarDictionary, node: Option < Gd < crate::classes::Node > >,) -> crate::global::Error {
            unimplemented !()
        }
        fn export_post(&mut self, state: Option < Gd < crate::classes::GltfState > >,) -> crate::global::Error {
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
    impl GltfDocumentExtension {
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
    impl crate::obj::GodotClass for GltfDocumentExtension {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GLTFDocumentExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfDocumentExtension {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for GltfDocumentExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GltfDocumentExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GltfDocumentExtension {
        
    }
    impl crate::obj::cap::GodotDefault for GltfDocumentExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfDocumentExtension {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfDocumentExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GltfDocumentExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GltfDocumentExtension__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfDocumentExtension > for $Class {
                
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GltfDocumentExtension;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for GltfDocumentExtension {
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