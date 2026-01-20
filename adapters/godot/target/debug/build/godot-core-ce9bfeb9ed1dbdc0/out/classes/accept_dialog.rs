#![doc = "Sidecar module for class [`AcceptDialog`][crate::classes::AcceptDialog].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AcceptDialog` enums](https://docs.godotengine.org/en/stable/classes/class_acceptdialog.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AcceptDialog.`\n\nInherits [`Window`][crate::classes::Window].\n\nRelated symbols:\n\n* [`accept_dialog`][crate::classes::accept_dialog]: sidecar module with related enum/flag types\n* [`IAcceptDialog`][crate::classes::IAcceptDialog]: virtual methods\n* [`SignalsOfAcceptDialog`][crate::classes::accept_dialog::SignalsOfAcceptDialog]: signal collection\n\n\nSee also [Godot docs for `AcceptDialog`](https://docs.godotengine.org/en/stable/classes/class_acceptdialog.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`AcceptDialog::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AcceptDialog {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AcceptDialog`][crate::classes::AcceptDialog].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IWindow`][crate::classes::IWindow] > ~~`IViewport`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `AcceptDialog` methods](https://docs.godotengine.org/en/stable/classes/class_acceptdialog.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAcceptDialog: crate::obj::GodotClass < Base = AcceptDialog > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AcceptDialog {
        pub fn get_ok_button(&mut self,) -> Option < Gd < crate::classes::Button > > {
            type CallRet = Option < Gd < crate::classes::Button > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(87usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AcceptDialog", "get_ok_button", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_label(&mut self,) -> Option < Gd < crate::classes::Label > > {
            type CallRet = Option < Gd < crate::classes::Label > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(88usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AcceptDialog", "get_label", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_hide_on_ok(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(89usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AcceptDialog", "set_hide_on_ok", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_hide_on_ok(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(90usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AcceptDialog", "get_hide_on_ok", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_close_on_escape(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(91usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AcceptDialog", "set_close_on_escape", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_close_on_escape(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(92usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AcceptDialog", "get_close_on_escape", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_button_full(&mut self, text: CowArg < GString >, right: bool, action: CowArg < GString >,) -> Option < Gd < crate::classes::Button > > {
            type CallRet = Option < Gd < crate::classes::Button > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, bool, CowArg < 'a1, GString >,);
            let args = (text, right, action,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(93usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AcceptDialog", "add_button", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_button_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_button(&mut self, text: impl AsArg < GString >,) -> Option < Gd < crate::classes::Button > > {
            self.add_button_ex(text,) . done()
        }
        #[inline]
        pub fn add_button_ex < 'a > (&'a mut self, text: impl AsArg < GString > + 'a,) -> ExAddButton < 'a > {
            ExAddButton::new(self, text,)
        }
        pub fn add_cancel_button(&mut self, name: impl AsArg < GString >,) -> Option < Gd < crate::classes::Button > > {
            type CallRet = Option < Gd < crate::classes::Button > >;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(94usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AcceptDialog", "add_cancel_button", Some(self.__validated_obj()), args,)
            }
        }
        pub fn remove_button(&mut self, button: impl AsArg < Option < Gd < crate::classes::Button >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Button > > >,);
            let args = (button.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(95usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AcceptDialog", "remove_button", Some(self.__validated_obj()), args,)
            }
        }
        pub fn register_text_enter(&mut self, line_edit: impl AsArg < Option < Gd < crate::classes::LineEdit >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::LineEdit > > >,);
            let args = (line_edit.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(96usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AcceptDialog", "register_text_enter", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_text(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(97usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AcceptDialog", "set_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(98usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AcceptDialog", "get_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_autowrap(&mut self, autowrap: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (autowrap,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(99usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AcceptDialog", "set_autowrap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn has_autowrap(&mut self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(100usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AcceptDialog", "has_autowrap", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_ok_button_text(&mut self, text: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (text.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(101usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AcceptDialog", "set_ok_button_text", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_ok_button_text(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(102usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AcceptDialog", "get_ok_button_text", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AcceptDialog {
        type Base = crate::classes::Window;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AcceptDialog"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AcceptDialog {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Window > for AcceptDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Viewport > for AcceptDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for AcceptDialog {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AcceptDialog {
        
    }
    impl crate::obj::cap::GodotDefault for AcceptDialog {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AcceptDialog {
        type Target = crate::classes::Window;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AcceptDialog {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AcceptDialog`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AcceptDialog__ensure_class_exists {
        ($Class: ident) => {
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
#[doc = "Default-param extender for [`AcceptDialog::add_button_ex`][super::AcceptDialog::add_button_ex]."]
#[must_use]
pub struct ExAddButton < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::AcceptDialog, text: CowArg < 'a, GString >, right: bool, action: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddButton < 'a > {
    fn new(surround_object: &'a mut re_export::AcceptDialog, text: impl AsArg < GString > + 'a,) -> Self {
        let right = false;
        let action = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, text: text.into_arg(), right: right, action: CowArg::Owned(action),
        }
    }
    #[inline]
    pub fn right(self, right: bool) -> Self {
        Self {
            right: right, .. self
        }
    }
    #[inline]
    pub fn action(self, action: impl AsArg < GString > + 'a) -> Self {
        Self {
            action: action.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::Button > > {
        let Self {
            _phantom, surround_object, text, right, action,
        }
        = self;
        re_export::AcceptDialog::add_button_full(surround_object, text, right, action,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AcceptDialog;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`AcceptDialog`][crate::classes::AcceptDialog] class."]
    pub struct SignalsOfAcceptDialog < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfAcceptDialog < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn confirmed(&mut self) -> SigConfirmed < 'c, C > {
            SigConfirmed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "confirmed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn canceled(&mut self) -> SigCanceled < 'c, C > {
            SigCanceled {
                typed: TypedSignal::extract(&mut self.__internal_obj, "canceled")
            }
        }
        #[doc = "Signature: `(action: StringName)`"]
        pub fn custom_action(&mut self) -> SigCustomAction < 'c, C > {
            SigCustomAction {
                typed: TypedSignal::extract(&mut self.__internal_obj, "custom_action")
            }
        }
    }
    type TypedSigConfirmed < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigConfirmed < 'c, C: WithSignals > {
        typed: TypedSigConfirmed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigConfirmed < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigConfirmed < 'c, C > {
        type Target = TypedSigConfirmed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigConfirmed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCanceled < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigCanceled < 'c, C: WithSignals > {
        typed: TypedSigCanceled < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCanceled < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCanceled < 'c, C > {
        type Target = TypedSigCanceled < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCanceled < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigCustomAction < 'c, C > = TypedSignal < 'c, C, (StringName,) >;
    pub struct SigCustomAction < 'c, C: WithSignals > {
        typed: TypedSigCustomAction < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigCustomAction < 'c, C > {
        pub fn emit(&mut self, action: StringName,) {
            self.typed.emit_tuple((action,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigCustomAction < 'c, C > {
        type Target = TypedSigCustomAction < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigCustomAction < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for AcceptDialog {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAcceptDialog < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfAcceptDialog < 'c, C > {
        type Target = < < AcceptDialog as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = AcceptDialog;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfAcceptDialog < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = AcceptDialog;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}