#![doc = "Sidecar module for class [`GltfObjectModelProperty`][crate::classes::GltfObjectModelProperty].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFObjectModelProperty` enums](https://docs.godotengine.org/en/stable/classes/class_gltfobjectmodelproperty.html#enumerations).\n\n"]
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
    #[doc = "Godot class `GLTFObjectModelProperty.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`gltf_object_model_property`][crate::classes::gltf_object_model_property]: sidecar module with related enum/flag types\n* [`IGltfObjectModelProperty`][crate::classes::IGltfObjectModelProperty]: virtual methods\n\n\nSee also [Godot docs for `GLTFObjectModelProperty`](https://docs.godotengine.org/en/stable/classes/class_gltfobjectmodelproperty.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`GltfObjectModelProperty::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct GltfObjectModelProperty {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`GltfObjectModelProperty`][crate::classes::GltfObjectModelProperty].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `GLTFObjectModelProperty` methods](https://docs.godotengine.org/en/stable/classes/class_gltfobjectmodelproperty.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IGltfObjectModelProperty: crate::obj::GodotClass < Base = GltfObjectModelProperty > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl GltfObjectModelProperty {
        pub fn append_node_path(&mut self, node_path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (node_path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3655usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "append_node_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn append_path_to_property(&mut self, node_path: impl AsArg < NodePath >, prop_name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, NodePath >, CowArg < 'a1, StringName >,);
            let args = (node_path.into_arg(), prop_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3656usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "append_path_to_property", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_accessor_type(&self,) -> crate::classes::gltf_accessor::GltfAccessorType {
            type CallRet = crate::classes::gltf_accessor::GltfAccessorType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3657usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "get_accessor_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gltf_to_godot_expression(&self,) -> Option < Gd < crate::classes::Expression > > {
            type CallRet = Option < Gd < crate::classes::Expression > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3658usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "get_gltf_to_godot_expression", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gltf_to_godot_expression(&mut self, gltf_to_godot_expr: impl AsArg < Option < Gd < crate::classes::Expression >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Expression > > >,);
            let args = (gltf_to_godot_expr.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3659usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "set_gltf_to_godot_expression", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_godot_to_gltf_expression(&self,) -> Option < Gd < crate::classes::Expression > > {
            type CallRet = Option < Gd < crate::classes::Expression > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3660usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "get_godot_to_gltf_expression", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_godot_to_gltf_expression(&mut self, godot_to_gltf_expr: impl AsArg < Option < Gd < crate::classes::Expression >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Expression > > >,);
            let args = (godot_to_gltf_expr.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3661usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "set_godot_to_gltf_expression", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_paths(&self,) -> Array < NodePath > {
            type CallRet = Array < NodePath >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3662usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "get_node_paths", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_node_paths(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3663usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "has_node_paths", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_node_paths(&mut self, node_paths: &Array < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < NodePath > >,);
            let args = (RefArg::new(node_paths),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3664usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "set_node_paths", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_object_model_type(&self,) -> crate::classes::gltf_object_model_property::GltfObjectModelType {
            type CallRet = crate::classes::gltf_object_model_property::GltfObjectModelType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3665usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "get_object_model_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_object_model_type(&mut self, type_: crate::classes::gltf_object_model_property::GltfObjectModelType,) {
            type CallRet = ();
            type CallParams = (crate::classes::gltf_object_model_property::GltfObjectModelType,);
            let args = (type_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3666usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "set_object_model_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_json_pointers(&self,) -> Array < PackedStringArray > {
            type CallRet = Array < PackedStringArray >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3667usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "get_json_pointers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_json_pointers(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3668usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "has_json_pointers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_json_pointers(&mut self, json_pointers: &Array < PackedStringArray >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Array < PackedStringArray > >,);
            let args = (RefArg::new(json_pointers),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3669usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "set_json_pointers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_variant_type(&self,) -> VariantType {
            type CallRet = VariantType;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3670usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "get_variant_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_variant_type(&mut self, variant_type: VariantType,) {
            type CallRet = ();
            type CallParams = (VariantType,);
            let args = (variant_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3671usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "set_variant_type", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_types(&mut self, variant_type: VariantType, obj_model_type: crate::classes::gltf_object_model_property::GltfObjectModelType,) {
            type CallRet = ();
            type CallParams = (VariantType, crate::classes::gltf_object_model_property::GltfObjectModelType,);
            let args = (variant_type, obj_model_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(3672usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "GltfObjectModelProperty", "set_types", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for GltfObjectModelProperty {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("GLTFObjectModelProperty"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for GltfObjectModelProperty {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for GltfObjectModelProperty {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for GltfObjectModelProperty {
        
    }
    impl crate::obj::cap::GodotDefault for GltfObjectModelProperty {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for GltfObjectModelProperty {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for GltfObjectModelProperty {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`GltfObjectModelProperty`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_GltfObjectModelProperty__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::GltfObjectModelProperty > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `GLTFObjectModelType`."]
pub struct GltfObjectModelType {
    ord: i32
}
impl GltfObjectModelType {
    #[doc(alias = "GLTF_OBJECT_MODEL_TYPE_UNKNOWN")]
    #[doc = "Godot enumerator name: `GLTF_OBJECT_MODEL_TYPE_UNKNOWN`"]
    pub const UNKNOWN: GltfObjectModelType = GltfObjectModelType {
        ord: 0i32
    };
    #[doc(alias = "GLTF_OBJECT_MODEL_TYPE_BOOL")]
    #[doc = "Godot enumerator name: `GLTF_OBJECT_MODEL_TYPE_BOOL`"]
    pub const BOOL: GltfObjectModelType = GltfObjectModelType {
        ord: 1i32
    };
    #[doc(alias = "GLTF_OBJECT_MODEL_TYPE_FLOAT")]
    #[doc = "Godot enumerator name: `GLTF_OBJECT_MODEL_TYPE_FLOAT`"]
    pub const FLOAT: GltfObjectModelType = GltfObjectModelType {
        ord: 2i32
    };
    #[doc(alias = "GLTF_OBJECT_MODEL_TYPE_FLOAT_ARRAY")]
    #[doc = "Godot enumerator name: `GLTF_OBJECT_MODEL_TYPE_FLOAT_ARRAY`"]
    pub const FLOAT_ARRAY: GltfObjectModelType = GltfObjectModelType {
        ord: 3i32
    };
    #[doc(alias = "GLTF_OBJECT_MODEL_TYPE_FLOAT2")]
    #[doc = "Godot enumerator name: `GLTF_OBJECT_MODEL_TYPE_FLOAT2`"]
    pub const FLOAT2: GltfObjectModelType = GltfObjectModelType {
        ord: 4i32
    };
    #[doc(alias = "GLTF_OBJECT_MODEL_TYPE_FLOAT3")]
    #[doc = "Godot enumerator name: `GLTF_OBJECT_MODEL_TYPE_FLOAT3`"]
    pub const FLOAT3: GltfObjectModelType = GltfObjectModelType {
        ord: 5i32
    };
    #[doc(alias = "GLTF_OBJECT_MODEL_TYPE_FLOAT4")]
    #[doc = "Godot enumerator name: `GLTF_OBJECT_MODEL_TYPE_FLOAT4`"]
    pub const FLOAT4: GltfObjectModelType = GltfObjectModelType {
        ord: 6i32
    };
    #[doc(alias = "GLTF_OBJECT_MODEL_TYPE_FLOAT2X2")]
    #[doc = "Godot enumerator name: `GLTF_OBJECT_MODEL_TYPE_FLOAT2X2`"]
    pub const FLOAT2X2: GltfObjectModelType = GltfObjectModelType {
        ord: 7i32
    };
    #[doc(alias = "GLTF_OBJECT_MODEL_TYPE_FLOAT3X3")]
    #[doc = "Godot enumerator name: `GLTF_OBJECT_MODEL_TYPE_FLOAT3X3`"]
    pub const FLOAT3X3: GltfObjectModelType = GltfObjectModelType {
        ord: 8i32
    };
    #[doc(alias = "GLTF_OBJECT_MODEL_TYPE_FLOAT4X4")]
    #[doc = "Godot enumerator name: `GLTF_OBJECT_MODEL_TYPE_FLOAT4X4`"]
    pub const FLOAT4X4: GltfObjectModelType = GltfObjectModelType {
        ord: 9i32
    };
    #[doc(alias = "GLTF_OBJECT_MODEL_TYPE_INT")]
    #[doc = "Godot enumerator name: `GLTF_OBJECT_MODEL_TYPE_INT`"]
    pub const INT: GltfObjectModelType = GltfObjectModelType {
        ord: 10i32
    };
    
}
impl std::fmt::Debug for GltfObjectModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GltfObjectModelType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GltfObjectModelType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 => Some(Self {
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
            Self::UNKNOWN => "UNKNOWN", Self::BOOL => "BOOL", Self::FLOAT => "FLOAT", Self::FLOAT_ARRAY => "FLOAT_ARRAY", Self::FLOAT2 => "FLOAT2", Self::FLOAT3 => "FLOAT3", Self::FLOAT4 => "FLOAT4", Self::FLOAT2X2 => "FLOAT2X2", Self::FLOAT3X3 => "FLOAT3X3", Self::FLOAT4X4 => "FLOAT4X4", Self::INT => "INT", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[GltfObjectModelType::UNKNOWN, GltfObjectModelType::BOOL, GltfObjectModelType::FLOAT, GltfObjectModelType::FLOAT_ARRAY, GltfObjectModelType::FLOAT2, GltfObjectModelType::FLOAT3, GltfObjectModelType::FLOAT4, GltfObjectModelType::FLOAT2X2, GltfObjectModelType::FLOAT3X3, GltfObjectModelType::FLOAT4X4, GltfObjectModelType::INT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GltfObjectModelType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNKNOWN", "GLTF_OBJECT_MODEL_TYPE_UNKNOWN", GltfObjectModelType::UNKNOWN), crate::meta::inspect::EnumConstant::new("BOOL", "GLTF_OBJECT_MODEL_TYPE_BOOL", GltfObjectModelType::BOOL), crate::meta::inspect::EnumConstant::new("FLOAT", "GLTF_OBJECT_MODEL_TYPE_FLOAT", GltfObjectModelType::FLOAT), crate::meta::inspect::EnumConstant::new("FLOAT_ARRAY", "GLTF_OBJECT_MODEL_TYPE_FLOAT_ARRAY", GltfObjectModelType::FLOAT_ARRAY), crate::meta::inspect::EnumConstant::new("FLOAT2", "GLTF_OBJECT_MODEL_TYPE_FLOAT2", GltfObjectModelType::FLOAT2), crate::meta::inspect::EnumConstant::new("FLOAT3", "GLTF_OBJECT_MODEL_TYPE_FLOAT3", GltfObjectModelType::FLOAT3), crate::meta::inspect::EnumConstant::new("FLOAT4", "GLTF_OBJECT_MODEL_TYPE_FLOAT4", GltfObjectModelType::FLOAT4), crate::meta::inspect::EnumConstant::new("FLOAT2X2", "GLTF_OBJECT_MODEL_TYPE_FLOAT2X2", GltfObjectModelType::FLOAT2X2), crate::meta::inspect::EnumConstant::new("FLOAT3X3", "GLTF_OBJECT_MODEL_TYPE_FLOAT3X3", GltfObjectModelType::FLOAT3X3), crate::meta::inspect::EnumConstant::new("FLOAT4X4", "GLTF_OBJECT_MODEL_TYPE_FLOAT4X4", GltfObjectModelType::FLOAT4X4), crate::meta::inspect::EnumConstant::new("INT", "GLTF_OBJECT_MODEL_TYPE_INT", GltfObjectModelType::INT)]
        }
    }
}
impl crate::meta::GodotConvert for GltfObjectModelType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for GltfObjectModelType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GltfObjectModelType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::GltfObjectModelProperty;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for GltfObjectModelProperty {
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