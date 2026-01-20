#![doc = "Sidecar module for class [`TileMapPattern`][crate::classes::TileMapPattern].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileMapPattern` enums](https://docs.godotengine.org/en/stable/classes/class_tilemappattern.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TileMapPattern.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`tile_map_pattern`][crate::classes::tile_map_pattern]: sidecar module with related enum/flag types\n* [`ITileMapPattern`][crate::classes::ITileMapPattern]: virtual methods\n\n\nSee also [Godot docs for `TileMapPattern`](https://docs.godotengine.org/en/stable/classes/class_tilemappattern.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`TileMapPattern::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TileMapPattern {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`TileMapPattern`][crate::classes::TileMapPattern].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `TileMapPattern` methods](https://docs.godotengine.org/en/stable/classes/class_tilemappattern.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITileMapPattern: crate::obj::GodotClass < Base = TileMapPattern > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl TileMapPattern {
        pub(crate) fn set_cell_full(&mut self, coords: Vector2i, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,) {
            type CallRet = ();
            type CallParams = (Vector2i, i32, Vector2i, i32,);
            let args = (coords, source_id, atlas_coords, alternative_tile,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9830usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapPattern", "set_cell", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_cell_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_cell(&mut self, coords: Vector2i,) {
            self.set_cell_ex(coords,) . done()
        }
        #[inline]
        pub fn set_cell_ex < 'a > (&'a mut self, coords: Vector2i,) -> ExSetCell < 'a > {
            ExSetCell::new(self, coords,)
        }
        pub fn has_cell(&self, coords: Vector2i,) -> bool {
            type CallRet = bool;
            type CallParams = (Vector2i,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9831usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapPattern", "has_cell", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_cell(&mut self, coords: Vector2i, update_size: bool,) {
            type CallRet = ();
            type CallParams = (Vector2i, bool,);
            let args = (coords, update_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9832usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapPattern", "remove_cell", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cell_source_id(&self, coords: Vector2i,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2i,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9833usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapPattern", "get_cell_source_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cell_atlas_coords(&self, coords: Vector2i,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = (Vector2i,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9834usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapPattern", "get_cell_atlas_coords", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_cell_alternative_tile(&self, coords: Vector2i,) -> i32 {
            type CallRet = i32;
            type CallParams = (Vector2i,);
            let args = (coords,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9835usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapPattern", "get_cell_alternative_tile", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_used_cells(&self,) -> Array < Vector2i > {
            type CallRet = Array < Vector2i >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9836usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapPattern", "get_used_cells", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9837usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapPattern", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_size(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9838usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapPattern", "set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_empty(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9839usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TileMapPattern", "is_empty", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TileMapPattern {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TileMapPattern"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TileMapPattern {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for TileMapPattern {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TileMapPattern {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TileMapPattern {
        
    }
    impl crate::obj::cap::GodotDefault for TileMapPattern {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for TileMapPattern {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TileMapPattern {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TileMapPattern`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TileMapPattern__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TileMapPattern > for $Class {
                
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
#[doc = "Default-param extender for [`TileMapPattern::set_cell_ex`][super::TileMapPattern::set_cell_ex]."]
#[must_use]
pub struct ExSetCell < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::TileMapPattern, coords: Vector2i, source_id: i32, atlas_coords: Vector2i, alternative_tile: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetCell < 'a > {
    fn new(surround_object: &'a mut re_export::TileMapPattern, coords: Vector2i,) -> Self {
        let source_id = - 1i32;
        let atlas_coords = Vector2i::new(- 1 as _, - 1 as _);
        let alternative_tile = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, coords: coords, source_id: source_id, atlas_coords: atlas_coords, alternative_tile: alternative_tile,
        }
    }
    #[inline]
    pub fn source_id(self, source_id: i32) -> Self {
        Self {
            source_id: source_id, .. self
        }
    }
    #[inline]
    pub fn atlas_coords(self, atlas_coords: Vector2i) -> Self {
        Self {
            atlas_coords: atlas_coords, .. self
        }
    }
    #[inline]
    pub fn alternative_tile(self, alternative_tile: i32) -> Self {
        Self {
            alternative_tile: alternative_tile, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, coords, source_id, atlas_coords, alternative_tile,
        }
        = self;
        re_export::TileMapPattern::set_cell_full(surround_object, coords, source_id, atlas_coords, alternative_tile,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TileMapPattern;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for TileMapPattern {
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