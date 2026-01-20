#![doc = "Sidecar module for class [`Mesh`][crate::classes::Mesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Mesh` enums](https://docs.godotengine.org/en/stable/classes/class_mesh.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Mesh.`\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`mesh`][crate::classes::mesh]: sidecar module with related enum/flag types\n* [`IMesh`][crate::classes::IMesh]: virtual methods\n\n\nSee also [Godot docs for `Mesh`](https://docs.godotengine.org/en/stable/classes/class_mesh.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`Mesh::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Mesh {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Mesh`][crate::classes::Mesh].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Mesh` methods](https://docs.godotengine.org/en/stable/classes/class_mesh.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMesh: crate::obj::GodotClass < Base = Mesh > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_surface_count(&self,) -> i32;
        fn surface_get_array_len(&self, index: i32,) -> i32;
        fn surface_get_array_index_len(&self, index: i32,) -> i32;
        fn surface_get_arrays(&self, index: i32,) -> VarArray;
        fn surface_get_blend_shape_arrays(&self, index: i32,) -> Array < VarArray >;
        fn surface_get_lods(&self, index: i32,) -> VarDictionary;
        fn surface_get_format(&self, index: i32,) -> u32;
        fn surface_get_primitive_type(&self, index: i32,) -> u32;
        fn surface_set_material(&mut self, index: i32, material: Option < Gd < crate::classes::Material > >,);
        fn surface_get_material(&self, index: i32,) -> Option < Gd < crate::classes::Material > >;
        fn get_blend_shape_count(&self,) -> i32;
        fn get_blend_shape_name(&self, index: i32,) -> StringName;
        fn set_blend_shape_name(&mut self, index: i32, name: StringName,);
        fn get_aabb(&self,) -> Aabb;
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
    impl Mesh {
        pub fn set_lightmap_size_hint(&mut self, size: Vector2i,) {
            type CallRet = ();
            type CallParams = (Vector2i,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "set_lightmap_size_hint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_lightmap_size_hint(&self,) -> Vector2i {
            type CallRet = Vector2i;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "get_lightmap_size_hint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_aabb(&self,) -> Aabb {
            type CallRet = Aabb;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "get_aabb", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_faces(&self,) -> PackedVector3Array {
            type CallRet = PackedVector3Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "get_faces", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_surface_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "get_surface_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_get_arrays(&self, surf_idx: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "surface_get_arrays", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_get_blend_shape_arrays(&self, surf_idx: i32,) -> Array < VarArray > {
            type CallRet = Array < VarArray >;
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "surface_get_blend_shape_arrays", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_set_material(&mut self, surf_idx: i32, material: impl AsArg < Option < Gd < crate::classes::Material >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Material > > >,);
            let args = (surf_idx, material.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "surface_set_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn surface_get_material(&self, surf_idx: i32,) -> Option < Gd < crate::classes::Material > > {
            type CallRet = Option < Gd < crate::classes::Material > >;
            type CallParams = (i32,);
            let args = (surf_idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "surface_get_material", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_placeholder(&self,) -> Option < Gd < crate::classes::Resource > > {
            type CallRet = Option < Gd < crate::classes::Resource > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5274usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "create_placeholder", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_trimesh_shape(&self,) -> Option < Gd < crate::classes::ConcavePolygonShape3D > > {
            type CallRet = Option < Gd < crate::classes::ConcavePolygonShape3D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5275usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "create_trimesh_shape", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn create_convex_shape_full(&self, clean: bool, simplify: bool,) -> Option < Gd < crate::classes::ConvexPolygonShape3D > > {
            type CallRet = Option < Gd < crate::classes::ConvexPolygonShape3D > >;
            type CallParams = (bool, bool,);
            let args = (clean, simplify,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5276usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "create_convex_shape", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_convex_shape_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_convex_shape(&self,) -> Option < Gd < crate::classes::ConvexPolygonShape3D > > {
            self.create_convex_shape_ex() . done()
        }
        #[inline]
        pub fn create_convex_shape_ex < 'a > (&'a self,) -> ExCreateConvexShape < 'a > {
            ExCreateConvexShape::new(self,)
        }
        pub fn create_outline(&self, margin: f32,) -> Option < Gd < crate::classes::Mesh > > {
            type CallRet = Option < Gd < crate::classes::Mesh > >;
            type CallParams = (f32,);
            let args = (margin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5277usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "create_outline", Some(self.__validated_obj()), args,)
            }
        }
        pub fn generate_triangle_mesh(&self,) -> Option < Gd < crate::classes::TriangleMesh > > {
            type CallRet = Option < Gd < crate::classes::TriangleMesh > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5278usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Mesh", "generate_triangle_mesh", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Mesh {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Mesh"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Mesh {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for Mesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Mesh {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Mesh {
        
    }
    impl crate::obj::cap::GodotDefault for Mesh {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Mesh {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Mesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Mesh`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Mesh__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Mesh > for $Class {
                
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
#[doc = "Default-param extender for [`Mesh::create_convex_shape_ex`][super::Mesh::create_convex_shape_ex]."]
#[must_use]
pub struct ExCreateConvexShape < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::Mesh, clean: bool, simplify: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateConvexShape < 'a > {
    fn new(surround_object: &'a re_export::Mesh,) -> Self {
        let clean = true;
        let simplify = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, clean: clean, simplify: simplify,
        }
    }
    #[inline]
    pub fn clean(self, clean: bool) -> Self {
        Self {
            clean: clean, .. self
        }
    }
    #[inline]
    pub fn simplify(self, simplify: bool) -> Self {
        Self {
            simplify: simplify, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::ConvexPolygonShape3D > > {
        let Self {
            _phantom, surround_object, clean, simplify,
        }
        = self;
        re_export::Mesh::create_convex_shape_full(surround_object, clean, simplify,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PrimitiveType {
    ord: i32
}
impl PrimitiveType {
    #[doc(alias = "PRIMITIVE_POINTS")]
    #[doc = "Godot enumerator name: `PRIMITIVE_POINTS`"]
    pub const POINTS: PrimitiveType = PrimitiveType {
        ord: 0i32
    };
    #[doc(alias = "PRIMITIVE_LINES")]
    #[doc = "Godot enumerator name: `PRIMITIVE_LINES`"]
    pub const LINES: PrimitiveType = PrimitiveType {
        ord: 1i32
    };
    #[doc(alias = "PRIMITIVE_LINE_STRIP")]
    #[doc = "Godot enumerator name: `PRIMITIVE_LINE_STRIP`"]
    pub const LINE_STRIP: PrimitiveType = PrimitiveType {
        ord: 2i32
    };
    #[doc(alias = "PRIMITIVE_TRIANGLES")]
    #[doc = "Godot enumerator name: `PRIMITIVE_TRIANGLES`"]
    pub const TRIANGLES: PrimitiveType = PrimitiveType {
        ord: 3i32
    };
    #[doc(alias = "PRIMITIVE_TRIANGLE_STRIP")]
    #[doc = "Godot enumerator name: `PRIMITIVE_TRIANGLE_STRIP`"]
    pub const TRIANGLE_STRIP: PrimitiveType = PrimitiveType {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PrimitiveType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PrimitiveType {
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
            Self::POINTS => "POINTS", Self::LINES => "LINES", Self::LINE_STRIP => "LINE_STRIP", Self::TRIANGLES => "TRIANGLES", Self::TRIANGLE_STRIP => "TRIANGLE_STRIP", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PrimitiveType::POINTS, PrimitiveType::LINES, PrimitiveType::LINE_STRIP, PrimitiveType::TRIANGLES, PrimitiveType::TRIANGLE_STRIP]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PrimitiveType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("POINTS", "PRIMITIVE_POINTS", PrimitiveType::POINTS), crate::meta::inspect::EnumConstant::new("LINES", "PRIMITIVE_LINES", PrimitiveType::LINES), crate::meta::inspect::EnumConstant::new("LINE_STRIP", "PRIMITIVE_LINE_STRIP", PrimitiveType::LINE_STRIP), crate::meta::inspect::EnumConstant::new("TRIANGLES", "PRIMITIVE_TRIANGLES", PrimitiveType::TRIANGLES), crate::meta::inspect::EnumConstant::new("TRIANGLE_STRIP", "PRIMITIVE_TRIANGLE_STRIP", PrimitiveType::TRIANGLE_STRIP)]
        }
    }
}
impl crate::meta::GodotConvert for PrimitiveType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PrimitiveType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PrimitiveType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ArrayType {
    ord: i32
}
impl ArrayType {
    #[doc(alias = "ARRAY_VERTEX")]
    #[doc = "Godot enumerator name: `ARRAY_VERTEX`"]
    pub const VERTEX: ArrayType = ArrayType {
        ord: 0i32
    };
    #[doc(alias = "ARRAY_NORMAL")]
    #[doc = "Godot enumerator name: `ARRAY_NORMAL`"]
    pub const NORMAL: ArrayType = ArrayType {
        ord: 1i32
    };
    #[doc(alias = "ARRAY_TANGENT")]
    #[doc = "Godot enumerator name: `ARRAY_TANGENT`"]
    pub const TANGENT: ArrayType = ArrayType {
        ord: 2i32
    };
    #[doc(alias = "ARRAY_COLOR")]
    #[doc = "Godot enumerator name: `ARRAY_COLOR`"]
    pub const COLOR: ArrayType = ArrayType {
        ord: 3i32
    };
    #[doc(alias = "ARRAY_TEX_UV")]
    #[doc = "Godot enumerator name: `ARRAY_TEX_UV`"]
    pub const TEX_UV: ArrayType = ArrayType {
        ord: 4i32
    };
    #[doc(alias = "ARRAY_TEX_UV2")]
    #[doc = "Godot enumerator name: `ARRAY_TEX_UV2`"]
    pub const TEX_UV2: ArrayType = ArrayType {
        ord: 5i32
    };
    #[doc(alias = "ARRAY_CUSTOM0")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM0`"]
    pub const CUSTOM0: ArrayType = ArrayType {
        ord: 6i32
    };
    #[doc(alias = "ARRAY_CUSTOM1")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM1`"]
    pub const CUSTOM1: ArrayType = ArrayType {
        ord: 7i32
    };
    #[doc(alias = "ARRAY_CUSTOM2")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM2`"]
    pub const CUSTOM2: ArrayType = ArrayType {
        ord: 8i32
    };
    #[doc(alias = "ARRAY_CUSTOM3")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM3`"]
    pub const CUSTOM3: ArrayType = ArrayType {
        ord: 9i32
    };
    #[doc(alias = "ARRAY_BONES")]
    #[doc = "Godot enumerator name: `ARRAY_BONES`"]
    pub const BONES: ArrayType = ArrayType {
        ord: 10i32
    };
    #[doc(alias = "ARRAY_WEIGHTS")]
    #[doc = "Godot enumerator name: `ARRAY_WEIGHTS`"]
    pub const WEIGHTS: ArrayType = ArrayType {
        ord: 11i32
    };
    #[doc(alias = "ARRAY_INDEX")]
    #[doc = "Godot enumerator name: `ARRAY_INDEX`"]
    pub const INDEX: ArrayType = ArrayType {
        ord: 12i32
    };
    #[doc(alias = "ARRAY_MAX")]
    #[doc = "Godot enumerator name: `ARRAY_MAX`"]
    pub const MAX: ArrayType = ArrayType {
        ord: 13i32
    };
    
}
impl std::fmt::Debug for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ArrayType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ArrayType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
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
            Self::VERTEX => "VERTEX", Self::NORMAL => "NORMAL", Self::TANGENT => "TANGENT", Self::COLOR => "COLOR", Self::TEX_UV => "TEX_UV", Self::TEX_UV2 => "TEX_UV2", Self::CUSTOM0 => "CUSTOM0", Self::CUSTOM1 => "CUSTOM1", Self::CUSTOM2 => "CUSTOM2", Self::CUSTOM3 => "CUSTOM3", Self::BONES => "BONES", Self::WEIGHTS => "WEIGHTS", Self::INDEX => "INDEX", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ArrayType::VERTEX, ArrayType::NORMAL, ArrayType::TANGENT, ArrayType::COLOR, ArrayType::TEX_UV, ArrayType::TEX_UV2, ArrayType::CUSTOM0, ArrayType::CUSTOM1, ArrayType::CUSTOM2, ArrayType::CUSTOM3, ArrayType::BONES, ArrayType::WEIGHTS, ArrayType::INDEX]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ArrayType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERTEX", "ARRAY_VERTEX", ArrayType::VERTEX), crate::meta::inspect::EnumConstant::new("NORMAL", "ARRAY_NORMAL", ArrayType::NORMAL), crate::meta::inspect::EnumConstant::new("TANGENT", "ARRAY_TANGENT", ArrayType::TANGENT), crate::meta::inspect::EnumConstant::new("COLOR", "ARRAY_COLOR", ArrayType::COLOR), crate::meta::inspect::EnumConstant::new("TEX_UV", "ARRAY_TEX_UV", ArrayType::TEX_UV), crate::meta::inspect::EnumConstant::new("TEX_UV2", "ARRAY_TEX_UV2", ArrayType::TEX_UV2), crate::meta::inspect::EnumConstant::new("CUSTOM0", "ARRAY_CUSTOM0", ArrayType::CUSTOM0), crate::meta::inspect::EnumConstant::new("CUSTOM1", "ARRAY_CUSTOM1", ArrayType::CUSTOM1), crate::meta::inspect::EnumConstant::new("CUSTOM2", "ARRAY_CUSTOM2", ArrayType::CUSTOM2), crate::meta::inspect::EnumConstant::new("CUSTOM3", "ARRAY_CUSTOM3", ArrayType::CUSTOM3), crate::meta::inspect::EnumConstant::new("BONES", "ARRAY_BONES", ArrayType::BONES), crate::meta::inspect::EnumConstant::new("WEIGHTS", "ARRAY_WEIGHTS", ArrayType::WEIGHTS), crate::meta::inspect::EnumConstant::new("INDEX", "ARRAY_INDEX", ArrayType::INDEX), crate::meta::inspect::EnumConstant::new("MAX", "ARRAY_MAX", ArrayType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ArrayType {
    const ENUMERATOR_COUNT: usize = 13usize;
    
}
impl crate::meta::GodotConvert for ArrayType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ArrayType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ArrayType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ArrayCustomFormat {
    ord: i32
}
impl ArrayCustomFormat {
    #[doc(alias = "ARRAY_CUSTOM_RGBA8_UNORM")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGBA8_UNORM`"]
    pub const RGBA8_UNORM: ArrayCustomFormat = ArrayCustomFormat {
        ord: 0i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RGBA8_SNORM")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGBA8_SNORM`"]
    pub const RGBA8_SNORM: ArrayCustomFormat = ArrayCustomFormat {
        ord: 1i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RG_HALF")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RG_HALF`"]
    pub const RG_HALF: ArrayCustomFormat = ArrayCustomFormat {
        ord: 2i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RGBA_HALF")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGBA_HALF`"]
    pub const RGBA_HALF: ArrayCustomFormat = ArrayCustomFormat {
        ord: 3i32
    };
    #[doc(alias = "ARRAY_CUSTOM_R_FLOAT")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_R_FLOAT`"]
    pub const R_FLOAT: ArrayCustomFormat = ArrayCustomFormat {
        ord: 4i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RG_FLOAT")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RG_FLOAT`"]
    pub const RG_FLOAT: ArrayCustomFormat = ArrayCustomFormat {
        ord: 5i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RGB_FLOAT")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGB_FLOAT`"]
    pub const RGB_FLOAT: ArrayCustomFormat = ArrayCustomFormat {
        ord: 6i32
    };
    #[doc(alias = "ARRAY_CUSTOM_RGBA_FLOAT")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_RGBA_FLOAT`"]
    pub const RGBA_FLOAT: ArrayCustomFormat = ArrayCustomFormat {
        ord: 7i32
    };
    #[doc(alias = "ARRAY_CUSTOM_MAX")]
    #[doc = "Godot enumerator name: `ARRAY_CUSTOM_MAX`"]
    pub const MAX: ArrayCustomFormat = ArrayCustomFormat {
        ord: 8i32
    };
    
}
impl std::fmt::Debug for ArrayCustomFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ArrayCustomFormat") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ArrayCustomFormat {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 => Some(Self {
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
            Self::RGBA8_UNORM => "RGBA8_UNORM", Self::RGBA8_SNORM => "RGBA8_SNORM", Self::RG_HALF => "RG_HALF", Self::RGBA_HALF => "RGBA_HALF", Self::R_FLOAT => "R_FLOAT", Self::RG_FLOAT => "RG_FLOAT", Self::RGB_FLOAT => "RGB_FLOAT", Self::RGBA_FLOAT => "RGBA_FLOAT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ArrayCustomFormat::RGBA8_UNORM, ArrayCustomFormat::RGBA8_SNORM, ArrayCustomFormat::RG_HALF, ArrayCustomFormat::RGBA_HALF, ArrayCustomFormat::R_FLOAT, ArrayCustomFormat::RG_FLOAT, ArrayCustomFormat::RGB_FLOAT, ArrayCustomFormat::RGBA_FLOAT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ArrayCustomFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RGBA8_UNORM", "ARRAY_CUSTOM_RGBA8_UNORM", ArrayCustomFormat::RGBA8_UNORM), crate::meta::inspect::EnumConstant::new("RGBA8_SNORM", "ARRAY_CUSTOM_RGBA8_SNORM", ArrayCustomFormat::RGBA8_SNORM), crate::meta::inspect::EnumConstant::new("RG_HALF", "ARRAY_CUSTOM_RG_HALF", ArrayCustomFormat::RG_HALF), crate::meta::inspect::EnumConstant::new("RGBA_HALF", "ARRAY_CUSTOM_RGBA_HALF", ArrayCustomFormat::RGBA_HALF), crate::meta::inspect::EnumConstant::new("R_FLOAT", "ARRAY_CUSTOM_R_FLOAT", ArrayCustomFormat::R_FLOAT), crate::meta::inspect::EnumConstant::new("RG_FLOAT", "ARRAY_CUSTOM_RG_FLOAT", ArrayCustomFormat::RG_FLOAT), crate::meta::inspect::EnumConstant::new("RGB_FLOAT", "ARRAY_CUSTOM_RGB_FLOAT", ArrayCustomFormat::RGB_FLOAT), crate::meta::inspect::EnumConstant::new("RGBA_FLOAT", "ARRAY_CUSTOM_RGBA_FLOAT", ArrayCustomFormat::RGBA_FLOAT), crate::meta::inspect::EnumConstant::new("MAX", "ARRAY_CUSTOM_MAX", ArrayCustomFormat::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for ArrayCustomFormat {
    const ENUMERATOR_COUNT: usize = 8usize;
    
}
impl crate::meta::GodotConvert for ArrayCustomFormat {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ArrayCustomFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ArrayCustomFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct ArrayFormat {
    ord: u64
}
impl ArrayFormat {
    #[doc(alias = "ARRAY_FORMAT_VERTEX")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_VERTEX`"]
    pub const VERTEX: ArrayFormat = ArrayFormat {
        ord: 1u64
    };
    #[doc(alias = "ARRAY_FORMAT_NORMAL")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_NORMAL`"]
    pub const NORMAL: ArrayFormat = ArrayFormat {
        ord: 2u64
    };
    #[doc(alias = "ARRAY_FORMAT_TANGENT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_TANGENT`"]
    pub const TANGENT: ArrayFormat = ArrayFormat {
        ord: 4u64
    };
    #[doc(alias = "ARRAY_FORMAT_COLOR")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_COLOR`"]
    pub const COLOR: ArrayFormat = ArrayFormat {
        ord: 8u64
    };
    #[doc(alias = "ARRAY_FORMAT_TEX_UV")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_TEX_UV`"]
    pub const TEX_UV: ArrayFormat = ArrayFormat {
        ord: 16u64
    };
    #[doc(alias = "ARRAY_FORMAT_TEX_UV2")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_TEX_UV2`"]
    pub const TEX_UV2: ArrayFormat = ArrayFormat {
        ord: 32u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM0")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM0`"]
    pub const CUSTOM0: ArrayFormat = ArrayFormat {
        ord: 64u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM1")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM1`"]
    pub const CUSTOM1: ArrayFormat = ArrayFormat {
        ord: 128u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM2")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM2`"]
    pub const CUSTOM2: ArrayFormat = ArrayFormat {
        ord: 256u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM3")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM3`"]
    pub const CUSTOM3: ArrayFormat = ArrayFormat {
        ord: 512u64
    };
    #[doc(alias = "ARRAY_FORMAT_BONES")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_BONES`"]
    pub const BONES: ArrayFormat = ArrayFormat {
        ord: 1024u64
    };
    #[doc(alias = "ARRAY_FORMAT_WEIGHTS")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_WEIGHTS`"]
    pub const WEIGHTS: ArrayFormat = ArrayFormat {
        ord: 2048u64
    };
    #[doc(alias = "ARRAY_FORMAT_INDEX")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_INDEX`"]
    pub const INDEX: ArrayFormat = ArrayFormat {
        ord: 4096u64
    };
    #[doc(alias = "ARRAY_FORMAT_BLEND_SHAPE_MASK")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_BLEND_SHAPE_MASK`"]
    pub const BLEND_SHAPE_MASK: ArrayFormat = ArrayFormat {
        ord: 7u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM_BASE")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM_BASE`"]
    pub const CUSTOM_BASE: ArrayFormat = ArrayFormat {
        ord: 13u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM_BITS")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM_BITS`"]
    pub const CUSTOM_BITS: ArrayFormat = ArrayFormat {
        ord: 3u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM0_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM0_SHIFT`"]
    pub const CUSTOM0_SHIFT: ArrayFormat = ArrayFormat {
        ord: 13u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM1_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM1_SHIFT`"]
    pub const CUSTOM1_SHIFT: ArrayFormat = ArrayFormat {
        ord: 16u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM2_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM2_SHIFT`"]
    pub const CUSTOM2_SHIFT: ArrayFormat = ArrayFormat {
        ord: 19u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM3_SHIFT")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM3_SHIFT`"]
    pub const CUSTOM3_SHIFT: ArrayFormat = ArrayFormat {
        ord: 22u64
    };
    #[doc(alias = "ARRAY_FORMAT_CUSTOM_MASK")]
    #[doc = "Godot enumerator name: `ARRAY_FORMAT_CUSTOM_MASK`"]
    pub const CUSTOM_MASK: ArrayFormat = ArrayFormat {
        ord: 7u64
    };
    #[doc(alias = "ARRAY_COMPRESS_FLAGS_BASE")]
    #[doc = "Godot enumerator name: `ARRAY_COMPRESS_FLAGS_BASE`"]
    pub const COMPRESS_FLAGS_BASE: ArrayFormat = ArrayFormat {
        ord: 25u64
    };
    #[doc(alias = "ARRAY_FLAG_USE_2D_VERTICES")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_USE_2D_VERTICES`"]
    pub const FLAG_USE_2D_VERTICES: ArrayFormat = ArrayFormat {
        ord: 33554432u64
    };
    #[doc(alias = "ARRAY_FLAG_USE_DYNAMIC_UPDATE")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_USE_DYNAMIC_UPDATE`"]
    pub const FLAG_USE_DYNAMIC_UPDATE: ArrayFormat = ArrayFormat {
        ord: 67108864u64
    };
    #[doc(alias = "ARRAY_FLAG_USE_8_BONE_WEIGHTS")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_USE_8_BONE_WEIGHTS`"]
    pub const FLAG_USE_8_BONE_WEIGHTS: ArrayFormat = ArrayFormat {
        ord: 134217728u64
    };
    #[doc(alias = "ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY`"]
    pub const FLAG_USES_EMPTY_VERTEX_ARRAY: ArrayFormat = ArrayFormat {
        ord: 268435456u64
    };
    #[doc(alias = "ARRAY_FLAG_COMPRESS_ATTRIBUTES")]
    #[doc = "Godot enumerator name: `ARRAY_FLAG_COMPRESS_ATTRIBUTES`"]
    pub const FLAG_COMPRESS_ATTRIBUTES: ArrayFormat = ArrayFormat {
        ord: 536870912u64
    };
    
}
impl std::fmt::Debug for ArrayFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::VERTEX => "VERTEX", Self::NORMAL => "NORMAL", Self::TANGENT => "TANGENT", Self::COLOR => "COLOR", Self::TEX_UV => "TEX_UV", Self::TEX_UV2 => "TEX_UV2", Self::CUSTOM0 => "CUSTOM0", Self::CUSTOM1 => "CUSTOM1", Self::CUSTOM2 => "CUSTOM2", Self::CUSTOM3 => "CUSTOM3", Self::BONES => "BONES", Self::WEIGHTS => "WEIGHTS", Self::INDEX => "INDEX", Self::BLEND_SHAPE_MASK => "BLEND_SHAPE_MASK", Self::CUSTOM_BASE => "CUSTOM_BASE", Self::CUSTOM_BITS => "CUSTOM_BITS", Self::CUSTOM0_SHIFT => "CUSTOM0_SHIFT", Self::CUSTOM1_SHIFT => "CUSTOM1_SHIFT", Self::CUSTOM2_SHIFT => "CUSTOM2_SHIFT", Self::CUSTOM3_SHIFT => "CUSTOM3_SHIFT", Self::CUSTOM_MASK => "CUSTOM_MASK", Self::COMPRESS_FLAGS_BASE => "COMPRESS_FLAGS_BASE", Self::FLAG_USE_2D_VERTICES => "FLAG_USE_2D_VERTICES", Self::FLAG_USE_DYNAMIC_UPDATE => "FLAG_USE_DYNAMIC_UPDATE", Self::FLAG_USE_8_BONE_WEIGHTS => "FLAG_USE_8_BONE_WEIGHTS", Self::FLAG_USES_EMPTY_VERTEX_ARRAY => "FLAG_USES_EMPTY_VERTEX_ARRAY", Self::FLAG_COMPRESS_ATTRIBUTES => "FLAG_COMPRESS_ATTRIBUTES", _ => {
                f.debug_struct("ArrayFormat") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for ArrayFormat {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ArrayFormat >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("VERTEX", "ARRAY_FORMAT_VERTEX", ArrayFormat::VERTEX), crate::meta::inspect::EnumConstant::new("NORMAL", "ARRAY_FORMAT_NORMAL", ArrayFormat::NORMAL), crate::meta::inspect::EnumConstant::new("TANGENT", "ARRAY_FORMAT_TANGENT", ArrayFormat::TANGENT), crate::meta::inspect::EnumConstant::new("COLOR", "ARRAY_FORMAT_COLOR", ArrayFormat::COLOR), crate::meta::inspect::EnumConstant::new("TEX_UV", "ARRAY_FORMAT_TEX_UV", ArrayFormat::TEX_UV), crate::meta::inspect::EnumConstant::new("TEX_UV2", "ARRAY_FORMAT_TEX_UV2", ArrayFormat::TEX_UV2), crate::meta::inspect::EnumConstant::new("CUSTOM0", "ARRAY_FORMAT_CUSTOM0", ArrayFormat::CUSTOM0), crate::meta::inspect::EnumConstant::new("CUSTOM1", "ARRAY_FORMAT_CUSTOM1", ArrayFormat::CUSTOM1), crate::meta::inspect::EnumConstant::new("CUSTOM2", "ARRAY_FORMAT_CUSTOM2", ArrayFormat::CUSTOM2), crate::meta::inspect::EnumConstant::new("CUSTOM3", "ARRAY_FORMAT_CUSTOM3", ArrayFormat::CUSTOM3), crate::meta::inspect::EnumConstant::new("BONES", "ARRAY_FORMAT_BONES", ArrayFormat::BONES), crate::meta::inspect::EnumConstant::new("WEIGHTS", "ARRAY_FORMAT_WEIGHTS", ArrayFormat::WEIGHTS), crate::meta::inspect::EnumConstant::new("INDEX", "ARRAY_FORMAT_INDEX", ArrayFormat::INDEX), crate::meta::inspect::EnumConstant::new("BLEND_SHAPE_MASK", "ARRAY_FORMAT_BLEND_SHAPE_MASK", ArrayFormat::BLEND_SHAPE_MASK), crate::meta::inspect::EnumConstant::new("CUSTOM_BASE", "ARRAY_FORMAT_CUSTOM_BASE", ArrayFormat::CUSTOM_BASE), crate::meta::inspect::EnumConstant::new("CUSTOM_BITS", "ARRAY_FORMAT_CUSTOM_BITS", ArrayFormat::CUSTOM_BITS), crate::meta::inspect::EnumConstant::new("CUSTOM0_SHIFT", "ARRAY_FORMAT_CUSTOM0_SHIFT", ArrayFormat::CUSTOM0_SHIFT), crate::meta::inspect::EnumConstant::new("CUSTOM1_SHIFT", "ARRAY_FORMAT_CUSTOM1_SHIFT", ArrayFormat::CUSTOM1_SHIFT), crate::meta::inspect::EnumConstant::new("CUSTOM2_SHIFT", "ARRAY_FORMAT_CUSTOM2_SHIFT", ArrayFormat::CUSTOM2_SHIFT), crate::meta::inspect::EnumConstant::new("CUSTOM3_SHIFT", "ARRAY_FORMAT_CUSTOM3_SHIFT", ArrayFormat::CUSTOM3_SHIFT), crate::meta::inspect::EnumConstant::new("CUSTOM_MASK", "ARRAY_FORMAT_CUSTOM_MASK", ArrayFormat::CUSTOM_MASK), crate::meta::inspect::EnumConstant::new("COMPRESS_FLAGS_BASE", "ARRAY_COMPRESS_FLAGS_BASE", ArrayFormat::COMPRESS_FLAGS_BASE), crate::meta::inspect::EnumConstant::new("FLAG_USE_2D_VERTICES", "ARRAY_FLAG_USE_2D_VERTICES", ArrayFormat::FLAG_USE_2D_VERTICES), crate::meta::inspect::EnumConstant::new("FLAG_USE_DYNAMIC_UPDATE", "ARRAY_FLAG_USE_DYNAMIC_UPDATE", ArrayFormat::FLAG_USE_DYNAMIC_UPDATE), crate::meta::inspect::EnumConstant::new("FLAG_USE_8_BONE_WEIGHTS", "ARRAY_FLAG_USE_8_BONE_WEIGHTS", ArrayFormat::FLAG_USE_8_BONE_WEIGHTS), crate::meta::inspect::EnumConstant::new("FLAG_USES_EMPTY_VERTEX_ARRAY", "ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY", ArrayFormat::FLAG_USES_EMPTY_VERTEX_ARRAY), crate::meta::inspect::EnumConstant::new("FLAG_COMPRESS_ATTRIBUTES", "ARRAY_FLAG_COMPRESS_ATTRIBUTES", ArrayFormat::FLAG_COMPRESS_ATTRIBUTES)]
        }
    }
}
impl std::ops::BitOr for ArrayFormat {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for ArrayFormat {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for ArrayFormat {
    type Via = u64;
    
}
impl crate::meta::ToGodot for ArrayFormat {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ArrayFormat {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct BlendShapeMode {
    ord: i32
}
impl BlendShapeMode {
    #[doc(alias = "BLEND_SHAPE_MODE_NORMALIZED")]
    #[doc = "Godot enumerator name: `BLEND_SHAPE_MODE_NORMALIZED`"]
    pub const NORMALIZED: BlendShapeMode = BlendShapeMode {
        ord: 0i32
    };
    #[doc(alias = "BLEND_SHAPE_MODE_RELATIVE")]
    #[doc = "Godot enumerator name: `BLEND_SHAPE_MODE_RELATIVE`"]
    pub const RELATIVE: BlendShapeMode = BlendShapeMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for BlendShapeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("BlendShapeMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for BlendShapeMode {
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
            Self::NORMALIZED => "NORMALIZED", Self::RELATIVE => "RELATIVE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[BlendShapeMode::NORMALIZED, BlendShapeMode::RELATIVE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < BlendShapeMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NORMALIZED", "BLEND_SHAPE_MODE_NORMALIZED", BlendShapeMode::NORMALIZED), crate::meta::inspect::EnumConstant::new("RELATIVE", "BLEND_SHAPE_MODE_RELATIVE", BlendShapeMode::RELATIVE)]
        }
    }
}
impl crate::meta::GodotConvert for BlendShapeMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for BlendShapeMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for BlendShapeMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Mesh;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for Mesh {
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