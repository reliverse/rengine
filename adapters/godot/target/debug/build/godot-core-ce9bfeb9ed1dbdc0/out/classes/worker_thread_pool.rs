#![doc = "Sidecar module for class [`WorkerThreadPool`][crate::classes::WorkerThreadPool].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WorkerThreadPool` enums](https://docs.godotengine.org/en/stable/classes/class_workerthreadpool.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WorkerThreadPool.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`worker_thread_pool`][crate::classes::worker_thread_pool]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `WorkerThreadPool`](https://docs.godotengine.org/en/stable/classes/class_workerthreadpool.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WorkerThreadPool {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl WorkerThreadPool {
        pub(crate) fn add_task_full(&mut self, action: RefArg < Callable >, high_priority: bool, description: CowArg < GString >,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Callable >, bool, CowArg < 'a1, GString >,);
            let args = (action, high_priority, description,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11113usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "add_task", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_task_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_task(&mut self, action: &Callable,) -> i64 {
            self.add_task_ex(action,) . done()
        }
        #[inline]
        pub fn add_task_ex < 'a > (&'a mut self, action: &'a Callable,) -> ExAddTask < 'a > {
            ExAddTask::new(self, action,)
        }
        pub fn is_task_completed(&self, task_id: i64,) -> bool {
            type CallRet = bool;
            type CallParams = (i64,);
            let args = (task_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11114usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "is_task_completed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn wait_for_task_completion(&mut self, task_id: i64,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (i64,);
            let args = (task_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11115usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "wait_for_task_completion", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_caller_task_id(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11116usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "get_caller_task_id", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_group_task_full(&mut self, action: RefArg < Callable >, elements: i32, tasks_needed: i32, high_priority: bool, description: CowArg < GString >,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Callable >, i32, i32, bool, CowArg < 'a1, GString >,);
            let args = (action, elements, tasks_needed, high_priority, description,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11117usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "add_group_task", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_group_task_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_group_task(&mut self, action: &Callable, elements: i32,) -> i64 {
            self.add_group_task_ex(action, elements,) . done()
        }
        #[inline]
        pub fn add_group_task_ex < 'a > (&'a mut self, action: &'a Callable, elements: i32,) -> ExAddGroupTask < 'a > {
            ExAddGroupTask::new(self, action, elements,)
        }
        pub fn is_group_task_completed(&self, group_id: i64,) -> bool {
            type CallRet = bool;
            type CallParams = (i64,);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11118usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "is_group_task_completed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_group_processed_element_count(&self, group_id: i64,) -> u32 {
            type CallRet = u32;
            type CallParams = (i64,);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11119usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "get_group_processed_element_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn wait_for_group_task_completion(&mut self, group_id: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (group_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11120usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "wait_for_group_task_completion", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_caller_group_id(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11121usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WorkerThreadPool", "get_caller_group_id", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for WorkerThreadPool {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("WorkerThreadPool"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WorkerThreadPool {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for WorkerThreadPool {
        
    }
    impl crate::obj::Singleton for WorkerThreadPool {
        fn singleton() -> crate::obj::Gd < Self > {
            unsafe {
                crate::classes::singleton_unchecked(&StringName::__cstr(c"WorkerThreadPool"))
            }
        }
    }
    impl std::ops::Deref for WorkerThreadPool {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WorkerThreadPool {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_WorkerThreadPool__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `WorkerThreadPool` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`WorkerThreadPool::add_task_ex`][super::WorkerThreadPool::add_task_ex]."]
#[must_use]
pub struct ExAddTask < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::WorkerThreadPool, action: CowArg < 'a, Callable >, high_priority: bool, description: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddTask < 'a > {
    fn new(surround_object: &'a mut re_export::WorkerThreadPool, action: &'a Callable,) -> Self {
        let high_priority = false;
        let description = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: CowArg::Borrowed(action), high_priority: high_priority, description: CowArg::Owned(description),
        }
    }
    #[inline]
    pub fn high_priority(self, high_priority: bool) -> Self {
        Self {
            high_priority: high_priority, .. self
        }
    }
    #[inline]
    pub fn description(self, description: impl AsArg < GString > + 'a) -> Self {
        Self {
            description: description.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, action, high_priority, description,
        }
        = self;
        re_export::WorkerThreadPool::add_task_full(surround_object, action.cow_as_arg(), high_priority, description,)
    }
}
#[doc = "Default-param extender for [`WorkerThreadPool::add_group_task_ex`][super::WorkerThreadPool::add_group_task_ex]."]
#[must_use]
pub struct ExAddGroupTask < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::WorkerThreadPool, action: CowArg < 'a, Callable >, elements: i32, tasks_needed: i32, high_priority: bool, description: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddGroupTask < 'a > {
    fn new(surround_object: &'a mut re_export::WorkerThreadPool, action: &'a Callable, elements: i32,) -> Self {
        let tasks_needed = - 1i32;
        let high_priority = false;
        let description = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, action: CowArg::Borrowed(action), elements: elements, tasks_needed: tasks_needed, high_priority: high_priority, description: CowArg::Owned(description),
        }
    }
    #[inline]
    pub fn tasks_needed(self, tasks_needed: i32) -> Self {
        Self {
            tasks_needed: tasks_needed, .. self
        }
    }
    #[inline]
    pub fn high_priority(self, high_priority: bool) -> Self {
        Self {
            high_priority: high_priority, .. self
        }
    }
    #[inline]
    pub fn description(self, description: impl AsArg < GString > + 'a) -> Self {
        Self {
            description: description.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, action, elements, tasks_needed, high_priority, description,
        }
        = self;
        re_export::WorkerThreadPool::add_group_task_full(surround_object, action.cow_as_arg(), elements, tasks_needed, high_priority, description,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::WorkerThreadPool;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for WorkerThreadPool {
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