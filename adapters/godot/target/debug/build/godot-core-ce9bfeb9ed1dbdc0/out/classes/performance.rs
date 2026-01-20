#![doc = "Sidecar module for class [`Performance`][crate::classes::Performance].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Performance` enums](https://docs.godotengine.org/en/stable/classes/class_performance.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Performance.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`performance`][crate::classes::performance]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `Performance`](https://docs.godotengine.org/en/stable/classes/class_performance.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Performance {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Performance {
        pub fn get_monitor(&self, monitor: crate::classes::performance::Monitor,) -> f64 {
            type CallRet = f64;
            type CallParams = (crate::classes::performance::Monitor,);
            let args = (monitor,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6400usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Performance", "get_monitor", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_custom_monitor_full(&mut self, id: CowArg < StringName >, callable: RefArg < Callable >, arguments: RefArg < VarArray >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, RefArg < 'a1, Callable >, RefArg < 'a2, VarArray >,);
            let args = (id, callable, arguments,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6401usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Performance", "add_custom_monitor", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_custom_monitor_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_custom_monitor(&mut self, id: impl AsArg < StringName >, callable: &Callable,) {
            self.add_custom_monitor_ex(id, callable,) . done()
        }
        #[inline]
        pub fn add_custom_monitor_ex < 'a > (&'a mut self, id: impl AsArg < StringName > + 'a, callable: &'a Callable,) -> ExAddCustomMonitor < 'a > {
            ExAddCustomMonitor::new(self, id, callable,)
        }
        pub fn remove_custom_monitor(&mut self, id: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (id.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6402usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Performance", "remove_custom_monitor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_custom_monitor(&mut self, id: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (id.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6403usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Performance", "has_custom_monitor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_monitor(&mut self, id: impl AsArg < StringName >,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (id.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Performance", "get_custom_monitor", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_monitor_modification_time(&mut self,) -> u64 {
            type CallRet = u64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Performance", "get_monitor_modification_time", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_custom_monitor_names(&mut self,) -> Array < StringName > {
            type CallRet = Array < StringName >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Performance", "get_custom_monitor_names", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Performance {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Performance"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Performance {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Performance {
        
    }
    impl crate::obj::Singleton for Performance {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"Performance"))
            }
        }
    }
    impl std::ops::Deref for Performance {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Performance {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Performance__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Performance` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Performance::add_custom_monitor_ex`][super::Performance::add_custom_monitor_ex]."]
#[must_use]
pub struct ExAddCustomMonitor < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Performance, id: CowArg < 'a, StringName >, callable: CowArg < 'a, Callable >, arguments: CowArg < 'a, VarArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCustomMonitor < 'a > {
    fn new(surround_object: &'a mut re_export::Performance, id: impl AsArg < StringName > + 'a, callable: &'a Callable,) -> Self {
        let arguments = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, id: id.into_arg(), callable: CowArg::Borrowed(callable), arguments: CowArg::Owned(arguments),
        }
    }
    #[inline]
    pub fn arguments(self, arguments: &'a VarArray) -> Self {
        Self {
            arguments: CowArg::Borrowed(arguments), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, id, callable, arguments,
        }
        = self;
        re_export::Performance::add_custom_monitor_full(surround_object, id, callable.cow_as_arg(), arguments.cow_as_arg(),)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Monitor {
    ord: i32
}
impl Monitor {
    pub const TIME_FPS: Monitor = Monitor {
        ord: 0i32
    };
    pub const TIME_PROCESS: Monitor = Monitor {
        ord: 1i32
    };
    pub const TIME_PHYSICS_PROCESS: Monitor = Monitor {
        ord: 2i32
    };
    pub const TIME_NAVIGATION_PROCESS: Monitor = Monitor {
        ord: 3i32
    };
    pub const MEMORY_STATIC: Monitor = Monitor {
        ord: 4i32
    };
    pub const MEMORY_STATIC_MAX: Monitor = Monitor {
        ord: 5i32
    };
    pub const MEMORY_MESSAGE_BUFFER_MAX: Monitor = Monitor {
        ord: 6i32
    };
    pub const OBJECT_COUNT: Monitor = Monitor {
        ord: 7i32
    };
    pub const OBJECT_RESOURCE_COUNT: Monitor = Monitor {
        ord: 8i32
    };
    pub const OBJECT_NODE_COUNT: Monitor = Monitor {
        ord: 9i32
    };
    pub const OBJECT_ORPHAN_NODE_COUNT: Monitor = Monitor {
        ord: 10i32
    };
    pub const RENDER_TOTAL_OBJECTS_IN_FRAME: Monitor = Monitor {
        ord: 11i32
    };
    pub const RENDER_TOTAL_PRIMITIVES_IN_FRAME: Monitor = Monitor {
        ord: 12i32
    };
    pub const RENDER_TOTAL_DRAW_CALLS_IN_FRAME: Monitor = Monitor {
        ord: 13i32
    };
    pub const RENDER_VIDEO_MEM_USED: Monitor = Monitor {
        ord: 14i32
    };
    pub const RENDER_TEXTURE_MEM_USED: Monitor = Monitor {
        ord: 15i32
    };
    pub const RENDER_BUFFER_MEM_USED: Monitor = Monitor {
        ord: 16i32
    };
    pub const PHYSICS_2D_ACTIVE_OBJECTS: Monitor = Monitor {
        ord: 17i32
    };
    pub const PHYSICS_2D_COLLISION_PAIRS: Monitor = Monitor {
        ord: 18i32
    };
    pub const PHYSICS_2D_ISLAND_COUNT: Monitor = Monitor {
        ord: 19i32
    };
    pub const PHYSICS_3D_ACTIVE_OBJECTS: Monitor = Monitor {
        ord: 20i32
    };
    pub const PHYSICS_3D_COLLISION_PAIRS: Monitor = Monitor {
        ord: 21i32
    };
    pub const PHYSICS_3D_ISLAND_COUNT: Monitor = Monitor {
        ord: 22i32
    };
    pub const AUDIO_OUTPUT_LATENCY: Monitor = Monitor {
        ord: 23i32
    };
    pub const NAVIGATION_ACTIVE_MAPS: Monitor = Monitor {
        ord: 24i32
    };
    pub const NAVIGATION_REGION_COUNT: Monitor = Monitor {
        ord: 25i32
    };
    pub const NAVIGATION_AGENT_COUNT: Monitor = Monitor {
        ord: 26i32
    };
    pub const NAVIGATION_LINK_COUNT: Monitor = Monitor {
        ord: 27i32
    };
    pub const NAVIGATION_POLYGON_COUNT: Monitor = Monitor {
        ord: 28i32
    };
    pub const NAVIGATION_EDGE_COUNT: Monitor = Monitor {
        ord: 29i32
    };
    pub const NAVIGATION_EDGE_MERGE_COUNT: Monitor = Monitor {
        ord: 30i32
    };
    pub const NAVIGATION_EDGE_CONNECTION_COUNT: Monitor = Monitor {
        ord: 31i32
    };
    pub const NAVIGATION_EDGE_FREE_COUNT: Monitor = Monitor {
        ord: 32i32
    };
    pub const NAVIGATION_OBSTACLE_COUNT: Monitor = Monitor {
        ord: 33i32
    };
    pub const PIPELINE_COMPILATIONS_CANVAS: Monitor = Monitor {
        ord: 34i32
    };
    pub const PIPELINE_COMPILATIONS_MESH: Monitor = Monitor {
        ord: 35i32
    };
    pub const PIPELINE_COMPILATIONS_SURFACE: Monitor = Monitor {
        ord: 36i32
    };
    pub const PIPELINE_COMPILATIONS_DRAW: Monitor = Monitor {
        ord: 37i32
    };
    pub const PIPELINE_COMPILATIONS_SPECIALIZATION: Monitor = Monitor {
        ord: 38i32
    };
    pub const NAVIGATION_2D_ACTIVE_MAPS: Monitor = Monitor {
        ord: 39i32
    };
    pub const NAVIGATION_2D_REGION_COUNT: Monitor = Monitor {
        ord: 40i32
    };
    pub const NAVIGATION_2D_AGENT_COUNT: Monitor = Monitor {
        ord: 41i32
    };
    pub const NAVIGATION_2D_LINK_COUNT: Monitor = Monitor {
        ord: 42i32
    };
    pub const NAVIGATION_2D_POLYGON_COUNT: Monitor = Monitor {
        ord: 43i32
    };
    pub const NAVIGATION_2D_EDGE_COUNT: Monitor = Monitor {
        ord: 44i32
    };
    pub const NAVIGATION_2D_EDGE_MERGE_COUNT: Monitor = Monitor {
        ord: 45i32
    };
    pub const NAVIGATION_2D_EDGE_CONNECTION_COUNT: Monitor = Monitor {
        ord: 46i32
    };
    pub const NAVIGATION_2D_EDGE_FREE_COUNT: Monitor = Monitor {
        ord: 47i32
    };
    pub const NAVIGATION_2D_OBSTACLE_COUNT: Monitor = Monitor {
        ord: 48i32
    };
    pub const NAVIGATION_3D_ACTIVE_MAPS: Monitor = Monitor {
        ord: 49i32
    };
    pub const NAVIGATION_3D_REGION_COUNT: Monitor = Monitor {
        ord: 50i32
    };
    pub const NAVIGATION_3D_AGENT_COUNT: Monitor = Monitor {
        ord: 51i32
    };
    pub const NAVIGATION_3D_LINK_COUNT: Monitor = Monitor {
        ord: 52i32
    };
    pub const NAVIGATION_3D_POLYGON_COUNT: Monitor = Monitor {
        ord: 53i32
    };
    pub const NAVIGATION_3D_EDGE_COUNT: Monitor = Monitor {
        ord: 54i32
    };
    pub const NAVIGATION_3D_EDGE_MERGE_COUNT: Monitor = Monitor {
        ord: 55i32
    };
    pub const NAVIGATION_3D_EDGE_CONNECTION_COUNT: Monitor = Monitor {
        ord: 56i32
    };
    pub const NAVIGATION_3D_EDGE_FREE_COUNT: Monitor = Monitor {
        ord: 57i32
    };
    pub const NAVIGATION_3D_OBSTACLE_COUNT: Monitor = Monitor {
        ord: 58i32
    };
    #[doc(alias = "MONITOR_MAX")]
    #[doc = "Godot enumerator name: `MONITOR_MAX`"]
    pub const MAX: Monitor = Monitor {
        ord: 59i32
    };
    
}
impl std::fmt::Debug for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Monitor") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Monitor {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 | ord @ 14i32 | ord @ 15i32 | ord @ 16i32 | ord @ 17i32 | ord @ 18i32 | ord @ 19i32 | ord @ 20i32 | ord @ 21i32 | ord @ 22i32 | ord @ 23i32 | ord @ 24i32 | ord @ 25i32 | ord @ 26i32 | ord @ 27i32 | ord @ 28i32 | ord @ 29i32 | ord @ 30i32 | ord @ 31i32 | ord @ 32i32 | ord @ 33i32 | ord @ 34i32 | ord @ 35i32 | ord @ 36i32 | ord @ 37i32 | ord @ 38i32 | ord @ 39i32 | ord @ 40i32 | ord @ 41i32 | ord @ 42i32 | ord @ 43i32 | ord @ 44i32 | ord @ 45i32 | ord @ 46i32 | ord @ 47i32 | ord @ 48i32 | ord @ 49i32 | ord @ 50i32 | ord @ 51i32 | ord @ 52i32 | ord @ 53i32 | ord @ 54i32 | ord @ 55i32 | ord @ 56i32 | ord @ 57i32 | ord @ 58i32 | ord @ 59i32 => Some(Self {
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
            Self::TIME_FPS => "TIME_FPS", Self::TIME_PROCESS => "TIME_PROCESS", Self::TIME_PHYSICS_PROCESS => "TIME_PHYSICS_PROCESS", Self::TIME_NAVIGATION_PROCESS => "TIME_NAVIGATION_PROCESS", Self::MEMORY_STATIC => "MEMORY_STATIC", Self::MEMORY_STATIC_MAX => "MEMORY_STATIC_MAX", Self::MEMORY_MESSAGE_BUFFER_MAX => "MEMORY_MESSAGE_BUFFER_MAX", Self::OBJECT_COUNT => "OBJECT_COUNT", Self::OBJECT_RESOURCE_COUNT => "OBJECT_RESOURCE_COUNT", Self::OBJECT_NODE_COUNT => "OBJECT_NODE_COUNT", Self::OBJECT_ORPHAN_NODE_COUNT => "OBJECT_ORPHAN_NODE_COUNT", Self::RENDER_TOTAL_OBJECTS_IN_FRAME => "RENDER_TOTAL_OBJECTS_IN_FRAME", Self::RENDER_TOTAL_PRIMITIVES_IN_FRAME => "RENDER_TOTAL_PRIMITIVES_IN_FRAME", Self::RENDER_TOTAL_DRAW_CALLS_IN_FRAME => "RENDER_TOTAL_DRAW_CALLS_IN_FRAME", Self::RENDER_VIDEO_MEM_USED => "RENDER_VIDEO_MEM_USED", Self::RENDER_TEXTURE_MEM_USED => "RENDER_TEXTURE_MEM_USED", Self::RENDER_BUFFER_MEM_USED => "RENDER_BUFFER_MEM_USED", Self::PHYSICS_2D_ACTIVE_OBJECTS => "PHYSICS_2D_ACTIVE_OBJECTS", Self::PHYSICS_2D_COLLISION_PAIRS => "PHYSICS_2D_COLLISION_PAIRS", Self::PHYSICS_2D_ISLAND_COUNT => "PHYSICS_2D_ISLAND_COUNT", Self::PHYSICS_3D_ACTIVE_OBJECTS => "PHYSICS_3D_ACTIVE_OBJECTS", Self::PHYSICS_3D_COLLISION_PAIRS => "PHYSICS_3D_COLLISION_PAIRS", Self::PHYSICS_3D_ISLAND_COUNT => "PHYSICS_3D_ISLAND_COUNT", Self::AUDIO_OUTPUT_LATENCY => "AUDIO_OUTPUT_LATENCY", Self::NAVIGATION_ACTIVE_MAPS => "NAVIGATION_ACTIVE_MAPS", Self::NAVIGATION_REGION_COUNT => "NAVIGATION_REGION_COUNT", Self::NAVIGATION_AGENT_COUNT => "NAVIGATION_AGENT_COUNT", Self::NAVIGATION_LINK_COUNT => "NAVIGATION_LINK_COUNT", Self::NAVIGATION_POLYGON_COUNT => "NAVIGATION_POLYGON_COUNT", Self::NAVIGATION_EDGE_COUNT => "NAVIGATION_EDGE_COUNT", Self::NAVIGATION_EDGE_MERGE_COUNT => "NAVIGATION_EDGE_MERGE_COUNT", Self::NAVIGATION_EDGE_CONNECTION_COUNT => "NAVIGATION_EDGE_CONNECTION_COUNT", Self::NAVIGATION_EDGE_FREE_COUNT => "NAVIGATION_EDGE_FREE_COUNT", Self::NAVIGATION_OBSTACLE_COUNT => "NAVIGATION_OBSTACLE_COUNT", Self::PIPELINE_COMPILATIONS_CANVAS => "PIPELINE_COMPILATIONS_CANVAS", Self::PIPELINE_COMPILATIONS_MESH => "PIPELINE_COMPILATIONS_MESH", Self::PIPELINE_COMPILATIONS_SURFACE => "PIPELINE_COMPILATIONS_SURFACE", Self::PIPELINE_COMPILATIONS_DRAW => "PIPELINE_COMPILATIONS_DRAW", Self::PIPELINE_COMPILATIONS_SPECIALIZATION => "PIPELINE_COMPILATIONS_SPECIALIZATION", Self::NAVIGATION_2D_ACTIVE_MAPS => "NAVIGATION_2D_ACTIVE_MAPS", Self::NAVIGATION_2D_REGION_COUNT => "NAVIGATION_2D_REGION_COUNT", Self::NAVIGATION_2D_AGENT_COUNT => "NAVIGATION_2D_AGENT_COUNT", Self::NAVIGATION_2D_LINK_COUNT => "NAVIGATION_2D_LINK_COUNT", Self::NAVIGATION_2D_POLYGON_COUNT => "NAVIGATION_2D_POLYGON_COUNT", Self::NAVIGATION_2D_EDGE_COUNT => "NAVIGATION_2D_EDGE_COUNT", Self::NAVIGATION_2D_EDGE_MERGE_COUNT => "NAVIGATION_2D_EDGE_MERGE_COUNT", Self::NAVIGATION_2D_EDGE_CONNECTION_COUNT => "NAVIGATION_2D_EDGE_CONNECTION_COUNT", Self::NAVIGATION_2D_EDGE_FREE_COUNT => "NAVIGATION_2D_EDGE_FREE_COUNT", Self::NAVIGATION_2D_OBSTACLE_COUNT => "NAVIGATION_2D_OBSTACLE_COUNT", Self::NAVIGATION_3D_ACTIVE_MAPS => "NAVIGATION_3D_ACTIVE_MAPS", Self::NAVIGATION_3D_REGION_COUNT => "NAVIGATION_3D_REGION_COUNT", Self::NAVIGATION_3D_AGENT_COUNT => "NAVIGATION_3D_AGENT_COUNT", Self::NAVIGATION_3D_LINK_COUNT => "NAVIGATION_3D_LINK_COUNT", Self::NAVIGATION_3D_POLYGON_COUNT => "NAVIGATION_3D_POLYGON_COUNT", Self::NAVIGATION_3D_EDGE_COUNT => "NAVIGATION_3D_EDGE_COUNT", Self::NAVIGATION_3D_EDGE_MERGE_COUNT => "NAVIGATION_3D_EDGE_MERGE_COUNT", Self::NAVIGATION_3D_EDGE_CONNECTION_COUNT => "NAVIGATION_3D_EDGE_CONNECTION_COUNT", Self::NAVIGATION_3D_EDGE_FREE_COUNT => "NAVIGATION_3D_EDGE_FREE_COUNT", Self::NAVIGATION_3D_OBSTACLE_COUNT => "NAVIGATION_3D_OBSTACLE_COUNT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Monitor::TIME_FPS, Monitor::TIME_PROCESS, Monitor::TIME_PHYSICS_PROCESS, Monitor::TIME_NAVIGATION_PROCESS, Monitor::MEMORY_STATIC, Monitor::MEMORY_STATIC_MAX, Monitor::MEMORY_MESSAGE_BUFFER_MAX, Monitor::OBJECT_COUNT, Monitor::OBJECT_RESOURCE_COUNT, Monitor::OBJECT_NODE_COUNT, Monitor::OBJECT_ORPHAN_NODE_COUNT, Monitor::RENDER_TOTAL_OBJECTS_IN_FRAME, Monitor::RENDER_TOTAL_PRIMITIVES_IN_FRAME, Monitor::RENDER_TOTAL_DRAW_CALLS_IN_FRAME, Monitor::RENDER_VIDEO_MEM_USED, Monitor::RENDER_TEXTURE_MEM_USED, Monitor::RENDER_BUFFER_MEM_USED, Monitor::PHYSICS_2D_ACTIVE_OBJECTS, Monitor::PHYSICS_2D_COLLISION_PAIRS, Monitor::PHYSICS_2D_ISLAND_COUNT, Monitor::PHYSICS_3D_ACTIVE_OBJECTS, Monitor::PHYSICS_3D_COLLISION_PAIRS, Monitor::PHYSICS_3D_ISLAND_COUNT, Monitor::AUDIO_OUTPUT_LATENCY, Monitor::NAVIGATION_ACTIVE_MAPS, Monitor::NAVIGATION_REGION_COUNT, Monitor::NAVIGATION_AGENT_COUNT, Monitor::NAVIGATION_LINK_COUNT, Monitor::NAVIGATION_POLYGON_COUNT, Monitor::NAVIGATION_EDGE_COUNT, Monitor::NAVIGATION_EDGE_MERGE_COUNT, Monitor::NAVIGATION_EDGE_CONNECTION_COUNT, Monitor::NAVIGATION_EDGE_FREE_COUNT, Monitor::NAVIGATION_OBSTACLE_COUNT, Monitor::PIPELINE_COMPILATIONS_CANVAS, Monitor::PIPELINE_COMPILATIONS_MESH, Monitor::PIPELINE_COMPILATIONS_SURFACE, Monitor::PIPELINE_COMPILATIONS_DRAW, Monitor::PIPELINE_COMPILATIONS_SPECIALIZATION, Monitor::NAVIGATION_2D_ACTIVE_MAPS, Monitor::NAVIGATION_2D_REGION_COUNT, Monitor::NAVIGATION_2D_AGENT_COUNT, Monitor::NAVIGATION_2D_LINK_COUNT, Monitor::NAVIGATION_2D_POLYGON_COUNT, Monitor::NAVIGATION_2D_EDGE_COUNT, Monitor::NAVIGATION_2D_EDGE_MERGE_COUNT, Monitor::NAVIGATION_2D_EDGE_CONNECTION_COUNT, Monitor::NAVIGATION_2D_EDGE_FREE_COUNT, Monitor::NAVIGATION_2D_OBSTACLE_COUNT, Monitor::NAVIGATION_3D_ACTIVE_MAPS, Monitor::NAVIGATION_3D_REGION_COUNT, Monitor::NAVIGATION_3D_AGENT_COUNT, Monitor::NAVIGATION_3D_LINK_COUNT, Monitor::NAVIGATION_3D_POLYGON_COUNT, Monitor::NAVIGATION_3D_EDGE_COUNT, Monitor::NAVIGATION_3D_EDGE_MERGE_COUNT, Monitor::NAVIGATION_3D_EDGE_CONNECTION_COUNT, Monitor::NAVIGATION_3D_EDGE_FREE_COUNT, Monitor::NAVIGATION_3D_OBSTACLE_COUNT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Monitor >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TIME_FPS", "TIME_FPS", Monitor::TIME_FPS), crate::meta::inspect::EnumConstant::new("TIME_PROCESS", "TIME_PROCESS", Monitor::TIME_PROCESS), crate::meta::inspect::EnumConstant::new("TIME_PHYSICS_PROCESS", "TIME_PHYSICS_PROCESS", Monitor::TIME_PHYSICS_PROCESS), crate::meta::inspect::EnumConstant::new("TIME_NAVIGATION_PROCESS", "TIME_NAVIGATION_PROCESS", Monitor::TIME_NAVIGATION_PROCESS), crate::meta::inspect::EnumConstant::new("MEMORY_STATIC", "MEMORY_STATIC", Monitor::MEMORY_STATIC), crate::meta::inspect::EnumConstant::new("MEMORY_STATIC_MAX", "MEMORY_STATIC_MAX", Monitor::MEMORY_STATIC_MAX), crate::meta::inspect::EnumConstant::new("MEMORY_MESSAGE_BUFFER_MAX", "MEMORY_MESSAGE_BUFFER_MAX", Monitor::MEMORY_MESSAGE_BUFFER_MAX), crate::meta::inspect::EnumConstant::new("OBJECT_COUNT", "OBJECT_COUNT", Monitor::OBJECT_COUNT), crate::meta::inspect::EnumConstant::new("OBJECT_RESOURCE_COUNT", "OBJECT_RESOURCE_COUNT", Monitor::OBJECT_RESOURCE_COUNT), crate::meta::inspect::EnumConstant::new("OBJECT_NODE_COUNT", "OBJECT_NODE_COUNT", Monitor::OBJECT_NODE_COUNT), crate::meta::inspect::EnumConstant::new("OBJECT_ORPHAN_NODE_COUNT", "OBJECT_ORPHAN_NODE_COUNT", Monitor::OBJECT_ORPHAN_NODE_COUNT), crate::meta::inspect::EnumConstant::new("RENDER_TOTAL_OBJECTS_IN_FRAME", "RENDER_TOTAL_OBJECTS_IN_FRAME", Monitor::RENDER_TOTAL_OBJECTS_IN_FRAME), crate::meta::inspect::EnumConstant::new("RENDER_TOTAL_PRIMITIVES_IN_FRAME", "RENDER_TOTAL_PRIMITIVES_IN_FRAME", Monitor::RENDER_TOTAL_PRIMITIVES_IN_FRAME), crate::meta::inspect::EnumConstant::new("RENDER_TOTAL_DRAW_CALLS_IN_FRAME", "RENDER_TOTAL_DRAW_CALLS_IN_FRAME", Monitor::RENDER_TOTAL_DRAW_CALLS_IN_FRAME), crate::meta::inspect::EnumConstant::new("RENDER_VIDEO_MEM_USED", "RENDER_VIDEO_MEM_USED", Monitor::RENDER_VIDEO_MEM_USED), crate::meta::inspect::EnumConstant::new("RENDER_TEXTURE_MEM_USED", "RENDER_TEXTURE_MEM_USED", Monitor::RENDER_TEXTURE_MEM_USED), crate::meta::inspect::EnumConstant::new("RENDER_BUFFER_MEM_USED", "RENDER_BUFFER_MEM_USED", Monitor::RENDER_BUFFER_MEM_USED), crate::meta::inspect::EnumConstant::new("PHYSICS_2D_ACTIVE_OBJECTS", "PHYSICS_2D_ACTIVE_OBJECTS", Monitor::PHYSICS_2D_ACTIVE_OBJECTS), crate::meta::inspect::EnumConstant::new("PHYSICS_2D_COLLISION_PAIRS", "PHYSICS_2D_COLLISION_PAIRS", Monitor::PHYSICS_2D_COLLISION_PAIRS), crate::meta::inspect::EnumConstant::new("PHYSICS_2D_ISLAND_COUNT", "PHYSICS_2D_ISLAND_COUNT", Monitor::PHYSICS_2D_ISLAND_COUNT), crate::meta::inspect::EnumConstant::new("PHYSICS_3D_ACTIVE_OBJECTS", "PHYSICS_3D_ACTIVE_OBJECTS", Monitor::PHYSICS_3D_ACTIVE_OBJECTS), crate::meta::inspect::EnumConstant::new("PHYSICS_3D_COLLISION_PAIRS", "PHYSICS_3D_COLLISION_PAIRS", Monitor::PHYSICS_3D_COLLISION_PAIRS), crate::meta::inspect::EnumConstant::new("PHYSICS_3D_ISLAND_COUNT", "PHYSICS_3D_ISLAND_COUNT", Monitor::PHYSICS_3D_ISLAND_COUNT), crate::meta::inspect::EnumConstant::new("AUDIO_OUTPUT_LATENCY", "AUDIO_OUTPUT_LATENCY", Monitor::AUDIO_OUTPUT_LATENCY), crate::meta::inspect::EnumConstant::new("NAVIGATION_ACTIVE_MAPS", "NAVIGATION_ACTIVE_MAPS", Monitor::NAVIGATION_ACTIVE_MAPS), crate::meta::inspect::EnumConstant::new("NAVIGATION_REGION_COUNT", "NAVIGATION_REGION_COUNT", Monitor::NAVIGATION_REGION_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_AGENT_COUNT", "NAVIGATION_AGENT_COUNT", Monitor::NAVIGATION_AGENT_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_LINK_COUNT", "NAVIGATION_LINK_COUNT", Monitor::NAVIGATION_LINK_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_POLYGON_COUNT", "NAVIGATION_POLYGON_COUNT", Monitor::NAVIGATION_POLYGON_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_EDGE_COUNT", "NAVIGATION_EDGE_COUNT", Monitor::NAVIGATION_EDGE_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_EDGE_MERGE_COUNT", "NAVIGATION_EDGE_MERGE_COUNT", Monitor::NAVIGATION_EDGE_MERGE_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_EDGE_CONNECTION_COUNT", "NAVIGATION_EDGE_CONNECTION_COUNT", Monitor::NAVIGATION_EDGE_CONNECTION_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_EDGE_FREE_COUNT", "NAVIGATION_EDGE_FREE_COUNT", Monitor::NAVIGATION_EDGE_FREE_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_OBSTACLE_COUNT", "NAVIGATION_OBSTACLE_COUNT", Monitor::NAVIGATION_OBSTACLE_COUNT), crate::meta::inspect::EnumConstant::new("PIPELINE_COMPILATIONS_CANVAS", "PIPELINE_COMPILATIONS_CANVAS", Monitor::PIPELINE_COMPILATIONS_CANVAS), crate::meta::inspect::EnumConstant::new("PIPELINE_COMPILATIONS_MESH", "PIPELINE_COMPILATIONS_MESH", Monitor::PIPELINE_COMPILATIONS_MESH), crate::meta::inspect::EnumConstant::new("PIPELINE_COMPILATIONS_SURFACE", "PIPELINE_COMPILATIONS_SURFACE", Monitor::PIPELINE_COMPILATIONS_SURFACE), crate::meta::inspect::EnumConstant::new("PIPELINE_COMPILATIONS_DRAW", "PIPELINE_COMPILATIONS_DRAW", Monitor::PIPELINE_COMPILATIONS_DRAW), crate::meta::inspect::EnumConstant::new("PIPELINE_COMPILATIONS_SPECIALIZATION", "PIPELINE_COMPILATIONS_SPECIALIZATION", Monitor::PIPELINE_COMPILATIONS_SPECIALIZATION), crate::meta::inspect::EnumConstant::new("NAVIGATION_2D_ACTIVE_MAPS", "NAVIGATION_2D_ACTIVE_MAPS", Monitor::NAVIGATION_2D_ACTIVE_MAPS), crate::meta::inspect::EnumConstant::new("NAVIGATION_2D_REGION_COUNT", "NAVIGATION_2D_REGION_COUNT", Monitor::NAVIGATION_2D_REGION_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_2D_AGENT_COUNT", "NAVIGATION_2D_AGENT_COUNT", Monitor::NAVIGATION_2D_AGENT_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_2D_LINK_COUNT", "NAVIGATION_2D_LINK_COUNT", Monitor::NAVIGATION_2D_LINK_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_2D_POLYGON_COUNT", "NAVIGATION_2D_POLYGON_COUNT", Monitor::NAVIGATION_2D_POLYGON_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_2D_EDGE_COUNT", "NAVIGATION_2D_EDGE_COUNT", Monitor::NAVIGATION_2D_EDGE_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_2D_EDGE_MERGE_COUNT", "NAVIGATION_2D_EDGE_MERGE_COUNT", Monitor::NAVIGATION_2D_EDGE_MERGE_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_2D_EDGE_CONNECTION_COUNT", "NAVIGATION_2D_EDGE_CONNECTION_COUNT", Monitor::NAVIGATION_2D_EDGE_CONNECTION_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_2D_EDGE_FREE_COUNT", "NAVIGATION_2D_EDGE_FREE_COUNT", Monitor::NAVIGATION_2D_EDGE_FREE_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_2D_OBSTACLE_COUNT", "NAVIGATION_2D_OBSTACLE_COUNT", Monitor::NAVIGATION_2D_OBSTACLE_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_3D_ACTIVE_MAPS", "NAVIGATION_3D_ACTIVE_MAPS", Monitor::NAVIGATION_3D_ACTIVE_MAPS), crate::meta::inspect::EnumConstant::new("NAVIGATION_3D_REGION_COUNT", "NAVIGATION_3D_REGION_COUNT", Monitor::NAVIGATION_3D_REGION_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_3D_AGENT_COUNT", "NAVIGATION_3D_AGENT_COUNT", Monitor::NAVIGATION_3D_AGENT_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_3D_LINK_COUNT", "NAVIGATION_3D_LINK_COUNT", Monitor::NAVIGATION_3D_LINK_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_3D_POLYGON_COUNT", "NAVIGATION_3D_POLYGON_COUNT", Monitor::NAVIGATION_3D_POLYGON_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_3D_EDGE_COUNT", "NAVIGATION_3D_EDGE_COUNT", Monitor::NAVIGATION_3D_EDGE_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_3D_EDGE_MERGE_COUNT", "NAVIGATION_3D_EDGE_MERGE_COUNT", Monitor::NAVIGATION_3D_EDGE_MERGE_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_3D_EDGE_CONNECTION_COUNT", "NAVIGATION_3D_EDGE_CONNECTION_COUNT", Monitor::NAVIGATION_3D_EDGE_CONNECTION_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_3D_EDGE_FREE_COUNT", "NAVIGATION_3D_EDGE_FREE_COUNT", Monitor::NAVIGATION_3D_EDGE_FREE_COUNT), crate::meta::inspect::EnumConstant::new("NAVIGATION_3D_OBSTACLE_COUNT", "NAVIGATION_3D_OBSTACLE_COUNT", Monitor::NAVIGATION_3D_OBSTACLE_COUNT), crate::meta::inspect::EnumConstant::new("MAX", "MONITOR_MAX", Monitor::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for Monitor {
    const ENUMERATOR_COUNT: usize = 59usize;
    
}
impl crate::meta::GodotConvert for Monitor {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Monitor {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Monitor {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Performance;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Performance {
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