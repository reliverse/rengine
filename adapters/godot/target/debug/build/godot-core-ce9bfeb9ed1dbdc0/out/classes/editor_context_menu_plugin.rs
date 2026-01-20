#![doc = "Sidecar module for class [`EditorContextMenuPlugin`][crate::classes::EditorContextMenuPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorContextMenuPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorcontextmenuplugin.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorContextMenuPlugin.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`editor_context_menu_plugin`][crate::classes::editor_context_menu_plugin]: sidecar module with related enum/flag types\n* [`IEditorContextMenuPlugin`][crate::classes::IEditorContextMenuPlugin]: virtual methods\n\n\nSee also [Godot docs for `EditorContextMenuPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorcontextmenuplugin.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`EditorContextMenuPlugin::new_gd()`][crate::obj::NewGd::new_gd]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorContextMenuPlugin {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorContextMenuPlugin`][crate::classes::EditorContextMenuPlugin].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `EditorContextMenuPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorcontextmenuplugin.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorContextMenuPlugin: crate::obj::GodotClass < Base = EditorContextMenuPlugin > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn popup_menu(&mut self, paths: PackedStringArray,) {
            unimplemented !()
        }
    }
    impl EditorContextMenuPlugin {
        pub fn add_menu_shortcut(&mut self, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> >, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Shortcut > > >, RefArg < 'a1, Callable >,);
            let args = (shortcut.into_arg(), RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(2usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorContextMenuPlugin", "add_menu_shortcut", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_context_menu_item_full(&mut self, name: CowArg < GString >, callback: RefArg < Callable >, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, RefArg < 'a1, Callable >, CowArg < 'a2, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (name, callback, icon,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(3usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorContextMenuPlugin", "add_context_menu_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_context_menu_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_context_menu_item(&mut self, name: impl AsArg < GString >, callback: &Callable,) {
            self.add_context_menu_item_ex(name, callback,) . done()
        }
        #[inline]
        pub fn add_context_menu_item_ex < 'a > (&'a mut self, name: impl AsArg < GString > + 'a, callback: &'a Callable,) -> ExAddContextMenuItem < 'a > {
            ExAddContextMenuItem::new(self, name, callback,)
        }
        pub(crate) fn add_context_menu_item_from_shortcut_full(&mut self, name: CowArg < GString >, shortcut: CowArg < Option < Gd < crate::classes::Shortcut > > >, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::Shortcut > > >, CowArg < 'a2, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (name, shortcut, icon,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(4usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorContextMenuPlugin", "add_context_menu_item_from_shortcut", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_context_menu_item_from_shortcut_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_context_menu_item_from_shortcut(&mut self, name: impl AsArg < GString >, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> >,) {
            self.add_context_menu_item_from_shortcut_ex(name, shortcut,) . done()
        }
        #[inline]
        pub fn add_context_menu_item_from_shortcut_ex < 'a > (&'a mut self, name: impl AsArg < GString > + 'a, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'a,) -> ExAddContextMenuItemFromShortcut < 'a > {
            ExAddContextMenuItemFromShortcut::new(self, name, shortcut,)
        }
        pub(crate) fn add_context_submenu_item_full(&mut self, name: CowArg < GString >, menu: CowArg < Option < Gd < crate::classes::PopupMenu > > >, icon: CowArg < Option < Gd < crate::classes::Texture2D > > >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::PopupMenu > > >, CowArg < 'a2, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (name, menu, icon,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(5usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorContextMenuPlugin", "add_context_submenu_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_context_submenu_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_context_submenu_item(&mut self, name: impl AsArg < GString >, menu: impl AsArg < Option < Gd < crate::classes::PopupMenu >> >,) {
            self.add_context_submenu_item_ex(name, menu,) . done()
        }
        #[inline]
        pub fn add_context_submenu_item_ex < 'a > (&'a mut self, name: impl AsArg < GString > + 'a, menu: impl AsArg < Option < Gd < crate::classes::PopupMenu >> > + 'a,) -> ExAddContextSubmenuItem < 'a > {
            ExAddContextSubmenuItem::new(self, name, menu,)
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
    impl crate::obj::GodotClass for EditorContextMenuPlugin {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorContextMenuPlugin"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorContextMenuPlugin {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for EditorContextMenuPlugin {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorContextMenuPlugin {
        
    }
    impl crate::obj::cap::GodotDefault for EditorContextMenuPlugin {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorContextMenuPlugin {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorContextMenuPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorContextMenuPlugin`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorContextMenuPlugin__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorContextMenuPlugin > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorContextMenuPlugin::add_context_menu_item_ex`][super::EditorContextMenuPlugin::add_context_menu_item_ex]."]
#[must_use]
pub struct ExAddContextMenuItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorContextMenuPlugin, name: CowArg < 'a, GString >, callback: CowArg < 'a, Callable >, icon: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddContextMenuItem < 'a > {
    fn new(surround_object: &'a mut re_export::EditorContextMenuPlugin, name: impl AsArg < GString > + 'a, callback: &'a Callable,) -> Self {
        let icon = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), callback: CowArg::Borrowed(callback), icon: icon.into_arg(),
        }
    }
    #[inline]
    pub fn icon(self, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a) -> Self {
        Self {
            icon: icon.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, callback, icon,
        }
        = self;
        re_export::EditorContextMenuPlugin::add_context_menu_item_full(surround_object, name, callback.cow_as_arg(), icon,)
    }
}
#[doc = "Default-param extender for [`EditorContextMenuPlugin::add_context_menu_item_from_shortcut_ex`][super::EditorContextMenuPlugin::add_context_menu_item_from_shortcut_ex]."]
#[must_use]
pub struct ExAddContextMenuItemFromShortcut < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorContextMenuPlugin, name: CowArg < 'a, GString >, shortcut: CowArg < 'a, Option < Gd < crate::classes::Shortcut > > >, icon: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddContextMenuItemFromShortcut < 'a > {
    fn new(surround_object: &'a mut re_export::EditorContextMenuPlugin, name: impl AsArg < GString > + 'a, shortcut: impl AsArg < Option < Gd < crate::classes::Shortcut >> > + 'a,) -> Self {
        let icon = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), shortcut: shortcut.into_arg(), icon: icon.into_arg(),
        }
    }
    #[inline]
    pub fn icon(self, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a) -> Self {
        Self {
            icon: icon.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, shortcut, icon,
        }
        = self;
        re_export::EditorContextMenuPlugin::add_context_menu_item_from_shortcut_full(surround_object, name, shortcut, icon,)
    }
}
#[doc = "Default-param extender for [`EditorContextMenuPlugin::add_context_submenu_item_ex`][super::EditorContextMenuPlugin::add_context_submenu_item_ex]."]
#[must_use]
pub struct ExAddContextSubmenuItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorContextMenuPlugin, name: CowArg < 'a, GString >, menu: CowArg < 'a, Option < Gd < crate::classes::PopupMenu > > >, icon: CowArg < 'a, Option < Gd < crate::classes::Texture2D > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddContextSubmenuItem < 'a > {
    fn new(surround_object: &'a mut re_export::EditorContextMenuPlugin, name: impl AsArg < GString > + 'a, menu: impl AsArg < Option < Gd < crate::classes::PopupMenu >> > + 'a,) -> Self {
        let icon = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, name: name.into_arg(), menu: menu.into_arg(), icon: icon.into_arg(),
        }
    }
    #[inline]
    pub fn icon(self, icon: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'a) -> Self {
        Self {
            icon: icon.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, name, menu, icon,
        }
        = self;
        re_export::EditorContextMenuPlugin::add_context_submenu_item_full(surround_object, name, menu, icon,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ContextMenuSlot {
    ord: i32
}
impl ContextMenuSlot {
    #[doc(alias = "CONTEXT_SLOT_SCENE_TREE")]
    #[doc = "Godot enumerator name: `CONTEXT_SLOT_SCENE_TREE`"]
    pub const SCENE_TREE: ContextMenuSlot = ContextMenuSlot {
        ord: 0i32
    };
    #[doc(alias = "CONTEXT_SLOT_FILESYSTEM")]
    #[doc = "Godot enumerator name: `CONTEXT_SLOT_FILESYSTEM`"]
    pub const FILESYSTEM: ContextMenuSlot = ContextMenuSlot {
        ord: 1i32
    };
    #[doc(alias = "CONTEXT_SLOT_SCRIPT_EDITOR")]
    #[doc = "Godot enumerator name: `CONTEXT_SLOT_SCRIPT_EDITOR`"]
    pub const SCRIPT_EDITOR: ContextMenuSlot = ContextMenuSlot {
        ord: 2i32
    };
    #[doc(alias = "CONTEXT_SLOT_FILESYSTEM_CREATE")]
    #[doc = "Godot enumerator name: `CONTEXT_SLOT_FILESYSTEM_CREATE`"]
    pub const FILESYSTEM_CREATE: ContextMenuSlot = ContextMenuSlot {
        ord: 3i32
    };
    #[doc(alias = "CONTEXT_SLOT_SCRIPT_EDITOR_CODE")]
    #[doc = "Godot enumerator name: `CONTEXT_SLOT_SCRIPT_EDITOR_CODE`"]
    pub const SCRIPT_EDITOR_CODE: ContextMenuSlot = ContextMenuSlot {
        ord: 4i32
    };
    #[doc(alias = "CONTEXT_SLOT_SCENE_TABS")]
    #[doc = "Godot enumerator name: `CONTEXT_SLOT_SCENE_TABS`"]
    pub const SCENE_TABS: ContextMenuSlot = ContextMenuSlot {
        ord: 5i32
    };
    #[doc(alias = "CONTEXT_SLOT_2D_EDITOR")]
    #[doc = "Godot enumerator name: `CONTEXT_SLOT_2D_EDITOR`"]
    pub const SLOT_2D_EDITOR: ContextMenuSlot = ContextMenuSlot {
        ord: 6i32
    };
    
}
impl std::fmt::Debug for ContextMenuSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ContextMenuSlot") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ContextMenuSlot {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 => Some(Self {
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
            Self::SCENE_TREE => "SCENE_TREE", Self::FILESYSTEM => "FILESYSTEM", Self::SCRIPT_EDITOR => "SCRIPT_EDITOR", Self::FILESYSTEM_CREATE => "FILESYSTEM_CREATE", Self::SCRIPT_EDITOR_CODE => "SCRIPT_EDITOR_CODE", Self::SCENE_TABS => "SCENE_TABS", Self::SLOT_2D_EDITOR => "SLOT_2D_EDITOR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ContextMenuSlot::SCENE_TREE, ContextMenuSlot::FILESYSTEM, ContextMenuSlot::SCRIPT_EDITOR, ContextMenuSlot::FILESYSTEM_CREATE, ContextMenuSlot::SCRIPT_EDITOR_CODE, ContextMenuSlot::SCENE_TABS, ContextMenuSlot::SLOT_2D_EDITOR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ContextMenuSlot >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SCENE_TREE", "CONTEXT_SLOT_SCENE_TREE", ContextMenuSlot::SCENE_TREE), crate::meta::inspect::EnumConstant::new("FILESYSTEM", "CONTEXT_SLOT_FILESYSTEM", ContextMenuSlot::FILESYSTEM), crate::meta::inspect::EnumConstant::new("SCRIPT_EDITOR", "CONTEXT_SLOT_SCRIPT_EDITOR", ContextMenuSlot::SCRIPT_EDITOR), crate::meta::inspect::EnumConstant::new("FILESYSTEM_CREATE", "CONTEXT_SLOT_FILESYSTEM_CREATE", ContextMenuSlot::FILESYSTEM_CREATE), crate::meta::inspect::EnumConstant::new("SCRIPT_EDITOR_CODE", "CONTEXT_SLOT_SCRIPT_EDITOR_CODE", ContextMenuSlot::SCRIPT_EDITOR_CODE), crate::meta::inspect::EnumConstant::new("SCENE_TABS", "CONTEXT_SLOT_SCENE_TABS", ContextMenuSlot::SCENE_TABS), crate::meta::inspect::EnumConstant::new("SLOT_2D_EDITOR", "CONTEXT_SLOT_2D_EDITOR", ContextMenuSlot::SLOT_2D_EDITOR)]
        }
    }
}
impl crate::meta::GodotConvert for ContextMenuSlot {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ContextMenuSlot {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ContextMenuSlot {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorContextMenuPlugin;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for EditorContextMenuPlugin {
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