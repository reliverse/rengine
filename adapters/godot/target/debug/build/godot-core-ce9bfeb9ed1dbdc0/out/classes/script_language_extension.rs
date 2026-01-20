#![doc = "Sidecar module for class [`ScriptLanguageExtension`][crate::classes::ScriptLanguageExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ScriptLanguageExtension` enums](https://docs.godotengine.org/en/stable/classes/class_scriptlanguageextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ScriptLanguageExtension.`\n\nInherits [`ScriptLanguage`][crate::classes::ScriptLanguage].\n\nRelated symbols:\n\n* [`script_language_extension`][crate::classes::script_language_extension]: sidecar module with related enum/flag types\n* [`IScriptLanguageExtension`][crate::classes::IScriptLanguageExtension]: virtual methods\n\n\nSee also [Godot docs for `ScriptLanguageExtension`](https://docs.godotengine.org/en/stable/classes/class_scriptlanguageextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`ScriptLanguageExtension::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ScriptLanguageExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ScriptLanguageExtension`][crate::classes::ScriptLanguageExtension].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IScriptLanguage`~~ > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `ScriptLanguageExtension` methods](https://docs.godotengine.org/en/stable/classes/class_scriptlanguageextension.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IScriptLanguageExtension: crate::obj::GodotClass < Base = ScriptLanguageExtension > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn get_name(&self,) -> GString;
        fn init_ext(&mut self,);
        fn get_type(&self,) -> GString;
        fn get_extension(&self,) -> GString;
        fn finish(&mut self,);
        fn get_reserved_words(&self,) -> PackedStringArray;
        fn is_control_flow_keyword(&self, keyword: GString,) -> bool;
        fn get_comment_delimiters(&self,) -> PackedStringArray;
        fn get_doc_comment_delimiters(&self,) -> PackedStringArray {
            unimplemented !()
        }
        fn get_string_delimiters(&self,) -> PackedStringArray;
        fn make_template(&self, template: GString, class_name: GString, base_class_name: GString,) -> Option < Gd < crate::classes::Script > >;
        fn get_built_in_templates(&self, object: StringName,) -> Array < VarDictionary >;
        fn is_using_templates(&mut self,) -> bool;
        fn validate(&self, script: GString, path: GString, validate_functions: bool, validate_errors: bool, validate_warnings: bool, validate_safe_lines: bool,) -> VarDictionary;
        fn validate_path(&self, path: GString,) -> GString;
        fn create_script(&self,) -> Option < Gd < crate::classes::Object > >;
        fn has_named_classes(&self,) -> bool;
        fn supports_builtin_mode(&self,) -> bool;
        fn supports_documentation(&self,) -> bool;
        fn can_inherit_from_file(&self,) -> bool;
        fn find_function(&self, function: GString, code: GString,) -> i32;
        fn make_function(&self, class_name: GString, function_name: GString, function_args: PackedStringArray,) -> GString;
        fn can_make_function(&self,) -> bool;
        fn open_in_external_editor(&mut self, script: Option < Gd < crate::classes::Script > >, line: i32, column: i32,) -> crate::global::Error;
        fn overrides_external_editor(&mut self,) -> bool;
        fn preferred_file_name_casing(&self,) -> crate::classes::script_language::ScriptNameCasing {
            unimplemented !()
        }
        fn complete_code(&self, code: GString, path: GString, owner: Option < Gd < crate::classes::Object > >,) -> VarDictionary;
        fn lookup_code(&self, code: GString, symbol: GString, path: GString, owner: Option < Gd < crate::classes::Object > >,) -> VarDictionary;
        fn auto_indent_code(&self, code: GString, from_line: i32, to_line: i32,) -> GString;
        fn add_global_constant(&mut self, name: StringName, value: Variant,);
        fn add_named_global_constant(&mut self, name: StringName, value: Variant,);
        fn remove_named_global_constant(&mut self, name: StringName,);
        fn thread_enter(&mut self,);
        fn thread_exit(&mut self,);
        fn debug_get_error(&self,) -> GString;
        fn debug_get_stack_level_count(&self,) -> i32;
        fn debug_get_stack_level_line(&self, level: i32,) -> i32;
        fn debug_get_stack_level_function(&self, level: i32,) -> GString;
        fn debug_get_stack_level_source(&self, level: i32,) -> GString;
        fn debug_get_stack_level_locals(&mut self, level: i32, max_subitems: i32, max_depth: i32,) -> VarDictionary;
        fn debug_get_stack_level_members(&mut self, level: i32, max_subitems: i32, max_depth: i32,) -> VarDictionary;
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn debug_get_stack_level_instance_rawptr(&mut self, level: i32,) -> * mut c_void;
        fn debug_get_globals(&mut self, max_subitems: i32, max_depth: i32,) -> VarDictionary;
        fn debug_parse_stack_level_expression(&mut self, level: i32, expression: GString, max_subitems: i32, max_depth: i32,) -> GString;
        fn debug_get_current_stack_info(&mut self,) -> Array < VarDictionary >;
        fn reload_all_scripts(&mut self,);
        fn reload_scripts(&mut self, scripts: VarArray, soft_reload: bool,);
        fn reload_tool_script(&mut self, script: Option < Gd < crate::classes::Script > >, soft_reload: bool,);
        fn get_recognized_extensions(&self,) -> PackedStringArray;
        fn get_public_functions(&self,) -> Array < VarDictionary >;
        fn get_public_constants(&self,) -> VarDictionary;
        fn get_public_annotations(&self,) -> Array < VarDictionary >;
        fn profiling_start(&mut self,);
        fn profiling_stop(&mut self,);
        fn profiling_set_save_native_calls(&mut self, enable: bool,);
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn profiling_get_accumulated_data_rawptr(&mut self, info_array: * mut ScriptLanguageExtensionProfilingInfo, info_max: i32,) -> i32;
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn profiling_get_frame_data_rawptr(&mut self, info_array: * mut ScriptLanguageExtensionProfilingInfo, info_max: i32,) -> i32;
        fn frame(&mut self,);
        fn handles_global_class_type(&self, type_: GString,) -> bool;
        fn get_global_class_name(&self, path: GString,) -> VarDictionary;
        
    }
    impl ScriptLanguageExtension {
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
    impl crate::obj::GodotClass for ScriptLanguageExtension {
        type Base = crate::classes::ScriptLanguage;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ScriptLanguageExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ScriptLanguageExtension {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::ScriptLanguage > for ScriptLanguageExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ScriptLanguageExtension {
        
    }
    impl crate::obj::cap::GodotDefault for ScriptLanguageExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ScriptLanguageExtension {
        type Target = crate::classes::ScriptLanguage;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ScriptLanguageExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ScriptLanguageExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ScriptLanguageExtension__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ScriptLanguageExtension > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::ScriptLanguage > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LookupResultType {
    ord: i32
}
impl LookupResultType {
    #[doc(alias = "LOOKUP_RESULT_SCRIPT_LOCATION")]
    #[doc = "Godot enumerator name: `LOOKUP_RESULT_SCRIPT_LOCATION`"]
    pub const SCRIPT_LOCATION: LookupResultType = LookupResultType {
        ord: 0i32
    };
    #[doc(alias = "LOOKUP_RESULT_CLASS")]
    #[doc = "Godot enumerator name: `LOOKUP_RESULT_CLASS`"]
    pub const CLASS: LookupResultType = LookupResultType {
        ord: 1i32
    };
    #[doc(alias = "LOOKUP_RESULT_CLASS_CONSTANT")]
    #[doc = "Godot enumerator name: `LOOKUP_RESULT_CLASS_CONSTANT`"]
    pub const CLASS_CONSTANT: LookupResultType = LookupResultType {
        ord: 2i32
    };
    #[doc(alias = "LOOKUP_RESULT_CLASS_PROPERTY")]
    #[doc = "Godot enumerator name: `LOOKUP_RESULT_CLASS_PROPERTY`"]
    pub const CLASS_PROPERTY: LookupResultType = LookupResultType {
        ord: 3i32
    };
    #[doc(alias = "LOOKUP_RESULT_CLASS_METHOD")]
    #[doc = "Godot enumerator name: `LOOKUP_RESULT_CLASS_METHOD`"]
    pub const CLASS_METHOD: LookupResultType = LookupResultType {
        ord: 4i32
    };
    #[doc(alias = "LOOKUP_RESULT_CLASS_SIGNAL")]
    #[doc = "Godot enumerator name: `LOOKUP_RESULT_CLASS_SIGNAL`"]
    pub const CLASS_SIGNAL: LookupResultType = LookupResultType {
        ord: 5i32
    };
    #[doc(alias = "LOOKUP_RESULT_CLASS_ENUM")]
    #[doc = "Godot enumerator name: `LOOKUP_RESULT_CLASS_ENUM`"]
    pub const CLASS_ENUM: LookupResultType = LookupResultType {
        ord: 6i32
    };
    #[doc(alias = "LOOKUP_RESULT_CLASS_TBD_GLOBALSCOPE")]
    #[doc = "Godot enumerator name: `LOOKUP_RESULT_CLASS_TBD_GLOBALSCOPE`"]
    pub const CLASS_TBD_GLOBALSCOPE: LookupResultType = LookupResultType {
        ord: 7i32
    };
    #[doc(alias = "LOOKUP_RESULT_CLASS_ANNOTATION")]
    #[doc = "Godot enumerator name: `LOOKUP_RESULT_CLASS_ANNOTATION`"]
    pub const CLASS_ANNOTATION: LookupResultType = LookupResultType {
        ord: 8i32
    };
    #[doc(alias = "LOOKUP_RESULT_LOCAL_CONSTANT")]
    #[doc = "Godot enumerator name: `LOOKUP_RESULT_LOCAL_CONSTANT`"]
    pub const LOCAL_CONSTANT: LookupResultType = LookupResultType {
        ord: 9i32
    };
    #[doc(alias = "LOOKUP_RESULT_LOCAL_VARIABLE")]
    #[doc = "Godot enumerator name: `LOOKUP_RESULT_LOCAL_VARIABLE`"]
    pub const LOCAL_VARIABLE: LookupResultType = LookupResultType {
        ord: 10i32
    };
    #[doc(alias = "LOOKUP_RESULT_MAX")]
    #[doc = "Godot enumerator name: `LOOKUP_RESULT_MAX`"]
    pub const MAX: LookupResultType = LookupResultType {
        ord: 11i32
    };
    
}
impl std::fmt::Debug for LookupResultType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LookupResultType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LookupResultType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 => Some(Self {
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
            Self::SCRIPT_LOCATION => "SCRIPT_LOCATION", Self::CLASS => "CLASS", Self::CLASS_CONSTANT => "CLASS_CONSTANT", Self::CLASS_PROPERTY => "CLASS_PROPERTY", Self::CLASS_METHOD => "CLASS_METHOD", Self::CLASS_SIGNAL => "CLASS_SIGNAL", Self::CLASS_ENUM => "CLASS_ENUM", Self::CLASS_TBD_GLOBALSCOPE => "CLASS_TBD_GLOBALSCOPE", Self::CLASS_ANNOTATION => "CLASS_ANNOTATION", Self::LOCAL_CONSTANT => "LOCAL_CONSTANT", Self::LOCAL_VARIABLE => "LOCAL_VARIABLE", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LookupResultType::SCRIPT_LOCATION, LookupResultType::CLASS, LookupResultType::CLASS_CONSTANT, LookupResultType::CLASS_PROPERTY, LookupResultType::CLASS_METHOD, LookupResultType::CLASS_SIGNAL, LookupResultType::CLASS_ENUM, LookupResultType::CLASS_TBD_GLOBALSCOPE, LookupResultType::CLASS_ANNOTATION, LookupResultType::LOCAL_CONSTANT, LookupResultType::LOCAL_VARIABLE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LookupResultType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SCRIPT_LOCATION", "LOOKUP_RESULT_SCRIPT_LOCATION", LookupResultType::SCRIPT_LOCATION), crate::meta::inspect::EnumConstant::new("CLASS", "LOOKUP_RESULT_CLASS", LookupResultType::CLASS), crate::meta::inspect::EnumConstant::new("CLASS_CONSTANT", "LOOKUP_RESULT_CLASS_CONSTANT", LookupResultType::CLASS_CONSTANT), crate::meta::inspect::EnumConstant::new("CLASS_PROPERTY", "LOOKUP_RESULT_CLASS_PROPERTY", LookupResultType::CLASS_PROPERTY), crate::meta::inspect::EnumConstant::new("CLASS_METHOD", "LOOKUP_RESULT_CLASS_METHOD", LookupResultType::CLASS_METHOD), crate::meta::inspect::EnumConstant::new("CLASS_SIGNAL", "LOOKUP_RESULT_CLASS_SIGNAL", LookupResultType::CLASS_SIGNAL), crate::meta::inspect::EnumConstant::new("CLASS_ENUM", "LOOKUP_RESULT_CLASS_ENUM", LookupResultType::CLASS_ENUM), crate::meta::inspect::EnumConstant::new("CLASS_TBD_GLOBALSCOPE", "LOOKUP_RESULT_CLASS_TBD_GLOBALSCOPE", LookupResultType::CLASS_TBD_GLOBALSCOPE), crate::meta::inspect::EnumConstant::new("CLASS_ANNOTATION", "LOOKUP_RESULT_CLASS_ANNOTATION", LookupResultType::CLASS_ANNOTATION), crate::meta::inspect::EnumConstant::new("LOCAL_CONSTANT", "LOOKUP_RESULT_LOCAL_CONSTANT", LookupResultType::LOCAL_CONSTANT), crate::meta::inspect::EnumConstant::new("LOCAL_VARIABLE", "LOOKUP_RESULT_LOCAL_VARIABLE", LookupResultType::LOCAL_VARIABLE), crate::meta::inspect::EnumConstant::new("MAX", "LOOKUP_RESULT_MAX", LookupResultType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for LookupResultType {
    const ENUMERATOR_COUNT: usize = 11usize;
    
}
impl crate::meta::GodotConvert for LookupResultType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LookupResultType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LookupResultType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CodeCompletionLocation {
    ord: i32
}
impl CodeCompletionLocation {
    #[doc(alias = "LOCATION_LOCAL")]
    #[doc = "Godot enumerator name: `LOCATION_LOCAL`"]
    pub const LOCAL: CodeCompletionLocation = CodeCompletionLocation {
        ord: 0i32
    };
    #[doc(alias = "LOCATION_PARENT_MASK")]
    #[doc = "Godot enumerator name: `LOCATION_PARENT_MASK`"]
    pub const PARENT_MASK: CodeCompletionLocation = CodeCompletionLocation {
        ord: 256i32
    };
    #[doc(alias = "LOCATION_OTHER_USER_CODE")]
    #[doc = "Godot enumerator name: `LOCATION_OTHER_USER_CODE`"]
    pub const OTHER_USER_CODE: CodeCompletionLocation = CodeCompletionLocation {
        ord: 512i32
    };
    #[doc(alias = "LOCATION_OTHER")]
    #[doc = "Godot enumerator name: `LOCATION_OTHER`"]
    pub const OTHER: CodeCompletionLocation = CodeCompletionLocation {
        ord: 1024i32
    };
    
}
impl std::fmt::Debug for CodeCompletionLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CodeCompletionLocation") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CodeCompletionLocation {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 256i32 | ord @ 512i32 | ord @ 1024i32 => Some(Self {
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
            Self::LOCAL => "LOCAL", Self::PARENT_MASK => "PARENT_MASK", Self::OTHER_USER_CODE => "OTHER_USER_CODE", Self::OTHER => "OTHER", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CodeCompletionLocation::LOCAL, CodeCompletionLocation::PARENT_MASK, CodeCompletionLocation::OTHER_USER_CODE, CodeCompletionLocation::OTHER]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CodeCompletionLocation >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LOCAL", "LOCATION_LOCAL", CodeCompletionLocation::LOCAL), crate::meta::inspect::EnumConstant::new("PARENT_MASK", "LOCATION_PARENT_MASK", CodeCompletionLocation::PARENT_MASK), crate::meta::inspect::EnumConstant::new("OTHER_USER_CODE", "LOCATION_OTHER_USER_CODE", CodeCompletionLocation::OTHER_USER_CODE), crate::meta::inspect::EnumConstant::new("OTHER", "LOCATION_OTHER", CodeCompletionLocation::OTHER)]
        }
    }
}
impl crate::meta::GodotConvert for CodeCompletionLocation {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CodeCompletionLocation {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CodeCompletionLocation {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CodeCompletionKind {
    ord: i32
}
impl CodeCompletionKind {
    #[doc(alias = "CODE_COMPLETION_KIND_CLASS")]
    #[doc = "Godot enumerator name: `CODE_COMPLETION_KIND_CLASS`"]
    pub const CLASS: CodeCompletionKind = CodeCompletionKind {
        ord: 0i32
    };
    #[doc(alias = "CODE_COMPLETION_KIND_FUNCTION")]
    #[doc = "Godot enumerator name: `CODE_COMPLETION_KIND_FUNCTION`"]
    pub const FUNCTION: CodeCompletionKind = CodeCompletionKind {
        ord: 1i32
    };
    #[doc(alias = "CODE_COMPLETION_KIND_SIGNAL")]
    #[doc = "Godot enumerator name: `CODE_COMPLETION_KIND_SIGNAL`"]
    pub const SIGNAL: CodeCompletionKind = CodeCompletionKind {
        ord: 2i32
    };
    #[doc(alias = "CODE_COMPLETION_KIND_VARIABLE")]
    #[doc = "Godot enumerator name: `CODE_COMPLETION_KIND_VARIABLE`"]
    pub const VARIABLE: CodeCompletionKind = CodeCompletionKind {
        ord: 3i32
    };
    #[doc(alias = "CODE_COMPLETION_KIND_MEMBER")]
    #[doc = "Godot enumerator name: `CODE_COMPLETION_KIND_MEMBER`"]
    pub const MEMBER: CodeCompletionKind = CodeCompletionKind {
        ord: 4i32
    };
    #[doc(alias = "CODE_COMPLETION_KIND_ENUM")]
    #[doc = "Godot enumerator name: `CODE_COMPLETION_KIND_ENUM`"]
    pub const ENUM: CodeCompletionKind = CodeCompletionKind {
        ord: 5i32
    };
    #[doc(alias = "CODE_COMPLETION_KIND_CONSTANT")]
    #[doc = "Godot enumerator name: `CODE_COMPLETION_KIND_CONSTANT`"]
    pub const CONSTANT: CodeCompletionKind = CodeCompletionKind {
        ord: 6i32
    };
    #[doc(alias = "CODE_COMPLETION_KIND_NODE_PATH")]
    #[doc = "Godot enumerator name: `CODE_COMPLETION_KIND_NODE_PATH`"]
    pub const NODE_PATH: CodeCompletionKind = CodeCompletionKind {
        ord: 7i32
    };
    #[doc(alias = "CODE_COMPLETION_KIND_FILE_PATH")]
    #[doc = "Godot enumerator name: `CODE_COMPLETION_KIND_FILE_PATH`"]
    pub const FILE_PATH: CodeCompletionKind = CodeCompletionKind {
        ord: 8i32
    };
    #[doc(alias = "CODE_COMPLETION_KIND_PLAIN_TEXT")]
    #[doc = "Godot enumerator name: `CODE_COMPLETION_KIND_PLAIN_TEXT`"]
    pub const PLAIN_TEXT: CodeCompletionKind = CodeCompletionKind {
        ord: 9i32
    };
    #[doc(alias = "CODE_COMPLETION_KIND_MAX")]
    #[doc = "Godot enumerator name: `CODE_COMPLETION_KIND_MAX`"]
    pub const MAX: CodeCompletionKind = CodeCompletionKind {
        ord: 10i32
    };
    
}
impl std::fmt::Debug for CodeCompletionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("CodeCompletionKind") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for CodeCompletionKind {
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
            Self::CLASS => "CLASS", Self::FUNCTION => "FUNCTION", Self::SIGNAL => "SIGNAL", Self::VARIABLE => "VARIABLE", Self::MEMBER => "MEMBER", Self::ENUM => "ENUM", Self::CONSTANT => "CONSTANT", Self::NODE_PATH => "NODE_PATH", Self::FILE_PATH => "FILE_PATH", Self::PLAIN_TEXT => "PLAIN_TEXT", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[CodeCompletionKind::CLASS, CodeCompletionKind::FUNCTION, CodeCompletionKind::SIGNAL, CodeCompletionKind::VARIABLE, CodeCompletionKind::MEMBER, CodeCompletionKind::ENUM, CodeCompletionKind::CONSTANT, CodeCompletionKind::NODE_PATH, CodeCompletionKind::FILE_PATH, CodeCompletionKind::PLAIN_TEXT]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < CodeCompletionKind >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CLASS", "CODE_COMPLETION_KIND_CLASS", CodeCompletionKind::CLASS), crate::meta::inspect::EnumConstant::new("FUNCTION", "CODE_COMPLETION_KIND_FUNCTION", CodeCompletionKind::FUNCTION), crate::meta::inspect::EnumConstant::new("SIGNAL", "CODE_COMPLETION_KIND_SIGNAL", CodeCompletionKind::SIGNAL), crate::meta::inspect::EnumConstant::new("VARIABLE", "CODE_COMPLETION_KIND_VARIABLE", CodeCompletionKind::VARIABLE), crate::meta::inspect::EnumConstant::new("MEMBER", "CODE_COMPLETION_KIND_MEMBER", CodeCompletionKind::MEMBER), crate::meta::inspect::EnumConstant::new("ENUM", "CODE_COMPLETION_KIND_ENUM", CodeCompletionKind::ENUM), crate::meta::inspect::EnumConstant::new("CONSTANT", "CODE_COMPLETION_KIND_CONSTANT", CodeCompletionKind::CONSTANT), crate::meta::inspect::EnumConstant::new("NODE_PATH", "CODE_COMPLETION_KIND_NODE_PATH", CodeCompletionKind::NODE_PATH), crate::meta::inspect::EnumConstant::new("FILE_PATH", "CODE_COMPLETION_KIND_FILE_PATH", CodeCompletionKind::FILE_PATH), crate::meta::inspect::EnumConstant::new("PLAIN_TEXT", "CODE_COMPLETION_KIND_PLAIN_TEXT", CodeCompletionKind::PLAIN_TEXT), crate::meta::inspect::EnumConstant::new("MAX", "CODE_COMPLETION_KIND_MAX", CodeCompletionKind::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for CodeCompletionKind {
    const ENUMERATOR_COUNT: usize = 10usize;
    
}
impl crate::meta::GodotConvert for CodeCompletionKind {
    type Via = i32;
    
}
impl crate::meta::ToGodot for CodeCompletionKind {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for CodeCompletionKind {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ScriptLanguageExtension;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for ScriptLanguageExtension {
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