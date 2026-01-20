#![doc = "Sidecar module for class [`VoxelGi`][crate::classes::VoxelGi].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VoxelGI` enums](https://docs.godotengine.org/en/stable/classes/class_voxelgi.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VoxelGI.`\n\nInherits [`VisualInstance3D`][crate::classes::VisualInstance3D].\n\nRelated symbols:\n\n* [`voxel_gi`][crate::classes::voxel_gi]: sidecar module with related enum/flag types\n* [`IVoxelGi`][crate::classes::IVoxelGi]: virtual methods\n\n\nSee also [Godot docs for `VoxelGI`](https://docs.godotengine.org/en/stable/classes/class_voxelgi.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`VoxelGi::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VoxelGi {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`VoxelGi`][crate::classes::VoxelGi].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IVisualInstance3D`][crate::classes::IVisualInstance3D] > [`INode3D`][crate::classes::INode3D] > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `VoxelGI` methods](https://docs.godotengine.org/en/stable/classes/class_voxelgi.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVoxelGi: crate::obj::GodotClass < Base = VoxelGi > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: Node3DNotification) {
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
        fn get_aabb(&self,) -> Aabb {
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
    impl VoxelGi {
        pub fn set_probe_data(&mut self, data: impl AsArg < Option < Gd < crate::classes::VoxelGiData >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::VoxelGiData > > >,);
            let args = (data.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10858usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "set_probe_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_probe_data(&self,) -> Option < Gd < crate::classes::VoxelGiData > > {
            type CallRet = Option < Gd < crate::classes::VoxelGiData > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "get_probe_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_subdiv(&mut self, subdiv: crate::classes::voxel_gi::Subdiv,) {
            type CallRet = ();
            type CallParams = (crate::classes::voxel_gi::Subdiv,);
            let args = (subdiv,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "set_subdiv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_subdiv(&self,) -> crate::classes::voxel_gi::Subdiv {
            type CallRet = crate::classes::voxel_gi::Subdiv;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10861usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "get_subdiv", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_size(&mut self, size: Vector3,) {
            type CallRet = ();
            type CallParams = (Vector3,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10862usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "set_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_size(&self,) -> Vector3 {
            type CallRet = Vector3;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10863usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "get_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_camera_attributes(&mut self, camera_attributes: impl AsArg < Option < Gd < crate::classes::CameraAttributes >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::CameraAttributes > > >,);
            let args = (camera_attributes.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10864usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "set_camera_attributes", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_camera_attributes(&self,) -> Option < Gd < crate::classes::CameraAttributes > > {
            type CallRet = Option < Gd < crate::classes::CameraAttributes > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10865usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "get_camera_attributes", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn bake_full(&mut self, from_node: CowArg < Option < Gd < crate::classes::Node > > >, create_visual_debug: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >, bool,);
            let args = (from_node, create_visual_debug,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10866usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "bake", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::bake_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn bake(&mut self,) {
            self.bake_ex() . done()
        }
        #[inline]
        pub fn bake_ex < 'a > (&'a mut self,) -> ExBake < 'a > {
            ExBake::new(self,)
        }
        pub fn debug_bake(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10867usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VoxelGi", "debug_bake", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for VoxelGi {
        type Base = crate::classes::VisualInstance3D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VoxelGI"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VoxelGi {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::VisualInstance3D > for VoxelGi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node3D > for VoxelGi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for VoxelGi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VoxelGi {
        
    }
    impl crate::obj::cap::GodotDefault for VoxelGi {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VoxelGi {
        type Target = crate::classes::VisualInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VoxelGi {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VoxelGi`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VoxelGi__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VoxelGi > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::VisualInstance3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node3D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`VoxelGi::bake_ex`][super::VoxelGi::bake_ex]."]
#[must_use]
pub struct ExBake < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::VoxelGi, from_node: CowArg < 'a, Option < Gd < crate::classes::Node > > >, create_visual_debug: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExBake < 'a > {
    fn new(surround_object: &'a mut re_export::VoxelGi,) -> Self {
        let from_node = Gd::null_arg();
        let create_visual_debug = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, from_node: from_node.into_arg(), create_visual_debug: create_visual_debug,
        }
    }
    #[inline]
    pub fn from_node(self, from_node: impl AsArg < Option < Gd < crate::classes::Node >> > + 'a) -> Self {
        Self {
            from_node: from_node.into_arg(), .. self
        }
    }
    #[inline]
    pub fn create_visual_debug(self, create_visual_debug: bool) -> Self {
        Self {
            create_visual_debug: create_visual_debug, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, from_node, create_visual_debug,
        }
        = self;
        re_export::VoxelGi::bake_full(surround_object, from_node, create_visual_debug,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Subdiv {
    ord: i32
}
impl Subdiv {
    pub const SUBDIV_64: Subdiv = Subdiv {
        ord: 0i32
    };
    pub const SUBDIV_128: Subdiv = Subdiv {
        ord: 1i32
    };
    pub const SUBDIV_256: Subdiv = Subdiv {
        ord: 2i32
    };
    pub const SUBDIV_512: Subdiv = Subdiv {
        ord: 3i32
    };
    #[doc(alias = "SUBDIV_MAX")]
    #[doc = "Godot enumerator name: `SUBDIV_MAX`"]
    pub const MAX: Subdiv = Subdiv {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for Subdiv {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Subdiv") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Subdiv {
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
            Self::SUBDIV_64 => "SUBDIV_64", Self::SUBDIV_128 => "SUBDIV_128", Self::SUBDIV_256 => "SUBDIV_256", Self::SUBDIV_512 => "SUBDIV_512", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Subdiv::SUBDIV_64, Subdiv::SUBDIV_128, Subdiv::SUBDIV_256, Subdiv::SUBDIV_512]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Subdiv >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SUBDIV_64", "SUBDIV_64", Subdiv::SUBDIV_64), crate::meta::inspect::EnumConstant::new("SUBDIV_128", "SUBDIV_128", Subdiv::SUBDIV_128), crate::meta::inspect::EnumConstant::new("SUBDIV_256", "SUBDIV_256", Subdiv::SUBDIV_256), crate::meta::inspect::EnumConstant::new("SUBDIV_512", "SUBDIV_512", Subdiv::SUBDIV_512), crate::meta::inspect::EnumConstant::new("MAX", "SUBDIV_MAX", Subdiv::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Subdiv {
    const ENUMERATOR_COUNT: usize = 4usize;
    
}
impl crate::meta::GodotConvert for Subdiv {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Subdiv {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Subdiv {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VoxelGi;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::node_3d::SignalsOfNode3D;
    impl WithSignals for VoxelGi {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfNode3D < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}