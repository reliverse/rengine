#![doc = "Sidecar module for class [`EditorVcsInterface`][crate::classes::EditorVcsInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorVCSInterface` enums](https://docs.godotengine.org/en/stable/classes/class_editorvcsinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorVCSInterface.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`editor_vcs_interface`][crate::classes::editor_vcs_interface]: sidecar module with related enum/flag types\n* [`IEditorVcsInterface`][crate::classes::IEditorVcsInterface]: virtual methods\n\n\nSee also [Godot docs for `EditorVCSInterface`](https://docs.godotengine.org/en/stable/classes/class_editorvcsinterface.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`EditorVcsInterface::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorVcsInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorVcsInterface`][crate::classes::EditorVcsInterface].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `EditorVCSInterface` methods](https://docs.godotengine.org/en/stable/classes/class_editorvcsinterface.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorVcsInterface: crate::obj::GodotClass < Base = EditorVcsInterface > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn initialize(&mut self, project_path: GString,) -> bool;
        fn set_credentials(&mut self, username: GString, password: GString, ssh_public_key_path: GString, ssh_private_key_path: GString, ssh_passphrase: GString,);
        fn get_modified_files_data(&mut self,) -> Array < VarDictionary >;
        fn stage_file(&mut self, file_path: GString,);
        fn unstage_file(&mut self, file_path: GString,);
        fn discard_file(&mut self, file_path: GString,);
        fn commit(&mut self, msg: GString,);
        fn get_diff(&mut self, identifier: GString, area: i32,) -> Array < VarDictionary >;
        fn shut_down(&mut self,) -> bool;
        fn get_vcs_name(&mut self,) -> GString;
        fn get_previous_commits(&mut self, max_commits: i32,) -> Array < VarDictionary >;
        fn get_branch_list(&mut self,) -> Array < GString >;
        fn get_remotes(&mut self,) -> Array < GString >;
        fn create_branch(&mut self, branch_name: GString,);
        fn remove_branch(&mut self, branch_name: GString,);
        fn create_remote(&mut self, remote_name: GString, remote_url: GString,);
        fn remove_remote(&mut self, remote_name: GString,);
        fn get_current_branch_name(&mut self,) -> GString;
        fn checkout_branch(&mut self, branch_name: GString,) -> bool;
        fn pull(&mut self, remote: GString,);
        fn push(&mut self, remote: GString, force: bool,);
        fn fetch(&mut self, remote: GString,);
        fn get_line_diff(&mut self, file_path: GString, text: GString,) -> Array < VarDictionary >;
        
    }
    impl EditorVcsInterface {
        pub fn create_diff_line(&mut self, new_line_no: i32, old_line_no: i32, content: impl AsArg < GString >, status: impl AsArg < GString >,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (i32, i32, CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (new_line_no, old_line_no, content.into_arg(), status.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorVcsInterface", "create_diff_line", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_diff_hunk(&mut self, old_start: i32, new_start: i32, old_lines: i32, new_lines: i32,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = (i32, i32, i32, i32,);
            let args = (old_start, new_start, old_lines, new_lines,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorVcsInterface", "create_diff_hunk", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_diff_file(&mut self, new_file: impl AsArg < GString >, old_file: impl AsArg < GString >,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (new_file.into_arg(), old_file.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(420usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorVcsInterface", "create_diff_file", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_commit(&mut self, msg: impl AsArg < GString >, author: impl AsArg < GString >, id: impl AsArg < GString >, unix_timestamp: i64, offset_minutes: i64,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, CowArg < 'a2, GString >, i64, i64,);
            let args = (msg.into_arg(), author.into_arg(), id.into_arg(), unix_timestamp, offset_minutes,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(421usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorVcsInterface", "create_commit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn create_status_file(&mut self, file_path: impl AsArg < GString >, change_type: crate::classes::editor_vcs_interface::ChangeType, area: crate::classes::editor_vcs_interface::TreeArea,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::editor_vcs_interface::ChangeType, crate::classes::editor_vcs_interface::TreeArea,);
            let args = (file_path.into_arg(), change_type, area,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(422usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorVcsInterface", "create_status_file", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_diff_hunks_into_diff_file(&mut self, diff_file: &VarDictionary, diff_hunks: &Array < VarDictionary >,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, VarDictionary >, RefArg < 'a1, Array < VarDictionary > >,);
            let args = (RefArg::new(diff_file), RefArg::new(diff_hunks),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(423usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorVcsInterface", "add_diff_hunks_into_diff_file", Some(self.__validated_obj()), args,)
            }
        }
        pub fn add_line_diffs_into_diff_hunk(&mut self, diff_hunk: &VarDictionary, line_diffs: &Array < VarDictionary >,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, VarDictionary >, RefArg < 'a1, Array < VarDictionary > >,);
            let args = (RefArg::new(diff_hunk), RefArg::new(line_diffs),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(424usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorVcsInterface", "add_line_diffs_into_diff_hunk", Some(self.__validated_obj()), args,)
            }
        }
        pub fn popup_error(&mut self, msg: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (msg.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(425usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorVcsInterface", "popup_error", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EditorVcsInterface {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorVCSInterface"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorVcsInterface {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorVcsInterface {
        
    }
    impl crate::obj::cap::GodotDefault for EditorVcsInterface {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorVcsInterface {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorVcsInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorVcsInterface`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorVcsInterface__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorVcsInterface > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ChangeType {
    ord: i32
}
impl ChangeType {
    #[doc(alias = "CHANGE_TYPE_NEW")]
    #[doc = "Godot enumerator name: `CHANGE_TYPE_NEW`"]
    pub const NEW: ChangeType = ChangeType {
        ord: 0i32
    };
    #[doc(alias = "CHANGE_TYPE_MODIFIED")]
    #[doc = "Godot enumerator name: `CHANGE_TYPE_MODIFIED`"]
    pub const MODIFIED: ChangeType = ChangeType {
        ord: 1i32
    };
    #[doc(alias = "CHANGE_TYPE_RENAMED")]
    #[doc = "Godot enumerator name: `CHANGE_TYPE_RENAMED`"]
    pub const RENAMED: ChangeType = ChangeType {
        ord: 2i32
    };
    #[doc(alias = "CHANGE_TYPE_DELETED")]
    #[doc = "Godot enumerator name: `CHANGE_TYPE_DELETED`"]
    pub const DELETED: ChangeType = ChangeType {
        ord: 3i32
    };
    #[doc(alias = "CHANGE_TYPE_TYPECHANGE")]
    #[doc = "Godot enumerator name: `CHANGE_TYPE_TYPECHANGE`"]
    pub const TYPECHANGE: ChangeType = ChangeType {
        ord: 4i32
    };
    #[doc(alias = "CHANGE_TYPE_UNMERGED")]
    #[doc = "Godot enumerator name: `CHANGE_TYPE_UNMERGED`"]
    pub const UNMERGED: ChangeType = ChangeType {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for ChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ChangeType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ChangeType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::NEW => "NEW", Self::MODIFIED => "MODIFIED", Self::RENAMED => "RENAMED", Self::DELETED => "DELETED", Self::TYPECHANGE => "TYPECHANGE", Self::UNMERGED => "UNMERGED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ChangeType::NEW, ChangeType::MODIFIED, ChangeType::RENAMED, ChangeType::DELETED, ChangeType::TYPECHANGE, ChangeType::UNMERGED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ChangeType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEW", "CHANGE_TYPE_NEW", ChangeType::NEW), crate::meta::inspect::EnumConstant::new("MODIFIED", "CHANGE_TYPE_MODIFIED", ChangeType::MODIFIED), crate::meta::inspect::EnumConstant::new("RENAMED", "CHANGE_TYPE_RENAMED", ChangeType::RENAMED), crate::meta::inspect::EnumConstant::new("DELETED", "CHANGE_TYPE_DELETED", ChangeType::DELETED), crate::meta::inspect::EnumConstant::new("TYPECHANGE", "CHANGE_TYPE_TYPECHANGE", ChangeType::TYPECHANGE), crate::meta::inspect::EnumConstant::new("UNMERGED", "CHANGE_TYPE_UNMERGED", ChangeType::UNMERGED)]
        }
    }
}
impl crate::meta::GodotConvert for ChangeType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ChangeType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ChangeType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TreeArea {
    ord: i32
}
impl TreeArea {
    #[doc(alias = "TREE_AREA_COMMIT")]
    #[doc = "Godot enumerator name: `TREE_AREA_COMMIT`"]
    pub const COMMIT: TreeArea = TreeArea {
        ord: 0i32
    };
    #[doc(alias = "TREE_AREA_STAGED")]
    #[doc = "Godot enumerator name: `TREE_AREA_STAGED`"]
    pub const STAGED: TreeArea = TreeArea {
        ord: 1i32
    };
    #[doc(alias = "TREE_AREA_UNSTAGED")]
    #[doc = "Godot enumerator name: `TREE_AREA_UNSTAGED`"]
    pub const UNSTAGED: TreeArea = TreeArea {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TreeArea {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TreeArea") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TreeArea {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
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
            Self::COMMIT => "COMMIT", Self::STAGED => "STAGED", Self::UNSTAGED => "UNSTAGED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TreeArea::COMMIT, TreeArea::STAGED, TreeArea::UNSTAGED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TreeArea >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("COMMIT", "TREE_AREA_COMMIT", TreeArea::COMMIT), crate::meta::inspect::EnumConstant::new("STAGED", "TREE_AREA_STAGED", TreeArea::STAGED), crate::meta::inspect::EnumConstant::new("UNSTAGED", "TREE_AREA_UNSTAGED", TreeArea::UNSTAGED)]
        }
    }
}
impl crate::meta::GodotConvert for TreeArea {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TreeArea {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TreeArea {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorVcsInterface;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EditorVcsInterface {
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