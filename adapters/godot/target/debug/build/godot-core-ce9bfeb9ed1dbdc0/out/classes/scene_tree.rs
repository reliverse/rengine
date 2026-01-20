#![doc = "Sidecar module for class [`SceneTree`][crate::classes::SceneTree].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SceneTree` enums](https://docs.godotengine.org/en/stable/classes/class_scenetree.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SceneTree.`\n\nInherits [`MainLoop`][crate::classes::MainLoop].\n\nRelated symbols:\n\n* [`scene_tree`][crate::classes::scene_tree]: sidecar module with related enum/flag types\n* [`ISceneTree`][crate::classes::ISceneTree]: virtual methods\n* [`SignalsOfSceneTree`][crate::classes::scene_tree::SignalsOfSceneTree]: signal collection\n\n\nSee also [Godot docs for `SceneTree`](https://docs.godotengine.org/en/stable/classes/class_scenetree.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`SceneTree::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SceneTree {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`SceneTree`][crate::classes::SceneTree].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IMainLoop`][crate::classes::IMainLoop] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `SceneTree` methods](https://docs.godotengine.org/en/stable/classes/class_scenetree.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISceneTree: crate::obj::GodotClass < Base = SceneTree > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: MainLoopNotification) {
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
        fn initialize(&mut self,) {
            unimplemented !()
        }
        fn physics_process(&mut self, delta: f64,) -> bool {
            unimplemented !()
        }
        fn process(&mut self, delta: f64,) -> bool {
            unimplemented !()
        }
        fn finalize(&mut self,) {
            unimplemented !()
        }
    }
    impl SceneTree {
        pub fn get_root(&self,) -> Option < Gd < crate::classes::Window > > {
            type CallRet = Option < Gd < crate::classes::Window > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7845usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "get_root", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_group(&self, name: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7846usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "has_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_accessibility_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7847usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "is_accessibility_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_accessibility_supported(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7848usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "is_accessibility_supported", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_auto_accept_quit(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7849usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "is_auto_accept_quit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auto_accept_quit(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7850usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "set_auto_accept_quit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_quit_on_go_back(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7851usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "is_quit_on_go_back", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_quit_on_go_back(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7852usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "set_quit_on_go_back", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_debug_collisions_hint(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7853usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "set_debug_collisions_hint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_debugging_collisions_hint(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7854usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "is_debugging_collisions_hint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_debug_paths_hint(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7855usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "set_debug_paths_hint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_debugging_paths_hint(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7856usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "is_debugging_paths_hint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_debug_navigation_hint(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7857usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "set_debug_navigation_hint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_debugging_navigation_hint(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7858usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "is_debugging_navigation_hint", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_edited_scene_root(&mut self, scene: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (scene.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7859usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "set_edited_scene_root", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_edited_scene_root(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7860usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "get_edited_scene_root", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pause(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7861usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "set_pause", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_paused(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7862usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "is_paused", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn create_timer_full(&mut self, time_sec: f64, process_always: bool, process_in_physics: bool, ignore_time_scale: bool,) -> Option < Gd < crate::classes::SceneTreeTimer > > {
            type CallRet = Option < Gd < crate::classes::SceneTreeTimer > >;
            type CallParams = (f64, bool, bool, bool,);
            let args = (time_sec, process_always, process_in_physics, ignore_time_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7863usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "create_timer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::create_timer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn create_timer(&mut self, time_sec: f64,) -> Option < Gd < crate::classes::SceneTreeTimer > > {
            self.create_timer_ex(time_sec,) . done()
        }
        #[inline]
        pub fn create_timer_ex < 'a > (&'a mut self, time_sec: f64,) -> ExCreateTimer < 'a > {
            ExCreateTimer::new(self, time_sec,)
        }
        pub fn create_tween(&mut self,) -> Option < Gd < crate::classes::Tween > > {
            type CallRet = Option < Gd < crate::classes::Tween > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7864usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "create_tween", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_processed_tweens(&mut self,) -> Array < Gd < crate::classes::Tween > > {
            type CallRet = Array < Gd < crate::classes::Tween > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7865usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "get_processed_tweens", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7866usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "get_node_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_frame(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7867usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "get_frame", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn quit_full(&mut self, exit_code: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (exit_code,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7868usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "quit", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::quit_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn quit(&mut self,) {
            self.quit_ex() . done()
        }
        #[inline]
        pub fn quit_ex < 'a > (&'a mut self,) -> ExQuit < 'a > {
            ExQuit::new(self,)
        }
        pub fn set_physics_interpolation_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7869usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "set_physics_interpolation_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_physics_interpolation_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7870usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "is_physics_interpolation_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn queue_delete(&mut self, obj: impl AsArg < Option < Gd < crate::classes::Object >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >,);
            let args = (obj.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7871usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "queue_delete", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub(crate) fn raw_call_group_flags(&mut self, flags: i64, group: impl AsArg < StringName >, method: impl AsArg < StringName >, varargs: &[Variant]) {
            Self::try_raw_call_group_flags(self, flags, group, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub(crate) fn try_raw_call_group_flags(&mut self, flags: i64, group: impl AsArg < StringName >, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < (), crate::meta::error::CallError > {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (i64, CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (flags, group.into_arg(), method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7872usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "SceneTree", "raw_call_group_flags", Some(self.__validated_obj()), args, varargs)
            }
        }
        pub(crate) fn raw_notify_group_flags(&mut self, call_flags: u32, group: impl AsArg < StringName >, notification: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (u32, CowArg < 'a0, StringName >, i32,);
            let args = (call_flags, group.into_arg(), notification,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7873usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "raw_notify_group_flags", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn raw_set_group_flags(&mut self, call_flags: u32, group: impl AsArg < StringName >, property: impl AsArg < GString >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (u32, CowArg < 'a0, StringName >, CowArg < 'a1, GString >, RefArg < 'a2, Variant >,);
            let args = (call_flags, group.into_arg(), property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7874usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "raw_set_group_flags", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = r" # Panics"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will panic in such a case."]
        pub fn call_group(&mut self, group: impl AsArg < StringName >, method: impl AsArg < StringName >, varargs: &[Variant]) {
            Self::try_call_group(self, group, method, varargs) . unwrap_or_else(| e | panic !("{e}"))
        }
        #[doc = r" # Return type"]
        #[doc = r" This is a _varcall_ method, meaning parameters and return values are passed as `Variant`."]
        #[doc = r" It can detect call failures and will return `Err` in such a case."]
        pub fn try_call_group(&mut self, group: impl AsArg < StringName >, method: impl AsArg < StringName >, varargs: &[Variant]) -> Result < (), crate::meta::error::CallError > {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (group.into_arg(), method.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7875usize);
                Signature::< CallParams, CallRet > ::out_class_varcall(method_bind, "SceneTree", "call_group", Some(self.__validated_obj()), args, varargs)
            }
        }
        pub(crate) fn raw_notify_group(&mut self, group: impl AsArg < StringName >, notification: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, i32,);
            let args = (group.into_arg(), notification,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7876usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "raw_notify_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_group(&mut self, group: impl AsArg < StringName >, property: impl AsArg < GString >, value: &Variant,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, StringName >, CowArg < 'a1, GString >, RefArg < 'a2, Variant >,);
            let args = (group.into_arg(), property.into_arg(), RefArg::new(value),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7877usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "set_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_nodes_in_group(&mut self, group: impl AsArg < StringName >,) -> Array < Gd < crate::classes::Node > > {
            type CallRet = Array < Gd < crate::classes::Node > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (group.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7878usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "get_nodes_in_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_first_node_in_group(&mut self, group: impl AsArg < StringName >,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (group.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7879usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "get_first_node_in_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_node_count_in_group(&self, group: impl AsArg < StringName >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (group.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7880usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "get_node_count_in_group", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_current_scene(&mut self, child_node: impl AsArg < Option < Gd < crate::classes::Node >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Node > > >,);
            let args = (child_node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7881usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "set_current_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_current_scene(&self,) -> Option < Gd < crate::classes::Node > > {
            type CallRet = Option < Gd < crate::classes::Node > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7882usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "get_current_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn change_scene_to_file(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7883usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "change_scene_to_file", Some(self.__validated_obj()), args,)
            }
        }
        pub fn change_scene_to_packed(&mut self, packed_scene: impl AsArg < Option < Gd < crate::classes::PackedScene >> >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::PackedScene > > >,);
            let args = (packed_scene.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7884usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "change_scene_to_packed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn reload_current_scene(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7885usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "reload_current_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub fn unload_current_scene(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7886usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "unload_current_scene", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn set_multiplayer_full(&mut self, multiplayer: CowArg < Option < Gd < crate::classes::MultiplayerApi > > >, root_path: CowArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::MultiplayerApi > > >, CowArg < 'a1, NodePath >,);
            let args = (multiplayer, root_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7887usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "set_multiplayer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::set_multiplayer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn set_multiplayer(&mut self, multiplayer: impl AsArg < Option < Gd < crate::classes::MultiplayerApi >> >,) {
            self.set_multiplayer_ex(multiplayer,) . done()
        }
        #[inline]
        pub fn set_multiplayer_ex < 'a > (&'a mut self, multiplayer: impl AsArg < Option < Gd < crate::classes::MultiplayerApi >> > + 'a,) -> ExSetMultiplayer < 'a > {
            ExSetMultiplayer::new(self, multiplayer,)
        }
        pub(crate) fn get_multiplayer_full(&self, for_path: CowArg < NodePath >,) -> Option < Gd < crate::classes::MultiplayerApi > > {
            type CallRet = Option < Gd < crate::classes::MultiplayerApi > >;
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (for_path,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7888usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "get_multiplayer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::get_multiplayer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn get_multiplayer(&self,) -> Option < Gd < crate::classes::MultiplayerApi > > {
            self.get_multiplayer_ex() . done()
        }
        #[inline]
        pub fn get_multiplayer_ex < 'a > (&'a self,) -> ExGetMultiplayer < 'a > {
            ExGetMultiplayer::new(self,)
        }
        pub fn set_multiplayer_poll_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7889usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "set_multiplayer_poll_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_multiplayer_poll_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7890usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneTree", "is_multiplayer_poll_enabled", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for SceneTree {
        type Base = crate::classes::MainLoop;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("SceneTree"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SceneTree {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::MainLoop > for SceneTree {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SceneTree {
        
    }
    impl crate::obj::cap::GodotDefault for SceneTree {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SceneTree {
        type Target = crate::classes::MainLoop;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SceneTree {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SceneTree`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_SceneTree__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SceneTree > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::MainLoop > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`SceneTree::create_timer_ex`][super::SceneTree::create_timer_ex]."]
#[must_use]
pub struct ExCreateTimer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SceneTree, time_sec: f64, process_always: bool, process_in_physics: bool, ignore_time_scale: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExCreateTimer < 'a > {
    fn new(surround_object: &'a mut re_export::SceneTree, time_sec: f64,) -> Self {
        let process_always = true;
        let process_in_physics = false;
        let ignore_time_scale = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, time_sec: time_sec, process_always: process_always, process_in_physics: process_in_physics, ignore_time_scale: ignore_time_scale,
        }
    }
    #[inline]
    pub fn process_always(self, process_always: bool) -> Self {
        Self {
            process_always: process_always, .. self
        }
    }
    #[inline]
    pub fn process_in_physics(self, process_in_physics: bool) -> Self {
        Self {
            process_in_physics: process_in_physics, .. self
        }
    }
    #[inline]
    pub fn ignore_time_scale(self, ignore_time_scale: bool) -> Self {
        Self {
            ignore_time_scale: ignore_time_scale, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::SceneTreeTimer > > {
        let Self {
            _phantom, surround_object, time_sec, process_always, process_in_physics, ignore_time_scale,
        }
        = self;
        re_export::SceneTree::create_timer_full(surround_object, time_sec, process_always, process_in_physics, ignore_time_scale,)
    }
}
#[doc = "Default-param extender for [`SceneTree::quit_ex`][super::SceneTree::quit_ex]."]
#[must_use]
pub struct ExQuit < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SceneTree, exit_code: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExQuit < 'a > {
    fn new(surround_object: &'a mut re_export::SceneTree,) -> Self {
        let exit_code = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, exit_code: exit_code,
        }
    }
    #[inline]
    pub fn exit_code(self, exit_code: i32) -> Self {
        Self {
            exit_code: exit_code, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, exit_code,
        }
        = self;
        re_export::SceneTree::quit_full(surround_object, exit_code,)
    }
}
#[doc = "Default-param extender for [`SceneTree::set_multiplayer_ex`][super::SceneTree::set_multiplayer_ex]."]
#[must_use]
pub struct ExSetMultiplayer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::SceneTree, multiplayer: CowArg < 'a, Option < Gd < crate::classes::MultiplayerApi > > >, root_path: CowArg < 'a, NodePath >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExSetMultiplayer < 'a > {
    fn new(surround_object: &'a mut re_export::SceneTree, multiplayer: impl AsArg < Option < Gd < crate::classes::MultiplayerApi >> > + 'a,) -> Self {
        let root_path = NodePath::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, multiplayer: multiplayer.into_arg(), root_path: CowArg::Owned(root_path),
        }
    }
    #[inline]
    pub fn root_path(self, root_path: impl AsArg < NodePath > + 'a) -> Self {
        Self {
            root_path: root_path.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, multiplayer, root_path,
        }
        = self;
        re_export::SceneTree::set_multiplayer_full(surround_object, multiplayer, root_path,)
    }
}
#[doc = "Default-param extender for [`SceneTree::get_multiplayer_ex`][super::SceneTree::get_multiplayer_ex]."]
#[must_use]
pub struct ExGetMultiplayer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a re_export::SceneTree, for_path: CowArg < 'a, NodePath >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExGetMultiplayer < 'a > {
    fn new(surround_object: &'a re_export::SceneTree,) -> Self {
        let for_path = NodePath::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, for_path: CowArg::Owned(for_path),
        }
    }
    #[inline]
    pub fn for_path(self, for_path: impl AsArg < NodePath > + 'a) -> Self {
        Self {
            for_path: for_path.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::MultiplayerApi > > {
        let Self {
            _phantom, surround_object, for_path,
        }
        = self;
        re_export::SceneTree::get_multiplayer_full(surround_object, for_path,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct GroupCallFlags {
    ord: u64
}
impl GroupCallFlags {
    #[doc(alias = "GROUP_CALL_DEFAULT")]
    #[doc = "Godot enumerator name: `GROUP_CALL_DEFAULT`"]
    pub const DEFAULT: GroupCallFlags = GroupCallFlags {
        ord: 0u64
    };
    #[doc(alias = "GROUP_CALL_REVERSE")]
    #[doc = "Godot enumerator name: `GROUP_CALL_REVERSE`"]
    pub const REVERSE: GroupCallFlags = GroupCallFlags {
        ord: 1u64
    };
    #[doc(alias = "GROUP_CALL_DEFERRED")]
    #[doc = "Godot enumerator name: `GROUP_CALL_DEFERRED`"]
    pub const DEFERRED: GroupCallFlags = GroupCallFlags {
        ord: 2u64
    };
    #[doc(alias = "GROUP_CALL_UNIQUE")]
    #[doc = "Godot enumerator name: `GROUP_CALL_UNIQUE`"]
    pub const UNIQUE: GroupCallFlags = GroupCallFlags {
        ord: 4u64
    };
    
}
impl std::fmt::Debug for GroupCallFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let enumerator = match * self {
            Self::DEFAULT => "DEFAULT", Self::REVERSE => "REVERSE", Self::DEFERRED => "DEFERRED", Self::UNIQUE => "UNIQUE", _ => {
                f.debug_struct("GroupCallFlags") . field("ord", &self.ord) . finish() ?;
                return Ok(());
                
            }
        };
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineBitfield for GroupCallFlags {
    fn try_from_ord(ord: u64) -> Option < Self > {
        Some(Self {
            ord
        })
    }
    fn ord(self) -> u64 {
        self.ord
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GroupCallFlags >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "GROUP_CALL_DEFAULT", GroupCallFlags::DEFAULT), crate::meta::inspect::EnumConstant::new("REVERSE", "GROUP_CALL_REVERSE", GroupCallFlags::REVERSE), crate::meta::inspect::EnumConstant::new("DEFERRED", "GROUP_CALL_DEFERRED", GroupCallFlags::DEFERRED), crate::meta::inspect::EnumConstant::new("UNIQUE", "GROUP_CALL_UNIQUE", GroupCallFlags::UNIQUE)]
        }
    }
}
impl std::ops::BitOr for GroupCallFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord
        }
    }
}
impl std::ops::BitOrAssign for GroupCallFlags {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        * self = * self | rhs;
        
    }
}
impl crate::meta::GodotConvert for GroupCallFlags {
    type Via = u64;
    
}
impl crate::meta::ToGodot for GroupCallFlags {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineBitfield > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GroupCallFlags {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineBitfield > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::SceneTree;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`SceneTree`][crate::classes::SceneTree] class."]
    pub struct SignalsOfSceneTree < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfSceneTree < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn tree_changed(&mut self) -> SigTreeChanged < 'c, C > {
            SigTreeChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tree_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn scene_changed(&mut self) -> SigSceneChanged < 'c, C > {
            SigSceneChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "scene_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn tree_process_mode_changed(&mut self) -> SigTreeProcessModeChanged < 'c, C > {
            SigTreeProcessModeChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "tree_process_mode_changed")
            }
        }
        #[doc = "Signature: `(node: Gd<Node>)`"]
        pub fn node_added(&mut self) -> SigNodeAdded < 'c, C > {
            SigNodeAdded {
                typed: TypedSignal::extract(&mut self.__internal_obj, "node_added")
            }
        }
        #[doc = "Signature: `(node: Gd<Node>)`"]
        pub fn node_removed(&mut self) -> SigNodeRemoved < 'c, C > {
            SigNodeRemoved {
                typed: TypedSignal::extract(&mut self.__internal_obj, "node_removed")
            }
        }
        #[doc = "Signature: `(node: Gd<Node>)`"]
        pub fn node_renamed(&mut self) -> SigNodeRenamed < 'c, C > {
            SigNodeRenamed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "node_renamed")
            }
        }
        #[doc = "Signature: `(node: Gd<Node>)`"]
        pub fn node_configuration_warning_changed(&mut self) -> SigNodeConfigurationWarningChanged < 'c, C > {
            SigNodeConfigurationWarningChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "node_configuration_warning_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn process_frame(&mut self) -> SigProcessFrame < 'c, C > {
            SigProcessFrame {
                typed: TypedSignal::extract(&mut self.__internal_obj, "process_frame")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn physics_frame(&mut self) -> SigPhysicsFrame < 'c, C > {
            SigPhysicsFrame {
                typed: TypedSignal::extract(&mut self.__internal_obj, "physics_frame")
            }
        }
    }
    type TypedSigTreeChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTreeChanged < 'c, C: WithSignals > {
        typed: TypedSigTreeChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTreeChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTreeChanged < 'c, C > {
        type Target = TypedSigTreeChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTreeChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigSceneChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigSceneChanged < 'c, C: WithSignals > {
        typed: TypedSigSceneChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSceneChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSceneChanged < 'c, C > {
        type Target = TypedSigSceneChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSceneChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigTreeProcessModeChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigTreeProcessModeChanged < 'c, C: WithSignals > {
        typed: TypedSigTreeProcessModeChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigTreeProcessModeChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigTreeProcessModeChanged < 'c, C > {
        type Target = TypedSigTreeProcessModeChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigTreeProcessModeChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigNodeAdded < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigNodeAdded < 'c, C: WithSignals > {
        typed: TypedSigNodeAdded < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigNodeAdded < 'c, C > {
        pub fn emit(&mut self, node: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((node,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigNodeAdded < 'c, C > {
        type Target = TypedSigNodeAdded < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigNodeAdded < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigNodeRemoved < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigNodeRemoved < 'c, C: WithSignals > {
        typed: TypedSigNodeRemoved < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigNodeRemoved < 'c, C > {
        pub fn emit(&mut self, node: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((node,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigNodeRemoved < 'c, C > {
        type Target = TypedSigNodeRemoved < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigNodeRemoved < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigNodeRenamed < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigNodeRenamed < 'c, C: WithSignals > {
        typed: TypedSigNodeRenamed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigNodeRenamed < 'c, C > {
        pub fn emit(&mut self, node: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((node,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigNodeRenamed < 'c, C > {
        type Target = TypedSigNodeRenamed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigNodeRenamed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigNodeConfigurationWarningChanged < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Node >,) >;
    pub struct SigNodeConfigurationWarningChanged < 'c, C: WithSignals > {
        typed: TypedSigNodeConfigurationWarningChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigNodeConfigurationWarningChanged < 'c, C > {
        pub fn emit(&mut self, node: Gd < crate::classes::Node >,) {
            self.typed.emit_tuple((node,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigNodeConfigurationWarningChanged < 'c, C > {
        type Target = TypedSigNodeConfigurationWarningChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigNodeConfigurationWarningChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigProcessFrame < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigProcessFrame < 'c, C: WithSignals > {
        typed: TypedSigProcessFrame < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigProcessFrame < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigProcessFrame < 'c, C > {
        type Target = TypedSigProcessFrame < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigProcessFrame < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigPhysicsFrame < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigPhysicsFrame < 'c, C: WithSignals > {
        typed: TypedSigPhysicsFrame < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPhysicsFrame < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPhysicsFrame < 'c, C > {
        type Target = TypedSigPhysicsFrame < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPhysicsFrame < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for SceneTree {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfSceneTree < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfSceneTree < 'c, C > {
        type Target = < < SceneTree as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = SceneTree;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfSceneTree < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = SceneTree;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}