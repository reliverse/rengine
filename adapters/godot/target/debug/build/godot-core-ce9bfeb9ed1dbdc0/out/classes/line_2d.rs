#![doc = "Sidecar module for class [`Line2D`][crate::classes::Line2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Line2D` enums](https://docs.godotengine.org/en/stable/classes/class_line2d.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Line2D.`\n\nInherits [`Node2D`][crate::classes::Node2D].\n\nRelated symbols:\n\n* [`line_2d`][crate::classes::line_2d]: sidecar module with related enum/flag types\n* [`ILine2D`][crate::classes::ILine2D]: virtual methods\n\n\nSee also [Godot docs for `Line2D`](https://docs.godotengine.org/en/stable/classes/class_line2d.html).\n\n"]
    #[doc = "# Construction\n\nThis class is manually managed. You can create a new instance using [`Line2D::new_alloc()`][crate::obj::NewAlloc::new_alloc].\n\nDo not forget to call [`free()`][crate::obj::Gd::free] or hand over ownership to Godot."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Line2D {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Line2D`][crate::classes::Line2D].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`INode2D`][crate::classes::INode2D] > ~~`ICanvasItem`~~ > [`INode`][crate::classes::INode] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `Line2D` methods](https://docs.godotengine.org/en/stable/classes/class_line2d.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ILine2D: crate::obj::GodotClass < Base = Line2D > + crate::private::You_forgot_the_attribute__godot_api {
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
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
    impl Line2D {
        pub fn set_points(&mut self, points: &PackedVector2Array,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedVector2Array >,);
            let args = (RefArg::new(points),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5016usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "set_points", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_points(&self,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5017usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "get_points", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_point_position(&mut self, index: i32, position: Vector2,) {
            type CallRet = ();
            type CallParams = (i32, Vector2,);
            let args = (index, position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5018usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "set_point_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_point_position(&self, index: i32,) -> Vector2 {
            type CallRet = Vector2;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5019usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "get_point_position", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_point_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5020usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "get_point_count", Some(self.__validated_obj()), args,)
            }
        }
        pub(crate) fn add_point_full(&mut self, position: Vector2, index: i32,) {
            type CallRet = ();
            type CallParams = (Vector2, i32,);
            let args = (position, index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5021usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "add_point", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::add_point_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn add_point(&mut self, position: Vector2,) {
            self.add_point_ex(position,) . done()
        }
        #[inline]
        pub fn add_point_ex < 'a > (&'a mut self, position: Vector2,) -> ExAddPoint < 'a > {
            ExAddPoint::new(self, position,)
        }
        pub fn remove_point(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5022usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "remove_point", Some(self.__validated_obj()), args,)
            }
        }
        pub fn clear_points(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5023usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "clear_points", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_closed(&mut self, closed: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (closed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5024usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "set_closed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_closed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5025usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "is_closed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_width(&mut self, width: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (width,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5026usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "set_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_width(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5027usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "get_width", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_curve(&mut self, curve: impl AsArg < Option < Gd < crate::classes::Curve >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Curve > > >,);
            let args = (curve.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5028usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "set_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_curve(&self,) -> Option < Gd < crate::classes::Curve > > {
            type CallRet = Option < Gd < crate::classes::Curve > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5029usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "get_curve", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_default_color(&mut self, color: Color,) {
            type CallRet = ();
            type CallParams = (Color,);
            let args = (color,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5030usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "set_default_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_default_color(&self,) -> Color {
            type CallRet = Color;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5031usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "get_default_color", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_gradient(&mut self, color: impl AsArg < Option < Gd < crate::classes::Gradient >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Gradient > > >,);
            let args = (color.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5032usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "set_gradient", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_gradient(&self,) -> Option < Gd < crate::classes::Gradient > > {
            type CallRet = Option < Gd < crate::classes::Gradient > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5033usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "get_gradient", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture(&mut self, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::Texture2D > > >,);
            let args = (texture.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5034usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "set_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5035usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "get_texture", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_texture_mode(&mut self, mode: crate::classes::line_2d::LineTextureMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::line_2d::LineTextureMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5036usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "set_texture_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_texture_mode(&self,) -> crate::classes::line_2d::LineTextureMode {
            type CallRet = crate::classes::line_2d::LineTextureMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5037usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "get_texture_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_joint_mode(&mut self, mode: crate::classes::line_2d::LineJointMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::line_2d::LineJointMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5038usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "set_joint_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_joint_mode(&self,) -> crate::classes::line_2d::LineJointMode {
            type CallRet = crate::classes::line_2d::LineJointMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5039usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "get_joint_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_begin_cap_mode(&mut self, mode: crate::classes::line_2d::LineCapMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::line_2d::LineCapMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5040usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "set_begin_cap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_begin_cap_mode(&self,) -> crate::classes::line_2d::LineCapMode {
            type CallRet = crate::classes::line_2d::LineCapMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5041usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "get_begin_cap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_end_cap_mode(&mut self, mode: crate::classes::line_2d::LineCapMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::line_2d::LineCapMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5042usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "set_end_cap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_end_cap_mode(&self,) -> crate::classes::line_2d::LineCapMode {
            type CallRet = crate::classes::line_2d::LineCapMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5043usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "get_end_cap_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_sharp_limit(&mut self, limit: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (limit,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5044usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "set_sharp_limit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_sharp_limit(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5045usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "get_sharp_limit", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_round_precision(&mut self, precision: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (precision,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5046usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "set_round_precision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_round_precision(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5047usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "get_round_precision", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_antialiased(&mut self, antialiased: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (antialiased,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5048usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "set_antialiased", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_antialiased(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5049usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Line2D", "get_antialiased", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for Line2D {
        type Base = crate::classes::Node2D;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Line2D"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Line2D {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node2D > for Line2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::CanvasItem > for Line2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Node > for Line2D {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Line2D {
        
    }
    impl crate::obj::cap::GodotDefault for Line2D {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for Line2D {
        type Target = crate::classes::Node2D;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Line2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Line2D`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Line2D__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Line2D > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Node2D > for $Class {
                
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
#[doc = "Default-param extender for [`Line2D::add_point_ex`][super::Line2D::add_point_ex]."]
#[must_use]
pub struct ExAddPoint < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Line2D, position: Vector2, index: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExAddPoint < 'a > {
    fn new(surround_object: &'a mut re_export::Line2D, position: Vector2,) -> Self {
        let index = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, position: position, index: index,
        }
    }
    #[inline]
    pub fn index(self, index: i32) -> Self {
        Self {
            index: index, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, position, index,
        }
        = self;
        re_export::Line2D::add_point_full(surround_object, position, index,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LineJointMode {
    ord: i32
}
impl LineJointMode {
    #[doc(alias = "LINE_JOINT_SHARP")]
    #[doc = "Godot enumerator name: `LINE_JOINT_SHARP`"]
    pub const SHARP: LineJointMode = LineJointMode {
        ord: 0i32
    };
    #[doc(alias = "LINE_JOINT_BEVEL")]
    #[doc = "Godot enumerator name: `LINE_JOINT_BEVEL`"]
    pub const BEVEL: LineJointMode = LineJointMode {
        ord: 1i32
    };
    #[doc(alias = "LINE_JOINT_ROUND")]
    #[doc = "Godot enumerator name: `LINE_JOINT_ROUND`"]
    pub const ROUND: LineJointMode = LineJointMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for LineJointMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LineJointMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LineJointMode {
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
            Self::SHARP => "SHARP", Self::BEVEL => "BEVEL", Self::ROUND => "ROUND", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LineJointMode::SHARP, LineJointMode::BEVEL, LineJointMode::ROUND]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LineJointMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("SHARP", "LINE_JOINT_SHARP", LineJointMode::SHARP), crate::meta::inspect::EnumConstant::new("BEVEL", "LINE_JOINT_BEVEL", LineJointMode::BEVEL), crate::meta::inspect::EnumConstant::new("ROUND", "LINE_JOINT_ROUND", LineJointMode::ROUND)]
        }
    }
}
impl crate::meta::GodotConvert for LineJointMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LineJointMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LineJointMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LineCapMode {
    ord: i32
}
impl LineCapMode {
    #[doc(alias = "LINE_CAP_NONE")]
    #[doc = "Godot enumerator name: `LINE_CAP_NONE`"]
    pub const NONE: LineCapMode = LineCapMode {
        ord: 0i32
    };
    #[doc(alias = "LINE_CAP_BOX")]
    #[doc = "Godot enumerator name: `LINE_CAP_BOX`"]
    pub const BOX: LineCapMode = LineCapMode {
        ord: 1i32
    };
    #[doc(alias = "LINE_CAP_ROUND")]
    #[doc = "Godot enumerator name: `LINE_CAP_ROUND`"]
    pub const ROUND: LineCapMode = LineCapMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for LineCapMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LineCapMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LineCapMode {
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
            Self::NONE => "NONE", Self::BOX => "BOX", Self::ROUND => "ROUND", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LineCapMode::NONE, LineCapMode::BOX, LineCapMode::ROUND]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LineCapMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "LINE_CAP_NONE", LineCapMode::NONE), crate::meta::inspect::EnumConstant::new("BOX", "LINE_CAP_BOX", LineCapMode::BOX), crate::meta::inspect::EnumConstant::new("ROUND", "LINE_CAP_ROUND", LineCapMode::ROUND)]
        }
    }
}
impl crate::meta::GodotConvert for LineCapMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LineCapMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LineCapMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LineTextureMode {
    ord: i32
}
impl LineTextureMode {
    #[doc(alias = "LINE_TEXTURE_NONE")]
    #[doc = "Godot enumerator name: `LINE_TEXTURE_NONE`"]
    pub const NONE: LineTextureMode = LineTextureMode {
        ord: 0i32
    };
    #[doc(alias = "LINE_TEXTURE_TILE")]
    #[doc = "Godot enumerator name: `LINE_TEXTURE_TILE`"]
    pub const TILE: LineTextureMode = LineTextureMode {
        ord: 1i32
    };
    #[doc(alias = "LINE_TEXTURE_STRETCH")]
    #[doc = "Godot enumerator name: `LINE_TEXTURE_STRETCH`"]
    pub const STRETCH: LineTextureMode = LineTextureMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for LineTextureMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LineTextureMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LineTextureMode {
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
            Self::NONE => "NONE", Self::TILE => "TILE", Self::STRETCH => "STRETCH", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LineTextureMode::NONE, LineTextureMode::TILE, LineTextureMode::STRETCH]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LineTextureMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "LINE_TEXTURE_NONE", LineTextureMode::NONE), crate::meta::inspect::EnumConstant::new("TILE", "LINE_TEXTURE_TILE", LineTextureMode::TILE), crate::meta::inspect::EnumConstant::new("STRETCH", "LINE_TEXTURE_STRETCH", LineTextureMode::STRETCH)]
        }
    }
}
impl crate::meta::GodotConvert for LineTextureMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for LineTextureMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LineTextureMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Line2D;
    use crate::registry::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::canvas_item::SignalsOfCanvasItem;
    impl WithSignals for Line2D {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfCanvasItem < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}