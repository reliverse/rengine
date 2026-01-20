#![doc = "Sidecar module for class [`PacketPeer`][crate::classes::PacketPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PacketPeer` enums](https://docs.godotengine.org/en/stable/classes/class_packetpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PacketPeer.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`packet_peer`][crate::classes::packet_peer]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `PacketPeer`](https://docs.godotengine.org/en/stable/classes/class_packetpeer.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<PacketPeer>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PacketPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl PacketPeer {
        pub(crate) fn get_var_full(&mut self, allow_objects: bool,) -> Variant {
            type CallRet = Variant;
            type CallParams = (bool,);
            let args = (allow_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6210usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeer", "get_var", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_var_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_var(&mut self,) -> Variant {
            self.get_var_ex() . done()
        }
        #[inline]
        pub fn get_var_ex < 'a > (&'a mut self,) -> ExGetVar < 'a > {
            ExGetVar::new(self,)
        }
        pub(crate) fn put_var_full(&mut self, var: RefArg < Variant >, full_objects: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, Variant >, bool,);
            let args = (var, full_objects,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6211usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeer", "put_var", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::put_var_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn put_var(&mut self, var: &Variant,) -> crate::global::Error {
            self.put_var_ex(var,) . done()
        }
        #[inline]
        pub fn put_var_ex < 'a > (&'a mut self, var: &'a Variant,) -> ExPutVar < 'a > {
            ExPutVar::new(self, var,)
        }
        pub fn get_packet(&mut self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeer", "get_packet", Some(self.__validated_obj()), args,)
            }
        }
        pub fn put_packet(&mut self, buffer: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(buffer),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeer", "put_packet", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_packet_error(&self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6214usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeer", "get_packet_error", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_available_packet_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6215usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeer", "get_available_packet_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_encode_buffer_max_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6216usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeer", "get_encode_buffer_max_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_encode_buffer_max_size(&mut self, max_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeer", "set_encode_buffer_max_size", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PacketPeer {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PacketPeer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PacketPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for PacketPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PacketPeer {
        
    }
    impl std::ops::Deref for PacketPeer {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PacketPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PacketPeer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `PacketPeer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`PacketPeer::get_var_ex`][super::PacketPeer::get_var_ex]."]
#[must_use]
pub struct ExGetVar < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PacketPeer, allow_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetVar < 'a > {
    fn new(surround_object: &'a mut re_export::PacketPeer,) -> Self {
        let allow_objects = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, allow_objects: allow_objects,
        }
    }
    #[inline]
    pub fn allow_objects(self, allow_objects: bool) -> Self {
        Self {
            allow_objects: allow_objects, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Variant {
        let Self {
            _phantom, surround_object, allow_objects,
        }
        = self;
        re_export::PacketPeer::get_var_full(surround_object, allow_objects,)
    }
}
#[doc = "Default-param extender for [`PacketPeer::put_var_ex`][super::PacketPeer::put_var_ex]."]
#[must_use]
pub struct ExPutVar < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::PacketPeer, var: CowArg < 'a, Variant >, full_objects: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPutVar < 'a > {
    fn new(surround_object: &'a mut re_export::PacketPeer, var: &'a Variant,) -> Self {
        let full_objects = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, var: CowArg::Borrowed(var), full_objects: full_objects,
        }
    }
    #[inline]
    pub fn full_objects(self, full_objects: bool) -> Self {
        Self {
            full_objects: full_objects, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, var, full_objects,
        }
        = self;
        re_export::PacketPeer::put_var_full(surround_object, var.cow_as_arg(), full_objects,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PacketPeer;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for PacketPeer {
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