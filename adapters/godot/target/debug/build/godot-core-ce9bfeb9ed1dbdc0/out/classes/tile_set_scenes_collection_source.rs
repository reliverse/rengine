#![doc = "Sidecar module for class [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileSetScenesCollectionSource` enums](https://docs.godotengine.org/en/stable/classes/class_tilesetscenescollectionsource.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileSetScenesCollectionSource.`\n\nInherits [`TileSetSource`][crate::classes::TileSetSource].\n\nRelated symbols:\n\n* [`tile_set_scenes_collection_source`][crate::classes::tile_set_scenes_collection_source]: sidecar module with related enum/flag types\n* [`ITileSetScenesCollectionSource`][crate::classes::ITileSetScenesCollectionSource]: virtual methods\n\n\nSee also [Godot docs for `TileSetScenesCollectionSource`](https://docs.godotengine.org/en/stable/classes/class_tilesetscenescollectionsource.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TileSetScenesCollectionSource::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileSetScenesCollectionSource {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TileSetScenesCollectionSource`][crate::classes::TileSetScenesCollectionSource].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`ITileSetSource`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `TileSetScenesCollectionSource` methods](https://docs.godotengine.org/en/stable/classes/class_tilesetscenescollectionsource.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileSetScenesCollectionSource: crate::obj::GodotClass < Base = TileSetScenesCollectionSource > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TileSetScenesCollectionSource {
        pub fn get_scene_tiles_count(&mut self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9970usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetScenesCollectionSource", "get_scene_tiles_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scene_tile_id(&mut self, index: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9971usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetScenesCollectionSource", "get_scene_tile_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_scene_tile_id(&mut self, id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9972usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetScenesCollectionSource", "has_scene_tile_id", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn create_scene_tile_full(&mut self, packed_scene: CowArg < Option < Gd < crate::classes::PackedScene > > >, id_override: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::PackedScene > > >, i32,);
            let args = (packed_scene, id_override,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9973usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetScenesCollectionSource", "create_scene_tile", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_scene_tile_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_scene_tile(&mut self, packed_scene: impl AsArg < Option < Gd < crate::classes::PackedScene >> >,) -> i32 {
            self.create_scene_tile_ex(packed_scene,) . done()
        }
        #[inline]
        pub fn create_scene_tile_ex < 'a > (&'a mut self, packed_scene: impl AsArg < Option < Gd < crate::classes::PackedScene >> > + 'a,) -> ExCreateSceneTile < 'a > {
            ExCreateSceneTile::new(self, packed_scene,)
        }
        pub fn set_scene_tile_id(&mut self, id: i32, new_id: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (id, new_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9974usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetScenesCollectionSource", "set_scene_tile_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scene_tile_scene(&mut self, id: i32, packed_scene: impl AsArg < Option < Gd < crate::classes::PackedScene >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::PackedScene > > >,);
            let args = (id, packed_scene.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9975usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetScenesCollectionSource", "set_scene_tile_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scene_tile_scene(&self, id: i32,) -> Option < Gd < crate::classes::PackedScene > > {
            type CallRet = Option < Gd < crate::classes::PackedScene > >;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9976usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetScenesCollectionSource", "get_scene_tile_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_scene_tile_display_placeholder(&mut self, id: i32, display_placeholder: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (id, display_placeholder,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9977usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetScenesCollectionSource", "set_scene_tile_display_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_scene_tile_display_placeholder(&self, id: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9978usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetScenesCollectionSource", "get_scene_tile_display_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_scene_tile(&mut self, id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9979usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetScenesCollectionSource", "remove_scene_tile", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_next_scene_tile_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9980usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileSetScenesCollectionSource", "get_next_scene_tile_id", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TileSetScenesCollectionSource {
        type Base = crate::classes::TileSetSource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TileSetScenesCollectionSource"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileSetScenesCollectionSource {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::TileSetSource > for TileSetScenesCollectionSource {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for TileSetScenesCollectionSource {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TileSetScenesCollectionSource {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TileSetScenesCollectionSource {
        
    }
    impl crate::obj::cap::GodotDefault for TileSetScenesCollectionSource {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileSetScenesCollectionSource {
        type Target = crate::classes::TileSetSource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileSetScenesCollectionSource {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TileSetScenesCollectionSource`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TileSetScenesCollectionSource__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TileSetScenesCollectionSource > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::TileSetSource > for $Class {
                
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
#[doc = "Default-param extender for [`TileSetScenesCollectionSource::create_scene_tile_ex`][super::TileSetScenesCollectionSource::create_scene_tile_ex]."]
#[must_use]
pub struct ExCreateSceneTile < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileSetScenesCollectionSource, packed_scene: CowArg < 'a, Option < Gd < crate::classes::PackedScene > > >, id_override: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateSceneTile < 'a > {
    fn new(surround_object: &'a mut re_export::TileSetScenesCollectionSource, packed_scene: impl AsArg < Option < Gd < crate::classes::PackedScene >> > + 'a,) -> Self {
        let id_override = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, packed_scene: packed_scene.into_arg(), id_override: id_override,
        }
    }
    #[inline]
    pub fn id_override(self, id_override: i32) -> Self {
        Self {
            id_override: id_override, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, packed_scene, id_override,
        }
        = self;
        re_export::TileSetScenesCollectionSource::create_scene_tile_full(surround_object, packed_scene, id_override,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TileSetScenesCollectionSource;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for TileSetScenesCollectionSource {
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