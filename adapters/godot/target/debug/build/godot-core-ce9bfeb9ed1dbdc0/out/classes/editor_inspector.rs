#![doc = "Sidecar module for class [`EditorInspector`][crate::classes::EditorInspector].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorInspector` enums](https://docs.godotengine.org/en/stable/classes/class_editorinspector.html#enumerations).\n\n"]
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
    #[doc = "Godot class `EditorInspector.`\n\nInherits [`ScrollContainer`][crate::classes::ScrollContainer].\n\nRelated symbols:\n\n* [`editor_inspector`][crate::classes::editor_inspector]: sidecar module with related enum/flag types\n* [`IEditorInspector`][crate::classes::IEditorInspector]: virtual methods\n* [`SignalsOfEditorInspector`][crate::classes::editor_inspector::SignalsOfEditorInspector]: signal collection\n\n\nSee also [Godot docs for `EditorInspector`](https://docs.godotengine.org/en/stable/classes/class_editorinspector.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`EditorInspector::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EditorInspector {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`EditorInspector`][crate::classes::EditorInspector].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IScrollContainer`][crate::classes::IScrollContainer] > [`IContainer`][crate::classes::IContainer] > [`IControl`][crate::classes::IControl] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `EditorInspector` methods](https://docs.godotengine.org/en/stable/classes/class_editorinspector.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IEditorInspector: crate::obj::GodotClass < Base = EditorInspector > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: ContainerNotification) {
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
        fn get_allowed_size_flags_horizontal(&self,) -> PackedInt32Array {
            unimplemented !()
        }
        fn get_allowed_size_flags_vertical(&self,) -> PackedInt32Array {
            unimplemented !()
        }
        fn has_point(&self, point: Vector2,) -> bool {
            unimplemented !()
        }
        fn structured_text_parser(&self, args: VarArray, text: GString,) -> Array < Vector3i > {
            unimplemented !()
        }
        fn get_minimum_size(&self,) -> Vector2 {
            unimplemented !()
        }
        fn get_tooltip(&self, at_position: Vector2,) -> GString {
            unimplemented !()
        }
        fn get_drag_data(&mut self, at_position: Vector2,) -> Variant {
            unimplemented !()
        }
        fn can_drop_data(&self, at_position: Vector2, data: Variant,) -> bool {
            unimplemented !()
        }
        fn drop_data(&mut self, at_position: Vector2, data: Variant,) {
            unimplemented !()
        }
        fn make_custom_tooltip(&self, for_text: GString,) -> Option < Gd < crate::classes::Object > > {
            unimplemented !()
        }
        fn accessibility_get_contextual_info(&self,) -> GString {
            unimplemented !()
        }
        fn get_accessibility_container_name(&self, node: Option < Gd < crate::classes::Node > >,) -> GString {
            unimplemented !()
        }
        fn gui_input(&mut self, event: Gd < crate::classes::InputEvent >,) {
            unimplemented !()
        }
        fn draw(&mut self,) {
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
    impl EditorInspector {
        pub fn edit(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >,);
            let args = (object.into_arg(),);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(166usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInspector", "edit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_selected_path(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(167usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInspector", "get_selected_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_edited_object(&mut self,) -> Option < Gd < crate::classes::Object > > {
            type CallRet = Option < Gd < crate::classes::Object > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(168usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInspector", "get_edited_object", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn instantiate_property_editor_full(object: CowArg < Option < Gd < crate::classes::Object > > >, type_: VariantType, path: CowArg < GString >, hint: crate::global::PropertyHint, hint_text: CowArg < GString >, usage: u32, wide: bool,) -> Option < Gd < crate::classes::EditorProperty > > {
            type CallRet = Option < Gd < crate::classes::EditorProperty > >;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, VariantType, CowArg < 'a1, GString >, crate::global::PropertyHint, CowArg < 'a2, GString >, u32, bool,);
            let args = (object, type_, path, hint, hint_text, usage, wide,);
            unsafe {
                let method_bind = sys::class_editor_api() . fptr_by_index(169usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "EditorInspector", "instantiate_property_editor", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::instantiate_property_editor_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn instantiate_property_editor(object: impl AsArg < Option < Gd < crate::classes::Object >> >, type_: VariantType, path: impl AsArg < GString >, hint: crate::global::PropertyHint, hint_text: impl AsArg < GString >, usage: u32,) -> Option < Gd < crate::classes::EditorProperty > > {
            Self::instantiate_property_editor_ex(object, type_, path, hint, hint_text, usage,) . done()
        }
        #[inline]
        pub fn instantiate_property_editor_ex < 'a > (object: impl AsArg < Option < Gd < crate::classes::Object >> > + 'a, type_: VariantType, path: impl AsArg < GString > + 'a, hint: crate::global::PropertyHint, hint_text: impl AsArg < GString > + 'a, usage: u32,) -> ExInstantiatePropertyEditor < 'a > {
            ExInstantiatePropertyEditor::new(object, type_, path, hint, hint_text, usage,)
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
    impl crate::obj::GodotClass for EditorInspector {
        type Base = crate::classes::ScrollContainer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("EditorInspector"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Editor;
        
    }
    unsafe impl crate::obj::Bounds for EditorInspector {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::ScrollContainer > for EditorInspector {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Container > for EditorInspector {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Control > for EditorInspector {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for EditorInspector {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for EditorInspector {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for EditorInspector {
        
    }
    impl crate::obj::cap::GodotDefault for EditorInspector {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for EditorInspector {
        type Target = crate::classes::ScrollContainer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for EditorInspector {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`EditorInspector`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_EditorInspector__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::EditorInspector > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::ScrollContainer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Container > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Control > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::CanvasItem > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`EditorInspector::instantiate_property_editor_ex`][super::EditorInspector::instantiate_property_editor_ex]."]
#[must_use]
pub struct ExInstantiatePropertyEditor < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, object: CowArg < 'a, Option < Gd < crate::classes::Object > > >, type_: VariantType, path: CowArg < 'a, GString >, hint: crate::global::PropertyHint, hint_text: CowArg < 'a, GString >, usage: u32, wide: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExInstantiatePropertyEditor < 'a > {
    fn new(object: impl AsArg < Option < Gd < crate::classes::Object >> > + 'a, type_: VariantType, path: impl AsArg < GString > + 'a, hint: crate::global::PropertyHint, hint_text: impl AsArg < GString > + 'a, usage: u32,) -> Self {
        let wide = false;
        Self {
            _phantom: std::marker::PhantomData, object: object.into_arg(), type_: type_, path: path.into_arg(), hint: hint, hint_text: hint_text.into_arg(), usage: usage, wide: wide,
        }
    }
    #[inline]
    pub fn wide(self, wide: bool) -> Self {
        Self {
            wide: wide, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::EditorProperty > > {
        let Self {
            _phantom, object, type_, path, hint, hint_text, usage, wide,
        }
        = self;
        re_export::EditorInspector::instantiate_property_editor_full(object, type_, path, hint, hint_text, usage, wide,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::EditorInspector;
    use crate::registry::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`EditorInspector`][crate::classes::EditorInspector] class."]
    pub struct SignalsOfEditorInspector < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfEditorInspector < 'c, C > {
        #[doc = "Signature: `(property: GString)`"]
        pub fn property_selected(&mut self) -> SigPropertySelected < 'c, C > {
            SigPropertySelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "property_selected")
            }
        }
        #[doc = "Signature: `(property: GString, value: Variant, advance: bool)`"]
        pub fn property_keyed(&mut self) -> SigPropertyKeyed < 'c, C > {
            SigPropertyKeyed {
                typed: TypedSignal::extract(&mut self.__internal_obj, "property_keyed")
            }
        }
        #[doc = "Signature: `(property: GString)`"]
        pub fn property_deleted(&mut self) -> SigPropertyDeleted < 'c, C > {
            SigPropertyDeleted {
                typed: TypedSignal::extract(&mut self.__internal_obj, "property_deleted")
            }
        }
        #[doc = "Signature: `(resource: Gd<Resource>, path: GString)`"]
        pub fn resource_selected(&mut self) -> SigResourceSelected < 'c, C > {
            SigResourceSelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "resource_selected")
            }
        }
        #[doc = "Signature: `(id: i64)`"]
        pub fn object_id_selected(&mut self) -> SigObjectIdSelected < 'c, C > {
            SigObjectIdSelected {
                typed: TypedSignal::extract(&mut self.__internal_obj, "object_id_selected")
            }
        }
        #[doc = "Signature: `(property: GString)`"]
        pub fn property_edited(&mut self) -> SigPropertyEdited < 'c, C > {
            SigPropertyEdited {
                typed: TypedSignal::extract(&mut self.__internal_obj, "property_edited")
            }
        }
        #[doc = "Signature: `(property: GString, checked: bool)`"]
        pub fn property_toggled(&mut self) -> SigPropertyToggled < 'c, C > {
            SigPropertyToggled {
                typed: TypedSignal::extract(&mut self.__internal_obj, "property_toggled")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn edited_object_changed(&mut self) -> SigEditedObjectChanged < 'c, C > {
            SigEditedObjectChanged {
                typed: TypedSignal::extract(&mut self.__internal_obj, "edited_object_changed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn restart_requested(&mut self) -> SigRestartRequested < 'c, C > {
            SigRestartRequested {
                typed: TypedSignal::extract(&mut self.__internal_obj, "restart_requested")
            }
        }
    }
    type TypedSigPropertySelected < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigPropertySelected < 'c, C: WithSignals > {
        typed: TypedSigPropertySelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPropertySelected < 'c, C > {
        pub fn emit(&mut self, property: GString,) {
            self.typed.emit_tuple((property,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPropertySelected < 'c, C > {
        type Target = TypedSigPropertySelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPropertySelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigPropertyKeyed < 'c, C > = TypedSignal < 'c, C, (GString, Variant, bool,) >;
    pub struct SigPropertyKeyed < 'c, C: WithSignals > {
        typed: TypedSigPropertyKeyed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPropertyKeyed < 'c, C > {
        pub fn emit(&mut self, property: GString, value: Variant, advance: bool,) {
            self.typed.emit_tuple((property, value, advance,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPropertyKeyed < 'c, C > {
        type Target = TypedSigPropertyKeyed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPropertyKeyed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigPropertyDeleted < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigPropertyDeleted < 'c, C: WithSignals > {
        typed: TypedSigPropertyDeleted < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPropertyDeleted < 'c, C > {
        pub fn emit(&mut self, property: GString,) {
            self.typed.emit_tuple((property,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPropertyDeleted < 'c, C > {
        type Target = TypedSigPropertyDeleted < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPropertyDeleted < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigResourceSelected < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::Resource >, GString,) >;
    pub struct SigResourceSelected < 'c, C: WithSignals > {
        typed: TypedSigResourceSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigResourceSelected < 'c, C > {
        pub fn emit(&mut self, resource: Gd < crate::classes::Resource >, path: GString,) {
            self.typed.emit_tuple((resource, path,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigResourceSelected < 'c, C > {
        type Target = TypedSigResourceSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigResourceSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigObjectIdSelected < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigObjectIdSelected < 'c, C: WithSignals > {
        typed: TypedSigObjectIdSelected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigObjectIdSelected < 'c, C > {
        pub fn emit(&mut self, id: i64,) {
            self.typed.emit_tuple((id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigObjectIdSelected < 'c, C > {
        type Target = TypedSigObjectIdSelected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigObjectIdSelected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigPropertyEdited < 'c, C > = TypedSignal < 'c, C, (GString,) >;
    pub struct SigPropertyEdited < 'c, C: WithSignals > {
        typed: TypedSigPropertyEdited < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPropertyEdited < 'c, C > {
        pub fn emit(&mut self, property: GString,) {
            self.typed.emit_tuple((property,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPropertyEdited < 'c, C > {
        type Target = TypedSigPropertyEdited < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPropertyEdited < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigPropertyToggled < 'c, C > = TypedSignal < 'c, C, (GString, bool,) >;
    pub struct SigPropertyToggled < 'c, C: WithSignals > {
        typed: TypedSigPropertyToggled < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPropertyToggled < 'c, C > {
        pub fn emit(&mut self, property: GString, checked: bool,) {
            self.typed.emit_tuple((property, checked,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPropertyToggled < 'c, C > {
        type Target = TypedSigPropertyToggled < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPropertyToggled < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigEditedObjectChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigEditedObjectChanged < 'c, C: WithSignals > {
        typed: TypedSigEditedObjectChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigEditedObjectChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigEditedObjectChanged < 'c, C > {
        type Target = TypedSigEditedObjectChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigEditedObjectChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigRestartRequested < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigRestartRequested < 'c, C: WithSignals > {
        typed: TypedSigRestartRequested < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigRestartRequested < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigRestartRequested < 'c, C > {
        type Target = TypedSigRestartRequested < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigRestartRequested < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for EditorInspector {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfEditorInspector < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfEditorInspector < 'c, C > {
        type Target = < < EditorInspector as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = EditorInspector;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfEditorInspector < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = EditorInspector;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}