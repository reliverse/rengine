#![doc = "Sidecar module for class [`CodeHighlighter`][crate::classes::CodeHighlighter].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CodeHighlighter` enums](https://docs.godotengine.org/en/stable/classes/class_codehighlighter.html#enumerations).\n\n"]
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
    #[doc = "Godot class `CodeHighlighter.`\n\nInherits [`SyntaxHighlighter`][crate::classes::SyntaxHighlighter].\n\nRelated symbols:\n\n* [`code_highlighter`][crate::classes::code_highlighter]: sidecar module with related enum/flag types\n* [`ICodeHighlighter`][crate::classes::ICodeHighlighter]: virtual methods\n\n\nSee also [Godot docs for `CodeHighlighter`](https://docs.godotengine.org/en/stable/classes/class_codehighlighter.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`CodeHighlighter::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct CodeHighlighter {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`CodeHighlighter`][crate::classes::CodeHighlighter].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`ISyntaxHighlighter`][crate::classes::ISyntaxHighlighter] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `CodeHighlighter` methods](https://docs.godotengine.org/en/stable/classes/class_codehighlighter.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ICodeHighlighter: crate::obj::GodotClass < Base = CodeHighlighter > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_line_syntax_highlighting(&self, line: i32,) -> VarDictionary {
            unimplemented !()
        }
        fn clear_highlighting_cache(&mut self,) {
            unimplemented !()
        }
        fn update_cache(&mut self,) {
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
    impl CodeHighlighter {
        pub fn add_keyword_color(&mut self, keyword: impl AsArg < GString >, color: Color,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, Color,);
            let args = (keyword.into_arg(), color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "add_keyword_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_keyword_color(&mut self, keyword: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (keyword.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "remove_keyword_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_keyword_color(&self, keyword: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (keyword.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "has_keyword_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_keyword_color(&self, keyword: impl AsArg < GString >,) -> Color {
            type CallRet = Color;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (keyword.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_keyword_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_keyword_colors(&mut self, keywords: &VarDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >,);
            let args = (RefArg::new(keywords),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "set_keyword_colors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_keyword_colors(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "clear_keyword_colors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_keyword_colors(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_keyword_colors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_member_keyword_color(&mut self, member_keyword: impl AsArg < GString >, color: Color,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, Color,);
            let args = (member_keyword.into_arg(), color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "add_member_keyword_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_member_keyword_color(&mut self, member_keyword: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (member_keyword.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "remove_member_keyword_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_member_keyword_color(&self, member_keyword: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (member_keyword.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "has_member_keyword_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_member_keyword_color(&self, member_keyword: impl AsArg < GString >,) -> Color {
            type CallRet = Color;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (member_keyword.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_member_keyword_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_member_keyword_colors(&mut self, member_keyword: &VarDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >,);
            let args = (RefArg::new(member_keyword),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "set_member_keyword_colors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_member_keyword_colors(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "clear_member_keyword_colors", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_member_keyword_colors(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_member_keyword_colors", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_color_region_full(&mut self, start_key: CowArg < GString >, end_key: CowArg < GString >, color: Color, line_only: bool,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, Color, bool,);
            let args = (start_key, end_key, color, line_only,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "add_color_region", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_color_region_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_color_region(&mut self, start_key: impl AsArg < GString >, end_key: impl AsArg < GString >, color: Color,) {
            self.add_color_region_ex(start_key, end_key, color,) . done()
        }
        #[inline]
        pub fn add_color_region_ex < 'a > (&'a mut self, start_key: impl AsArg < GString > + 'a, end_key: impl AsArg < GString > + 'a, color: Color,) -> ExAddColorRegion < 'a > {
            ExAddColorRegion::new(self, start_key, end_key, color,)
        }
        pub fn remove_color_region(&mut self, start_key: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (start_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "remove_color_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_color_region(&self, start_key: impl AsArg < GString >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (start_key.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "has_color_region", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_color_regions(&mut self, color_regions: &VarDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, VarDictionary >,);
            let args = (RefArg::new(color_regions),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "set_color_regions", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_color_regions(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "clear_color_regions", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_color_regions(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_color_regions", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_function_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "set_function_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_function_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_function_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_number_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "set_number_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_number_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_number_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_symbol_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "set_symbol_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_symbol_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_symbol_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_member_variable_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "set_member_variable_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_member_variable_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "CodeHighlighter", "get_member_variable_color", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for CodeHighlighter {
        type Base = crate::classes::SyntaxHighlighter;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("CodeHighlighter"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for CodeHighlighter {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::SyntaxHighlighter > for CodeHighlighter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for CodeHighlighter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for CodeHighlighter {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for CodeHighlighter {
        
    }
    impl crate::obj::cap::GodotDefault for CodeHighlighter {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for CodeHighlighter {
        type Target = crate::classes::SyntaxHighlighter;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for CodeHighlighter {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`CodeHighlighter`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_CodeHighlighter__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::CodeHighlighter > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::SyntaxHighlighter > for $Class {
                
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
#[doc = "Default-param extender for [`CodeHighlighter::add_color_region_ex`][super::CodeHighlighter::add_color_region_ex]."]
#[must_use]
pub struct ExAddColorRegion < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::CodeHighlighter, start_key: CowArg < 'a, GString >, end_key: CowArg < 'a, GString >, color: Color, line_only: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddColorRegion < 'a > {
    fn new(surround_object: &'a mut re_export::CodeHighlighter, start_key: impl AsArg < GString > + 'a, end_key: impl AsArg < GString > + 'a, color: Color,) -> Self {
        let line_only = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, start_key: start_key.into_arg(), end_key: end_key.into_arg(), color: color, line_only: line_only,
        }
    }
    #[inline]
    pub fn line_only(self, line_only: bool) -> Self {
        Self {
            line_only: line_only, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, start_key, end_key, color, line_only,
        }
        = self;
        re_export::CodeHighlighter::add_color_region_full(surround_object, start_key, end_key, color, line_only,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::CodeHighlighter;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for CodeHighlighter {
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