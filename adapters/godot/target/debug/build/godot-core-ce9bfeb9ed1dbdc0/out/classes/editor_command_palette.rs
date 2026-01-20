#![doc = "Sidecar module for class [`EditorCommandPalette`][crate::classes::EditorCommandPalette].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorCommandPalette` enums](https://docs.godotengine.org/en/stable/classes/class_editorcommandpalette.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorCommandPalette.`\n\nInherits [`ConfirmationDialog`][crate::classes::ConfirmationDialog].\n\nRelated symbols:\n\n* [`editor_command_palette`][crate::classes::editor_command_palette]: sidecar module with related enum/flag types\n* [`IEditorCommandPalette`][crate::classes::IEditorCommandPalette]: virtual methods\n\n\nSee also [Godot docs for `EditorCommandPalette`](https://docs.godotengine.org/en/stable/classes/class_editorcommandpalette.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`EditorCommandPalette::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorCommandPalette {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorCommandPalette`][crate::classes::EditorCommandPalette].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IConfirmationDialog`][crate::classes::IConfirmationDialog] > [`IAcceptDialog`][crate::classes::IAcceptDialog] > [`IWindow`][crate::classes::IWindow] > ~~`IViewport`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `EditorCommandPalette` methods](https://docs.godotengine.org/en/stable/classes/class_editorcommandpalette.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorCommandPalette: crate::obj::GodotClass < Base = EditorCommandPalette > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: WindowNotification) {
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
        fn get_contents_minimum_size(&self,) -> Vector2 {
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
    impl EditorCommandPalette {
        pub(crate) fn add_command_full(&mut self, command_name: CowArg < GString >, key_name: CowArg < GString >, binded_callable: RefArg < Callable >, shortcut_text: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, 'a2, 'a3, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >, RefArg < 'a2, Callable >, CowArg < 'a3, GString >,);
            let args = (command_name, key_name, binded_callable, shortcut_text,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(0usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorCommandPalette", "add_command", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_command_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_command(&mut self, command_name: impl AsArg < GString >, key_name: impl AsArg < GString >, binded_callable: &Callable,) {
            self.add_command_ex(command_name, key_name, binded_callable,) . done()
        }
        #[inline]
        pub fn add_command_ex < 'a > (&'a mut self, command_name: impl AsArg < GString > + 'a, key_name: impl AsArg < GString > + 'a, binded_callable: &'a Callable,) -> ExAddCommand < 'a > {
            ExAddCommand::new(self, command_name, key_name, binded_callable,)
        }
        pub fn remove_command(&mut self, key_name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (key_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(1usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorCommandPalette", "remove_command", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for EditorCommandPalette {
        type Base = crate::classes::ConfirmationDialog;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorCommandPalette"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorCommandPalette {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::ConfirmationDialog > for EditorCommandPalette {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AcceptDialog > for EditorCommandPalette {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Window > for EditorCommandPalette {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Viewport > for EditorCommandPalette {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for EditorCommandPalette {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorCommandPalette {
        
    }
    impl crate::obj::cap::GodotDefault for EditorCommandPalette {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorCommandPalette {
        type Target = crate::classes::ConfirmationDialog;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorCommandPalette {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorCommandPalette`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorCommandPalette__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorCommandPalette > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::ConfirmationDialog > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AcceptDialog > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Window > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Viewport > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorCommandPalette::add_command_ex`][super::EditorCommandPalette::add_command_ex]."]
#[must_use]
pub struct ExAddCommand < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::EditorCommandPalette, command_name: CowArg < 'a, GString >, key_name: CowArg < 'a, GString >, binded_callable: CowArg < 'a, Callable >, shortcut_text: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddCommand < 'a > {
    fn new(surround_object: &'a mut re_export::EditorCommandPalette, command_name: impl AsArg < GString > + 'a, key_name: impl AsArg < GString > + 'a, binded_callable: &'a Callable,) -> Self {
        let shortcut_text = GString::from("None");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, command_name: command_name.into_arg(), key_name: key_name.into_arg(), binded_callable: CowArg::Borrowed(binded_callable), shortcut_text: CowArg::Owned(shortcut_text),
        }
    }
    #[inline]
    pub fn shortcut_text(self, shortcut_text: impl AsArg < GString > + 'a) -> Self {
        Self {
            shortcut_text: shortcut_text.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, command_name, key_name, binded_callable, shortcut_text,
        }
        = self;
        re_export::EditorCommandPalette::add_command_full(surround_object, command_name, key_name, binded_callable.cow_as_arg(), shortcut_text,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorCommandPalette;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::accept_dialog::SignalsOfAcceptDialog;
    impl WithSignals for EditorCommandPalette {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAcceptDialog < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}